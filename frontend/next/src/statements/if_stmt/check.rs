//! The check part of `statements/if`, with the condition and the
//! init-statement that `switch`, `while`, and `for` share.

use crate::expressions::check::{expr, first_node};
use crate::foundations::constant_evaluation::check as ce;
use crate::foundations::conversions::check as conv;
use crate::sem::check::Check;
use crate::sem::tree::{Cond, Stmt, SK};
use crate::sem::ScopeKind;
use crate::statements::check::sub_statement;
use crate::syntax::{Kid, Node};

/// An init-statement (IF-2, IF-21): a simple declaration, an alias
/// declaration, or an expression statement.
pub fn init_stmt(cx: &mut Check, n: &Node) -> Option<Box<Stmt>> {
    let c = first_node(n)?;
    let s = match c.prod {
        "simple_decl" => crate::declarations::check::simple_decl(cx, c).unwrap_or(Stmt::new(SK::Null, c.span)),
        "alias_decl" => {
            crate::declarations::type_names::check::alias_decl(cx, c);
            Stmt::new(SK::Null, c.span)
        }
        "expression_stmt" => crate::statements::blocks::check::expression_stmt(cx, c),
        "null_stmt" => Stmt::new(SK::Null, c.span),
        _ => {
            cx.unsupported(c.span, "a structured binding in an init-statement");
            Stmt::new(SK::Unsupported("a structured binding".into()), c.span)
        }
    };
    Some(Box::new(s))
}

/// A condition (IF-4, IF-6): an expression contextually converted to
/// `bool`, or a declaration with an initializer. For a switch, `as_bool`
/// is false and the value keeps its type.
pub fn condition(cx: &mut Check, n: &Node, rule: &'static str, as_bool: bool) -> Option<Cond> {
    let c = first_node(n)?;
    if c.prod == "cond_decl" {
        let (attrs, _) = crate::declarations::check::leading_attrs(cx, c);
        let ds = c.child("decl_specifiers")?;
        let specs = crate::declarations::check::decl_specifiers(cx, ds, attrs);
        if specs.is_static || specs.is_extern || specs.thread_local || specs.typedef || specs.defines_type {
            cx.error(c.span, "IF-6", "a condition declaration with a specifier other than a type specifier");
            return None;
        }
        let dn = c.child("declarator")?;
        let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
        if d.ty.is_array() || d.ty.is_func() {
            cx.error(c.span, "IF-6", "a condition declaration of an array or a function");
            return None;
        }
        let after: Vec<Kid> = c.kids.iter().skip_while(|k| !matches!(k, Kid::Node(x) if x.prod == "declarator")).skip(1).cloned().collect();
        let init = Node { prod: "init", ext: None, span: c.span, kids: after };
        let mut first = None;
        let (id, i) = crate::declarations::initialization::forms::check::declare_variable(cx, &specs, &d, Some(&init), &[], &mut first)?;
        if let Some(v) = cx.u.var_mut(id) {
            v.used = true;
        }
        // JMP-6: a jump into the body skips the initialization.
        if let Some(f) = cx.func.as_mut() {
            f.live.push(id);
        }
        let ty = cx.u.var(id)?.ty.clone();
        let (name, span) = d.name.clone()?;
        let mut e = crate::sem::tree::Expr::new(crate::sem::tree::EK::Var(id), ty.nonref(), crate::sem::tree::Cat::L, span);
        let _ = name;
        e = if as_bool { conv::contextual_bool(cx, e, rule) } else { crate::foundations::values::check::rvalue(cx, e) };
        if e.ty.is_error() {
            return None;
        }
        return Some(Cond::Decl(id, i, e));
    }
    let e = expr(cx, c);
    if e.ty.is_error() {
        return None;
    }
    let e = if as_bool { conv::contextual_bool(cx, e, rule) } else { crate::foundations::values::check::rvalue(cx, e) };
    if e.ty.is_error() {
        return None;
    }
    Some(Cond::Expr(e))
}

