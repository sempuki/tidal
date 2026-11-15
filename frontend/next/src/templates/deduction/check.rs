//! The check part of `templates/deduction`: the deduction of a function
//! template's type parameters from the arguments of a call (DED-P3,
//! build step 5, first piece). Each parameter of the function type that
//! names a template parameter is matched against its argument's type:
//! through pointers, arrays, and cv-qualifiers, by value after the
//! argument decays, and by reference with the argument's own type. A
//! forwarding reference deduces an lvalue reference for an lvalue.

use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr};
use crate::sem::types::{DepType, QType, TArg, Type};
use std::rc::Rc;

/// The type names a template parameter, at any depth.
pub fn has_dep(t: &QType) -> bool {
    match &t.t {
        Type::Dep(_) => true,
        Type::Ptr(x) | Type::Ref(x, _) | Type::Array(x, _) | Type::MemPtr(_, x) => has_dep(x),
        Type::Func(f) => has_dep(&f.ret) || f.params.iter().any(has_dep),
        _ => false,
    }
}

/// The type with each template parameter replaced by its argument
/// (TARG-1).
pub fn substitute(t: &QType, args: &[QType]) -> QType {
    let q = match &t.t {
        Type::Dep(d) => match &**d {
            DepType::Param { index, .. } => {
                let a = args.get(*index as usize).cloned().unwrap_or_else(QType::error);
                return a.with_cv(a.c || t.c, a.v || t.v);
            }
            DepType::Opaque { .. } => return t.clone(),
            DepType::Spec { template, args: xs, name } => {
                let xs = xs.iter().map(|x| match x {
                    TArg::Type(y) => TArg::Type(substitute(y, args)),
                    v => v.clone(),
                }).collect();
                Type::Dep(Rc::new(DepType::Spec { template: *template, args: xs, name: name.clone() }))
            }
        },
        Type::Ptr(x) => Type::Ptr(Rc::new(substitute(x, args))),
        Type::Ref(x, rv) => {
            // A reference to a reference collapses (DCL rules).
            let inner = substitute(x, args);
            if let Type::Ref(y, rv2) = &inner.t {
                return QType::new(Type::Ref(y.clone(), *rv && *rv2));
            }
            Type::Ref(Rc::new(inner), *rv)
        }
        Type::Array(x, n) => Type::Array(Rc::new(substitute(x, args)), *n),
        Type::MemPtr(c, x) => Type::MemPtr(*c, Rc::new(substitute(x, args))),
        Type::Func(f) => {
            let mut g = (**f).clone();
            g.ret = substitute(&f.ret, args);
            g.params = f.params.iter().map(|p| substitute(p, args)).collect();
            Type::Func(Rc::new(g))
        }
        other => other.clone(),
    };
    QType { t: q, c: t.c, v: t.v }
}

/// `a` without the cv-qualifiers `c` and `v` (`with_cv` only adds them).
fn without_cv(a: &QType, c: bool, v: bool) -> QType {
    match &a.t {
        Type::Array(..) | Type::Func(_) | Type::Ref(..) => a.clone(),
        _ => QType { t: a.t.clone(), c: a.c && !c, v: a.v && !v },
    }
}

/// The arguments of the class `c` as a specialization of the class
/// template `v`, if it is one.
fn spec_args(u: &crate::sem::Unit, v: usize, c: crate::sem::types::ClassId) -> Option<&Vec<TArg>> {
    u.class_templates.get(v)?.specs.iter().find(|(_, k)| *k == c).map(|(a, _)| a)
}

/// Match the arguments `ps` of a dependent specialization against the
/// arguments `xs` of a class, exactly.
fn match_args(u: &crate::sem::Unit, ps: &[TArg], xs: &[TArg], bound: &mut [Option<QType>]) -> bool {
    ps.len() == xs.len() && ps.iter().zip(xs).all(|(p, x)| match (p, x) {
        (TArg::Type(p), TArg::Type(x)) => match_exact(u, p, x, bound),
        (p, x) => p == x,
    })
}

/// Match a pattern type `p` against a type `a` exactly, binding the
/// parameters in `bound` (TSP-14, DED rules for template arguments).
pub fn match_exact(u: &crate::sem::Unit, p: &QType, a: &QType, bound: &mut [Option<QType>]) -> bool {
    if !has_dep(p) {
        return p == a;
    }
    match (&p.t, &a.t) {
        (Type::Dep(d), _) => match &**d {
            DepType::Param { index, .. } => {
                if (p.c && !a.c) || (p.v && !a.v) {
                    return false;
                }
                let t = without_cv(a, p.c, p.v);
                match &mut bound[*index as usize] {
                    Some(x) => *x == t,
                    slot => {
                        *slot = Some(t);
                        true
                    }
                }
            }
            DepType::Spec { template, args, .. } => {
                let Type::Class(c) = a.t else { return false };
                p.c == a.c && p.v == a.v && spec_args(u, *template, c).is_some_and(|xs| match_args(u, args, xs, bound))
            }
            DepType::Opaque { .. } => false,
        },
        _ if p.c != a.c || p.v != a.v => false,
        (Type::Ptr(x), Type::Ptr(y)) => match_exact(u, x, y, bound),
        (Type::Ref(x, r), Type::Ref(y, s)) => r == s && match_exact(u, x, y, bound),
        (Type::Array(x, n), Type::Array(y, m)) => n == m && match_exact(u, x, y, bound),
        _ => false,
    }
}

