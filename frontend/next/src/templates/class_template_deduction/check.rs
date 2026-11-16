//! `templates/class-template-deduction`, the Check segment: the
//! arguments of a class template named without an argument list in the
//! declaration of a variable (CTAD rules, build step 5).

use crate::foundations::names::check as names;
use crate::overloading::conversion_sequences::check::{better, sequence, Ics};
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr};
use crate::sem::types::{DepType, QType, TArg, Type};
use crate::sem::{EntKind, ScopeKind};
use crate::source::Span;
use crate::syntax::Node;
use std::rc::Rc;

/// The kinds of guides (CTAD-5 to CTAD-9).
#[derive(Clone, Copy, PartialEq)]
enum From {
    User,
    Constructor,
    ConstructorTemplate,
    NoParameters,
    Copy,
    Aggregate,
}

/// A guide: its parameter types, in terms of the class's parameters and
/// then the constructor template's, the number of parameters that need
/// an argument, and whether it is explicit.
struct Guide {
    from: From,
    params: Vec<QType>,
    required: usize,
    explicit: bool,
    /// The number of the guide's own template parameters after the
    /// class's (a constructor template's), or a user guide's parameters.
    extra: usize,
    /// A user guide's result: the class's arguments in terms of its own
    /// parameters.
    result: Vec<TArg>,
    /// A user guide's position among the class template's guides.
    extra_index: usize,
    /// The last parameter is a function parameter pack whose pattern
    /// names the class's pack.
    pack: bool,
}

/// CTAD-11, CTAD-12: a `deduction_guide` with the parameters `params` of
/// its head, if any. False for a form that the front end does not model,
/// which the caller reports.
pub fn user_guide(cx: &mut Check, n: &Node, params: Vec<crate::templates::declarations::check::Param>, defaults: Vec<Option<Node>>) -> bool {
    if cx.func.is_some() || cx.member_of.is_some() || cx.head_pack {
        return false;
    }
    let Some(name) = n.child("template_name").map(|t| cx.source(t)) else { return false };
    let Some(tid) = n.child("template_id") else { return false };
    // CTAD-12: the guide is declared in the scope of its class template,
    // and its result names that template.
    let Some(v) = names::declared_here(cx, cx.scope, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::ClassTemplate(v) => Some(v),
        _ => None,
    }) else {
        return false;
    };
    if tid.child("template_name").map(|t| cx.source(t)) != Some(name.clone()) {
        cx.error(tid.span, "CTAD-12", format!("a deduction guide for '{}' whose result names another template", name));
        return true;
    }
    if params.iter().any(|p| p.2.is_some()) {
        return false;
    }
    // CTAD-12: one guide is not declared twice.
    let text: String = cx.source(n).split_whitespace().collect();
    if cx.u.class_templates[v].guides.iter().any(|(ps, _, g, _)| ps.len() == params.len() && cx.source(g).split_whitespace().collect::<String>() == text) {
        cx.error(n.span, "CTAD-12", format!("a deduction guide for '{}' declared twice", name));
        return true;
    }
    let limit = cx.u.entities.len();
    cx.u.class_templates[v].guides.push((params, defaults, n.clone(), limit));
    true
}

