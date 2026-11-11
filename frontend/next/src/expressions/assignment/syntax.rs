//! The Syntax segment of `expressions/assignment`.

use crate::declarations::initialization::list::syntax::braced_init;
use crate::exceptions::throw::syntax::throw_expr;
use crate::expressions::conditional::syntax::conditional_rest;
use crate::expressions::logical::syntax::logical_or_expr;
use crate::expressions::operator_here;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `assignment_expr: conditional_expr
/// | logical_or_expr assign_op (assignment_expr | braced_init) | throw_expr`.
pub fn assignment_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("assignment_expr") {
        return Ok(n);
    }
    if p.at("throw") {
        return throw_expr(p);
    }
    let sp = p.span();
    let lhs = logical_or_expr(p)?;
    if let Ok(op) = assign_op(p) {
        let rhs = if p.at("{") { braced_init(p)? } else { assignment_expr(p)? };
        return Ok(p.node("assignment_expr", sp, vec![Kid::Node(lhs), Kid::Node(op), Kid::Node(rhs)]));
    }
    conditional_rest(p, lhs)
}

/// `assign_op: "=" | "*=" | "/=" | "%=" | "+=" | "-=" | "<<=" | ">>=" | "&=" | "^=" | "|="`.
pub fn assign_op(p: &mut Parser) -> PResult<Node> {
    const OPS: &[&str] = &["=", "*=", "/=", "%=", "+=", "-=", "<<=", ">>=", "&=", "^=", "|="];
    let sp = p.span();
    if OPS.iter().any(|o| operator_here(p, o)) {
        let k = p.bump();
        return Ok(p.node("assign_op", sp, vec![k]));
    }
    Err(p.fail("an assignment operator"))
}
