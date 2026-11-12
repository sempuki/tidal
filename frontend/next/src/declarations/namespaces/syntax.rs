//! The Syntax segment of `declarations/namespaces`.

use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::declaration;
use crate::expressions::primary::syntax::{identifier, nested_name, qualified_name};
use crate::syntax::names::{DeclCtx, Enter};
use crate::syntax::{Kid, Node, PResult, Parser};

/// `namespace_def: ["inline"] "namespace" {attribute_spec} [identifier]
/// "{" {declaration} "}" | "namespace" ns_path "{" {declaration} "}"`.
/// The first form's name sits in an `ns_path` node of one identifier, so
/// the declare pass reads both forms alike.
pub fn namespace_def(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    let inline = p.at("inline");
    if let Some(k) = p.eat("inline") {
        kids.push(k);
    }
    kids.push(p.expect("namespace")?);
    let attributed = p.at_attr();
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    if !p.at("{") {
        let path = if inline || attributed || p.ahead(1) != "::" {
            let psp = p.span();
            let id = identifier(p)?;
            p.node("ns_path", psp, vec![id])
        } else {
            ns_path(p)?
        };
        kids.push(Kid::Node(path));
    }
    if !p.at("{") {
        return Err(p.fail("'{'"));
    }
    let head = p.node("namespace_def", sp, kids.clone());
    let scopes = p.declared(&head, DeclCtx::default());
    for s in &scopes {
        p.names.enter(Enter::Scope(*s));
    }
    kids.push(p.bump());
    let r = (|| -> PResult<()> {
        while !p.at("}") {
            if p.at_eof() {
                return Err(p.fail("'}'"));
            }
            if let Some(mut d) = p.item(declaration)? {
                p.fill(&mut d);
                kids.push(Kid::Node(d));
            }
        }
        kids.push(p.bump());
        Ok(())
    })();
    for _ in &scopes {
        p.names.leave();
    }
    r?;
    Ok(p.node("namespace_def", sp, kids))
}

/// `ns_path: ["inline"] identifier {"::" ["inline"] identifier}`.
pub fn ns_path(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    loop {
        if let Some(k) = p.eat("inline") {
            kids.push(k);
        }
        kids.push(identifier(p)?);
        match p.eat("::") {
            Some(k) => kids.push(k),
            None => break,
        }
    }
    Ok(p.node("ns_path", sp, kids))
}

/// `namespace_alias: "namespace" identifier "=" [nested_name] identifier ";"`.
pub fn namespace_alias(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("namespace")?, identifier(p)?, p.expect("=")?];
    namespace_name(p, &mut kids)?;
    kids.push(p.expect(";")?);
    let n = p.node("namespace_alias", sp, kids);
    p.declared(&n, DeclCtx::default());
    Ok(n)
}

/// `[nested_name] identifier`, the namespace that an alias or a
/// directive names.
fn namespace_name(p: &mut Parser, kids: &mut Vec<Kid>) -> PResult<()> {
    if p.at("::") || p.ahead(1) == "::" {
        kids.push(Kid::Node(nested_name(p)?.0));
    }
    kids.push(identifier(p)?);
    Ok(())
}

/// `using_decl: "using" ["typename"] qualified_name ["..."]
/// {"," ["typename"] qualified_name ["..."]} ";"`.
pub fn using_decl(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("using")?];
    loop {
        if let Some(k) = p.eat("typename") {
            kids.push(k);
        }
        kids.push(Kid::Node(qualified_name(p)?));
        if let Some(k) = p.eat("...") {
            kids.push(k);
        }
        match p.eat(",") {
            Some(k) => kids.push(k),
            None => break,
        }
    }
    kids.push(p.expect(";")?);
    let n = p.node("using_decl", sp, kids);
    p.declared(&n, DeclCtx::default());
    Ok(n)
}

/// `using_directive: {attribute_spec} "using" "namespace" [nested_name] identifier ";"`.
pub fn using_directive(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    kids.push(p.expect("using")?);
    kids.push(p.expect("namespace")?);
    namespace_name(p, &mut kids)?;
    kids.push(p.expect(";")?);
    let n = p.node("using_directive", sp, kids);
    p.declared(&n, DeclCtx::default());
    Ok(n)
}
