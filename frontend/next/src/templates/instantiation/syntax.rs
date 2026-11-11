//! The Syntax segment of `templates/instantiation`.

use crate::declarations::declaration;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `explicit_instantiation: ["extern"] "template" declaration`.
pub fn explicit_instantiation(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if let Some(k) = p.eat("extern") {
        kids.push(k);
    }
    kids.push(p.expect("template")?);
    kids.push(Kid::Node(declaration(p)?));
    Ok(p.node("explicit_instantiation", sp, kids))
}
