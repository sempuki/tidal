//! The Syntax segment of `expressions/conditional`.

use crate::expressions::assignment::syntax::assignment_expr;
use crate::expressions::comma::syntax::expression;
use crate::expressions::logical::syntax::logical_or_expr;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `conditional_expr: logical_or_expr ["?" expression ":" assignment_expr]`.
pub fn conditional_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("conditional_expr") {
        return Ok(n);
    }
    let l = logical_or_expr(p)?;
    conditional_rest(p, l)
}

/// The rest of a `conditional_expr` after its condition.
pub fn conditional_rest(p: &mut Parser, cond: Node) -> PResult<Node> {
    if !p.at("?") {
        return Ok(cond);
    }
    let sp = cond.span;
    let q = p.bump();
    let mid = p.bracketed(expression)?;
    let colon = p.expect(":")?;
    let last = assignment_expr(p)?;
    Ok(p.node("conditional_expr", sp, vec![Kid::Node(cond), q, Kid::Node(mid), colon, Kid::Node(last)]))
}
