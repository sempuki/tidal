//! The check part of `expressions/conditional`.

use crate::expressions::check::{bad, expr, nodes};
use crate::expressions::multiplicative::check::arithmetic_operand;
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::{deref_ref, rvalue};
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Conv, Expr, EK};
use crate::sem::types::QType;
use crate::syntax::Node;

/// `conditional_expr` (COND-1 to COND-5, COND-11, COND-16).
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
    combine(cx, c, a, b, span, 0)
}

/// What an operand converts to for the other operand (COND-16).
enum Target {
    /// A glvalue of the type, of the category, by a binding to the
    /// operand itself or to the result of the conversion function.
    Glvalue(QType, Cat, Option<crate::sem::EntityId>),
    /// A prvalue of the type, by copy-initialization.
    Prvalue(QType),
}

/// Whether the cv-qualifiers of `t` include those of `x`.
fn covers(t: &QType, x: &QType) -> bool {
    (t.c || !x.c) && (t.v || !x.v)
}

/// The class `d` is the class `b` or derived from it.
fn same_or_derived(cx: &Check, d: crate::sem::types::ClassId, b: crate::sem::types::ClassId) -> bool {
    d == b || crate::classes::derived::check::is_base(cx, d, b)
}

/// COND-16: what the operand `x` converts to for the operand `y`, if
/// anything. Err when the conversion sequence of step 3 is ambiguous.
fn target(cx: &Check, x: &Expr, y: &Expr) -> Result<Option<Target>, ()> {
    let t = y.ty.clone();
    // Steps 1 and 2: a reference binds directly to `x`, or to the
    // glvalue that a conversion function of `x` returns.
    for (cat, rv) in [(Cat::L, false), (Cat::X, true)] {
        if y.cat != cat || (cat == Cat::L && x.cat != Cat::L) {
            continue;
        }
        if let (Some(cx_), Some(ct)) = (x.ty.class(), t.class()) {
            if x.cat == cat && same_or_derived(cx, cx_, ct) && covers(&t, &x.ty) {
                return Ok(Some(Target::Glvalue(t, cat, None)));
            }
        }
        if let Some(c) = crate::classes::conversion_functions::check::converting_class(cx, x) {
            let fits = |f: &crate::sem::Func| {
                let r = &f.ty.ret;
                let binds = matches!(&r.t, crate::sem::types::Type::Ref(_, v) if *v == rv);
                let u = r.nonref();
                let compatible = u.unqual().t == t.unqual().t || matches!((u.class(), t.class()), (Some(d), Some(b)) if same_or_derived(cx, d, b));
                !f.explicit && binds && compatible && covers(&t, &u) && (!x.ty.c || f.obj_cv.0) && (!x.ty.v || f.obj_cv.1)
            };
            let fs: Vec<crate::sem::EntityId> = crate::classes::conversion_functions::check::functions(&cx.u, c).into_iter().filter(|&f| cx.u.func(f).is_some_and(fits)).collect();
            if let [f] = fs[..] {
                return Ok(Some(Target::Glvalue(t, cat, Some(f))));
            }
        }
    }
    // Step 3: a prvalue of the target.
    if let (Some(cx_), Some(ct)) = (x.ty.class(), t.class()) {
        if cx_ == ct || crate::classes::derived::check::is_base(cx, cx_, ct) {
            return Ok(covers(&t, &x.ty).then(|| Target::Prvalue(t.unqual().with_cv(t.c, t.v))));
        }
        if crate::classes::derived::check::is_base(cx, ct, cx_) {
            return Ok(None);
        }
    }
    let to = if t.is_class() { t.clone() } else { crate::declarations::auto::check::decay(&t).unqual() };
    let xv = crate::foundations::values::check::deref_ref(x.clone());
    // A conversion function of `x`; access and deletion do not count
    // here (Q-0378).
    if crate::classes::conversion_functions::check::converting_class(cx, &xv).is_some() {
        match crate::classes::conversion_functions::check::user_sequence(cx, &xv, &to) {
            Some(i) if i.user.is_none() => return Err(()),
            Some(_) => return Ok(Some(Target::Prvalue(to))),
            None => {}
        }
    }
    // A converting constructor of the target class.
    if let Some(ct) = to.class() {
        let ok = crate::classes::constructors::check::ctors(cx, ct).into_iter().any(|f| {
            cx.u.func(f).is_some_and(|g| !g.explicit && !g.ty.params.is_empty() && (1..g.ty.params.len()).all(|k| g.defaults.get(k).is_some_and(|d| d.is_some())) && crate::overloading::conversion_sequences::check::sequence(cx, &xv, &g.ty.params[0]).is_some_and(|i| i.user.is_none()))
        });
        return Ok(ok.then_some(Target::Prvalue(to)));
    }
    if x.ty.is_class() {
        return Ok(None);
    }
    Ok(crate::overloading::conversion_sequences::check::sequence(cx, &xv, &to).map(|_| Target::Prvalue(to)))
}

