//! The Syntax segment of `expressions/new-delete`.

use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::declarators::syntax::{cv, type_id};
use crate::declarations::initialization::list::syntax::braced_init;
use crate::declarations::type_specifier;
use crate::expressions::casts::syntax::cast_expr;
use crate::expressions::comma::syntax::expression;
use crate::expressions::postfix::syntax::expression_list;
use crate::foundations::constant_evaluation::syntax::constant_expr;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `new_expr: ["::"] "new" [placement] (new_type | "(" type_id ")") [new_init]`.
pub fn new_expr(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if let Some(k) = p.eat("::") {
        kids.push(k);
    }
    kids.push(p.expect("new")?);
    let typed = |p: &mut Parser| -> PResult<Vec<Kid>> {
        if p.at("(") {
            let open = p.bump();
            let t = p.with(true, true, type_id)?;
            Ok(vec![open, Kid::Node(t), p.expect(")")?])
        } else {
            Ok(vec![Kid::Node(p.with(true, true, new_type)?)])
        }
    };
    if p.at("(") {
        if let Ok(k) = p.attempt(|p| {
            let pl = placement(p)?;
            let mut k = vec![Kid::Node(pl)];
            k.extend(typed(p)?);
            Ok(k)
        }) {
            kids.extend(k);
        } else {
            kids.extend(typed(p)?);
        }
    } else {
        kids.extend(typed(p)?);
    }
    if p.at("(") || p.at("{") {
        kids.push(Kid::Node(new_init(p)?));
    }
    Ok(p.node("new_expr", sp, kids))
}

/// `placement: "(" expression_list ")"`.
pub fn placement(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let open = p.expect("(")?;
    let l = p.bracketed(expression_list)?;
    let close = p.expect(")")?;
    Ok(p.node("placement", sp, vec![open, Kid::Node(l), close]))
}

/// `new_type: type_specifier {type_specifier} {attribute_spec}
/// {"*" {cv}} [new_bounds]`.
pub fn new_type(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    let mut st = crate::declarations::Specs::default();
    while let Some(t) = p.opt(|p| type_specifier(p, &mut st)) {
        kids.push(Kid::Node(t));
    }
    if kids.is_empty() {
        return Err(p.fail("a type"));
    }
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    while p.at("*") {
        kids.push(p.bump());
        while let Some(c) = p.opt(cv) {
            kids.push(Kid::Node(c));
        }
    }
    if p.at("[") && !p.at_attr() {
        kids.push(Kid::Node(new_bounds(p)?));
    }
    Ok(p.node("new_type", sp, kids))
}

/// `new_bounds: "[" [expression] "]" {attribute_spec}
/// {"[" constant_expr "]" {attribute_spec}}`.
pub fn new_bounds(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("[")?];
    if !p.at("]") {
        kids.push(Kid::Node(p.bracketed(expression)?));
    }
    kids.push(p.expect("]")?);
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    while p.at("[") && !p.at_attr() {
        kids.push(p.bump());
        kids.push(Kid::Node(p.bracketed(constant_expr)?));
        kids.push(p.expect("]")?);
        while p.at_attr() {
            kids.push(Kid::Node(attribute_spec(p)?));
        }
    }
    Ok(p.node("new_bounds", sp, kids))
}

/// `new_init: "(" [expression_list] ")" | braced_init`.
pub fn new_init(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at("{") {
        let b = braced_init(p)?;
        return Ok(p.node("new_init", sp, vec![Kid::Node(b)]));
    }
    let mut kids = vec![p.expect("(")?];
    if !p.at(")") {
        kids.push(Kid::Node(p.bracketed(expression_list)?));
    }
    kids.push(p.expect(")")?);
    Ok(p.node("new_init", sp, kids))
}

/// `delete_expr: ["::"] "delete" ["[" "]"] cast_expr`.
pub fn delete_expr(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if let Some(k) = p.eat("::") {
        kids.push(k);
    }
    kids.push(p.expect("delete")?);
    if p.at("[") && p.ahead(1) == "]" {
        kids.push(p.bump());
        kids.push(p.bump());
    }
    kids.push(Kid::Node(cast_expr(p)?));
    Ok(p.node("delete_expr", sp, kids))
}
