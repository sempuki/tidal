//! The check part of `declarations/attributes`: the standard attributes
//! that build step 3 reads, and the warnings of the recommended rules.

use crate::sem::check::Check;
use crate::sem::EntityId;
use crate::source::Span;
use crate::syntax::{Kid, Node};

/// The standard attributes on one declaration or statement.
#[derive(Clone, Debug, Default)]
pub struct Attrs {
    pub deprecated: Option<String>,
    pub nodiscard: bool,
    pub noreturn: bool,
    pub maybe_unused: bool,
    pub carries_dependency: bool,
    pub fallthrough: bool,
    pub likely: bool,
    pub unlikely: bool,
    /// The expression of `[[assume(e)]]`.
    pub assume: Option<Node>,
    pub any: bool,
    pub span: Span,
}

/// Read an `attribute_spec`. ATTR-2: the arguments of a standard
/// attribute.
pub fn attribute_spec(cx: &mut Check, n: &Node, a: &mut Attrs) {
    a.any = true;
    a.span = n.span;
    let using = cx.words(n).contains(&"using");
    for at in n.children("attribute") {
        let words = cx.words(at);
        // ATTR-17: after a `using` prefix, an attribute name has no
        // namespace of its own.
        if using && words.contains(&"::") {
            cx.error(at.span, "ATTR-17", "a qualified attribute name after a 'using' prefix");
        }
        let scoped = words.contains(&"::") || using;
        let Some(name) = words.iter().rev().find(|w| **w != "::").copied() else { continue };
        let args = at.child("attribute_args");
        if scoped {
            continue;
        }
        let arg_count = args.map(|x| x.kids.len().saturating_sub(2)).unwrap_or(0);
        match name {
            "deprecated" | "nodiscard" => {
                let toks: Vec<usize> = args.map(|x| {
                    let mut ts = Vec::new();
                    x.tokens(&mut ts);
                    ts
                }).unwrap_or_default();
                let inner: Vec<usize> = if toks.len() >= 2 { toks[1..toks.len() - 1].to_vec() } else { Vec::new() };
                let mut text = None;
                if !inner.is_empty() && inner.iter().all(|&t| cx.toks[t].kind == crate::lex::TK::Str) {
                    let mut s = String::new();
                    for &t in &inner {
                        if let Some(crate::lex::Lit::Str { units, .. }) = &cx.toks[t].lit {
                            s.extend(units.iter().filter_map(|&u| char::from_u32(u)));
                        }
                    }
                    text = Some(s);
                }
                let _ = arg_count;
                if args.is_some() && text.is_none() {
                    cx.error(at.span, "ATTR-2", format!("'{}' takes no argument or one string literal", name));
                }
                if name == "deprecated" {
                    a.deprecated = Some(text.unwrap_or_default());
                } else {
                    a.nodiscard = true;
                }
            }
            "noreturn" | "fallthrough" | "likely" | "unlikely" | "maybe_unused" | "no_unique_address" => {
                if args.is_some() {
                    cx.error(at.span, "ATTR-2", format!("'{}' takes no argument", name));
                }
                match name {
                    "noreturn" => a.noreturn = true,
                    "fallthrough" => a.fallthrough = true,
                    "likely" => a.likely = true,
                    "unlikely" => a.unlikely = true,
                    "maybe_unused" => a.maybe_unused = true,
                    _ => {}
                }
            }
            "carries_dependency" => a.carries_dependency = true,
            "assume" => {
                a.assume = args.cloned();
            }
            _ => {}
        }
    }
    if a.likely && a.unlikely {
        cx.error(n.span, "ATTR-2", "'likely' and 'unlikely' on the same statement");
    }
}

/// ATTR-2: `noreturn` applies only to functions, `nodiscard` not to a
/// variable.
/// ATTR-2: a standard attribute that applies to a type, where none of
/// those that build step 3 reads applies.
pub fn on_type(cx: &mut Check, a: &Attrs) {
    if a.deprecated.is_some() || a.nodiscard || a.noreturn || a.maybe_unused || a.fallthrough || a.likely || a.unlikely || a.carries_dependency || a.assume.is_some() {
        cx.error(a.span, "ATTR-2", "a standard attribute that does not apply to a type");
    }
}

