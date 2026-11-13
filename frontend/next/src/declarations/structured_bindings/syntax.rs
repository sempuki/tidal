//! The Syntax segment of `declarations/structured-bindings`.

use crate::declarations::declarators::syntax::cv;
use crate::expressions::assignment::syntax::assignment_expr;
use crate::expressions::primary::syntax::identifier;
use crate::syntax::names::DeclCtx;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `sb_decl: {attribute_spec} ["static" | "thread_local"] {cv} "auto" ["&" | "&&"]
/// "[" identifier {"," identifier} "]" sb_init ";"`.
pub fn sb_decl(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    while p.at_attr() {
        kids.push(Kid::Node(crate::declarations::attributes::syntax::attribute_spec(p)?));
    }
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
        _ => return Err(p.fail_rule("SB-15", "a structured binding declaration needs an initializer")),
    };
    Ok(p.node("sb_init", sp, kids))
}

/// SB-15: a declaration whose declarator is a list of names in brackets,
/// after specifiers or a declarator operator that `sb_decl` does not
/// take. The stop goes at the `]`, past the places where the other
/// readings stop.
pub fn near_miss(p: &mut Parser) {
    let mut k = 0;
    while k < 12 && !matches!(p.ahead(k), "[" | ";" | "{" | "}" | "=" | "" | ",") {
        k += 1;
    }
    let before = if k > 0 { p.ahead(k - 1) } else { "" };
    let typed = matches!(before, "auto" | ")" | "*" | "&" | "&&") || crate::foundations::types::check::is_fundamental(before);
    if p.ahead(k) != "[" || !typed || p.ahead_kind(k + 1) != crate::lex::TK::Ident {
        return;
    }
    let mut j = k + 2;
    while p.ahead(j) == "," && p.ahead_kind(j + 1) == crate::lex::TK::Ident {
        j += 2;
    }
    if p.ahead(j) != "]" || !matches!(p.ahead(j + 1), "=" | "(" | "{" | ";" | ")" | ",") {
        return;
    }
    let m = p.mark();
    for _ in 0..j {
        p.bump();
    }
    p.fail_rule("SB-15", "a structured binding declaration has 'auto' as its type, no other specifiers than static, thread_local, const, and volatile, and no declarator operator but '&' or '&&'");
    p.reset(m);
}
