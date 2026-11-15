//! The check part of `templates/declarations`: function templates whose
//! parameters are types (build step 5, first piece,
//! `frontend/TEMPLATE-MODEL.md` sections 1 and 3). The declaration gives
//! the function type with each parameter as a dependent type. A call
//! deduces the arguments (`templates/deduction`), and the specialization
//! is the definition checked with each parameter bound to its argument.
//! The definition itself is not checked yet (model section 5), so a
//! template that nothing specializes leaves the unit unsupported.

use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::types::{DepType, QType, TArg, Type};
use crate::sem::{EntKind, EntityId, FnTemplate, Linkage, ScopeKind};
use crate::source::Span;
use crate::syntax::{Kid, Node};
use std::rc::Rc;

/// A template parameter: its name, where it is declared, and None for a
/// type parameter or the type of a constant parameter.
type Param = (Rc<str>, Span, Option<QType>);

/// The parameters of a template head whose parameters are type
/// parameters and constant parameters of integral or enumeration type,
/// without a default, a pack, or a constraint; None for another head.
fn type_params(cx: &mut Check, head: &Node) -> Option<Vec<Param>> {
    if head.child("requires_clause").is_some() {
        return None;
    }
    let mut out = Vec::new();
    for tp in head.child("template_params")?.children("template_param") {
        // TARG-13: a constant parameter of integral or enumeration type.
        if let Some(p) = tp.child("constant_param").and_then(|c| c.child("parameter")) {
            if cx.words(p).iter().any(|w| matches!(*w, "=" | "...")) || p.find("ptr_part").is_some() {
                return None;
            }
            let ds = p.child("decl_specifiers")?;
            let marks = cx.u.diags.len();
            let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
            let ty = specs.ty.clone().filter(|t| (t.is_integral() || t.is_enum()) && cx.u.diags.len() == marks && !specs.unsupported);
            let Some(ty) = ty else {
                cx.u.diags.truncate(marks);
                return None;
            };
            let id = p.find("declarator_id")?;
            out.push((cx.source(id).as_str().into(), id.span, Some(ty.unqual())));
            continue;
        }
        let t = tp.child("type_param")?;
        if t.child("type_key").is_none() || t.kids.iter().any(|k| matches!(k, Kid::Tok(_)) && matches!(cx.text(k), "=" | "...")) {
            return None;
        }
        let k = t.kids.iter().rev().find(|k| matches!(k, Kid::Tok(_)))?;
        let name: Rc<str> = cx.text(k).into();
        if name.as_ref() == "class" || name.as_ref() == "typename" {
            return None;
        }
        out.push((name, cx.token(k).map(|t| t.span).unwrap_or(t.span), None));
    }
    Some(out)
}

/// Declare the parameters in the scope `ts` at a definition: a type
/// parameter as a dependent type, a constant parameter as a variable of
/// its type whose value is not known.
fn declare_params(cx: &mut Check, ts: crate::sem::ScopeId, params: &[Param]) {
    for (k, (name, span, kind)) in params.iter().enumerate() {
        match kind {
            None => {
                let t = QType::new(Type::Dep(Rc::new(DepType::Param { depth: 0, index: k as u32, name: name.clone() })));
                names::declare_in(cx, ts, name, EntKind::Typedef(t), *span, Linkage::None);
            }
            Some(ty) => {
                let v = crate::sem::Var { ty: ty.with_cv(true, false), storage: crate::sem::Storage::Automatic, constexpr: false, constinit: false, inline: false, is_extern: false, is_static: false, param: true, defined: true, init: None, const_init: false, used: true, maybe_unused: true, align: None };
                names::declare_in(cx, ts, name, EntKind::Var(v), *span, Linkage::None);
            }
        }
    }
}