/// `if consteval` and `if !consteval` (IF-18, IF-19, IF-24): the branch
/// that runs in constant evaluation is an immediate function context.
/// No jump enters a branch (IF-17).
fn if_consteval(cx: &mut Check, n: &Node, negated: bool) -> Stmt {
    let id = cx.next_stmt_id();
    if let Some(f) = cx.func.as_mut() {
        f.constexpr_ifs.push(id);
    }
    let branches: Vec<&Node> = n.children("block").collect();
    let mut out = Vec::new();
    for (k, b) in branches.iter().enumerate() {
        let constant = (k == 0) != negated;
        if let Some(f) = cx.func.as_mut() {
            if constant {
                f.immediate += 1;
            } else {
                f.runtime += 1;
            }
        }
        let s = match b.child("compound_stmt") {
            Some(cs) => crate::statements::blocks::check::compound_stmt(cx, cs, None),
            None => Stmt::new(SK::Null, b.span),
        };
        out.push(s);
        if let Some(f) = cx.func.as_mut() {
            if constant {
                f.immediate -= 1;
            } else {
                f.runtime -= 1;
            }
        }
    }
    if let Some(f) = cx.func.as_mut() {
        f.constexpr_ifs.pop();
    }
    let mut out = out.into_iter();
    let first = out.next().unwrap_or(Stmt::new(SK::Null, n.span));
    let second = out.next().unwrap_or(Stmt::new(SK::Null, n.span));
    let (constant, runtime) = if negated { (second, first) } else { (first, second) };
    Stmt::new(SK::Consteval { constant: Box::new(constant), runtime: Box::new(runtime) }, n.span)
}

/// `if_stmt` (IF-1 to IF-10, IF-16, IF-20, IF-23).
pub fn if_stmt(cx: &mut Check, n: &Node) -> Stmt {
    let words = cx.words(n);
    if words.contains(&"consteval") {
        return if_consteval(cx, n, words.contains(&"!"));
    }
    let constexpr = words.contains(&"constexpr");
    let saved = cx.scope;
    let cs = cx.enter(ScopeKind::Block);
    let live = cx.func.as_ref().map_or(0, |f| f.live.len());
    let init = n.child("init_stmt").and_then(|i| init_stmt(cx, i));
    let cond_node = n.child("condition");
    let mut chosen = None;
    let cond = if constexpr {
        let c = cond_node.and_then(first_node);
        match c {
            Some(c) if c.prod != "cond_decl" => {
                let e = expr(cx, c);
                let b = conv::contextual_bool(cx, e, "IF-16");
                if !b.ty.is_error() {
                    match ce::evaluate(cx, &b) {
                        Ok(v) => chosen = Some(v.truthy()),
                        Err(why) => ce::report(cx, b.span, "IF-16", why),
                    }
                }
                None
            }
            _ => {
                cx.error(n.span, "IF-16", "the condition of 'if constexpr' must be an expression");
                None
            }
        }
    } else {
        cond_node.and_then(|c| condition(cx, c, "IF-4", true))
    };
    let branches: Vec<&Node> = n.children("branch").collect();
    let discard = |cx: &mut Check, on: bool, d: i32| {
        if let Some(f) = cx.func.as_mut().filter(|_| on) {
            f.discarded = (f.discarded as i32 + d) as u32;
        }
    };
    let (skip_then, skip_else) = (constexpr && chosen == Some(false), constexpr && chosen == Some(true));
    if constexpr {
        let id = cx.next_stmt_id();
        if let Some(f) = cx.func.as_mut() {
            f.constexpr_ifs.push(id);
        }
    }
    discard(cx, skip_then, 1);
    let then = branches.first().map(|b| sub_statement(cx, b, Some((cs, "IF-10"))));
    discard(cx, skip_then, -1);
    discard(cx, skip_else, 1);
    let els = branches.get(1).map(|b| sub_statement(cx, b, Some((cs, "IF-10"))));
    discard(cx, skip_else, -1);
    if let Some(f) = cx.func.as_mut() {
        if constexpr {
            f.constexpr_ifs.pop();
        }
        f.live.truncate(live);
    }
    cx.scope = saved;
    let then = then.unwrap_or(Stmt::new(SK::Null, n.span));
    if constexpr {
        let pick = match chosen {
            Some(true) => Some(Box::new(then)),
            Some(false) => els.map(Box::new),
            None => return Stmt::new(SK::Null, n.span),
        };
        let chosen_stmt = Stmt::new(SK::Chosen(pick), n.span);
        return match init {
            Some(i) => Stmt::new(SK::Block(vec![*i, chosen_stmt]), n.span),
            None => chosen_stmt,
        };
    }
    let Some(cond) = cond else { return Stmt::new(SK::Null, n.span) };
    Stmt::new(SK::If { init, cond, then: Box::new(then), els: els.map(Box::new) }, n.span)
}
