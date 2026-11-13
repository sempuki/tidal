//! The check part of `expressions/additive`.

use crate::expressions::check::{bad, operands, operator};
use crate::expressions::multiplicative::check::arithmetic_operand;
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::rvalue;
use crate::sem::check::Check;
use crate::sem::tree::{BOp, Cat, Expr, EK};
use crate::sem::types::{QType, Type};
use crate::source::Span;
use crate::syntax::Node;

fn integral_operand(cx: &Check, t: &QType) -> bool {
    t.is_integral() || matches!(t.t, Type::Enum(en) if !cx.u.enums[en].scoped)
}

/// A pointer to a complete object type, which arithmetic can move.
fn object_pointer(cx: &Check, t: &QType) -> bool {
    t.pointee().is_some_and(|p| !p.is_void() && !p.is_func() && !cx.u.is_incomplete(p))
}

/// `+` and `-` on two operands (ADD-1 to ADD-3).
pub fn operate(cx: &mut Check, op: BOp, l: Expr, r: Expr, span: Span) -> Expr {
    crate::expressions::check::mixed_enumerations(cx, &l, &r, span, "ADD-16", true);
    let (l, r) = (rvalue(cx, l), rvalue(cx, r));
    if l.ty.is_error() || r.ty.is_error() {
        return bad(span);
    }
    let sub = op == BOp::Sub;
    if l.ty.is_ptr() && integral_operand(cx, &r.ty) {
        if !object_pointer(cx, &l.ty) {
            cx.error(span, "ADD-3", "arithmetic on a pointer to an incomplete or function type");
            return bad(span);
        }
        let t = l.ty.clone();
        let r = conv::promote(cx, r);
        return Expr::new(EK::PtrAdd(Box::new(l), Box::new(r), sub), t, Cat::PR, span);
    }
    if !sub && r.ty.is_ptr() && integral_operand(cx, &l.ty) {
        if !object_pointer(cx, &r.ty) {
            cx.error(span, "ADD-3", "arithmetic on a pointer to an incomplete or function type");
            return bad(span);
        }
        let t = r.ty.clone();
        let l = conv::promote(cx, l);
        return Expr::new(EK::PtrAdd(Box::new(r), Box::new(l), false), t, Cat::PR, span);
    }
    if sub && l.ty.is_ptr() && r.ty.is_ptr() {
        let (a, b) = (l.ty.pointee().unwrap(), r.ty.pointee().unwrap());
        if a.t != b.t || !object_pointer(cx, &l.ty) {
            cx.error(span, "ADD-3", format!("the difference of {} and {}", l.ty.show(), r.ty.show()));
            return bad(span);
        }
        let t = QType::new(Type::Int(cx.u.target.ptrdiff_t()));
        return Expr::new(EK::PtrDiff(Box::new(l), Box::new(r)), t, Cat::PR, span);
    }
    if !arithmetic_operand(cx, &l.ty) || !arithmetic_operand(cx, &r.ty) {
        cx.error(span, "ADD-3", format!("invalid operands of types {} and {}", l.ty.show(), r.ty.show()));
        return bad(span);
    }
    let (l, r, t) = conv::arithmetic(cx, l, r);
    Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), t, Cat::PR, span)
}

/// `add_expr`.
pub fn add_expr(cx: &mut Check, n: &Node) -> Expr {
    let op = BOp::from_str(operator(cx, n)).unwrap_or(BOp::Add);
    let Some((l, r)) = operands(cx, n) else { return bad(n.span) };
    // OPR-6: an operand of class or enumeration type can select an
    // operator function.
    if let Some(e) = crate::classes::operators::check::binary(cx, cx.first_word(n), &l, &r, n.span) {
        return e;
    }
    crate::foundations::evaluation_order::check::unsequenced(cx, &l, &r, n.span);
    operate(cx, op, l, r, n.span)
}