/// Bind the parameters `names` to the arguments `args` in the scope `ts`
/// of a specialization: a type parameter names its type, and a constant
/// parameter is a constant of its value (TARG-13).
fn bind_params(cx: &mut Check, ts: crate::sem::ScopeId, names_: &[Rc<str>], args: &[TArg], span: Span) {
    for (name, a) in names_.iter().zip(args) {
        match a {
            TArg::Type(t) => {
                names::declare_in(cx, ts, name, EntKind::Typedef(t.clone()), span, Linkage::None);
            }
            TArg::Value(t, v) => {
                let init = crate::sem::tree::Init::Expr(crate::sem::tree::Expr::int(*v, t.clone(), span));
                let var = crate::sem::Var { ty: t.with_cv(true, false), storage: crate::sem::Storage::Static, constexpr: true, constinit: false, inline: true, is_extern: false, is_static: false, param: false, defined: true, init: Some(init), const_init: true, used: true, maybe_unused: true, align: None };
                let id = names::declare_in(cx, ts, name, EntKind::Var(var), span, Linkage::None);
                cx.template_constants.insert(id);
            }
        }
    }
}

/// The arguments of a template-id for parameters of the kinds `kinds`: a
/// type for a type parameter, and a constant for a constant parameter,
/// converted to its type. None for another number or form of arguments.
pub fn template_args(cx: &mut Check, tid: &Node, kinds: &[Option<QType>]) -> Option<Vec<TArg>> {
    let nodes: Vec<&Node> = tid.child("template_args").map(|ta| ta.children("template_arg").collect()).unwrap_or_default();
    if nodes.len() != kinds.len() {
        return None;
    }
    let mut out = Vec::new();
    for (a, kind) in nodes.into_iter().zip(kinds) {
        match kind {
            None => out.push(TArg::Type(crate::declarations::declarators::check::type_id(cx, a.child("type_id")?))),
            Some(ty) => {
                let (e, v) = constant_arg(cx, a)?;
                // TARG-13, CE-19: the argument converts to the parameter's
                // type as a converted constant expression: an enumeration
                // only from itself, and no narrowing.
                let scoped = matches!(ty.t, Type::Enum(en) if cx.u.enums[en].scoped);
                let other_enum = (ty.is_enum() || matches!(e.ty.t, Type::Enum(en) if cx.u.enums[en].scoped)) && e.ty.unqual().t != ty.t;
                if (scoped || other_enum) && e.ty.unqual().t != ty.t || crate::foundations::conversions::check::is_narrowing(cx, &e, ty) {
                    cx.error_cause(a.span, "TARG-13", Some("CE-19"), format!("the template argument of type {} does not convert to {} as a constant", e.ty.show(), ty.show()));
                    return None;
                }
                let v = match ty.int_kind() {
                    Some(k) => cx.u.target.wrap(k, v),
                    None => v,
                };
                out.push(TArg::Value(ty.clone(), v));
            }
        }
    }
    Some(out)
}

/// The value of a template argument for a constant parameter: an
/// expression, or a name that the parser read as a type.
fn constant_arg(cx: &mut Check, a: &Node) -> Option<(crate::sem::tree::Expr, i128)> {
    let node = crate::expressions::check::first_node(a)?;
    let e = if node.prod == "type_id" {
        let name = cx.source(node);
        let var = names::lookup(cx, &name).into_iter().find(|&e| cx.u.var(e).is_some())?;
        let ty = cx.u.var(var)?.ty.clone();
        crate::sem::tree::Expr::new(crate::sem::tree::EK::Var(var), ty.nonref(), crate::sem::tree::Cat::L, node.span)
    } else {
        crate::expressions::check::expr(cx, node)
    };
    let e = crate::foundations::values::check::rvalue(cx, e);
    if e.ty.is_error() || !(e.ty.is_integral() || e.ty.is_enum()) {
        return None;
    }
    let v = crate::foundations::constant_evaluation::check::try_value(cx, &e).and_then(|v| v.int())?;
    Some((e, v))
}

