//! The Syntax segment of `exceptions/handlers`.

use crate::classes::constructors::syntax::ctor_init;
use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::declarators::syntax::{abstract_declarator, declarator};
use crate::declarations::type_specifiers;
use crate::statements::blocks::syntax::compound_stmt;
use crate::syntax::names::{DeclCtx, Enter};
use crate::syntax::{Kid, Node, PResult, Parser};

fn handlers(p: &mut Parser, kids: &mut Vec<Kid>) -> PResult<()> {
    kids.push(Kid::Node(handler(p)?));
    while p.at("catch") {
        kids.push(Kid::Node(handler(p)?));
    }
    Ok(())
}

/// `try_block: "try" compound_stmt handler {handler}`.
pub fn try_block(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("try")?, Kid::Node(compound_stmt(p)?)];
    handlers(p, &mut kids)?;
    Ok(p.node("try_block", sp, kids))
}

/// `handler: "catch" "(" exception_decl ")" compound_stmt`.
pub fn handler(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    p.names.enter(Enter::Block);
    let r = (|| -> PResult<Vec<Kid>> {
        let mut kids = vec![p.expect("catch")?, p.expect("(")?];
        kids.push(Kid::Node(p.bracketed(exception_decl)?));
        kids.push(p.expect(")")?);
        kids.push(Kid::Node(compound_stmt(p)?));
        Ok(kids)
    })();
    p.names.leave();
    Ok(p.node("handler", sp, r?))
}

/// `exception_decl: {attribute_spec} type_specifiers [declarator | abstract_declarator] | "..."`.
pub fn exception_decl(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at("...") {
        let k = p.bump();
        return Ok(p.node("exception_decl", sp, vec![k]));
    }
    let mut kids = Vec::new();
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    let specs = type_specifiers(p)?;
    kids.push(Kid::Node(specs.clone()));
    if let Some(d) = p.opt(|p| {
        let d = declarator(p)?;
        if p.at(")") || p.at_eof() {
            Ok(d)
        } else {
            Err(p.fail("')'"))
        }
    }) {
        p.declared(&d, DeclCtx { specs: Some(&specs), template: false, param: false });
        kids.push(Kid::Node(d));
    } else if let Some(a) = p.opt(abstract_declarator) {
        kids.push(Kid::Node(a));
    }
    Ok(p.node("exception_decl", sp, kids))
}

/// `function_try_block: "try" [ctor_init] compound_stmt handler {handler}`.
pub fn function_try_block(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("try")?];
    if p.at(":") {
        kids.push(Kid::Node(ctor_init(p)?));
    }
    kids.push(Kid::Node(compound_stmt(p)?));
    handlers(p, &mut kids)?;
    Ok(p.node("function_try_block", sp, kids))
}
