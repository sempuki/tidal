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
