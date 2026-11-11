//! The Syntax segment of `exceptions/throw`.

use crate::expressions::assignment::syntax::assignment_expr;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `throw_expr: "throw" [assignment_expr]`.
pub fn throw_expr(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("throw")?];
    if !p.at_any(&[";", ")", "]", "}", ",", ":"]) && !p.at_eof() {
        kids.push(Kid::Node(assignment_expr(p)?));
    }
    Ok(p.node("throw_expr", sp, kids))
}
