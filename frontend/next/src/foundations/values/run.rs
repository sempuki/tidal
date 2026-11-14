//! The run part of `foundations/values`: the lvalue-to-rvalue
//! conversion, which reads the value of the object a glvalue names. The
//! checks on the object sit in `foundations::objects`, and the limits of
//! constant evaluation in `foundations::constant_evaluation`.

use crate::foundations::{constant_evaluation, objects};
use crate::machine::{Machine, Ptr, Val, R};
use crate::sem::types::QType;

/// Read the value at a place (VC-1, VC-2: the value of a glvalue in a
/// context that needs a prvalue).
pub fn read(m: &mut Machine, p: &Ptr, ty: &QType) -> R<Val> {
    objects::run::check_read(m, p, ty)?;
    if p.view.is_some() {
        return crate::foundations::storage::run::read(m, p, ty);
    }
    crate::declarations::volatile::run::check_access(m, p, ty)?;
    m.record(p, false);
    if m.constant {
        constant_evaluation::run::check_read(m, p, ty)?;
    }
    crate::classes::unions::run::check_active(m, p)?;
    let v = match m.slot(p) {
        Some(v) => v.clone(),
        None => return objects::run::outside(m, p),
    };
    if v == Val::Uninit {
        return if m.constant { constant_evaluation::run::no_value(m) } else { objects::run::no_value(m, p) };
    }
    // CE-22: constant evaluation does not copy a value with a part that
    // has no value. A trivial copy of a union reads none of its parts.
    fn empty_part(m: &Machine, ty: &QType, v: &Val) -> bool {
        match (v, &ty.t) {
            (Val::Uninit, _) => true,
            (Val::Agg(_), crate::sem::types::Type::Class(c)) if m.u.classes[*c].key == crate::sem::ClassKey::Union => false,
            (Val::Agg(xs), crate::sem::types::Type::Class(c)) => xs.iter().zip(m.u.field_types(*c)).any(|(x, t)| empty_part(m, &t, x)),
            (Val::Agg(xs), crate::sem::types::Type::Array(e, _)) => xs.iter().any(|x| empty_part(m, e, x)),
            (Val::Agg(xs), _) => xs.iter().any(|x| empty_part(m, ty, x)),
            _ => false,
        }
    }
    if m.constant && matches!(v, Val::Agg(_)) && empty_part(m, ty, &v) {
        return constant_evaluation::run::no_value(m);
    }
    // An integer object read through an lvalue of another integer type,
    // after a reinterpret_cast, gives the value of the lvalue's type.
    match (&v, &ty.t) {
        (Val::Int(x), crate::sem::types::Type::Int(k)) => Ok(Val::Int(m.target.wrap(*k, *x))),
        // ENUM-6: an object of an enumeration that got another value,
        // such as through its bytes.
        (Val::Int(x), crate::sem::types::Type::Enum(e)) => {
            crate::declarations::enumerations::run::check_value(m, *e, *x)?;
            Ok(v)
        }
        (Val::Float(_) | Val::Ext(_), crate::sem::types::Type::Int(_)) | (Val::Int(_), crate::sem::types::Type::Float(_)) => crate::machine::unsup("an object read through an lvalue of another type"),
        _ => Ok(v),
    }
}
