//! The run part of `declarations/attributes`.

use crate::machine::{ub, Machine, Val, R};
use crate::sem::tree::{Conv, Expr, EK};

/// Whether evaluating the expression of an assumption could change the
/// program: a call, a write, or a volatile read.
pub fn has_side_effect(e: &Expr) -> bool {
    e.any(&|x| matches!(x.k, EK::Call(..) | EK::Assign(..) | EK::Compound(..) | EK::IncDec(..)) || (matches!(x.k, EK::Conv(Conv::LToR, _)) && x.ty.is_volatile()))
}

/// ATTR-10: an assumption whose expression is false. In constant
/// evaluation the undefined behavior makes the evaluation not constant.
pub fn assume(_m: &mut Machine, v: Val) -> R<Val> {
    if !v.truthy() {
        return ub("ATTR-10", "the expression of an assumption is false");
    }
    Ok(Val::Void)
}
