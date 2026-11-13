//! The run part of `foundations/conversions`: the value of a converted
//! scalar.

use crate::machine::{ub, unsup, Machine, Val, R};
use crate::foundations::types::run::F80;
use crate::sem::types::{FloatK, QType, Type};

/// A floating value in the format of `k`: rounded to `float` or
/// `double`, or exact in the extended format of `long double`.
pub fn to_float(v: Val, k: FloatK) -> Val {
    match (v, k) {
        (Val::Float(f), FloatK::LongDouble) => Val::Ext(F80::from_f64(f)),
        (Val::Float(f), FloatK::Float) => Val::Float(f as f32 as f64),
        (Val::Ext(x), FloatK::LongDouble) => Val::Ext(x),
        (Val::Ext(x), k) => Val::Float(x.to_f64(k == FloatK::Float)),
        (v, _) => v,
    }
}

/// Convert a value of type `from` to type `to`.
pub fn convert(m: &mut Machine, v: Val, from: &QType, to: &QType) -> R<Val> {
    if let (Type::Enum(e), Val::Int(x)) = (&to.t, &v) {
        if from.t != to.t {
            crate::declarations::enumerations::run::check_value(m, *e, *x)?;
        }
    }
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
        (Val::Int(x), Type::Float(FloatK::LongDouble)) => Ok(Val::Ext(F80::from_int(x))),
        (Val::Int(x), Type::Float(k)) => Ok(Val::Float(m.round_float(x as f64, *k))),
        (v @ (Val::Float(_) | Val::Ext(_)), Type::Float(k)) => Ok(to_float(v, *k)),
        (Val::Ext(x), Type::Int(k)) => match x.trunc().filter(|&t| t >= m.target.int_min(*k) && t <= m.target.int_max(*k)) {
            Some(t) => Ok(Val::Int(t)),
            None => ub("CONV-5", format!("{} is outside the range of {}", x.to_f64(false), k.name())),
        },
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
    use crate::foundations::storage::run as storage;
    match (v, &to.t) {
        (Val::Null, Type::Int(_)) => Ok(Val::Int(0)),
        (Val::Int(0), Type::Ptr(_)) => Ok(Val::Null),
        // CAST-16: the address in both directions.
        (Val::Int(x), Type::Ptr(p)) => Ok(storage::from_address(m, x as u64, p)),
        (v @ (Val::Addr(_) | Val::Ptr(_) | Val::Fn(_)), Type::Int(k)) => match storage::address(m, &v) {
            Some(a) => Ok(Val::Int(m.target.wrap(*k, a as i128))),
            None => unsup("a reinterpret_cast of an address the machine does not model"),
        },
        (Val::Ptr(p), Type::Ptr(t)) => Ok(Val::Ptr(storage::convert(m, p, t))),
        (Val::Fn(id), Type::Ptr(t)) if t.is_func() => Ok(Val::Fn(id)),
        (v @ Val::Fn(_), Type::Ptr(t)) => match storage::address(m, &v) {
            Some(a) => Ok(storage::from_address(m, a, t)),
            None => unsup("a reinterpret_cast of an address the machine does not model"),
        },
        (v @ (Val::Null | Val::Addr(_)), Type::Ptr(_)) => Ok(v),
        (Val::Addr(a), Type::Ptr(t)) => Ok(storage::from_address(m, a, t)),
        _ => unsup("a reinterpret_cast of an address the machine does not model"),
    }
}
