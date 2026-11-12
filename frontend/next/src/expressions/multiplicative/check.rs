//! The check part of `expressions/multiplicative`.

use crate::expressions::check::{bad, operands, operator};
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::rvalue;
use crate::sem::check::Check;
use crate::sem::tree::{BOp, Cat, Expr, EK};
use crate::sem::types::{QType, Type};
use crate::syntax::Node;

/// An operand type that arithmetic accepts: arithmetic or an unscoped
/// enumeration.
pub fn arithmetic_operand(cx: &Check, t: &QType) -> bool {
    t.is_arithmetic() || matches!(t.t, Type::Enum(en) if !cx.u.enums[en].scoped)
}

/// `*`, `/`, `%` on two prvalues (MUL-3, MUL-4).
pub fn operate(cx: &mut Check, op: BOp, l: Expr, r: Expr, span: crate::source::Span) -> Expr {
    let (l, r) = (rvalue(cx, l), rvalue(cx, r));
    if l.ty.is_error() || r.ty.is_error() {
        return bad(span);
    }
    if !arithmetic_operand(cx, &l.ty) || !arithmetic_operand(cx, &r.ty) {
        cx.error(span, "MUL-3", format!("invalid operands of types {} and {}", l.ty.show(), r.ty.show()));
        return bad(span);
    }
    if op == BOp::Rem && (l.ty.is_floating() || r.ty.is_floating()) {
        cx.error(span, "MUL-4", "an operand of '%' has floating-point type");
        return bad(span);
    }
    let (l, r, t) = conv::arithmetic(cx, l, r);
    Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), t, Cat::PR, span)
}

/// `mul_expr`.
pub fn mul_expr(cx: &mut Check, n: &Node) -> Expr {
    let op = BOp::from_str(operator(cx, n)).unwrap_or(BOp::Mul);
    let Some((l, r)) = operands(cx, n) else { return bad(n.span) };
    crate::foundations::evaluation_order::check::unsequenced(cx, &l, &r, n.span);
    operate(cx, op, l, r, n.span)
}