/// `template_decl` at namespace scope: a function template definition
/// whose parameters are types. False for another form, which the caller
/// reports as unsupported.
pub fn template_decl(cx: &mut Check, n: &Node) -> bool {
    if cx.func.is_some() || cx.member_of.is_some() {
        return false;
    }
    let Some(params) = n.child("template_head").and_then(|h| type_params(cx, h)) else { return false };
    if let Some(sd) = n.child("declaration").and_then(|d| d.child("block_decl")).and_then(|b| b.child("simple_decl")) {
        if sd.child("init_declarator").is_none() {
            return class_template_decl(cx, sd, params);
        }
        return var_template_decl(cx, sd, params);
    }
    let Some(fd) = n.child("declaration").and_then(|d| d.child("function_definition")) else { return false };
    let (Some(ds), Some(dn)) = (fd.child("decl_specifiers"), fd.child("declarator")) else { return false };
    let saved = cx.scope;
    let ts = cx.enter(ScopeKind::Template);
    declare_params(cx, ts, &params);
    let marks = cx.u.diags.len();
    let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    cx.leave(saved);
    let fty = d.ty.func().cloned();
    let (Some((name, span)), Some(fty)) = (d.name.clone(), fty) else {
        cx.u.diags.truncate(marks);
        return false;
    };
    if specs.unsupported || d.unsupported || d.qual != names::Qual::None || cx.u.diags.len() > marks || d.placeholder.is_some() {
        cx.u.diags.truncate(marks);
        return false;
    }
    // The definition checks of model section 5 wait for a later piece, so
    // a definition with a dependent construct that the first piece does
    // not model, or a name another function or template shares, stays
    // unsupported.
    if !names::declared_here(cx, saved, &name).is_empty() || dependent_forms(cx, fd, &params) {
        return false;
    }
    let id = cx.u.templates.len();
    let limit = cx.u.entities.len() + 1;
    cx.u.templates.push(FnTemplate { name: name.clone(), kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node: fd.clone(), scope: saved, fty: Rc::new(fty), specs: Vec::new(), span, limit });
    names::declare_in(cx, saved, &name, EntKind::Template(id), span, Linkage::External);
    true
}

/// The definition uses a form that needs the dependent names of model
/// sections 1 and 2: `typename`, `template`, `requires`, `decltype`,
/// `concept`, or a name qualified by a template parameter.
fn dependent_forms(cx: &Check, fd: &Node, params: &[Param]) -> bool {
    let mut toks = Vec::new();
    fd.tokens(&mut toks);
    let words: Vec<&str> = toks.iter().map(|&t| cx.toks[t].text.as_str()).collect();
    // DEP-31: a class that is incomplete at the definition and complete
    // at a specialization needs the definition checks.
    let incomplete = words.iter().any(|w| names::lookup(cx, w).iter().any(|&e| matches!(cx.u.entities[e].kind, EntKind::Class(c) if !cx.u.classes[c].complete)));
    // `decltype` of an operand that names no parameter is independent.
    let mut dependent_decltype = false;
    for (k, w) in words.iter().enumerate() {
        if *w != "decltype" {
            continue;
        }
        let mut depth = 0;
        for x in &words[k + 1..] {
            match *x {
                "(" => depth += 1,
                ")" => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ if params.iter().any(|(p, _, _)| p.as_ref() == *x) => dependent_decltype = true,
                _ => {}
            }
        }
    }
    incomplete
        || dependent_decltype
        || words.iter().any(|w| matches!(*w, "typename" | "template" | "requires" | "concept"))
        // A pack expansion or a fold of a parameter (packs wait for a
        // later piece); `catch (...)` and a C ellipsis are not.
        || words.iter().enumerate().any(|(k, w)| *w == "..." && words[k.saturating_sub(3)..k].iter().any(|x| params.iter().any(|(p, _, _)| p.as_ref() == *x)))
        || words.windows(2).any(|w| w[1] == "::" && params.iter().any(|(p, _, _)| p.as_ref() == w[0]))
}

