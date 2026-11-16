//! The check part of `overloading/overload-resolution` for calls whose
//! candidates are functions and whose arguments are not classes (build
//! step 3). Templates, members, and class arguments come with later
//! steps; a call that needs them is unsupported.

use crate::overloading::conversion_sequences::check::{better, sequence, Ics};
use crate::sem::check::Check;
use crate::sem::tree::Expr;
use crate::sem::types::QType;
use crate::sem::EntityId;
use crate::source::Span;

/// Whether two expressions are the same (OVL-38): the same operations on
/// the same entities and literals, whatever the spelling or the
/// parentheses.
fn same_expr(a: &Expr, b: &Expr) -> bool {
    use crate::sem::tree::EK::*;
    let same_args = |x: &[crate::sem::tree::Arg], y: &[crate::sem::tree::Arg]| {
        x.len() == y.len() && x.iter().zip(y).all(|(p, q)| p.by_ref == q.by_ref && same_expr(&p.e, &q.e))
    };
    a.ty == b.ty
        && match (&a.k, &b.k) {
            (Int(x), Int(y)) => x == y,
            (Float(x), Float(y)) => x == y,
            (Str(x), Str(y)) => x == y,
            (Var(x), Var(y)) | (Func(x), Func(y)) | (Closure(x), Closure(y)) => x == y,
            (Conv(c, x), Conv(d, y)) => c == d && same_expr(x, y),
            (Unary(o, x), Unary(p, y)) => o == p && same_expr(x, y),
            (Binary(o, x1, x2), Binary(p, y1, y2)) => o == p && same_expr(x1, y1) && same_expr(x2, y2),
            (PtrAdd(x1, x2, s), PtrAdd(y1, y2, t)) => s == t && same_expr(x1, y1) && same_expr(x2, y2),
            (Logical(o, x1, x2), Logical(p, y1, y2)) => o == p && same_expr(x1, y1) && same_expr(x2, y2),
            (PtrDiff(x1, x2), PtrDiff(y1, y2)) | (Assign(x1, x2), Assign(y1, y2)) | (Comma(x1, x2), Comma(y1, y2)) | (Index(x1, x2), Index(y1, y2)) => {
                same_expr(x1, y1) && same_expr(x2, y2)
            }
            (Compound(o, x1, x2, _), Compound(p, y1, y2, _)) => o == p && same_expr(x1, y1) && same_expr(x2, y2),
            (IncDec(a1, a2, x), IncDec(b1, b2, y)) => a1 == b1 && a2 == b2 && same_expr(x, y),
            (Cond(x1, x2, x3), Cond(y1, y2, y3)) => same_expr(x1, y1) && same_expr(x2, y2) && same_expr(x3, y3),
            (Call(f, xs, r), Call(g, ys, s)) => r == s && same_expr(f, g) && same_args(xs, ys),
            (Deref(x), Deref(y)) | (AddrOf(x), AddrOf(y)) | (Unevaluated(x), Unevaluated(y)) | (Assume(x), Assume(y)) => same_expr(x, y),
            _ => false,
        }
}

/// OVL-1: the declarations of one function with C language linkage in
/// several namespaces (LSP-4) are one candidate. The candidates come back
/// with each such function once, and with the functions whose
/// declarations give different default arguments that the call uses
/// (OVL-38).
fn merge_c_functions(cx: &Check, cands: &[EntityId], nargs: usize) -> (Vec<EntityId>, Vec<EntityId>) {
    let mut out: Vec<EntityId> = Vec::new();
    let mut differ: Vec<EntityId> = Vec::new();
    for &f in cands {
        let Some(func) = cx.u.func(f) else {
            out.push(f);
            continue;
        };
        let twin = out.iter().copied().find(|&g| {
            cx.u.func(g).is_some_and(|h| h.c_linkage && func.c_linkage && h.ty == func.ty && cx.u.entities[g].name == cx.u.entities[f].name)
        });
        let Some(g) = twin else {
            out.push(f);
            continue;
        };
        let (mine, theirs) = (func.defaults.clone(), cx.u.func(g).map(|h| h.defaults.clone()).unwrap_or_default());
        let clash = (nargs..mine.len().min(theirs.len())).any(|k| matches!((&mine[k], &theirs[k]), (Some(a), Some(b)) if !same_expr(a, b)));
        // A default that only the later declaration gives serves the call.
        let kept = if mine.iter().zip(theirs.iter()).any(|(a, b)| a.is_some() && b.is_none()) {
            let pos = out.iter().position(|&h| h == g).unwrap();
            out[pos] = f;
            f
        } else {
            g
        };
        if clash || differ.contains(&g) {
            differ.push(kept);
        }
    }
    (out, differ)
}

/// The candidate that a call chooses, or None after an error.
pub fn resolve(cx: &mut Check, cands: &[EntityId], args: &[Expr], span: Span, name: &str) -> Option<EntityId> {
    resolve_with(cx, cands, None, args, span, name)
}

