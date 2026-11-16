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
pub type Param = (Rc<str>, Span, Option<QType>);

/// The constraints of a template head: its requires clause (None), and
/// the type constraint of each parameter that has one (its index).
pub type Constraints = Vec<(Option<usize>, Node)>;

/// The parameters of a template head whose parameters are type
/// parameters and constant parameters of integral or enumeration type,
/// without a pack, with the default of each (TPL-13) and the head's
/// constraints (CPT-8, CPT-17); None for another head.
pub fn type_params(cx: &mut Check, head: &Node) -> Option<(Vec<Param>, Vec<Option<Node>>, Constraints, bool)> {
    let mut constraints: Constraints = Vec::new();
    // PACK rules: a type parameter pack as the last parameter.
    let mut pack = false;
    let mut out = Vec::new();
    let mut defaults = Vec::new();
    for tp in head.child("template_params")?.children("template_param") {
        // TARG-13: a constant parameter of integral or enumeration type.
        if let Some(p) = tp.child("constant_param").and_then(|c| c.child("parameter")).filter(|_| !pack) {
            // PACK rules: a constant parameter pack, which has no default.
            let mut toks = Vec::new();
            p.tokens(&mut toks);
            if p.find("ptr_part").is_some() {
                return None;
            }
            if toks.iter().any(|&t| cx.toks[t].text == "...") {
                if cx.words(p).contains(&"=") {
                    return None;
                }
                pack = true;
            }
            // The default is the expression after `=`.
            let default = match cx.words(p).contains(&"=") {
                true => Some(p.kids.iter().rev().find_map(|k| match k {
                    Kid::Node(c) => Some(c.clone()),
                    _ => None,
                })?),
                false => None,
            };
            let ds = p.child("decl_specifiers")?;
            // TPL-33: no storage class, and no class or enumeration defined.
            if let Some(d) = ds.children("decl_specifier").find(|d| matches!(cx.source(d).as_str(), "static" | "extern" | "thread_local" | "register" | "mutable")) {
                cx.error(d.span, "TPL-33", format!("the storage class '{}' on a constant template parameter", cx.source(d)));
                return None;
            }
            if ds.find("enum_def").is_some() || ds.find("class_def").is_some() || ds.find("union_def").is_some() {
                cx.error(ds.span, "TPL-33", "a constant template parameter that defines a class or an enumeration");
                return None;
            }
            let marks = cx.u.diags.len();
            let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
            let ty = specs.ty.clone().filter(|t| (t.is_integral() || t.is_enum()) && cx.u.diags.len() == marks && !specs.unsupported);
            let Some(ty) = ty else {
                cx.u.diags.truncate(marks);
                return None;
            };
            let id = p.find("declarator_id")?;
            out.push((cx.source(id).as_str().into(), id.span, Some(ty.unqual())));
            defaults.push(default);
            continue;
        }
        let t = tp.child("type_param")?;
        if pack {
            return None;
        }
        if t.kids.iter().any(|k| matches!(k, Kid::Tok(_)) && cx.text(k) == "...") {
            // TPL-34: a pack has no default.
            if t.child("type_id").is_some() {
                return None;
            }
            pack = true;
        }
        match (t.child("type_key"), t.child("type_constraint")) {
            (Some(_), None) => {}
            (None, Some(tc)) => constraints.push((Some(out.len()), tc.clone())),
            _ => return None,
        }
        let k = t.kids.iter().rev().find(|k| matches!(k, Kid::Tok(_)) && cx.text(k) != "=")?;
        let name: Rc<str> = cx.text(k).into();
        if name.as_ref() == "class" || name.as_ref() == "typename" {
            return None;
        }
        out.push((name, cx.token(k).map(|t| t.span).unwrap_or(t.span), None));
        defaults.push(t.child("type_id").cloned());
    }
    if let Some(rc) = head.child("requires_clause") {
        constraints.push((None, rc.clone()));
    }
    if pack && !constraints.is_empty() {
        return None;
    }
    Some((out, defaults, constraints, pack))
}

/// TPL-14: unless the template is a function template, each parameter
/// after one with a default has a default.
fn defaults_last(cx: &mut Check, defaults: &[Option<Node>], span: Span) -> bool {
    if let Some(k) = defaults.iter().position(|d| d.is_some()) {
        if defaults[k..].iter().any(|d| d.is_none()) {
            cx.error(span, "TPL-14", "a template parameter without a default after one with a default");
            return false;
        }
    }
    true
}

/// TPL-15, TPL-16: the defaults of a later declaration add to those of an
/// earlier one, and the two do not both give one parameter a default.
fn merge_defaults(cx: &mut Check, earlier: &[Option<Node>], later: &[Option<Node>], span: Span) -> Option<Vec<Option<Node>>> {
    if earlier.iter().zip(later).any(|(a, b)| a.is_some() && b.is_some()) {
        cx.error(span, "TPL-16", "a default template argument given again in a later declaration");
        return None;
    }
    Some(earlier.iter().zip(later).map(|(a, b)| a.clone().or_else(|| b.clone())).collect())
}

/// The arguments of a template-id, with the defaults of the template for
/// the parameters after them (TPL-13). Each default sees the parameters
/// before it bound to their arguments.
pub fn args_with_defaults(cx: &mut Check, tid: &Node, kinds: &[Option<QType>], defaults: &[Option<Node>], names_: &[Rc<str>], scope: crate::sem::ScopeId, limit: EntityId, pack: bool) -> Option<Vec<TArg>> {
    let n = tid.child("template_args").map(|ta| ta.children("template_arg").count()).unwrap_or(0);
    // An expansion gives as many arguments as its pack has elements.
    let expands = tid.child("template_args").is_some_and(|ta| cx.words(ta).contains(&"..."));
    if pack || n >= kinds.len() || expands {
        return template_args_packed(cx, tid, kinds, pack);
    }
    let mut args = template_args(cx, tid, &kinds[..n])?;
    fill_defaults(cx, kinds, defaults, names_, scope, limit, &mut args, tid.span).then_some(args)
}

/// Fill `args` up to the parameters' number from their defaults.
fn fill_defaults(cx: &mut Check, kinds: &[Option<QType>], defaults: &[Option<Node>], names_: &[Rc<str>], scope: crate::sem::ScopeId, limit: EntityId, args: &mut Vec<TArg>, span: Span) -> bool {
    while args.len() < kinds.len() {
        let k = args.len();
        let Some(Some(d)) = defaults.get(k).cloned() else { return false };
        let saved_scope = cx.scope;
        let saved_limit = cx.lookup_limit.replace(limit);
        cx.scope = scope;
        let ts = cx.enter(ScopeKind::Template);
        bind_params(cx, ts, &names_[..k], args, span);
        let a = match &kinds[k] {
            None => Some(TArg::Type(crate::declarations::declarators::check::type_id(cx, &d))),
            Some(ty) => {
                let e = crate::expressions::check::expr(cx, &d);
                let e = crate::foundations::values::check::rvalue(cx, e);
                let v = (e.ty.is_integral() || e.ty.is_enum()).then(|| crate::foundations::constant_evaluation::check::try_value(cx, &e).and_then(|v| v.int())).flatten();
                v.and_then(|v| constant_of(cx, &e, v, ty, d.span))
            }
        };
        cx.scope = saved_scope;
        cx.lookup_limit = saved_limit;
        match a {
            Some(a) => args.push(a),
            None => return false,
        }
    }
    true
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
pub fn bind_params(cx: &mut Check, ts: crate::sem::ScopeId, names_: &[Rc<str>], args: &[TArg], span: Span) {
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
            // PACK rules: a type pack names its types, and a constant pack
            // names one constant for each value.
            TArg::Pack(xs) if xs.iter().any(|x| matches!(x, TArg::Value(..))) => {
                let mut ids = Vec::new();
                for x in xs {
                    let TArg::Value(t, v) = x else { continue };
                    let init = crate::sem::tree::Init::Expr(crate::sem::tree::Expr::int(*v, t.clone(), span));
                    let var = crate::sem::Var { ty: t.with_cv(true, false), storage: crate::sem::Storage::Static, constexpr: true, constinit: false, inline: true, is_extern: false, is_static: false, param: false, defined: true, init: Some(init), const_init: true, used: true, maybe_unused: true, align: None };
                    // The element has no name of its own, so the name finds the pack.
                    let id = cx.u.add(crate::sem::Entity { name: "".into(), kind: EntKind::Var(var), span, scope: ts, linkage: Linkage::None, deprecated: None });
                    cx.template_constants.insert(id);
                    ids.push(id);
                }
                names::declare_in(cx, ts, name, EntKind::Pack(ids), span, Linkage::None);
            }
            TArg::Pack(xs) => {
                names::declare_in(cx, ts, name, EntKind::TypePack(TArg::types(xs)), span, Linkage::None);
            }
        }
    }
}

/// The arguments of a template-id for parameters of the kinds `kinds`: a
/// type for a type parameter, and a constant for a constant parameter,
/// converted to its type. None for another number or form of arguments.
pub fn template_args(cx: &mut Check, tid: &Node, kinds: &[Option<QType>]) -> Option<Vec<TArg>> {
    template_args_packed(cx, tid, kinds, false)
}

/// `template_args` for parameters whose last is a pack when `pack`
/// holds: the arguments after the others form the pack (PACK rules). An
/// argument `P...` expands to one argument for each element of the
/// packs that its pattern names.
pub fn template_args_packed(cx: &mut Check, tid: &Node, kinds: &[Option<QType>], pack: bool) -> Option<Vec<TArg>> {
    // Each argument node, and whether a `...` follows it.
    let nodes: Vec<(&Node, bool)> = tid.child("template_args").map(|ta| ta.kids.iter().enumerate().filter_map(|(k, kid)| match kid {
        Kid::Node(a) if a.prod == "template_arg" => Some((a, matches!(ta.kids.get(k + 1), Some(t @ Kid::Tok(_)) if cx.text(t) == "..."))),
        _ => None,
    }).collect()).unwrap_or_default();
    // Each argument, with the pack elements its pattern takes: a type, or
    // the constant of a constant pack.
    let mut items: Vec<(&Node, Vec<(Rc<str>, Result<QType, EntityId>)>)> = Vec::new();
    for (a, followed) in nodes {
        let expansion = followed || a.child("type_id").and_then(|t| t.child("abstract_declarator")).is_some_and(|d| d.kids.iter().any(|k| matches!(k, Kid::Tok(_)) && cx.text(k) == "..."));
        if !expansion {
            if a.find("abstract_declarator").is_some_and(|d| cx.words(d).contains(&"...")) || cx.words(a).contains(&"...") {
                return None;
            }
            items.push((a, Vec::new()));
            continue;
        }
        let mut toks = Vec::new();
        a.tokens(&mut toks);
        let mut packs: Vec<(Rc<str>, Vec<Result<QType, EntityId>>)> = Vec::new();
        for &t in &toks {
            let w = cx.toks[t].text.as_str();
            if packs.iter().any(|(n, _)| n.as_ref() == w) {
                continue;
            }
            match names::lookup(cx, w).first().map(|&e| cx.u.entities[e].kind.clone()) {
                Some(EntKind::TypePack(ts)) => packs.push((w.into(), ts.into_iter().map(Ok).collect())),
                Some(EntKind::Pack(ids)) => packs.push((w.into(), ids.into_iter().map(Err).collect())),
                _ => {}
            }
        }
        // PACK-8: the pattern names a pack, and the packs have one length.
        let Some(n) = packs.first().map(|p| p.1.len()) else { return None };
        if packs.iter().any(|p| p.1.len() != n) {
            return None;
        }
        for i in 0..n {
            items.push((a, packs.iter().map(|(name, ts)| (name.clone(), ts[i].clone())).collect()));
        }
    }
    let fixed = if pack { kinds.len().checked_sub(1)? } else { kinds.len() };
    if items.len() < fixed || (!pack && items.len() != kinds.len()) {
        return None;
    }
    let mut out = Vec::new();
    for (k, (a, binds)) in items.into_iter().enumerate() {
        let kind = if k < fixed { &kinds[k] } else { kinds.last()? };
        let saved = cx.scope;
        let saved_expanding = cx.expanding;
        if !binds.is_empty() {
            let s = cx.enter(ScopeKind::Template);
            for (name, b) in &binds {
                match b {
                    Ok(t) => {
                        names::declare_in(cx, s, name, EntKind::Typedef(t.clone()), a.span, Linkage::None);
                    }
                    Err(id) => cx.u.scopes[s].names.entry(name.clone()).or_default().push(*id),
                }
            }
            cx.expanding = true;
        }
        let arg = match kind {
            None => a.child("type_id").map(|t| TArg::Type(crate::declarations::declarators::check::type_id(cx, t))),
            Some(ty) => constant_arg(cx, a).and_then(|(e, v)| constant_of(cx, &e, v, ty, a.span)),
        };
        cx.leave(saved);
        cx.expanding = saved_expanding;
        out.push(arg?);
    }
    if pack {
        let rest = out.split_off(fixed);
        out.push(TArg::Pack(rest));
    }
    Some(out)
}

