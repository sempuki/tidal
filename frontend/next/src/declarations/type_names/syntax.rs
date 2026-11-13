//! The Syntax segment of `declarations/type-names`. `typedef_decl`,
//! `forward_class`, and `specialization_class` are shapes, as recognizers
//! over a simple declaration.

use crate::declarations::alignment::syntax::alignment_specifier;
use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::declarators::syntax::type_id;
use crate::expressions::primary::syntax::{identifier, nested_name};
use crate::lex::Token;
use crate::syntax::names::{DeclCtx, Qual};
use crate::syntax::{Kid, Node, PResult, Parser};
use crate::templates::arguments::syntax::template_id;

/// `alias_decl: "using" identifier {attribute_spec} "=" type_id ";"`.
pub fn alias_decl(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("using")?, identifier(p)?];
    if p.at("alignas") {
        return Err(p.fail_rule("ALN-2", "alignas does not apply to an alias declaration"));
    }
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    kids.push(p.expect("=")?);
    kids.push(Kid::Node(p.with(true, true, type_id)?));
    kids.push(p.expect(";")?);
    let n = p.node("alias_decl", sp, kids);
    let tmpl = p.take_template();
    p.declared(&n, DeclCtx { specs: None, template: tmpl, param: false });
    Ok(n)
}

/// `elaborated_type: class_key {attribute_spec | alignment_specifier}
/// [nested_name] (identifier | template_id) | "enum" [nested_name] identifier`.
pub fn elaborated_type(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    let is_enum = p.at("enum");
    if is_enum {
        kids.push(p.bump());
    } else {
        kids.push(Kid::Node(class_key(p)?));
        while p.at_attr() || p.at("alignas") {
            kids.push(Kid::Node(if p.at("alignas") { alignment_specifier(p)? } else { attribute_spec(p)? }));
        }
    }
    let mut q = Qual::None;
    if p.at("::") || p.ahead(1) == "::" || (p.ahead(1) == "<" && !is_enum) {
        if let Some((nn, qq, _)) = p.opt(nested_name) {
            kids.push(Kid::Node(nn));
            q = qq;
        }
    }
    if !is_enum && p.ahead(1) == "<" && (p.find(q, p.peek()).is_template() || q == Qual::Dependent) {
        kids.push(Kid::Node(template_id(p, q, false)?.0));
    } else {
        kids.push(identifier(p)?);
    }
    let n = p.node("elaborated_type", sp, kids);
    let tmpl = p.template && p.at(";");
    if tmpl {
        p.template = false;
    }
    p.declared(&n, DeclCtx { specs: None, template: tmpl, param: false });
    Ok(n)
}

/// `class_key: "class" | "struct" | "union"`.
pub fn class_key(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at_any(&["class", "struct", "union"]) {
        let k = p.bump();
        return Ok(p.node("class_key", sp, vec![k]));
    }
    Err(p.fail("a class key"))
}

/// `typedef_decl: "typedef" decl_specifiers declarator {"," declarator} ";"`:
/// a simple declaration whose first specifier is `typedef` and whose
/// declarators have no initializer.
pub fn typedef_decl(n: &Node, toks: &[Token]) -> bool {
    let Some(sd) = n.find("simple_decl") else { return false };
    let Some(specs) = sd.child("decl_specifiers") else { return false };
    let first = specs.children("decl_specifier").next().map(|d| d.words(toks));
    first == Some(vec!["typedef"]) && sd.children("init_declarator").all(|d| d.kids.len() == 1)
}

/// `forward_class: class_key {attribute_spec | alignment_specifier} identifier ";"`:
/// a simple declaration of an elaborated class type alone.
pub fn forward_class(n: &Node, toks: &[Token]) -> bool {
    let Some(sd) = n.find("simple_decl") else { return false };
    let Some(specs) = sd.child("decl_specifiers") else { return false };
    let ds: Vec<&Node> = specs.children("decl_specifier").collect();
    ds.len() == 1
        && sd.child("init_declarator").is_none()
        && ds[0].find("elaborated_type").is_some_and(|e| e.child("class_key").is_some() && e.child("nested_name").is_none() && e.child("template_id").is_none())
        && sd.words(toks).last() == Some(&";")
}

/// `specialization_class: class_key {attribute_spec | alignment_specifier}
/// [nested_name] template_id ";"`: a simple declaration of an elaborated
/// class type alone, named by a template-id.
pub fn specialization_class(n: &Node, toks: &[Token]) -> bool {
    let Some(sd) = n.find("simple_decl") else { return false };
    let Some(specs) = sd.child("decl_specifiers") else { return false };
    let ds: Vec<&Node> = specs.children("decl_specifier").collect();
    ds.len() == 1
        && sd.child("init_declarator").is_none()
        && ds[0].find("elaborated_type").is_some_and(|e| e.child("class_key").is_some() && e.child("template_id").is_some())
        && sd.words(toks).last() == Some(&";")
}
