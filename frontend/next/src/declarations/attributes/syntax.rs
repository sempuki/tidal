//! The Syntax segment of `declarations/attributes`. The lexer keeps `[[`
//! and `]]` as two tokens each.

use crate::expressions::primary::syntax::identifier;
use crate::lex::TK;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `attribute_spec: "[[" ["using" identifier ":"] [attribute {"," attribute}] "]]"`.
pub fn attribute_spec(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if !p.at_attr() {
        return Err(p.fail("'[['"));
    }
    let mut kids = vec![p.bump(), p.bump()];
    if p.at("using") {
        kids.push(p.bump());
        kids.push(identifier(p)?);
        kids.push(p.expect(":")?);
    }
    if !p.at("]") {
        loop {
            kids.push(Kid::Node(attribute(p)?));
            match p.eat(",") {
                Some(k) => kids.push(k),
                None => break,
            }
        }
    }
    kids.push(p.expect("]")?);
    kids.push(p.expect("]")?);
    Ok(p.node("attribute_spec", sp, kids))
}

/// An attribute's name: an identifier, or a keyword spelled as one.
fn attr_name(p: &mut Parser) -> PResult<Kid> {
    if p.kind() == TK::Ident {
        Ok(p.bump())
    } else {
        Err(p.fail("an attribute name"))
    }
}

/// `attribute: [identifier "::"] identifier [attribute_args]`. The
/// arguments of `assume` are an expression where they parse as one.
pub fn attribute(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let assume = p.at("assume") && p.ahead(1) == "(";
    let mut kids = vec![attr_name(p)?];
    if p.at("::") {
        kids.push(p.bump());
        kids.push(attr_name(p)?);
    }
    if assume && p.at("(") {
        if let Ok(a) = p.attempt(assume_args) {
            kids.push(Kid::Node(a));
            return Ok(p.node("attribute", sp, kids));
        }
    }
    if p.at("(") {
        kids.push(Kid::Node(attribute_args(p)?));
    }
    Ok(p.node("attribute", sp, kids))
}

/// The `attribute_args` of `assume` where its tokens form one
/// expression in parentheses (ATTR-9).
fn assume_args(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let open = p.expect("(")?;
    let e = crate::expressions::comma::syntax::expression(p)?;
    let close = p.expect(")")?;
    Ok(p.node("attribute_args", sp, vec![open, Kid::Node(e), close]))
}

/// `attribute_args: "(" {balanced_token} ")"`.
pub fn attribute_args(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("(")?];
    while !p.at(")") {
        kids.push(Kid::Node(balanced_token(p)?));
    }
    kids.push(p.bump());
    Ok(p.node("attribute_args", sp, kids))
}

/// `balanced_token`: a bracketed run of balanced tokens, or one token.
pub fn balanced_token(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let close = match p.peek() {
        "(" => ")",
        "[" => "]",
        "{" => "}",
        _ => {
            let t = token(p)?;
            return Ok(p.node("balanced_token", sp, vec![Kid::Node(t)]));
        }
    };
    let mut kids = vec![p.bump()];
    while !p.at(close) {
        kids.push(Kid::Node(balanced_token(p)?));
    }
    kids.push(p.bump());
    Ok(p.node("balanced_token", sp, kids))
}

/// `token`: any single token other than a bracket.
pub fn token(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at_eof() || p.at_any(&["(", ")", "[", "]", "{", "}"]) {
        return Err(p.fail("a token"));
    }
    let k = p.bump();
    Ok(p.node("token", sp, vec![k]))
}
