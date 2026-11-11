//! The Syntax segment of `expressions/shift`.

use crate::expressions::binary;
use crate::syntax::{Node, PResult, Parser};

/// `shift_expr: add_expr {("<<" | ">>") add_expr}`. Inside a template argument list, `>>` closes two lists.
pub fn shift_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("shift_expr") {
        return Ok(n);
    }
    binary(p, "shift_expr", &["<<", ">>"], crate::expressions::additive::syntax::add_expr)
}