/// TARG-1, DED-P3: the specialization of the function template `t` for
/// the arguments `args`, made the first time: the definition checked in
/// a scope that binds each parameter to its argument. The function is
/// recorded before its body is checked, so a call of the same
/// specialization in the body finds it. None when the declaration does
/// not check.
pub fn specialize(cx: &mut Check, t: usize, args: &[TArg]) -> Option<EntityId> {
    if let Some(&(_, f)) = cx.u.templates[t].specs.iter().find(|(a, _)| a.as_slice() == args) {
        return Some(f);
    }
    let tpl = cx.u.templates[t].clone();
    let saved_scope = cx.scope;
    let saved_func = cx.func.take();
    let saved_limit = cx.lookup_limit.replace(tpl.limit);
    // The specialization is a function of the template's namespace, made
    // outside the class, lambda, or operand that the call stands in.
    let saved_member = cx.member_of.take();
    let saved_lambdas = std::mem::take(&mut cx.lambdas);
    let saved_unevaluated = std::mem::replace(&mut cx.unevaluated, 0);
    let saved_default = std::mem::replace(&mut cx.default_arg, false);
    let saved_access = cx.access_class.take();
    cx.instantiating += 1;
    let marks = cx.u.diags.len();
    cx.scope = tpl.scope;
    let ts = cx.enter(ScopeKind::Template);
    bind_params(cx, ts, &tpl.params, args, tpl.span);
    let fd = &tpl.node;
    let r = (|| {
        let specs = crate::declarations::check::decl_specifiers(cx, fd.child("decl_specifiers")?, Default::default());
        let dn = fd.child("declarator")?;
        let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
        if specs.unsupported || d.unsupported || !d.ty.is_func() {
            return None;
        }
        let body = fd.child("function_body");
        let f = crate::declarations::functions::definitions::check::declare_function(cx, &specs, &d, body)?;
        cx.u.templates[t].specs.push((args.to_vec(), f));
        let cs = body.and_then(|b| b.child("compound_stmt"));
        if let Some(cs) = cs {
            crate::declarations::functions::definitions::check::define(cx, f, &d, cs);
        }
        Some(f)
    })();
    cx.scope = saved_scope;
    cx.func = saved_func;
    cx.lookup_limit = saved_limit;
    cx.member_of = saved_member;
    cx.lambdas = saved_lambdas;
    cx.unevaluated = saved_unevaluated;
    cx.default_arg = saved_default;
    cx.access_class = saved_access;
    cx.instantiating -= 1;
    // An error in a specialization can come from a part of the model that
    // the first piece does not have (the points of instantiation, the
    // definition checks), so it leaves the unit unsupported.
    if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
        let tail = cx.u.diags.split_off(marks);
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev != crate::diag::Severity::Error));
        cx.unsupported(tpl.span, format!("an error in a specialization of the function template '{}' (the first piece of build step 5 does not tell it from a gap of its model)", tpl.name));
    }
    r
}

/// The function templates that a call by name finds, after their
/// specializations for the call's arguments are made: each template
/// whose arguments deduce (DED-P3), with the explicit arguments first.
pub fn call_candidates(cx: &mut Check, found: &[EntityId], tid: Option<&Node>, args: &[crate::sem::tree::Expr]) -> Vec<EntityId> {
    let mut out = Vec::new();
    for &e in found {
        let EntKind::Template(t) = cx.u.entities[e].kind else { continue };
        // The explicit arguments, which fill the first parameters.
        let explicit = match tid {
            Some(tid) => {
                let kinds = cx.u.templates[t].kinds.clone();
                let n = tid.child("template_args").map(|ta| ta.children("template_arg").count()).unwrap_or(0);
                match template_args(cx, tid, &kinds[..n.min(kinds.len())]) {
                    Some(a) if n <= kinds.len() => a,
                    _ => continue,
                }
            }
            None => Vec::new(),
        };
        let Some(bound) = crate::templates::deduction::check::deduce_call(cx, t, &explicit, args) else { continue };
        if let Some(f) = specialize(cx, t, &bound) {
            out.push(f);
        }
    }
    out
}

/// The function and class templates of the unit that nothing
/// specialized: their definitions are not checked yet, so the unit is
/// unsupported.
pub fn end_of_unit(cx: &mut Check) {
    let mut unused: Vec<(Rc<str>, Span)> = cx.u.templates.iter().filter(|t| t.specs.is_empty()).map(|t| (t.name.clone(), t.span)).collect();
    unused.extend(cx.u.class_templates.iter().filter(|t| t.specs.is_empty()).map(|t| (t.name.clone(), t.span)));
    for (name, span) in unused {
        cx.unsupported(span, format!("the template '{}', which nothing specializes (its definition is not checked yet)", name));
    }
}

