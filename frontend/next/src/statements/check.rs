//! The analysis of a statement node: each production goes to the check
//! part of its page. The chapter's index page has no rules, so this
//! module names none. A statement that a proposal's alternative made
//! goes to the proposal's `Ext` check (`DESIGN-rewrite.md` section 7).

use crate::expressions::check::first_node;
use crate::sem::check::Check;
use crate::sem::tree::{Stmt, SK};
use crate::statements::{blocks, if_stmt, jumps, loops, range_for, switch};
use crate::syntax::{Kid, Node};

/// The body of a function: its statements in the function's scope,
/// which the outermost block shares with the parameters (FN-1).
pub fn function_body(cx: &mut Check, n: &Node) -> Stmt {
    let name = cx.func.as_ref().and_then(|f| f.id).map(|id| cx.u.entities[id].name.to_string()).unwrap_or_default();
    crate::declarations::functions::definitions::check::func_name_var(cx, &name);
    blocks::check::items(cx, n, None)
}

/// `statement`.
pub fn statement(cx: &mut Check, n: &Node) -> Stmt {
    if n.ext.is_some() {
        return ext(cx, n);
    }
    let mut attrs = crate::declarations::attributes::check::Attrs::default();
    let mut inner = None;
    for k in &n.kids {
        if let Kid::Node(c) = k {
            match c.prod {
                "attribute_spec" => crate::declarations::attributes::check::attribute_spec(cx, c, &mut attrs),
                _ => inner = Some(c),
            }
        }
    }
    let Some(c) = inner else { return Stmt::new(SK::Null, n.span) };
    // ATTR-2: `fallthrough` is a null statement of its own.
    let null = c.prod == "plain_stmt" && first_node(c).is_some_and(|x| x.prod == "null_stmt");
    if attrs.fallthrough && !null {
        cx.error(n.span, "ATTR-2", "'fallthrough' applies only to a null statement");
    }
    // The labels of a chain mark a loop or switch only if it follows.
    let marks_target = c.prod == "labeled_stmt"
        || (c.prod == "plain_stmt" && first_node(c).is_some_and(|x| matches!(x.prod, "while_stmt" | "do_stmt" | "for_stmt" | "range_for_stmt" | "switch_stmt")));
    if !marks_target {
        if let Some(f) = cx.func.as_mut() {
            f.pending_labels.clear();
        }
    }
    match c.prod {
        "labeled_stmt" => blocks::check::labeled_stmt(cx, c),
        "block_decl" => {
            if attrs.any {
                if attrs.assume.is_some() {
                    cx.error(n.span, "ATTR-13", "an assumption applied to a declaration");
                }
            }
            match crate::declarations::check::block_decl(cx, c, true) {
                Some(s) => s,
                None => Stmt::new(SK::Null, c.span),
            }
        }
        "plain_stmt" => plain_stmt(cx, c, &attrs),
        _ => {
            cx.unsupported(c.span, format!("a statement of the form {}", c.prod));
            Stmt::new(SK::Unsupported(format!("a statement of the form {}", c.prod)), c.span)
        }
    }
}

/// A statement node of a proposal's alternative.
fn ext(cx: &mut Check, n: &Node) -> Stmt {
    match cx.hooks.ext_stmt {
        Some(f) => f(cx, n),
        None => {
            cx.unsupported(n.span, "a proposal's statement without its check");
            Stmt::new(SK::Unsupported("a proposal's statement".into()), n.span)
        }
    }
}

/// `plain_stmt`.
pub fn plain_stmt(cx: &mut Check, n: &Node, attrs: &crate::declarations::attributes::check::Attrs) -> Stmt {
    let Some(c) = first_node(n) else { return Stmt::new(SK::Null, n.span) };
    if c.ext.is_some() {
        return ext(cx, c);
    }
    match c.prod {
        "expression_stmt" => blocks::check::expression_stmt(cx, c),
        "null_stmt" => {
            if let Some(a) = &attrs.assume {
                return blocks::check::assume(cx, a);
            }
            if attrs.fallthrough {
                return Stmt::new(SK::Fallthrough, c.span);
            }
            Stmt::new(SK::Null, c.span)
        }
        "compound_stmt" => blocks::check::compound_stmt(cx, c, None),
        "if_stmt" => if_stmt::check::if_stmt(cx, c),
        "switch_stmt" => switch::check::switch_stmt(cx, c),
        "while_stmt" => loops::check::while_stmt(cx, c),
        "do_stmt" => loops::check::do_stmt(cx, c),
        "for_stmt" => loops::check::for_stmt(cx, c),
        "range_for_stmt" => range_for::check::range_for_stmt(cx, c),
        "jump_stmt" => jumps::check::jump_stmt(cx, c),
        _ => {
            cx.unsupported(c.span, format!("a statement of the form {} (build step 4)", c.prod));
            if let Some(f) = cx.func.as_mut() {
                f.unsupported = Some(format!("a statement of the form {}", c.prod));
            }
            Stmt::new(SK::Unsupported(format!("a statement of the form {}", c.prod)), c.span)
        }
    }
}

/// A sub-statement (a branch or a loop body) in a scope of its own
/// (IF-8, LOOP-4): a block if it is not one already. `cond` is the scope
/// of the statement's condition and the rule that forbids a
/// redeclaration of its names in the outermost block of the body.
pub fn sub_statement(cx: &mut Check, n: &Node, cond: Option<(usize, &'static str)>) -> Stmt {
    let inner = match n.prod {
        "branch" => first_node(n).unwrap_or(n),
        _ => n,
    };
    if let Some(cs) = inner.find_direct_compound() {
        if inner.ext.is_none() {
            return blocks::check::compound_stmt(cx, cs, cond);
        }
    }
    let saved = cx.scope;
    cx.enter(crate::sem::ScopeKind::Block);
    let live = cx.func.as_ref().map_or(0, |f| f.live.len());
    let s = statement(cx, inner);
    if let Some((cs, rule)) = cond {
        blocks::check::redeclared_from(cx, cs, cx.scope, rule);
    }
    if let Some(f) = cx.func.as_mut() {
        f.live.truncate(live);
    }
    cx.scope = saved;
    s
}

impl Node {
    /// The compound statement that a `statement` node is, if it is one.
    pub fn find_direct_compound(&self) -> Option<&Node> {
        let ps = match self.prod {
            "statement" => self.kids.iter().find_map(|k| match k {
                Kid::Node(c) if c.prod == "plain_stmt" => Some(c),
                _ => None,
            })?,
            "plain_stmt" => self,
            _ => return None,
        };
        ps.child("compound_stmt")
    }
}