/// CTAD-17, CTAD-24: the specialization of the class template `v` that
/// the arguments `args` of an initializer choose. `list` is a braced
/// list, and `copy` copy-initialization. None after an error or an
/// unsupported form, which it reports.
pub fn deduce(cx: &mut Check, v: usize, args: &[Expr], list: bool, copy: bool, span: Span) -> Option<QType> {
    let ct = cx.u.class_templates[v].clone();
    let name = ct.name.clone();
    // PACK rules: a type pack as the last parameter takes the elements
    // that a guide's function parameter pack deduces.
    if ct.pack && ct.kinds.last().is_some_and(|k| k.is_some()) || ct.kinds.iter().any(|k| k.is_some()) && ct.defaults.iter().zip(&ct.kinds).any(|(d, k)| k.is_some() && d.is_none()) {
        return unsupported(cx, span, "a deduction for a class template with a pack or a constant parameter without a default");
    }
    if !ct.defined {
        return unsupported(cx, span, "a deduction for a class template that is declared and not defined");
    }
    if args.iter().any(|a| a.ty.is_error()) {
        return None;
    }
    // CTAD-9: the aggregate candidate takes a list in braces or in
    // parentheses.
    let guides = guides(cx, v, !(copy && !list), args.len(), span)?;
    let n = ct.params.len();
    // Each viable guide, with the conversion sequences of the arguments
    // and the class's arguments that it deduces.
    let mut viable: Vec<(usize, Vec<Ics>, Vec<TArg>)> = Vec::new();
    for (k, g) in guides.iter().enumerate() {
        // CTAD-17: copy-initialization leaves out the explicit guides.
        if g.explicit && copy && !list {
            continue;
        }
        let fixed = if g.pack { g.params.len() - 1 } else { g.params.len() };
        if (args.len() > g.params.len() && !g.pack) || args.len() < g.required.min(fixed) {
            continue;
        }
        let user = g.from == From::User;
        let mut bound: Vec<Option<QType>> = vec![None; if user { g.extra } else { n + g.extra }];
        let ok = g.params[..fixed].iter().zip(args).all(|(p, a)| {
            // CTAD-8: `T&&` with `T` a parameter of the class is no
            // forwarding reference.
            match &p.t {
                Type::Ref(inner, true) if !user && matches!(&inner.t, Type::Dep(d) if matches!(&**d, DepType::Param { index, .. } if (*index as usize) < n)) => {
                    a.cat != Cat::L && crate::templates::deduction::check::match_type(&cx.u, inner, &a.ty.nonref(), &mut bound)
                }
                _ => crate::templates::deduction::check::match_arg(cx, p, a, &[], &mut bound),
            }
        });
        if !ok {
            continue;
        }
        // PACK rules: each argument after the others deduces one element
        // of the class's pack from the pack's pattern.
        let mut elements: Vec<QType> = Vec::new();
        if g.pack {
            let p = g.params.last().unwrap();
            let mut fits = true;
            for a in &args[fixed..] {
                let mut each = bound.clone();
                each[n - 1] = None;
                if !crate::templates::deduction::check::match_arg(cx, p, a, &[], &mut each) {
                    fits = false;
                    break;
                }
                match each[n - 1].clone() {
                    Some(t) => elements.push(t),
                    None => {
                        fits = false;
                        break;
                    }
                }
            }
            if !fits {
                continue;
            }
        }
        if !user && bound[n..].iter().any(|b| b.is_none()) {
            continue;
        }
        // A user guide's parameter that its arguments leave takes its
        // default (CTAD-11, CTAD-13).
        if user && bound.iter().any(|b| b.is_none()) {
            let (gparams, gdefaults, _, glimit) = ct.guides[g.extra_index].clone();
            let names_: Vec<Rc<str>> = gparams.iter().map(|p| p.0.clone()).collect();
            let kinds: Vec<Option<QType>> = gparams.iter().map(|p| p.2.clone()).collect();
            let mut filled: Vec<TArg> = Vec::new();
            let mut fine = true;
            for (i, b) in bound.iter().enumerate() {
                match b {
                    Some(t) if fine => filled.push(TArg::Type(t.clone())),
                    _ if fine => {
                        let marks = cx.u.diags.len();
                        fine = crate::templates::declarations::check::fill_defaults(cx, &kinds[..=i], &gdefaults, &names_, ct.scope, glimit, &mut filled, span) && cx.u.diags.len() == marks;
                        if !fine {
                            cx.u.diags.truncate(marks);
                        }
                    }
                    _ => {}
                }
            }
            if !fine {
                continue;
            }
            bound = filled.into_iter().map(|a| match a {
                TArg::Type(t) => Some(t),
                _ => None,
            }).collect();
            if bound.iter().any(|b| b.is_none()) {
                continue;
            }
        }
        // CTAD-11: a user guide's result gives the class's arguments.
        let from_result: Vec<Option<QType>> = if user {
            let known: Vec<QType> = bound.iter().map(|b| b.clone().unwrap_or_else(QType::error)).collect();
            let mut out: Vec<Option<QType>> = g.result.iter().map(|a| match a {
                TArg::Type(t) => Some(concrete(cx, &crate::templates::deduction::check::substitute(t, &known), span)),
                _ => None,
            }).collect();
            out.resize(n, None);
            out
        } else {
            bound[..n].to_vec()
        };
        // TPL-13: a class parameter that the guide does not deduce takes
        // its default.
        let mut class_args: Vec<TArg> = Vec::new();
        let mut failed = false;
        for (i, b) in from_result.iter().enumerate() {
            match b {
                _ if ct.pack && i == n - 1 => class_args.push(TArg::Pack(elements.iter().cloned().map(TArg::Type).collect())),
                Some(t) if ct.kinds[i].is_none() => class_args.push(TArg::Type(t.clone())),
                _ => {
                    let marks = cx.u.diags.len();
                    let filled = crate::templates::declarations::check::fill_defaults(cx, &ct.kinds[..=i], &ct.defaults, &ct.params, ct.scope, ct.limit, &mut class_args, span);
                    if !filled || cx.u.diags.len() > marks {
                        cx.u.diags.truncate(marks);
                        failed = true;
                        break;
                    }
                }
            }
        }
        if failed {
            continue;
        }
        let known: Vec<QType> = if user {
            bound.iter().map(|b| b.clone().unwrap_or_else(QType::error)).collect()
        } else {
            (0..n + g.extra).map(|i| match class_args.get(i) {
                Some(TArg::Type(t)) => t.clone(),
                Some(_) => QType::error(),
                None => bound[i].clone().unwrap_or_else(QType::error),
            }).collect()
        };
        let mut seqs = Vec::new();
        for (j, a) in args.iter().enumerate() {
            let (p, known) = match g.params.get(j.min(g.params.len().saturating_sub(1))) {
                Some(p) if g.pack && j >= fixed => {
                    let mut each = known.clone();
                    each[n - 1] = elements[j - fixed].clone();
                    (p, each)
                }
                Some(p) => (p, known.clone()),
                None => break,
            };
            let p = crate::templates::deduction::check::substitute(p, &known);
            let p = concrete(cx, &p, span);
            match sequence(cx, a, &p) {
                Some(s) => seqs.push(s),
                None => break,
            }
        }
        if seqs.len() == args.len() {
            viable.push((k, seqs, class_args));
        }
    }
    if viable.is_empty() {
        cx.error_cause(span, "CTAD-24", Some("OVL-35"), format!("no guide of the class template '{}' fits the initializer", name));
        return None;
    }
    // OVL-14: the best guide by the arguments' sequences, then CTAD-21
    // and CTAD-22.
    let wins = |a: &(usize, Vec<Ics>, Vec<TArg>), b: &(usize, Vec<Ics>, Vec<TArg>)| -> bool {
        let mut some = false;
        for (x, y) in a.1.iter().zip(&b.1) {
            if better(y, x) {
                return false;
            }
            if better(x, y) {
                some = true;
            }
        }
        if some {
            return true;
        }
        let (fa, fb) = (guides[a.0].from, guides[b.0].from);
        // OVL-27: a user guide that is no template wins over a guide that
        // is one, which every other guide is.
        let plain = |k: usize| guides[k].from == From::User && guides[k].extra == 0;
        if plain(a.0) != plain(b.0) {
            return plain(a.0);
        }
        // ORD rules: of two guide templates, the more specialized wins.
        let ab = at_least_as_specialized(cx, &guides[a.0], &guides[b.0], args.len());
        let ba = at_least_as_specialized(cx, &guides[b.0], &guides[a.0], args.len());
        if ab != ba {
            return ab;
        }
        (fa == From::User && fb != From::User) || (fb != From::User && fa == From::Copy && fb != From::Copy) || (fa == From::Constructor && fb == From::ConstructorTemplate)
    };
    let mut best = 0;
    for i in 1..viable.len() {
        if wins(&viable[i], &viable[best]) {
            best = i;
        }
    }
    if (0..viable.len()).any(|i| i != best && !wins(&viable[best], &viable[i])) {
        cx.error_cause(span, "CTAD-24", Some("OVL-36"), format!("the guides of the class template '{}' are ambiguous for the initializer", name));
        return None;
    }
    // CTAD-23: copy-list-initialization does not choose an explicit guide.
    if copy && list && guides[viable[best].0].explicit {
        cx.error_cause(span, "CTAD-23", Some("ILC-8"), format!("copy-list-initialization of '{}' chooses an explicit guide", name));
        return None;
    }
    let args = viable[best].2.clone();
    Some(crate::templates::declarations::check::class_specialization(cx, v, args, span))
}

