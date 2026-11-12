//! The check part of `declarations/initialization/references`: how a
//! reference binds to its initializer (REF-1 to REF-3, REF-8 to REF-11).

use crate::expressions::check::{expr, first_node, nodes};
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::deref_ref;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Conv, Expr, Init};
use crate::sem::types::{QType, Type};
use crate::syntax::Node;

/// Bind a reference of type `rt` to an expression; the result is the
/// glvalue the reference names. `rule` names a failed conversion.
pub fn bind(cx: &mut Check, e: Expr, rt: &QType, rule: &'static str) -> Option<Expr> {
    let Type::Ref(inner, rv) = &rt.t else { return None };
    let (inner, rv) = ((**inner).clone(), *rv);
    if e.ty.is_error() || inner.is_error() {
        return None;
    }
    let e = deref_ref(e);
    let e = crate::declarations::initialization::forms::check::select_overload(cx, e, rt)?;
    let span = e.span;
    let (compatible, cv_ok) = related(&e.ty, &inner);
    if compatible && e.cat == Cat::L {
        if rv && !inner.is_func() {
            cx.error(span, "REF-10", "an rvalue reference bound to an lvalue");
            return None;
        }
        if !cv_ok {
            cx.error(span, "REF-8", format!("the reference to {} drops a cv-qualifier of {}", inner.show(), e.ty.show()));
            return None;
        }
        return Some(e.conv(Conv::NoOp, inner, Cat::L));
    }
    if inner.is_func() {
        if e.ty.t == inner.t || crate::foundations::conversions::check::pointer_convertible(&e.ty, &inner) {
            return Some(e.conv(Conv::NoOp, inner, Cat::L));
        }
        cx.error(span, rule, "the reference to a function binds to another type");
        return None;
    }
    // A temporary: a const lvalue reference that is not volatile, or an
    // rvalue reference.
    if !rv && !(inner.c && !inner.v) {
        if e.cat == Cat::L {
            cx.error(span, "REF-8", format!("a reference to {} bound to an lvalue of type {}", inner.show(), e.ty.show()));
        } else {
            cx.error(span, "REF-9", "a non-const lvalue reference bound to an rvalue");
        }
        return None;
    }
    if compatible && e.cat == Cat::X {
        if !cv_ok {
            cx.error(span, "REF-10", "the reference drops a cv-qualifier");
            return None;
        }
        return Some(e.conv(Conv::NoOp, inner, Cat::L));
    }
    if compatible && e.cat == Cat::PR && !cv_ok {
        cx.error(span, "REF-10", "the reference drops a cv-qualifier");
        return None;
    }
    if rv && compatible && e.cat == Cat::L {
        cx.error(span, "REF-10", "an rvalue reference bound to an lvalue");
        return None;
    }
    let v = match conv::implicit(cx, e, &inner.unqual(), false) {
        Ok(v) => v,
        Err(m) => {
            cx.error(span, conv::rule_of(&m).unwrap_or(rule), format!("cannot bind the reference: {}", m));
            return None;
        }
    };
    Some(v.conv(Conv::Materialize, inner, Cat::L))
}

/// REF-1: the referred type is reference-related to the expression's
/// type (the same type apart from cv-qualifiers, with an array of known
/// bound related to one of unknown bound), and whether the reference
/// keeps every cv-qualifier.
fn related(e: &QType, r: &QType) -> (bool, bool) {
    match (&e.t, &r.t) {
        (Type::Array(a, n), Type::Array(b, m)) if n == m || m.is_none() => {
            let (x, y) = (a.base_elem(), b.base_elem());
            let same = a.unqual_deep() == b.unqual_deep() || (x.unqual().t == y.unqual().t && !a.is_array() && !b.is_array());
            (same, (!x.c || y.c) && (!x.v || y.v))
        }
        _ => (e.unqual().t == r.unqual().t, (!e.c || r.c) && (!e.v || r.v)),
    }
}

/// The initializer of a reference variable (REF-11).
pub fn initializer(cx: &mut Check, ty: &QType, init: &Node) -> Option<Init> {
    let ns = nodes(init);
    let words = cx.words(init);
    let one = if let Some(b) = ns.first().filter(|c| c.prod == "braced_init") {
        let its: Vec<&Node> = b.children("init_clause").collect();
        if ty.nonref().is_array() {
            cx.unsupported(init.span, "a braced list bound to a reference to an array");
            return None;
        }
        if its.len() != 1 {
            cx.error(init.span, "REF-11", "a braced list for a reference must hold one value");
            return None;
        }
        let c = first_node(its[0])?;
        let e = expr(cx, c);
        let inner = ty.nonref();
        let rv = crate::foundations::values::check::rvalue(cx, e.clone());
        if conv::is_narrowing(cx, &rv, &inner) && e.ty.unqual().t != inner.unqual().t {
            cx.error(init.span, "REF-11", "a narrowing conversion in the braces of a reference");
            return None;
        }
        return bind(cx, e, ty, "INIT-7").map(Init::Ref);
    } else if words.first() == Some(&"(") {
        let args: Vec<&Node> = ns.first().map(|l| l.children("argument").collect()).unwrap_or_default();
        if args.len() != 1 {
            cx.error(init.span, "REF-11", "a parenthesized initializer for a reference must hold one expression");
            return None;
        }
        args[0]
    } else {
        ns.first()?
    };
    let e = expr(cx, one);
    bind(cx, e, ty, "INIT-7").map(Init::Ref)
}
