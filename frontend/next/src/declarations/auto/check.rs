//! The check part of `declarations/auto`: placeholder types and their
//! deduction from an initializer.

use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr};
use crate::sem::types::{QType, Type};
use crate::source::Span;
use crate::syntax::Node;

/// `auto` or `decltype(auto)`, with the cv-qualifiers of the specifiers.
#[derive(Clone, Debug, PartialEq)]
pub struct Placeholder {
    pub decltype: bool,
    pub c: bool,
    pub v: bool,
    pub span: Span,
}

pub fn placeholder(cx: &mut Check, n: &Node) -> Placeholder {
    if n.child("type_constraint").is_some() {
        cx.unsupported(n.span, "a type constraint before a placeholder");
    }
    Placeholder { decltype: cx.has_word(n, "decltype"), c: false, v: false, span: n.span }
}

/// The type a placeholder stands for, from the declarator's form and the
/// initializer (AUTO-2 to AUTO-5, AUTO-10). `declarator` is the
/// declarator node, applied again to the deduced type by the caller.
pub fn deduce(cx: &mut Check, p: &Placeholder, declarator: &Node, init: &Expr) -> Option<QType> {
    if init.ty.is_error() {
        return None;
    }
    if init.ty.is_void() {
        cx.error(init.span, "AUTO-8", "a variable declared with 'auto' has an initializer of type void");
        return None;
    }
    if p.decltype {
        if p.c || p.v || declarator.child("ptr_part").is_some() {
            cx.error(p.span, "AUTO-11", "decltype(auto) combined with a cv-qualifier, '*', or '&'");
            return None;
        }
        return Some(crate::expressions::decltype::check::decltype_of(init));
    }
    let ptrs: Vec<&str> = declarator.children("ptr_part").map(|pp| cx.first_word(pp)).collect();
    if declarator.child("ptr_part").is_some() && declarator.child("declarator").and_then(|d| d.child("ptr_part")).is_some() {
        cx.unsupported(declarator.span, "a placeholder under two declarator operators");
        return None;
    }
    let t = init.ty.clone();
    let deduced = match ptrs.first().copied() {
        None => decay(&t).unqual_top(),
        Some("&") => {
            // AUTO-18: `auto&` binds to an lvalue unless the type is const.
            if init.cat == Cat::PR && !(p.c || t.c) {
                cx.error(init.span, "AUTO-18", "'auto&' with an initializer that is not an lvalue");
                return None;
            }
            t.clone()
        }
        Some("&&") => {
            if init.cat == Cat::L {
                // A forwarding reference: an lvalue gives an lvalue
                // reference (AUTO-4).
                return Some(QType::lref(t.with_cv(p.c, p.v)));
            }
            t.unqual_top()
        }
        Some("*") => match &decay(&t).t {
            Type::Ptr(q) => (**q).clone(),
            _ => {
                cx.error(init.span, "AUTO-19", "'auto*' with an initializer that is not a pointer");
                return None;
            }
        },
        _ => return None,
    };
    Some(deduced.with_cv(p.c, p.v))
}

/// The type after array-to-pointer and function-to-pointer conversion.
pub fn decay(t: &QType) -> QType {
    match &t.t {
        Type::Array(e, _) => QType::ptr_to((**e).clone()),
        Type::Func(_) => QType::ptr_to(t.clone()),
        _ => t.clone(),
    }
}

/// AUTO-8: a variable declared with `auto` needs an initializer.
pub fn needs_initializer(cx: &mut Check, span: Span, name: &str) {
    cx.error(span, "AUTO-8", format!("'{}' is declared with 'auto' and has no initializer", name));
}

/// AUTO-8: a variable declared with `auto` must not occur in its own
/// initializer.
pub fn used_in_own_initializer(cx: &mut Check, span: Span, name: &str) {
    cx.error(span, "AUTO-8", format!("'{}' is used in its own initializer", name));
}

/// AUTO-20: a parenthesized or braced initializer with one expression.
pub fn one_expression(cx: &mut Check, span: Span) {
    cx.error(span, "AUTO-20", "a placeholder needs exactly one expression in its initializer");
}

/// AUTO-22: one declaration deduces different types.
pub fn differs(cx: &mut Check, span: Span) {
    cx.error(span, "AUTO-22", "'auto' stands for different types in one declaration");
}