/// TARG-13, CE-19: the constant `e` of value `v` as an argument for a
/// constant parameter of type `ty`. It converts as a converted constant
/// expression: an enumeration only from itself, and no narrowing.
fn constant_of(cx: &mut Check, e: &crate::sem::tree::Expr, v: i128, ty: &QType, span: Span) -> Option<TArg> {
    let scoped = matches!(ty.t, Type::Enum(en) if cx.u.enums[en].scoped);
    let other_enum = (ty.is_enum() || matches!(e.ty.t, Type::Enum(en) if cx.u.enums[en].scoped)) && e.ty.unqual().t != ty.t;
    if (scoped || other_enum) && e.ty.unqual().t != ty.t || crate::foundations::conversions::check::is_narrowing(cx, e, ty) {
        cx.error_cause(span, "TARG-13", Some("CE-19"), format!("the template argument of type {} does not convert to {} as a constant", e.ty.show(), ty.show()));
        return None;
    }
    let v = match ty.int_kind() {
        Some(k) => cx.u.target.wrap(k, v),
        None => v,
    };
    Some(TArg::Value(ty.clone(), v))
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
    if c_linkage(cx, n) {
        return true;
    }
    if fold_expanded(cx, n) {
        return true;
    }
    let Some((params, defaults, constraints, pack)) = n.child("template_head").and_then(|h| type_params(cx, h)) else { return false };
    cx.head_pack = pack;
    // Constraints wait for a later piece outside a function template.
    let fn_decl = n.child("declaration").is_some_and(|d| d.child("function_definition").is_some() || d.child("block_decl").and_then(|b| b.child("simple_decl")).is_some_and(|sd| sd.children("init_declarator").any(|i| i.child("init").is_none() && i.child("declarator").is_some_and(|d| d.find("function_suffix").is_some()))));
    // CNS-13: a class template's constraints are checked at each
    // template-id.
    let class_decl = n.child("declaration").and_then(|d| d.child("block_decl")).and_then(|b| b.child("simple_decl")).is_some_and(|sd| sd.child("init_declarator").is_none());
    if !constraints.is_empty() && !fn_decl && !class_decl {
        return false;
    }
    if let Some(done) = outside_member(cx, n, &params) {
        // TPL-17: a member defined after the class body gives no default.
        if done && defaults.iter().any(|d| d.is_some()) {
            cx.error(n.span, "TPL-17", "a default template argument on a member defined outside its class template");
        }
        return done;
    }
    if let Some(ad) = n.child("declaration").and_then(|d| d.child("block_decl")).and_then(|b| b.child("alias_decl")) {
        return alias_template_decl(cx, ad, params, defaults);
    }
    if let Some(sd) = n.child("declaration").and_then(|d| d.child("block_decl")).and_then(|b| b.child("simple_decl")) {
        if sd.child("init_declarator").is_none() {
            return class_template_decl(cx, sd, params, defaults, constraints);
        }
        // `template <class T> R f(P);` declares a function template.
        let mut ids = sd.children("init_declarator");
        let one = ids.next().filter(|i| i.child("init").is_none() && i.child("declarator").is_some_and(|d| d.find("function_suffix").is_some()));
        if let (Some(one), None) = (one, ids.next()) {
            let (Some(ds), Some(dn)) = (sd.child("decl_specifiers"), one.child("declarator")) else { return false };
            return function_template(cx, sd, ds, dn, params, defaults, constraints);
        }
        return var_template_decl(cx, sd, params, defaults);
    }
    let Some(fd) = n.child("declaration").and_then(|d| d.child("function_definition")) else { return false };
    let (Some(ds), Some(dn)) = (fd.child("decl_specifiers"), fd.child("declarator")) else { return false };
    function_template(cx, fd, ds, dn, params, defaults, constraints)
}

/// TPL rules: a function template's declaration `fd`, a definition or
/// not, with its decl-specifiers `ds` and its declarator `dn`. A
/// definition after a declaration completes the template.
fn function_template(cx: &mut Check, fd: &Node, ds: &Node, dn: &Node, params: Vec<Param>, defaults: Vec<Option<Node>>, constraints: Constraints) -> bool {
    // CPT-11: a trailing requires clause adds to the head's constraints.
    let mut constraints = constraints;
    if let Some(rc) = fd.child("requires_clause") {
        constraints.push((None, rc.clone()));
    }
    for (k, c) in &constraints {
        if k.is_none() && !crate::templates::concepts::check::definition_checks(cx, c, &params) {
            return true;
        }
    }
    if shadows_param(cx, fd, &params) || expands_no_pack(cx, fd) {
        return true;
    }
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
    // A deduced return type (`auto`) is deduced in each specialization;
    // `decltype(auto)` and a constrained placeholder wait for a later
    // piece.
    let placeholder_ok = d.placeholder.as_ref().is_none_or(|p| !p.decltype);
    if specs.unsupported || d.unsupported || d.qual != names::Qual::None || cx.u.diags.len() > marks || !placeholder_ok {
        cx.u.diags.truncate(marks);
        return false;
    }
    // TPL-29: `main` is no template.
    if name.as_ref() == "main" && saved == cx.u.root {
        cx.error(span, "TPL-29", "the function 'main' declared as a template");
        return true;
    }
    // PACK rules: a function parameter pack as the last parameter, whose
    // pattern names the template's pack.
    let pack_param = d.params.last().is_some_and(|p| p.pack.is_some());
    if d.params.iter().rev().skip(1).any(|p| p.pack.is_some()) || (pack_param && (!cx.head_pack || fty.variadic)) {
        return false;
    }
    // An operator function template beside another of its name waits for
    // a later piece.
    if (name.starts_with("operator") || dn.find("operator_function_id").is_some() || dn.find("literal_operator_id").is_some()) && !names::declared_here(cx, saved, &name).is_empty() {
        return false;
    }
    // The definition checks of model section 5 wait for a later piece, so
    // a definition with a dependent construct that the first piece does
    // not model, or a name another function or template shares, stays
    // unsupported.
    if dependent_forms(cx, fd, &params) {
        return false;
    }
    let found = names::declared_here(cx, saved, &name);
    // Functions and function templates of the name overload; another
    // entity of the name waits for a later piece.
    if found.iter().any(|&e| !matches!(cx.u.entities[e].kind, EntKind::Template(_) | EntKind::Func(_))) {
        return false;
    }
    let kinds: Vec<Option<QType>> = params.iter().map(|p| p.2.clone()).collect();
    let same = found.iter().find_map(|&e| match cx.u.entities[e].kind {
        EntKind::Template(t) if *cx.u.templates[t].fty == fty && cx.u.templates[t].kinds == kinds && cx.u.templates[t].pack_param == pack_param => Some(e),
        _ => None,
    });
    if let Some(e) = same {
        // A second declaration of the template, which a definition
        // completes. Constraints whose equivalence decides between a
        // redeclaration and an overload wait for a later piece.
        let EntKind::Template(t) = cx.u.entities[e].kind else { return false };
        let tpl = &cx.u.templates[t];
        if !constraints.is_empty() || !tpl.constraints.is_empty() {
            return false;
        }
        // DEFARG-13: a later declaration adds no default argument.
        if let Some(p) = dn.find("parameters").and_then(|ps| ps.children("parameter").find(|p| cx.has_word(p, "="))) {
            cx.error(p.span, "DEFARG-13", "a default argument in a later declaration of a function template");
            return true;
        }
        let earlier = tpl.defaults.clone();
        let Some(merged) = merge_defaults(cx, &earlier, &defaults, span) else { return true };
        cx.u.templates[t].defaults = merged;
        let tpl = &cx.u.templates[t];
        if fd.prod != "function_definition" {
            return true;
        }
        if tpl.node.prod == "function_definition" {
            return false;
        }
        let limit = cx.u.entities.len() + 1;
        let tpl = &mut cx.u.templates[t];
        tpl.params = params.into_iter().map(|p| p.0).collect();
        tpl.node = fd.clone();
        tpl.span = span;
        tpl.limit = limit;
        // The specializations that uses made before the definition get
        // their bodies now.
        let made: Vec<(Vec<TArg>, EntityId)> = tpl.specs.iter().filter(|(_, f)| !tpl.explicit.contains(f)).cloned().collect();
        for (args, f) in made {
            define_later(cx, t, &args, f);
        }
        return true;
    }
    let id = cx.u.templates.len();
    let limit = cx.u.entities.len() + 1;
    cx.u.templates.push(FnTemplate { name: name.clone(), pack: cx.head_pack, defaults: defaults.clone(), kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node: fd.clone(), scope: saved, fty: Rc::new(fty), pack_param, pack_name: d.params.last().and_then(|p| p.pack.clone()), abbrev: cx.abbrev_count, specs: Vec::new(), explicit: Vec::new(), class: None, constraints, span, limit });
    names::declare_in(cx, saved, &name, EntKind::Template(id), span, Linkage::External);
    true
}

/// TPL-3: a template or a specialization in a C linkage specification.
fn c_linkage(cx: &mut Check, n: &Node) -> bool {
    if cx.lang_c {
        cx.error(n.span, "TPL-3", "a template or a specialization with C language linkage");
    }
    cx.lang_c
}

/// PACK-8: in a function body of a template with no pack, `x...` after a
/// name or a parenthesized expression expands no pack.
fn expands_no_pack(cx: &mut Check, fd: &Node) -> bool {
    if cx.head_pack {
        return false;
    }
    let Some(body) = fd.find("compound_stmt") else { return false };
    let mut toks = Vec::new();
    body.tokens(&mut toks);
    // A generic lambda's pack (`auto...`) can be expanded.
    if toks.windows(3).any(|w| cx.toks[w[0]].text == "auto" && (cx.toks[w[1]].text == "..." || cx.toks[w[2]].text == "...")) {
        return false;
    }
    for w in toks.windows(2) {
        let (prev, t) = (cx.toks[w[0]].text.as_str(), cx.toks[w[1]].text.as_str());
        let name = is_word(prev) && !crate::foundations::types::check::is_fundamental(prev) && !matches!(prev, "sizeof" | "auto" | "const" | "volatile");
        if t == "..." && (name || prev == ")") {
            let span = cx.toks[w[1]].span;
            cx.error(span, "PACK-8", "an expansion whose pattern holds no pack");
            return true;
        }
    }
    false
}

/// PACK-8: `( ... )...` whose parenthesized pattern is a fold, which
/// expands its pack itself, so the outer expansion holds no pack.
fn fold_expanded(cx: &mut Check, n: &Node) -> bool {
    let Some(d) = n.child("declaration") else { return false };
    let mut toks = Vec::new();
    d.tokens(&mut toks);
    let w: Vec<&str> = toks.iter().map(|&t| cx.toks[t].text.as_str()).collect();
    for i in 1..w.len() {
        if w[i] != "..." || w[i - 1] != ")" {
            continue;
        }
        // The group that the `)` closes.
        let mut depth = 0;
        let mut j = i - 1;
        loop {
            match w[j] {
                ")" => depth += 1,
                "(" => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
            if j == 0 {
                return false;
            }
            j -= 1;
        }
        // A fold: `...` beside an operator at the group's top level, with
        // no call or cast before the group.
        let inner = &w[j + 1..i - 1];
        let call = j > 0 && (is_word(w[j - 1]) || matches!(w[j - 1], ">" | ")" | "]"));
        let mut depth = 0;
        let top_fold = inner.iter().enumerate().any(|(k, t)| {
            match *t {
                "(" | "[" | "{" => depth += 1,
                ")" | "]" | "}" => depth -= 1,
                _ => {}
            }
            depth == 0 && *t == "..." && (k > 0 && !is_word(inner[k - 1]) && !matches!(inner[k - 1], ")" | ">" | "]") || k + 1 < inner.len() && !is_word(inner[k + 1]) && !matches!(inner[k + 1], "(" | ")"))
        });
        if !call && top_fold {
            let span = cx.toks[toks[i]].span;
            cx.error(span, "PACK-8", "an expansion whose pattern is a fold, which holds no unexpanded pack");
            return true;
        }
    }
    false
}

/// TPL-18: a declaration in the template of a name that one of its
/// parameters has. A friend function can have the name (TPL-48).
fn shadows_param(cx: &mut Check, n: &Node, params: &[Param]) -> bool {
    fn walk<'n>(n: &'n Node, out: &mut Vec<&'n Node>) {
        for k in &n.kids {
            let Kid::Node(c) = k else { continue };
            match c.prod {
                "declarator_id" => out.push(c),
                "friend_decl" => {}
                _ => walk(c, out),
            }
        }
    }
    let mut ids = Vec::new();
    walk(n, &mut ids);
    let hit = ids.into_iter().find(|d| params.iter().any(|p| p.0.as_ref() == cx.source(d)));
    if let Some(d) = hit {
        cx.error(d.span, "TPL-18", format!("'{}' declared again in the scope of the template parameter of that name", cx.source(d)));
        return true;
    }
    false
}

