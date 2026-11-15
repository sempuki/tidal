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
use crate::sem::types::{DepType, QType, Type};
use crate::sem::{EntKind, EntityId, FnTemplate, Linkage, ScopeKind};
use crate::source::Span;
use crate::syntax::{Kid, Node};
use std::rc::Rc;

/// The names of a template head whose parameters are all type
/// parameters without a default, a pack, or a constraint; None for
/// another head.
fn type_params(cx: &Check, head: &Node) -> Option<Vec<(Rc<str>, Span)>> {
    if head.child("requires_clause").is_some() {
        return None;
    }
    let mut out = Vec::new();
    for tp in head.child("template_params")?.children("template_param") {
        let t = tp.child("type_param")?;
        if t.child("type_key").is_none() || t.kids.iter().any(|k| matches!(k, Kid::Tok(_)) && matches!(cx.text(k), "=" | "...")) {
            return None;
        }
        let k = t.kids.iter().rev().find(|k| matches!(k, Kid::Tok(_)))?;
        let name: Rc<str> = cx.text(k).into();
        if name.as_ref() == "class" || name.as_ref() == "typename" {
            return None;
        }
        out.push((name, cx.token(k).map(|t| t.span).unwrap_or(t.span)));
    }
    Some(out)
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
        return var_template_decl(cx, sd, params);
    }
    let Some(fd) = n.child("declaration").and_then(|d| d.child("function_definition")) else { return false };
    let (Some(ds), Some(dn)) = (fd.child("decl_specifiers"), fd.child("declarator")) else { return false };
    let saved = cx.scope;
    let ts = cx.enter(ScopeKind::Template);
    for (k, (name, span)) in params.iter().enumerate() {
        let t = QType::new(Type::Dep(Rc::new(DepType::Param { depth: 0, index: k as u32, name: name.clone() })));
        names::declare_in(cx, ts, name, EntKind::Typedef(t), *span, Linkage::None);
    }
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
    cx.u.templates.push(FnTemplate { name: name.clone(), params: params.into_iter().map(|p| p.0).collect(), node: fd.clone(), scope: saved, fty: Rc::new(fty), specs: Vec::new(), span, limit });
    names::declare_in(cx, saved, &name, EntKind::Template(id), span, Linkage::External);
    true
}

/// The definition uses a form that needs the dependent names of model
/// sections 1 and 2: `typename`, `template`, `requires`, `decltype`,
/// `concept`, or a name qualified by a template parameter.
fn dependent_forms(cx: &Check, fd: &Node, params: &[(Rc<str>, Span)]) -> bool {
    let mut toks = Vec::new();
    fd.tokens(&mut toks);
    let words: Vec<&str> = toks.iter().map(|&t| cx.toks[t].text.as_str()).collect();
    // DEP-31: a class that is incomplete at the definition and complete
    // at a specialization needs the definition checks.
    let incomplete = words.iter().any(|w| names::lookup(cx, w).iter().any(|&e| matches!(cx.u.entities[e].kind, EntKind::Class(c) if !cx.u.classes[c].complete)));
    incomplete
        || words.iter().any(|w| matches!(*w, "typename" | "template" | "requires" | "decltype" | "concept"))
        || words.windows(2).any(|w| w[1] == "::" && params.iter().any(|(p, _)| p.as_ref() == w[0]))
}

/// TARG-1, DED-P3: the specialization of the function template `t` for
/// the arguments `args`, made the first time: the definition checked in
/// a scope that binds each parameter to its argument. The function is
/// recorded before its body is checked, so a call of the same
/// specialization in the body finds it. None when the declaration does
/// not check.
pub fn specialize(cx: &mut Check, t: usize, args: &[QType]) -> Option<EntityId> {
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
    for (k, name) in tpl.params.iter().enumerate() {
        names::declare_in(cx, ts, name, EntKind::Typedef(args[k].clone()), tpl.span, Linkage::None);
    }
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
pub fn call_candidates(cx: &mut Check, found: &[EntityId], explicit: &[QType], args: &[crate::sem::tree::Expr]) -> Vec<EntityId> {
    let mut out = Vec::new();
    for &e in found {
        let EntKind::Template(t) = cx.u.entities[e].kind else { continue };
        let Some(bound) = crate::templates::deduction::check::deduce_call(cx, t, explicit, args) else { continue };
        if let Some(f) = specialize(cx, t, &bound) {
            out.push(f);
        }
    }
    out
}

/// The function templates of the unit that nothing specialized: their
/// definitions are not checked yet, so the unit is unsupported.
pub fn end_of_unit(cx: &mut Check) {
    let unused: Vec<(Rc<str>, Span)> = cx.u.templates.iter().filter(|t| t.specs.is_empty()).map(|t| (t.name.clone(), t.span)).collect();
    for (name, span) in unused {
        cx.unsupported(span, format!("the function template '{}', which nothing specializes (its definition is not checked yet)", name));
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
fn var_template_decl(cx: &mut Check, sd: &Node, params: Vec<(Rc<str>, Span)>) -> bool {
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
            cx.u.var_templates.push(crate::sem::VarTemplate { name: name.as_str().into(), params: params.into_iter().map(|p| p.0).collect(), node: sd.clone(), scope: saved, partials: Vec::new(), specs: Vec::new(), span: dn.span, limit });
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
            let ts = cx.enter(ScopeKind::Template);
            for (k, (p, span)) in params.iter().enumerate() {
                let t = QType::new(Type::Dep(Rc::new(DepType::Param { depth: 0, index: k as u32, name: p.clone() })));
                names::declare_in(cx, ts, p, EntKind::Typedef(t), *span, Linkage::None);
            }
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
    let Some(args) = template_type_args(cx, tid) else { return unsupported(cx, "a variable template with an argument that is not a type") };
    if args.iter().any(|a| a.is_error()) {
        return Some(crate::expressions::check::bad(span));
    }
    let vt = cx.u.var_templates[v].clone();
    if args.len() != vt.params.len() || args.iter().any(crate::templates::deduction::check::has_dep) {
        return unsupported(cx, "a variable template with another number of arguments or a dependent argument");
    }
    let mut matches = Vec::new();
    for (k, (ps, pattern, _)) in vt.partials.iter().enumerate() {
        let mut bound: Vec<Option<QType>> = vec![None; ps.len()];
        if pattern.iter().zip(&args).all(|(p, a)| match_exact(p, a, &mut bound)) {
            if let Some(b) = bound.into_iter().collect::<Option<Vec<QType>>>() {
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
fn instantiate_var(cx: &mut Check, v: usize, params: &[Rc<str>], args: &[QType], node: &Node) -> Option<EntityId> {
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
    for (k, name) in params.iter().enumerate() {
        names::declare_in(cx, ts, name, EntKind::Typedef(args[k].clone()), vt.span, Linkage::None);
    }
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
