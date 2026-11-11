//! The Syntax segment of `templates/concepts`.

use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::declarators::syntax::parameters;
use crate::expressions::comma::syntax::expression;
use crate::expressions::logical::syntax::logical_or_expr;
use crate::expressions::primary::syntax::{identifier, nested_name, primary};
use crate::lex::TK;
use crate::syntax::names::{DeclCtx, Enter, Kind, Qual, TKind};
use crate::syntax::{Kid, Node, PResult, Parser};
use crate::templates::arguments::syntax::{template_args, template_id};
use crate::templates::declarations::syntax::template_head;

/// `concept_definition: template_head "concept" identifier {attribute_spec}
/// "=" logical_or_expr ";"`.
pub fn concept_definition(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    p.names.enter(Enter::Template);
    let r = (|| -> PResult<Vec<Kid>> {
        let mut kids = vec![Kid::Node(template_head(p)?)];
        kids.push(p.expect("concept")?);
        kids.push(identifier(p)?);
        while p.at_attr() {
            kids.push(Kid::Node(attribute_spec(p)?));
        }
        let head = p.node("concept_definition", sp, kids.clone());
        p.declared(&head, DeclCtx { specs: None, template: true, param: false });
        kids.push(p.expect("=")?);
        kids.push(Kid::Node(logical_or_expr(p)?));
        kids.push(p.expect(";")?);
        Ok(kids)
    })();
    p.names.leave();
    Ok(p.node("concept_definition", sp, r?))
}

/// `requires_clause: "requires" constraint_or`.
pub fn requires_clause(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kw = p.expect("requires")?;
    let c = constraint_or(p)?;
    Ok(p.node("requires_clause", sp, vec![kw, Kid::Node(c)]))
}

/// `constraint_or: constraint_and {"||" constraint_and}`.
pub fn constraint_or(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![Kid::Node(constraint_and(p)?)];
    while p.at("||") {
        kids.push(p.bump());
        kids.push(Kid::Node(constraint_and(p)?));
    }
    Ok(p.node("constraint_or", sp, kids))
}

/// `constraint_and: primary {"&&" primary}`.
pub fn constraint_and(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![Kid::Node(primary(p)?)];
    while p.at("&&") {
        kids.push(p.bump());
        kids.push(Kid::Node(primary(p)?));
    }
    Ok(p.node("constraint_and", sp, kids))
}

/// `type_constraint: [nested_name] identifier ["<" [template_args] ">"]`.
/// The name names a concept.
pub fn type_constraint(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    let mut q = Qual::None;
    if p.at("::") || p.ahead(1) == "::" {
        let (nn, qq, _) = nested_name(p)?;
        kids.push(Kid::Node(nn));
        q = qq;
    }
    if p.kind() != TK::Ident || p.find(q, p.peek()).kind != Kind::Template(TKind::Concept) {
        return Err(p.fail("a concept name"));
    }
    kids.push(identifier(p)?);
    if p.at("<") {
        kids.push(p.bump());
        let r = p.with(false, true, |p| -> PResult<()> {
            if !p.peek().starts_with('>') {
                kids.push(Kid::Node(template_args(p)?));
            }
            kids.push(p.close_angle()?);
            Ok(())
        });
        r?;
    }
    Ok(p.node("type_constraint", sp, kids))
}

/// `requires_expression: "requires" ["(" parameters ")"] "{" requirement {requirement} "}"`.
pub fn requires_expression(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("requires")?];
    p.names.enter(Enter::Prototype);
    let r = p.bracketed(|p| -> PResult<()> {
        if p.at("(") {
            kids.push(p.bump());
            kids.push(Kid::Node(parameters(p)?));
            kids.push(p.expect(")")?);
        }
        kids.push(p.expect("{")?);
        kids.push(Kid::Node(requirement(p)?));
        while !p.at("}") {
            kids.push(Kid::Node(requirement(p)?));
        }
        kids.push(p.bump());
        Ok(())
    });
    p.names.leave();
    r?;
    Ok(p.node("requires_expression", sp, kids))
}

/// `requirement: simple_requirement | type_requirement | compound_requirement
/// | nested_requirement`.
pub fn requirement(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let n = if p.at("typename") {
        match p.attempt(type_requirement) {
            Ok(n) => n,
            Err(_) => simple_requirement(p)?,
        }
    } else if p.at("{") {
        compound_requirement(p)?
    } else if p.at("requires") {
        match p.attempt(nested_requirement) {
            Ok(n) => n,
            Err(_) => simple_requirement(p)?,
        }
    } else {
        simple_requirement(p)?
    };
    Ok(p.node("requirement", sp, vec![Kid::Node(n)]))
}

/// `simple_requirement: expression ";"`.
pub fn simple_requirement(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let e = expression(p)?;
    let s = p.expect(";")?;
    Ok(p.node("simple_requirement", sp, vec![Kid::Node(e), s]))
}

/// `type_requirement: "typename" [nested_name ["template"]] (identifier | template_id) ";"`.
pub fn type_requirement(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("typename")?];
    let mut q = Qual::None;
    let mut tkw = false;
    if p.at("::") || p.ahead(1) == "::" || p.ahead(1) == "<" {
        if let Some((nn, qq, _)) = p.opt(nested_name) {
            kids.push(Kid::Node(nn));
            q = qq;
            if p.at("template") {
                kids.push(p.bump());
                tkw = true;
            }
        }
    }
    if p.ahead(1) == "<" && (tkw || p.find(q, p.peek()).is_template()) {
        kids.push(Kid::Node(template_id(p, q, tkw)?.0));
    } else {
        kids.push(identifier(p)?);
    }
    kids.push(p.expect(";")?);
    Ok(p.node("type_requirement", sp, kids))
}

/// `compound_requirement: "{" expression "}" ["noexcept"] ["->" type_constraint] ";"`.
pub fn compound_requirement(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("{")?, Kid::Node(expression(p)?), p.expect("}")?];
    if let Some(k) = p.eat("noexcept") {
        kids.push(k);
    }
    if p.at("->") {
        kids.push(p.bump());
        kids.push(Kid::Node(type_constraint(p)?));
    }
    kids.push(p.expect(";")?);
    Ok(p.node("compound_requirement", sp, kids))
}

/// `nested_requirement: "requires" logical_or_expr ";"`.
pub fn nested_requirement(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kids = vec![p.expect("requires")?, Kid::Node(logical_or_expr(p)?), p.expect(";")?];
    Ok(p.node("nested_requirement", sp, kids))
}