/// TSP rules: `template <> class C<A...> { ... };` or
/// `template <> T v<A...> = e;`, an explicit specialization of a class
/// template or a variable template of the scope, before any use of those
/// arguments. False for another form, such as a function's.
pub fn explicit_specialization(cx: &mut Check, n: &Node) -> bool {
    if cx.func.is_some() || cx.member_of.is_some() {
        return false;
    }
    if c_linkage(cx, n) {
        return true;
    }
    if let Some(fd) = n.child("declaration").and_then(|d| d.child("function_definition")) {
        return explicit_function(cx, fd);
    }
    let Some(sd) = n.child("declaration").and_then(|d| d.child("block_decl")).and_then(|b| b.child("simple_decl")) else { return false };
    let saved = cx.scope;
    if sd.child("init_declarator").is_none() {
        let Some(ds) = sd.child("decl_specifiers") else { return false };
        let specs: Vec<&Node> = ds.children("decl_specifier").collect();
        let [one] = specs[..] else { return false };
        let Some(cd) = one.child("type_specifier").and_then(|t| t.child("class_def").or_else(|| t.child("union_def"))) else { return false };
        // A `union_def` is its own head (`classes/unions`).
        let head = if cd.prod == "union_def" { cd } else {
            let Some(h) = cd.child("class_head") else { return false };
            h
        };
        // TSP-5, TSP-19: a qualified name names a template of an enclosed
        // namespace.
        let home = match head.child("nested_name") {
            None => saved,
            Some(nn) => match names::nested_name(cx, nn) {
                names::Qual::Scope(s) if cx.u.scopes[s].kind == ScopeKind::Namespace => {
                    let mut p = Some(s);
                    while p.is_some_and(|x| x != saved) {
                        p = cx.u.scopes[p.unwrap()].parent;
                    }
                    if p.is_none() {
                        cx.error(head.span, "TSP-5", "an explicit specialization outside a namespace that encloses its template");
                        return true;
                    }
                    s
                }
                names::Qual::Error => return true,
                _ => return false,
            },
        };
        let Some(tid) = head.child("template_id") else { return false };
        let Some(name) = tid.child("template_name").map(|t| cx.source(t)) else { return false };
        let Some(v) = names::declared_here(cx, home, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
            EntKind::ClassTemplate(v) => Some(v),
            _ => None,
        }) else {
            return false;
        };
        let kinds = cx.u.class_templates[v].kinds.clone();
        let marks = cx.u.diags.len();
        let Some(args) = template_args(cx, tid, &kinds).filter(|_| cx.u.diags.len() == marks) else {
            cx.u.diags.truncate(marks);
            return false;
        };
        // TSP-32: `union` agrees only with `union`.
        let primary = cx.u.class_templates[v].node.clone();
        if (primary.prod == "union_def") != (cd.prod == "union_def") {
            cx.error(head.span, "TSP-32", format!("an explicit specialization of '{}' with a class key that does not agree with the template's", name));
            return true;
        }
        // INST-19: no explicit specialization after an explicit
        // instantiation of it.
        if cx.explicit_insts.iter().any(|(k, w, a, _, _)| *k == 'c' && *w == v && *a == args) {
            cx.error(head.span, "INST-19", format!("an explicit specialization of '{}' after an explicit instantiation of it", name));
            return true;
        }
        let ct = &cx.u.class_templates[v];
        if ct.specs.iter().any(|(a, c)| *a == args && ct.explicit.contains(c)) {
            return false;
        }
        // TSP-6: a use before the explicit specialization made the class
        // from the primary template.
        if ct.specs.iter().any(|(a, _)| *a == args) {
            cx.error(head.span, "TSP-6", format!("an explicit specialization of '{}' after a use that made the specialization from the template", name));
            return true;
        }
        // CNS-13: the arguments satisfy the template's constraints.
        match crate::templates::concepts::check::class_satisfied(cx, v, &args) {
            Some(true) => {}
            Some(false) => {
                cx.error(head.span, "CNS-13", format!("an explicit specialization of '{}' whose arguments do not satisfy its constraints", name));
                return true;
            }
            None => return false,
        }
        let predicted = cx.u.classes.len();
        cx.u.class_templates[v].specs.push((args.clone(), predicted));
        cx.scope = home;
        cx.enter(ScopeKind::Template);
        cx.explicit_head = true;
        let t = crate::classes::members::check::class_def(cx, cd);
        cx.explicit_head = false;
        cx.leave(saved);
        match t.class() == Some(predicted) {
            true => cx.u.class_templates[v].explicit.push(predicted),
            false => cx.u.class_templates[v].specs.retain(|(a, _)| *a != args),
        }
        return true;
    }
    let Some((ds, dn, _, Some(tid))) = var_parts(sd) else { return false };
    if dn.find("ptr_operator").is_some() {
        return false;
    }
    // TSP-24: of the storage classes, only `thread_local`.
    if let Some(d) = ds.children("decl_specifier").find(|d| matches!(cx.source(d).as_str(), "static" | "extern" | "register" | "mutable")) {
        cx.error(d.span, "TSP-24", format!("the storage class '{}' on an explicit specialization", cx.source(d)));
        return true;
    }
    let Some(name) = tid.child("template_name").map(|t| cx.source(t)) else { return false };
    let Some(v) = names::declared_here(cx, saved, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::VarTemplate(v) => Some(v),
        _ => None,
    }) else {
        return false;
    };
    let kinds = cx.u.var_templates[v].kinds.clone();
    let marks = cx.u.diags.len();
    let Some(args) = template_args(cx, tid, &kinds).filter(|_| cx.u.diags.len() == marks) else {
        cx.u.diags.truncate(marks);
        return false;
    };
    if cx.explicit_insts.iter().any(|(k, w, a, _, _)| *k == 'v' && *w == v && *a == args) {
        cx.error(tid.span, "INST-19", format!("an explicit specialization of '{}' after an explicit instantiation of it", name));
        return true;
    }
    let vt = &cx.u.var_templates[v];
    if vt.explicit.iter().any(|(a, _)| *a == args) {
        return false;
    }
    if vt.specs.iter().any(|(s, a, _)| s.is_none() && *a == args) {
        cx.error(tid.span, "TSP-6", format!("an explicit specialization of '{}' after a use that made the specialization from the template", name));
        return true;
    }
    let Some(id) = instantiate_var(cx, v, &[], &[], sd) else { return false };
    cx.u.var_templates[v].explicit.push((args, id));
    true
}

/// INST-16 to INST-29: `template D;` or `extern template D;`, an explicit
/// instantiation definition or declaration of a class, a function, or a
/// variable of a template of the scope. A definition makes the
/// specialization. False for a form that waits for a later piece, such
/// as a declaration of a function or a member.
pub fn explicit_instantiation(cx: &mut Check, n: &Node) -> bool {
    let ext = cx.first_word(n) == "extern";
    // INST-21: at namespace scope.
    if cx.func.is_some() || cx.member_of.is_some() {
        cx.error(n.span, "INST-21", "an explicit instantiation outside namespace scope");
        return true;
    }
    let Some(sd) = n.child("declaration").and_then(|d| d.child("block_decl")).and_then(|b| b.child("simple_decl")) else { return false };
    let Some(ds) = sd.child("decl_specifiers") else { return false };
    // INST-25: no `inline`, `constexpr`, or `consteval`, no storage class
    // but `thread_local`, and no attribute.
    if sd.find("attribute_spec").is_some() {
        cx.error(n.span, "INST-25", "an attribute on an explicit instantiation");
        return true;
    }
    if let Some(d) = ds.children("decl_specifier").find(|d| matches!(cx.source(d).as_str(), "inline" | "constexpr" | "consteval" | "static" | "extern" | "register" | "mutable")) {
        cx.error(d.span, "INST-25", format!("'{}' on an explicit instantiation", cx.source(d)));
        return true;
    }
    let saved = cx.scope;
    let decls: Vec<&Node> = sd.children("init_declarator").collect();
    // A class: `template struct C<A...>;`.
    if decls.is_empty() {
        let Some(et) = ds.find("elaborated_type") else {
            // INST-23: a class has a class key.
            if ds.find("template_id").is_some() {
                cx.error(n.span, "INST-23", "an explicit instantiation of a class without a class key");
                return true;
            }
            return false;
        };
        let Some(tid) = et.child("template_id") else { return false };
        if et.child("nested_name").is_some() {
            return false;
        }
        let Some(name) = tid.child("template_name").map(|t| cx.source(t)) else { return false };
        let Some(v) = names::lookup(cx, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
            EntKind::ClassTemplate(v) => Some(v),
            _ => None,
        }) else {
            cx.error(n.span, "INST-22", format!("an explicit instantiation of '{}', which names no class template", name));
            return true;
        };
        let ct = cx.u.class_templates[v].clone();
        if ct.scope != saved {
            cx.error(n.span, "INST-21", format!("an explicit instantiation of '{}' outside its namespace with an unqualified name", name));
            return true;
        }
        // INST-23: `union` for a union template, `class` or `struct` for
        // another.
        let key_union = et.child("class_key").is_some_and(|k| cx.first_word(k) == "union");
        let primary_union = ct.node.prod == "union_def" || (ct.node.prod == "elaborated_type" && cx.first_word(ct.node.child("class_key").unwrap_or(&ct.node)) == "union");
        if key_union != primary_union {
            cx.error(n.span, "INST-23", format!("an explicit instantiation of '{}' with a class key that does not agree with the template's", name));
            return true;
        }
        // INST-20: the class template is defined before.
        if !ct.defined {
            cx.error(n.span, "INST-20", format!("an explicit instantiation of '{}', whose template is not defined", name));
            return true;
        }
        let marks = cx.u.diags.len();
        let Some(args) = args_with_defaults(cx, tid, &ct.kinds, &ct.defaults, &ct.params, ct.scope, ct.limit, ct.pack) else {
            if cx.u.diags.len() > marks {
                return true;
            }
            return false;
        };
        if !record_inst(cx, 'c', v, &args, !ext, n.span) {
            return true;
        }
        if ext {
            return false;
        }
        // INST-18: an explicit specialization stays the definition.
        let _ = class_template_use_in(cx, tid, None);
        return true;
    }
    // INST-23: one declarator, with no initializer.
    let [idecl] = decls[..] else {
        cx.error(n.span, "INST-23", "an explicit instantiation with more than one declarator");
        return true;
    };
    if idecl.child("init").is_some() {
        cx.error(n.span, "INST-23", "an explicit instantiation with an initializer");
        return true;
    }
    let Some(dn) = idecl.child("declarator") else { return false };
    let Some(did) = dn.find("declarator_id") else { return false };
    if did.find("nested_name").is_some() {
        return false;
    }
    let tid = did.find("template_id");
    let name = match tid {
        Some(tid) => tid.child("template_name").map(|t| cx.source(t)),
        None => did.find("unqualified_id").map(|u| cx.source(u)),
    };
    let Some(name) = name else { return false };
    let found = names::lookup(cx, &name);
    // A variable of a variable template.
    if dn.find("function_suffix").is_none() {
        let Some(tid) = tid else { return false };
        let Some(v) = found.iter().find_map(|&e| match cx.u.entities[e].kind {
            EntKind::VarTemplate(v) => Some(v),
            _ => None,
        }) else {
            cx.error(n.span, "INST-22", format!("an explicit instantiation of '{}', which names no variable template", name));
            return true;
        };
        let vt = cx.u.var_templates[v].clone();
        if vt.scope != saved {
            cx.error(n.span, "INST-21", format!("an explicit instantiation of '{}' outside its namespace with an unqualified name", name));
            return true;
        }
        let marks = cx.u.diags.len();
        let Some(args) = args_with_defaults(cx, tid, &vt.kinds, &vt.defaults, &vt.params, vt.scope, vt.limit, vt.pack) else {
            return cx.u.diags.len() > marks;
        };
        // INST-22: the declared type is the specialization's.
        let declared = {
            cx.explicit_head = true;
            let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
            let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
            cx.explicit_head = false;
            if specs.unsupported || d.unsupported || specs.placeholder.is_some() {
                return false;
            }
            d.ty.unqual()
        };
        if !record_inst(cx, 'v', v, &args, !ext, n.span) {
            return true;
        }
        let Some(e) = var_template_use(cx, tid) else { return false };
        if e.ty.is_error() {
            return true;
        }
        if declared.t != e.ty.unqual().t && !declared.is_error() {
            cx.error(n.span, "INST-22", format!("an explicit instantiation of '{}' with the type {}, not {}", name, declared.show(), e.ty.show()));
            return true;
        }
        return !ext;
    }
    // A function of a function template, whose type gives the arguments
    // that the template-id leaves out (DED-32).
    let templates: Vec<usize> = found.iter().filter_map(|&e| match cx.u.entities[e].kind {
        EntKind::Template(t) => Some(t),
        _ => None,
    }).collect();
    if templates.is_empty() {
        cx.error(n.span, "INST-22", format!("an explicit instantiation of '{}', which names no function template", name));
        return true;
    }
    // INST-21: outside the template's namespace, the name is qualified.
    if templates.iter().any(|&t| cx.u.templates[t].scope != saved) {
        cx.error(n.span, "INST-21", format!("an explicit instantiation of '{}' outside its namespace with an unqualified name", name));
        return true;
    }
    let marks = cx.u.diags.len();
    cx.enter(ScopeKind::Template);
    cx.explicit_head = true;
    let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    cx.explicit_head = false;
    cx.leave(saved);
    let Some(fty) = d.ty.func().cloned().filter(|_| !specs.unsupported && !d.unsupported && cx.u.diags.len() == marks) else {
        cx.u.diags.truncate(marks);
        return false;
    };
    let mut chosen: Vec<(usize, Vec<TArg>)> = Vec::new();
    for t in templates {
        let tpl = cx.u.templates[t].clone();
        if tpl.pack_param || !tpl.constraints.is_empty() {
            return false;
        }
        let explicit = match tid {
            Some(tid) => {
                let k = tid.child("template_args").map(|ta| ta.children("template_arg").count()).unwrap_or(0);
                match template_args(cx, tid, &tpl.kinds[..k.min(tpl.kinds.len())]) {
                    Some(a) if k <= tpl.kinds.len() => a,
                    _ => continue,
                }
            }
            None => Vec::new(),
        };
        let mut bound: Vec<Option<QType>> = (0..tpl.params.len()).map(|k| match explicit.get(k) {
            Some(TArg::Type(t)) => Some(t.clone()),
            _ => None,
        }).collect();
        let same_shape = fty.params.len() == tpl.fty.params.len() && fty.variadic == tpl.fty.variadic;
        if !same_shape || !tpl.fty.params.iter().zip(&fty.params).all(|(p, a)| match_exact(&cx.u, p, a, &mut bound)) || !match_exact(&cx.u, &tpl.fty.ret, &fty.ret, &mut bound) {
            continue;
        }
        let args: Option<Vec<TArg>> = (0..tpl.params.len()).map(|k| match &tpl.kinds[k] {
            None => bound[k].clone().map(TArg::Type),
            Some(_) => explicit.get(k).cloned().filter(|a| matches!(a, TArg::Value(..))),
        }).collect();
        if let Some(args) = args {
            chosen.push((t, args));
        }
    }
    let [(t, args)] = chosen.as_slice() else {
        if chosen.is_empty() {
            cx.error(n.span, "INST-22", format!("an explicit instantiation of '{}' whose type matches no specialization of its templates", name));
            return true;
        }
        return false;
    };
    let (t, args) = (*t, args.clone());
    // The noexcept of the declaration is the specialization's.
    if fty.noexcept && !cx.u.templates[t].fty.noexcept {
        cx.error(n.span, "INST-22", format!("an explicit instantiation of '{}' whose exception specification differs from the template's", name));
        return true;
    }
    if !record_inst(cx, 'f', t, &args, !ext, n.span) {
        return true;
    }
    if ext {
        return false;
    }
    let _ = specialize(cx, t, &args);
    true
}

/// INST-26: one explicit instantiation definition of a specialization, and
/// no declaration after it. False after an error.
fn record_inst(cx: &mut Check, kind: char, v: usize, args: &[TArg], def: bool, span: Span) -> bool {
    if let Some(&(_, _, _, d, _)) = cx.explicit_insts.iter().find(|(k, w, a, _, _)| *k == kind && *w == v && a.as_slice() == args) {
        if d {
            let what = if def { "a second explicit instantiation definition" } else { "an explicit instantiation declaration after the definition" };
            cx.error(span, "INST-26", format!("{} of one specialization", what));
            return false;
        }
    }
    cx.explicit_insts.push((kind, v, args.to_vec(), def, span));
    true
}

