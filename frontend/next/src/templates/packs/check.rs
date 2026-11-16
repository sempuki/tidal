//! `templates/packs`, the Check segment: pack expansions in a
//! specialization (build step 5).

use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::tree::Expr;
use crate::sem::{EntKind, Linkage, ScopeKind};
use crate::syntax::{Kid, Node};
use std::rc::Rc;

/// The packs that a pattern names, with their elements: a function
/// parameter pack's parameters, or a type pack's types.
enum Elems {
    Vars(Vec<crate::sem::EntityId>),
    Types(Vec<crate::sem::types::QType>),
}

impl Elems {
    fn len(&self) -> usize {
        match self {
            Elems::Vars(v) => v.len(),
            Elems::Types(t) => t.len(),
        }
    }
}

/// The bound packs that the tokens of `n` name.
fn packs_in(cx: &Check, n: &Node) -> Vec<(Rc<str>, Elems)> {
    let mut toks = Vec::new();
    n.tokens(&mut toks);
    let mut out: Vec<(Rc<str>, Elems)> = Vec::new();
    for &t in &toks {
        let w = cx.toks[t].text.as_str();
        if out.iter().any(|(m, _)| m.as_ref() == w) {
            continue;
        }
        match names::lookup(cx, w).first().map(|&e| cx.u.entities[e].kind.clone()) {
            Some(EntKind::Pack(ids)) => out.push((w.into(), Elems::Vars(ids))),
            Some(EntKind::TypePack(ts)) => out.push((w.into(), Elems::Types(ts))),
            _ => {}
        }
    }
    out
}

/// The number of elements of the pack named `name`, if it is one.
pub fn pack_len(cx: &Check, name: &str) -> Option<usize> {
    match names::lookup(cx, name).first().map(|&e| &cx.u.entities[e].kind) {
        Some(EntKind::Pack(ids)) => Some(ids.len()),
        Some(EntKind::TypePack(ts)) => Some(ts.len()),
        _ => None,
    }
}

/// PACK rules: `f(n)` for each element of the expansion whose pattern is
/// `pattern`, with each pack that the pattern names bound to that
/// element. None when the pattern names no bound pack, or packs of
/// different lengths.
pub fn expand<T>(cx: &mut Check, pattern: &Node, mut f: impl FnMut(&mut Check, &Node) -> T) -> Option<Vec<T>> {
    let packs = packs_in(cx, pattern);
    let n = packs.first()?.1.len();
    if packs.iter().any(|(_, e)| e.len() != n) {
        return None;
    }
    let mut out = Vec::new();
    for i in 0..n {
        let saved = cx.scope;
        let s = cx.enter(ScopeKind::Block);
        for (name, e) in &packs {
            match e {
                // The element is the parameter itself, under the pack's name.
                Elems::Vars(ids) => cx.u.scopes[s].names.entry(name.clone()).or_default().push(ids[i]),
                Elems::Types(ts) => {
                    names::declare_in(cx, s, name, EntKind::Typedef(ts[i].clone()), pattern.span, Linkage::None);
                }
            }
        }
        let saved_expanding = std::mem::replace(&mut cx.expanding, true);
        out.push(f(cx, pattern));
        cx.expanding = saved_expanding;
        cx.leave(saved);
    }
    Some(out)
}

/// The arguments of a call or an initializer, the `argument` nodes of
/// `list`: each argument, or, for an argument that a `...` follows, the
/// expressions of its expansion. None after an expansion that the front
/// end does not model.
pub fn arguments(cx: &mut Check, list: Option<&Node>) -> Option<Vec<Expr>> {
    let mut out = Vec::new();
    let Some(list) = list else { return Some(out) };
    let kids = &list.kids;
    for (k, kid) in kids.iter().enumerate() {
        let Kid::Node(a) = kid else { continue };
        if a.prod != "argument" {
            continue;
        }
        let expansion = matches!(kids.get(k + 1), Some(t @ Kid::Tok(_)) if cx.text(t) == "...");
        if !expansion {
            out.push(crate::expressions::check::expr(cx, a));
            continue;
        }
        out.extend(expand(cx, a, |cx, p| crate::expressions::check::expr(cx, p))?);
    }
    Some(out)
}

