//! The Syntax segment of `expressions/logical`.

use crate::expressions::binary;
use crate::syntax::{Node, PResult, Parser};

/// `logical_and_expr: or_expr {"&&" or_expr}`.
pub fn logical_and_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("logical_and_expr") {
        return Ok(n);
    }
    binary(p, "logical_and_expr", &["&&"], crate::expressions::bitwise::syntax::or_expr)
}

/// `logical_or_expr: logical_and_expr {"||" logical_and_expr}`.
pub fn logical_or_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("logical_or_expr") {
        return Ok(n);
    }
    binary(p, "logical_or_expr", &["||"], logical_and_expr)
}