/// The declarator, the initializer, and the template-id of the name of a
/// variable template's declaration with one declarator, if it has that
/// form.
fn var_parts(sd: &Node) -> Option<(&Node, &Node, &Node, Option<&Node>)> {
    let ds = sd.child("decl_specifiers")?;
    let mut decls = sd.children("init_declarator");
    let idecl = decls.next()?;
    if decls.next().is_some() {
        return None;
    }
    let dn = idecl.child("declarator")?;
    let init = idecl.child("init")?;
    let tid = dn.find("declarator_id").and_then(|d| d.find("template_id"));
    Some((ds, dn, init, tid))
}

/// TPL rules: a variable template at namespace scope whose parameters are
/// types, with one declarator and an initializer, or a partial
/// specialization of one (TSP-9). False for another form.
fn var_template_decl(cx: &mut Check, sd: &Node, params: Vec<Param>) -> bool {
    let Some((_, dn, init, tid)) = var_parts(sd) else { return false };
    if dn.find("ptr_operator").is_some() || dependent_forms(cx, init, &params) {
        return false;
    }
    let saved = cx.scope;
    match tid {
        None => {
            let Some(name) = dn.find("declarator_id").and_then(|d| d.find("unqualified_id")).map(|u| cx.source(u)) else { return false };
            if !names::declared_here(cx, saved, &name).is_empty() {
                return false;
            }
            let id = cx.u.var_templates.len();
            let limit = cx.u.entities.len() + 1;
            cx.u.var_templates.push(crate::sem::VarTemplate { name: name.as_str().into(), kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node: sd.clone(), scope: saved, partials: Vec::new(), specs: Vec::new(), span: dn.span, limit });
            names::declare_in(cx, saved, &name, EntKind::VarTemplate(id), dn.span, Linkage::External);
            true
        }
        Some(tid) => {
            // TSP-9: a partial specialization of a primary of the scope.
            let Some(name) = tid.child("template_name").map(|t| cx.source(t)) else { return false };
            let Some(v) = names::declared_here(cx, saved, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
                EntKind::VarTemplate(v) => Some(v),
                _ => None,
            }) else {
                return false;
            };
            // A partial specialization with a constant parameter waits for
            // a later piece.
            if params.iter().any(|p| p.2.is_some()) || cx.u.var_templates[v].kinds.iter().any(|k| k.is_some()) {
                return false;
            }
            let ts = cx.enter(ScopeKind::Template);
            declare_params(cx, ts, &params);
            let marks = cx.u.diags.len();
            let pattern = template_type_args(cx, tid);
            cx.leave(saved);
            let Some(pattern) = pattern.filter(|_| cx.u.diags.len() == marks) else {
                cx.u.diags.truncate(marks);
                return false;
            };
            if pattern.len() != cx.u.var_templates[v].params.len() {
                return false;
            }
            cx.u.var_templates[v].partials.push((params.into_iter().map(|p| p.0).collect(), pattern, sd.clone()));
            true
        }
    }
}

/// The type arguments of a template-id, or None when one is not a type.
fn template_type_args(cx: &mut Check, tid: &Node) -> Option<Vec<QType>> {
    let mut out = Vec::new();
    if let Some(ta) = tid.child("template_args") {
        for a in ta.children("template_arg") {
            let t = a.child("type_id")?;
            out.push(crate::declarations::declarators::check::type_id(cx, t));
        }
    }
    Some(out)
}

/// Match a partial specialization's pattern type `p` against an argument
/// `a` exactly, binding its parameters (TSP-14).
fn match_exact(p: &QType, a: &QType, bound: &mut [Option<QType>]) -> bool {
    if !crate::templates::deduction::check::has_dep(p) {
        return p == a;
    }
    match (&p.t, &a.t) {
        (Type::Dep(d), _) => {
            let DepType::Param { index, .. } = &**d;
            if (p.c && !a.c) || (p.v && !a.v) {
                return false;
            }
            let t = a.with_cv(a.c && !p.c, a.v && !p.v);
            match &mut bound[*index as usize] {
                Some(x) => *x == t,
                slot => {
                    *slot = Some(t);
                    true
                }
            }
        }
        _ if p.c != a.c || p.v != a.v => false,
        (Type::Ptr(x), Type::Ptr(y)) => match_exact(x, y, bound),
        (Type::Ref(x, r), Type::Ref(y, s)) => r == s && match_exact(x, y, bound),
        (Type::Array(x, n), Type::Array(y, m)) => n == m && match_exact(x, y, bound),
        _ => false,
    }
}

