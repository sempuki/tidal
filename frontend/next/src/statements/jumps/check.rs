//! The check part of `statements/jumps`: `break`, `continue`, `return`,
//! and `goto` (JMP-1 to JMP-7, JMP-13, JMP-14).

use crate::expressions::check::{expr, first_node};
use crate::sem::check::{Check, FnCtx};
use crate::sem::tree::{Stmt, SK};
use crate::sem::types::QType;
use crate::syntax::Node;
use std::rc::Rc;

/// The statement that `break` (or `continue`, for a loop) acts on: the
/// innermost loop or switch (JMP-1, JMP-2), or None (JMP-7).
pub fn target(cx: &Check, is_continue: bool) -> Option<u32> {
    let f = cx.func.as_ref()?;
    f.targets.iter().rev().find(|t| t.is_loop || !is_continue).map(|t| t.id)
}

/// The enclosing loops and switches, innermost last, with the labels
/// that mark each: the public query that a proposal's check reads.
pub fn enclosing(cx: &Check) -> Vec<crate::sem::check::JumpTarget> {
    cx.func.as_ref().map(|f| f.targets.clone()).unwrap_or_default()
}

/// `jump_stmt`.
pub fn jump_stmt(cx: &mut Check, n: &Node) -> Stmt {
    let span = n.span;
    match cx.first_word(n) {
        w @ ("break" | "continue") => match target(cx, w == "continue") {
            Some(id) => Stmt::new(if w == "break" { SK::Break(id) } else { SK::Continue(id) }, span),
            None => {
                let what = if w == "break" { "a loop or switch" } else { "a loop" };
                cx.error(span, "JMP-7", format!("'{}' outside {}", w, what));
                Stmt::new(SK::Null, span)
            }
        },
        "goto" => {
            let Some(name) = n.kids.get(1).map(|k| Rc::<str>::from(cx.text(k))) else { return Stmt::new(SK::Null, span) };
            let live = cx.func.as_ref().map(|f| f.live.clone()).unwrap_or_default();
            if let Some(f) = cx.func.as_mut() {
                let ifs = f.constexpr_ifs.clone();
                f.gotos.push((name.clone(), span, live, ifs));
            }
            Stmt::new(SK::Goto(name), span)
        }
        _ => return_stmt(cx, n),
    }
}

/// `return` (JMP-3, JMP-13, RET-1, RET-7).
fn return_stmt(cx: &mut Check, n: &Node) -> Stmt {
    let span = n.span;
    let Some(ctx) = cx.func.clone() else { return Stmt::new(SK::Null, span) };
    let e = first_node(n);
    if let Some(b) = e.filter(|b| b.prod == "braced_init") {
        if ctx.deduce {
            cx.error(span, "RET-7", "a return of a braced list in a function with a deduced return type");
            return Stmt::new(SK::Null, span);
        }
        let ret = ctx.ret.clone().unwrap_or_else(QType::error);
        if ret.is_void() {
            cx.error(span, "JMP-13", "a return with a value in a void function");
            return Stmt::new(SK::Null, span);
        }
        return match crate::declarations::initialization::list::check::scalar_list(cx, b, &ret, "LIST-2") {
            Some(x) => Stmt::new(SK::Return(Some(x)), span),
            None => Stmt::new(SK::Null, span),
        };
    }
    let value = e.map(|c| expr(cx, c));
    if ctx.deduce && ctx.discarded > 0 {
        // RET-1: a return in a discarded statement deduces nothing.
        return Stmt::new(SK::Null, span);
    }
    if ctx.deduce {
        let t = match &value {
            Some(v) if crate::expressions::primary::check::unresolved_set(v) => {
                crate::expressions::primary::check::no_target(cx, v, "OVL-20");
                return Stmt::new(SK::Null, span);
            }
            Some(v) if v.ty.is_error() => return Stmt::new(SK::Null, span),
            Some(v) if !v.ty.is_void() && ctx.placeholder.as_ref().is_some_and(|(p, op)| p.decltype || op.is_some()) => match placeholder_return(cx, v, &ctx) {
                Some(t) => t,
                None => {
                    if let Some(f) = cx.func.as_mut() {
                        f.failed_return = true;
                    }
                    return Stmt::new(SK::Null, span);
                }
            },
            Some(v) => crate::declarations::auto::check::decay(&v.ty).unqual_top(),
            None => QType::void(),
        };
        match &ctx.ret {
            Some(old) if *old != t => {
                cx.error(span, "RET-7", format!("the returns give different types, {} and {}", old.show(), t.show()));
                return Stmt::new(SK::Null, span);
            }
            None => {
                if let Some(f) = cx.func.as_mut() {
                    f.ret = Some(t.clone());
                }
                if let Some(id) = ctx.id {
                    if let Some(fm) = cx.u.func_mut(id) {
                        let mut ft = (*fm.ty).clone();
                        ft.ret = t.clone();
                        fm.ty = Rc::new(ft);
                        fm.pending_return = false;
                    }
                }
            }
            _ => {}
        }
        return finish(cx, value, &t, span, &ctx);
    }
    let ret = ctx.ret.clone().unwrap_or_else(QType::error);
    finish(cx, value, &ret, span, &ctx)
}