/// CTAD-1: the class template that the node `n` names with no argument
/// list, as one name.
pub fn named_template(cx: &Check, n: &Node) -> Option<usize> {
    let name = cx.source(n);
    let name = name.trim();
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    names::lookup(cx, name).first().and_then(|&e| match cx.u.entities[e].kind {
        EntKind::ClassTemplate(v) => Some(v),
        _ => None,
    })
}

/// ORD-1 to ORD-3 for guides: the parameters of `g2` for the first
/// `count` arguments deduce from those of `g1`, with each type without
/// its reference and its top-level cv-qualifiers (ORD-2).
fn at_least_as_specialized(cx: &Check, g1: &Guide, g2: &Guide, count: usize) -> bool {
    let size = 256;
    let mut bound: Vec<Option<QType>> = vec![None; size];
    g1.params.iter().zip(&g2.params).take(count).all(|(p1, p2)| crate::templates::deduction::check::match_type(&cx.u, &p2.nonref().unqual(), &p1.nonref().unqual(), &mut bound))
}

/// `t` with each specialization whose arguments hold no parameter made
/// into its class.
fn concrete(cx: &mut Check, t: &QType, span: Span) -> QType {
    let q = match &t.t {
        Type::Dep(d) => match &**d {
            DepType::Spec { template, args, .. } if !args.iter().any(|a| matches!(a, TArg::Type(x) if crate::templates::deduction::check::has_dep(x))) => {
                let c = crate::templates::declarations::check::class_specialization(cx, *template, args.clone(), span);
                return c.with_cv(c.c || t.c, c.v || t.v);
            }
            _ => return t.clone(),
        },
        Type::Ref(x, rv) => Type::Ref(Rc::new(concrete(cx, x, span)), *rv),
        Type::Ptr(x) => Type::Ptr(Rc::new(concrete(cx, x, span))),
        _ => return t.clone(),
    };
    QType { t: q, c: t.c, v: t.v }
}

