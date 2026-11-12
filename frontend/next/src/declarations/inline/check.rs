//! The check part of `declarations/inline`.

use crate::sem::check::Check;
use crate::source::Span;

/// INL-2: `inline` on a declaration in a block, or on a function after
/// its definition.
pub fn misplaced(cx: &mut Check, span: Span, why: &str) {
    cx.error(span, "INL-2", format!("'inline' {}", why));
}