/// Match the parameter type `p` against the argument type `a`, binding
/// the template parameters in `bound`. False when they do not match.
fn match_type(u: &crate::sem::Unit, p: &QType, a: &QType, bound: &mut [Option<QType>]) -> bool {
    if !has_dep(p) {
        return true;
    }
    match (&p.t, &a.t) {
        // DED rules: a specialization deduces from the class of the
        // argument, or from one of its bases.
        (Type::Dep(d), Type::Class(c)) if matches!(&**d, DepType::Spec { .. }) => {
            let DepType::Spec { template, args, .. } = &**d else { return false };
            let mut classes = vec![*c];
            classes.extend(u.bases(*c).into_iter().map(|(_, b)| b));
            for k in classes {
                let Some(xs) = spec_args(u, *template, k) else { continue };
                let mut trial = bound.to_vec();
                if match_args(u, args, xs, &mut trial) {
                    bound.clone_from_slice(&trial);
                    return true;
                }
            }
            false
        }
        (Type::Dep(d), _) => {
            // A type after a dependent qualifier deduces nothing.
            let DepType::Param { index, .. } = &**d else { return matches!(&**d, DepType::Opaque { .. }) };
            // The parameter's own cv-qualifiers are not part of the
            // argument, which can have fewer; the argument keeps the rest.
            let t = without_cv(a, p.c, p.v);
            match &mut bound[*index as usize] {
                Some(x) => *x == t,
                slot => {
                    *slot = Some(t);
                    true
                }
            }
        }
        (Type::Ptr(x), Type::Ptr(y)) => match_type(u, x, y, bound),
        (Type::Array(x, n), Type::Array(y, m)) => n == m && match_type(u, x, y, bound),
        _ => false,
    }
}

/// DED-P3: the arguments of the function template `t` for a call with
/// `args`, after the explicit arguments `explicit`. None when a
/// parameter does not deduce or two deductions differ.
pub fn deduce_call(cx: &Check, t: usize, explicit: &[TArg], args: &[Expr]) -> Option<Vec<TArg>> {
    let tpl = &cx.u.templates[t];
    let n = tpl.params.len();
    if explicit.len() > n {
        return None;
    }
    let mut bound: Vec<Option<QType>> = (0..n).map(|k| match explicit.get(k) {
        Some(TArg::Type(t)) => Some(t.clone()),
        _ => None,
    }).collect();
    let fty = &tpl.fty;
    if args.len() > fty.params.len() && !fty.variadic {
        return None;
    }
    if args.len() < fty.params.len() {
        return None;
    }
    for (p, a) in fty.params.iter().zip(args) {
        let known: Vec<QType> = bound.iter().map(|b| b.clone().unwrap_or_else(QType::error)).collect();
        let p = if explicit.is_empty() { p.clone() } else { substitute_known(p, &bound, &known) };
        if !has_dep(&p) {
            continue;
        }
        let ok = match &p.t {
            Type::Ref(inner, rv) => {
                // A forwarding reference: an lvalue deduces `A&`.
                let forwarding = *rv && !inner.c && !inner.v && matches!(inner.t, Type::Dep(_));
                if forwarding && a.cat == Cat::L {
                    match_type(&cx.u, inner, &QType::lref(a.ty.nonref().clone()), &mut bound)
                } else {
                    match_type(&cx.u, inner, &a.ty.nonref(), &mut bound)
                }
            }
            _ => {
                let at = a.ty.nonref();
                let decayed = match &at.t {
                    Type::Array(el, _) => QType::ptr_to((**el).clone()),
                    Type::Func(_) => QType::ptr_to(at.clone()),
                    _ => at.unqual(),
                };
                match_type(&cx.u, &p.unqual(), &decayed, &mut bound)
            }
        };
        if !ok {
            return None;
        }
    }
    // A constant parameter takes its explicit argument; the first piece
    // of build step 5 deduces none.
    (0..n).map(|k| match &tpl.kinds[k] {
        None => bound[k].clone().map(TArg::Type),
        Some(_) => explicit.get(k).cloned().filter(|a| matches!(a, TArg::Value(..))),
    }).collect()
}

/// `p` with the template parameters that `bound` already holds replaced.
fn substitute_known(p: &QType, bound: &[Option<QType>], known: &[QType]) -> QType {
    match &p.t {
        Type::Dep(d) => {
            let DepType::Param { index, .. } = &**d else { return p.clone() };
            if bound[*index as usize].is_some() {
                substitute(p, known)
            } else {
                p.clone()
            }
        }
        Type::Ptr(x) => QType { t: Type::Ptr(Rc::new(substitute_known(x, bound, known))), ..p.clone() },
        Type::Ref(x, rv) => QType { t: Type::Ref(Rc::new(substitute_known(x, bound, known)), *rv), ..p.clone() },
        Type::Array(x, n) => QType { t: Type::Array(Rc::new(substitute_known(x, bound, known)), *n), ..p.clone() },
        _ => p.clone(),
    }
}
