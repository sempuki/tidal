//! The Syntax segment of `declarations/exception-specifications`.

use crate::foundations::constant_evaluation::syntax::constant_expr;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `exception_spec: noexcept_specifier | "throw" "(" ")"`. EXS-5 gives
/// the `throw()` form (Q-0077).
pub fn exception_spec(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at("throw") {
        let kids = vec![p.bump(), p.expect("(")?, p.expect(")")?];
        return Ok(p.node("exception_spec", sp, kids));
    }
    let n = noexcept_specifier(p)?;
    Ok(p.node("exception_spec", sp, vec![Kid::Node(n)]))
}

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
