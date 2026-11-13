//! The check part of `expressions/assignment`: `=` and the compound
//! assignments on scalars. Class assignment comes with build step 4.

use crate::expressions::check::{bad, expr, nodes};
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::{deref_ref, rvalue};
use crate::sem::check::Check;
use crate::sem::tree::{BOp, Cat, Expr, EK};
use crate::sem::types::Type;
use crate::source::Span;
use crate::syntax::Node;

/// ASN-2: the left operand is a modifiable lvalue.
fn modifiable(cx: &mut Check, l: &Expr, span: Span) -> bool {
    if l.cat != Cat::L {
        cx.error(span, "ASN-2", "an assignment to an rvalue");
        return false;
    }
    if l.ty.is_array() {
        cx.error(span, "ASN-2", "an assignment to an array");
        return false;
    }
    if l.ty.is_func() {
        cx.error(span, "ASN-2", "an assignment to a function");
        return false;
    }
    if l.ty.c {
        crate::declarations::declarators::check::modify_const(cx, span, "an assignment");
        return false;
    }
    true
}

/// `assignment_expr`.
pub fn assignment_expr(cx: &mut Check, n: &Node) -> Expr {
    let span = n.span;
    let ns = nodes(n);
    if ns.len() != 3 {
        let t = cx.unsupported(span, "a throw expression (build step 4)");
        return Expr::new(EK::Int(0), t, Cat::PR, span);
    }
    let op = cx.first_word(ns[1]);
    let l = deref_ref(expr(cx, ns[0]));
    if l.ty.is_error() {
        return bad(span);
    }
    // ATTR-6: a variable on the left of `=` is written, not read.
    if let (EK::Var(id), "=", 0) = (&l.k, op, cx.unevaluated) {
        *cx.assigned.entry(*id).or_insert(0) += 1;
    }
    let rhs = ns[2];
    if rhs.prod == "braced_init" {
        if op != "=" {
            cx.error(span, "ASN-14", "a braced list on the right of a compound assignment");
            return bad(span);
        }
        if !modifiable(cx, &l, span) {
            return bad(span);
        }
        let Some(r) = crate::declarations::initialization::list::check::scalar_list(cx, rhs, &l.ty, "ASN-14") else { return bad(span) };
        let t = l.ty.clone();
        return Expr::new(EK::Assign(Box::new(l), Box::new(r)), t, Cat::L, span);
    }
    let r = expr(cx, rhs);
    if r.ty.is_error() {
        return bad(span);
    }
    if !modifiable(cx, &l, span) {
        return bad(span);
    }
    crate::declarations::volatile::check::assignment(cx, &l.ty, span);
    if op == "=" {
        let r = match conv::implicit(cx, r, &l.ty, false) {
            Ok(r) => r,
            Err(m) => {
                let rule = conv::rule_of(&m).unwrap_or("ASN-12");
                cx.error(span, rule, m);
                return bad(span);
            }
        };
        let t = l.ty.clone();
        return Expr::new(EK::Assign(Box::new(l), Box::new(r)), t, Cat::L, span);
    }
    compound(cx, op, l, r, span)
}

/// `E1 op= E2` (ASN-5, ASN-13): `op` must be valid on the operands, and
/// the result converts back to the type of `E1`.
fn compound(cx: &mut Check, op: &str, l: Expr, r: Expr, span: Span) -> Expr {
    let bop = BOp::from_str(op.trim_end_matches('=')).unwrap_or(BOp::Add);
    let r = rvalue(cx, r);
    let lt = l.ty.clone();
    if lt.is_ptr() && matches!(bop, BOp::Add | BOp::Sub) {
        let int_ok = r.ty.is_integral() || matches!(r.ty.t, Type::Enum(en) if !cx.u.enums[en].scoped);
        if !int_ok || lt.pointee().is_some_and(|p| p.is_void() || p.is_func() || p.is_incomplete()) {
            cx.error(span, "ASN-13", format!("'{}' with operands {} and {}", op, lt.show(), r.ty.show()));
            return bad(span);
        }
        let r = conv::promote(cx, r);
        let t = lt.clone();
        return Expr::new(EK::Compound(bop, Box::new(l), Box::new(r), t.unqual()), lt, Cat::L, span);
    }
    let arith = |cx: &Check, t: &crate::sem::types::QType| t.is_arithmetic() || matches!(t.t, Type::Enum(en) if !cx.u.enums[en].scoped);
    let int = |cx: &Check, t: &crate::sem::types::QType| t.is_integral() || matches!(t.t, Type::Enum(en) if !cx.u.enums[en].scoped);
    let lv = lt.unqual();
    let ok = match bop {
        BOp::Rem | BOp::BitAnd | BOp::BitOr | BOp::BitXor | BOp::Shl | BOp::Shr => int(cx, &lv) && int(cx, &r.ty),
        _ => arith(cx, &lv) && arith(cx, &r.ty),
    };
    if !ok || lv.is_enum() {
        cx.error(span, "ASN-13", format!("'{}' with operands {} and {}", op, lt.show(), r.ty.show()));
        return bad(span);
    }
    let ct = if matches!(bop, BOp::Shl | BOp::Shr) { conv::promoted(cx, &lv) } else { conv::common_type(cx, &lv, &r.ty) };
    let r = if matches!(bop, BOp::Shl | BOp::Shr) {
        conv::promote(cx, r)
    } else if r.ty.t != ct.t {
        r.conv(crate::sem::tree::Conv::Arith, ct.clone(), Cat::PR)
    } else {
        r
    };
    Expr::new(EK::Compound(bop, Box::new(l), Box::new(r), ct), lt, Cat::L, span)
}
