//! The Syntax segment of `declarations/exception-specifications`.

use crate::foundations::constant_evaluation::syntax::constant_expr;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `noexcept_specifier: "noexcept" ["(" constant_expr ")"]`.
pub fn noexcept_specifier(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("noexcept")?];
    if p.at("(") {
        kids.push(p.bump());
        kids.push(Kid::Node(p.bracketed(constant_expr)?));
        kids.push(p.expect(")")?);
    }
    Ok(p.node("noexcept_specifier", sp, kids))
}