/// ATTR-2: on a type alias, only `deprecated` and `maybe_unused` apply.
pub fn on_alias(cx: &mut Check, a: &Attrs) {
    if a.nodiscard || a.noreturn || a.fallthrough || a.likely || a.unlikely || a.carries_dependency || a.assume.is_some() {
        cx.error(a.span, "ATTR-2", "a standard attribute that does not apply to a type alias");
    }
}

pub fn on_variable(cx: &mut Check, a: &Attrs) {
    if a.assume.is_some() {
        cx.error(a.span, "ATTR-13", "an assumption applied to a declaration");
    }
    if a.fallthrough {
        cx.error(a.span, "ATTR-2", "'fallthrough' applies only to a null statement");
    }
    if a.noreturn {
        cx.error(a.span, "ATTR-2", "'noreturn' applies only to functions");
    }
    if a.nodiscard {
        cx.error(a.span, "ATTR-2", "'nodiscard' does not apply to a variable");
    }
}

/// ATTR-2, ATTR-19: an attribute before a using-directive applies to the
/// directive, and no standard attribute that build step 3 reads applies
/// to one.
pub fn on_using_directive(cx: &mut Check, a: &Attrs) {
    let names = [
        (a.deprecated.is_some(), "deprecated"),
        (a.nodiscard, "nodiscard"),
        (a.noreturn, "noreturn"),
        (a.maybe_unused, "maybe_unused"),
        (a.fallthrough, "fallthrough"),
        (a.likely || a.unlikely, "likely or unlikely"),
        (a.assume.is_some(), "assume"),
    ];
    for (on, name) in names {
        if on {
            cx.error(a.span, "ATTR-2", format!("'{}' does not apply to a using-directive", name));
        }
    }
}

/// ATTR-4: a use of a deprecated entity warns.
pub fn use_entity(cx: &mut Check, e: EntityId, span: Span) {
    if let Some(msg) = cx.u.entities[e].deprecated.clone() {
        let name = cx.u.entities[e].name.clone();
        let tail = if msg.is_empty() { String::new() } else { format!(": {}", msg) };
        cx.warning(span, "ATTR-4", format!("'{}' is deprecated{}", name, tail));
    }
}

/// ATTR-5: a discarded result of a `nodiscard` function.
pub fn discarded_nodiscard(cx: &mut Check, span: Span, name: &str) {
    cx.warning(span, "ATTR-5", format!("the result of '{}' is discarded", name));
}

/// ATTR-6: a variable of a block that the program never reads.
pub fn unused_variable(cx: &mut Check, span: Span, name: &str) {
    if cx.wflag(&["-Wunused", "-Wunused-variable", "-Wunused-but-set-variable", "-Wall"]) {
        cx.warning(span, "ATTR-6", format!("unused variable '{}'", name));
    }
}

/// ATTR-7: control falls through to the next label of a switch.
pub fn falls_through(cx: &mut Check, span: Span) {
    if cx.wflag(&["-Wimplicit-fallthrough", "-Wextra"]) {
        cx.warning(span, "ATTR-7", "control falls through to the next case label");
    }
}

/// What comes after a statement, for ATTR-2's check of `fallthrough`.
#[derive(Clone, Copy, PartialEq)]
enum Next {
    /// A `case` or `default` label of this switch.
    Case(crate::sem::tree::StmtId),
    /// Another statement, or the end of a switch or of the function.
    Other,
    /// The end of a loop body.
    LoopEnd,
}

