//! The Syntax segment of `expressions/multiplicative`.

use crate::expressions::binary;
use crate::syntax::{Node, PResult, Parser};

/// `mul_expr: pm_expr {("*" | "/" | "%") pm_expr}`.
pub fn mul_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("mul_expr") {
        return Ok(n);
    }
    binary(p, "mul_expr", &["*", "/", "%"], crate::expressions::member_pointers::syntax::pm_expr)
}
