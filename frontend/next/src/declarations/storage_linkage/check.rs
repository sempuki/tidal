//! The check part of `declarations/storage-linkage`: storage classes,
//! storage duration, and linkage.

use crate::declarations::check::Specs;
use crate::sem::check::Check;
use crate::sem::{EntityId, Linkage, ScopeKind, Storage};
use crate::source::Span;

/// STO-6: two storage classes, except `thread_local` with `static` or
/// `extern`.
pub fn two_storage_classes(cx: &mut Check, span: Span, seen: &[&str], w: &str) {
    cx.error(span, "STO-6", format!("the storage class '{}' after '{}'", w, seen.join(" ")));
}

/// STO-5: `register` is not a storage class.
pub fn register(cx: &mut Check, span: Span) {
    cx.error(span, "STO-5", "'register' is not a storage class");
}

/// STO-6: a storage class with `typedef`.
pub fn typedef_with_storage(cx: &mut Check, span: Span) {
    cx.error_cause(span, "STO-6", Some("TYN-2"), "a storage class with typedef");
}

/// STO-6: a storage class on a function parameter.
pub fn parameter_storage(cx: &mut Check, span: Span) {
    cx.error(span, "STO-6", "a storage class on a function parameter");
}

/// STO-6 on a function: no `thread_local`, no `mutable`, no `static` in
/// a block, no `static` on `main`.
pub fn function_specifiers(cx: &mut Check, s: &Specs, local: bool, is_main: bool) {
    if s.thread_local {
        cx.error(s.span, "STO-6", "a function declared thread_local");
    }
    if s.mutable {
        cx.error(s.span, "STO-6", "a function declared mutable");
    }
    if s.is_static && local {
        cx.error(s.span, "STO-6", "a function declared static in a block");
    }
    if s.is_static && is_main {
        cx.error(s.span, "STO-6", "main declared static");
    }
}

/// STO-6: `mutable` applies only to non-static data members.
pub fn variable_specifiers(cx: &mut Check, s: &Specs) {
    if s.mutable {
        cx.error(s.span, "STO-6", "a variable declared mutable outside a class");
    }
}

/// The storage duration of a variable (STO-1 to STO-4).
pub fn storage(cx: &Check, s: &Specs) -> Storage {
    let local = cx.u.is_local(cx.scope);
    if s.thread_local {
        Storage::Thread
    } else if !local || s.is_static || s.is_extern {
        Storage::Static
    } else {
        Storage::Automatic
    }
}

/// The linkage of a variable or function at the current scope (LNK-1 to
/// LNK-3, LNK-5, LNK-9).
pub fn linkage(cx: &Check, s: &Specs, is_var: bool, ty_const: bool, ty_volatile: bool) -> Linkage {
    if cx.u.is_local(cx.scope) {
        return if s.is_extern || !is_var { Linkage::External } else { Linkage::None };
    }
    if s.is_static {
        return Linkage::Internal;
    }
    if in_unnamed(cx, cx.scope) {
        return Linkage::Internal;
    }
    if is_var && ty_const && !ty_volatile && !s.inline && !s.is_extern {
        return Linkage::Internal;
    }
    Linkage::External
}

fn in_unnamed(cx: &Check, mut s: usize) -> bool {
    loop {
        let sc = &cx.u.scopes[s];
        if sc.kind == ScopeKind::Namespace && sc.name.is_empty() && sc.parent.is_some() {
            return true;
        }
        match sc.parent {
            Some(p) => s = p,
            None => return false,
        }
    }
}

/// LNK-10: a `static` declaration after one with external linkage.
pub fn redeclared_linkage(cx: &mut Check, span: Span, old: Linkage, s: &Specs, name: &str) {
    if s.is_static && old == Linkage::External {
        cx.error(span, "LNK-10", format!("'{}' is declared static after a declaration with external linkage", name));
    }
}

/// STO-12: all declarations of a variable agree on `thread_local`.
pub fn thread_local_agrees(cx: &mut Check, span: Span, old: Storage, new: Storage, name: &str) {
    if (old == Storage::Thread) != (new == Storage::Thread) {
        cx.error(span, "STO-12", format!("the declarations of '{}' disagree on thread_local", name));
    }
}

/// STO-13: a block-scope `extern` declaration has no initializer.
pub fn block_extern_init(cx: &mut Check, span: Span) {
    cx.error(span, "STO-13", "a block-scope extern declaration has an initializer");
}

pub fn is_static_storage(cx: &Check, id: EntityId) -> bool {
    cx.u.var(id).is_some_and(|v| v.storage != Storage::Automatic)
}
