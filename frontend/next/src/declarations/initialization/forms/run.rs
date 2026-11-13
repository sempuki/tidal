//! The run part of `declarations/initialization/forms`: the value a new
//! object gets from its initializer.

use crate::machine::eval::{eval_any, place};
use crate::machine::{ub, Machine, Val, R};
use crate::sem::tree::Init;
use crate::sem::types::{QType, Type};

/// INIT-1: a copy of an `unsigned char` that has no value, into an
/// `unsigned char`, through casts to `unsigned char`, either branch of
/// `?:`, and the right operand of a comma, is not undefined, and the copy
/// has no value. Constant evaluation rejects the copy (CE-22). None when
/// `e` has a value or is not such a copy.
pub fn indeterminate_copy(m: &mut Machine, ty: &QType, e: &crate::sem::tree::Expr) -> Option<R<Val>> {
    use crate::sem::tree::{Conv, EK};
    let uchar = |t: &QType| t.unqual().t == Type::Int(crate::sem::types::IntK::UChar);
    if !uchar(ty) || !uchar(&e.ty) {
        return None;
    }
    match &e.k {
        EK::Conv(Conv::LToR, x) if uchar(&x.ty) => {
            let p = match place(m, x) {
                Ok(p) => p,
                Err(s) => return Some(Err(s)),
            };
            if !matches!(m.slot(&p), Some(Val::Uninit)) || m.objs.get(p.obj).is_some_and(|o| !o.alive) {
                return None;
            }
            if m.constant {
                return Some(crate::foundations::constant_evaluation::run::no_value(m));
            }
            Some(Ok(Val::Uninit))
        }
        EK::Conv(Conv::Arith | Conv::NoOp, x) if uchar(&x.ty) => indeterminate_copy(m, ty, x),
        EK::Cond(c, a, b) => {
            let pick = match crate::machine::eval::eval(m, c) {
                Ok(v) => v.truthy(),
                Err(s) => return Some(Err(s)),
            };
            let r = indeterminate_copy(m, ty, if pick { a } else { b });
            r.or_else(|| Some(crate::machine::eval::eval(m, if pick { a } else { b })))
        }
        EK::Comma(a, b) => {
            if let Err(s) = crate::machine::eval::eval_discard(m, a) {
                return Some(Err(s));
            }
            indeterminate_copy(m, ty, b).or_else(|| Some(crate::machine::eval::eval(m, b)))
        }
        _ => None,
    }
}

/// The value of an object of type `ty` from its initializer (INIT-2 to
/// INIT-4): none for default-initialization of a scalar (INIT-1), zero
/// for value-initialization, the converted value, or the list.
pub fn value(m: &mut Machine, ty: &QType, init: &Init) -> R<Val> {
    match init {
        Init::Default => Ok(m.uninit(ty)),
        Init::Zero => Ok(m.zero(ty)),
        Init::Expr(e) => {
            if let Some(v) = indeterminate_copy(m, ty, e) {
                return v;
            }
            let v = eval_any(m, e)?;
            match (&ty.t, v) {
                (Type::Float(k), v @ (Val::Float(_) | Val::Ext(_))) => Ok(crate::foundations::conversions::run::to_float(v, *k)),
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
