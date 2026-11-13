//! The check part of `declarations/volatile`.

use crate::sem::check::Check;
use crate::sem::types::QType;
use crate::source::Span;

/// VOL-5: `++` and `--` on a volatile object are deprecated, and an
/// implementation should warn.
pub fn inc_dec(cx: &mut Check, t: &QType, span: Span) {
    if t.v && cx.wflag(&["-Wall", "-Wdeprecated", "-Wvolatile", "-Wdeprecated-volatile"]) {
        cx.warning(span, "VOL-5", "an increment or decrement of a volatile object is deprecated");
    }
}

/// VOL-5: a compound assignment to a volatile object is deprecated.
pub fn assignment(cx: &mut Check, t: &QType, span: Span) {
    let _ = (cx, t, span);
}

/// The flags that turn on the deprecation warnings of VOL-5 and CV-9.
fn on(cx: &Check) -> bool {
    cx.wflag(&["-Wall", "-Wdeprecated", "-Wvolatile", "-Wdeprecated-volatile"])
}

/// VOL-5: the value of an assignment to a volatile object, used in a
/// larger expression (its conversion to a prvalue).
pub fn assignment_value_used(cx: &mut Check, e: &crate::sem::tree::Expr) {
    if matches!(e.k, crate::sem::tree::EK::Assign(..)) && e.ty.v && on(cx) {
        cx.warning(e.span, "VOL-5", "the value of an assignment to a volatile object is used");
    }
}

/// CV-9: a return type or a parameter type with a top-level `volatile`.
pub fn top_level(cx: &mut Check, t: &QType, span: Span, what: &str) {
    if t.v && !t.is_ref() && on(cx) {
        cx.warning(span, "CV-9", format!("a {} with a top-level volatile is deprecated", what));
    }
}