/// TARG-1, TSP-13 to TSP-15: `v<A...>`, a specialization of a variable
/// template: the one partial specialization whose pattern the arguments
/// match, or the primary. None when the name is no variable template.
pub fn var_template_use(cx: &mut Check, tid: &Node) -> Option<crate::sem::tree::Expr> {
    use crate::sem::tree::{Cat, EK, Expr};
    let span = tid.span;
    let name = cx.source(tid.child("template_name")?);
    let v = names::lookup(cx, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::VarTemplate(v) => Some(v),
        _ => None,
    })?;
    let unsupported = |cx: &mut Check, why: &str| {
        let t = cx.unsupported(span, why.to_string());
        Some(Expr::new(EK::Int(0), t, Cat::PR, span))
    };
    let vt = cx.u.var_templates[v].clone();
    let marks = cx.u.diags.len();
    let Some(args) = template_args(cx, tid, &vt.kinds) else {
        if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
            return Some(crate::expressions::check::bad(span));
        }
        return unsupported(cx, "a variable template with arguments of another number or form");
    };
    if args.iter().any(|a| matches!(a, TArg::Type(t) if t.is_error())) {
        return Some(crate::expressions::check::bad(span));
    }
    if args.iter().any(|a| matches!(a, TArg::Type(t) if crate::templates::deduction::check::has_dep(t))) {
        return unsupported(cx, "a variable template with a dependent argument");
    }
    let types: Vec<QType> = args.iter().filter_map(|a| match a {
        TArg::Type(t) => Some(t.clone()),
        TArg::Value(..) => None,
    }).collect();
    let mut matches = Vec::new();
    for (k, (ps, pattern, _)) in vt.partials.iter().enumerate() {
        let mut bound: Vec<Option<QType>> = vec![None; ps.len()];
        if types.len() == args.len() && pattern.iter().zip(&types).all(|(p, a)| match_exact(p, a, &mut bound)) {
            if let Some(b) = bound.into_iter().map(|b| b.map(TArg::Type)).collect::<Option<Vec<TArg>>>() {
                matches.push((k, b));
            }
        }
    }
    if matches.len() > 1 {
        return unsupported(cx, "a variable template whose arguments match several partial specializations (partial ordering)");
    }
    let (source, bound) = match matches.pop() {
        Some((k, b)) => (Some(k), b),
        None => (None, args.clone()),
    };
    let var = match vt.specs.iter().find(|(s, a, _)| *s == source && *a == bound) {
        Some(&(_, _, id)) => id,
        None => {
            let (params, node) = match source {
                Some(k) => (vt.partials[k].0.clone(), vt.partials[k].2.clone()),
                None => (vt.params.clone(), vt.node.clone()),
            };
            match instantiate_var(cx, v, &params, &bound, &node) {
                Some(id) => {
                    cx.u.var_templates[v].specs.push((source, bound, id));
                    id
                }
                None => return unsupported(cx, "a specialization of a variable template that the first pieces of build step 5 do not make"),
            }
        }
    };
    crate::declarations::initialization::forms::check::note_use(cx, var, span);
    let ty = cx.u.var(var)?.ty.clone();
    Some(Expr::new(EK::Var(var), ty, Cat::L, span))
}

