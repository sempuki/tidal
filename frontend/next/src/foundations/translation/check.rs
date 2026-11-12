//! The check part of `foundations/translation`.

use crate::sem::check::Check;
use crate::source::Span;

/// TR-12: a declaration with specifiers that declares nothing is no
/// form of `declaration` that a program can use.
pub fn declares_nothing(cx: &mut Check, span: Span) {
    cx.error(span, "TR-12", "the declaration declares nothing");
}
