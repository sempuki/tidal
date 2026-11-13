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
            (Call(f, xs), Call(g, ys)) => same_expr(f, g) && same_args(xs, ys),
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
fn object_sequence(cx: &Check, f: EntityId, object: &Expr) -> Option<Ics> {
    let func = cx.u.func(f)?;
    let Some(c) = func.class.filter(|_| !func.member_static) else {
        return Some(Ics { rank: crate::overloading::conversion_sequences::check::Rank::Exact, qual: false, binds: None, ellipsis: false });
    };
    let param = QType::lref(QType::new(crate::sem::types::Type::Class(c)).with_cv(func.obj_cv.0, func.obj_cv.1));
    let mut o = object.clone();
    o.cat = crate::sem::tree::Cat::L;
    sequence(cx, &o, &param)
}

/// Overload resolution with the object of a member call, if any.
pub fn resolve_with(cx: &mut Check, cands: &[EntityId], object: Option<&Expr>, args: &[Expr], span: Span, name: &str) -> Option<EntityId> {
    // A conversion by a converting constructor is a user-defined
    // conversion sequence, which a later piece models (ICS-4).
    if cands.len() > 1 {
        let udc = cands.iter().filter_map(|&f| cx.u.func(f)).any(|f| {
            f.ty.params.iter().zip(args).any(|(p, a)| {
                p.nonref().class().is_some_and(|c| crate::classes::constructors::check::has_ctors(cx, c) && a.ty.nonref().class() != Some(c))
            })
        });
        if udc {
            cx.unsupported(span, "a user-defined conversion by a constructor in overload resolution (build step 4)");
            return None;
        }
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
        let ty = func.ty.clone();
        let defaults = func.defaults.iter().filter(|d| d.is_some()).count();
        // OVL-22: one parameter for each argument, with defaults or an
        // ellipsis for the rest.
        let n = ty.params.len();
        if args.len() > n && !ty.variadic {
            continue;
        }
        if args.len() + defaults < n {
            let first_default = func.defaults.iter().position(|d| d.is_some()).unwrap_or(n);
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
                seqs.push(Ics { rank: crate::overloading::conversion_sequences::check::Rank::Conversion, qual: false, binds: None, ellipsis: true });
                continue;
            }
            match sequence(cx, a, &ty.params[k]) {
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
    let wins = |a: &Vec<Ics>, b: &Vec<Ics>| -> bool {
        let mut some = false;
        for (x, y) in a.iter().zip(b.iter()) {
            if better(y, x) {
                return false;
            }
            if better(x, y) {
                some = true;
            }
        }
        some
    };
    let mut best = 0;
    for i in 1..viable.len() {
        if wins(&viable[i].1, &viable[best].1) {
            best = i;
        }
    }
    for i in 0..viable.len() {
        if i != best && !wins(&viable[best].1, &viable[i].1) {
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
