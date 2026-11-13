//! The check part of `declarations/constexpr`: where `constexpr`,
//! `consteval`, and `constinit` can appear (CX-5, CINIT-2), and what a
//! `constexpr` variable needs (CX-1, CINIT-1).

use crate::declarations::check::Specs;
use crate::sem::check::Check;
use crate::source::Span;

/// CX-5, CINIT-2: specifiers that do not go together.
pub fn specifier_pairs(cx: &mut Check, s: &Specs) {
    if s.constexpr && s.consteval {
        cx.error(s.span, "CX-5", "'constexpr' and 'consteval' on one declaration");
    }
    if s.constinit && (s.constexpr || s.consteval) {
        cx.error(s.span, "CINIT-2", "'constinit' with 'constexpr' or 'consteval'");
    }
    if s.typedef && (s.constexpr || s.consteval || s.constinit) {
        cx.error(s.span, "CX-5", "'constexpr' on a typedef");
    }
}

/// CX-5: no `constexpr` on a parameter.
pub fn on_parameter(cx: &mut Check, span: Span) {
    cx.error(span, "CX-5", "'constexpr', 'consteval', or 'constinit' on a parameter");
}

/// CX-5: `consteval` applies to functions only.
pub fn consteval_variable(cx: &mut Check, span: Span) {
    cx.error(span, "CX-5", "'consteval' on a variable");
}

/// CINIT-2: `constinit` on a function, or on a variable with automatic
/// storage.
pub fn constinit_place(cx: &mut Check, span: Span, why: &str) {
    cx.error(span, "CINIT-2", format!("'constinit' {}", why));
}

/// CX-1: a `constexpr` variable needs an initializer that is a constant
/// expression, and a literal type.
pub fn needs_initializer(cx: &mut Check, span: Span, name: &str) {
    cx.error(span, "CX-1", format!("the constexpr variable '{}' has no initializer", name));
}

/// The rule that a constexpr variable's initializer breaks.
pub const VARIABLE: &str = "CX-1";
/// The rule that a constinit variable's initializer breaks.
pub const CONSTINIT: &str = "CINIT-1";

/// CEV-1: a call of a `consteval` function outside an immediate function
/// context must be a constant expression.
pub fn immediate_call(cx: &mut Check, f: crate::sem::EntityId, call: crate::sem::tree::Expr) -> crate::sem::tree::Expr {
    if !cx.u.func(f).is_some_and(|x| x.consteval) || cx.func.as_ref().is_some_and(|c| c.consteval) || cx.unevaluated > 0 {
        return call;
    }
    if let Err(why) = crate::foundations::constant_evaluation::check::evaluate(cx, &call) {
        crate::foundations::constant_evaluation::check::report(cx, call.span, "CEV-1", why);
    }
    call
}

/// CEV-2: the address of a `consteval` function is taken only in an
/// immediate function context (the body of a `consteval` function).
pub fn address_of(cx: &mut Check, f: crate::sem::EntityId, span: Span) {
    let immediate = cx.u.func(f).is_some_and(|x| x.consteval);
    let inside = cx.func.as_ref().is_some_and(|c| c.consteval);
    if immediate && !inside {
        cx.error(span, "CEV-2", format!("the address of the consteval function '{}' is taken", cx.u.entities[f].name));
    }
}