fn finish(cx: &mut Check, value: Option<crate::sem::tree::Expr>, ret: &QType, span: crate::source::Span, ctx: &FnCtx) -> Stmt {
    match value {
        None => {
            if !ret.is_void() && !ret.is_error() {
                cx.error(span, "JMP-13", "a return without a value in a function that returns a value");
                return Stmt::new(SK::Null, span);
            }
            Stmt::new(SK::Return(None), span)
        }
        Some(v) => {
            if v.ty.is_error() || ret.is_error() {
                return Stmt::new(SK::Null, span);
            }
            if ret.is_void() {
                if !v.ty.is_void() {
                    cx.error(span, "JMP-13", "a return with a value in a void function");
                    return Stmt::new(SK::Null, span);
                }
                return Stmt::new(SK::Expr(v), span);
            }
            let _ = ctx;
            // JMP-10: a local variable or parameter named in a return is
            // an xvalue, so it does not bind to an lvalue reference
            // return type (JMP-15).
            let mut v = v;
            if ret.is_ref() && move_eligible(cx, &v) {
                v.cat = crate::sem::tree::Cat::X;
            }
            if ret.is_ref() {
                return match crate::declarations::initialization::references::check::bind(cx, v, ret, "INIT-7") {
                    Some(x) => Stmt::new(SK::ReturnRef(x), span),
                    None => Stmt::new(SK::Null, span),
                };
            }
            match crate::declarations::initialization::forms::check::copy_init(cx, v, ret, "INIT-7") {
                Some(x) => Stmt::new(SK::Return(Some(x)), span),
                None => Stmt::new(SK::Null, span),
            }
        }
    }
}

/// JMP-10: a name in a return that is an xvalue, a local variable or
/// parameter of automatic storage that is not `volatile`, or a local
/// rvalue reference. A structured binding stays an lvalue.
fn move_eligible(cx: &Check, v: &crate::sem::tree::Expr) -> bool {
    let crate::sem::tree::EK::Var(id) = &v.k else { return false };
    if cx.bindings.contains(id) {
        return false;
    }
    let movable = |x: &crate::sem::Var| match &x.ty.t {
        crate::sem::types::Type::Ref(t, true) => !t.is_volatile(),
        crate::sem::types::Type::Ref(_, false) => false,
        _ => !x.ty.is_volatile(),
    };
    cx.u.var(*id).is_some_and(|x| x.storage == crate::sem::Storage::Automatic && movable(x))
}

/// The return type that a return deduces for `decltype(auto)`, `auto&`,
/// `auto&&`, or `auto*` (RET-2): the placeholder deduces as for a
/// variable, from the operand as JMP-10 makes it, and the declarator
/// operator applies to the result. `auto&` from a local variable is
/// RET-3.
fn placeholder_return(cx: &mut Check, v: &crate::sem::tree::Expr, ctx: &FnCtx) -> Option<QType> {
    let (p, op) = ctx.placeholder.clone()?;
    let mut v = v.clone();
    if move_eligible(cx, &v) {
        v.cat = crate::sem::tree::Cat::X;
        if op == Some("&") && !(p.c || v.ty.is_const()) {
            cx.error(v.span, "RET-3", "'auto&' deduced from a local variable, which a return makes an xvalue");
            return None;
        }
    }
    let t = crate::declarations::auto::check::deduce_with(cx, &p, op, &v)?;
    Some(match op {
        Some("&") => QType::lref(t),
        Some("&&") if !t.is_ref() => QType::new(crate::sem::types::Type::Ref(Rc::new(t), true)),
        Some("*") => QType::ptr_to(t),
        _ => t,
    })
}

/// At the end of a function: each `goto` names a label of the function
/// (JMP-14), skips no initialization (JMP-6), and enters no branch of
/// `if constexpr` (IF-17).
pub fn end_of_function(cx: &mut Check, ctx: &FnCtx) {
    for (name, span, live, ifs) in &ctx.gotos {
        match ctx.labels.get(name) {
            None => {
                cx.error(*span, "JMP-14", format!("the label '{}' is not defined in the function", name));
            }
            Some((_, at, at_ifs)) => {
                if at.iter().any(|v| !live.contains(v)) {
                    cx.error(*span, "JMP-6", format!("the jump to '{}' skips the initialization of a variable", name));
                }
                if at_ifs.iter().any(|i| !ifs.contains(i)) {
                    cx.error(*span, "IF-17", format!("the jump to '{}' enters a branch of 'if constexpr'", name));
                }
            }
        }
    }
}

/// The labels of the function being checked, with their spans: the
/// table that `goto` uses (NAM-7), a public query for proposals.
pub fn labels(cx: &Check) -> Vec<Rc<str>> {
    cx.func.as_ref().map(|f| f.labels.keys().cloned().collect()).unwrap_or_default()
}
