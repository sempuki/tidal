//! The check part of `declarations/namespaces`: namespace definitions,
//! aliases, using-directives, and using-declarations.

use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::{EntKind, Linkage, ScopeId, ScopeKind};
use crate::syntax::{Kid, Node};

/// Open (or reopen) the namespace `name` in the current scope.
fn open(cx: &mut Check, name: &str, inline: bool, span: crate::source::Span) -> Option<ScopeId> {
    for e in names::declared_here(cx, cx.scope, name) {
        match cx.u.entities[e].kind {
            EntKind::Namespace(s) if &*cx.u.scopes[s].name != name || cx.u.scopes[s].parent != Some(cx.scope) => {
                // NS-14: a namespace alias does not reopen its namespace.
                cx.error(span, "NS-14", format!("'{}' is a namespace alias, which a namespace definition cannot reopen", name));
                return None;
            }
            EntKind::Namespace(s) => {
                // NS-12: `inline` only on the first definition, and a
                // namespace first defined without it stays without it.
                if inline && !cx.u.scopes[s].inline {
                    cx.error(span, "NS-12", format!("the namespace '{}' was first defined without 'inline'", name));
                }
                return Some(s);
            }
            _ => {
                cx.error(span, "NS-10", format!("'{}' is already declared as another entity", name));
                return None;
            }
        }
    }
    let s = cx.u.new_scope(ScopeKind::Namespace, Some(cx.scope), name);
    cx.u.scopes[s].inline = inline;
    if name.is_empty() || inline {
        // NAM-25, LNK-5: the members of an inline or unnamed namespace
        // are visible in the enclosing namespace.
        let parent = cx.scope;
        cx.u.scopes[parent].using.push(s);
    }
    if !name.is_empty() {
        names::declare(cx, name, EntKind::Namespace(s), span, Linkage::External);
    }
    Some(s)
}

/// `namespace_def` (NS-1 to NS-12).
pub fn namespace_def(cx: &mut Check, n: &Node) {
    if cx.in_function() {
        cx.error(n.span, "NS-10", "a namespace definition in a block");
        return;
    }
    let inline = cx.first_word(n) == "inline";
    let saved = cx.scope;
    let mut ok = true;
    match n.child("ns_path") {
        None => {
            let s = open(cx, "", inline, n.span);
            ok = s.is_some();
            if let Some(s) = s {
                cx.scope = s;
            }
        }
        Some(path) => {
            let mut inl = inline;
            for k in &path.kids {
                let t = cx.text(k);
                match t {
                    "inline" => inl = true,
                    "::" => {}
                    _ => {
                        let sp = cx.token(k).map(|t| t.span).unwrap_or(n.span);
                        match open(cx, t, inl, sp) {
                            Some(s) => cx.scope = s,
                            None => ok = false,
                        }
                        inl = false;
                    }
                }
            }
        }
    }
    if ok {
        for k in &n.kids {
            if let Kid::Node(d) = k {
                if d.prod == "declaration" {
                    crate::declarations::check::declaration(cx, d);
                }
            }
        }
    }
    cx.scope = saved;
}

/// The namespace that `[nested_name] identifier` names.
fn named_namespace(cx: &mut Check, n: &Node, rule: &'static str) -> Option<ScopeId> {
    let qual = match n.child("nested_name") {
        Some(nn) => match names::nested_name(cx, nn) {
            names::Qual::Scope(s) => Some(s),
            _ => return None,
        },
        None => None,
    };
    let last = n.kids.iter().rev().find(|k| matches!(k, Kid::Tok(_)) && cx.text(k) != ";")?;
    let name = cx.text(last);
    let span = cx.token(last).map(|t| t.span).unwrap_or(n.span);
    let found = match qual {
        Some(s) => names::lookup_in(cx, s, name),
        None => names::lookup(cx, name),
    };
    for e in &found {
        if let EntKind::Namespace(s) = cx.u.entities[*e].kind {
            return Some(s);
        }
    }
    if found.iter().any(|&e| matches!(cx.u.entities[e].kind, EntKind::Unsupported(_))) {
        cx.unsupported(span, format!("'{}' names an entity the front end does not model", name));
        return None;
    }
    if found.is_empty() {
        cx.not_found(span, "NAM-8", name);
    } else {
        cx.error(span, rule, format!("'{}' does not name a namespace", name));
    }
    None
}

/// `using_directive` (NS-8, NS-17).
pub fn using_directive(cx: &mut Check, n: &Node) {
    let mut attrs = crate::declarations::attributes::check::Attrs::default();
    for a in n.children("attribute_spec") {
        crate::declarations::attributes::check::attribute_spec(cx, a, &mut attrs);
    }
    crate::declarations::attributes::check::on_using_directive(cx, &attrs);
    if let Some(s) = named_namespace(cx, n, "NS-17") {
        let here = cx.scope;
        if !cx.u.scopes[here].using.contains(&s) {
            cx.u.scopes[here].using.push(s);
        }
    }
}

