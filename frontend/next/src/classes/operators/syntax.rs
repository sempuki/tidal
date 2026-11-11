//! The Syntax segment of `classes/operators`.

use crate::lex::{Lit, TK};
use crate::syntax::{Kid, Node, PResult, Parser};

/// `operator_function_id: "operator" overloadable_operator`.
pub fn operator_function_id(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let op = p.expect("operator")?;
    let o = overloadable_operator(p)?;
    Ok(p.node("operator_function_id", sp, vec![op, Kid::Node(o)]))
}

const SINGLE: &[&str] = &[
    "co_await", "->", "->*", "~", "!", "+", "-", "*", "/", "%", "^", "&", "|", "=", "+=", "-=", "*=", "/=", "%=", "^=",
    "&=", "|=", "==", "!=", "<", ">", "<=", ">=", "<=>", "&&", "||", "<<", ">>", "<<=", ">>=", "++", "--", ",",
];

/// `overloadable_operator`: `new`, `delete`, their array forms, `()`,
/// `[]`, and the operators of the list.
pub fn overloadable_operator(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if p.at("new") || p.at("delete") {
        kids.push(p.bump());
        if p.at("[") && p.ahead(1) == "]" {
            kids.push(p.bump());
            kids.push(p.bump());
        }
    } else if p.at("(") && p.ahead(1) == ")" || p.at("[") && p.ahead(1) == "]" {
        kids.push(p.bump());
        kids.push(p.bump());
    } else if p.at_any(SINGLE) {
        kids.push(p.bump());
    } else {
        return Err(p.fail("an operator"));
    }
    Ok(p.node("overloadable_operator", sp, kids))
}

/// `literal_operator_id: "operator" '""' identifier`. The lexer joins
/// `""_x` into one token with a user-defined suffix.
pub fn literal_operator_id(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let op = p.expect("operator")?;
    if p.kind() != TK::Str {
        return Err(p.fail("'\"\"'"));
    }
    let empty = |l: &Option<Lit>| matches!(l, Some(Lit::Str { units, .. }) if units.is_empty());
    let t = p.tok();
    let joined = matches!(&t.lit, Some(Lit::UserDefined { base, .. }) if empty(&Some((**base).clone())));
    if joined {
        let s = p.bump();
        return Ok(p.node("literal_operator_id", sp, vec![op, s]));
    }
    if !empty(&t.lit) {
        return Err(p.fail("'\"\"'"));
    }
    let s = p.bump();
    let id = crate::expressions::primary::syntax::identifier(p)?;
    Ok(p.node("literal_operator_id", sp, vec![op, s, id]))
}
