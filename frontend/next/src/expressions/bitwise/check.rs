//! The check part of `expressions/bitwise`.

use crate::expressions::check::{bad, operands, operator};
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::rvalue;
use crate::sem::check::Check;
use crate::sem::tree::{BOp, Cat, Expr, EK};
use crate::sem::types::Type;
use crate::source::Span;
use crate::syntax::Node;

/// `&`, `^`, `|` (BIT-1 to BIT-3): the usual arithmetic conversions on
/// integer operands.
pub fn operate(cx: &mut Check, op: BOp, l: Expr, r: Expr, span: Span) -> Expr {
    crate::expressions::check::mixed_enumerations(cx, &l, &r, span, "BIT-8", false);
    // BIT-6: a comparison as an operand without parentheses.
    if [&l, &r].iter().any(|e| !e.paren && matches!(e.k, EK::Binary(o, ..) if o.is_comparison())) && cx.wflag(&["-Wall", "-Wparentheses"]) {
        cx.warning(span, "BIT-6", "a comparison as an operand of a bitwise operator without parentheses");
    }
    let (l, r) = (rvalue(cx, l), rvalue(cx, r));
    if l.ty.is_error() || r.ty.is_error() {
        return bad(span);
    }
    let ok = |cx: &Check, t: &crate::sem::types::QType| t.is_integral() || matches!(t.t, Type::Enum(en) if !cx.u.enums[en].scoped);
    if !ok(cx, &l.ty) || !ok(cx, &r.ty) {
        cx.error(span, "BIT-3", format!("invalid operands of types {} and {}", l.ty.show(), r.ty.show()));
        return bad(span);
    }
    let (l, r, t) = conv::arithmetic(cx, l, r);
    Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), t, Cat::PR, span)
}

/// `and_expr`, `xor_expr`, `or_expr`.
pub fn bitwise(cx: &mut Check, n: &Node) -> Expr {
    let op = BOp::from_str(operator(cx, n)).unwrap_or(BOp::BitAnd);
    let Some((l, r)) = operands(cx, n) else { return bad(n.span) };
    // OPR-6: an operand of class or enumeration type can select an
    // operator function.
    if let Some(e) = crate::classes::operators::check::binary(cx, cx.first_word(n), &l, &r, n.span) {
        return e;
    }
    // CVF-2: a class operand converts by its conversion function.
    let Some((l, r)) = crate::classes::conversion_functions::check::builtin_operands(cx, l, r) else { return bad(n.span) };
    operate(cx, op, l, r, n.span)
}
