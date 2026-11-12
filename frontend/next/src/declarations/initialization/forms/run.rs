//! The run part of `declarations/initialization/forms`: the value a new
//! object gets from its initializer.

use crate::machine::eval::{eval_any, place};
use crate::machine::{ub, Machine, Val, R};
use crate::sem::tree::Init;
use crate::sem::types::{QType, Type};

/// The value of an object of type `ty` from its initializer (INIT-2 to
/// INIT-4): none for default-initialization of a scalar (INIT-1), zero
/// for value-initialization, the converted value, or the list.
pub fn value(m: &mut Machine, ty: &QType, init: &Init) -> R<Val> {
    match init {
        Init::Default => Ok(m.uninit(ty)),
        Init::Zero => Ok(m.zero(ty)),
        Init::Expr(e) => {
            let v = eval_any(m, e)?;
            match (&ty.t, v) {
                (Type::Float(k), Val::Float(f)) => Ok(Val::Float(m.round_float(f, *k))),
                (_, v) => Ok(v),
            }
        }
        Init::Ref(e) => {
            let p = place(m, e)?;
            Ok(Val::Ptr(p))
        }
        Init::List(items) => {
            let (Type::Array(et, Some(n))) = &ty.t else { return crate::machine::unsup("a list for a type that is not an array") };
            let mut out = Vec::new();
            for k in 0..*n as usize {
                out.push(match items.get(k) {
                    Some(i) => value(m, et, i)?,
                    None => m.zero(et),
                });
            }
            Ok(Val::Agg(out))
        }
    }
}

/// INIT-1: a read of a variable with automatic storage before it has a
/// value.
pub fn read_no_value<T>(name: &str) -> R<T> {
    ub("INIT-1", format!("a read of {} before it has a value", name))
}