/// `namespace_alias` (NS-14).
pub fn namespace_alias(cx: &mut Check, n: &Node) {
    let Some(k) = n.kids.get(1) else { return };
    let name = cx.text(k);
    let span = cx.token(k).map(|t| t.span).unwrap_or(n.span);
    let target = {
        let rest = Node { prod: "namespace_name", ext: None, span: n.span, kids: n.kids[3..].to_vec() };
        named_namespace(cx, &rest, "NS-14")
    };
    let Some(s) = target else { return };
    for e in names::declared_here(cx, cx.scope, name) {
        if let EntKind::Namespace(old) = cx.u.entities[e].kind {
            if old != s {
                cx.error(span, "NS-14", format!("the alias '{}' names a different namespace", name));
            }
            return;
        }
    }
    names::declare(cx, name, EntKind::Namespace(s), span, Linkage::None);
}

/// `using_decl`: the named entities become visible here (NS-16).
pub fn using_decl(cx: &mut Check, n: &Node) {
    for q in n.children("qualified_name") {
        let Some(nn) = q.child("nested_name") else { continue };
        let scope = match names::nested_name(cx, nn) {
            names::Qual::Scope(s) => s,
            _ => continue,
        };
        let Some(Kid::Tok(t)) = q.child("unqualified_id").and_then(|u| u.kids.first()) else {
            cx.unsupported(q.span, "a using-declaration of an operator or a template");
            continue;
        };
        let name = cx.toks[*t].text.clone();
        let span = cx.toks[*t].span;
        // NS-9: a using-declaration at namespace scope names no member
        // of a class.
        if cx.u.scopes[scope].kind == crate::sem::ScopeKind::Class && cx.u.scopes[cx.scope].kind == crate::sem::ScopeKind::Namespace {
            cx.error(span, "NS-9", format!("a using-declaration at namespace scope names the class member '{}'", name));
            continue;
        }
        let found = names::lookup_in(cx, scope, &name);
        if found.is_empty() {
            cx.error(span, "NAM-14", format!("'{}' is not a member of the named scope", name));
            continue;
        }
        if found.iter().any(|&e| matches!(cx.u.entities[e].kind, EntKind::Namespace(_))) {
            cx.error(span, "NS-16", format!("a using-declaration names the namespace '{}'", name));
            continue;
        }
        let here = cx.scope;
        let existing = names::declared_here(cx, here, &name);
        for e in found {
            if existing.contains(&e) {
                continue;
            }
            let conflict = existing.iter().any(|&x| !matches!(cx.u.entities[x].kind, EntKind::Func(_)) || !matches!(cx.u.entities[e].kind, EntKind::Func(_)));
            if conflict {
                cx.error(span, "NS-16", format!("the using-declaration of '{}' conflicts with a declaration in this scope", name));
                break;
            }
            cx.u.scopes[here].names.entry(name.clone().into()).or_default().push(e);
        }
    }
}

/// NS-15: a definition with a qualified name needs a declaration of the
/// member in its namespace first.
pub fn member_not_declared(cx: &mut Check, span: crate::source::Span, name: &str) {
    cx.error(span, "NS-15", format!("no member '{}' is declared in the namespace", name));
}

/// NS-15: a definition with a qualified name is in a namespace that
/// encloses the member's namespace, and not in that namespace itself.
pub fn qualified_place(cx: &mut Check, member_scope: ScopeId, span: crate::source::Span, name: &str) {
    let here = cx.u.enclosing_namespace(cx.scope);
    let mut s = cx.u.scopes[member_scope].parent;
    while let Some(x) = s {
        if x == here {
            return;
        }
        s = cx.u.scopes[x].parent;
    }
    cx.error(span, "NS-15", format!("the definition of '{}' is not in a namespace that encloses its namespace", name));
}

/// NS-15: the namespace that declares the member `name` of `ns` that a
/// qualified definition defines: `ns` itself or one of its inline
/// namespaces. A declaration that a using-directive or an unnamed
/// namespace makes visible does not count.
pub fn member_home(cx: &Check, ns: ScopeId, name: &str, keep: impl Fn(&crate::sem::EntKind) -> bool) -> Option<ScopeId> {
    let mut work = vec![ns];
    while let Some(s) = work.pop() {
        if crate::foundations::names::check::declared_here(cx, s, name).iter().any(|&e| keep(&cx.u.entities[e].kind)) {
            return Some(s);
        }
        for (i, sc) in cx.u.scopes.iter().enumerate() {
            if sc.parent == Some(s) && sc.inline && !sc.name.is_empty() {
                work.push(i);
            }
        }
    }
    None
}
