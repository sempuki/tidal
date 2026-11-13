//! The Syntax segment of `statements/blocks`.

use crate::declarations::attributes::syntax::attribute_spec;
use crate::expressions::comma::syntax::expression;
use crate::expressions::primary::syntax::identifier;
use crate::foundations::constant_evaluation::syntax::constant_expr;
use crate::statements::statement;
use crate::syntax::names::Enter;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `expression_stmt: expression ";"`.
pub fn expression_stmt(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let e = expression(p)?;
    let semi = p.expect(";")?;
    Ok(p.node("expression_stmt", sp, vec![Kid::Node(e), semi]))
}

/// `null_stmt: ";"`.
pub fn null_stmt(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let k = p.expect(";")?;
    Ok(p.node("null_stmt", sp, vec![k]))
}

/// `compound_stmt: "{" {statement} {label} "}"`.
pub fn compound_stmt(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("compound_stmt") {
        return Ok(n);
    }
    let sp = p.span();
    let open = p.expect("{")?;
    p.names.enter(Enter::Block);
    let r = p.with(true, false, |p| -> PResult<Vec<Kid>> {
        let mut kids = vec![open];
        let mut labels_only = false;
        while !p.at("}") {
            if p.at_eof() {
                return Err(p.fail("'}'"));
            }
            if p.recover {
                if let Some(s) = p.item(|p| p.attempt(statement).or_else(|_| label(p)))? {
                    kids.push(Kid::Node(s));
                }
                continue;
            }
            if !labels_only {
                if let Ok(s) = p.attempt(statement) {
                    kids.push(Kid::Node(s));
                    continue;
                }
            }
            labels_only = true;
            kids.push(Kid::Node(label(p)?));
        }
        kids.push(p.bump());
        Ok(kids)
    });
    p.names.leave();
    Ok(p.node("compound_stmt", sp, r?))
}

/// `labeled_stmt: label statement`.
pub fn labeled_stmt(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let l = label(p)?;
    let s = statement(p)?;
    Ok(p.node("labeled_stmt", sp, vec![Kid::Node(l), Kid::Node(s)]))
}

/// `label: {attribute_spec} identifier ":" | {attribute_spec} "case" constant_expr ":"
/// | {attribute_spec} "default" ":"`.
pub fn label(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    if p.at("case") {
        kids.push(p.bump());
        kids.push(Kid::Node(constant_expr(p)?));
        if p.at(",") {
            return Err(p.fail_rule("COMMA-8", "a case label's expression has no comma operator outside parentheses"));
        }
    } else if p.at("default") {
        kids.push(p.bump());
    } else {
        kids.push(identifier(p)?);
    }
    kids.push(p.expect(":")?);
    Ok(p.node("label", sp, kids))
}
