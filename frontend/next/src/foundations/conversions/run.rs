//! The run part of `foundations/conversions`: the value of a converted
//! scalar.

use crate::machine::{ub, unsup, Machine, Val, R};
use crate::sem::types::{QType, Type};

/// Convert a value of type `from` to type `to`.
pub fn convert(m: &mut Machine, v: Val, from: &QType, to: &QType) -> R<Val> {
    let to_t = match &to.t {
        Type::Enum(e) => match m.u.enums.get(*e).and_then(|i| i.underlying) {
            Some(k) => Type::Int(k),
            None => return Ok(v),
        },
        t => t.clone(),
    };
    match (v, &to_t) {
        // CONV-3: zero, a null pointer, and false give false.
        (v, Type::Int(k)) if *k == crate::sem::types::IntK::Bool => Ok(Val::Int(v.truthy() as i128)),
        // CONV-2: an integer keeps its low bits.
        (Val::Int(x), Type::Int(k)) => Ok(Val::Int(m.target.wrap(*k, x))),
        // CONV-6, CONV-7: to a floating type, rounded to the target.
        (Val::Int(x), Type::Float(k)) => Ok(Val::Float(m.round_float(x as f64, *k))),
        (Val::Float(f), Type::Float(k)) => Ok(Val::Float(m.round_float(f, *k))),
        // CONV-4, CONV-5: toward zero; undefined outside the range.
        (Val::Float(f), Type::Int(k)) => {
            let t = f.trunc();
            if !t.is_finite() || t < m.target.int_min(*k) as f64 || t > m.target.int_max(*k) as f64 {
                return ub("CONV-5", format!("{} is outside the range of {}", f, k.name()));
            }
            Ok(Val::Int(t as i128))
        }
        (v @ (Val::Ptr(_) | Val::Null | Val::Fn(_) | Val::Addr(_)), Type::Ptr(_) | Type::NullPtr) => Ok(v),
        (v, _) if from.t == to_t => Ok(v),
        (v, _) => unsup(format!("a conversion of {:?} from {} to {}", v, from.show(), to.show())),
    }
}

/// The `reinterpret_cast` conversions between pointers and integers.
pub fn reinterpret(m: &mut Machine, v: Val, _from: &QType, to: &QType) -> R<Val> {
    if m.constant {
        return crate::foundations::constant_evaluation::run::reinterpret();
    }
    match (v, &to.t) {
        (Val::Null, Type::Int(_)) => Ok(Val::Int(0)),
        (Val::Int(0), Type::Ptr(_)) => Ok(Val::Null),
        (Val::Int(x), Type::Ptr(_)) => Ok(Val::Addr(x as u64)),
        (Val::Addr(a), Type::Int(k)) => Ok(Val::Int(m.target.wrap(*k, a as i128))),
        (v @ (Val::Ptr(_) | Val::Null | Val::Addr(_)), Type::Ptr(_)) => Ok(v),
        _ => unsup("a reinterpret_cast of an address the machine does not model"),
    }
}