/// The operand `x` converted to `t` (COND-16).
fn apply(cx: &mut Check, x: Expr, t: Target, span: crate::source::Span) -> Option<Expr> {
    match t {
        Target::Glvalue(ty, cat, None) => {
            let x = match (x.ty.class(), ty.class()) {
                (Some(d), Some(b)) if d != b => crate::classes::derived::check::to_base(cx, x, b, span)?,
                _ => x,
            };
            Some(x.conv(Conv::NoOp, ty, cat))
        }
        Target::Glvalue(ty, cat, Some(f)) => {
            let r = deref_ref(crate::classes::conversion_functions::check::call(cx, f, x, span));
            if r.ty.is_error() {
                return None;
            }
            let r = match (r.ty.class(), ty.class()) {
                (Some(d), Some(b)) if d != b => crate::classes::derived::check::to_base(cx, r, b, span)?,
                _ => r,
            };
            Some(r.conv(Conv::NoOp, ty, cat))
        }
        // A class prvalue keeps the target's cv-qualifiers (COND-16).
        Target::Prvalue(ty) => crate::declarations::initialization::forms::check::copy_init(cx, x, &ty, "COND-16").map(|mut e| {
            if ty.is_class() {
                e.ty = ty;
            }
            e
        }),
    }
}

/// The result of `c ? a : b` from its evaluated operands (COND-2 to
/// COND-5, COND-11, COND-16). `depth` counts the conversions of COND-16
/// and COND-11 so far.
fn combine(cx: &mut Check, c: Expr, a: Expr, b: Expr, span: crate::source::Span, depth: u8) -> Expr {
    let mk = |c: Expr, a: Expr, b: Expr, t: QType, cat: Cat| Expr::new(EK::Cond(Box::new(c), Box::new(a), Box::new(b)), t, cat, span);
    // COND-2: two glvalues of the same type and category give that.
    if a.cat == b.cat && a.cat != Cat::PR && a.ty == b.ty {
        let t = a.ty.clone();
        let cat = a.cat;
        return mk(c, a, b, t, cat);
    }
    // COND-3: two lvalues, or two xvalues, of one type that differ in
    // cv-qualifiers give a glvalue of that category and the more
    // qualified type.
    if a.cat == b.cat && a.cat != Cat::PR && a.ty.unqual().t == b.ty.unqual().t && !a.ty.is_array() {
        let cat = a.cat;
        let t = a.ty.with_cv(b.ty.c, b.ty.v);
        let (ac, bc) = (a.ty.c != t.c || a.ty.v != t.v, b.ty.c != t.c || b.ty.v != t.v);
        if (ac && !bc) || (!ac && bc) || (!ac && !bc) {
            let a = a.conv(Conv::NoOp, t.clone(), cat);
            let b = b.conv(Conv::NoOp, t.clone(), cat);
            return mk(c, a, b, t, cat);
        }
    }
    // COND-3: glvalues of a derived class and its base give a glvalue of
    // the base.
    if a.cat != Cat::PR && a.cat == b.cat {
        if let (Some(x), Some(y)) = (a.ty.class(), b.ty.class()) {
            let flip = crate::classes::derived::check::is_base(cx, y, x);
            if flip || crate::classes::derived::check::is_base(cx, x, y) {
                let (d, base) = if flip { (b.clone(), a.clone()) } else { (a.clone(), b.clone()) };
                let bc = base.ty.class().unwrap();
                let Some(d) = crate::classes::derived::check::to_base(cx, d, bc, span) else { return bad(span) };
                let t = base.ty.with_cv(d.ty.c || base.ty.c, d.ty.v || base.ty.v);
                let cat = a.cat;
                let d = d.conv(Conv::NoOp, t.clone(), cat);
                let base = base.conv(Conv::NoOp, t.clone(), cat);
                let (a, b) = if flip { (base, d) } else { (d, base) };
                return mk(c, a, b, t, cat);
            }
        }
    }
    // COND-16: operands of different types, one of class type: each is
    // tried against the other.
    if depth < 2 && a.ty != b.ty && (a.ty.is_class() || b.ty.is_class()) && !a.ty.is_void() && !b.ty.is_void() {
        let (ta, tb) = (target(cx, &a, &b), target(cx, &b, &a));
        if ta.is_err() || tb.is_err() {
            cx.error(span, "COND-5", format!("the conversion of an operand of type {} or {} to the other type is ambiguous", a.ty.show(), b.ty.show()));
            return bad(span);
        }
        match (ta.unwrap(), tb.unwrap()) {
            (Some(_), Some(_)) => {
                cx.error(span, "COND-5", format!("the operands of types {} and {} each convert to the other's type", a.ty.show(), b.ty.show()));
                return bad(span);
            }
            (Some(t), None) => {
                let Some(a) = apply(cx, a, t, span) else { return bad(span) };
                return combine(cx, c, a, b, span, depth + 1);
            }
            (None, Some(t)) => {
                let Some(b) = apply(cx, b, t, span) else { return bad(span) };
                return combine(cx, c, a, b, span, depth + 1);
            }
            (None, None) => {
                // COND-11: the built-in candidates, which the conversion
                // functions of the class operands reach. Each gives a
                // prvalue.
                let builtin = |cx: &Check, e: &Expr| -> usize {
                    let Some(k) = crate::classes::conversion_functions::check::converting_class(cx, e) else { return 0 };
                    let mut tys: Vec<QType> = Vec::new();
                    for f in crate::classes::conversion_functions::check::functions(&cx.u, k) {
                        let Some(g) = cx.u.func(f) else { continue };
                        let r = crate::declarations::auto::check::decay(&g.ty.ret.nonref()).unqual();
                        if !g.explicit && (r.is_arithmetic() || r.is_enum() || r.is_ptr() || r.is_member_ptr()) && !tys.contains(&r) {
                            tys.push(r);
                        }
                    }
                    tys.len()
                };
                if builtin(cx, &a) > 1 || builtin(cx, &b) > 1 {
                    cx.error(span, "COND-5", format!("the operands of types {} and {} have no best built-in candidate", a.ty.show(), b.ty.show()));
                    return bad(span);
                }
                let (Some(a2), Some(b2)) = (crate::classes::conversion_functions::check::builtin_operand(cx, a), crate::classes::conversion_functions::check::builtin_operand(cx, b)) else { return bad(span) };
                if a2.ty.is_class() || b2.ty.is_class() || a2.ty.is_error() || b2.ty.is_error() {
                    if !a2.ty.is_error() && !b2.ty.is_error() {
                        cx.error(span, "COND-5", format!("the operands of types {} and {} have no common type", a2.ty.show(), b2.ty.show()));
                    }
                    return bad(span);
                }
                let (a2, b2) = (rvalue(cx, a2), rvalue(cx, b2));
                // COND-11: the built-in candidate for arithmetic operands
                // takes them after the usual arithmetic conversions.
                if arithmetic_operand(cx, &a2.ty) && arithmetic_operand(cx, &b2.ty) {
                    let (a3, b3, t) = conv::arithmetic(cx, a2, b2);
                    return mk(c, a3, b3, t, Cat::PR);
                }
                return combine(cx, c, a2, b2, span, 2);
            }
        }
    }
    if a.ty.is_void() || b.ty.is_void() {
        if a.ty.is_void() && b.ty.is_void() {
            return mk(c, a, b, QType::void(), Cat::PR);
        }
        // COND-4: with a throw expression as one operand, the result has
        // the type and value category of the other.
        let throws = |e: &Expr| matches!(e.k, EK::Throw(_));
        if throws(&a) || throws(&b) {
            let other = if throws(&a) { &b } else { &a };
            let (ty, cat) = (other.ty.clone(), other.cat);
            return mk(c, a, b, ty, cat);
        }
        cx.error(span, "COND-13", "a void operand with a value operand");
        return bad(span);
    }
    let (an, bn) = (conv::is_null_constant(&a), conv::is_null_constant(&b));
    let cv = (a.ty.nonref().c || b.ty.nonref().c, a.ty.nonref().v || b.ty.nonref().v);
    let (a, b) = (rvalue(cx, a), rvalue(cx, b));
    if a.ty.t == b.ty.t {
        // COND-3: a class prvalue keeps its cv-qualifiers.
        let t = if a.ty.is_class() { a.ty.with_cv(cv.0, cv.1) } else { a.ty.unqual() };
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
        if let (Some(x), Some(y)) = (pa.class(), pb.class()) {
            let flip = crate::classes::derived::check::is_base(cx, y, x);
            if flip || crate::classes::derived::check::is_base(cx, x, y) {
                let t = QType::ptr_to(if flip { pa.clone() } else { pb.clone() }.with_cv(pa.c || pb.c, pa.v || pb.v));
                let (a, b) = if flip {
                    let Some(b) = crate::classes::derived::check::ptr_to_base(cx, b, t.clone(), span) else { return bad(span) };
                    (a.conv(Conv::PtrNoOp, t.clone(), Cat::PR), b)
                } else {
                    let Some(a) = crate::classes::derived::check::ptr_to_base(cx, a, t.clone(), span) else { return bad(span) };
                    (a, b.conv(Conv::PtrNoOp, t.clone(), Cat::PR))
                };
                return mk(c, a, b, t, Cat::PR);
            }
        }
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
        // A pointer to an object type and a pointer to void: a pointer to
        // void with the cv-qualifiers of both.
        if (pa.is_void() || pb.is_void()) && !pa.is_func() && !pb.is_func() {
            let t = QType::ptr_to(QType::void().with_cv(pa.c || pb.c, pa.v || pb.v));
            if let (Ok(a), Ok(b)) = (conv::implicit(cx, a.clone(), &t, false), conv::implicit(cx, b.clone(), &t, false)) {
                return mk(c, a, b, t, Cat::PR);
            }
        }
    }
    // MPTR-4: a pointer to member and a null pointer constant or another
    // pointer to member that converts to its type.
    for flip in [false, true] {
        let (x, y, yn) = if flip { (&b, &a, an) } else { (&a, &b, bn) };
        if x.ty.is_member_ptr() {
            let t = x.ty.unqual();
            if let Some(Ok(y2)) = crate::expressions::member_pointers::check::implicit(cx, y.clone(), yn || y.ty.is_nullptr(), &t) {
                let (a, b) = if flip { (y2, b) } else { (a, y2) };
                return mk(c, a, b, t, Cat::PR);
            }
        }
    }
    cx.error(span, "COND-5", format!("the operands of types {} and {} have no common type", a.ty.show(), b.ty.show()));
    bad(span)
}