/// The object of a call of a member function meets the implicit object
/// parameter of each non-static candidate: an lvalue reference to the
/// class with the function's cv-qualifiers, which also binds an rvalue
/// (ICS-13, ICS-40). A static member function matches any object.
pub fn object_sequence(cx: &Check, f: EntityId, object: &Expr) -> Option<Ics> {
    let func = cx.u.func(f)?;
    // ICS-41: the object initializes an explicit object parameter as an
    // argument.
    if func.explicit_object {
        return sequence(cx, object, &func.ty.params[0]);
    }
    let Some(c) = func.class.filter(|_| !func.member_static) else {
        return Some(Ics { rank: crate::overloading::conversion_sequences::check::Rank::Exact, qual: false, binds: None, ellipsis: false, user: None, extra: Default::default() });
    };
    // OVL rules: a function that a using-declaration brings into the
    // object's class takes that class as its object type.
    let c = object.ty.nonref().class().filter(|&o| cx.u.using_access.contains_key(&(o, f))).unwrap_or(c);
    let class = QType::new(crate::sem::types::Type::Class(c)).with_cv(func.obj_cv.0, func.obj_cv.1);
    // CLS-6, ICS-40: with a ref-qualifier, the object binds as an
    // argument of that reference type; without one, an rvalue binds too.
    match func.ref_q {
        Some(rv) => sequence(cx, object, &QType::new(crate::sem::types::Type::Ref(std::rc::Rc::new(class), rv))),
        None => {
            let mut o = object.clone();
            o.cat = crate::sem::tree::Cat::L;
            sequence(cx, &o, &QType::lref(class))
        }
    }
}