/// PRIM-27: a fold `( E op ... )`, `( ... op E )`, `( E op ... op I )`, or
/// `( I op ... op E )`: the operator applied between the elements of the
/// expansion of `E`, right to left for a right fold and left to right
/// for a left fold, with `I` at the far end. An empty unary fold of `&&`
/// is `true`, of `||` `false`, and of `,` `void()` (PACK-12).
pub fn fold_expr(cx: &mut Check, n: &Node) -> Expr {
    use crate::sem::tree::{Cat, EK};
    let span = n.span;
    let unsupported = |cx: &mut Check, why: &str| {
        let t = cx.unsupported(span, why.to_string());
        Expr::new(EK::Int(0), t, Cat::PR, span)
    };
    let kids: Vec<&Kid> = n.kids.iter().filter(|k| !matches!(k, Kid::Tok(_)) || cx.text(k) == "...").collect();
    let Some(ell) = kids.iter().position(|k| matches!(k, Kid::Tok(_))) else { return unsupported(cx, "a fold of this form") };
    let node = |k: &&Kid| match k {
        Kid::Node(c) => Some(c.clone()),
        _ => None,
    };
    let ops: Vec<Node> = kids.iter().filter_map(node).filter(|c| c.prod == "fold_op").collect();
    let operands: Vec<Node> = kids.iter().filter_map(node).filter(|c| c.prod != "fold_op").collect();
    let Some(op) = ops.first() else { return unsupported(cx, "a fold of this form") };
    let Some(&op_tok) = op.kids.iter().find_map(|k| match k {
        Kid::Tok(t) => Some(t),
        _ => None,
    }) else {
        return unsupported(cx, "a fold of this form");
    };
    let text = cx.toks[op_tok].text.clone();
    // PRIM-29: the two operators of a binary fold are the same.
    if ops.len() == 2 && cx.source(&ops[0]) != cx.source(&ops[1]) {
        cx.error(span, "PRIM-29", format!("a fold whose two operators '{}' and '{}' differ", cx.source(&ops[0]), cx.source(&ops[1])));
        return Expr::new(EK::Int(0), crate::sem::types::QType::error(), Cat::PR, span);
    }
    // A pattern before `...` makes a right fold; the other operand of a
    // binary fold is its initial value.
    let first_is_operand = kids.first().is_some_and(|k| matches!(k, Kid::Node(c) if c.prod != "fold_op"));
    let (pattern, init, right) = match operands.as_slice() {
        [e] => (e.clone(), None, first_is_operand && ell > 0),
        [a, b] => {
            if !packs_in(cx, a).is_empty() {
                (a.clone(), Some(b.clone()), true)
            } else {
                (b.clone(), Some(a.clone()), false)
            }
        }
        _ => return unsupported(cx, "a fold of this form"),
    };
    let Some(mut es) = expand(cx, &pattern, |cx, p| crate::expressions::check::expr(cx, p)) else { return unsupported(cx, "a fold whose pattern names no pack that the front end binds") };
    if let Some(i) = &init {
        let x = crate::expressions::check::expr(cx, i);
        if right {
            es.push(x);
        } else {
            es.insert(0, x);
        }
    }
    if es.is_empty() {
        return match text.as_str() {
            "&&" => Expr::int(1, crate::sem::types::QType::bool(), span),
            "||" => Expr::int(0, crate::sem::types::QType::bool(), span),
            "," => Expr::new(EK::Int(0), crate::sem::types::QType::void(), Cat::PR, span),
            _ => {
                cx.error(span, "PACK-12", format!("an empty unary fold of '{}'", text));
                crate::expressions::check::bad(span)
            }
        };
    }
    let prod = match text.as_str() {
        "+" | "-" => "add_expr",
        "*" | "/" | "%" => "mul_expr",
        "<<" | ">>" => "shift_expr",
        "<" | ">" | "<=" | ">=" => "relational_expr",
        "==" | "!=" => "equality_expr",
        "&" => "and_expr",
        "^" => "xor_expr",
        "|" => "or_expr",
        "&&" => "logical_and_expr",
        "||" => "logical_or_expr",
        "," => "expression",
        _ => return unsupported(cx, "a fold of this operator"),
    };
    let combine = |cx: &mut Check, a: Expr, b: Expr| -> Expr {
        let mut a = a;
        let mut b = b;
        a.paren = true;
        b.paren = true;
        let (sa, sb) = (a.span, b.span);
        let syn = Node { prod, ext: None, span, kids: vec![Kid::Node(prechecked(cx, a, sa)), Kid::Tok(op_tok), Kid::Node(prechecked(cx, b, sb))] };
        crate::expressions::check::expr(cx, &syn)
    };
    let mut it = es.into_iter();
    let mut acc;
    if right {
        let mut rest: Vec<Expr> = it.collect();
        acc = rest.pop().unwrap();
        while let Some(x) = rest.pop() {
            acc = combine(cx, x, acc);
        }
    } else {
        acc = it.next().unwrap();
        for x in it {
            acc = combine(cx, acc, x);
        }
    }
    acc.paren = true;
    acc
}

