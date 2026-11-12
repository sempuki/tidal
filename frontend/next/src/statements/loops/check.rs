//! The check part of `statements/loops`: `while`, `do`, and `for`.

use crate::expressions::check::{expr, first_node};
use crate::foundations::conversions::check as conv;
use crate::sem::check::{Check, JumpTarget};
use crate::sem::tree::{Stmt, SK};
use crate::sem::ScopeKind;
use crate::statements::check::sub_statement;
use crate::statements::if_stmt::check::{condition, init_stmt};
use crate::syntax::Node;

/// Open a loop or switch that `break` (and `continue` for a loop) can
/// act on (JMP-1, JMP-2). The labels that mark the statement go with it.
pub fn push_target(cx: &mut Check, is_loop: bool) -> u32 {
    let id = cx.next_stmt_id();
    if let Some(f) = cx.func.as_mut() {
        let labels = std::mem::take(&mut f.pending_labels);
        f.targets.push(JumpTarget { id, is_loop, labels });
    }
    id
}

pub fn pop_target(cx: &mut Check) {
    if let Some(f) = cx.func.as_mut() {
        f.targets.pop();
    }
}

/// `while_stmt` (LOOP-1, LOOP-5, LOOP-8).
pub fn while_stmt(cx: &mut Check, n: &Node) -> Stmt {
    let id = push_target(cx, true);
    let saved = cx.scope;
    let cs = cx.enter(ScopeKind::Block);
    let live = cx.func.as_ref().map_or(0, |f| f.live.len());
    let cond = n.child("condition").and_then(|c| condition(cx, c, "LOOP-5", true));
    let body = n.child("statement").map(|b| sub_statement(cx, b, Some((cs, "LOOP-8"))));
    if let Some(f) = cx.func.as_mut() {
        f.live.truncate(live);
    }
    cx.scope = saved;
    pop_target(cx);
    match (cond, body) {
        (Some(cond), Some(body)) => Stmt::new(SK::While { id, cond, body: Box::new(body) }, n.span),
        _ => Stmt::new(SK::Null, n.span),
    }
}

/// `do_stmt` (LOOP-2, LOOP-5, LOOP-6).
pub fn do_stmt(cx: &mut Check, n: &Node) -> Stmt {
    let id = push_target(cx, true);
    let body = n.child("statement").map(|b| sub_statement(cx, b, None));
    pop_target(cx);
    let c = crate::expressions::check::nodes(n).into_iter().find(|c| c.prod != "statement");
    let cond = c.map(|c| {
        let e = expr(cx, c);
        conv::contextual_bool(cx, e, "LOOP-5")
    });
    match (cond, body) {
        (Some(cond), Some(body)) if !cond.ty.is_error() => Stmt::new(SK::Do { id, body: Box::new(body), cond }, n.span),
        _ => Stmt::new(SK::Null, n.span),
    }
}

/// `for_stmt` (LOOP-3, LOOP-5, LOOP-7, LOOP-8).
pub fn for_stmt(cx: &mut Check, n: &Node) -> Stmt {
    let saved = cx.scope;
    let cs = cx.enter(ScopeKind::Block);
    let live = cx.func.as_ref().map_or(0, |f| f.live.len());
    let labels = cx.func.as_mut().map(|f| std::mem::take(&mut f.pending_labels)).unwrap_or_default();
    let init = n.child("init_stmt").and_then(|i| init_stmt(cx, i));
    let cond = n.child("condition").and_then(|c| condition(cx, c, "LOOP-5", true));
    let step = crate::expressions::check::nodes(n).into_iter().find(|c| !matches!(c.prod, "init_stmt" | "condition" | "statement")).map(|s| expr(cx, s));
    if let Some(f) = cx.func.as_mut() {
        f.pending_labels = labels;
    }
    let id = push_target(cx, true);
    let body = n.child("statement").map(|b| sub_statement(cx, b, Some((cs, "LOOP-8"))));
    pop_target(cx);
    if let Some(f) = cx.func.as_mut() {
        f.live.truncate(live);
    }
    cx.scope = saved;
    let _ = first_node;
    let Some(body) = body else { return Stmt::new(SK::Null, n.span) };
    if n.child("condition").is_some() && cond.is_none() {
        return Stmt::new(SK::Null, n.span);
    }
    let step = match step {
        Some(e) if e.ty.is_error() => return Stmt::new(SK::Null, n.span),
        s => s,
    };
    Stmt::new(SK::For { id, init, cond, step, body: Box::new(body) }, n.span)
}
