//! The check part of `foundations/values`: value categories and the
//! conversions that turn a glvalue into the prvalue an operator needs.

use crate::foundations::conversions::check as conv;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Conv, Expr};
use crate::sem::types::QType;

/// The prvalue of an operand (VC-1, VC-2): an array or a function decays
/// to a pointer (CONV-13), and a glvalue of another type is read, which
/// drops its top-level cv-qualifiers.
pub fn rvalue(cx: &mut Check, e: Expr) -> Expr {
    if crate::expressions::primary::check::unresolved_set(&e) {
        crate::expressions::primary::check::no_target(cx, &e, "OVL-20");
        return Expr::new(crate::sem::tree::EK::Int(0), QType::error(), Cat::PR, e.span);
    }
    if e.ty.is_error() {
        return e;
    }
    let e = deref_ref(e);
    if e.ty.is_array() || e.ty.is_func() {
        return conv::decay(cx, e);
    }
    if e.cat == Cat::PR {
        if e.ty.c || e.ty.v {
            let t = e.ty.unqual();
            let mut e = e;
            e.ty = t;
            return e;
        }
        return e;
    }
    crate::declarations::volatile::check::assignment_value_used(cx, &e);
    // LAM-9: a constant of an enclosing function read in a lambda body.
    if !e.ty.is_class() {
        crate::expressions::lambdas::check::read(cx, &e);
    }
    if let Some(x) = crate::expressions::lambdas::check::empty_copy(cx, &e) {
        return x;
    }
    if let Some(c) = e.ty.class() {
        // UN-6: no value of an incomplete class. VOL-11: the implicit copy
        // of a class does not take a volatile object.
        if !cx.u.classes[c].complete {
            cx.error(e.span, "UN-6", "a read of an object of an incomplete class");
            return Expr::new(crate::sem::tree::EK::Int(0), QType::error(), Cat::PR, e.span);
        }
        // CPY-9: a copy or move constructor that runs code.
        if crate::classes::copy_move::check::needs_copy_function(cx, c) {
            let span = e.span;
            return crate::classes::copy_move::check::copy(cx, e, true, span).unwrap_or_else(|| Expr::new(crate::sem::tree::EK::Int(0), QType::error(), Cat::PR, span));
        }
        if e.ty.v {
            cx.error(e.span, "VOL-11", "a copy of a volatile object of a class");
            return Expr::new(crate::sem::tree::EK::Int(0), QType::error(), Cat::PR, e.span);
        }
        // CPY-7: an rvalue reference member deletes the implicit copy
        // constructor.
        if e.cat == Cat::L && crate::classes::copy_move::check::rvalue_ref_member(cx, c) {
            cx.error(e.span, "CPY-23", format!("the copy constructor of '{}' is deleted, because the class has an rvalue reference member", cx.u.classes[c].name));
            return Expr::new(crate::sem::tree::EK::Int(0), QType::error(), Cat::PR, e.span);
        }

    }
    let t = e.ty.unqual();
    e.conv(Conv::LToR, t, Cat::PR)
}

/// An expression of reference type names the object it refers to, as an
/// lvalue (or an xvalue for an rvalue reference to an object returned
/// by a call).
pub fn deref_ref(e: Expr) -> Expr {
    match &e.ty.t {
        crate::sem::types::Type::Ref(inner, rv) => {
            let cat = if *rv && e.cat == Cat::PR { Cat::X } else { Cat::L };
            let t = (**inner).clone();
            let mut e = e;
            e.ty = t;
            e.cat = cat;
            e
        }
        _ => e,
    }
}

/// VC-12, VC-13: the address of a prvalue or an xvalue.
pub fn address_of_non_lvalue(cx: &mut Check, e: &Expr) -> QType {
    if e.cat == Cat::X {
        cx.error(e.span, "VC-13", "the address of an xvalue")
    } else {
        // UN-9 for `&`, and MUL-19 for the result of `*`, `/`, `%`.
        let cause = if matches!(e.k, crate::sem::tree::EK::Binary(crate::sem::tree::BOp::Mul | crate::sem::tree::BOp::Div | crate::sem::tree::BOp::Rem, ..)) { "MUL-19" } else { "UN-9" };
        cx.error_cause(e.span, "VC-12", Some(cause), "the address of a prvalue");
        QType::error()
    }
}

/// VC-12: a prvalue of a type that is not a class cannot be assigned to.
pub fn assign_to_prvalue(cx: &mut Check, e: &Expr) -> QType {
    cx.error(e.span, "VC-12", "an assignment to a prvalue")
}
