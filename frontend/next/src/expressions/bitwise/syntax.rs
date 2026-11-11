//! The Syntax segment of `expressions/bitwise`.

use crate::expressions::binary;
use crate::syntax::{Node, PResult, Parser};

/// `and_expr: equality_expr {"&" equality_expr}`.
pub fn and_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("and_expr") {
        return Ok(n);
    }
    binary(p, "and_expr", &["&"], crate::expressions::comparison::syntax::equality_expr)
}

/// `xor_expr: and_expr {"^" and_expr}`.
pub fn xor_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("xor_expr") {
        return Ok(n);
    }
    binary(p, "xor_expr", &["^"], and_expr)
}

/// `or_expr: xor_expr {"|" xor_expr}`.
pub fn or_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("or_expr") {
        return Ok(n);
    }
    binary(p, "or_expr", &["|"], xor_expr)
}
