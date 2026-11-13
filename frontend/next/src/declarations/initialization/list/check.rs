//! The check part of `declarations/initialization/list`: braced lists
//! for scalars and arrays (LIST-1, LIST-2, LIST-11, LIST-15). Aggregate
//! classes and `std::initializer_list` come with build step 4.

use crate::expressions::check::{expr, first_node};
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::rvalue;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Conv, Expr, Init, EK};
use crate::sem::types::{IntK, QType, Type};
use crate::source::Span;
use crate::syntax::Node;

/// The element nodes of a braced list.
pub fn items(b: &Node) -> Vec<&Node> {
    b.children("init_clause").collect()
}

/// A value converted in braces: LIST-2 forbids a narrowing conversion.
pub fn braced_value(cx: &mut Check, e: Expr, to: &QType, rule: &'static str) -> Option<Expr> {
    if e.ty.is_error() || to.is_error() {
        return None;
    }
    let span = e.span;
    let narrow = {
        let r = rvalue(cx, e.clone());
        (r.ty.is_arithmetic() || r.ty.is_enum() || r.ty.is_ptr() || r.ty.is_nullptr()) && (to.is_arithmetic() || to.is_enum()) && conv::is_narrowing(cx, &r, to)
    };
    if narrow {
        cx.error(span, if rule == "INIT-7" { "LIST-2" } else { rule }, format!("a narrowing conversion to {} in braces", to.show()));
        if rule != "LIST-2" && rule != "INIT-7" {
            // The construct's own rule names it; LIST-2 is the general one.
        }
        return None;
    }
    // ENUM-8: an enumeration with a fixed underlying type takes an
    // integer in braces.
    if let Type::Enum(en) = to.t {
        let r = rvalue(cx, e);
        if r.ty.t == to.t {
            return Some(r);
        }
        if cx.u.enums[en].fixed && (r.ty.is_integral() || matches!(r.ty.t, Type::Enum(o) if !cx.u.enums[o].scoped)) {
            return Some(r.conv(Conv::Arith, to.unqual(), Cat::PR));
        }
        cx.error(span, "ENUM-19", format!("{} does not initialize the enumeration in braces", r.ty.show()));
        return None;
    }
    match conv::implicit(cx, e, to, true) {
        Ok(x) => Some(x),
        Err(m) => {
            cx.error(span, conv::rule_of(&m).unwrap_or(if rule == "LIST-2" { "INIT-7" } else { rule }), m);
            None
        }
    }
}

/// A braced list for a scalar (LIST-11): empty for zero, or one
/// expression that converts without narrowing.
pub fn scalar_list(cx: &mut Check, b: &Node, to: &QType, rule: &'static str) -> Option<Expr> {
    let its = items(b);
    let span = b.span;
    if to.is_error() {
        return None;
    }
    if to.is_array() {
        return array_prvalue(cx, b, to);
    }
    if to.is_void() {
        cx.unsupported(span, "a braced list for this type here");
        return None;
    }
    match its.len() {
        0 => Some(zero(to, span)),
        1 => {
            let c = first_node(its[0])?;
            if c.prod == "braced_init" || c.prod == "designator" {
                cx.error(span, "LIST-11", "a braced list inside the braces for a scalar");
                return None;
            }
            let e = expr(cx, c);
            braced_value(cx, e, to, if rule == "INIT-7" { "LIST-2" } else { rule })
        }
        _ => {
            cx.error(span, "LIST-11", "more than one value in the braces for a scalar");
            None
        }
    }
}

/// An array prvalue from a braced list (LIST-3, CAST-18). An array of
/// unknown bound takes one element for each value.
pub fn array_prvalue(cx: &mut Check, b: &Node, to: &QType) -> Option<Expr> {
    let ty = match &to.t {
        Type::Array(el, None) => {
            let n = items(b).len() as u64;
            if n == 0 {
                cx.error(b.span, "DCL-19", "an array of unknown bound with an empty initializer");
                return None;
            }
            QType { t: Type::Array(el.clone(), Some(n)), c: to.c, v: to.v }
        }
        _ => to.clone(),
    };
    let init = list_init(cx, b, &ty, "")?;
    Some(Expr::new(EK::List(Box::new(init)), ty, Cat::PR, b.span))
}

