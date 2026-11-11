//! The Syntax segment of `declarations/namespaces`.

use crate::declarations::declaration;
use crate::expressions::primary::syntax::{identifier, qualified_name};
use crate::syntax::names::{DeclCtx, Enter};
use crate::syntax::{Kid, Node, PResult, Parser};

/// `namespace_def: ["inline"] "namespace" [ns_path] "{" {declaration} "}"`.
pub fn namespace_def(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if let Some(k) = p.eat("inline") {
        kids.push(k);
    }
    kids.push(p.expect("namespace")?);
    if !p.at("{") {
        kids.push(Kid::Node(ns_path(p)?));
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
            let mut d = declaration(p)?;
            p.fill(&mut d);
            kids.push(Kid::Node(d));
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

/// `namespace_alias: "namespace" identifier "=" qualified_name ";"`.
pub fn namespace_alias(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kids = vec![p.expect("namespace")?, identifier(p)?, p.expect("=")?, Kid::Node(qualified_name_or_id(p)?), p.expect(";")?];
    let n = p.node("namespace_alias", sp, kids);
    p.declared(&n, DeclCtx::default());
    Ok(n)
}

/// A `qualified_name`. A namespace alias and a using-directive name a
/// namespace, which can stand without a qualifier; the grammar asks for
/// `qualified_name` there.
fn qualified_name_or_id(p: &mut Parser) -> PResult<Node> {
    qualified_name(p)
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

/// `using_directive: "using" "namespace" qualified_name ";"`.
pub fn using_directive(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kids = vec![p.expect("using")?, p.expect("namespace")?, Kid::Node(qualified_name_or_id(p)?), p.expect(";")?];
    let n = p.node("using_directive", sp, kids);
    p.declared(&n, DeclCtx::default());
    Ok(n)
}
