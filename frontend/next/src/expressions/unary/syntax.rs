//! The Syntax segment of `expressions/unary`.

use crate::declarations::declarators::syntax::type_id;
use crate::declarations::type_start;
use crate::expressions::{casts, comma, new_delete, postfix};
use crate::expressions::primary::syntax::identifier;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `unary_expr`: a postfix expression, a prefix operator, `sizeof`,
/// `alignof`, `noexcept`, or a `new` or `delete` expression.
pub fn unary_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("unary_expr") {
        return Ok(n);
    }
    let sp = p.span();
    if p.at_any(&["+", "-", "*", "&", "!", "~"]) {
        let op = p.bump();
        let e = casts::syntax::cast_expr(p)?;
        return Ok(p.node("unary_expr", sp, vec![op, Kid::Node(e)]));
    }
    if p.at_any(&["++", "--"]) {
        let op = p.bump();
        let e = unary_expr(p)?;
        return Ok(p.node("unary_expr", sp, vec![op, Kid::Node(e)]));
    }
    if p.at("sizeof") {
        let kw = p.bump();
        if p.at("...") {
            let dots = p.bump();
            let open = p.expect("(")?;
            let id = identifier(p)?;
            let close = p.expect(")")?;
            return Ok(p.node("unary_expr", sp, vec![kw, dots, open, id, close]));
        }
        if p.at("(") {
            if let Ok(kids) = p.attempt(|p| {
                let open = p.bump();
                if !type_start(p) {
                    return Err(p.fail("a type"));
                }
                let t = p.bracketed(type_id)?;
                let close = p.expect(")")?;
                Ok(vec![open, Kid::Node(t), close])
            }) {
                let mut all = vec![kw];
                all.extend(kids);
                return Ok(p.node("unary_expr", sp, all));
            }
        }
        let e = unary_expr(p)?;
        return Ok(p.node("unary_expr", sp, vec![kw, Kid::Node(e)]));
    }
    if p.at("alignof") {
        let kw = p.bump();
        let open = p.expect("(")?;
        let t = p.bracketed(type_id)?;
        let close = p.expect(")")?;
        return Ok(p.node("unary_expr", sp, vec![kw, open, Kid::Node(t), close]));
    }
    if p.at("noexcept") {
        let kw = p.bump();
        let open = p.expect("(")?;
        let e = p.bracketed(comma::syntax::expression)?;
        let close = p.expect(")")?;
        return Ok(p.node("unary_expr", sp, vec![kw, open, Kid::Node(e), close]));
    }
    let global = p.at("::");
    if p.at("new") || (global && p.ahead(1) == "new") {
        let n = new_delete::syntax::new_expr(p)?;
        return Ok(p.node("unary_expr", sp, vec![Kid::Node(n)]));
    }
    if p.at("delete") || (global && p.ahead(1) == "delete") {
        let n = new_delete::syntax::delete_expr(p)?;
        return Ok(p.node("unary_expr", sp, vec![Kid::Node(n)]));
    }
    postfix::syntax::postfix_expr(p)
}
