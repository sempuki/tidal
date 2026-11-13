//! The check part of `declarations/enumerations`: enumeration types,
//! their underlying types, and their enumerators.

use crate::foundations::constant_evaluation::check as ce;
use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::types::{IntK, QType, Type};
use crate::sem::types::EnumId;
use crate::sem::{EntKind, EnumInfo, Linkage, ScopeKind};
use crate::source::Span;
use crate::syntax::{Kid, Node};
use std::rc::Rc;

/// The name in an `enum_head`, `enum_opaque`, or `elaborated_type`.
fn head_name<'a>(cx: &Check<'a>, n: &Node) -> Option<(&'a str, Span)> {
    n.kids.iter().rev().find_map(|k| match k {
        Kid::Tok(_) => {
            let t = cx.token(k)?;
            (t.kind == crate::lex::TK::Ident && !crate::foundations::keywords::is_keyword(&t.text)).then(|| (t.text.as_str(), t.span))
        }
        _ => None,
    })
}

/// The fixed underlying type after `:` (ENUM-4, ENUM-17).
fn fixed_type(cx: &mut Check, n: &Node) -> Option<IntK> {
    let ts = n.child("type_specifiers")?;
    let specs: Vec<&Node> = ts.children("type_specifier").collect();
    let (t, _, _, _) = crate::foundations::types::check::type_specifiers(cx, &specs, ts.span);
    match t.map(|t| t.t) {
        Some(Type::Int(k)) => Some(k),
        Some(Type::Error) | None => Some(IntK::Int),
        Some(_) => {
            cx.error(ts.span, "ENUM-17", "the underlying type of an enumeration must be an integer type");
            Some(IntK::Int)
        }
    }
}

/// An enumeration declared before, in this scope, by name.
fn earlier(cx: &Check, name: &str) -> Option<EnumId> {
    names::declared_here(cx, cx.scope, name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::Enum(en) => Some(en),
        _ => None,
    })
}

fn new_enum(cx: &mut Check, name: &str, span: Span, scoped: bool, fixed: Option<IntK>) -> EnumId {
    let scope = cx.u.new_scope(ScopeKind::Enum, Some(cx.scope), name);
    cx.u.enums.push(EnumInfo { name: Rc::from(name), scoped, underlying: if scoped { Some(fixed.unwrap_or(IntK::Int)) } else { fixed }, fixed: fixed.is_some() || scoped, enumerators: Vec::new(), scope, complete: false });
    let id = cx.u.enums.len() - 1;
    if !name.is_empty() {
        names::declare(cx, name, EntKind::Enum(id), span, Linkage::External);
    }
    id
}

