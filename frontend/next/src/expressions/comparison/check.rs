//! The check part of `expressions/comparison`: the relational and
//! equality operators. `<=>` needs the library's ordering types, which
//! come with build step 4.

use crate::expressions::check::{bad, operands, operator};
use crate::expressions::multiplicative::check::arithmetic_operand;
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::rvalue;
use crate::sem::check::Check;
use crate::sem::tree::{BOp, Cat, Conv, Expr, EK};
use crate::sem::types::{QType, Type};
use crate::source::Span;
use crate::syntax::Node;

/// The composite pointer type of two pointer operands, if one exists
/// (CMP-3, CMP-9).
fn composite(a: &QType, b: &QType) -> Option<QType> {
    let (pa, pb) = (a.pointee()?, b.pointee()?);
    if pa.unqual().t == pb.unqual().t {
        return Some(QType::ptr_to(pa.with_cv(pb.c, pb.v)));
    }
    if pa.is_void() || pb.is_void() {
        if pa.is_func() || pb.is_func() {
            return None;
        }
        return Some(QType::ptr_to(QType::void().with_cv(pa.c || pb.c, pa.v || pb.v)));
    }
    fn strip(t: &QType) -> QType {
        match &t.t {
            Type::Ptr(p) => QType::ptr_to(strip(p)),
            _ => t.unqual(),
        }
    }
    if strip(a).t == strip(b).t {
        return Some(a.clone());
    }
    if conv::pointer_convertible(pa, pb) {
        return Some(b.clone());
    }
    if conv::pointer_convertible(pb, pa) {
        return Some(a.clone());
    }
    None
}

/// A relational or equality operator on two operands.
pub fn operate(cx: &mut Check, op: BOp, l: Expr, r: Expr, span: Span) -> Expr {
    warnings(cx, op, &l, &r, span);
    let (ln, rn) = (conv::is_null_constant(&l), conv::is_null_constant(&r));
    let (l, r) = (rvalue(cx, l), rvalue(cx, r));
    if l.ty.is_error() || r.ty.is_error() {
        return bad(span);
    }
    let eq = matches!(op, BOp::Eq | BOp::Ne);
    let mk = |l: Expr, r: Expr| Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), QType::bool(), Cat::PR, span);
    // CMP-23: a scoped enumeration compares only with its own type.
    if let (Type::Enum(a), Type::Enum(b)) = (&l.ty.t, &r.ty.t) {
        if a == b {
            return mk(l, r);
        }
    }
    for t in [&l.ty, &r.ty] {
        if let Type::Enum(en) = t.t {
            if cx.u.enums[en].scoped {
                cx.error(span, "CMP-23", format!("a scoped enumeration compared with {}", if t == &l.ty { r.ty.show() } else { l.ty.show() }));
                return bad(span);
            }
        }
    }
    if arithmetic_operand(cx, &l.ty) && arithmetic_operand(cx, &r.ty) {
        let (l, r, _) = conv::arithmetic(cx, l, r);
        return mk(l, r);
    }
    let lp = l.ty.is_ptr() || l.ty.is_nullptr();
    let rp = r.ty.is_ptr() || r.ty.is_nullptr();
    if l.ty.is_nullptr() && r.ty.is_nullptr() {
        if !eq {
            cx.error(span, "CMP-9", "a relational comparison of two values of the type of nullptr");
            return bad(span);
        }
        return mk(l, r);
    }
    if lp && rp {
        if l.ty.is_nullptr() || r.ty.is_nullptr() {
            if !eq {
                cx.error(span, "CMP-9", "a relational comparison of a pointer with nullptr");
                return bad(span);
            }
            let t = if l.ty.is_ptr() { l.ty.clone() } else { r.ty.clone() };
            let l = if l.ty.is_nullptr() { l.conv(Conv::NullPtr, t.clone(), Cat::PR) } else { l };
            let r = if r.ty.is_nullptr() { r.conv(Conv::NullPtr, t, Cat::PR) } else { r };
            return mk(l, r);
        }
        if composite(&l.ty, &r.ty).is_none() {
            cx.error(span, "CMP-9", format!("a comparison of unrelated pointers {} and {}", l.ty.show(), r.ty.show()));
            return bad(span);
        }
        return mk(l, r);
    }
    if lp && rn || rp && ln {
        if !eq {
            cx.error(span, "CMP-9", "a relational comparison of a pointer with a null pointer constant");
            return bad(span);
        }
        let t = if lp { l.ty.clone() } else { r.ty.clone() };
        let (l, r) = if lp { (l, r.conv(Conv::NullPtr, t, Cat::PR)) } else { (l.conv(Conv::NullPtr, t, Cat::PR), r) };
        return mk(l, r);
    }
    cx.error(span, "CMP-9", format!("invalid operands of types {} and {}", l.ty.show(), r.ty.show()));
    bad(span)
}

