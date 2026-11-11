//! The Syntax segment of `expressions/lambdas`.

use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::declarators::syntax::{parameters, type_id};
use crate::declarations::exception_specifications::syntax::noexcept_specifier;
use crate::declarations::initialization::forms::syntax::init;
use crate::expressions::primary::syntax::identifier;
use crate::statements::blocks::syntax::compound_stmt;
use crate::syntax::names::{DeclCtx, Enter};
use crate::syntax::{Kid, Node, PResult, Parser};
use crate::templates::concepts::syntax::requires_clause;
use crate::templates::declarations::syntax::template_params;

/// `lambda_expr: "[" [captures] "]" [lambda_template] ["(" parameters ")"]
/// {lambda_specifier} [noexcept_specifier] ["->" type_id] [requires_clause]
/// compound_stmt`.
pub fn lambda_expr(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("[")?];
    p.names.enter(Enter::Prototype);
    let r = p.bracketed(|p| -> PResult<()> {
        if !p.at("]") {
            kids.push(Kid::Node(captures(p)?));
        }
        kids.push(p.expect("]")?);
        if p.at("<") {
            kids.push(Kid::Node(lambda_template(p)?));
        }
        if p.at("(") {
            kids.push(p.bump());
            kids.push(Kid::Node(p.with(true, true, parameters)?));
            kids.push(p.expect(")")?);
        }
        while let Ok(s) = lambda_specifier(p) {
            kids.push(Kid::Node(s));
        }
        if p.at("noexcept") {
            kids.push(Kid::Node(noexcept_specifier(p)?));
        }
        if p.at("->") {
            kids.push(p.bump());
            kids.push(Kid::Node(p.with(true, true, type_id)?));
        }
        if p.at("requires") {
            kids.push(Kid::Node(requires_clause(p)?));
        }
        let t = p.take_template();
        kids.push(Kid::Node(compound_stmt(p)?));
        p.template = t;
        Ok(())
    });
    p.names.leave();
    r?;
    Ok(p.node("lambda_expr", sp, kids))
}

/// `lambda_template: "<" template_params ">" [requires_clause] {attribute_spec}`.
pub fn lambda_template(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let lt = p.expect("<")?;
    let ps = p.with(false, true, template_params)?;
    let mut kids = vec![lt, Kid::Node(ps), p.close_angle()?];
    if p.at("requires") {
        kids.push(Kid::Node(requires_clause(p)?));
    }
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    Ok(p.node("lambda_template", sp, kids))
}

/// `captures: capture_default {"," capture} | capture {"," capture}`.
pub fn captures(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let first = match p.opt(capture_default) {
        Some(d) => d,
        None => capture(p)?,
    };
    let mut kids = vec![Kid::Node(first)];
    while p.at(",") {
        kids.push(p.bump());
        kids.push(Kid::Node(capture(p)?));
    }
    Ok(p.node("captures", sp, kids))
}

/// `capture_default: "=" | "&"`.
pub fn capture_default(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at("=") || (p.at("&") && matches!(p.ahead(1), "," | "]")) {
        let k = p.bump();
        return Ok(p.node("capture_default", sp, vec![k]));
    }
    Err(p.fail("'=' or '&'"))
}

/// `capture: ["&"] identifier ["..."] | ["&"] ["..."] identifier init
/// | "this" | "*" "this"`.
pub fn capture(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at("this") {
        let k = p.bump();
        return Ok(p.node("capture", sp, vec![k]));
    }
    if p.at("*") && p.ahead(1) == "this" {
        let a = p.bump();
        let b = p.bump();
        return Ok(p.node("capture", sp, vec![a, b]));
    }
    let mut kids = Vec::new();
    if let Some(k) = p.eat("&") {
        kids.push(k);
    }
    let dots_first = p.at("...");
    if dots_first {
        kids.push(p.bump());
    }
    kids.push(identifier(p)?);
    if !dots_first && p.at("...") {
        kids.push(p.bump());
        return Ok(p.node("capture", sp, kids));
    }
    if p.at_any(&["=", "(", "{"]) {
        kids.push(Kid::Node(init(p)?));
        let n = p.node("capture", sp, kids);
        p.declared(&n, DeclCtx::default());
        return Ok(n);
    }
    if dots_first {
        return Err(p.fail("an initializer"));
    }
    Ok(p.node("capture", sp, kids))
}

/// `lambda_specifier: "mutable" | "static" | "constexpr" | "consteval"`.
pub fn lambda_specifier(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at_any(&["mutable", "static", "constexpr", "consteval"]) {
        let k = p.bump();
        return Ok(p.node("lambda_specifier", sp, vec![k]));
    }
    Err(p.fail("a lambda specifier"))
}