/// The variable of a specialization: the declaration `node` checked with
/// each of `params` bound to its argument in `args`. An error in it
/// leaves the unit unsupported, as for a function template.
fn instantiate_var(cx: &mut Check, v: usize, params: &[Rc<str>], args: &[TArg], node: &Node) -> Option<EntityId> {
    let vt = cx.u.var_templates[v].clone();
    let (ds, _, init, _) = var_parts(node)?;
    let saved_scope = cx.scope;
    let saved_func = cx.func.take();
    let saved_member = cx.member_of.take();
    let saved_lambdas = std::mem::take(&mut cx.lambdas);
    let saved_unevaluated = std::mem::replace(&mut cx.unevaluated, 0);
    let saved_limit = cx.lookup_limit.replace(vt.limit);
    cx.instantiating += 1;
    let marks = cx.u.diags.len();
    cx.scope = vt.scope;
    let ts = cx.enter(ScopeKind::Template);
    bind_params(cx, ts, params, args, vt.span);
    let r = (|| {
        let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
        if specs.unsupported || specs.placeholder.is_some() {
            return None;
        }
        let ty = specs.ty.clone()?;
        let ty = if specs.constexpr { ty.with_cv(true, ty.v) } else { ty };
        let i = crate::declarations::initialization::forms::check::initializer(cx, &ty.unqual(), init, &vt.name)?;
        let var = crate::sem::Var { ty, storage: crate::sem::Storage::Static, constexpr: specs.constexpr, constinit: false, inline: true, is_extern: false, is_static: false, param: false, defined: true, init: Some(i), const_init: specs.constexpr, used: false, maybe_unused: false, align: None };
        let id = cx.u.add(crate::sem::Entity { name: vt.name.clone(), kind: EntKind::Var(var), span: vt.span, scope: ts, linkage: Linkage::External, deprecated: None });
        cx.u.statics.push(id);
        Some(id)
    })();
    cx.scope = saved_scope;
    cx.func = saved_func;
    cx.member_of = saved_member;
    cx.lambdas = saved_lambdas;
    cx.unevaluated = saved_unevaluated;
    cx.lookup_limit = saved_limit;
    cx.instantiating -= 1;
    if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error || d.sev == crate::diag::Severity::Unsupported) {
        let tail = cx.u.diags.split_off(marks);
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev != crate::diag::Severity::Error));
        return None;
    }
    r
}

/// TPL rules: a class template definition at namespace scope whose
/// parameters are types. False for another form, such as a declaration
/// that is not a definition.
fn class_template_decl(cx: &mut Check, sd: &Node, params: Vec<Param>) -> bool {
    let Some(ds) = sd.child("decl_specifiers") else { return false };
    let specs: Vec<&Node> = ds.children("decl_specifier").collect();
    let [one] = specs[..] else { return false };
    let Some(cd) = one.child("type_specifier").and_then(|t| t.child("class_def").or_else(|| t.child("union_def"))) else { return false };
    // A `union_def` is its own head (`classes/unions`).
    let head = if cd.prod == "union_def" { cd } else {
        let Some(h) = cd.child("class_head") else { return false };
        h
    };
    if head.child("nested_name").is_some() || head.child("template_id").is_some() {
        return false;
    }
    let Some(name) = head.child("class_name").map(|c| cx.source(c)) else { return false };
    let saved = cx.scope;
    if !names::declared_here(cx, saved, &name).is_empty() || dependent_forms(cx, cd, &params) {
        return false;
    }
    // LSUP rules: `std::initializer_list`, which the language builds
    // braced lists into (ILC rules), and `std::allocator`, whose
    // allocation constant evaluation allows (LSUP-11), are classes that
    // the first pieces of build step 5 do not model.
    if matches!(&*name, "initializer_list" | "allocator") && cx.u.scopes[saved].name.as_ref() == "std" {
        return false;
    }
    let id = cx.u.class_templates.len();
    let limit = cx.u.entities.len() + 1;
    cx.u.class_templates.push(crate::sem::ClassTemplate { name: name.as_str().into(), kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node: cd.clone(), scope: saved, specs: Vec::new(), span: head.span, limit });
    names::declare_in(cx, saved, &name, EntKind::ClassTemplate(id), head.span, Linkage::External);
    true
}

/// TARG-1: `C<A...>`, the class of a specialization of a class template
/// with type arguments, made the first time. None when the name is no
/// class template.
pub fn class_template_use(cx: &mut Check, tid: &Node) -> Option<QType> {
    class_template_use_in(cx, tid, None)
}

