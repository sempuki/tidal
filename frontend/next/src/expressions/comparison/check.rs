//! The check part of `expressions/comparison`: the relational and
//! equality operators. `<=>` needs the library's ordering types, which
//! come with build step 4.

use crate::expressions::check::{bad, first_node, operands, operator};
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
    if l.ty.is_member_ptr() || r.ty.is_member_ptr() {
        return crate::expressions::member_pointers::check::compare(cx, op, l, r, (ln, rn), span);
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
        // CMP-3: a pointer to a derived class converts to a pointer to
        // the base.
        if let (Some(a), Some(b)) = (l.ty.pointee().and_then(|p| p.class()), r.ty.pointee().and_then(|p| p.class())) {
            let (lp, rp) = (l.ty.pointee().unwrap().clone(), r.ty.pointee().unwrap().clone());
            if crate::classes::derived::check::is_base(cx, a, b) {
                let t = QType::ptr_to(rp.with_cv(lp.c || rp.c, lp.v || rp.v));
                let Some(l) = crate::classes::derived::check::ptr_to_base(cx, l, t.clone(), span) else { return bad(span) };
                let r = r.conv(Conv::PtrNoOp, t, Cat::PR);
                return mk(l, r);
            }
            if crate::classes::derived::check::is_base(cx, b, a) {
                let t = QType::ptr_to(lp.with_cv(lp.c || rp.c, lp.v || rp.v));
                let Some(r) = crate::classes::derived::check::ptr_to_base(cx, r, t.clone(), span) else { return bad(span) };
                let l = l.conv(Conv::PtrNoOp, t, Cat::PR);
                return mk(l, r);
            }
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
    // OPR-6: a three-way comparison of class or enumeration operands can
    // select an operator function.
    if op == BOp::Spaceship && cx.u.entities.iter().any(|e| &*e.name == "operator<=>") {
        let Some((l, r)) = operands(cx, n) else { return bad(n.span) };
        if let Some(e) = crate::classes::operators::check::binary(cx, "<=>", &l, &r, n.span) {
            return e;
        }
    }
    if op == BOp::Spaceship {
        let Some((l, r)) = operands(cx, n) else { return bad(n.span) };
        if let Some(e) = crate::classes::operators::check::binary(cx, "<=>", &l, &r, n.span) {
            return e;
        }
        let Some((l, r)) = crate::classes::conversion_functions::check::builtin_operands(cx, l, r) else { return bad(n.span) };
        crate::foundations::evaluation_order::check::unsequenced(cx, &l, &r, n.span);
        return three_way(cx, l, r, n.span);
    }
    let Some((l, r)) = operands(cx, n) else { return bad(n.span) };
    // OPR-6: an operand of class or enumeration type can select an
    // operator function.
    if let Some(e) = crate::classes::operators::check::binary(cx, cx.first_word(n), &l, &r, n.span) {
        // CMP-14: a chained comparison, also of class operands.
        let relational = matches!(op, BOp::Lt | BOp::Gt | BOp::Le | BOp::Ge);
        if relational && l.ty.is_bool() && first_node(n).is_some_and(|x| x.prod == "relational_expr") && cx.wflag(&["-Wall", "-Wparentheses"]) {
            cx.warning(n.span, "CMP-14", "comparisons like 'a < b < c' do not compare b with both");
        }
        return e;
    }
    // CVF-2: a class operand converts by its conversion function.
    let Some((l, r)) = crate::classes::conversion_functions::check::builtin_operands(cx, l, r) else { return bad(n.span) };
    crate::foundations::evaluation_order::check::unsequenced(cx, &l, &r, n.span);
    operate(cx, op, l, r, n.span)
}

/// The ordering class `std::<name>`, if the unit declares it (LSUP-2).
pub fn ordering_class(cx: &Check, name: &str) -> Option<crate::sem::types::ClassId> {
    let std = crate::foundations::names::check::declared_here(cx, cx.u.root, "std").into_iter().find_map(|e| match cx.u.entities[e].kind {
        crate::sem::EntKind::Namespace(sc) => Some(sc),
        _ => None,
    })?;
    crate::foundations::names::check::declared_here(cx, std, name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        crate::sem::EntKind::Class(c) if cx.u.classes[c].complete => Some(c),
        _ => None,
    })
}

