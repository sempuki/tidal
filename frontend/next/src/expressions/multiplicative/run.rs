//! The run part of `expressions/multiplicative`: `*`, `/`, and `%`.

use crate::machine::{ub, unsup, Machine, Val, R};
use crate::sem::tree::BOp;
use crate::sem::types::{QType, Type};

/// MUL-9: division by zero; MUL-10: a signed result that does not fit;
/// MUL-12: a finite floating result out of range.
pub fn binary(m: &mut Machine, op: BOp, a: Val, b: Val, ty: &QType) -> R<Val> {
    match (a, b, &ty.t) {
        (Val::Int(x), Val::Int(y), Type::Int(k)) => {
            if op != BOp::Mul && y == 0 {
                return ub("MUL-9", "division by zero");
            }
            let r = match op {
                BOp::Mul => x * y,
                BOp::Div => x / y,
                _ => x % y,
            };
            if k.signed() {
                let q = if op == BOp::Mul { r } else { x / y };
                if !m.target.fits(*k, q) {
                    return ub("MUL-10", format!("the result of {} and {} does not fit in {}", x, y, k.name()));
                }
            }
            Ok(Val::Int(m.target.wrap(*k, r)))
        }
        (Val::Float(x), Val::Float(y), Type::Float(k)) => {
            if op == BOp::Div && y == 0.0 && m.constant {
                return crate::foundations::constant_evaluation::run::float_divide_by_zero();
            }
            let r = if op == BOp::Mul { x * y } else { x / y };
            let r2 = m.round_float(r, *k);
            if x.is_finite() && y.is_finite() && y != 0.0 && !r2.is_finite() {
                return ub("MUL-12", "the result is outside the range of its type");
            }
            Ok(Val::Float(r2))
        }
        (Val::Ext(x), Val::Ext(y), _) => {
            if op == BOp::Div && y.is_zero() && m.constant {
                return crate::foundations::constant_evaluation::run::float_divide_by_zero();
            }
            let r = if op == BOp::Mul { x.mul(y) } else { x.div(y) };
            if x.is_finite() && y.is_finite() && !y.is_zero() && !r.is_finite() {
                return ub("MUL-12", "the result is outside the range of its type");
            }
            Ok(Val::Ext(r))
        }
        _ => unsup("a multiplicative operator on these values"),
    }
}
