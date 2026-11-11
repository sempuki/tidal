//! The Syntax segment of `statements/jumps`.

use crate::declarations::initialization::list::syntax::braced_init;
use crate::expressions::comma::syntax::expression;
use crate::expressions::primary::syntax::identifier;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `jump_stmt: "break" ";" | "continue" ";" | "return" [expression | braced_init] ";"
/// | "goto" identifier ";"`.
pub fn jump_stmt(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("jump_stmt") {
        return Ok(n);
    }
    let sp = p.span();
    let mut kids = Vec::new();
    match p.peek() {
        "break" | "continue" => kids.push(p.bump()),
        "return" => {
            kids.push(p.bump());
            if p.at("{") {
                kids.push(Kid::Node(braced_init(p)?));
            } else if !p.at(";") {
                kids.push(Kid::Node(expression(p)?));
            }
        }
        "goto" => {
            kids.push(p.bump());
            kids.push(identifier(p)?);
        }
        _ => return Err(p.fail("a jump statement")),
    }
    kids.push(p.expect(";")?);
    Ok(p.node("jump_stmt", sp, kids))
}
