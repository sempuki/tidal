//! The Syntax segment of `declarations/alignment`.

use crate::declarations::declarators::syntax::type_id;
use crate::declarations::type_start;
use crate::foundations::constant_evaluation::syntax::constant_expr;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `alignment_specifier: "alignas" "(" constant_expr ["..."] ")"
/// | "alignas" "(" type_id ["..."] ")"`.
pub fn alignment_specifier(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("alignas")?, p.expect("(")?];
    let arg = if type_start(p) {
        match p.attempt(|p| {
            let t = p.bracketed(type_id)?;
            if p.at(")") || p.at("...") {
                Ok(t)
            } else {
                Err(p.fail("')'"))
            }
        }) {
            Ok(t) => t,
            Err(_) => p.bracketed(constant_expr)?,
        }
    } else {
        p.bracketed(constant_expr)?
    };
    kids.push(Kid::Node(arg));
    if let Some(k) = p.eat("...") {
        kids.push(k);
    }
    kids.push(p.expect(")")?);
    Ok(p.node("alignment_specifier", sp, kids))
}
