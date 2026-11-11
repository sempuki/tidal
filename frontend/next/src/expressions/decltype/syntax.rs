//! The Syntax segment of `expressions/decltype`.

use crate::expressions::comma::syntax::expression;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `decltype_spec: "decltype" "(" expression ")" | "decltype" "(" "auto" ")"`.
pub fn decltype_spec(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kw = p.expect("decltype")?;
    let open = p.expect("(")?;
    let inner = if p.at("auto") && p.ahead(1) == ")" { p.bump() } else { Kid::Node(p.bracketed(expression)?) };
    let close = p.expect(")")?;
    Ok(p.node("decltype_spec", sp, vec![kw, open, inner, close]))
}
