//! The check part of `statements/blocks`: compound statements, labels,
//! and expression statements.

use crate::expressions::check::{expr, first_node};
use crate::sem::check::Check;
use crate::sem::tree::{Expr, Stmt, EK, SK};
use crate::sem::{EntKind, ScopeId, ScopeKind};
use crate::statements::check::statement;
use crate::syntax::{Kid, Node};
use std::rc::Rc;

/// The statements of a compound statement, in the current scope.
pub fn items(cx: &mut Check, n: &Node, _cond: Option<(ScopeId, &'static str)>) -> Stmt {
    let mut out = Vec::new();
    let mut pending: Vec<Node> = Vec::new();
    for k in &n.kids {
        let Kid::Node(c) = k else { continue };
        match c.prod {
            "statement" => {
                let s = statement(cx, c);
                out.push(s);
            }
            // A label at the end of the block labels no statement
            // (STMT-5): a null statement carries it.
            "label" => pending.push(c.clone()),
            _ => {}
        }
    }
    for l in pending {
        let s = label_on(cx, &l, Stmt::new(SK::Null, l.span));
        out.push(s);
    }
    Stmt::new(SK::Block(out), n.span)
}

/// `compound_stmt`: a block with its own scope (STMT-1, STMT-2).
pub fn compound_stmt(cx: &mut Check, n: &Node, cond: Option<(ScopeId, &'static str)>) -> Stmt {
    if n.ext.is_some() {
        return statement(cx, n);
    }
    let saved = cx.scope;
    let s = cx.enter(ScopeKind::Block);
    let live = cx.func.as_ref().map_or(0, |f| f.live.len());
    let b = items(cx, n, cond);
    if let Some((cs, rule)) = cond {
        redeclared_from(cx, cs, s, rule);
    }
    unused_variables(cx, s);
    if let Some(f) = cx.func.as_mut() {
        f.live.truncate(live);
    }
    cx.scope = saved;
    b
}

/// A name of a condition's scope declared again in the outermost block
/// of the body: the statement's rule (IF-10, LOOP-8, SW-6, RFOR-6).
pub fn redeclared_from(cx: &mut Check, cond: ScopeId, body: ScopeId, rule: &'static str) {
    let names: Vec<(Rc<str>, Vec<usize>)> = cx.u.scopes[body].names.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    for (name, ids) in names {
        if cx.u.scopes[cond].names.contains_key(&name) {
            let span = cx.u.entities[ids[0]].span;
            cx.error(span, rule, format!("'{}' declared in the condition is declared again in the body", name));
        }
    }
}

/// ATTR-6: the variables of a block that the program never reads.
fn unused_variables(cx: &mut Check, s: ScopeId) {
    let ids: Vec<usize> = cx.u.scopes[s].names.values().flatten().copied().collect();
    for id in ids {
        let read = crate::declarations::initialization::forms::check::is_read(cx, id);
        if let EntKind::Var(v) = &cx.u.entities[id].kind {
            if !read && !v.maybe_unused && !v.param && v.storage == crate::sem::Storage::Automatic && !v.ty.is_error() {
                let (name, span) = (cx.u.entities[id].name.clone(), cx.u.entities[id].span);
                crate::declarations::attributes::check::unused_variable(cx, span, &name);
            }
        }
    }
}

/// `labeled_stmt`.
pub fn labeled_stmt(cx: &mut Check, n: &Node) -> Stmt {
    let Some(l) = n.child("label") else { return Stmt::new(SK::Null, n.span) };
    let inner = n.child("statement");
    // An identifier label marks the statement after it, also through
    // further labels (the target of a labeled jump in a proposal).
    if let Some(name) = label_name(cx, l) {
        if let Some(f) = cx.func.as_mut() {
            f.pending_labels.push(name);
        }
    }
    let live = cx.func.as_ref().map(|f| f.live.clone()).unwrap_or_default();
    let s = match inner {
        Some(i) => {
            let x = crate::statements::switch::check::label_prefix(cx, l);
            let s = statement(cx, i);
            return wrap(cx, l, x, s, live);
        }
        None => Stmt::new(SK::Null, n.span),
    };
    label_on(cx, l, s)
}

fn label_name(cx: &Check, l: &Node) -> Option<Rc<str>> {
    match cx.first_word(l) {
        "case" | "default" => None,
        _ => l.kids.iter().find_map(|k| match k {
            Kid::Tok(_) if cx.text(k) != ":" => Some(Rc::from(cx.text(k))),
            _ => None,
        }),
    }
}

/// A label: an identifier label (JMP-14, NAM-11), or a `case` or
/// `default` label of the enclosing switch.
fn label_on(cx: &mut Check, l: &Node, s: Stmt) -> Stmt {
    let x = crate::statements::switch::check::label_prefix(cx, l);
    let live = cx.func.as_ref().map(|f| f.live.clone()).unwrap_or_default();
    wrap(cx, l, x, s, live)
}

fn wrap(cx: &mut Check, l: &Node, case: Option<Stmt>, s: Stmt, live: Vec<usize>) -> Stmt {
    if let Some(mut c) = case {
        if let SK::Case(_, _, inner) = &mut c.k {
            *inner = Box::new(s);
        }
        return c;
    }
    let Some(name) = label_name(cx, l) else { return s };
    if let Some(f) = cx.func.as_mut() {
        if f.labels.contains_key(&name) {
            cx.error(l.span, "JMP-14", format!("the label '{}' is defined twice in the function", name));
        } else {
            let ifs = f.constexpr_ifs.clone();
            f.labels.insert(name.clone(), (l.span, live, ifs));
        }
    }
    Stmt::new(SK::Label(name, Box::new(s)), l.span)
}

/// `expression_stmt`. ATTR-5: a discarded call of a `nodiscard`
/// function.
pub fn expression_stmt(cx: &mut Check, n: &Node) -> Stmt {
    let Some(c) = first_node(n) else { return Stmt::new(SK::Null, n.span) };
    let e = expr(cx, c);
    nodiscard(cx, &e);
    if crate::expressions::primary::check::unresolved_set(&e) {
        crate::expressions::primary::check::no_target(cx, &e, "OVL-20");
    }
    if e.ty.is_error() {
        return Stmt::new(SK::Null, n.span);
    }
    crate::expressions::decltype::check::no_effect_warning(cx, &e, "STMT-4", "the expression statement");
    Stmt::new(SK::Expr(e), n.span)
}

fn nodiscard(cx: &mut Check, e: &Expr) {
    let EK::Call(f, _) = &e.k else { return };
    let EK::Func(id) = f.k else { return };
    if crate::declarations::functions::definitions::check::is_nodiscard(cx, id) && !e.ty.is_void() {
        let name = cx.u.entities[id].name.clone();
        crate::declarations::attributes::check::discarded_nodiscard(cx, e.span, &name);
    }
}

/// `[[assume(e)]];` (ATTR-9, ATTR-13): the expression converts to
/// `bool`; at run time it must hold (ATTR-10).
pub fn assume(cx: &mut Check, args: &Node) -> Stmt {
    let span = args.span;
    let Some(c) = first_node(args).filter(|c| c.prod != "balanced_token") else {
        cx.unsupported(span, "the expression of an assumption (its tokens are not parsed as an expression)");
        return Stmt::new(SK::Unsupported("an assumption".into()), span);
    };
    let e = expr(cx, c);
    if e.ty.is_error() {
        return Stmt::new(SK::Null, span);
    }
    // The operand is a conditional expression, so a comma needs
    // parentheses.
    if matches!(e.k, EK::Comma(..)) && !e.paren {
        cx.error(span, "ATTR-13", "the expression of an assumption is a comma expression");
        return Stmt::new(SK::Null, span);
    }
    let b = crate::foundations::conversions::check::contextual_bool(cx, e, "ATTR-9");
    if b.ty.is_error() {
        return Stmt::new(SK::Null, span);
    }
    Stmt::new(SK::Expr(Expr::new(EK::Assume(Box::new(b)), crate::sem::types::QType::void(), crate::sem::tree::Cat::PR, span)), span)
}
