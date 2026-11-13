//! The check part of `expressions/shift`.

use crate::expressions::check::{bad, operands, operator};
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::rvalue;
use crate::sem::check::Check;
use crate::sem::tree::{BOp, Cat, Expr, EK};
use crate::sem::types::Type;
use crate::source::Span;
use crate::syntax::Node;

/// `<<` and `>>` (SHF-1 to SHF-3): each operand is promoted, and the
/// result has the type of the promoted left operand.
pub fn operate(cx: &mut Check, op: BOp, l: Expr, r: Expr, span: Span) -> Expr {
    let (l, r) = (rvalue(cx, l), rvalue(cx, r));
    if l.ty.is_error() || r.ty.is_error() {
        return bad(span);
    }
    let ok = |cx: &Check, t: &crate::sem::types::QType| t.is_integral() || matches!(t.t, Type::Enum(en) if !cx.u.enums[en].scoped);
    if !ok(cx, &l.ty) || !ok(cx, &r.ty) {
        cx.error(span, "SHF-3", format!("invalid operands of types {} and {}", l.ty.show(), r.ty.show()));
        return bad(span);
    }
    let l = conv::promote(cx, l);
    let r = conv::promote(cx, r);
    // SHF-9: a constant count that is negative or not less than the
    // width of the promoted left operand.
    if let (Some(crate::machine::Val::Int(c)), Some(k)) = (crate::foundations::constant_evaluation::check::try_value(cx, &r), l.ty.int_kind()) {
        if c < 0 || c >= cx.u.target.int_bits(k) as i128 {
            cx.warning(span, "SHF-9", format!("the shift count {} is negative or not less than the width of {}", c, l.ty.show()));
        }
    }
    let t = l.ty.clone();
    Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), t, Cat::PR, span)
}

/// `shift_expr`.
pub fn shift_expr(cx: &mut Check, n: &Node) -> Expr {
    let op = BOp::from_str(operator(cx, n)).unwrap_or(BOp::Shl);
    let Some((l, r)) = operands(cx, n) else { return bad(n.span) };
    operate(cx, op, l, r, n.span)
}