fn unsupported<T>(cx: &mut Check, span: Span, why: &str) -> Option<T> {
    cx.unsupported(span, why.to_string());
    None
}

/// The guides of the class template `v` (CTAD-5 to CTAD-9).
fn guides(cx: &mut Check, v: usize, list: bool, given: usize, span: Span) -> Option<Vec<Guide>> {
    let ct = cx.u.class_templates[v].clone();
    let n = ct.params.len();
    let class = ct.node.clone();
    let mut out = Vec::new();
    let mut constructors = 0;
    let mut members: Vec<QType> = Vec::new();
    let mut aggregate = class.find("base_clause").is_none() && class.prod == "class_def" && cx.first_word(class.child("class_head").unwrap_or(&class)) == "struct";
    let params: Vec<crate::templates::declarations::check::Param> = ct.params.iter().zip(&ct.kinds).map(|(p, k)| (p.clone(), span, k.clone())).collect();
    let saved = cx.scope;
    let saved_limit = cx.lookup_limit.replace(ct.limit);
    cx.scope = ct.scope;
    let ts = cx.enter(ScopeKind::Template);
    crate::templates::declarations::check::declare_params(cx, ts, &params);
    let mut failed = false;
    for m in class.children("member_decl") {
        // A member template: a constructor template gives a guide.
        let (inner, head) = match m.child("template_head") {
            Some(h) => match m.child("member_decl") {
                Some(i) => (i, Some(h)),
                None => continue,
            },
            None => (m, None),
        };
        if m.find("access_spec").is_some() && m.child("decl_specifiers").is_none() && m.child("function_definition").is_none() {
            // `public:` and the like.
            if !cx.has_token(m, "public") {
                aggregate = false;
            }
            continue;
        }
        let (ds, dn) = match inner.child("function_definition") {
            Some(fd) => (fd.child("decl_specifiers"), fd.child("declarator")),
            None => (inner.child("decl_specifiers"), inner.child("member_declarator").and_then(|md| md.child("declarator"))),
        };
        let Some(dn) = dn else { continue };
        let is_ctor = dn.find("function_suffix").is_some() && dn.find("declarator_id").is_some_and(|i| cx.source(i) == *ct.name);
        if !is_ctor {
            if dn.find("function_suffix").is_none() && head.is_none() && !ds.is_some_and(|ds| cx.has_token(ds, "static") || cx.has_token(ds, "typedef")) && inner.child("member_declarator").is_some() {
                // A data member, for the aggregate candidate.
                if let Some(ds) = ds {
                    for md in inner.children("member_declarator") {
                        let Some(d) = md.child("declarator") else { continue };
                        let marks = cx.u.diags.len();
                        let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
                        let decl = crate::declarations::declarators::check::declarator(cx, &specs, d);
                        if specs.unsupported || decl.unsupported || cx.u.diags.len() > marks {
                            cx.u.diags.truncate(marks);
                            failed = true;
                        }
                        members.push(decl.ty);
                    }
                }
            }
            if dn.find("function_suffix").is_some() && ds.is_some_and(|ds| cx.has_token(ds, "virtual")) {
                aggregate = false;
            }
            continue;
        }
        constructors += 1;
        aggregate = false;
        // The constructor template's own parameters come after the
        // class's.
        let mut extra = 0;
        let marks = cx.u.diags.len();
        let inner_scope = cx.enter(ScopeKind::Template);
        if let Some(h) = head {
            let Some((ps, defaults, constraints, pack)) = crate::templates::declarations::check::type_params(cx, h) else {
                cx.leave(ts);
                cx.u.diags.truncate(marks);
                failed = true;
                continue;
            };
            if pack || !constraints.is_empty() || defaults.iter().any(|d| d.is_some()) || ps.iter().any(|p| p.2.is_some()) {
                cx.leave(ts);
                failed = true;
                continue;
            }
            for (k, (name, sp, _)) in ps.iter().enumerate() {
                let t = QType::new(Type::Dep(Rc::new(DepType::Param { depth: 0, index: (n + k) as u32, name: name.clone() })));
                names::declare_in(cx, inner_scope, name, EntKind::Typedef(t), *sp, crate::sem::Linkage::None);
            }
            extra = ps.len();
        }
        // A constructor's specifiers are `explicit`, `constexpr`,
        // `consteval`, and `inline`, which give a guide nothing else.
        let only_ctor_words = ds.is_none_or(|ds| {
            let mut toks = Vec::new();
            ds.tokens(&mut toks);
            toks.iter().all(|&t| matches!(cx.toks[t].text.as_str(), "explicit" | "constexpr" | "consteval" | "inline"))
        });
        let mut specs = match ds {
            Some(ds) if !only_ctor_words => crate::declarations::check::decl_specifiers(cx, ds, Default::default()),
            _ => Default::default(),
        };
        let explicit = ds.is_some_and(|ds| cx.has_token(ds, "explicit"));
        specs.ty = Some(QType::void());
        let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
        cx.leave(ts);
        // A function parameter pack comes last, of the class's pack.
        let pack_param = d.params.last().is_some_and(|p| p.pack.is_some());
        if cx.u.diags.len() > marks || d.unsupported || specs.unsupported || d.params.iter().rev().skip(1).any(|p| p.pack.is_some()) || (pack_param && (!ct.pack || head.is_some())) {
            // A constructor whose parameters name the class itself, such
            // as a copy constructor, gives its guide through the copy
            // candidate.
            cx.u.diags.truncate(marks);
            let own = dn.find("parameters").is_some_and(|ps| {
                let mut toks = Vec::new();
                ps.tokens(&mut toks);
                toks.iter().any(|&t| cx.toks[t].text == *ct.name)
            });
            if !own {
                failed = true;
            }
            continue;
        }
        let Some(f) = d.ty.func() else { continue };
        // CTAD-18: an initializer-list guide waits for a later piece.
        if f.variadic || f.params.first().is_some_and(|p| p.nonref().class().is_some_and(|c| crate::classes::initializer_lists::check::template(cx).is_some_and(|il| cx.u.class_templates[il].specs.iter().any(|&(_, k)| k == c))) || matches!(&p.nonref().t, Type::Dep(d) if matches!(&**d, DepType::Spec { template, .. } if Some(*template) == crate::classes::initializer_lists::check::template(cx)))) {
            failed = true;
            continue;
        }
        let required = d.params.iter().take_while(|p| p.default.is_none()).count();
        out.push(Guide { from: if head.is_some() { From::ConstructorTemplate } else { From::Constructor }, params: f.params.clone(), required, explicit, extra, result: Vec::new(), extra_index: 0, pack: pack_param });
    }
    cx.leave(saved);
    cx.lookup_limit = saved_limit;
    if failed {
        return unsupported(cx, span, "a deduction from a class template whose constructors or members the front end does not model as guides");
    }
    // CTAD-27: the aggregate candidate of a class template with a pack
    // waits for a later piece.
    if ct.pack && constructors == 0 && given > 0 {
        return unsupported(cx, span, "a deduction for an aggregate class template with a pack (CTAD-27)");
    }
    // CTAD-6: a class template with no constructor has a guide with no
    // parameters.
    if constructors == 0 {
        out.push(Guide { from: From::NoParameters, params: Vec::new(), required: 0, explicit: false, extra: 0, result: Vec::new(), extra_index: 0, pack: false });
    }
    // CTAD-7: the copy candidate, which waits for a later piece for a
    // class template with a pack.
    if !ct.pack {
        let own: Vec<TArg> = ct.params.iter().enumerate().map(|(k, p)| TArg::Type(QType::new(Type::Dep(Rc::new(DepType::Param { depth: 0, index: k as u32, name: p.clone() }))))).collect();
        let spec = QType::new(Type::Dep(Rc::new(DepType::Spec { template: v, args: own, name: ct.name.clone() })));
        out.push(Guide { from: From::Copy, params: vec![spec], required: 1, explicit: false, extra: 0, result: Vec::new(), extra_index: 0, pack: false });
    }
    // CTAD-11: the user guides declared before the use.
    for (gi, (gparams, _, node, limit)) in ct.guides.clone().into_iter().enumerate() {
        let saved = cx.scope;
        let saved_limit = cx.lookup_limit.replace(limit);
        cx.scope = ct.scope;
        let gs = cx.enter(ScopeKind::Template);
        crate::templates::declarations::check::declare_params(cx, gs, &gparams);
        let marks = cx.u.diags.len();
        let (ps, variadic) = crate::declarations::declarators::check::parameters(cx, node.child("parameters"));
        let result = node.child("template_id").and_then(|tid| crate::templates::declarations::check::template_args(cx, tid, &ct.kinds[..tid.child("template_args").map(|ta| ta.children("template_arg").count()).unwrap_or(0).min(ct.kinds.len())]));
        cx.leave(saved);
        cx.lookup_limit = saved_limit;
        let Some(result) = result.filter(|_| !variadic && cx.u.diags.len() == marks && !ps.iter().any(|p| p.pack.is_some())) else {
            cx.u.diags.truncate(marks);
            return unsupported(cx, span, "a user guide of a form that the front end does not model");
        };
        let required = ps.iter().take_while(|p| p.default.is_none()).count();
        let explicit = node.child("explicit_spec").is_some();
        out.push(Guide { from: From::User, params: ps.iter().map(|p| p.ty.clone()).collect(), required, explicit, extra: gparams.len(), result, extra_index: gi, pack: false });
    }
    // CTAD-9, CTAD-10: the aggregate candidate, for a list with an
    // element for each of its first members, when the class template
    // has no user guide. Brace elision waits for a later piece.
    if aggregate && list && given > 0 && ct.guides.is_empty() && !ct.pack {
        // Brace elision into an array or a class whose type does not
        // depend on a parameter waits for a later piece (CTAD-10).
        if members.iter().any(|m| m.is_array() || m.class().is_some()) || given > members.len() {
            return unsupported(cx, span, "an aggregate candidate with an array or a class member, or with brace elision");
        }
        out.push(Guide { from: From::Aggregate, params: members[..given].to_vec(), required: given, explicit: false, extra: 0, result: Vec::new(), extra_index: 0, pack: false });
    }
    Some(out)
}