/// TSP-2: `template <> R f<A...>(P...) { ... }`, an explicit
/// specialization of a function template of the scope. The arguments
/// that the template-id leaves out come from the function's type
/// (DED-32). False for another form, such as a declaration that is not
/// a definition.
fn explicit_function(cx: &mut Check, fd: &Node) -> bool {
    let (Some(ds), Some(dn)) = (fd.child("decl_specifiers"), fd.child("declarator")) else { return false };
    let Some(did) = dn.find("declarator_id") else { return false };
    let tid = did.find("template_id");
    let name = match tid {
        Some(tid) => tid.child("template_name").map(|t| cx.source(t)),
        None => did.find("unqualified_id").map(|u| cx.source(u)),
    };
    let Some(name) = name else { return false };
    let saved = cx.scope;
    let Some(t) = names::declared_here(cx, saved, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::Template(t) => Some(t),
        _ => None,
    }) else {
        return false;
    };
    // TSP-24: of the storage classes, only `thread_local`.
    if let Some(d) = ds.children("decl_specifier").find(|d| matches!(cx.source(d).as_str(), "static" | "extern")) {
        cx.error(d.span, "TSP-24", format!("the storage class '{}' on an explicit specialization", cx.source(d)));
        return true;
    }
    let tpl = cx.u.templates[t].clone();
    let marks = cx.u.diags.len();
    let explicit = match tid {
        Some(tid) => {
            let n = tid.child("template_args").map(|ta| ta.children("template_arg").count()).unwrap_or(0);
            match template_args(cx, tid, &tpl.kinds[..n.min(tpl.kinds.len())]) {
                Some(a) if n <= tpl.kinds.len() && cx.u.diags.len() == marks => a,
                _ => {
                    cx.u.diags.truncate(marks);
                    return false;
                }
            }
        }
        None => Vec::new(),
    };
    cx.enter(ScopeKind::Template);
    cx.explicit_head = true;
    let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    cx.explicit_head = false;
    let fty = d.ty.func().cloned();
    let done = |cx: &mut Check| {
        cx.leave(saved);
        cx.u.diags.truncate(marks);
        false
    };
    let Some(fty) = fty.filter(|_| !specs.unsupported && !d.unsupported && d.qual == names::Qual::None && cx.u.diags.len() == marks) else { return done(cx) };
    // DED-32: the function type gives the arguments that the template-id
    // leaves out.
    let mut bound: Vec<Option<QType>> = (0..tpl.params.len()).map(|k| match explicit.get(k) {
        Some(TArg::Type(t)) => Some(t.clone()),
        _ => None,
    }).collect();
    let same_shape = fty.params.len() == tpl.fty.params.len() && fty.variadic == tpl.fty.variadic;
    if !same_shape || !tpl.fty.params.iter().zip(&fty.params).all(|(p, a)| match_exact(&cx.u, p, a, &mut bound)) || !match_exact(&cx.u, &tpl.fty.ret, &fty.ret, &mut bound) {
        return done(cx);
    }
    let args: Option<Vec<TArg>> = (0..tpl.params.len()).map(|k| match &tpl.kinds[k] {
        None => bound[k].clone().map(TArg::Type),
        Some(_) => explicit.get(k).cloned().filter(|a| matches!(a, TArg::Value(..))),
    }).collect();
    let Some(args) = args else { return done(cx) };
    // INST-19: no explicit specialization after an explicit instantiation
    // of it.
    if cx.explicit_insts.iter().any(|(k, w, a, _, _)| *k == 'f' && *w == t && *a == args) {
        cx.leave(saved);
        cx.error(dn.span, "INST-19", format!("an explicit specialization of '{}' after an explicit instantiation of it", name));
        return true;
    }
    // CNS-13: the arguments of an explicit specialization satisfy the
    // template's constraints.
    match crate::templates::concepts::check::function_satisfied(cx, t, &args) {
        Some(true) => {}
        Some(false) => {
            cx.leave(saved);
            cx.error(dn.span, "CNS-13", format!("an explicit specialization of '{}' whose arguments do not satisfy its constraints", name));
            return true;
        }
        None => return done(cx),
    }
    if let Some(&(_, f)) = tpl.specs.iter().find(|(a, _)| *a == args) {
        cx.leave(saved);
        if tpl.explicit.contains(&f) {
            return false;
        }
        cx.error(d.name.as_ref().map(|n| n.1).unwrap_or(dn.span), "TSP-6", format!("an explicit specialization of '{}' after a use that made the specialization from the template", name));
        return true;
    }
    let body = fd.child("function_body");
    let Some(f) = crate::declarations::functions::definitions::check::declare_function(cx, &specs, &d, body) else { return done(cx) };
    cx.u.templates[t].specs.push((args, f));
    cx.u.templates[t].explicit.push(f);
    if let Some(cs) = body.and_then(|b| b.child("compound_stmt")) {
        crate::declarations::functions::definitions::check::define(cx, f, &d, cs);
    }
    cx.leave(saved);
    true
}

/// TPL rules: `template <class T> R C<T>::f(P...) { ... }` or
/// `template <class T> T C<T>::s = e;`, a member of a class template
/// defined outside the class. Each specialization of the class, made
/// before or after, gets the member with the parameters bound. None when
/// the declaration has another form.
fn outside_member(cx: &mut Check, n: &Node, params: &[Param]) -> Option<bool> {
    let decl = n.child("declaration")?;
    let (node, dn) = match decl.child("function_definition") {
        Some(fd) => (fd, fd.child("declarator")?),
        None => {
            let bd = decl.child("block_decl")?;
            let sd = bd.child("simple_decl")?;
            let mut decls = sd.children("init_declarator");
            let one = decls.next()?;
            if decls.next().is_some() {
                return None;
            }
            (bd, one.child("declarator")?)
        }
    };
    let nn = dn.find("declarator_id")?.find("qualified_name")?.child("nested_name")?;
    let [Kid::Node(tid), _] = nn.kids.as_slice() else { return None };
    if tid.prod != "template_id" {
        return None;
    }
    let name = cx.source(tid.child("template_name")?);
    let v = names::declared_here(cx, cx.scope, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::ClassTemplate(v) => Some(v),
        _ => None,
    })?;
    // The arguments are the parameters in order; another list names a
    // partial specialization's member.
    let args: Vec<String> = tid.child("template_args").map(|ta| ta.children("template_arg").map(|a| cx.source(a)).collect()).unwrap_or_default();
    let kinds: Vec<Option<QType>> = params.iter().map(|p| p.2.clone()).collect();
    if args.len() != params.len() || args.iter().zip(params).any(|(a, p)| a.as_str() != p.0.as_ref()) || kinds != cx.u.class_templates[v].kinds {
        return Some(false);
    }
    if cx.func.is_some() || cx.member_of.is_some() || dependent_forms(cx, node, params) {
        return Some(false);
    }
    // DEFARG-13: no default argument in the definition of a member of a
    // class template outside its class.
    if node.prod == "function_definition" {
        if let Some(p) = dn.find("parameters").and_then(|ps| ps.children("parameter").find(|p| cx.has_word(p, "="))) {
            cx.error(p.span, "DEFARG-13", "a default argument in the definition of a member of a class template outside its class");
            return Some(true);
        }
    }
    let names_: Vec<Rc<str>> = params.iter().map(|p| p.0.clone()).collect();
    let limit = cx.u.entities.len();
    cx.u.class_templates[v].outside.push((names_, node.clone(), limit));
    let k = cx.u.class_templates[v].outside.len() - 1;
    let ct = cx.u.class_templates[v].clone();
    for (args, c) in &ct.specs {
        if !ct.explicit.contains(c) {
            define_outside(cx, v, k, args);
        }
    }
    Some(true)
}

/// The member that outside definition `k` of class template `v` gives
/// the specialization of `args`. An error in it leaves the unit
/// unsupported, as for a function template.
fn define_outside(cx: &mut Check, v: usize, k: usize, args: &[TArg]) {
    let ct = cx.u.class_templates[v].clone();
    let (names_, node, limit) = ct.outside[k].clone();
    let saved_scope = cx.scope;
    let saved_func = cx.func.take();
    let saved_member = cx.member_of.take();
    let saved_lambdas = std::mem::take(&mut cx.lambdas);
    let saved_unevaluated = std::mem::replace(&mut cx.unevaluated, 0);
    let saved_default = std::mem::replace(&mut cx.default_arg, false);
    let saved_access = cx.access_class.take();
    let saved_limit = cx.lookup_limit.replace(limit);
    cx.instantiating += 1;
    let marks = cx.u.diags.len();
    cx.scope = ct.scope;
    let ts = cx.enter(ScopeKind::Template);
    bind_params(cx, ts, &names_, args, ct.span);
    // TPL-19: inside the definition, the parameters of its own head come
    // before the names of the namespace, so they are bound for the time
    // of the definition in the template scope of the class too.
    let class_ts = ct.specs.iter().find(|(a, _)| a.as_slice() == args).and_then(|&(_, c)| cx.u.scopes[cx.u.classes[c].scope].parent).filter(|&s| cx.u.scopes[s].kind == ScopeKind::Template);
    let mut shadowed: Vec<(Rc<str>, Option<Vec<EntityId>>)> = Vec::new();
    if let Some(s) = class_ts {
        for name in &names_ {
            let ids = cx.u.scopes[ts].names.get(name).cloned().unwrap_or_default();
            let old = cx.u.scopes[s].names.insert(name.clone(), ids);
            shadowed.push((name.clone(), old));
        }
    }
    match node.prod {
        "function_definition" => crate::declarations::functions::definitions::check::function_definition(cx, &node),
        _ => {
            let before = cx.u.statics.len();
            crate::declarations::check::block_decl(cx, &node, false);
            let made: Vec<EntityId> = cx.u.statics[before..].to_vec();
            // ODR-12: each unit that uses the specialization defines the
            // member, and the definitions are one variable, as for an
            // inline variable (INL-1).
            for &id in &made {
                if let Some(x) = cx.u.var_mut(id) {
                    x.inline = true;
                }
            }
            cx.inst_statics.extend(made);
        }
    }
    if let Some(s) = class_ts {
        for (name, old) in shadowed {
            match old {
                Some(ids) => cx.u.scopes[s].names.insert(name, ids),
                None => cx.u.scopes[s].names.remove(&name),
            };
        }
    }
    cx.scope = saved_scope;
    cx.func = saved_func;
    cx.member_of = saved_member;
    cx.lambdas = saved_lambdas;
    cx.unevaluated = saved_unevaluated;
    cx.default_arg = saved_default;
    cx.access_class = saved_access;
    cx.lookup_limit = saved_limit;
    cx.instantiating -= 1;
    if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
        let tail = cx.u.diags.split_off(marks);
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev != crate::diag::Severity::Error));
        cx.unsupported(node.span, format!("an error in a member of a specialization of the class template '{}' (the first pieces of build step 5 do not tell it from a gap of their model)", ct.name));
    }
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
    // A member template sees its own class complete (CLS-2).
    let incomplete = words.iter().any(|w| names::lookup(cx, w).iter().any(|&e| matches!(cx.u.entities[e].kind, EntKind::Class(c) if !cx.u.classes[c].complete && cx.member_of != Some(c))));
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
    let _ = dependent_decltype;
    // DEP-12: unqualified lookup skips a dependent base, which the first
    // pieces of build step 5 do not tell from another base.
    let dependent_base = fd.find("base_clause").is_some_and(|bc| {
        let mut ts = Vec::new();
        bc.tokens(&mut ts);
        let words: Vec<&str> = ts.iter().map(|&t| cx.toks[t].text.as_str()).collect();
        // A base `C<..T..>`: a parameter between `<` and `>`.
        let mut depth = 0;
        words.iter().any(|w| {
            match *w {
                "<" => depth += 1,
                ">" => depth -= 1,
                _ => {}
            }
            depth > 0 && params.iter().any(|(p, _, _)| p.as_ref() == *w)
        })
    });
    let _ = dependent_base;
    incomplete
        || words.iter().any(|w| *w == "concept")
        // A requires clause on a member other than a member function of
        // a class template waits for a later piece.
        || (matches!(fd.prod, "class_def" | "union_def") && words.windows(2).any(|w| w[0] == "requires" && matches!(w[1], "requires")))
        // `x.template f<A>()`: a dependent member template (DEP rules),
        // and `x.f<A>()`, which needs `template` when `x` is dependent
        // (DEP-24).
        || words.windows(2).any(|w| matches!(w[0], "." | "->" | "::") && w[1] == "template")
        || words.windows(3).any(|w| matches!(w[0], "." | "->") && is_word(w[1]) && w[2] == "<")
        // A pack expansion or a fold of a parameter (packs wait for a
        // later piece); `catch (...)` and a C ellipsis are not.
        || (!matches!(fd.prod, "class_def" | "union_def") && !cx.head_pack && words.iter().enumerate().any(|(k, w)| *w == "..." && words[k.saturating_sub(3)..k].iter().any(|x| params.iter().any(|(p, _, _)| p.as_ref() == *x))))
        // `using B<T>::name;` without `typename`, which can name a type
        // of a dependent base (DEP-19).
        || words.windows(3).any(|w| w[0] == "using" && is_word(w[1]) && w[1] != "typename" && w[1] != "namespace" && w[2] == "<")
        // `using enum T::E`: a dependent enumeration (ENUM-20).
        || words.windows(3).any(|w| w[0] == "enum" && params.iter().any(|(p, _, _)| p.as_ref() == w[1]) && w[2] == "::")
        // `T::name x`: a type named after a parameter without `typename`
        // (DEP-19). Another use of `T::` is checked with `T` bound.
        || words.windows(4).enumerate().any(|(k, w)| w[1] == "::" && params.iter().any(|(p, _, _)| p.as_ref() == w[0]) && is_word(w[2]) && (is_word(w[3]) || matches!(w[3], "*" | "&" | "&&")) && (k == 0 || words[k - 1] != "typename"))
        // A type named after a template-id qualifier, which can need
        // `typename` (DEP-19).
        || words.windows(4).any(|w| w[0] == ">" && w[1] == "::" && is_word(w[2]) && is_word(w[3]))
}

