//! The Syntax segment of `templates/declarations`.

use crate::declarations::declarators::syntax::{ptr_part, type_id};
use crate::declarations::functions::default_arguments::syntax::parameter;
use crate::declarations::{decl_specifiers, declaration};
use crate::expressions::primary::syntax::{identifier, nested_name};
use crate::lex::TK;
use crate::syntax::names::{DeclCtx, Enter};
use crate::syntax::{Kid, Node, PResult, Parser};
use crate::templates::concepts::syntax::{requires_clause, type_constraint};

/// `template_decl: template_head declaration`. The head's parameters
/// are in scope for the declaration.
pub fn template_decl(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    p.names.enter(Enter::Template);
    let r = (|| -> PResult<Vec<Kid>> {
        let h = template_head(p)?;
        p.template = true;
        let d = declaration(p);
        p.template = false;
        Ok(vec![Kid::Node(h), Kid::Node(d?)])
    })();
    p.names.leave();
    Ok(p.node("template_decl", sp, r?))
}

/// `template_head: "template" "<" template_params ">" [requires_clause]`.
pub fn template_head(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("template")?, p.expect("<")?];
    kids.push(Kid::Node(p.with(false, true, template_params)?));
    kids.push(p.close_angle()?);
    if p.at("requires") {
        kids.push(Kid::Node(requires_clause(p)?));
    }
    Ok(p.node("template_head", sp, kids))
}

/// `template_params: template_param {"," template_param}`.
pub fn template_params(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![Kid::Node(template_param(p)?)];
    while p.at(",") {
        kids.push(p.bump());
        kids.push(Kid::Node(template_param(p)?));
    }
    Ok(p.node("template_params", sp, kids))
}

fn param_ends(p: &Parser) -> bool {
    p.at(",") || p.peek().starts_with('>') || p.at_eof()
}

/// `template_param: type_param | constant_param | template_template_param`.
pub fn template_param(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let n = if p.at("template") {
        template_template_param(p)?
    } else if let Ok(n) = p.attempt(|p| {
        let n = type_param(p)?;
        if param_ends(p) {
            Ok(n)
        } else {
            Err(p.fail("',' or '>'"))
        }
    }) {
        n
    } else {
        constant_param(p)?
    };
    Ok(p.node("template_param", sp, vec![Kid::Node(n)]))
}

/// `type_param: type_key ["..."] [identifier] | type_key [identifier] "=" type_id
/// | type_constraint ["..."] [identifier] | type_constraint [identifier] "=" type_id`.
pub fn type_param(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![Kid::Node(if p.at("class") || p.at("typename") { type_key(p)? } else { type_constraint(p)? })];
    let pack = p.at("...");
    if pack {
        kids.push(p.bump());
    }
    if p.kind() == TK::Ident && !p.at(",") {
        kids.push(identifier(p)?);
    }
    let n = p.node("type_param", sp, kids.clone());
    p.declared(&n, DeclCtx { specs: None, template: false, param: true });
    if !pack && p.at("=") {
        kids.push(p.bump());
        kids.push(Kid::Node(type_id(p)?));
    }
    Ok(p.node("type_param", sp, kids))
}

/// `type_key: "class" | "typename"`.
pub fn type_key(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at("class") || p.at("typename") {
        let k = p.bump();
        return Ok(p.node("type_key", sp, vec![k]));
    }
    Err(p.fail("'class' or 'typename'"))
}

/// `constant_param: parameter | decl_specifiers {ptr_part} "..." [identifier]`.
pub fn constant_param(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if let Ok(prm) = p.attempt(|p| {
        let prm = parameter(p, true)?;
        if param_ends(p) {
            Ok(prm)
        } else {
            Err(p.fail("',' or '>'"))
        }
    }) {
        return Ok(p.node("constant_param", sp, vec![Kid::Node(prm)]));
    }
    let mut kids = vec![Kid::Node(decl_specifiers(p)?)];
    while let Some(pp) = p.opt(ptr_part) {
        kids.push(Kid::Node(pp));
    }
    kids.push(p.expect("...")?);
    if p.kind() == TK::Ident {
        kids.push(identifier(p)?);
    }
    let n = p.node("constant_param", sp, kids);
    p.declared(&n, DeclCtx { specs: None, template: false, param: true });
    Ok(n)
}

/// `template_template_param: template_head type_key ["..."] [identifier]
/// | template_head type_key [identifier] "=" [nested_name] ["template"] identifier`.
pub fn template_template_param(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    p.names.enter(Enter::Template);
    let head = template_head(p);
    p.names.leave();
    let mut kids = vec![Kid::Node(head?), Kid::Node(type_key(p)?)];
    let pack = p.at("...");
    if pack {
        kids.push(p.bump());
    }
    if p.kind() == TK::Ident {
        kids.push(identifier(p)?);
    }
    let n = p.node("template_template_param", sp, kids.clone());
    p.declared(&n, DeclCtx { specs: None, template: false, param: true });
    if !pack && p.at("=") {
        kids.push(p.bump());
        if p.at("::") || p.ahead(1) == "::" {
            kids.push(Kid::Node(nested_name(p)?.0));
            if let Some(k) = p.eat("template") {
                kids.push(k);
            }
        }
        kids.push(identifier(p)?);
    }
    Ok(p.node("template_template_param", sp, kids))
}
