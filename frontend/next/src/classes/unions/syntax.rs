//! The Syntax segment of `classes/unions`.

use crate::classes::members::syntax::{class_body, class_name, member_init};
use crate::declarations::alignment::syntax::alignment_specifier;
use crate::declarations::attributes::syntax::attribute_spec;
use crate::expressions::primary::syntax::identifier;
use crate::foundations::constant_evaluation::syntax::constant_expr;
use crate::lex::{Token, TK};
use crate::syntax::names::DeclCtx;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `union_def: "union" {attribute_spec | alignment_specifier}
/// [class_name ["final"]] "{" {member_decl} "}"`.
pub fn union_def(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("union")?];
    while p.at_attr() || p.at("alignas") {
        kids.push(Kid::Node(if p.at("alignas") { alignment_specifier(p)? } else { attribute_spec(p)? }));
    }
    if p.kind() == TK::Ident {
        kids.push(Kid::Node(class_name(p)?));
        if p.at("final") && p.ahead(1) == "{" {
            kids.push(p.bump());
        }
    }
    if !p.at("{") {
        return Err(p.fail("'{'"));
    }
    let tmpl = p.take_template();
    let head = p.node("union_def", sp, kids.clone());
    let scopes = p.declared(&head, DeclCtx { specs: None, template: tmpl, param: false });
    kids.extend(class_body(p, scopes.last().copied())?);
    Ok(p.node("union_def", sp, kids))
}

/// `anonymous_union: ["static"] "union" "{" {member_decl} "}" ";"`: a
/// simple declaration whose only type is an unnamed union and which has
/// no declarator.
pub fn anonymous_union(n: &Node, toks: &[Token]) -> bool {
    let Some(sd) = n.find("simple_decl") else { return false };
    if sd.child("init_declarator").is_some() {
        return false;
    }
    let Some(specs) = sd.child("decl_specifiers") else { return false };
    let mut u = 0;
    for ds in specs.children("decl_specifier") {
        let mut ids = Vec::new();
        ds.tokens(&mut ids);
        match ds.find("union_def") {
            Some(ud) if ud.child("class_name").is_none() => u += 1,
            _ if ids.len() == 1 && toks[ids[0]].text == "static" => {}
            _ => return false,
        }
    }
    u == 1
}

/// `bitfield: [identifier] ":" constant_expr [member_init]`.
pub fn bitfield(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if p.kind() == TK::Ident {
        kids.push(identifier(p)?);
    }
    kids.push(p.expect(":")?);
    kids.push(Kid::Node(constant_expr(p)?));
    if p.at("=") || p.at("{") {
        kids.push(Kid::Node(member_init(p)?));
    }
    Ok(p.node("bitfield", sp, kids))
}