/// An identifier or a keyword.
fn is_word(w: &str) -> bool {
    w.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_')
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
    if cx.instantiating >= cx.depth_limit {
        cx.depth_exceeded = true;
        return None;
    }
    let saved_scope = cx.scope;
    let saved_func = cx.func.take();
    let saved_limit = cx.lookup_limit.replace(tpl.limit);
    // The specialization is a function of the template's namespace or
    // class, made outside the class, lambda, or operand that the call
    // stands in.
    let saved_member = std::mem::replace(&mut cx.member_of, tpl.class);
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
    // An abbreviated template's `auto` parameters take their arguments.
    if tpl.abbrev > 0 {
        cx.abbrev_types = TArg::types(&args[args.len() - tpl.abbrev..]).into();
    }
    // The first diagnostic of the body, which a use instantiates (INST-7).
    let mut body_mark = None;
    let r = (|| {
        if tpl.name.as_ref() == crate::classes::constructors::check::CTOR {
            return ctor_specialization(cx, t, args, fd, &mut body_mark);
        }
        let specs = crate::declarations::check::decl_specifiers(cx, fd.child("decl_specifiers")?, Default::default());
        // A template declared and not defined has a declaration.
        let dn = fd.child("declarator").or_else(|| fd.child("init_declarator").and_then(|i| i.child("declarator")))?;
        let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
        if specs.unsupported || d.unsupported || !d.ty.is_func() {
            return None;
        }
        let body = fd.child("function_body");
        let f = crate::declarations::functions::definitions::check::declare_function(cx, &specs, &d, body)?;
        cx.u.templates[t].specs.push((args.to_vec(), f));
        let cs = body.and_then(|b| b.child("compound_stmt"));
        if let Some(cs) = cs {
            body_mark = Some(cx.u.diags.len());
            crate::declarations::functions::definitions::check::define(cx, f, &d, cs);
        }
        Some(f)
    })();
    cx.abbrev_types.clear();
    cx.scope = saved_scope;
    cx.func = saved_func;
    cx.lookup_limit = saved_limit;
    cx.member_of = saved_member;
    cx.lambdas = saved_lambdas;
    cx.unevaluated = saved_unevaluated;
    cx.default_arg = saved_default;
    cx.access_class = saved_access;
    cx.instantiating -= 1;
    // INST-7: an error only in the body of a specialization that no other
    // specialization makes counts once the program uses the function in
    // an evaluated operand; the end of the unit decides.
    // An immediate function that the body calls can make the
    // specialization immediate (CEV rules), which waits for a later piece.
    let body_only = body_mark.is_some_and(|b| !cx.u.diags[marks..b].iter().any(|d| d.sev == crate::diag::Severity::Error)) && !cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Unsupported || matches!(d.rule, Some("CEV-1" | "NAM-8")));
    if let (Some(f), true) = (r, body_only && cx.instantiating == 0 && !cx.trialing && !cx.depth_exceeded && cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error)) {
        let tail = cx.u.diags.split_off(marks);
        let (errs, rest): (Vec<_>, Vec<_>) = tail.into_iter().partition(|d| d.sev == crate::diag::Severity::Error);
        cx.u.diags.extend(rest);
        let point = cx.u.entities.len();
        cx.fn_inst_errors.push((f, errs, point));
        return Some(f);
    }
    // An error in a specialization can come from a part of the model that
    // the first piece does not have (the points of instantiation, the
    // definition checks), so it leaves the unit unsupported.
    if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) || cx.depth_exceeded {
        let tail = cx.u.diags.split_off(marks);
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev != crate::diag::Severity::Error));
        if !depth_failed(cx, tpl.span) {
            cx.unsupported(tpl.span, format!("an error in a specialization of the function template '{}' (the first piece of build step 5 does not tell it from a gap of its model)", tpl.name));
        }
        return None;
    }
    r
}

/// The specialization of the constructor template `t` for `args`: a
/// constructor of its class, defined at once.
fn ctor_specialization(cx: &mut Check, t: usize, args: &[TArg], fd: &Node, body_mark: &mut Option<usize>) -> Option<EntityId> {
    let mut specs = match fd.child("decl_specifiers") {
        Some(ds) => crate::declarations::check::decl_specifiers(cx, ds, Default::default()),
        None => Default::default(),
    };
    specs.ty = Some(QType::void());
    let dn = fd.child("declarator")?;
    let mut d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    if specs.unsupported || d.unsupported || !d.ty.is_func() {
        return None;
    }
    d.name = d.name.clone().map(|(_, s)| (Rc::from(crate::classes::constructors::check::CTOR), s));
    let f = crate::declarations::functions::definitions::check::declare_function(cx, &specs, &d, None)?;
    let home = cx.u.templates[t].scope;
    let access = cx.u.templates[t].class.and_then(|_| {
        let ctor = crate::classes::constructors::check::CTOR;
        names::declared_here(cx, home, ctor).into_iter().find(|&e| matches!(cx.u.entities[e].kind, EntKind::Template(k) if k == t)).and_then(|e| cx.u.access.get(&e).copied())
    });
    if let Some(a) = access {
        cx.u.access.insert(f, a);
    }
    if let Some(x) = cx.u.func_mut(f) {
        x.special = crate::sem::Special::Ctor;
        x.explicit = specs.explicit;
        x.inline = true;
    }
    cx.u.templates[t].specs.push((args.to_vec(), f));
    *body_mark = Some(cx.u.diags.len());
    crate::classes::constructors::check::define(cx, f, fd);
    Some(f)
}

/// TR-13: a chain of instantiations went past the limit. Inside the
/// chain the failure is quiet; the use that began it reports the error.
fn depth_failed(cx: &mut Check, span: Span) -> bool {
    if !cx.depth_exceeded {
        return false;
    }
    if cx.instantiating == 0 {
        cx.depth_exceeded = false;
        let limit = cx.depth_limit;
        cx.error(span, "TR-13", format!("the instantiations nest deeper than the limit of {}", limit));
    }
    true
}

/// The body of the specialization `f` of the function template `t`,
/// which a use declared before the template's definition.
fn define_later(cx: &mut Check, t: usize, args: &[TArg], f: EntityId) {
    let tpl = cx.u.templates[t].clone();
    let saved_scope = cx.scope;
    let saved_func = cx.func.take();
    let saved_limit = cx.lookup_limit.replace(tpl.limit);
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
    if tpl.abbrev > 0 {
        cx.abbrev_types = TArg::types(&args[args.len() - tpl.abbrev..]).into();
    }
    (|| {
        let specs = crate::declarations::check::decl_specifiers(cx, fd.child("decl_specifiers")?, Default::default());
        let d = crate::declarations::declarators::check::declarator(cx, &specs, fd.child("declarator")?);
        cx.abbrev_types.clear();
        let cs = fd.child("function_body").and_then(|b| b.child("compound_stmt"))?;
        crate::declarations::functions::definitions::check::define(cx, f, &d, cs);
        Some(())
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
    if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
        let tail = cx.u.diags.split_off(marks);
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev != crate::diag::Severity::Error));
        cx.unsupported(tpl.span, format!("an error in a specialization of the function template '{}' (the first piece of build step 5 does not tell it from a gap of its model)", tpl.name));
    }
}

/// A parameter of `t` that is a specialization of a class template, or a
/// reference or pointer to one, whose argument has that class as a base
/// more than once.
fn ambiguous_base(cx: &Check, t: usize, bound: &[TArg], args: &[crate::sem::tree::Expr]) -> bool {
    let tpl = &cx.u.templates[t];
    let types: Vec<QType> = bound.iter().filter_map(|a| match a {
        TArg::Type(t) => Some(t.clone()),
        TArg::Value(..) | TArg::Pack(_) => None,
    }).collect();
    tpl.fty.params.iter().zip(args).any(|(p, a)| {
        let inner = match &p.t {
            Type::Ref(x, _) | Type::Ptr(x) => (**x).clone(),
            _ => p.clone(),
        };
        let Type::Dep(d) = &inner.t else { return false };
        let DepType::Spec { template, args: xs, .. } = &**d else { return false };
        if types.len() != bound.len() {
            return false;
        }
        let want: Vec<TArg> = xs.iter().map(|x| match x {
            TArg::Type(y) => TArg::Type(crate::templates::deduction::check::substitute(y, &types)),
            v => v.clone(),
        }).collect();
        let Some(&(_, k)) = cx.u.class_templates[*template].specs.iter().find(|(a, _)| *a == want) else { return false };
        let at = a.ty.nonref();
        let c = match (&p.t, at.pointee()) {
            (Type::Ptr(_), Some(q)) => q.class(),
            _ => at.class(),
        };
        c.is_some_and(|c| c != k && crate::classes::derived::check::paths(cx, c, k).len() > 1)
    })
}

/// The function template that the function `f` specializes, if any.
pub fn template_of(cx: &Check, f: EntityId) -> Option<usize> {
    cx.u.templates.iter().position(|t| t.specs.iter().any(|&(_, g)| g == f))
}

/// ORD-1 to ORD-3: the function template `t1` is at least as specialized
/// as `t2`: with `t1`'s parameters as unique types, `t2`'s parameters
/// deduce from `t1`'s function parameter types, after each type loses
/// its reference and its top-level cv-qualifiers (ORD-2). None when the
/// pair needs a step that the first pieces of build step 5 do not take
/// (both types references, ORD-10, ORD-11; a non-deduced position).
fn at_least_as_specialized(cx: &Check, t1: usize, t2: usize) -> Option<bool> {
    let (a, b) = (&cx.u.templates[t1], &cx.u.templates[t2]);
    // A function parameter pack (ORD-4) waits for a later piece.
    if a.pack_param || b.pack_param {
        return None;
    }
    if a.fty.params.len() != b.fty.params.len() || a.fty.variadic != b.fty.variadic {
        return Some(false);
    }
    let strip = |t: &QType| t.nonref().unqual();
    let opaque = |t: &QType| -> bool {
        match &t.t {
            Type::Dep(d) => matches!(&**d, DepType::Opaque { .. }),
            Type::Ptr(x) | Type::Ref(x, _) | Type::Array(x, _) => {
                let mut q = (**x).clone();
                q.c = false;
                matches!(&q.t, Type::Dep(d) if matches!(&**d, DepType::Opaque { .. }))
            }
            _ => false,
        }
    };
    let mut bound: Vec<Option<QType>> = vec![None; b.params.len()];
    for (p1, p2) in a.fty.params.iter().zip(&b.fty.params) {
        if (p1.is_ref() && p2.is_ref()) || opaque(p1) || opaque(p2) {
            return None;
        }
        if !crate::templates::deduction::check::match_exact(&cx.u, &strip(p2), &strip(p1), &mut bound) {
            return Some(false);
        }
    }
    Some(true)
}

/// OVL-29, ORD rules: with equal conversion sequences, the function `f`
/// beats `g` when `f` is no template specialization and `g` is one, or
/// when both are and `f`'s template is more specialized. None when the
/// order needs a step that the first pieces of build step 5 do not take.
pub fn beats_by_template(cx: &Check, f: EntityId, g: EntityId) -> Option<bool> {
    match (template_of(cx, f), template_of(cx, g)) {
        (None, Some(_)) => Some(true),
        (Some(t1), Some(t2)) if t1 != t2 => {
            let ab = at_least_as_specialized(cx, t1, t2)?;
            let ba = at_least_as_specialized(cx, t2, t1)?;
            // ORD-13: the more constrained template, which the first
            // pieces of build step 5 do not order.
            if ab && ba && (!cx.u.templates[t1].constraints.is_empty() || !cx.u.templates[t2].constraints.is_empty()) {
                return None;
            }
            Some(ab && !ba)
        }
        _ => Some(false),
    }
}

/// TPL rules: a function definition with `auto` or `C auto` parameters
/// at namespace scope, an abbreviated function template. Each such
/// parameter invents a type parameter, with the type constraint as its
/// constraint (CPT-20). False for another declaration.
pub fn abbreviated(cx: &mut Check, fd: &Node) -> bool {
    let Some(dn) = fd.child("declarator") else { return false };
    let Some(ps) = dn.find("parameters") else { return false };
    let autos: Vec<&Node> = ps.children("parameter").filter(|p| p.child("decl_specifiers").is_some_and(|ds| ds.find("placeholder").is_some())).collect();
    if autos.is_empty() || cx.func.is_some() || cx.member_of.is_some() || cx.instantiating > 0 {
        return false;
    }
    // `decltype(auto)` and `auto...` wait for a later piece.
    let mut constraints: Constraints = Vec::new();
    let mut params: Vec<Param> = Vec::new();
    for (k, p) in autos.iter().enumerate() {
        let Some(ph) = p.find("placeholder") else { return false };
        let mut toks = Vec::new();
        p.tokens(&mut toks);
        if cx.words(ph).contains(&"decltype") || toks.iter().any(|&t| cx.toks[t].text == "...") {
            return false;
        }
        if let Some(tc) = ph.child("type_constraint") {
            constraints.push((Some(k), tc.clone()));
        }
        params.push((format!("auto#{}", k).as_str().into(), p.span, None));
    }
    let Some(ds) = fd.child("decl_specifiers") else { return false };
    // The parameters' types at the definition are the invented ones.
    cx.abbrev_types = params.iter().enumerate().map(|(k, p)| QType::new(Type::Dep(Rc::new(DepType::Param { depth: 0, index: k as u32, name: p.0.clone() })))).collect();
    cx.abbrev_count = params.len();
    cx.head_pack = false;
    let defaults = vec![None; params.len()];
    let r = function_template(cx, fd, ds, dn, params, defaults, constraints);
    cx.abbrev_types.clear();
    cx.abbrev_count = 0;
    if !r {
        cx.u.diags.retain(|d| !(d.sev == crate::diag::Severity::Unsupported && d.msg.contains("abbreviated function template")));
    }
    r
}

/// The function templates that a call by name finds, after their
/// specializations for the call's arguments are made: each template
/// whose arguments deduce (DED-P3), with the explicit arguments first.
pub fn call_candidates(cx: &mut Check, found: &[EntityId], tid: Option<&Node>, args: &[crate::sem::tree::Expr]) -> Vec<EntityId> {
    call_candidates_given(cx, found, tid, None, args)
}

/// `call_candidates` with explicit arguments that are given already, as
/// `get<i>` of the tuple protocol (SB-5).
pub fn call_candidates_with(cx: &mut Check, found: &[EntityId], explicit: &[TArg], args: &[crate::sem::tree::Expr]) -> Vec<EntityId> {
    call_candidates_given(cx, found, None, Some(explicit), args)
}

