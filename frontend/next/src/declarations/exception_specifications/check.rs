//! The check part of `declarations/exception-specifications`.

use crate::foundations::constant_evaluation::check as ce;
use crate::sem::check::Check;
use crate::syntax::Node;

/// `exception_spec`: whether the function is non-throwing (EXS-1,
/// EXS-2). `noexcept(e)` needs a constant of type `bool`.
pub fn exception_spec(cx: &mut Check, n: &Node) -> bool {
    let Some(ns) = n.child("noexcept_specifier") else {
        // `throw()` is non-throwing (EXS-5).
        return true;
    };
    match ns.kids.iter().find_map(|k| match k {
        crate::syntax::Kid::Node(c) => Some(c),
        _ => None,
    }) {
        None => true,
        Some(e) if matches!(cx.source(e).as_str(), "true" | "false") => ce::bool_constant(cx, e, "EXS-3").unwrap_or(true),
        Some(e) => {
            // The operand can name the parameters, which the declarator
            // has not declared yet.
            cx.unsupported(e.span, "a noexcept specifier with an operand");
            true
        }
    }
}
