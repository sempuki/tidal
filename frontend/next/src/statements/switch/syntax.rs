//! The Syntax segment of `statements/switch`.

use crate::statements::if_stmt::syntax::head;
use crate::statements::statement;
use crate::syntax::names::Enter;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `switch_stmt: "switch" "(" [init_stmt] condition ")" statement`.
pub fn switch_stmt(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("switch")?];
    p.names.enter(Enter::Block);
    let r = (|| -> PResult<()> {
        kids.extend(head(p)?);
        kids.push(Kid::Node(statement(p)?));
        Ok(())
    })();
    p.names.leave();
    r?;
    Ok(p.node("switch_stmt", sp, kids))
}
