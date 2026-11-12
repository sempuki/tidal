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
pub fn on_variable(cx: &mut Check, a: &Attrs) {
    if a.noreturn {
        cx.error(a.span, "ATTR-2", "'noreturn' applies only to functions");
    }
    if a.nodiscard {
        cx.error(a.span, "ATTR-2", "'nodiscard' does not apply to a variable");
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