/// `enum_def`: define an enumeration and its enumerators (ENUM-1 to
/// ENUM-4, ENUM-14 to ENUM-17).
pub fn enum_def(cx: &mut Check, n: &Node) -> QType {
    let Some(head) = n.child("enum_head") else { return QType::error() };
    if head.child("nested_name").is_some() {
        return cx.unsupported(head.span, "a qualified enumeration name");
    }
    let scoped = cx.has_word(head, "class") || cx.has_word(head, "struct");
    let name = head_name(cx, head);
    if scoped && name.is_none() {
        cx.error(head.span, "ENUM-16", "a scoped enumeration with no name");
        return QType::error();
    }
    let fixed = fixed_type(cx, n);
    let (nm, sp) = name.unwrap_or(("", head.span));
    let id = match earlier(cx, nm).filter(|_| !nm.is_empty()) {
        Some(e) if !cx.u.enums[e].complete && cx.u.enums[e].scoped == scoped => {
            if cx.u.enums[e].fixed && fixed.is_some() && cx.u.enums[e].underlying != fixed && !scoped {
                cx.error(head.span, "ENUM-18", "the declarations of the enumeration disagree on the underlying type");
            }
            e
        }
        Some(e) if cx.u.enums[e].complete => {
            cx.error(sp, "ODR-1", format!("the enumeration '{}' is defined twice", nm));
            return QType::new(Type::Enum(e));
        }
        Some(_) => {
            cx.error(sp, "ENUM-18", "the declarations of the enumeration disagree on scoped or unscoped");
            return QType::error();
        }
        None => new_enum(cx, nm, sp, scoped, fixed),
    };
    let under = cx.u.enums[id].underlying;
    let mut next: i128 = 0;
    let mut cur_ty = IntK::Int;
    let mut values = Vec::new();
    let target = cx.u.target;
    for en in n.children("enumerator") {
        let Some(k) = en.kids.first() else { continue };
        let ename = cx.text(k);
        let espan = cx.token(k).map(|t| t.span).unwrap_or(en.span);
        let value_node = en.kids.iter().filter_map(|k| match k {
            Kid::Node(c) if c.prod != "attribute_spec" => Some(c),
            _ => None,
        }).next();
        // ENUM-12: the value sees the enumerators declared before it,
        // which a scoped enumeration keeps in its own scope.
        let saved = cx.scope;
        if scoped {
            cx.scope = cx.u.enums[id].scope;
        }
        let computed = value_node.map(|e| ce::enumerator_value(cx, e, under));
        cx.scope = saved;
        let value = match computed {
            Some(r) => match r {
                Some((v, k)) => {
                    cur_ty = k;
                    v
                }
                None => next,
            },
            None => {
                // ENUM-13: a type that holds the value when the previous
                // enumerator's type does not.
                if !target.fits(cur_ty, next) {
                    for k in [IntK::Int, IntK::UInt, IntK::Long, IntK::ULong, IntK::LongLong, IntK::ULongLong] {
                        if k.rank() >= cur_ty.rank() && target.fits(k, next) {
                            cur_ty = k;
                            break;
                        }
                    }
                }
                if let Some(k) = under {
                    if !target.fits(k, next) && cx.u.enums[id].fixed {
                        cx.error(espan, "ENUM-14", format!("the implicit value of '{}' does not fit in the underlying type", ename));
                    }
                }
                next
            }
        };
        if let (Some(k), Some(_)) = (under.filter(|_| cx.u.enums[id].fixed), value_node) {
            if !target.fits(k, value) {
                cx.error(espan, "ENUM-17", format!("the value of '{}' does not fit in the underlying type", ename));
            }
        }
        next = value + 1;
        values.push(value);
        let scope = if scoped { cx.u.enums[id].scope } else { cx.scope };
        // ENUM-14: two enumerators of one name; ENUM-15: a name already
        // declared in the enclosing scope.
        if !names::declared_here(cx, scope, ename).is_empty() {
            let rule = if cx.u.enums[id].enumerators.iter().any(|&e| &*cx.u.entities[e].name == ename) { "ENUM-14" } else { "ENUM-15" };
            cx.error(espan, rule, format!("'{}' is already declared", ename));
            continue;
        }
        let e = names::declare_in(cx, scope, ename, EntKind::Enumerator { en: id, value, ty: cur_ty }, espan, Linkage::None);
        let mut attrs = crate::declarations::attributes::check::Attrs::default();
        for a in en.children("attribute_spec") {
            crate::declarations::attributes::check::attribute_spec(cx, a, &mut attrs);
        }
        cx.u.entities[e].deprecated = attrs.deprecated;
        cx.u.enums[id].enumerators.push(e);
        if !scoped {
            // Unscoped enumerators can also be named through the
            // enumeration (ENUM-2).
            let s = cx.u.enums[id].scope;
            cx.u.scopes[s].names.entry(Rc::from(ename)).or_default().push(e);
        }
    }
    if under.is_none() {
        cx.u.enums[id].underlying = Some(underlying_for(cx, &values));
    }
    cx.u.enums[id].complete = true;
    QType::new(Type::Enum(id))
}

/// The underlying type of an enumeration with no fixed type: the first
/// of `int`, `unsigned int`, `long`, `unsigned long`, `long long`, and
/// `unsigned long long` that holds every value (ENUM-5).
fn underlying_for(cx: &Check, values: &[i128]) -> IntK {
    let t = cx.u.target;
    for k in [IntK::Int, IntK::UInt, IntK::Long, IntK::ULong, IntK::LongLong, IntK::ULongLong] {
        if values.iter().all(|&v| t.fits(k, v)) {
            return k;
        }
    }
    IntK::LongLong
}

