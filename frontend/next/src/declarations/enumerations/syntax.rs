//! The Syntax segment of `declarations/enumerations`.

use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::type_specifiers;
use crate::expressions::primary::syntax::{identifier, nested_name};
use crate::foundations::constant_evaluation::syntax::constant_expr;
use crate::lex::TK;
use crate::syntax::names::DeclCtx;
use crate::syntax::{Kid, Node, PResult, Parser};

fn attrs(p: &mut Parser, kids: &mut Vec<Kid>) -> PResult<()> {
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    Ok(())
}

/// `enum_def: enum_head [":" type_specifiers] "{" [enumerator {"," enumerator} [","]] "}"`.
pub fn enum_def(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let head = enum_head(p)?;
    let mut kids = vec![Kid::Node(head.clone())];
    if p.at(":") {
        kids.push(p.bump());
        kids.push(Kid::Node(type_specifiers(p)?));
    }
    if !p.at("{") {
        return Err(p.fail("'{'"));
    }
    let tmpl = p.take_template();
    p.declared(&head, DeclCtx { specs: None, template: tmpl, param: false });
    kids.push(p.bump());
    p.bracketed(|p| -> PResult<()> {
        while !p.at("}") {
            kids.push(Kid::Node(enumerator(p)?));
            match p.eat(",") {
                Some(k) => kids.push(k),
                None => break,
            }
        }
        Ok(())
    })?;
    kids.push(p.expect("}")?);
    Ok(p.node("enum_def", sp, kids))
}

/// `enum_opaque: "enum" ["class" | "struct"] {attribute_spec} [nested_name]
/// identifier [":" type_specifiers] ";"`.
pub fn enum_opaque(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("enum")?];
    if p.at("class") || p.at("struct") {
        kids.push(p.bump());
    }
    attrs(p, &mut kids)?;
    if p.at("::") || p.ahead(1) == "::" || p.ahead(1) == "<" {
        kids.push(Kid::Node(nested_name(p)?.0));
    }
    kids.push(identifier(p)?);
    if p.at(":") {
        kids.push(p.bump());
        kids.push(Kid::Node(type_specifiers(p)?));
    }
    kids.push(p.expect(";")?);
    let n = p.node("enum_opaque", sp, kids);
    let tmpl = p.take_template();
    p.declared(&n, DeclCtx { specs: None, template: tmpl, param: false });
    Ok(n)
}

/// `enum_head: "enum" {attribute_spec} [[nested_name] identifier]
/// | "enum" ("class" | "struct") {attribute_spec} [nested_name] identifier`.
pub fn enum_head(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("enum")?];
    let scoped = p.at("class") || p.at("struct");
    if scoped {
        kids.push(p.bump());
    }
    attrs(p, &mut kids)?;
    if p.kind() == TK::Ident || p.at("::") || scoped {
        if p.at("::") || p.ahead(1) == "::" || p.ahead(1) == "<" {
            kids.push(Kid::Node(nested_name(p)?.0));
        }
        kids.push(identifier(p)?);
    }
    Ok(p.node("enum_head", sp, kids))
}

/// `enumerator: identifier {attribute_spec} ["=" constant_expr]`.
pub fn enumerator(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![identifier(p)?];
    attrs(p, &mut kids)?;
    if p.at("=") {
        kids.push(p.bump());
        kids.push(Kid::Node(constant_expr(p)?));
    }
    Ok(p.node("enumerator", sp, kids))
}

/// `using_enum_decl: "using" "enum" [nested_name] identifier ";"`, the
/// `using enum` declaration of ENUM-9, at namespace scope, in a block,
/// and in a class.
pub fn using_enum_decl(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("using")?, p.expect("enum")?];
    if p.at("::") || p.ahead(1) == "::" || p.ahead(1) == "<" {
        if let Some((nn, _, _)) = p.opt(nested_name) {
            kids.push(Kid::Node(nn));
        }
    }
    if p.kind() == TK::Ident && p.ahead(1) == "<" {
        kids.push(Kid::Node(crate::templates::arguments::syntax::template_id(p, crate::syntax::names::Qual::None, false)?.0));
    } else {
        kids.push(identifier(p)?);
    }
    kids.push(p.expect(";")?);
    Ok(p.node("using_enum_decl", sp, kids))
}
