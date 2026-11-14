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
    let e = crate::declarations::initialization::forms::check::select_overload(cx, e, rt)?;
    if e.ty.is_error() || inner.is_error() {
        return None;
    }
    let e = deref_ref(e);
    let span = e.span;
    // CVF-2: a class object converts by a conversion function, and the
    // reference binds the result.
    if crate::classes::conversion_functions::check::converting_class(cx, &e).is_some() && !matches!((e.ty.class(), inner.class()), (Some(d), Some(b)) if d == b || crate::classes::derived::check::is_base(cx, d, b)) {
        if inner.is_func() || inner.is_class() {
            cx.unsupported(span, "a reference to a function or a class bound by a conversion function");
            return None;
        }
        match crate::classes::conversion_functions::check::convert(cx, &e, rt, false) {
            Some(Ok(x)) if x.ty.is_error() => return None,
            Some(Ok(x)) => return bind(cx, x, rt, rule),
            Some(Err(m)) if m.contains("(CVF-11)") => {
                cx.unsupported(span, "a reference bound by an explicit conversion function");
                return None;
            }
            Some(Err(m)) => {
                cx.error_cause(span, crate::foundations::conversions::check::rule_of(&m).unwrap_or(rule), None, m);
                return None;
            }
            None => {}
        }
    }
    // BF-4: a reference binds no bit-field. A const reference that is not
    // volatile binds a temporary that holds a copy of its value (REF-2).
    let e = if crate::classes::unions::check::width_of(&cx.u, &e).is_some() && e.cat != Cat::PR {
        if !inner.c && !rv {
            cx.error(span, "BF-4", "a reference that is not const binds a bit-field");
            return None;
        }
        if inner.v {
            cx.error_cause(span, rule, Some("VOL-10"), "a volatile reference binds a bit-field");
            return None;
        }
        crate::foundations::values::check::rvalue(cx, e)
    } else {
        e
    };
    // DER-5: a reference to a base binds the base subobject.
    let e = match (e.ty.class(), inner.class()) {
        (Some(d), Some(b)) if crate::classes::derived::check::is_base(cx, d, b) => {
            let e = if e.cat == Cat::PR {
                let t = e.ty.clone();
                e.conv(Conv::Materialize, t, Cat::X)
            } else {
                e
            };
            crate::classes::derived::check::to_base(cx, e, b, span)?
        }
        (Some(b), Some(d)) if crate::classes::derived::check::is_base(cx, d, b) => {
            cx.error_cause(span, rule, Some("DER-10"), format!("a reference to {} does not bind an object of its base {}", inner.show(), e.ty.show()));
            return None;
        }
        _ => e,
    };
    let (compatible, cv_ok) = related(&e.ty, &inner);
    if compatible && e.cat == Cat::L {
        if rv && !inner.is_func() {
            // VC-14: the name of an rvalue reference is an lvalue.
            let named_rref = matches!(&e.k, crate::sem::tree::EK::Var(id) if cx.u.var(*id).is_some_and(|v| matches!(v.ty.t, Type::Ref(_, true))));
            cx.error_cause(span, "REF-10", named_rref.then_some("VC-14"), "an rvalue reference bound to an lvalue");
            return None;
        }
        if !cv_ok {
            let vol = (e.ty.v && !inner.v).then_some("VOL-10");
            cx.error_cause(span, "REF-8", vol, format!("the reference to {} drops a cv-qualifier of {}", inner.show(), e.ty.show()));
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
        // VOL-10: a reference to volatile binds no temporary.
        let vol = (inner.v || e.ty.v).then_some("VOL-10");
        if e.cat == Cat::L {
            cx.error_cause(span, "REF-8", vol, format!("a reference to {} bound to an lvalue of type {}", inner.show(), e.ty.show()));
        } else {
            cx.error_cause(span, "REF-9", vol, "a non-const lvalue reference bound to an rvalue");
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
    if inner.class().is_some_and(|c| crate::classes::constructors::check::has_ctors(cx, c)) {
        let v = crate::declarations::initialization::forms::check::copy_init(cx, e, &inner.unqual(), rule)?;
        return Some(v.conv(Conv::Materialize, inner, Cat::L));
    }
    let v = match conv::implicit(cx, e, &inner.unqual(), false) {
        Ok(v) => v,
        Err(m) => {
            cx.error_cause(span, conv::rule_of(&m).unwrap_or(rule), conv::cause_of(&m), format!("cannot bind the reference: {}", m));
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
        // A pointer type is related to a pointer type that a
        // qualification conversion of its inner levels reaches (REF-1).
        (Type::Ptr(_), Type::Ptr(_)) => (conv::pointer_convertible(&e.unqual(), r), (!e.c || r.c) && (!e.v || r.v)),
        _ => (e.unqual().t == r.unqual().t, (!e.c || r.c) && (!e.v || r.v)),
    }
}

/// A braced list for a reference to an array. A list of one array
/// that the reference can name binds directly (REF-6). Any other list
/// makes a temporary array of the referenced type (REF-14), with one
/// element for each value where the bound is unknown. REF-11 rejects
/// more values than the bound, and an empty list for an unknown bound.
fn array_reference(cx: &mut Check, ty: &QType, b: &Node) -> Option<Expr> {
    use crate::declarations::initialization::list::check as list;
    let Type::Ref(inner, rv) = &ty.t else { return None };
    let (inner, rv) = ((**inner).clone(), *rv);
    let Type::Array(el, bound) = inner.t.clone() else { return None };
    let its = list::items(b);
    let with_bound = |n: u64| QType { t: Type::Array(el.clone(), Some(n)), c: inner.c, v: inner.v };
    let single = (its.len() == 1).then(|| first_node(its[0])).flatten().filter(|c| !matches!(c.prod, "braced_init" | "designator"));
    let (arr, init) = match single {
        Some(c) => {
            let e = expr(cx, c);
            if e.ty.is_error() {
                return None;
            }
            if related(&e.ty.nonref(), &inner).0 {
                return bind(cx, e, ty, "INIT-7");
            }
            if let (crate::sem::tree::EK::Str(units), true) = (&e.k, el.is_integral()) {
                let arr = with_bound(bound.unwrap_or(units.len() as u64));
                let init = list::string_init(cx, e, &arr)?;
                (arr, init)
            } else {
                let arr = with_bound(bound.unwrap_or(1));
                let x = list::braced_value(cx, e, &el, "LIST-2")?;
                (arr, Init::List(vec![Init::Expr(x)]))
            }
        }
        None => {
            if its.is_empty() && bound.is_none() {
                cx.error(b.span, "REF-11", "an empty braced list for a reference to an array of unknown bound");
                return None;
            }
            if bound.is_some_and(|n| its.len() as u64 > n) {
                cx.error(b.span, "REF-11", format!("{} values in the braces for a reference to an array of {}", its.len(), bound.unwrap_or(0)));
                return None;
            }
            let arr = with_bound(bound.unwrap_or(its.len() as u64));
            let init = list::list_init(cx, b, &arr, "")?;
            (arr, init)
        }
    };
    // REF-14: the temporary binds as REF-2 and REF-3 give.
    if !rv && !(inner.is_const() && !inner.is_volatile()) {
        cx.error(b.span, "REF-9", "a non-const lvalue reference bound to a temporary array from a braced list");
        return None;
    }
    let temp = Expr::new(crate::sem::tree::EK::List(Box::new(init)), arr.clone(), Cat::PR, b.span);
    Some(temp.conv(Conv::Materialize, arr, Cat::L))
}

/// The initializer of a reference variable (REF-11).
pub fn initializer(cx: &mut Check, ty: &QType, init: &Node) -> Option<Init> {
    let ns = nodes(init);
    let words = cx.words(init);
    let one = if let Some(b) = ns.first().filter(|c| c.prod == "braced_init") {
        let its: Vec<&Node> = b.children("init_clause").collect();
        if ty.nonref().is_array() {
            return array_reference(cx, ty, b).map(Init::Ref);
        }
        // REF-14: a list of a class's values makes a temporary of it.
        let one_of_class = its.len() == 1 && first_node(its[0]).is_some_and(|c| c.prod != "braced_init" && {
            let probe = cx.u.diags.len();
            let e = expr(cx, c);
            cx.u.diags.truncate(probe);
            e.ty.nonref().class().zip(ty.nonref().class()).is_some_and(|(a, b)| crate::classes::derived::check::derives(cx, a, b))
        });
        if ty.nonref().is_class() && !one_of_class {
            let inner = ty.nonref();
            let Type::Ref(_, rv) = ty.t else { return None };
            if !rv && !(inner.c && !inner.v) {
                cx.error(init.span, "REF-9", "a non-const lvalue reference bound to a temporary from a braced list");
                return None;
            }
            let e = crate::declarations::initialization::list::check::scalar_list(cx, b, &inner.unqual(), "LIST-2")?;
            return Some(Init::Ref(e.conv(Conv::Materialize, inner.clone(), Cat::L)));
        }
        if its.len() != 1 {
            cx.error(init.span, "REF-11", "a braced list for a reference must hold one value");
            return None;
        }
        let c = first_node(its[0])?;
        let e = expr(cx, c);
        let inner = ty.nonref();
        // REF-14: a braced class object that converts by a conversion
        // function makes a temporary, which the front end does not model.
        if crate::classes::conversion_functions::check::converting_class(cx, &e).is_some() && e.ty.nonref().class() != inner.class() {
            cx.unsupported(init.span, "a braced class object for a reference, which converts by a conversion function");
            return None;
        }
        if crate::expressions::primary::check::unresolved_set(&e) {
            return bind(cx, e, ty, "INIT-7").map(Init::Ref);
        }
        if inner.is_func() && !e.ty.is_error() && e.ty.nonref().t != inner.t {
            // A function type has no temporary, so the element must
            // have the referenced function type, also as to noexcept.
            cx.error(init.span, "REF-11", "the braced element does not have the referenced function type, and a function type has no temporary");
            return None;
        }
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
