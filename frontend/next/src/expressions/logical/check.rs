//! The check part of `expressions/logical`.

use crate::expressions::check::{bad, operands, operator};
use crate::foundations::conversions::check as conv;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, EK};
use crate::sem::types::QType;
use crate::syntax::Node;

/// An operand of `!`, `&&`, `||`: contextually converted to `bool`
/// (LOG-3; ENUM-11 for a scoped enumeration).
pub fn bool_operand(cx: &mut Check, e: Expr) -> Expr {
    conv::contextual_bool(cx, e, "LOG-3")
}

/// `logical_and_expr` and `logical_or_expr` (LOG-1, LOG-2).
pub fn logical(cx: &mut Check, n: &Node) -> Expr {
    let and = operator(cx, n) == "&&";
    let Some((l, r)) = operands(cx, n) else { return bad(n.span) };
    let l = bool_operand(cx, l);
    let r = bool_operand(cx, r);
    if l.ty.is_error() || r.ty.is_error() {
        return bad(n.span);
    }
    Expr::new(EK::Logical(and, Box::new(l), Box::new(r)), QType::bool(), Cat::PR, n.span)
}