/// The static data members of the ordering class `c` that a result
/// copies, in the order of `EK::ThreeWay` (LSUP-2), or None when one is
/// missing.
pub fn ordering_members(cx: &Check, c: crate::sem::types::ClassId, partial: bool) -> Option<Vec<crate::sem::EntityId>> {
    let names: &[&str] = if partial { &["less", "equivalent", "greater", "unordered"] } else { &["less", "equal", "greater"] };
    let scope = cx.u.classes[c].scope;
    names.iter().map(|n| crate::foundations::names::check::declared_here(cx, scope, n).into_iter().find(|&e| cx.u.var(e).is_some())).collect()
}

/// A built-in `l <=> r` (CMP-12, CMP-13, CMP-17, CMP-18, CMP-24, CMP-25,
/// CMP-27).
pub fn three_way(cx: &mut Check, l: Expr, r: Expr, span: Span) -> Expr {
    if l.ty.is_array() && r.ty.is_array() {
        cx.error(span, "CMP-24", "a three-way comparison of two arrays");
        return bad(span);
    }
    let (ln, rn) = (conv::is_null_constant(&l), conv::is_null_constant(&r));
    let (l, r) = (rvalue(cx, l), rvalue(cx, r));
    if l.ty.is_error() || r.ty.is_error() {
        return bad(span);
    }
    let fnptr = |t: &QType| t.pointee().is_some_and(|p| p.is_func());
    if l.ty.is_member_ptr() || r.ty.is_member_ptr() || (l.ty.is_nullptr() && r.ty.is_nullptr()) || fnptr(&l.ty) || fnptr(&r.ty) {
        cx.error(span, "CMP-25", format!("a three-way comparison of {} and {}", l.ty.show(), r.ty.show()));
        return bad(span);
    }
    // CMP-13: a bool with another type, and a pointer with the literal 0.
    if l.ty.is_bool() != r.ty.is_bool() {
        cx.error(span, "CMP-13", "a three-way comparison of a bool and another type");
        return bad(span);
    }
    if (l.ty.is_ptr() && rn && !r.ty.is_nullptr()) || (r.ty.is_ptr() && ln && !l.ty.is_nullptr()) {
        cx.error(span, "CMP-13", "a three-way comparison of a pointer and the literal 0");
        return bad(span);
    }
    // CMP-18: two unscoped enumerations, or one and a floating-point value.
    if let (Type::Enum(a), Type::Enum(b)) = (&l.ty.t, &r.ty.t) {
        if a != b && !cx.u.enums[*a].scoped && !cx.u.enums[*b].scoped {
            cx.error(span, "CMP-18", "a three-way comparison of two different enumeration types");
            return bad(span);
        }
    }
    if (l.ty.is_enum() && r.ty.is_floating()) || (r.ty.is_enum() && l.ty.is_floating()) {
        cx.error(span, "CMP-18", "a three-way comparison of an enumeration and a floating-point value");
        return bad(span);
    }
    // CMP-13: no narrowing conversion, other than an integer to a
    // floating-point type.
    if arithmetic_operand(cx, &l.ty) && arithmetic_operand(cx, &r.ty) && !l.ty.is_enum() && !r.ty.is_enum() {
        let (cl, _, t) = conv::arithmetic(cx, l.clone(), r.clone());
        let _ = cl;
        for e in [&l, &r] {
            let to_float = t.is_floating() && !e.ty.is_floating();
            if !to_float && conv::is_narrowing(cx, e, &t) {
                cx.error(span, "CMP-13", format!("the three-way comparison converts {} to {}, which narrows", e.ty.show(), t.show()));
                return bad(span);
            }
        }
    }
    let partial = l.ty.is_floating() || r.ty.is_floating();
    let name = if partial { "partial_ordering" } else { "strong_ordering" };
    let Some(c) = ordering_class(cx, name) else {
        cx.error_cause(span, "CMP-25", Some("LSUP-2"), format!("the three-way comparison needs a declaration of std::{}", name));
        return bad(span);
    };
    let Some(members) = ordering_members(cx, c, partial) else {
        cx.error(span, "LSUP-2", format!("std::{} does not declare the static data members of its values", name));
        return bad(span);
    };
    // The operands convert as for `<` (CMP-3 and the usual arithmetic
    // conversions).
    let cmp = operate(cx, BOp::Lt, l, r, span);
    let EK::Binary(_, a, b) = cmp.k else { return bad(span) };
    Expr::new(EK::ThreeWay(a, b, members), QType::new(Type::Class(c)), Cat::PR, span)
}
