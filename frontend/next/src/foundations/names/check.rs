//! The check part of `foundations/names`: unqualified and qualified
//! lookup over the scopes of `sem`, and the declaration of a name in a
//! scope. Classes and their bases come with step 4.

use crate::sem::check::Check;
use crate::sem::{EntKind, Entity, EntityId, Linkage, ScopeId, ScopeKind};
use crate::source::Span;
use crate::syntax::{Kid, Node};
use std::rc::Rc;

/// What a qualifier names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qual {
    None,
    Scope(ScopeId),
    /// A qualifier the front end does not model (a class); the name it
    /// qualifies is unsupported.
    Unsupported,
    /// A qualifier that names nothing: the error was reported.
    Error,
}

/// The entities a scope declares under a name, with those of the
/// namespaces that a using-directive, an inline namespace, or an unnamed
/// namespace makes visible (NAM-23, NAM-25).
fn in_scope(cx: &Check, s: ScopeId, name: &str, seen: &mut Vec<ScopeId>) -> Vec<EntityId> {
    if seen.contains(&s) {
        return Vec::new();
    }
    seen.push(s);
    let mut out: Vec<EntityId> = cx.u.scopes[s].names.get(name).cloned().unwrap_or_default();
    for &u in &cx.u.scopes[s].using.clone() {
        for e in in_scope(cx, u, name, seen) {
            if !out.contains(&e) {
                out.push(e);
            }
        }
    }
    out
}

/// The nearest scope that encloses both scopes.
fn common(cx: &Check, a: ScopeId, b: ScopeId) -> ScopeId {
    let mut up = Vec::new();
    let mut s = Some(a);
    while let Some(x) = s {
        up.push(x);
        s = cx.u.scopes[x].parent;
    }
    let mut s = Some(b);
    while let Some(x) = s {
        if up.contains(&x) {
            return x;
        }
        s = cx.u.scopes[x].parent;
    }
    0
}

/// NAM-1, NAM-2, NAM-23: unqualified lookup. The innermost scope that
/// declares the name gives its entities. A using-directive makes the
/// members of its namespace visible as if they were declared in the
/// nearest scope that encloses both the directive and the namespace.
pub fn lookup(cx: &Check, name: &str) -> Vec<EntityId> {
    let mut nominated: Vec<(ScopeId, ScopeId)> = Vec::new();
    let mut s = Some(cx.scope);
    while let Some(sc) = s {
        // The directives of this scope, with the directives inside the
        // namespaces they name.
        let mut work: Vec<ScopeId> = cx.u.scopes[sc].using.clone();
        while let Some(n) = work.pop() {
            if nominated.iter().any(|&(m, _)| m == n) {
                continue;
            }
            nominated.push((n, common(cx, sc, n)));
            work.extend(cx.u.scopes[n].using.iter().copied());
        }
        let mut found: Vec<EntityId> = cx.u.scopes[sc].names.get(name).cloned().unwrap_or_default();
        for &(n, at) in &nominated {
            if at == sc {
                for e in cx.u.scopes[n].names.get(name).cloned().unwrap_or_default() {
                    if !found.contains(&e) {
                        found.push(e);
                    }
                }
            }
        }
        if !found.is_empty() {
            return found;
        }
        s = cx.u.scopes[sc].parent;
    }
    Vec::new()
}

/// NAM-13, NAM-19: lookup of `x` in the scope a qualifier names: its
/// own declarations first, then the namespaces that its using-directives
/// and inline or unnamed namespaces name.
pub fn lookup_in(cx: &Check, s: ScopeId, name: &str) -> Vec<EntityId> {
    if let Some(own) = cx.u.scopes[s].names.get(name) {
        if !own.is_empty() {
            return own.clone();
        }
    }
    in_scope(cx, s, name, &mut Vec::new())
}

/// Unqualified lookup that finds only the entities `keep` accepts, as
/// the name before `::` finds only namespaces and types (NAM-15).
pub fn lookup_where(cx: &Check, name: &str, keep: impl Fn(&EntKind) -> bool) -> Vec<EntityId> {
    let mut s = Some(cx.scope);
    while let Some(sc) = s {
        let found: Vec<EntityId> = in_scope(cx, sc, name, &mut Vec::new()).into_iter().filter(|&e| keep(&cx.u.entities[e].kind)).collect();
        if !found.is_empty() {
            return found;
        }
        s = cx.u.scopes[sc].parent;
    }
    Vec::new()
}

/// NAM-26: names that using-directives make visible for different
/// entities, not all functions, are ambiguous.
pub fn check_ambiguous(cx: &mut Check, found: &[EntityId], name: &str, span: Span) -> bool {
    if found.len() > 1 && !found.iter().all(|&e| matches!(cx.u.entities[e].kind, EntKind::Func(_))) {
        // A scope can hold an enumeration and another entity of one name
        // (NAM-39); the other entity wins (NAM-40).
        let others: Vec<_> = found.iter().filter(|&&e| !matches!(cx.u.entities[e].kind, EntKind::Enum(_))).collect();
        if others.len() == 1 {
            return false;
        }
        cx.error(span, "NAM-26", format!("'{}' is ambiguous", name));
        return true;
    }
    false
}

