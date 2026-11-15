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
