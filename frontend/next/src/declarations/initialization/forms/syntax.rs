//! The Syntax segment of `declarations/initialization/forms`.

use crate::declarations::initialization::list::syntax::braced_init;
use crate::expressions::assignment::syntax::assignment_expr;
use crate::expressions::postfix::syntax::expression_list;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `init: "=" assignment_expr | "(" expression_list ")" | braced_init | "=" braced_init`.
pub fn init(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kids = if p.at("{") {
        vec![Kid::Node(braced_init(p)?)]
    } else if p.at("(") {
        let open = p.bump();
        let l = p.bracketed(expression_list)?;
        vec![open, Kid::Node(l), p.expect(")")?]
    } else {
        let eq = p.expect("=")?;
        let v = if p.at("{") { braced_init(p)? } else { assignment_expr(p)? };
        vec![eq, Kid::Node(v)]
    };
    Ok(p.node("init", sp, kids))
}