/// The components of a `nested_name`: a leading `::`, and the names.
fn components<'a>(cx: &Check<'a>, n: &Node, global: &mut bool, out: &mut Vec<(&'a str, Span)>) -> bool {
    for k in &n.kids {
        match k {
            Kid::Node(c) if c.prod == "nested_name" => {
                if !components(cx, c, global, out) {
                    return false;
                }
            }
            Kid::Node(_) => return false,
            _ => {
                let t = cx.text(k);
                if t == "::" {
                    if out.is_empty() {
                        *global = true;
                    }
                } else {
                    out.push((t, cx.token(k).map(|t| t.span).unwrap_or(n.span)));
                }
            }
        }
    }
    true
}

/// The scope a `nested_name` names (NAM-15, NAM-16).
pub fn nested_name(cx: &mut Check, n: &Node) -> Qual {
    let mut global = false;
    let mut parts = Vec::new();
    if !components(cx, n, &mut global, &mut parts) {
        cx.unsupported(n.span, "a qualifier with a template-id or decltype");
        return Qual::Unsupported;
    }
    let mut cur: Option<ScopeId> = if global { Some(0) } else { None };
    for (name, span) in parts {
        let keep = |k: &EntKind| matches!(k, EntKind::Namespace(_) | EntKind::Enum(_) | EntKind::Typedef(_) | EntKind::Unsupported(_));
        let found: Vec<EntityId> = match cur {
            None => lookup_where(cx, name, keep),
            Some(s) => lookup_in(cx, s, name),
        };
        // NAM-15: only namespaces and types are found before `::`.
        let pick = found.iter().copied().find(|&e| matches!(cx.u.entities[e].kind, EntKind::Namespace(_) | EntKind::Enum(_) | EntKind::Typedef(_) | EntKind::Unsupported(_)));
        let next = match pick.map(|e| cx.u.entities[e].kind.clone()) {
            Some(EntKind::Namespace(s)) => s,
            Some(EntKind::Enum(en)) => cx.u.enums[en].scope,
            Some(EntKind::Typedef(t)) => match &t.t {
                crate::sem::types::Type::Enum(en) => cx.u.enums[*en].scope,
                _ => {
                    cx.error(span, "NAM-16", format!("'{}' does not name a namespace, class, or enumeration", name));
                    return Qual::Error;
                }
            },
            Some(EntKind::Unsupported(_)) => {
                cx.unsupported(span, format!("the qualifier '{}', which names a class or template", name));
                return Qual::Unsupported;
            }
            _ => {
                if found.is_empty() {
                    cx.not_found(span, "NAM-8", name);
                } else {
                    cx.error(span, "NAM-16", format!("'{}' does not name a namespace, class, or enumeration", name));
                }
                return Qual::Error;
            }
        };
        cur = Some(next);
    }
    match cur {
        Some(s) => Qual::Scope(s),
        None => Qual::None,
    }
}

/// Declare an entity in the current scope, after checking that the
/// scope does not declare the name for another entity (NAM-3). The
/// caller has looked for an earlier declaration of the same entity.
pub fn declare(cx: &mut Check, name: &str, kind: EntKind, span: Span, linkage: Linkage) -> EntityId {
    declare_in(cx, cx.scope, name, kind, span, linkage)
}

pub fn declare_in(cx: &mut Check, scope: ScopeId, name: &str, kind: EntKind, span: Span, linkage: Linkage) -> EntityId {
    if !name.is_empty() {
        let earlier: Vec<EntityId> = cx.u.scopes[scope].names.get(name).cloned().unwrap_or_default();
        let new_is_enum = matches!(kind, EntKind::Enum(_));
        for e in earlier {
            let old_is_enum = matches!(cx.u.entities[e].kind, EntKind::Enum(_));
            // NAM-39: an enumeration and one other entity can share a
            // name in a scope.
            if new_is_enum != old_is_enum && !matches!(cx.u.entities[e].kind, EntKind::Typedef(_) | EntKind::Namespace(_)) && !matches!(kind, EntKind::Typedef(_) | EntKind::Namespace(_)) {
                continue;
            }
            if matches!(cx.u.entities[e].kind, EntKind::Unsupported(_)) || matches!(kind, EntKind::Unsupported(_)) {
                continue;
            }
            // Functions are overloads or redeclarations, which their page
            // decides.
            if matches!(cx.u.entities[e].kind, EntKind::Func(_)) && matches!(kind, EntKind::Func(_)) {
                continue;
            }
            redeclared(cx, scope, name, span);
            break;
        }
    }
    cx.u.add(Entity { name: Rc::from(name), kind, span, scope, linkage, deprecated: None })
}

/// NAM-3, with FN-13 for a parameter name redeclared in the outermost
/// block of a function.
fn redeclared(cx: &mut Check, scope: ScopeId, name: &str, span: Span) {
    let in_body = cx.u.scopes[scope].kind == ScopeKind::Function;
    let _ = in_body;
    cx.error(span, "NAM-3", format!("'{}' is already declared in this scope", name));
}

/// A name whose declaration build step 3 does not model, so a use of it
/// gives `unsupported` and no false NAM-8.
pub fn declare_unsupported(cx: &mut Check, name: &str, span: Span) {
    cx.u.add(Entity { name: Rc::from(name), kind: EntKind::Unsupported("a declaration the front end does not model"), span, scope: cx.scope, linkage: Linkage::None, deprecated: None });
}

/// The entities of one name in one scope only (no using-directives),
/// for redeclarations.
pub fn declared_here(cx: &Check, scope: ScopeId, name: &str) -> Vec<EntityId> {
    cx.u.scopes[scope].names.get(name).cloned().unwrap_or_default()
}
