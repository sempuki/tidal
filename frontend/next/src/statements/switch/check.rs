//! The check part of `statements/switch`: the condition and the `case`
//! and `default` labels (SW-2, SW-3, SW-5, SW-6).

use crate::foundations::constant_evaluation::check as ce;
use crate::foundations::conversions::check as conv;
use crate::sem::check::{Check, SwitchCtx};
use crate::sem::tree::{Cond, Stmt, SK};
use crate::sem::types::{QType, Type};
use crate::sem::ScopeKind;
use crate::statements::check::sub_statement;
use crate::statements::if_stmt::check::{condition, init_stmt};
use crate::statements::loops::check::{pop_target, push_target};
use crate::syntax::{Kid, Node};

/// `switch_stmt`.
pub fn switch_stmt(cx: &mut Check, n: &Node) -> Stmt {
    let saved = cx.scope;
    let cs = cx.enter(ScopeKind::Block);
    let live = cx.func.as_ref().map_or(0, |f| f.live.len());
    let labels = cx.func.as_mut().map(|f| std::mem::take(&mut f.pending_labels)).unwrap_or_default();
    let init = n.child("init_stmt").and_then(|i| init_stmt(cx, i));
    let mut cond = n.child("condition").and_then(|c| condition(cx, c, "SW-2", false));
    let mut ty = QType::error();
    if let Some(c) = &mut cond {
        let e = match c {
            Cond::Expr(e) | Cond::Decl(_, _, e) => e,
        };
        let ok = e.ty.is_integral() || e.ty.is_enum();
        if !ok {
            cx.error(e.span, "SW-2", format!("the condition of a switch has type {}", e.ty.show()));
            cond = None;
        } else {
            let p = conv::promoted(cx, &e.ty);
            if p.t != e.ty.t && !e.ty.is_enum() {
                let x = std::mem::replace(e, crate::sem::tree::Expr::int(0, QType::int(), n.span));
                *e = conv::promote(cx, x);
            }
            ty = if e.ty.is_enum() { e.ty.clone() } else { p };
        }
    }
    if let Some(f) = cx.func.as_mut() {
        f.pending_labels = labels;
    }
    let id = push_target(cx, false);
    let start = cx.func.as_ref().map_or(0, |f| f.live.len());
    if let Some(f) = cx.func.as_mut() {
        f.switches.push(SwitchCtx { id, ty, cases: Vec::new(), default: false, live_at_start: start });
    }
    let body = n.child("statement").map(|b| sub_statement(cx, b, Some((cs, "SW-6"))));
    let sw = cx.func.as_mut().and_then(|f| f.switches.pop());
    pop_target(cx);
    if let Some(f) = cx.func.as_mut() {
        f.live.truncate(live);
    }
    cx.scope = saved;
    match (cond, body, sw) {
        (Some(cond), Some(body), Some(sw)) => Stmt::new(SK::Switch { id, init, cond, body: Box::new(body), cases: sw.cases }, n.span),
        _ => Stmt::new(SK::Null, n.span),
    }
}

/// A `case` or `default` label: the statement that the caller fills
/// with the labeled statement, or None for an identifier label.
pub fn label_prefix(cx: &mut Check, l: &Node) -> Option<Stmt> {
    let w = cx.first_word(l);
    if w != "case" && w != "default" {
        return None;
    }
    let Some(sw) = cx.func.as_ref().and_then(|f| f.switches.last().cloned()) else {
        cx.error(l.span, "SW-3", format!("a '{}' label outside a switch", w));
        return Some(Stmt::new(SK::Null, l.span));
    };
    // SW-5: the transfer to the label must not skip the initialization
    // of a variable in scope at the label.
    let live = cx.func.as_ref().map(|f| f.live.len()).unwrap_or(0);
    if live > sw.live_at_start {
        cx.error(l.span, "SW-5", format!("the jump to the '{}' label skips the initialization of a variable", w));
    }
    if w == "default" {
        if sw.default {
            cx.error(l.span, "SW-3", "two default labels in one switch");
        }
        if let Some(s) = cx.func.as_mut().and_then(|f| f.switches.last_mut()) {
            s.default = true;
        }
        return Some(Stmt::new(SK::Case(sw.id, None, Box::new(Stmt::new(SK::Null, l.span))), l.span));
    }
    let e = l.kids.iter().find_map(|k| match k {
        Kid::Node(c) if c.prod != "attribute_spec" => Some(c),
        _ => None,
    })?;
    let x = crate::expressions::check::expr(cx, e);
    let x = crate::foundations::values::check::rvalue(cx, x);
    if x.ty.is_error() || sw.ty.is_error() {
        return Some(Stmt::new(SK::Null, l.span));
    }
    if let Type::Enum(en) = sw.ty.t {
        if cx.u.enums[en].scoped && x.ty.t != sw.ty.t {
            cx.error(l.span, "SW-3", "a case value of another type than the scoped enumeration");
            return Some(Stmt::new(SK::Null, l.span));
        }
    }
    if !(x.ty.is_integral() || x.ty.is_enum()) {
        cx.error_cause(l.span, "SW-3", Some("CE-11"), format!("a case value of type {}", x.ty.show()));
        return Some(Stmt::new(SK::Null, l.span));
    }
    let target = if let Type::Enum(en) = sw.ty.t { QType::new(Type::Int(cx.u.enums[en].underlying.unwrap_or(crate::sem::types::IntK::Int))) } else { sw.ty.clone() };
    if x.ty.t != sw.ty.t && conv::is_narrowing(cx, &x, &target) {
        cx.error_cause(l.span, "SW-3", Some("CE-19"), "the case value narrows to the type of the condition");
        return Some(Stmt::new(SK::Null, l.span));
    }
    let v = match ce::evaluate(cx, &x) {
        Ok(v) => v.int().unwrap_or(0),
        Err(why) => {
            ce::report(cx, l.span, "SW-3", why);
            return Some(Stmt::new(SK::Null, l.span));
        }
    };
    let v = match target.int_kind() {
        Some(k) => cx.u.target.wrap(k, v),
        None => v,
    };
    if sw.cases.contains(&v) {
        cx.error(l.span, "SW-3", format!("two case labels with the value {}", v));
        return Some(Stmt::new(SK::Null, l.span));
    }
    let k = sw.cases.len();
    if let Some(s) = cx.func.as_mut().and_then(|f| f.switches.last_mut()) {
        s.cases.push(v);
    }
    Some(Stmt::new(SK::Case(sw.id, Some(k), Box::new(Stmt::new(SK::Null, l.span))), l.span))
}