/// A constant of a signed type whose value is not negative, which keeps
/// its value as unsigned.
fn nonnegative_constant(e: &Expr) -> bool {
    match &e.k {
        EK::Int(v) => *v >= 0,
        EK::Conv(Conv::Arith | Conv::NoOp | Conv::LToR, x) if !e.paren => nonnegative_constant(x),
        _ => false,
    }
}

/// The recommended warnings of a comparison, on its operands as written.
fn warnings(cx: &mut Check, op: BOp, l: &Expr, r: &Expr, span: Span) {
    let relational = |o: BOp| matches!(o, BOp::Lt | BOp::Gt | BOp::Le | BOp::Ge);
    // LOG-9: `!` that applies only to the left operand, as in `!a == b`.
    if !l.paren && matches!(l.k, EK::Unary(crate::sem::tree::UOp::Not, _)) && !r.ty.is_bool() && cx.wflag(&["-Wall", "-Wlogical-not-parentheses"]) {
        cx.warning(span, "LOG-9", "'!' applies only to the left operand of the comparison");
    }
    // CMP-14: a chained comparison, `a < b < c`.
    if relational(op) && !l.paren && matches!(l.k, EK::Binary(o, ..) if relational(o)) && cx.wflag(&["-Wall", "-Wparentheses"]) {
        cx.warning(span, "CMP-14", "comparisons like 'a < b < c' do not compare b with both");
    }
    // CMP-16: two array operands.
    if l.ty.is_array() && r.ty.is_array() && cx.wflag(&["-Wall", "-Wextra", "-Warray-compare"]) {
        cx.warning(span, "CMP-16", "a comparison of two arrays compares their addresses");
    }
    // CMP-21: two different enumerations, or an unscoped enumeration and
    // a floating-point value.
    crate::expressions::check::mixed_enumerations(cx, l, r, span, "CMP-21", true);
    // CMP-4: a signed operand converted to an unsigned type.
    if cx.wflag(&["-Wall", "-Wsign-compare"]) {
        let kind = |cx: &Check, e: &Expr| conv::promoted(cx, &e.ty).int_kind();
        if let (Some(a), Some(b)) = (kind(cx, l), kind(cx, r)) {
            let (sa, sb) = (a.signed(), b.signed());
            if sa != sb && a != crate::sem::types::IntK::Bool && b != crate::sem::types::IntK::Bool {
                let (signed, other) = if sa { (l, b) } else { (r, a) };
                let wider = if sa { a } else { b };
                let to_unsigned = cx.u.target.int_max(other) > cx.u.target.int_max(wider) || other.rank() >= wider.rank();
                if to_unsigned && !nonnegative_constant(signed) {
                    cx.warning(span, "CMP-4", "a comparison of integers of different signedness converts the signed one");
                }
            }
        }
    }
}

/// `relational_expr`, `equality_expr`, `three_way_expr`.
pub fn comparison(cx: &mut Check, n: &Node) -> Expr {
    let op = BOp::from_str(operator(cx, n)).unwrap_or(BOp::Eq);
    if op == BOp::Spaceship && !crate::foundations::names::check::std_declares(cx, "strong_ordering") {
        // The operands decide which ordering type the result has. Each
        // needs std::strong_ordering or a weaker type, and a unit that
        // declares none has none of them.
        if !["partial_ordering", "weak_ordering"].iter().any(|t| crate::foundations::names::check::std_declares(cx, t)) {
            cx.error_cause(n.span, "CMP-25", Some("LSUP-2"), "the three-way comparison needs a declaration of its ordering type in namespace std");
            return bad(n.span);
        }
    }
    if op == BOp::Spaceship {
        let t = cx.unsupported(n.span, "the three-way comparison (its result types come from the library)");
        return Expr::new(EK::Int(0), t, Cat::PR, n.span);
    }
    let Some((l, r)) = operands(cx, n) else { return bad(n.span) };
    crate::foundations::evaluation_order::check::unsequenced(cx, &l, &r, n.span);
    operate(cx, op, l, r, n.span)
}
