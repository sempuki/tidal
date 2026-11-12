//! The Syntax segment of `expressions/decltype`.

use crate::expressions::comma::syntax::expression;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `decltype_spec: decltype_expr | "decltype" "(" "auto" ")"`.
pub fn decltype_spec(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at("decltype") && p.ahead(1) == "(" && p.ahead(2) == "auto" && p.ahead(3) == ")" {
        let kids = vec![p.bump(), p.bump(), p.bump(), p.bump()];
        return Ok(p.node("decltype_spec", sp, kids));
    }
    let e = decltype_expr(p)?;
    Ok(p.node("decltype_spec", sp, vec![Kid::Node(e)]))
}

/// `decltype_expr: "decltype" "(" expression ")"`: the form that names a
/// base, starts a `nested_name`, follows `~`, or is a functional cast's
/// type.
pub fn decltype_expr(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kw = p.expect("decltype")?;
    let open = p.expect("(")?;
    let inner = Kid::Node(p.bracketed(expression)?);
    let close = p.expect(")")?;
    Ok(p.node("decltype_expr", sp, vec![kw, open, inner, close]))
}
