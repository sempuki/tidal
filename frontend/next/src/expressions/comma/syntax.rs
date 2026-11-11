//! The Syntax segment of `expressions/comma`.

use crate::expressions::assignment::syntax::assignment_expr;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `expression: assignment_expr {"," assignment_expr}`.
pub fn expression(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("expression") {
        return Ok(n);
    }
    let sp = p.span();
    let first = assignment_expr(p)?;
    if !p.at(",") {
        return Ok(first);
    }
    let mut kids = vec![Kid::Node(first)];
    while p.at(",") {
        kids.push(p.bump());
        kids.push(Kid::Node(assignment_expr(p)?));
    }
    Ok(p.node("expression", sp, kids))
}