fn call_candidates_given(cx: &mut Check, found: &[EntityId], tid: Option<&Node>, given: Option<&[TArg]>, args: &[crate::sem::tree::Expr]) -> Vec<EntityId> {
    let mut out = Vec::new();
    cx.constraints_failed = false;
    for &e in found {
        let EntKind::Template(t) = cx.u.entities[e].kind else { continue };
        // The explicit arguments, which fill the first parameters.
        let explicit = match (tid, given) {
            (_, Some(g)) => {
                // A constant argument takes the parameter's type.
                let kinds = cx.u.templates[t].kinds.clone();
                if g.len() > kinds.len() {
                    continue;
                }
                g.iter().zip(&kinds).map(|(a, k)| match (a, k) {
                    (TArg::Value(_, v), Some(ty)) => TArg::Value(ty.clone(), *v),
                    (a, _) => a.clone(),
                }).collect()
            }
            (Some(tid), None) => {
                let kinds = cx.u.templates[t].kinds.clone();
                let n = tid.child("template_args").map(|ta| ta.children("template_arg").count()).unwrap_or(0);
                // PACK rules: explicit arguments for a pack come last.
                let packed = cx.u.templates[t].pack && n + 1 >= kinds.len();
                // DED-5: explicit arguments that do not fit remove the
                // candidate.
                let marks = cx.u.diags.len();
                let a = if packed { template_args_packed(cx, tid, &kinds, true) } else { template_args(cx, tid, &kinds[..n.min(kinds.len())]) };
                if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
                    cx.u.diags.truncate(marks);
                    continue;
                }
                match a {
                    Some(a) if packed || n <= kinds.len() => a,
                    _ => continue,
                }
            }
            (None, None) => Vec::new(),
        };
        // A constant parameter takes a value, and a type parameter a type.
        if explicit.iter().zip(&cx.u.templates[t].kinds).any(|(a, k)| match a {
            TArg::Pack(xs) => xs.iter().any(|x| matches!(x, TArg::Value(..)) != k.is_some()),
            _ => matches!(a, TArg::Value(..)) != k.is_some(),
        }) {
            continue;
        }
        let Some(partial) = crate::templates::deduction::check::deduce_call(cx, t, &explicit, args) else {
            // A call that leaves parameters to their default arguments
            // waits for a later piece (ORD-5).
            let tpl = &cx.u.templates[t];
            if args.len() < tpl.fty.params.len() && tpl.node.find("parameters").is_some_and(|ps| ps.children("parameter").any(|p| cx.has_word(p, "="))) {
                let name = tpl.name.clone();
                cx.unsupported(tpl.span, format!("a call of the function template '{}' that uses a default argument", name));
            }
            continue;
        };
        // TPL-13: a parameter that deduction leaves takes its default.
        let tpl = cx.u.templates[t].clone();
        // DED-24: an error in a default is a substitution failure, though
        // one in a lambda's body is not (DED-29).
        let mut bound: Vec<TArg> = Vec::new();
        for (k, a) in partial.iter().enumerate() {
            let marks = if tpl.defaults.get(k).cloned().flatten().is_some_and(|d| d.find("lambda_expr").is_some()) { usize::MAX } else { cx.u.diags.len() };
            match a {
                Some(a) => bound.push(a.clone()),
                None if fill_defaults(cx, &tpl.kinds[..=k], &tpl.defaults, &tpl.params, tpl.scope, tpl.limit, &mut bound, tpl.span) && !cx.u.diags.get(marks..).is_some_and(|ds| ds.iter().any(|d| d.sev == crate::diag::Severity::Error)) => {}
                None => {
                    if cx.u.diags.get(marks..).is_some_and(|ds| ds.iter().any(|d| d.sev == crate::diag::Severity::Error)) {
                        cx.u.diags.truncate(marks);
                    }
                    bound.clear();
                    break;
                }
            }
        }
        if bound.len() < partial.len() {
            continue;
        }
        // PACK-27, PACK-28: a pack of `T...` with a void element fails
        // substitution.
        if tpl.pack_param && matches!(tpl.fty.params.last().map(|p| &p.t), Some(Type::Dep(_))) && bound.last().is_some_and(|a| matches!(a, TArg::Pack(xs) if TArg::types(xs).iter().any(|t| t.is_void()))) {
            continue;
        }
        // DED-15: an argument whose class has the deduced base more than
        // once waits for a later piece (OVL-37).
        if ambiguous_base(cx, t, &bound, args) {
            cx.unsupported(tpl.span, format!("a call of the function template '{}' whose argument has the deduced base more than once", tpl.name));
            continue;
        }
        // CNS-11: a specialization whose arguments do not satisfy the
        // constraints is no candidate.
        match crate::templates::concepts::check::function_satisfied(cx, t, &bound) {
            Some(true) => {}
            Some(false) => {
                cx.constraints_failed = true;
                continue;
            }
            None => {
                cx.unsupported(tpl.span, format!("the constraints of the function template '{}', which the first pieces of build step 5 do not check", tpl.name));
                continue;
            }
        }
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
    // INST-8: a static data member of a specialization that the program
    // does not use is not defined, so its initializer does not run.
    let unused: Vec<EntityId> = cx.inst_statics.iter().copied().filter(|&id| cx.u.var(id).is_some_and(|v| !v.used)).collect();
    if !unused.is_empty() {
        cx.u.statics.retain(|id| !unused.contains(id));
    }
    // INST-2, INST-6: the errors of a class instantiation count when the
    // unit needs the class complete: an object, a member, a base, or a
    // parameter or result of a definition, of the class. Otherwise the
    // front end does not tell whether a use needs it.
    // INST-7: the errors of a function specialization's body count when
    // the program uses the function in an evaluated operand.
    // A function declared at namespace scope after the specialization can
    // change a call or an operator in its body at the point of
    // instantiation at the end of the unit (INST-31, INST-32, DEP-9). A
    // unit with an unsupported construct does not tell either.
    let any_unsupported = cx.u.diags.iter().any(|d| d.sev == crate::diag::Severity::Unsupported);
    for (f, errs, point) in std::mem::take(&mut cx.fn_inst_errors) {
        let later = cx.u.entities[point.min(cx.u.entities.len())..].iter().any(|e| matches!(e.kind, EntKind::Func(_) | EntKind::Template(_)) && cx.u.scopes[e.scope].kind == ScopeKind::Namespace);
        if cx.u.func(f).is_some_and(|x| x.used) && !later && !any_unsupported {
            cx.u.diags.extend(errs);
        } else if let Some(d) = errs.first() {
            let span = d.span;
            cx.unsupported(span, "an error in the body of a function specialization that the program does not use in an evaluated operand");
        }
    }
    for (c, errs) in std::mem::take(&mut cx.inst_errors) {
        if needs_complete(cx, c) && !any_unsupported {
            cx.u.diags.extend(errs);
        } else if let Some(d) = errs.first() {
            let span = d.span;
            cx.unsupported(span, "an error in a class that an instantiation made, whose completeness the front end does not tell a use needs");
        }
    }
    // INST-20: an explicit instantiation definition of a function needs
    // the template's definition in the unit.
    for (kind, t, _, def, span) in cx.explicit_insts.clone() {
        if kind == 'f' && def && cx.u.templates[t].node.prod != "function_definition" {
            let name = cx.u.templates[t].name.clone();
            cx.error(span, "INST-20", format!("an explicit instantiation definition of '{}', whose template the unit does not define", name));
        }
    }
    // The templates that a chain past the limit used are specialized.
    if cx.u.diags.iter().any(|d| d.rule == Some("TR-13")) {
        return;
    }
    let mut unused: Vec<(Rc<str>, Span)> = Vec::new();
    for t in 0..cx.u.templates.len() {
        let tpl = &cx.u.templates[t];
        if tpl.specs.iter().all(|(_, f)| tpl.explicit.contains(f)) && !trial(cx, Some(t), None) {
            unused.push((cx.u.templates[t].name.clone(), cx.u.templates[t].span));
        }
    }
    // An explicit specialization does not check the primary's definition.
    for v in 0..cx.u.class_templates.len() {
        let ct = &cx.u.class_templates[v];
        if ct.defined && ct.specs.iter().all(|(_, c)| ct.explicit.contains(c)) && !trial(cx, None, Some(v)) {
            unused.push((cx.u.class_templates[v].name.clone(), cx.u.class_templates[v].span));
        }
    }
    for (name, span) in unused {
        cx.unsupported(span, format!("the template '{}', which nothing specializes (its definition is not checked yet)", name));
    }
}

/// A trial specialization of an unused function template `t` or class
/// template `v` with `int` for each type parameter. A definition that it
/// checks with no diagnostic has no error that is independent of the
/// arguments. The trial leaves no diagnostic and no static object.
fn trial(cx: &mut Check, t: Option<usize>, v: Option<usize>) -> bool {
    let kinds = match (t, v) {
        (Some(t), _) => cx.u.templates[t].kinds.clone(),
        (_, Some(v)) => cx.u.class_templates[v].kinds.clone(),
        _ => return false,
    };
    // A constant parameter takes 1.
    let mut args: Vec<TArg> = kinds.iter().map(|k| match k {
        None => TArg::Type(QType::new(Type::Int(crate::sem::types::IntK::Int))),
        Some(t) => TArg::Value(t.clone(), 1),
    }).collect();
    // A pack takes one `int`.
    let pack = match (t, v) {
        (Some(t), _) => cx.u.templates[t].pack,
        (_, Some(v)) => cx.u.class_templates[v].pack,
        _ => false,
    };
    if pack {
        if let Some(last) = args.pop() {
            args.push(TArg::Pack(vec![last]));
        }
    }
    let marks = cx.u.diags.len();
    let statics = cx.u.statics.len();
    let saved_trialing = std::mem::replace(&mut cx.trialing, true);
    let ok = match (t, v) {
        (Some(t), _) => !cx.u.templates[t].specs.iter().any(|(a, _)| *a == args) && specialize(cx, t, &args).is_some(),
        (_, Some(v)) => !cx.u.class_templates[v].specs.iter().any(|(a, _)| *a == args) && instantiate_class(cx, v, &args, None).is_some(),
        _ => false,
    };
    cx.trialing = saved_trialing;
    let clean = ok && cx.u.diags.len() == marks;
    cx.u.diags.truncate(marks);
    cx.u.statics.truncate(statics);
    cx.depth_exceeded = false;
    clean
}