/// `class_template_use` for a template-id after the qualifier that names
/// the scope `scope`.
pub fn class_template_use_in(cx: &mut Check, tid: &Node, scope: Option<crate::sem::ScopeId>) -> Option<QType> {
    let span = tid.span;
    let name = cx.source(tid.child("template_name")?);
    let found = match scope {
        Some(s) => names::lookup_in(cx, s, &name),
        None => names::lookup(cx, &name),
    };
    // Inside a specialization, the class's own name finds the class,
    // which leads to its template (DEP-15).
    let v = found.into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::ClassTemplate(v) => Some(v),
        EntKind::Class(c) => cx.u.class_templates.iter().position(|t| t.specs.iter().any(|&(_, k)| k == c)),
        _ => None,
    })?;
    let ct = cx.u.class_templates[v].clone();
    let marks = cx.u.diags.len();
    let Some(args) = template_args(cx, tid, &ct.kinds) else {
        if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
            return Some(QType::error());
        }
        return Some(cx.unsupported(span, "a class template with arguments of another number or form"));
    };
    if args.iter().any(|a| matches!(a, TArg::Type(t) if t.is_error())) {
        return Some(QType::error());
    }
    if args.iter().any(|a| matches!(a, TArg::Type(t) if crate::templates::deduction::check::has_dep(t))) {
        return Some(cx.unsupported(span, "a class template with a dependent argument"));
    }
    if let Some(&(_, c)) = ct.specs.iter().find(|(a, _)| *a == args) {
        return Some(QType::new(Type::Class(c)));
    }
    // INST rules: a member declared with a parameter that a function type
    // binds is a data member of function type, which the first pieces of
    // build step 5 do not tell from a member function.
    if args.iter().any(|a| matches!(a, TArg::Type(t) if t.is_func())) {
        return Some(cx.unsupported(span, "a class template with a function type as an argument"));
    }
    match instantiate_class(cx, v, &args) {
        Some(c) => Some(QType::new(Type::Class(c))),
        None => Some(cx.unsupported(span, format!("a specialization of the class template '{}' that the first pieces of build step 5 do not make", name))),
    }
}

/// The class of a specialization: the definition checked with each
/// parameter bound to its argument, as a class of the template's
/// namespace. The class is recorded before its body, so the body finds
/// it by the template-id. An error in it leaves the unit unsupported.
fn instantiate_class(cx: &mut Check, v: usize, args: &[TArg]) -> Option<crate::sem::types::ClassId> {
    let ct = cx.u.class_templates[v].clone();
    let saved_scope = cx.scope;
    let saved_func = cx.func.take();
    let saved_member = cx.member_of.take();
    let saved_lambdas = std::mem::take(&mut cx.lambdas);
    let saved_unevaluated = std::mem::replace(&mut cx.unevaluated, 0);
    let saved_default = std::mem::replace(&mut cx.default_arg, false);
    let saved_access = cx.access_class.take();
    let saved_limit = cx.lookup_limit.replace(ct.limit);
    cx.instantiating += 1;
    let marks = cx.u.diags.len();
    cx.scope = ct.scope;
    let ts = cx.enter(ScopeKind::Template);
    bind_params(cx, ts, &ct.params, args, ct.span);
    // The class that `class_def` makes first is the specialization.
    let predicted = cx.u.classes.len();
    cx.u.class_templates[v].specs.push((args.to_vec(), predicted));
    let t = crate::classes::members::check::class_def(cx, &ct.node);
    cx.scope = saved_scope;
    cx.func = saved_func;
    cx.member_of = saved_member;
    cx.lambdas = saved_lambdas;
    cx.unevaluated = saved_unevaluated;
    cx.default_arg = saved_default;
    cx.access_class = saved_access;
    cx.lookup_limit = saved_limit;
    cx.instantiating -= 1;
    let bad = cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error || d.sev == crate::diag::Severity::Unsupported);
    if t.class() != Some(predicted) || bad {
        cx.u.class_templates[v].specs.retain(|(a, _)| a.as_slice() != args);
        let tail = cx.u.diags.split_off(marks);
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev != crate::diag::Severity::Error));
        return None;
    }
    Some(predicted)
}
