//! The Syntax segment of `statements/range-for`.

use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::declarators::syntax::{declarator, ref_qualifier};
use crate::declarations::decl_specifiers;
use crate::declarations::initialization::list::syntax::braced_init;
use crate::expressions::comma::syntax::expression;
use crate::expressions::primary::syntax::identifier;
use crate::statements::if_stmt::syntax::init_stmt;
use crate::statements::statement;
use crate::syntax::names::{DeclCtx, Enter};
use crate::syntax::{Kid, Node, PResult, Parser};

/// `range_for_stmt: "for" "(" [init_stmt] range_decl ":" range_init ")" statement`.
pub fn range_for_stmt(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    p.names.enter(Enter::Block);
    let r = (|| -> PResult<Vec<Kid>> {
        let mut kids = vec![p.expect("for")?, p.expect("(")?];
        p.bracketed(|p| -> PResult<()> {
            let with_init = p.attempt(|p| {
                let i = init_stmt(p)?;
                let d = range_decl(p)?;
                let c = p.expect(":")?;
                Ok(vec![Kid::Node(i), Kid::Node(d), c])
            });
            match with_init {
                Ok(k) => kids.extend(k),
                Err(_) => {
                    kids.push(Kid::Node(range_decl(p)?));
                    kids.push(p.expect(":")?);
                }
            }
            kids.push(Kid::Node(range_init(p)?));
            Ok(())
        })?;
        kids.push(p.expect(")")?);
        kids.push(Kid::Node(statement(p)?));
        Ok(kids)
    })();
    p.names.leave();
    Ok(p.node("range_for_stmt", sp, r?))
}

/// `range_decl: {attribute_spec} decl_specifiers declarator
/// | {attribute_spec} decl_specifiers [ref_qualifier] "[" identifier {"," identifier} "]"`.
pub fn range_decl(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    let specs = decl_specifiers(p)?;
    if let Some(d) = p.opt(|p| {
        let d = declarator(p)?;
        if p.at(":") {
            Ok(d)
        } else {
            Err(p.fail("':'"))
        }
    }) {
        p.declared(&d, DeclCtx { specs: Some(&specs), template: false, param: false });
        kids.push(Kid::Node(specs));
        kids.push(Kid::Node(d));
        return Ok(p.node("range_decl", sp, kids));
    }
    kids.push(Kid::Node(specs));
    if let Some(r) = p.opt(ref_qualifier) {
        kids.push(Kid::Node(r));
    }
    kids.push(p.expect("[")?);
    loop {
        kids.push(identifier(p)?);
        match p.eat(",") {
            Some(k) => kids.push(k),
            None => break,
        }
    }
    kids.push(p.expect("]")?);
    Ok(p.node("range_decl", sp, kids))
}

/// `range_init: expression | braced_init`.
pub fn range_init(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let n = if p.at("{") { braced_init(p)? } else { expression(p)? };
    Ok(p.node("range_init", sp, vec![Kid::Node(n)]))
}
