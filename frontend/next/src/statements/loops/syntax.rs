//! The Syntax segment of `statements/loops`.

use crate::expressions::comma::syntax::expression;
use crate::statements::if_stmt::syntax::{condition, init_stmt};
use crate::statements::statement;
use crate::syntax::names::Enter;
use crate::syntax::{Kid, Node, PResult, Parser};

/// Run `f` in a block scope of its own.
fn scoped(p: &mut Parser, f: impl FnOnce(&mut Parser) -> PResult<Vec<Kid>>) -> PResult<Vec<Kid>> {
    p.names.enter(Enter::Block);
    let r = f(p);
    p.names.leave();
    r
}

/// `while_stmt: "while" "(" condition ")" statement`.
pub fn while_stmt(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kids = scoped(p, |p| {
        let mut kids = vec![p.expect("while")?, p.expect("(")?];
        kids.push(Kid::Node(p.bracketed(condition)?));
        kids.push(p.expect(")")?);
        kids.push(Kid::Node(statement(p)?));
        Ok(kids)
    })?;
    Ok(p.node("while_stmt", sp, kids))
}

/// `do_stmt: "do" statement "while" "(" expression ")" ";"`.
pub fn do_stmt(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("do")?];
    kids.extend(scoped(p, |p| Ok(vec![Kid::Node(statement(p)?)]))?);
    kids.push(p.expect("while")?);
    kids.push(p.expect("(")?);
    kids.push(Kid::Node(p.bracketed(expression)?));
    kids.push(p.expect(")")?);
    kids.push(p.expect(";")?);
    Ok(p.node("do_stmt", sp, kids))
}

/// `for_stmt: "for" "(" init_stmt [condition] ";" [expression] ")" statement`.
pub fn for_stmt(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kids = scoped(p, |p| {
        let mut kids = vec![p.expect("for")?, p.expect("(")?];
        p.bracketed(|p| -> PResult<()> {
            kids.push(Kid::Node(init_stmt(p)?));
            if !p.at(";") {
                kids.push(Kid::Node(condition(p)?));
            }
            kids.push(p.expect(";")?);
            if !p.at(")") {
                kids.push(Kid::Node(expression(p)?));
            }
            Ok(())
        })?;
        kids.push(p.expect(")")?);
        kids.push(Kid::Node(statement(p)?));
        Ok(kids)
    })?;
    Ok(p.node("for_stmt", sp, kids))
}