/// INST-2: the unit needs class `c` complete: a variable or a data member
/// of the class or an array of it, a class with it as a base, or a
/// defined function with a parameter or a result of it.
fn needs_complete(cx: &Check, c: crate::sem::types::ClassId) -> bool {
    let is_c = |t: &QType| t.base_elem().class() == Some(c) && !t.is_ref() && !t.is_ptr();
    let specs: std::collections::HashSet<EntityId> = cx.u.templates.iter().flat_map(|t| t.specs.iter().map(|&(_, g)| g)).collect();
    cx.u.entities.iter().enumerate().any(|(id, e)| match &e.kind {
        EntKind::Var(v) => is_c(&v.ty),
        EntKind::Field(f) => is_c(&f.ty) && (f.base || f.class != c),
        // A specialization of a function template can be made for an
        // unevaluated operand, which needs no complete class.
        EntKind::Func(f) => f.body.is_some() && !specs.contains(&id) && (is_c(&f.ty.ret) || f.ty.params.iter().any(is_c)),
        _ => false,
    })
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
fn var_template_decl(cx: &mut Check, sd: &Node, params: Vec<Param>, defaults: Vec<Option<Node>>) -> bool {
    let Some((_, dn, init, tid)) = var_parts(sd) else { return false };
    if dn.find("ptr_operator").is_some() || dependent_forms(cx, init, &params) {
        return false;
    }
    if tid.is_some() && defaults.iter().any(|d| d.is_some()) {
        cx.error(dn.span, "TSP-11", "a default template argument on a partial specialization");
        return true;
    }
    if !defaults_last(cx, &defaults, dn.span) {
        return true;
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
            cx.u.var_templates.push(crate::sem::VarTemplate { name: name.as_str().into(), pack: cx.head_pack, defaults: defaults.clone(), kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node: sd.clone(), scope: saved, partials: Vec::new(), specs: Vec::new(), explicit: Vec::new(), span: dn.span, limit });
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

fn match_exact(u: &crate::sem::Unit, p: &QType, a: &QType, bound: &mut [Option<QType>]) -> bool {
    crate::templates::deduction::check::match_exact(u, p, a, bound)
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
    let Some(args) = args_with_defaults(cx, tid, &vt.kinds, &vt.defaults, &vt.params, vt.scope, vt.limit, vt.pack) else {
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
    // TSP rules: an explicit specialization is the variable of its
    // arguments.
    if let Some(&(_, var)) = vt.explicit.iter().find(|(a, _)| *a == args) {
        crate::declarations::initialization::forms::check::note_use(cx, var, span);
        let ty = cx.u.var(var)?.ty.clone();
        return Some(Expr::new(EK::Var(var), ty, Cat::L, span));
    }
    let types: Vec<QType> = args.iter().filter_map(|a| match a {
        TArg::Type(t) => Some(t.clone()),
        TArg::Value(..) | TArg::Pack(_) => None,
    }).collect();
    let mut matches = Vec::new();
    for (k, (ps, pattern, _)) in vt.partials.iter().enumerate() {
        let mut bound: Vec<Option<QType>> = vec![None; ps.len()];
        if types.len() == args.len() && pattern.iter().zip(&types).all(|(p, a)| match_exact(&cx.u, p, a, &mut bound)) {
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
                Some(EntityId::MAX) => return Some(crate::expressions::check::bad(span)),
                Some(id) => {
                    cx.u.var_templates[v].specs.push((source, bound, id));
                    id
                }
                None if depth_failed(cx, span) => return Some(crate::expressions::check::bad(span)),
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
    let (ds, dn, init, _) = var_parts(node)?;
    // A use in an unevaluated operand, or a declarator with more than a
    // name, keeps no error of the specialization.
    let plain = dn.find("array_suffix").is_none() && dn.find("function_suffix").is_none() && dn.find("ptr_operator").is_none();
    if cx.instantiating >= cx.depth_limit {
        cx.depth_exceeded = true;
        return None;
    }
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
    // An error in the initializer of a specialization is an error of the
    // program, unless the front end left a part of it unsupported.
    let unsupported = cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Unsupported);
    if plain && saved_unevaluated == 0 && !unsupported && !cx.depth_exceeded && !cx.trialing && cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
        return r.or(Some(EntityId::MAX));
    }
    if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) || unsupported || cx.depth_exceeded {
        let tail = cx.u.diags.split_off(marks);
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev != crate::diag::Severity::Error));
        return None;
    }
    r
}

/// TPL rules: a class template definition at namespace scope whose
/// parameters are types. False for another form, such as a declaration
/// that is not a definition.
fn class_template_decl(cx: &mut Check, sd: &Node, params: Vec<Param>, defaults: Vec<Option<Node>>, constraints: Constraints) -> bool {
    let Some(ds) = sd.child("decl_specifiers") else { return false };
    for (k, c) in &constraints {
        if k.is_none() && !crate::templates::concepts::check::definition_checks(cx, c, &params) {
            return true;
        }
        if k.is_some() && !crate::templates::concepts::check::type_constraint_form(cx, c) {
            return true;
        }
    }
    let specs: Vec<&Node> = ds.children("decl_specifier").collect();
    let [one] = specs[..] else { return false };
    let saved = cx.scope;
    // `template <class T> struct C;` declares the template.
    if let Some(et) = one.child("type_specifier").and_then(|t| t.child("elaborated_type")) {
        if et.child("nested_name").is_some() || et.find("template_id").is_some() || cx.first_word(et.child("class_key").unwrap_or(et)) == "enum" {
            return false;
        }
        let Some(name) = cx.words(et).last().map(|w| w.to_string()) else { return false };
        let found = names::declared_here(cx, saved, &name);
        if let Some(v) = found.iter().find_map(|&e| match cx.u.entities[e].kind {
            EntKind::ClassTemplate(v) => Some(v),
            _ => None,
        }) {
            if cx.u.class_templates[v].kinds != params.iter().map(|p| p.2.clone()).collect::<Vec<_>>() || !constraints.is_empty() || !cx.u.class_templates[v].constraints.is_empty() {
                return false;
            }
            let earlier = cx.u.class_templates[v].defaults.clone();
            if let Some(m) = merge_defaults(cx, &earlier, &defaults, et.span) {
                if defaults_last(cx, &m, et.span) {
                    cx.u.class_templates[v].defaults = m;
                }
            }
            return true;
        }
        if !defaults_last(cx, &defaults, et.span) {
            return true;
        }
        if !found.is_empty() {
            return false;
        }
        let id = cx.u.class_templates.len();
        let limit = cx.u.entities.len() + 1;
        cx.u.class_templates.push(crate::sem::ClassTemplate { name: name.as_str().into(), pack: cx.head_pack, defaults: defaults.clone(), kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node: et.clone(), scope: saved, specs: Vec::new(), explicit: Vec::new(), outside: Vec::new(), partials: Vec::new(), defined: false, constraints, span: et.span, limit });
        names::declare_in(cx, saved, &name, EntKind::ClassTemplate(id), et.span, Linkage::External);
        return true;
    }
    let Some(cd) = one.child("type_specifier").and_then(|t| t.child("class_def").or_else(|| t.child("union_def"))) else { return false };
    // A `union_def` is its own head (`classes/unions`).
    let head = if cd.prod == "union_def" { cd } else {
        let Some(h) = cd.child("class_head") else { return false };
        h
    };
    if let Some(tid) = head.child("template_id") {
        if !constraints.is_empty() {
            return false;
        }
        // TSP-11: a partial specialization's parameters have no default.
        if defaults.iter().any(|d| d.is_some()) {
            cx.error(head.span, "TSP-11", "a default template argument on a partial specialization");
            return true;
        }
        return class_partial(cx, cd, head, tid, params);
    }
    if head.child("nested_name").is_some() {
        return false;
    }
    let Some(name) = head.child("class_name").map(|c| cx.source(c)) else { return false };
    if shadows_param(cx, cd, &params) {
        return true;
    }
    if dependent_forms(cx, cd, &params) {
        return false;
    }
    // The definition of a template declared before.
    let found = names::declared_here(cx, saved, &name);
    if let Some(v) = found.iter().find_map(|&e| match cx.u.entities[e].kind {
        EntKind::ClassTemplate(v) => Some(v),
        _ => None,
    }) {
        let ct = &cx.u.class_templates[v];
        // TPL-20: the heads of two declarations of one template agree.
        if ct.pack != cx.head_pack {
            cx.error(head.span, "TPL-20", format!("a declaration of '{}' whose template head differs from an earlier one", name));
            return true;
        }
        if ct.defined || ct.kinds != params.iter().map(|p| p.2.clone()).collect::<Vec<_>>() || !ct.specs.is_empty() || !constraints.is_empty() || !ct.constraints.is_empty() {
            return false;
        }
        let earlier = ct.defaults.clone();
        let Some(merged) = merge_defaults(cx, &earlier, &defaults, head.span) else { return true };
        if !defaults_last(cx, &merged, head.span) {
            return true;
        }
        let limit = cx.u.entities.len() + 1;
        let ct = &mut cx.u.class_templates[v];
        ct.defaults = merged;
        ct.params = params.into_iter().map(|p| p.0).collect();
        ct.node = cd.clone();
        ct.span = head.span;
        ct.limit = limit;
        ct.defined = true;
        return true;
    }
    if !found.is_empty() {
        return false;
    }
    // LSUP rules: `std::initializer_list`, which the language builds
    // braced lists into (ILC rules), and `std::allocator`, whose
    // allocation constant evaluation allows (LSUP-11), are classes that
    // the first pieces of build step 5 do not model.
    if &*name == "allocator" && cx.u.scopes[saved].name.as_ref() == "std" {
        return false;
    }
    if !defaults_last(cx, &defaults, head.span) {
        return true;
    }
    let id = cx.u.class_templates.len();
    let limit = cx.u.entities.len() + 1;
    cx.u.class_templates.push(crate::sem::ClassTemplate { name: name.as_str().into(), pack: cx.head_pack, defaults: defaults.clone(), kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node: cd.clone(), scope: saved, specs: Vec::new(), explicit: Vec::new(), outside: Vec::new(), partials: Vec::new(), defined: true, constraints, span: head.span, limit });
    names::declare_in(cx, saved, &name, EntKind::ClassTemplate(id), head.span, Linkage::External);
    true
}

/// TPL rules: `template <class T> R f(P...) { ... }` in the class `class`,
/// a member function template. False for another form, such as a
/// constructor template.
pub fn member_template(cx: &mut Check, class: crate::sem::types::ClassId, m: &Node, access: crate::sem::Access) -> bool {
    // TPL-2: no member template in a local class.
    if cx.func.is_some() {
        cx.error(m.span, "TPL-2", "a member template of a local class");
        return true;
    }
    let Some((params, defaults, constraints, pack)) = m.child("template_head").and_then(|h| type_params(cx, h)) else { return false };
    cx.head_pack = pack;
    // A member class template defined in the class.
    if let Some(inner) = m.child("member_decl").filter(|d| d.child("member_declarator").is_none() && d.child("decl_specifiers").is_some_and(|ds| ds.find("class_def").is_some() && ds.find("class_head").is_some_and(|h| h.child("template_id").is_none() && h.child("nested_name").is_none()))) {
        let before = cx.u.entities.len();
        let as_sd = Node { prod: "simple_decl", ext: None, span: inner.span, kids: inner.kids.clone() };
        if !class_template_decl(cx, &as_sd, params, defaults, constraints) {
            return false;
        }
        for e in before..cx.u.entities.len() {
            if matches!(cx.u.entities[e].kind, EntKind::ClassTemplate(_)) {
                cx.u.access.insert(e, access);
            }
        }
        return true;
    }
    // A default sees the class incomplete (CLS-2), and constraints on a
    // member wait for a later piece.
    if defaults.iter().any(|d| d.is_some()) || !constraints.is_empty() {
        return false;
    }
    let Some(inner) = m.child("member_decl") else { return false };
    let Some(fd) = inner.child("function_definition") else { return false };
    if let Some(dn) = fd.child("declarator").filter(|dn| crate::classes::constructors::check::special_name(cx, class, dn) == Some(false)) {
        return ctor_template(cx, class, fd, dn, params, defaults, access);
    }
    // Operator and conversion function templates wait for a later piece.
    if fd.child("declarator").is_some_and(|d| d.find("operator_function_id").is_some() || d.find("conversion_function_id").is_some() || d.find("literal_operator_id").is_some()) {
        return false;
    }
    let (Some(ds), Some(dn)) = (fd.child("decl_specifiers"), fd.child("declarator")) else { return false };
    if shadows_param(cx, fd, &params) || expands_no_pack(cx, fd) {
        return true;
    }
    if dependent_forms(cx, fd, &params) || cx.words(ds).iter().any(|w| matches!(*w, "virtual" | "friend")) {
        return false;
    }
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
    let placeholder_ok = d.placeholder.as_ref().is_none_or(|p| !p.decltype);
    if specs.unsupported || d.unsupported || d.qual != names::Qual::None || cx.u.diags.len() > marks || !placeholder_ok || specs.is_static || d.params.iter().any(|p| p.this_param) {
        cx.u.diags.truncate(marks);
        return false;
    }
    // A name that the class declares already waits for a later piece.
    if !names::declared_here(cx, saved, &name).is_empty() || name.starts_with('~') || name.as_ref() == cx.u.classes[class].name.as_ref() {
        return false;
    }
    // PACK rules: a function parameter pack as the last parameter.
    let pack_param = d.params.last().is_some_and(|p| p.pack.is_some());
    if d.params.iter().rev().skip(1).any(|p| p.pack.is_some()) || (pack_param && (!cx.head_pack || fty.variadic)) {
        return false;
    }
    let id = cx.u.templates.len();
    let limit = cx.u.entities.len() + 1;
    cx.u.templates.push(FnTemplate { name: name.clone(), pack: cx.head_pack, defaults, kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node: fd.clone(), scope: saved, fty: Rc::new(fty), pack_param, pack_name: d.params.last().and_then(|p| p.pack.clone()), abbrev: 0, specs: Vec::new(), explicit: Vec::new(), class: Some(class), constraints: Vec::new(), span, limit });
    let e = names::declare_in(cx, saved, &name, EntKind::Template(id), span, Linkage::External);
    cx.u.access.insert(e, access);
    true
}

/// TPL rules: a constructor template of the class `class`, defined in
/// the class. Its specializations are constructors (CTOR-1).
fn ctor_template(cx: &mut Check, class: crate::sem::types::ClassId, fd: &Node, dn: &Node, params: Vec<Param>, defaults: Vec<Option<Node>>, access: crate::sem::Access) -> bool {
    if shadows_param(cx, fd, &params) || expands_no_pack(cx, fd) {
        return true;
    }
    if dependent_forms(cx, fd, &params) {
        return false;
    }
    let saved = cx.scope;
    let ts = cx.enter(ScopeKind::Template);
    declare_params(cx, ts, &params);
    let marks = cx.u.diags.len();
    let mut specs = match fd.child("decl_specifiers") {
        Some(ds) => crate::declarations::check::decl_specifiers(cx, ds, Default::default()),
        None => Default::default(),
    };
    let bad_specs = specs.unsupported || specs.ty.is_some() || specs.is_static || specs.is_virtual || specs.typedef;
    specs.ty = Some(QType::void());
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    cx.leave(saved);
    let fty = d.ty.func().cloned();
    let Some(fty) = fty.filter(|_| !bad_specs && !d.unsupported && cx.u.diags.len() == marks && !d.params.iter().any(|p| p.this_param) && d.obj_cv == (false, false) && !d.ref_q && !d.params.iter().any(|p| p.default.is_some())) else {
        cx.u.diags.truncate(marks);
        return false;
    };
    // PACK rules: a function parameter pack as the last parameter.
    let pack_param = d.params.last().is_some_and(|p| p.pack.is_some());
    if d.params.iter().rev().skip(1).any(|p| p.pack.is_some()) || (pack_param && (!cx.head_pack || fty.variadic)) {
        return false;
    }
    let span = d.name.as_ref().map(|n| n.1).unwrap_or(dn.span);
    let id = cx.u.templates.len();
    let limit = cx.u.entities.len() + 1;
    let ctor: Rc<str> = crate::classes::constructors::check::CTOR.into();
    cx.u.templates.push(FnTemplate { name: ctor.clone(), pack: cx.head_pack, defaults, kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), node: fd.clone(), scope: saved, fty: Rc::new(fty), pack_param, pack_name: d.params.last().and_then(|p| p.pack.clone()), abbrev: 0, specs: Vec::new(), explicit: Vec::new(), class: Some(class), constraints: Vec::new(), span, limit });
    let e = names::declare_in(cx, saved, &ctor, EntKind::Template(id), span, Linkage::External);
    cx.u.access.insert(e, access);
    true
}

/// TPL rules: `template <class T> using A = type-id;`, an alias template
/// at namespace scope.
fn alias_template_decl(cx: &mut Check, ad: &Node, params: Vec<Param>, defaults: Vec<Option<Node>>) -> bool {
    let Some(node) = ad.child("type_id") else { return false };
    let words = cx.words(ad);
    let Some(name) = words.iter().skip_while(|w| **w != "using").nth(1).map(|w| w.to_string()) else { return false };
    let saved = cx.scope;
    if !names::declared_here(cx, saved, &name).is_empty() || ad.child("attribute_spec").is_some() {
        return false;
    }
    if !defaults_last(cx, &defaults, ad.span) {
        return true;
    }
    let id = cx.u.alias_templates.len();
    let limit = cx.u.entities.len() + 1;
    cx.u.alias_templates.push(crate::sem::AliasTemplate { name: name.as_str().into(), pack: cx.head_pack, kinds: params.iter().map(|p| p.2.clone()).collect(), params: params.into_iter().map(|p| p.0).collect(), defaults, node: node.clone(), scope: saved, span: ad.span, limit, specs: Vec::new() });
    names::declare_in(cx, saved, &name, EntKind::AliasTemplate(id), ad.span, Linkage::None);
    true
}

/// `A<X...>` for an alias template: its type-id with the parameters bound
/// to the arguments. A dependent argument gives a dependent type.
fn alias_use(cx: &mut Check, a: usize, tid: &Node) -> QType {
    let at = cx.u.alias_templates[a].clone();
    let marks = cx.u.diags.len();
    let Some(args) = args_with_defaults(cx, tid, &at.kinds, &at.defaults, &at.params, at.scope, at.limit, at.pack) else {
        if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
            return QType::error();
        }
        return cx.unsupported(tid.span, "an alias template with arguments of another number or form");
    };
    if let Some((_, t)) = at.specs.iter().find(|(a, _)| *a == args) {
        return t.clone();
    }
    let saved_scope = cx.scope;
    let saved_limit = cx.lookup_limit.replace(at.limit);
    cx.scope = at.scope;
    let ts = cx.enter(ScopeKind::Template);
    bind_params(cx, ts, &at.params, &args, at.span);
    let t = crate::declarations::declarators::check::type_id(cx, &at.node);
    cx.scope = saved_scope;
    cx.lookup_limit = saved_limit;
    if !t.is_error() && !args.iter().any(|a| matches!(a, TArg::Type(x) if crate::templates::deduction::check::has_dep(x))) {
        cx.u.alias_templates[a].specs.push((args, t.clone()));
    }
    t
}

