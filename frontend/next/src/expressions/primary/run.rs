//! The run part of `expressions/primary`.

use crate::machine::{ub, R};

/// PRIM-8: a write to a character of a string literal.
pub fn write_string() -> R<()> {
    ub("PRIM-8", "a write to a character of a string literal")
}
