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
    if m.constant {
        constant_evaluation::run::check_read(m, p, ty)?;
    }
    let v = match m.slot(p) {
        Some(v) => v.clone(),
        None => return objects::run::outside(m, p),
    };
    if v == Val::Uninit {
        return if m.constant { constant_evaluation::run::no_value(m) } else { objects::run::no_value(m, p) };
    }
    Ok(v)
}