/// ATTR-2 and ATTR-7 over a function body: each `[[fallthrough]];` must
/// be followed by a `case` or `default` label of its own switch; and
/// (-Wimplicit-fallthrough) control that falls from the statements after
/// one label into the next label warns, unless they end with
/// `[[fallthrough]];`.
pub fn fallthrough_checks(cx: &mut Check, body: &crate::sem::tree::Stmt) {
    let mut bad = Vec::new();
    placement(body, Next::Other, None, &mut bad);
    for span in bad {
        cx.error(span, "ATTR-2", "'fallthrough' is not followed by a case label of its switch");
    }
    if cx.wflag(&["-Wextra", "-Wimplicit-fallthrough"]) {
        let mut warn = Vec::new();
        falls(cx, body, &mut warn);
        for span in warn {
            cx.warning(span, "ATTR-7", "control falls through to the next case label");
        }
    }
}

fn placement(s: &crate::sem::tree::Stmt, next: Next, sw: Option<crate::sem::tree::StmtId>, bad: &mut Vec<Span>) {
    use crate::sem::tree::SK;
    match &s.k {
        SK::Fallthrough => {
            if sw.is_none() || next != Next::Case(sw.unwrap()) {
                bad.push(s.span);
            }
        }
        SK::Block(items) => {
            for (i, it) in items.iter().enumerate() {
                let after = match items.get(i + 1).map(|x| &x.k) {
                    Some(SK::Case(id, _, _)) => Next::Case(*id),
                    Some(_) => Next::Other,
                    None => next,
                };
                placement(it, after, sw, bad);
            }
        }
        SK::If { then, els, .. } => {
            placement(then, next, sw, bad);
            if let Some(e) = els {
                placement(e, next, sw, bad);
            }
        }
        SK::Consteval { constant, runtime } => {
            placement(constant, next, sw, bad);
            placement(runtime, next, sw, bad);
        }
        SK::Chosen(Some(x)) | SK::Case(_, _, x) | SK::Label(_, x) => placement(x, next, sw, bad),
        SK::While { body, .. } | SK::Do { body, .. } | SK::For { body, .. } | SK::RangeArray { body, .. } => placement(body, Next::LoopEnd, sw, bad),
        SK::Switch { id, body, .. } => placement(body, Next::Other, Some(*id), bad),
        _ => {}
    }
}

/// The spans of the labels that control falls into without
/// `[[fallthrough]];`.
fn falls(cx: &Check, s: &crate::sem::tree::Stmt, warn: &mut Vec<Span>) {
    use crate::sem::tree::{Stmt, SK};
    match &s.k {
        SK::Switch { body, .. } => {
            if let SK::Block(items) = &body.k {
                let mut start: Option<usize> = None;
                for (i, it) in items.iter().enumerate() {
                    if matches!(it.k, SK::Case(..)) {
                        if let Some(st) = start {
                            let seg: Vec<Stmt> = items[st..i].to_vec();
                            let last_fall = matches!(seg.last().map(|x| &x.k), Some(SK::Fallthrough)) || matches!(seg.last().map(|x| &x.k), Some(SK::Case(_, _, inner)) if matches!(inner.k, SK::Fallthrough));
                            let seg = Stmt::new(SK::Block(seg), it.span);
                            if !last_fall && crate::declarations::functions::definitions::check::can_complete(cx, &seg) {
                                warn.push(it.span);
                            }
                        }
                        start = Some(i);
                    }
                }
            }
            falls(cx, body, warn);
        }
        SK::Block(items) => items.iter().for_each(|x| falls(cx, x, warn)),
        SK::If { then, els, .. } => {
            falls(cx, then, warn);
            if let Some(e) = els {
                falls(cx, e, warn);
            }
        }
        SK::Consteval { constant, runtime } => {
            falls(cx, constant, warn);
            falls(cx, runtime, warn);
        }
        SK::Chosen(Some(x)) | SK::Case(_, _, x) | SK::Label(_, x) => falls(cx, x, warn),
        SK::While { body, .. } | SK::Do { body, .. } | SK::For { body, .. } | SK::RangeArray { body, .. } => falls(cx, body, warn),
        _ => {}
    }
}
