//! The check part of `expressions/conditional`.

use crate::expressions::check::{bad, expr, nodes};
use crate::expressions::multiplicative::check::arithmetic_operand;
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::{deref_ref, rvalue};
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Conv, Expr, EK};
use crate::sem::types::QType;
use crate::syntax::Node;

/// `conditional_expr` (COND-1 to COND-5).
pub fn conditional_expr(cx: &mut Check, n: &Node) -> Expr {
    let ns = nodes(n);
    let span = n.span;
    if ns.len() != 3 {
        cx.error(span, "COND-14", "the second operand of the conditional operator is missing");
        return bad(span);
    }
    let c = expr(cx, ns[0]);
    let c = conv::contextual_bool(cx, c, "COND-1");
    let a = deref_ref(expr(cx, ns[1]));
    let b = deref_ref(expr(cx, ns[2]));
    if c.ty.is_error() || a.ty.is_error() || b.ty.is_error() {
        return bad(span);
    }
    let mk = |c: Expr, a: Expr, b: Expr, t: QType, cat: Cat| Expr::new(EK::Cond(Box::new(c), Box::new(a), Box::new(b)), t, cat, span);
    // COND-2: two glvalues of the same type and category give that.
    if a.cat == b.cat && a.cat != Cat::PR && a.ty == b.ty {
        let t = a.ty.clone();
        let cat = a.cat;
        return mk(c, a, b, t, cat);
    }
    // COND-3: two lvalues of one type that differ in cv-qualifiers give
    // an lvalue of the more qualified type.
    if a.cat == Cat::L && b.cat == Cat::L && a.ty.unqual().t == b.ty.unqual().t && !a.ty.is_array() {
        let t = a.ty.with_cv(b.ty.c, b.ty.v);
        let (ac, bc) = (a.ty.c != t.c || a.ty.v != t.v, b.ty.c != t.c || b.ty.v != t.v);
        if (ac && !bc) || (!ac && bc) || (!ac && !bc) {
            let a = a.conv(Conv::NoOp, t.clone(), Cat::L);
            let b = b.conv(Conv::NoOp, t.clone(), Cat::L);
            return mk(c, a, b, t, Cat::L);
        }
    }
    if a.ty.is_void() || b.ty.is_void() {
        if a.ty.is_void() && b.ty.is_void() {
            return mk(c, a, b, QType::void(), Cat::PR);
        }
        cx.error(span, "COND-13", "a void operand with a value operand");
        return bad(span);
    }
    let (an, bn) = (conv::is_null_constant(&a), conv::is_null_constant(&b));
    let (a, b) = (rvalue(cx, a), rvalue(cx, b));
    if a.ty.t == b.ty.t {
        let t = a.ty.unqual();
        return mk(c, a, b, t, Cat::PR);
    }
    if arithmetic_operand(cx, &a.ty) && arithmetic_operand(cx, &b.ty) {
        let (a, b, t) = conv::arithmetic(cx, a, b);
        return mk(c, a, b, t, Cat::PR);
    }
    // COND-3: pointers to a composite pointer type, and null pointer
    // constants.
    if (a.ty.is_nullptr() && bn) || (b.ty.is_nullptr() && an) {
        let t = QType::new(crate::sem::types::Type::NullPtr);
        let a = if a.ty.is_nullptr() { a } else { a.conv(Conv::NullPtr, t.clone(), Cat::PR) };
        let b = if b.ty.is_nullptr() { b } else { b.conv(Conv::NullPtr, t.clone(), Cat::PR) };
        return mk(c, a, b, t, Cat::PR);
    }
    if a.ty.is_ptr() && (bn || b.ty.is_nullptr()) {
        let t = a.ty.clone();
        let b = b.conv(Conv::NullPtr, t.clone(), Cat::PR);
        return mk(c, a, b, t, Cat::PR);
    }
    if b.ty.is_ptr() && (an || a.ty.is_nullptr()) {
        let t = b.ty.clone();
        let a = a.conv(Conv::NullPtr, t.clone(), Cat::PR);
        return mk(c, a, b, t, Cat::PR);
    }
    if let (Some(pa), Some(pb)) = (a.ty.pointee(), b.ty.pointee()) {
        if conv::pointer_convertible(pa, pb) {
            let t = b.ty.clone();
            let a = a.conv(Conv::PtrNoOp, t.clone(), Cat::PR);
            return mk(c, a, b, t, Cat::PR);
        }
        if conv::pointer_convertible(pb, pa) {
            let t = a.ty.clone();
            let b = b.conv(Conv::PtrNoOp, t.clone(), Cat::PR);
            return mk(c, a, b, t, Cat::PR);
        }
    }
    cx.error(span, "COND-5", format!("the operands of types {} and {} have no common type", a.ty.show(), b.ty.show()));
    bad(span)
}
