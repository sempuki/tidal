//! The Syntax segment of `classes/derived`.

use crate::classes::access::syntax::access_specifier;
use crate::classes::members::syntax::class_name;
use crate::declarations::alignment::syntax::alignment_specifier;
use crate::declarations::attributes::syntax::attribute_spec;
use crate::expressions::primary::syntax::nested_name;
use crate::lex::{Token, TK};
use crate::syntax::names::Qual;
use crate::syntax::{Kid, Node, PResult, Parser};
use crate::templates::arguments::syntax::template_id;

/// `base_clause: ":" base_spec ["..."] {"," base_spec ["..."]}`.
pub fn base_clause(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect(":")?];
    p.with(true, true, |p| -> PResult<()> {
        loop {
            kids.push(Kid::Node(base_spec(p)?));
            if let Some(k) = p.eat("...") {
                kids.push(k);
            }
            match p.eat(",") {
                Some(k) => kids.push(k),
                None => return Ok(()),
            }
        }
    })?;
    Ok(p.node("base_clause", sp, kids))
}

/// `base_spec: [access_specifier] ["virtual"] [nested_name] class_name
/// | "virtual" access_specifier [nested_name] class_name`.
pub fn base_spec(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if p.at("virtual") {
        kids.push(p.bump());
        if let Some(a) = p.opt(access_specifier) {
            kids.push(Kid::Node(a));
        }
    } else {
        if let Some(a) = p.opt(access_specifier) {
            kids.push(Kid::Node(a));
        }
        if let Some(k) = p.eat("virtual") {
            kids.push(k);
        }
    }
    if p.at("::") || p.ahead(1) == "::" || p.at("decltype") {
        kids.push(Kid::Node(nested_name(p)?.0));
    }
    kids.push(Kid::Node(class_name(p)?));
    Ok(p.node("base_spec", sp, kids))
}

/// `class_head: ("class" | "struct") {attribute_spec | alignment_specifier}
/// [[nested_name] (class_name | template_id) ["final"]] [base_clause]`.
pub fn class_head(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if !(p.at("class") || p.at("struct")) {
        return Err(p.fail("'class' or 'struct'"));
    }
    let mut kids = vec![p.bump()];
    while p.at_attr() || p.at("alignas") {
        kids.push(Kid::Node(if p.at("alignas") { alignment_specifier(p)? } else { attribute_spec(p)? }));
    }
    if p.kind() == TK::Ident || p.at("::") {
        let mut q = Qual::None;
        if p.at("::") || p.ahead(1) == "::" || p.ahead(1) == "<" {
            if let Some((nn, qq, _)) = p.opt(nested_name) {
                kids.push(Kid::Node(nn));
                q = qq;
            }
        }
        if p.ahead(1) == "<" && p.find(q, p.peek()).is_template() {
            kids.push(Kid::Node(template_id(p, q, false)?.0));
        } else {
            kids.push(Kid::Node(class_name(p)?));
        }
        if p.at("final") && matches!(p.ahead(1), "{" | ":") {
            kids.push(p.bump());
        }
    }
    if p.at(":") {
        kids.push(Kid::Node(base_clause(p)?));
    }
    Ok(p.node("class_head", sp, kids))
}

/// `inheriting_decl: "using" [nested_name] class_name "::" class_name ["..."] ";"`,
/// as a shape of a `using_decl`: the two names are the same, or the last
/// names the class the declaration sits in.
pub fn inheriting_decl(n: &Node, toks: &[Token]) -> bool {
    if n.prod != "using_decl" {
        return false;
    }
    let mut ids = Vec::new();
    n.tokens(&mut ids);
    let words: Vec<&str> = ids.iter().map(|&i| toks[i].text.as_str()).collect();
    let k = words.len();
    k >= 5 && words[0] == "using" && words[k - 1] == ";" && words[k - 3] == "::" && {
        let last = words[k - 2];
        let before = words[k - 4];
        let base = if before == ">" { words.iter().rposition(|w| *w == "<").map(|i| words[i - 1]).unwrap_or("") } else { before };
        last == base
    }
}
