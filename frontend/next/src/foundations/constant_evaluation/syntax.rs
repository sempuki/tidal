//! The Syntax segment of `foundations/constant-evaluation`.

use crate::expressions::conditional::syntax::conditional_expr;
use crate::syntax::{Node, PResult, Parser};

/// `constant_expr: conditional_expr`.
pub fn constant_expr(p: &mut Parser) -> PResult<Node> {
    conditional_expr(p)
}
