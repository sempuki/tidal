//! The run part of `expressions/decltype`: the discarded-value
//! expression.

use crate::sem::tree::{Expr, EK};

/// UNEV-2: a discarded glvalue reads the volatile object that it refers
/// to when it has one of these forms: a name, a subscript, a member
/// access, an indirection, a pointer-to-member access, one of these in
/// parentheses, a conditional whose second and third operands both have
/// such a form, or a comma whose right operand has such a form.
pub fn reads_volatile(e: &Expr) -> bool {
    if !e.ty.is_volatile() {
        return false;
    }
    fn form(e: &Expr) -> bool {
        match &e.k {
            EK::Var(_) | EK::Index(..) | EK::Deref(_) => true,
            EK::Cond(_, a, b) => form(a) && form(b),
            EK::Comma(_, b) => form(b),
            _ => false,
        }
    }
    form(e)
}
