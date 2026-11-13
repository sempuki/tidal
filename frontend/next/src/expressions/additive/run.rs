//! The run part of `expressions/additive`: `+` and `-` on arithmetic
//! values and on pointers.

use crate::machine::{ub, unsup, Machine, Ptr, Val, R};
use crate::sem::tree::BOp;
use crate::sem::types::{QType, Type};

/// `+` or `-` on two values of the common type. ADD-9: a signed result
/// out of range; ADD-17: a finite floating result out of range.
pub fn binary(m: &mut Machine, op: BOp, a: Val, b: Val, ty: &QType) -> R<Val> {
    match (a, b, &ty.t) {
        (Val::Int(x), Val::Int(y), Type::Int(k)) => {
            let r = if op == BOp::Add { x + y } else { x - y };
            if k.signed() && !m.target.fits(*k, r) {
                return ub("ADD-9", format!("{} {} {} does not fit in {}", x, if op == BOp::Add { "+" } else { "-" }, y, k.name()));
            }
            Ok(Val::Int(m.target.wrap(*k, r)))
        }
        (Val::Float(x), Val::Float(y), Type::Float(k)) => {
            let r = if op == BOp::Add { x + y } else { x - y };
            let r2 = m.round_float(r, *k);
            if x.is_finite() && y.is_finite() && !r2.is_finite() {
                return ub("ADD-17", "the result is outside the range of its type");
            }
            Ok(Val::Float(r2))
        }
        (Val::Ext(x), Val::Ext(y), _) => {
            let r = if op == BOp::Add { x.add(y) } else { x.sub(y) };
            if x.is_finite() && y.is_finite() && !r.is_finite() {
                return ub("ADD-17", "the result is outside the range of its type");
            }
            Ok(Val::Ext(r))
        }
        _ => unsup("an additive operator on these values"),
    }
}

/// A pointer moved by `d` elements, if it stays in its array or one
/// past its end (ADD-10).
pub fn moved(m: &Machine, p: Ptr, d: i128) -> Option<Ptr> {
    if p.view.is_some() {
        return crate::foundations::storage::run::moved(m, &p, d as i64).ok();
    }
    let mut q = p;
    if !q.array {
        q.path.push(0);
        q.array = true;
    }
    let n = m.array_len(&q)? as i128;
    let i = *q.path.last()? as i128 + d;
    if i < 0 || i > n {
        return None;
    }
    *q.path.last_mut()? = i as i64;
    Some(q)
}

/// `p + n`, `n + p`, `p - n`. ADD-10: the result leaves the array, or a
/// null pointer moves.
pub fn ptr_add(m: &mut Machine, p: Val, n: Val, sub: bool, _ty: &QType) -> R<Val> {
    let Val::Int(n) = n else { return unsup("pointer arithmetic with this value") };
    let d = if sub { -n } else { n };
    match p {
        Val::Null if d == 0 => Ok(Val::Null),
        Val::Null => ub("ADD-10", "arithmetic on a null pointer"),
        Val::Ptr(p) if p.view.is_some() => crate::foundations::storage::run::moved(m, &p, d as i64).map(Val::Ptr),
        Val::Ptr(p) => match moved(m, p, d) {
            Some(q) => Ok(Val::Ptr(q)),
            None => ub("ADD-10", "the result points outside the array"),
        },
        Val::Uninit => crate::declarations::initialization::forms::run::read_no_value("the pointer"),
        _ => unsup("pointer arithmetic on this value"),
    }
}

/// `p - q`. ADD-11: pointers into different arrays.
pub fn ptr_diff(m: &mut Machine, a: Val, b: Val, ty: &QType) -> R<Val> {
    let norm = |p: Ptr| -> Ptr {
        let mut p = p;
        if !p.array {
            p.path.push(0);
            p.array = true;
        }
        p
    };
    match (a, b) {
        (Val::Null, Val::Null) => Ok(Val::Int(0)),
        // STG-13: byte pointers count bytes, and a view counts its
        // pointee's size.
        (Val::Ptr(x), Val::Ptr(y)) if x.view.is_some() || y.view.is_some() => {
            let size = x.view.as_ref().or(y.view.as_ref()).map_or(1, |v| v.size);
            match crate::foundations::storage::run::offsets(m, &x, &y) {
                Some((a, b)) => Ok(Val::Int(((a - b) / size) as i128)),
                None => ub("ADD-11", "the pointers do not point into the same object"),
            }
        }
        (Val::Ptr(x), Val::Ptr(y)) => {
            let (x, y) = (norm(x), norm(y));
            if x.obj != y.obj || x.path[..x.path.len() - 1] != y.path[..y.path.len() - 1] {
                return ub("ADD-11", "the pointers do not point into the same array");
            }
            let d = (*x.path.last().unwrap() - *y.path.last().unwrap()) as i128;
            match ty.int_kind() {
                Some(k) if !m.target.fits(k, d) => ub("ADD-18", "the difference does not fit in its type"),
                _ => Ok(Val::Int(d)),
            }
        }
        _ => ub("ADD-11", "the pointers do not point into the same array"),
    }
}
