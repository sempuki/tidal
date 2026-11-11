//! The Syntax segment of `classes/virtual`.

use crate::declarations::declarators::syntax::type_id;
use crate::expressions::comma::syntax::expression;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `virt_specifiers: "override" | "final" | "override" "final" | "final" "override"`.
/// The two words are identifiers with this meaning only here (KW-3).
pub fn virt_specifiers(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if !(p.at("override") || p.at("final")) {
        return Err(p.fail("'override' or 'final'"));
    }
    let first = p.peek();
    let mut kids = vec![p.bump()];
    let other = if first == "override" { "final" } else { "override" };
    if p.at(other) {
        kids.push(p.bump());
    }
    Ok(p.node("virt_specifiers", sp, kids))
}

/// `pure_specifier: "=" "0"`.
pub fn pure_specifier(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let eq = p.expect("=")?;
    if p.tok().text != "0" {
        return Err(p.fail("'0'"));
    }
    let z = p.bump();
    Ok(p.node("pure_specifier", sp, vec![eq, z]))
}

/// `dynamic_cast: "dynamic_cast" "<" type_id ">" "(" expression ")"`.
pub fn dynamic_cast(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kw = p.expect("dynamic_cast")?;
    let lt = p.expect("<")?;
    let t = p.with(false, true, type_id)?;
    let gt = p.close_angle()?;
    let open = p.expect("(")?;
    let e = p.bracketed(expression)?;
    let close = p.expect(")")?;
    Ok(p.node("dynamic_cast", sp, vec![kw, lt, Kid::Node(t), gt, open, Kid::Node(e), close]))
}
