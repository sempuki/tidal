//! The run part of `expressions/comparison`.

use crate::machine::{unsup, Machine, Ptr, Val, R};
use crate::sem::tree::BOp;
use crate::sem::types::QType;

fn order(a: std::cmp::Ordering, op: BOp) -> i128 {
    use std::cmp::Ordering::*;
    (match op {
        BOp::Lt => a == Less,
        BOp::Gt => a == Greater,
        BOp::Le => a != Greater,
        BOp::Ge => a != Less,
        BOp::Eq => a == Equal,
        BOp::Ne => a != Equal,
        _ => false,
    }) as i128
}

fn norm(p: &Ptr) -> Ptr {
    let mut p = p.clone();
    if !p.array {
        p.path.push(0);
        p.array = true;
    }
    p
}

/// The position of a place among the scalar elements of its complete
/// object, so that the end of one row equals the start of the next
/// (CMP-26).
fn flat(m: &Machine, p: &Ptr) -> Option<i64> {
    fn count(t: &crate::sem::types::QType) -> i64 {
        match &t.t {
            crate::sem::types::Type::Array(e, Some(n)) => *n as i64 * count(e),
            _ => 1,
        }
    }
    let mut t = m.objs.get(p.obj)?.ty.clone();
    let mut off = 0;
    for &i in &p.path {
        match t.t.clone() {
            crate::sem::types::Type::Array(e, _) => {
                off += i * count(&e);
                t = (*e).clone();
            }
            _ => off += i * count(&t),
        }
    }
    Some(off)
}

/// A comparison of two values of the common type.
pub fn compare(m: &mut Machine, op: BOp, a: Val, b: Val, ty: &QType) -> R<Val> {
    if ty.t == crate::sem::types::Type::Float(crate::sem::types::FloatK::LongDouble) {
        return unsup("a comparison of long double values (the machine holds a double's precision)");
    }
    if op == BOp::Spaceship {
        return unsup("the three-way comparison's result type");
    }
    match (&a, &b) {
        (Val::Int(x), Val::Int(y)) => Ok(Val::Int(order(x.cmp(y), op))),
        (Val::Float(x), Val::Float(y)) => match x.partial_cmp(y) {
            Some(o) => Ok(Val::Int(order(o, op))),
            None => Ok(Val::Int((op == BOp::Ne) as i128)),
        },
        (Val::Ptr(x), Val::Ptr(y)) => {
            let (x, y) = (norm(x), norm(y));
            if x.obj == y.obj {
                if let (Some(a), Some(b)) = (flat(m, &x), flat(m, &y)) {
                    return Ok(Val::Int(order(a.cmp(&b), op)));
                }
            }
            let same_array = x.obj == y.obj && x.path[..x.path.len() - 1] == y.path[..y.path.len() - 1];
            if matches!(op, BOp::Eq | BOp::Ne) {
                if m.constant && x.obj != y.obj && m.objs[x.obj].string && m.objs[y.obj].string {
                    return crate::foundations::constant_evaluation::run::string_compare();
                }
                return Ok(Val::Int(order(if x == y { std::cmp::Ordering::Equal } else { std::cmp::Ordering::Less }, op)));
            }
            if !same_array {
                if m.constant {
                    return crate::foundations::constant_evaluation::run::unrelated_compare();
                }
                // CMP-8: the order of unrelated objects is unspecified;
                // the machine orders them by creation.
                return Ok(Val::Int(order((x.obj, x.path.clone()).cmp(&(y.obj, y.path.clone())), op)));
            }
            Ok(Val::Int(order(x.path.cmp(&y.path), op)))
        }
        (Val::Null, Val::Null) => Ok(Val::Int(order(std::cmp::Ordering::Equal, op))),
        (Val::Null, _) | (_, Val::Null) if matches!(op, BOp::Eq | BOp::Ne) => Ok(Val::Int(order(std::cmp::Ordering::Less, op))),
        (Val::Fn(x), Val::Fn(y)) if matches!(op, BOp::Eq | BOp::Ne) => Ok(Val::Int(order(x.cmp(y), op))),
        _ => unsup("a comparison of these values"),
    }
}
