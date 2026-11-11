//! The Syntax segment of `expressions/member-pointers`.

use crate::expressions::binary;
use crate::syntax::{Node, PResult, Parser};

/// `pm_expr: cast_expr {(".*" | "->*") cast_expr}`.
pub fn pm_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("pm_expr") {
        return Ok(n);
    }
    binary(p, "pm_expr", &[".*", "->*"], crate::expressions::casts::syntax::cast_expr)
}
