//! The Syntax segment of `templates/specialization`.

use crate::declarations::declaration;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `explicit_specialization: "template" "<" ">" declaration`.
pub fn explicit_specialization(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kids = vec![p.expect("template")?, p.expect("<")?, p.close_angle()?, Kid::Node(declaration(p)?)];
    Ok(p.node("explicit_specialization", sp, kids))
}