/// A node that stands for the checked expression `e`, which `expr`
/// takes once (PACK rules).
pub fn prechecked(cx: &mut Check, e: Expr, span: crate::source::Span) -> Node {
    cx.prechecked.push(Some(e));
    Node { prod: "prechecked", ext: None, span, kids: vec![Kid::Deferred(cx.prechecked.len() - 1)] }
}

/// PACK rules: `n` with each argument `E...` of its parenthesized or
/// braced list replaced by the arguments of the expansion, checked
/// already. None when the list holds no expansion, or one that the
/// front end does not model.
pub fn rewrite(cx: &mut Check, n: &Node) -> Option<Node> {
    let k = n.kids.iter().position(|k| matches!(k, Kid::Node(c) if matches!(c.prod, "expression_list" | "braced_init") && has_expansion(cx, c)))?;
    let Kid::Node(list) = &n.kids[k] else { return None };
    let new_list = rewrite_list(cx, list)?;
    let mut out = n.clone();
    out.kids[k] = Kid::Node(new_list);
    Some(out)
}

/// A list's `...` after an argument or an initializer clause.
fn has_expansion(cx: &Check, list: &Node) -> bool {
    list.kids.windows(2).any(|w| matches!(&w[0], Kid::Node(c) if matches!(c.prod, "argument" | "init_clause")) && matches!(&w[1], t @ Kid::Tok(_) if cx.text(t) == "..."))
}

/// The list `list` with its expansions replaced.
pub fn rewrite_list(cx: &mut Check, list: &Node) -> Option<Node> {
    let mut kids = Vec::new();
    let mut i = 0;
    while i < list.kids.len() {
        let kid = &list.kids[i];
        let next_ell = matches!(list.kids.get(i + 1), Some(t @ Kid::Tok(_)) if cx.text(t) == "...");
        match kid {
            Kid::Node(a) if next_ell && matches!(a.prod, "argument" | "init_clause") => {
                let es = expand(cx, a, |cx, p| crate::expressions::check::expr(cx, p))?;
                for e in es {
                    let span = e.span;
                    let p = prechecked(cx, e, span);
                    kids.push(Kid::Node(Node { prod: a.prod, ext: None, span: a.span, kids: vec![Kid::Node(p)] }));
                }
                i += 2;
            }
            _ => {
                kids.push(kid.clone());
                i += 1;
            }
        }
    }
    Some(Node { prod: list.prod, ext: list.ext, span: list.span, kids })
}