/// TSP-9: `template <class T> struct C<P...> { ... };`, a partial
/// specialization of a class template of the scope whose parameters are
/// types.
fn class_partial(cx: &mut Check, cd: &Node, head: &Node, tid: &Node, params: Vec<Param>) -> bool {
    let saved = cx.scope;
    if cx.head_pack {
        return false;
    }
    // TSP-19: a qualified name names a template of an enclosed namespace.
    let home = match head.child("nested_name") {
        None => saved,
        Some(nn) => match names::nested_name(cx, nn) {
            names::Qual::Scope(s) if cx.u.scopes[s].kind == ScopeKind::Namespace => {
                let mut p = Some(s);
                while p.is_some_and(|x| x != saved) {
                    p = cx.u.scopes[p.unwrap()].parent;
                }
                if p.is_none() {
                    cx.error(head.span, "TSP-5", "a partial specialization outside a namespace that encloses its template");
                    return true;
                }
                s
            }
            names::Qual::Error => return true,
            _ => return false,
        },
    };
    let Some(name) = tid.child("template_name").map(|t| cx.source(t)) else { return false };
    let Some(v) = names::declared_here(cx, home, &name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::ClassTemplate(v) => Some(v),
        _ => None,
    }) else {
        return false;
    };
    if shadows_param(cx, cd, &params) {
        return true;
    }
    if dependent_forms(cx, cd, &params) {
        return false;
    }
    // TSP-32: `union` agrees only with `union`.
    let ct = &cx.u.class_templates[v];
    let primary_union = ct.node.prod == "union_def" || (ct.node.prod == "elaborated_type" && cx.first_word(ct.node.child("class_key").unwrap_or(&ct.node)) == "union");
    if primary_union != (cd.prod == "union_def") {
        return false;
    }
    let primary_kinds = ct.kinds.clone();
    let nodes: Vec<Node> = tid.child("template_args").map(|ta| ta.children("template_arg").cloned().collect()).unwrap_or_default();
    if nodes.len() != primary_kinds.len() || !cx.u.class_templates[v].specs.is_empty() || nodes.iter().any(|a| cx.words(a).contains(&"...")) {
        return false;
    }
    cx.scope = home;
    let ts = cx.enter(ScopeKind::Template);
    declare_params(cx, ts, &params);
    let marks = cx.u.diags.len();
    // The pattern: a type, a constant parameter of the partial
    // specialization, or a value.
    let mut pattern: Vec<crate::sem::PatArg> = Vec::new();
    for (a, kind) in nodes.iter().zip(&primary_kinds) {
        let item = match kind {
            None => a.child("type_id").map(|t| crate::sem::PatArg::Type(crate::declarations::declarators::check::type_id(cx, t))),
            Some(ty) => {
                let src = cx.source(a);
                match params.iter().position(|p| p.2.is_some() && p.0.as_ref() == src.as_str()) {
                    Some(i) => Some(crate::sem::PatArg::Param(i)),
                    None => constant_arg(cx, a).and_then(|(e, x)| constant_of(cx, &e, x, ty, a.span)).and_then(|t| match t {
                        TArg::Value(_, x) => Some(crate::sem::PatArg::Value(x)),
                        _ => None,
                    }),
                }
            }
        };
        match item {
            Some(i) => pattern.push(i),
            None => {
                cx.leave(saved);
                cx.u.diags.truncate(marks);
                return false;
            }
        }
    }
    cx.leave(saved);
    if cx.u.diags.len() > marks {
        cx.u.diags.truncate(marks);
        return false;
    }
    // TSP-11: each parameter is deducible from the pattern, and a pattern
    // of the parameters alone, in any order, matches every list.
    let index = |p: &crate::sem::PatArg| match p {
        crate::sem::PatArg::Type(t) => match &t.t {
            Type::Dep(d) if !t.c && !t.v => match &**d {
                DepType::Param { index, .. } => Some(*index as usize),
                _ => None,
            },
            _ => None,
        },
        crate::sem::PatArg::Param(i) => Some(*i),
        crate::sem::PatArg::Value(_) => None,
    };
    let bare: Vec<Option<usize>> = pattern.iter().map(index).collect();
    let mut seen: Vec<usize> = bare.iter().flatten().copied().collect();
    seen.sort();
    seen.dedup();
    if bare.iter().all(|b| b.is_some()) && seen.len() == bare.len() {
        cx.error(tid.span, "TSP-11", format!("a partial specialization of '{}' that matches every argument list of the template", name));
        return true;
    }
    let appears = |k: usize| pattern.iter().any(|p| match p {
        crate::sem::PatArg::Type(t) => params[k].2.is_none() && mentions(t, k as u32),
        crate::sem::PatArg::Param(i) => *i == k,
        crate::sem::PatArg::Value(_) => false,
    });
    if let Some(k) = (0..params.len()).find(|&k| !appears(k)) {
        cx.error(tid.span, "TSP-11", format!("the parameter '{}' of a partial specialization of '{}' does not appear in its arguments", params[k].0, name));
        return true;
    }
    let kinds: Vec<Option<QType>> = params.iter().map(|p| p.2.clone()).collect();
    cx.u.class_templates[v].partials.push((params.into_iter().map(|p| p.0).collect(), kinds, pattern, cd.clone()));
    true
}

/// The type `t` names the parameter `index`.
fn mentions(t: &QType, index: u32) -> bool {
    match &t.t {
        Type::Dep(d) => match &**d {
            DepType::Param { index: i, .. } => *i == index,
            DepType::Spec { args, .. } => args.iter().any(|a| matches!(a, TArg::Type(t) if mentions(t, index))),
            DepType::Opaque { .. } => false,
        },
        Type::Ptr(x) | Type::Ref(x, _) | Type::Array(x, _) | Type::MemPtr(_, x) => mentions(x, index),
        Type::Func(f) => mentions(&f.ret, index) || f.params.iter().any(|p| mentions(p, index)),
        _ => false,
    }
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
    if let Some(a) = found.iter().find_map(|&e| match cx.u.entities[e].kind {
        EntKind::AliasTemplate(a) => Some(a),
        _ => None,
    }) {
        return Some(alias_use(cx, a, tid));
    }
    // Inside a specialization, the class's own name finds the class,
    // which leads to its template (DEP-15).
    let v = found.into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::ClassTemplate(v) => Some(v),
        EntKind::Class(c) => cx.u.class_templates.iter().position(|t| t.specs.iter().any(|&(_, k)| k == c)),
        _ => None,
    })?;
    let ct = cx.u.class_templates[v].clone();
    let marks = cx.u.diags.len();
    let Some(args) = args_with_defaults(cx, tid, &ct.kinds, &ct.defaults, &ct.params, ct.scope, ct.limit, ct.pack) else {
        if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
            return Some(QType::error());
        }
        return Some(cx.unsupported(span, "a class template with arguments of another number or form"));
    };
    if args.iter().any(|a| matches!(a, TArg::Type(t) if t.is_error())) {
        return Some(QType::error());
    }
    // In a template's definition, a specialization with a parameter in
    // its arguments is a dependent type.
    if args.iter().any(|a| matches!(a, TArg::Type(t) if crate::templates::deduction::check::has_dep(t))) {
        let name: Rc<str> = cx.source(tid).into();
        return Some(QType::new(Type::Dep(Rc::new(DepType::Spec { template: v, args, name }))));
    }
    Some(class_specialization(cx, v, args, span))
}

/// The class of the specialization of class template `v` for `args`:
/// one made before, or one that its primary or a partial specialization
/// makes now (TSP-13 to TSP-15).
pub fn class_specialization(cx: &mut Check, v: usize, args: Vec<TArg>, span: Span) -> QType {
    let ct = cx.u.class_templates[v].clone();
    let name = ct.name.clone();
    if let Some(&(_, c)) = ct.specs.iter().find(|(a, _)| *a == args) {
        return QType::new(Type::Class(c));
    }
    match crate::templates::concepts::check::class_satisfied(cx, v, &args) {
        Some(true) => {}
        Some(false) => {
            cx.error(span, "CNS-13", format!("arguments of the class template '{}' that do not satisfy its constraints", name));
            return QType::error();
        }
        None => return cx.unsupported(span, format!("the constraints of the class template '{}' for these arguments", name)),
    }
    // TSP-13 to TSP-15: the one partial specialization whose pattern the
    // arguments match, or the primary.
    let mut matches = Vec::new();
    for (k, (ps, kinds, pattern, _)) in ct.partials.iter().enumerate() {
        let mut bound: Vec<Option<QType>> = vec![None; ps.len()];
        let mut values: Vec<Option<i128>> = vec![None; ps.len()];
        let fits = pattern.len() == args.len() && pattern.iter().zip(&args).all(|(p, a)| match (p, a) {
            (crate::sem::PatArg::Type(p), TArg::Type(a)) => match_exact(&cx.u, p, a, &mut bound),
            (crate::sem::PatArg::Value(v), TArg::Value(_, x)) => v == x,
            (crate::sem::PatArg::Param(i), TArg::Value(_, x)) => match values[*i] {
                Some(y) => y == *x,
                None => {
                    values[*i] = Some(*x);
                    true
                }
            },
            _ => false,
        });
        if fits {
            let b: Option<Vec<TArg>> = (0..ps.len()).map(|i| match &kinds[i] {
                None => bound[i].clone().map(TArg::Type),
                Some(t) => values[i].map(|v| TArg::Value(t.clone(), v)),
            }).collect();
            if let Some(b) = b {
                matches.push((k, b));
            }
        }
    }
    if matches.len() > 1 {
        return cx.unsupported(span, "a class template whose arguments match several partial specializations (partial ordering)");
    }
    let source = matches.pop();
    if source.is_none() && !ct.defined {
        return cx.unsupported(span, format!("a specialization of the class template '{}', which is declared and not defined", name));
    }
    // INST rules: a member declared with a parameter that a function type
    // binds is a data member of function type, which the first pieces of
    // build step 5 do not tell from a member function.
    if args.iter().any(|a| matches!(a, TArg::Type(t) if t.is_func())) {
        return cx.unsupported(span, "a class template with a function type as an argument");
    }
    match instantiate_class(cx, v, &args, source) {
        Some(c) => QType::new(Type::Class(c)),
        None if depth_failed(cx, span) => QType::error(),
        None => cx.unsupported(span, format!("a specialization of the class template '{}' that the first pieces of build step 5 do not make", name)),
    }
}

/// The class of a specialization: the definition checked with each
/// parameter bound to its argument, as a class of the template's
/// namespace. The class is recorded before its body, so the body finds
/// it by the template-id. An error in it leaves the unit unsupported.
fn instantiate_class(cx: &mut Check, v: usize, args: &[TArg], source: Option<(usize, Vec<TArg>)>) -> Option<crate::sem::types::ClassId> {
    let ct = cx.u.class_templates[v].clone();
    // A partial specialization binds its own parameters.
    let (params, bound, node) = match &source {
        Some((k, b)) => (ct.partials[*k].0.clone(), b.clone(), ct.partials[*k].3.clone()),
        None => (ct.params.clone(), args.to_vec(), ct.node.clone()),
    };
    if cx.instantiating >= cx.depth_limit {
        cx.depth_exceeded = true;
        return None;
    }
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
    bind_params(cx, ts, &params, &bound, ct.span);
    // The class that `class_def` makes first is the specialization.
    let predicted = cx.u.classes.len();
    cx.u.class_templates[v].specs.push((args.to_vec(), predicted));
    cx.explicit_head = source.is_some();
    let saved_lazy = cx.lazy_mark.take();
    let t = crate::classes::members::check::class_def(cx, &node);
    let lazy = std::mem::replace(&mut cx.lazy_mark, saved_lazy);
    cx.explicit_head = false;
    cx.scope = saved_scope;
    cx.func = saved_func;
    cx.member_of = saved_member;
    cx.lambdas = saved_lambdas;
    cx.unevaluated = saved_unevaluated;
    cx.default_arg = saved_default;
    cx.access_class = saved_access;
    cx.lookup_limit = saved_limit;
    cx.instantiating -= 1;
    // An error in the declarations that the instantiation made waits for
    // a use that needs the class complete (INST-2, INST-6). One in the
    // parts that a use instantiates, or a construct that the front end
    // does not model, leaves the unit unsupported.
    let lazy = lazy.unwrap_or(cx.u.diags.len()).max(marks);
    let declared_only = t.class() == Some(predicted) && !cx.depth_exceeded && !cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Unsupported) && !cx.u.diags[lazy..].iter().any(|d| d.sev == crate::diag::Severity::Error);
    // An argument class that is not complete yet makes the point of
    // instantiation matter (INST-30), which waits for a later piece.
    let args_complete = TArg::types(args).iter().all(|t| t.base_elem().class().is_none_or(|k| cx.u.classes[k].complete));
    if declared_only && args_complete && !cx.trialing && cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) && cx.instantiating == 0 {
        let tail = cx.u.diags.split_off(marks);
        let (errs, rest): (Vec<_>, Vec<_>) = tail.into_iter().partition(|d| d.sev == crate::diag::Severity::Error);
        cx.u.diags.extend(rest);
        cx.inst_errors.push((predicted, errs));
        if source.is_some() {
            cx.u.class_templates[v].explicit.push(predicted);
            return Some(predicted);
        }
        for k in 0..cx.u.class_templates[v].outside.len() {
            define_outside(cx, v, k, args);
        }
        return Some(predicted);
    }
    let bad = cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error || d.sev == crate::diag::Severity::Unsupported);
    if t.class() != Some(predicted) || bad || cx.depth_exceeded {
        cx.u.class_templates[v].specs.retain(|(a, _)| a.as_slice() != args);
        let tail = cx.u.diags.split_off(marks);
        cx.u.diags.extend(tail.into_iter().filter(|d| d.sev != crate::diag::Severity::Error));
        return None;
    }
    if source.is_some() {
        cx.u.class_templates[v].explicit.push(predicted);
        return Some(predicted);
    }
    for k in 0..cx.u.class_templates[v].outside.len() {
        define_outside(cx, v, k, args);
    }
    Some(predicted)
}
