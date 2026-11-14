//! The check part of `foundations/constant-evaluation`: the places that
//! need a constant expression evaluate it on the machine with the limits
//! of this page (`run.rs`), and report the place's rule with the limit's
//! rule as its cause.

use super::run::value_of;
use crate::expressions;
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::rvalue;
use crate::machine::{Machine, Val};
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr};
use crate::sem::types::{IntK, QType, Type};
use crate::syntax::Node;

/// Evaluate a typed expression as a constant expression.
pub fn evaluate(cx: &Check, e: &Expr) -> Result<Val, (&'static str, String)> {
    let mut m = Machine::new(&cx.u, true);
    let r = if e.cat == Cat::PR { crate::machine::eval::eval(&mut m, e) } else { crate::machine::eval::eval_any(&mut m, e) };
    let r = r.and_then(|v| {
        crate::expressions::new_delete::run::leaked(&m)?;
        // CE-14: a pointer that is the value of a constant expression
        // must not point to an automatic or temporary object.
        if let Val::Ptr(p) = &v {
            if !m.objs[p.obj].is_static {
                return crate::machine::limit("CE-14", "the value points to an object that is not static");
            }
        }
        Ok(v)
    });
    value_of(r)
}

/// Evaluate the glvalue a reference binds to as a constant expression:
/// it must name a function or an object with static storage (CE-14,
/// CX-2). `static_storage` says that the reference has static storage,
/// so a temporary it binds does too.
pub fn evaluate_ref(cx: &Check, e: &Expr, static_storage: bool) -> Result<(), (&'static str, String)> {
    let mut m = Machine::new(&cx.u, true);
    m.static_temps = static_storage;
    let r = crate::machine::eval::place(&mut m, e).and_then(|p| {
        crate::expressions::new_delete::run::leaked(&m)?;
        if !m.objs[p.obj].is_static {
            return crate::machine::limit("CE-14", "the reference refers to an object that is not static");
        }
        Ok(Val::Void)
    });
    value_of(r).map(|_| ())
}

/// The value of an expression if it is a constant expression.
pub fn try_value(cx: &Check, e: &Expr) -> Option<Val> {
    if e.ty.is_error() {
        return None;
    }
    evaluate(cx, e).ok()
}

/// Report a failed constant evaluation at the place's rule. An evaluation
/// that met a construct the machine does not run is unsupported.
pub fn report(cx: &mut Check, span: crate::source::Span, rule: &'static str, why: (&'static str, String)) {
    if why.0.is_empty() {
        cx.unsupported(span, format!("constant evaluation: {}", why.1));
        return;
    }
    cx.error_cause(span, rule, Some(why.0), format!("not a constant expression: {}", why.1));
}

/// A constant expression of an integral or unscoped enumeration type
/// (CE-11), converted to `long long` range. `rule` is the place's rule.
pub fn integral_constant(cx: &mut Check, n: &Node, rule: &'static str) -> Option<i128> {
    let e = expressions::check::expr(cx, n);
    let e = rvalue(cx, e);
    if e.ty.is_error() {
        return None;
    }
    let ok = e.ty.is_integral() || matches!(e.ty.t, Type::Enum(en) if !cx.u.enums[en].scoped);
    if !ok {
        cx.error_cause(e.span, rule, Some("CE-11"), format!("the constant has type {}, not an integer type", e.ty.show()));
        return None;
    }
    match evaluate(cx, &e) {
        Ok(Val::Int(v)) => Some(v),
        Ok(_) => None,
        Err(why) => {
            report(cx, e.span, rule, why);
            None
        }
    }
}

/// A contextually converted constant expression of type `bool`
/// (CE-19: the conversion must not narrow).
pub fn bool_constant(cx: &mut Check, n: &Node, rule: &'static str) -> Option<bool> {
    let e = expressions::check::expr(cx, n);
    let e = rvalue(cx, e);
    if e.ty.is_error() {
        return None;
    }
    let b = conv::contextual_bool(cx, e, rule);
    if b.ty.is_error() {
        return None;
    }
    match evaluate(cx, &b) {
        Ok(v) => Some(v.truthy()),
        Err(why) => {
            report(cx, b.span, rule, why);
            None
        }
    }
}

/// An array bound (DCL-19): a constant of integral or unscoped
/// enumeration type, greater than zero.
pub fn array_bound(cx: &mut Check, n: &Node) -> Option<u64> {
    let e = expressions::check::expr(cx, n);
    let e = rvalue(cx, e);
    if e.ty.is_error() {
        return None;
    }
    let ok = e.ty.is_integral() || matches!(e.ty.t, Type::Enum(en) if !cx.u.enums[en].scoped);
    if !ok {
        cx.error(e.span, "DCL-19", format!("an array bound of type {}", e.ty.show()));
        return None;
    }
    match evaluate(cx, &e) {
        Ok(Val::Int(v)) if v > 0 => Some(v as u64),
        Ok(_) => {
            cx.error(e.span, "DCL-19", "the array bound is not greater than zero");
            None
        }
        Err(why) => {
            if why.0.is_empty() {
                report(cx, e.span, "DCL-19", why);
            } else if cx.in_function() && why.0 == "CE-3" {
                // A bound that is not constant makes a variable-length
                // array, which Tidal does not have (DCL-19).
                cx.error_cause(e.span, "DCL-19", Some(why.0), format!("the array bound is not a constant expression: {}", why.1));
            } else {
                report(cx, e.span, "DCL-19", why);
            }
            None
        }
    }
}

/// An enumerator's value (ENUM-14), converted to the fixed underlying
/// type without narrowing (ENUM-17).
pub fn enumerator_value(cx: &mut Check, n: &Node, fixed: Option<IntK>) -> Option<(i128, IntK)> {
    let e = expressions::check::expr(cx, n);
    let e = rvalue(cx, e);
    if e.ty.is_error() {
        return None;
    }
    let ok = e.ty.is_integral() || e.ty.is_enum();
    if !ok {
        cx.error_cause(e.span, "ENUM-14", Some("CE-11"), "an enumerator value must have an integer type");
        return None;
    }
    if let Some(k) = fixed {
        if conv::is_narrowing(cx, &e, &QType::new(Type::Int(k))) {
            cx.error(e.span, "ENUM-17", "the enumerator value does not fit in the underlying type");
            return None;
        }
    }
    let k = match &e.ty.t {
        Type::Int(k) => *k,
        Type::Enum(en) => cx.u.enums[*en].underlying.unwrap_or(IntK::Int),
        _ => IntK::Int,
    };
    match evaluate(cx, &e) {
        Ok(Val::Int(v)) => Some((v, k)),
        Ok(_) => None,
        Err(why) => {
            report(cx, e.span, "ENUM-14", why);
            None
        }
    }
}

/// Whether a typed expression is a constant expression, for a variable's
/// constant initializer (CE-24).
pub fn is_constant(cx: &Check, e: &Expr) -> bool {
    try_value(cx, e).is_some()
}