/// The values of an enumeration (ENUM-22): all values of a fixed type,
/// or the smallest bit-field that holds every enumerator, at least one
/// bit wide (so 0 and 1 with no enumerators).
pub fn value_range(u: &crate::sem::Unit, en: EnumId) -> (i128, i128) {
    let info = &u.enums[en];
    let t = u.target;
    if info.fixed {
        let k = info.underlying.unwrap_or(IntK::Int);
        return (t.int_min(k), t.int_max(k));
    }
    let vals: Vec<i128> = info.enumerators.iter().filter_map(|&e| match u.entities[e].kind {
        EntKind::Enumerator { value, .. } => Some(value),
        _ => None,
    }).collect();
    let (lo, hi) = (vals.iter().copied().min().unwrap_or(0), vals.iter().copied().max().unwrap_or(0));
    let mut bits = 0;
    while !((lo >= 0 || lo >= -(1i128 << bits)) && hi < (1i128 << bits)) {
        bits += 1;
    }
    if lo < 0 {
        (-(1i128 << bits), (1i128 << bits) - 1)
    } else {
        (0, (1i128 << bits.max(1)) - 1)
    }
}

/// `elaborated_type` naming an enumeration (TYN-10); a class key gives
/// `unsupported` in build step 3.
pub fn elaborated_type(cx: &mut Check, n: &Node) -> QType {
    if cx.first_word(n) != "enum" {
        return cx.unsupported(n.span, "a class named by an elaborated type specifier");
    }
    lookup_enum(cx, n)
}

/// The enumeration that the name of `n` finds.
fn lookup_enum(cx: &mut Check, n: &Node) -> QType {
    let Some((name, span)) = head_name(cx, n) else { return QType::error() };
    let found = match n.child("nested_name") {
        Some(nn) => match names::nested_name(cx, nn) {
            names::Qual::Scope(s) => names::lookup_in(cx, s, name),
            _ => return QType::error(),
        },
        None => names::lookup(cx, name),
    };
    for e in found {
        match &cx.u.entities[e].kind {
            EntKind::Enum(en) => return QType::new(Type::Enum(*en)),
            EntKind::Typedef(t) if t.is_enum() => return t.clone(),
            EntKind::Unsupported(_) => return cx.unsupported(span, "an elaborated type specifier for a class"),
            _ => {}
        }
    }
    cx.error(span, "TYN-10", format!("'enum {}' names no declared enumeration", name))
}

/// `enum_opaque` (ENUM-7, ENUM-18).
pub fn enum_opaque(cx: &mut Check, n: &Node) {
    if n.child("nested_name").is_some() {
        cx.unsupported(n.span, "a qualified opaque enumeration declaration");
        return;
    }
    let scoped = cx.has_word(n, "class") || cx.has_word(n, "struct");
    let fixed = fixed_type(cx, n);
    let Some((name, span)) = head_name(cx, n) else { return };
    if !scoped && fixed.is_none() {
        cx.error(n.span, "ENUM-18", "an opaque declaration of an unscoped enumeration needs an underlying type");
        return;
    }
    match earlier(cx, name) {
        Some(e) => {
            let info = &cx.u.enums[e];
            if info.scoped != scoped || (info.fixed && info.underlying != Some(fixed.unwrap_or(IntK::Int))) {
                cx.error(n.span, "ENUM-18", "the declarations of the enumeration disagree");
            }
        }
        None => {
            new_enum(cx, name, span, scoped, fixed);
        }
    }
}

/// `using enum E;` (ENUM-9, ENUM-20).
pub fn using_enum_decl(cx: &mut Check, n: &Node) {
    let ty = lookup_enum(cx, n);
    let Type::Enum(en) = ty.t else {
        if !ty.is_error() {
            cx.error(n.span, "ENUM-20", "'using enum' names no enumeration");
        }
        return;
    };
    if !cx.u.enums[en].complete {
        cx.error(n.span, "ENUM-20", "'using enum' names an enumeration that is not complete");
        return;
    }
    for e in cx.u.enums[en].enumerators.clone() {
        let name = cx.u.entities[e].name.clone();
        if !names::declared_here(cx, cx.scope, &name).is_empty() {
            cx.error(n.span, "ENUM-20", format!("'{}' is already declared in this scope", name));
            continue;
        }
        let s = cx.scope;
        cx.u.scopes[s].names.entry(name).or_default().push(e);
    }
}

/// ENUM-11: a scoped enumeration does not convert to `bool` by a
/// contextual conversion.
pub fn scoped_to_bool(cx: &mut Check, span: Span, rule: &str) {
    cx.error(span, "ENUM-11", format!("a scoped enumeration value used as a condition ({})", rule));
}
