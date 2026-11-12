//! The run part of `declarations/attributes`.

use crate::machine::{ub, Machine, Val, R};

/// ATTR-10: an assumption whose expression is false.
pub fn assume(m: &mut Machine, v: Val) -> R<Val> {
    if m.constant {
        // Constant evaluation of a false assumption is not constant.
    }
    if !v.truthy() {
        return ub("ATTR-10", "the expression of an assumption is false");
    }
    Ok(Val::Void)
}
