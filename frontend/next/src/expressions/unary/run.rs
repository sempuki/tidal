//! The run part of `expressions/unary`: unary `-`, indirection, and
//! prefix `++` and `--`.

use crate::machine::{ub, unsup, Machine, Ptr, Val, R};
use crate::sem::tree::UOp;
use crate::sem::types::{QType, Type};

/// A unary arithmetic operator on a promoted operand. UN-4: `-` on a
/// signed value whose negative does not fit.
pub fn unary(m: &mut Machine, op: UOp, v: Val, ty: &QType) -> R<Val> {
    match (op, v, &ty.t) {
        (UOp::Neg, Val::Int(x), Type::Int(k)) => {
            if k.signed() && !m.target.fits(*k, -x) {
                return ub("UN-4", format!("-({}) does not fit in {}", x, k.name()));
            }
            Ok(Val::Int(m.target.wrap(*k, -x)))
        }
        (UOp::Neg, Val::Float(f), _) => Ok(Val::Float(-f)),
        (op, v, _) => crate::machine::eval::simple_unary(m, op, v, ty),
    }
}

/// UN-7: `*p` where `p` points to no object.
pub fn indirection(m: &mut Machine, v: Val, _ty: &QType) -> R<Ptr> {
    match v {
        Val::Ptr(p) => {
            if p.array {
                let n = m.array_len(&p).unwrap_or(1);
                let i = *p.path.last().unwrap_or(&0);
                if i < 0 || i >= n {
                    return ub("UN-7", "indirection through a pointer past the end of an array");
                }
            }
            Ok(p)
        }
        Val::Null => ub("UN-7", "indirection through a null pointer"),
        Val::Addr(a) => ub("UN-7", format!("indirection through the address {:#x}, where no object is", a)),
        Val::Uninit => crate::declarations::initialization::forms::run::read_no_value("the pointer"),
        _ => unsup("indirection through this value"),
    }
}

/// Prefix `++` and `--`. UN-21: a signed result out of range, or a
/// pointer that leaves its array.
pub fn increment(m: &mut Machine, v: Val, inc: bool, ty: &QType) -> R<Val> {
    let d = if inc { 1 } else { -1 };
    match (v, &ty.t) {
        (Val::Int(x), Type::Int(k)) if *k == crate::sem::types::IntK::Bool => Ok(Val::Int(if inc { 1 } else { x })),
        (Val::Int(x), Type::Int(k)) => {
            let r = x + d;
            if k.signed() && k.rank() >= 3 && !m.target.fits(*k, r) {
                return ub("UN-21", format!("{} {} 1 does not fit in {}", x, if inc { "+" } else { "-" }, k.name()));
            }
            Ok(Val::Int(m.target.wrap(*k, r)))
        }
        (Val::Int(x), Type::Enum(_)) => Ok(Val::Int(x + d)),
        (Val::Float(f), Type::Float(k)) => Ok(Val::Float(m.round_float(f + d as f64, *k))),
        (Val::Ptr(p), Type::Ptr(_)) => match crate::expressions::additive::run::moved(m, p, d) {
            Some(p) => Ok(Val::Ptr(p)),
            None => ub("UN-21", "the pointer moves outside its array"),
        },
        (Val::Null, Type::Ptr(_)) => ub("UN-21", "an increment of a null pointer"),
        _ => unsup("an increment of this value"),
    }
}