/// The zero value of a scalar type.
pub fn zero(to: &QType, span: Span) -> Expr {
    match &to.t {
        Type::Float(_) => Expr::new(EK::Float(0.0), to.unqual(), Cat::PR, span),
        Type::Ptr(_) | Type::NullPtr => Expr::int(0, QType::new(Type::NullPtr), span).conv(Conv::NullPtr, to.unqual(), Cat::PR),
        _ => Expr::int(0, to.unqual(), span),
    }
}

/// The list initialization of a variable of type `ty`.
pub fn list_init(cx: &mut Check, b: &Node, ty: &QType, name: &str) -> Option<Init> {
    let _ = name;
    match &ty.t {
        Type::Array(el, n) => array_list(cx, b, el, *n),
        _ => {
            if items(b).is_empty() {
                return Some(Init::Zero);
            }
            scalar_list(cx, b, ty, "LIST-2").map(Init::Expr)
        }
    }
}

/// An array from a braced list (LIST-3, LIST-5, DCL-19): at most one
/// value per element, nested lists for nested arrays, and brace elision.
fn array_list(cx: &mut Check, b: &Node, el: &QType, bound: Option<u64>) -> Option<Init> {
    let its = items(b);
    if its.len() == 1 && el.is_integral() && matches!(el.int_kind(), Some(IntK::Char | IntK::SChar | IntK::UChar | IntK::Char8 | IntK::Char16 | IntK::Char32 | IntK::WChar)) {
        if let Some(c) = first_node(its[0]) {
            let e = expr(cx, c);
            if let EK::Str(_) = e.k {
                return string_init(cx, e, &QType::new(Type::Array(std::rc::Rc::new(el.clone()), bound)));
            }
        }
    }
    if its.is_empty() && bound.is_none() {
        cx.error(b.span, "DCL-19", "an array of unknown bound with an empty initializer");
        return None;
    }
    let mut out = Vec::new();
    for it in &its {
        let c = first_node(it)?;
        if c.prod == "designator" || it.child("designator").is_some() {
            cx.error(it.span, "LIST-14", "a designator in the list of an array");
            return None;
        }
        let v = if c.prod == "braced_init" {
            match &el.t {
                Type::Array(e2, n2) => array_list(cx, c, e2, *n2)?,
                _ => Init::Expr(scalar_list(cx, c, el, "LIST-2")?),
            }
        } else if el.is_array() {
            cx.unsupported(c.span, "brace elision in a nested array");
            return None;
        } else {
            let e = expr(cx, c);
            Init::Expr(braced_value(cx, e, el, "LIST-2")?)
        };
        out.push(v);
    }
    if let Some(n) = bound {
        if out.len() as u64 > n {
            cx.error(b.span, "DCL-19", format!("the initializer has {} elements for an array of {}", out.len(), n));
            return None;
        }
    }
    Some(Init::List(out))
}

/// A character array from a string literal (LIST-6, LIST-15).
pub fn string_init(cx: &mut Check, e: Expr, ty: &QType) -> Option<Init> {
    let (Type::Array(el, n), EK::Str(units)) = (&ty.t, &e.k) else { return None };
    let lit_el = e.ty.elem().and_then(|x| x.int_kind());
    let ok = match (el.int_kind(), lit_el) {
        (Some(IntK::Char | IntK::SChar | IntK::UChar), Some(IntK::Char)) => true,
        (Some(IntK::Char | IntK::UChar | IntK::Char8), Some(IntK::Char8)) => true,
        (Some(a), Some(b)) => a == b,
        _ => false,
    };
    if !ok {
        cx.error(e.span, "LIST-15", format!("a string literal of type {} does not initialize an array of {}", e.ty.show(), el.show()));
        return None;
    }
    if let Some(n) = n {
        if units.len() as u64 > *n {
            if units.len() as u64 - 1 > *n || true {
                cx.error(e.span, "LIST-15", format!("the string literal with its null character does not fit in {} elements", n));
                return None;
            }
        }
    }
    let items = units.iter().map(|&u| Init::Expr(Expr::int(u as i128, el.unqual(), e.span))).collect();
    Some(Init::List(items))
}
