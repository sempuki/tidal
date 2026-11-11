//! The Syntax segment of `declarations/structured-bindings`.

use crate::declarations::declarators::syntax::cv;
use crate::expressions::assignment::syntax::assignment_expr;
use crate::expressions::primary::syntax::identifier;
use crate::syntax::names::DeclCtx;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `sb_decl: ["static" | "thread_local"] {cv} "auto" ["&" | "&&"]
/// "[" identifier {"," identifier} "]" sb_init ";"`.
pub fn sb_decl(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if p.at("static") || p.at("thread_local") {
        kids.push(p.bump());
    }
    while let Some(c) = p.opt(cv) {
        kids.push(Kid::Node(c));
    }
    kids.push(p.expect("auto")?);
    if p.at("&") || p.at("&&") {
        kids.push(p.bump());
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
    let head = p.node("sb_decl", sp, kids.clone());
    p.declared(&head, DeclCtx::default());
    kids.push(Kid::Node(sb_init(p)?));
    kids.push(p.expect(";")?);
    Ok(p.node("sb_decl", sp, kids))
}

/// `sb_init: "=" assignment_expr | "(" assignment_expr ")" | "{" assignment_expr "}"`.
pub fn sb_init(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kids = match p.peek() {
        "=" => vec![p.bump(), Kid::Node(assignment_expr(p)?)],
        "(" => vec![p.bump(), Kid::Node(p.bracketed(assignment_expr)?), p.expect(")")?],
        "{" => vec![p.bump(), Kid::Node(p.bracketed(assignment_expr)?), p.expect("}")?],
        _ => return Err(p.fail("an initializer")),
    };
    Ok(p.node("sb_init", sp, kids))
}
