//! The Syntax segment of `declarations/linkage-specifications`.

use crate::declarations::declaration;
use crate::expressions::primary::syntax::string_literal;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `linkage_spec: "extern" string_literal "{" {declaration} "}"
/// | "extern" string_literal declaration`.
pub fn linkage_spec(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("extern")?, string_literal(p)?];
    if p.at("{") {
        kids.push(p.bump());
        while !p.at("}") {
            if p.at_eof() {
                return Err(p.fail("'}'"));
            }
            kids.push(Kid::Node(declaration(p)?));
        }
        kids.push(p.bump());
    } else {
        kids.push(Kid::Node(declaration(p)?));
    }
    Ok(p.node("linkage_spec", sp, kids))
}
