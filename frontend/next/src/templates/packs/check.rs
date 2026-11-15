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