/// Overload resolution with the object of a member call, if any.
pub fn resolve_with(cx: &mut Check, cands: &[EntityId], object: Option<&Expr>, args: &[Expr], span: Span, name: &str) -> Option<EntityId> {
    // CNS-11: a member function whose requires clause is not satisfied
    // is no candidate.
    let mut kept: Vec<EntityId> = Vec::new();
    for &f in cands {
        match crate::templates::concepts::check::member_satisfied(cx, f) {
            Some(true) => kept.push(f),
            Some(false) => {}
            None => {
                cx.unsupported(span, format!("a call of '{}' whose candidate's requires clause the front end does not check", name));
                return None;
            }
        }
    }
    if kept.is_empty() && !cands.is_empty() {
        cx.error_cause(span, "OVL-35", Some("CNS-12"), format!("no function '{}' fits: the requires clauses are not satisfied", name));
        return None;
    }
    let cands = &kept[..];
    if cands.len() > 1 {
        let closure = cands.iter().filter_map(|&f| cx.u.func(f)).any(|f| {
            f.ty.params.iter().zip(args).any(|(p, a)| p.is_ptr() && a.ty.nonref().class().is_some_and(|c| cx.u.closures.contains_key(&c)))
        });
        if closure {
            cx.unsupported(span, "a conversion of a closure to a pointer to function in overload resolution");
            return None;
        }
    }
    let (merged, differ) = merge_c_functions(cx, cands, args.len());
    let cands = &merged[..];
    let mut viable: Vec<(EntityId, Vec<Ics>)> = Vec::new();
    for &f in cands {
        let Some(func) = cx.u.func(f) else { continue };
        // With an object, the arguments meet the parameters after an
        // explicit object parameter.
        let skip = usize::from(object.is_some() && func.explicit_object);
        let params: Vec<QType> = func.ty.params[skip..].to_vec();
        let variadic = func.ty.variadic;
        let own_defaults = &func.defaults[skip.min(func.defaults.len())..];
        let defaults = own_defaults.iter().filter(|d| d.is_some()).count();
        // OVL-22: one parameter for each argument, with defaults or an
        // ellipsis for the rest.
        let n = params.len();
        if args.len() > n && !variadic {
            continue;
        }
        if args.len() + defaults < n {
            let first_default = own_defaults.iter().position(|d| d.is_some()).unwrap_or(n);
            if args.len() < first_default {
                continue;
            }
        }
        let mut seqs = Vec::new();
        let mut ok = true;
        let member = func.class.is_some() && !func.member_static && func.special == crate::sem::Special::None;
        match object {
            Some(o) => match object_sequence(cx, f, o) {
                Some(s) => seqs.push(s),
                None => continue,
            },
            None if member => continue,
            None => {}
        }
        for (k, a) in args.iter().enumerate() {
            if k >= n {
                // ICS-6: an overload set has no sequence to an ellipsis.
                if crate::expressions::primary::check::unresolved_set(a) {
                    ok = false;
                    break;
                }
                seqs.push(Ics { rank: crate::overloading::conversion_sequences::check::Rank::Conversion, qual: false, binds: None, ellipsis: true, user: None, extra: Default::default() });
                continue;
            }
            // ICS-4: a class parameter takes another class or a value by
            // a converting constructor, or a class object by a conversion
            // function to the class.
            let s = sequence(cx, a, &params[k]).or_else(|| {
                let p = &params[k];
                if !p.nonref().is_class() {
                    return None;
                }
                crate::classes::operators::check::user_conversion(cx, a, p).or_else(|| crate::classes::conversion_functions::check::user_sequence(cx, a, p)).map(|mut s| {
                    // The reference binds the result of the conversion.
                    if let crate::sem::types::Type::Ref(inner, rv) = &p.t {
                        s.binds = Some((*rv, true, (**inner).clone()));
                    }
                    s
                })
            });
            match s {
                Some(s) => seqs.push(s),
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            viable.push((f, seqs));
        }
    }
    if viable.is_empty() {
        if cands.len() == 1 {
            // One candidate: the call's own checks name the fault
            // (PFX-5 for the count, the parameter's initialization for a
            // type).
            return Some(cands[0]);
        }
        cx.error(span, "OVL-35", format!("no function '{}' fits the arguments", name));
        return None;
    }
    // OVL-29: the best viable function is better than every other.
    // OVL-31: a constructor of the derived class beats an inherited one.
    let ctor_class = |f: EntityId| cx.u.func(f).filter(|x| x.special == crate::sem::Special::Ctor).and_then(|x| x.class);
    let inherited_tie = |f: EntityId, g: EntityId| match (ctor_class(f), ctor_class(g)) {
        (Some(a), Some(b)) => crate::classes::derived::check::is_base(cx, a, b),
        _ => false,
    };
    // ORD-13, CNS-23: of two functions with the same parameter types that
    // are no templates, the more constrained.
    let constrained_tie = |f: EntityId, g: EntityId| -> bool {
        let (Some(x), Some(y)) = (cx.u.func(f), cx.u.func(g)) else { return false };
        if x.ty.params != y.ty.params || crate::templates::declarations::check::template_of(cx, f).is_some() || crate::templates::declarations::check::template_of(cx, g).is_some() {
            return false;
        }
        let form = |h: EntityId| cx.u.member_constraints.get(&h).map(|(n, _)| crate::templates::concepts::check::associated_form(cx, &[], &[(None, n.clone())]));
        let (fa, fb) = (form(f), form(g));
        if fa.as_ref().is_some_and(|x| !x.1) || fb.as_ref().is_some_and(|x| !x.1) || (fa.is_none() && fb.is_none()) {
            return false;
        }
        let (pa, pb) = (fa.and_then(|x| x.0), fb.and_then(|x| x.0));
        crate::templates::concepts::check::at_least_as_constrained(pa.as_ref(), pb.as_ref()) && !crate::templates::concepts::check::at_least_as_constrained(pb.as_ref(), pa.as_ref())
    };
    let wins = |a: &(EntityId, Vec<Ics>), b: &(EntityId, Vec<Ics>)| -> bool {
        let mut some = false;
        for (x, y) in a.1.iter().zip(b.1.iter()) {
            if better(y, x) {
                return false;
            }
            if better(x, y) {
                some = true;
            }
        }
        some || inherited_tie(a.0, b.0) || crate::templates::declarations::check::beats_by_template(cx, a.0, b.0) == Some(true) || constrained_tie(a.0, b.0)
    };
    let mut best = 0;
    for i in 1..viable.len() {
        if wins(&viable[i], &viable[best]) {
            best = i;
        }
    }
    for i in 0..viable.len() {
        if i != best && !wins(&viable[best], &viable[i]) {
            // Two specializations that partial ordering does not order by
            // the steps that the first pieces of build step 5 take, or two
            // functions that only their constraints order (CNS-22).
            let (f, g) = (viable[best].0, viable[i].0);
            // CNS-24: neither of two constrained functions is more
            // constrained.
            if cx.u.member_constraints.contains_key(&f) || cx.u.member_constraints.contains_key(&g) {
                if cx.u.func(f).zip(cx.u.func(g)).is_some_and(|(x, y)| x.ty.params == y.ty.params) {
                    cx.error_cause(span, "OVL-36", Some("CNS-24"), format!("the call of '{}' is ambiguous: neither function is more constrained", name));
                    return None;
                }
                cx.unsupported(span, format!("a call of '{}' that the subsumption of constraints decides", name));
                return None;
            }
            if crate::templates::declarations::check::beats_by_template(cx, f, g).is_none() || crate::templates::declarations::check::beats_by_template(cx, g, f).is_none() {
                cx.unsupported(span, format!("a call of '{}' that needs a step of partial ordering that the front end does not take", name));
                return None;
            }
            cx.error(span, "OVL-36", format!("the call of '{}' is ambiguous", name));
            return None;
        }
    }
    if differ.contains(&viable[best].0) {
        cx.error(span, "OVL-38", format!("the call of '{}' uses a default argument that two scopes give differently", name));
        return None;
    }
    Some(viable[best].0)
}
