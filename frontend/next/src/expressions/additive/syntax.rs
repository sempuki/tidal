//! The Syntax segment of `expressions/additive`.

use crate::expressions::binary;
use crate::syntax::{Node, PResult, Parser};

/// `add_expr: mul_expr {("+" | "-") mul_expr}`.
pub fn add_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("add_expr") {
        return Ok(n);
    }
    binary(p, "add_expr", &["+", "-"], crate::expressions::multiplicative::syntax::mul_expr)
}
