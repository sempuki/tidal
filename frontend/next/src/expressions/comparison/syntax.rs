//! The Syntax segment of `expressions/comparison`.

use crate::expressions::binary;
use crate::syntax::{Node, PResult, Parser};

/// `three_way_expr: shift_expr {"<=>" shift_expr}`.
pub fn three_way_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("three_way_expr") {
        return Ok(n);
    }
    binary(p, "three_way_expr", &["<=>"], crate::expressions::shift::syntax::shift_expr)
}

/// `relational_expr: three_way_expr {("<" | ">" | "<=" | ">=") three_way_expr}`.
/// Inside a template argument list, `>` closes the list.
pub fn relational_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("relational_expr") {
        return Ok(n);
    }
    binary(p, "relational_expr", &["<", ">", "<=", ">="], three_way_expr)
}

/// `equality_expr: relational_expr {("==" | "!=") relational_expr}`.
pub fn equality_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("equality_expr") {
        return Ok(n);
    }
    binary(p, "equality_expr", &["==", "!="], relational_expr)
}
