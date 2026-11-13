//! The check part of `declarations/initialization/list`: braced lists
//! for scalars and arrays (LIST-1, LIST-2, LIST-11, LIST-15). Aggregate
//! classes and `std::initializer_list` come with build step 4.

use crate::expressions::check::{expr, first_node};
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::rvalue;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Conv, Expr, Init, EK};
use crate::sem::types::{IntK, QType, Type};
use crate::source::Span;
use crate::syntax::Node;

/// The element nodes of a braced list.
pub fn items(b: &Node) -> Vec<&Node> {
    b.children("init_clause").collect()
}

/// A value converted in braces: LIST-2 forbids a narrowing conversion.
pub fn braced_value(cx: &mut Check, e: Expr, to: &QType, rule: &'static str) -> Option<Expr> {
    if e.ty.is_error() || to.is_error() {
        return None;
    }
    let span = e.span;
    let narrow = {
        let r = rvalue(cx, e.clone());
        (r.ty.is_arithmetic() || r.ty.is_enum() || r.ty.is_ptr() || r.ty.is_nullptr()) && (to.is_arithmetic() || to.is_enum()) && conv::is_narrowing(cx, &r, to)
    };
    if narrow {
        cx.error(span, if rule == "INIT-7" { "LIST-2" } else { rule }, format!("a narrowing conversion to {} in braces", to.show()));
        if rule != "LIST-2" && rule != "INIT-7" {
            // The construct's own rule names it; LIST-2 is the general one.
        }
        return None;
    }
    // ENUM-8: an enumeration with a fixed underlying type takes an
    // integer in braces.
    if let Type::Enum(en) = to.t {
        let r = rvalue(cx, e);
        if r.ty.t == to.t {
            return Some(r);
        }
        if cx.u.enums[en].fixed && (r.ty.is_integral() || matches!(r.ty.t, Type::Enum(o) if !cx.u.enums[o].scoped)) {
            return Some(r.conv(Conv::Arith, to.unqual(), Cat::PR));
        }
        cx.error(span, "ENUM-19", format!("{} does not initialize the enumeration in braces", r.ty.show()));
        return None;
    }
    match conv::implicit(cx, e, to, true) {
        Ok(x) => Some(x),
        Err(m) => {
            cx.error_cause(span, conv::rule_of(&m).unwrap_or(if rule == "LIST-2" { "INIT-7" } else { rule }), conv::cause_of(&m), m);
            None
        }
    }
}

/// A braced list for a scalar (LIST-11): empty for zero, or one
/// expression that converts without narrowing.
pub fn scalar_list(cx: &mut Check, b: &Node, to: &QType, rule: &'static str) -> Option<Expr> {
    if to.base_elem().is_class() && !to.is_array() {
        let i = aggregate(cx, b, to)?;
        return Some(Expr::new(EK::List(Box::new(i)), to.unqual(), Cat::PR, b.span));
    }
    let its = items(b);
    let span = b.span;
    if to.is_error() {
        return None;
    }
    if to.is_array() {
        return array_prvalue(cx, b, to);
    }
    if to.is_void() {
        cx.unsupported(span, "a braced list for this type here");
        return None;
    }
    match its.len() {
        0 => Some(zero(to, span)),
        1 => {
            let c = first_node(its[0])?;
            if c.prod == "braced_init" || c.prod == "designator" {
                // LIST-14: a designator names a member of a class.
                cx.error_cause(span, "LIST-11", (c.prod == "designator").then_some("LIST-14"), "a braced list inside the braces for a scalar");
                return None;
            }
            let e = expr(cx, c);
            braced_value(cx, e, to, if rule == "INIT-7" { "LIST-2" } else { rule })
        }
        _ => {
            cx.error(span, "LIST-11", "more than one value in the braces for a scalar");
            None
        }
    }
}

/// An array prvalue from a braced list (LIST-3, CAST-18). An array of
/// unknown bound takes one element for each value.
pub fn array_prvalue(cx: &mut Check, b: &Node, to: &QType) -> Option<Expr> {
    let ty = match &to.t {
        Type::Array(el, None) => {
            let n = items(b).len() as u64;
            if n == 0 {
                cx.error(b.span, "DCL-19", "an array of unknown bound with an empty initializer");
                return None;
            }
            QType { t: Type::Array(el.clone(), Some(n)), c: to.c, v: to.v }
        }
        _ => to.clone(),
    };
    let init = list_init(cx, b, &ty, "")?;
    Some(Expr::new(EK::List(Box::new(init)), ty, Cat::PR, b.span))
}

/// The zero value of a scalar type.
pub fn zero(to: &QType, span: Span) -> Expr {
    match &to.t {
        Type::Float(_) => Expr::new(EK::Float(0.0), to.unqual(), Cat::PR, span),
        Type::Ptr(_) | Type::NullPtr => Expr::int(0, QType::new(Type::NullPtr), span).conv(Conv::NullPtr, to.unqual(), Cat::PR),
        _ => Expr::int(0, to.unqual(), span),
    }
}

/// The list initialization of a variable of type `ty`.
pub fn list_init(cx: &mut Check, b: &Node, ty: &QType, name: &str) -> Option<Init> {
    let _ = name;
    if ty.base_elem().is_class() {
        return aggregate(cx, b, ty);
    }
    match &ty.t {
        Type::Array(el, n) => array_list(cx, b, el, *n),
        _ => {
            if items(b).is_empty() {
                return Some(Init::Zero);
            }
            scalar_list(cx, b, ty, "LIST-2").map(Init::Expr)
        }
    }
}

/// An aggregate with a class in it from a braced list (LIST-3, LIST-4,
/// LIST-12, LIST-14, LIST-16): the values initialize the members in
/// order, a member without one gets its default member initializer or
/// its value-initialization, and the braces of an inner aggregate can be
/// left out (brace elision). A list for a class that is not an aggregate
/// waits for constructors.
pub fn aggregate(cx: &mut Check, b: &Node, ty: &QType) -> Option<Init> {
    let its = items(b);
    if let Some(c) = ty.class() {
        if its.is_empty() {
            return Some(crate::classes::members::check::value_init(cx, ty));
        }
        if !crate::classes::members::check::is_aggregate(cx, c) {
            cx.unsupported(b.span, "a braced list for a class that is not an aggregate (constructors come with build step 4)");
            return None;
        }
        if its.iter().any(|it| it.child("designator").is_some() || first_node(it).is_some_and(|c| c.prod == "designator")) {
            return designated(cx, b, c);
        }
    }
    let mut cache: Vec<Option<Expr>> = vec![None; its.len()];
    let mut pos = 0;
    let (init, ty2) = aggregate_from(cx, ty, &its, &mut cache, &mut pos, true)?;
    let _ = ty2;
    if pos < its.len() {
        let rule = if ty.is_array() { "DCL-19" } else { "LIST-12" };
        cx.error(its[pos].span, rule, "the list has more values than the aggregate has members");
        return None;
    }
    Some(init)
}

/// The members of an aggregate: a class's data members with their
/// default member initializers, or an array's elements (all of them, or
/// as many as the values need for an unknown bound at the top).
fn members(cx: &Check, ty: &QType, left: usize, top: bool) -> Option<Vec<(QType, Option<Init>)>> {
    match &ty.t {
        Type::Class(c) => Some(cx.u.classes[*c].fields.iter().filter_map(|&f| cx.u.field(f).map(|x| (x.ty.clone(), x.init.clone()))).collect()),
        Type::Array(e, Some(n)) => Some((0..*n).map(|_| ((**e).clone(), None)).collect()),
        Type::Array(e, None) if top => Some((0..left.max(1)).map(|_| ((**e).clone(), None)).collect()),
        _ => None,
    }
}

/// The value of a list item, evaluated once.
fn item_value(cx: &mut Check, its: &[&Node], cache: &mut [Option<Expr>], k: usize) -> Option<Expr> {
    if cache[k].is_none() {
        let c = first_node(its[k])?;
        cache[k] = Some(expr(cx, c));
    }
    cache[k].clone()
}

/// Initialize an aggregate of type `ty` from the items at `pos`. `braced`
/// says the items are the aggregate's own list; otherwise the braces were
/// left out, and the aggregate takes only the values it needs. The
/// result, and the type, with the bound of an array of unknown bound.
fn aggregate_from(cx: &mut Check, ty: &QType, its: &[&Node], cache: &mut [Option<Expr>], pos: &mut usize, braced: bool) -> Option<(Init, QType)> {
    let ms = members(cx, ty, its.len() - (*pos).min(its.len()), braced)?;
    let mut out = Vec::new();
    for (mty, dmi) in ms {
        if *pos >= its.len() {
            if ty.is_array() && matches!(ty.t, Type::Array(_, None)) {
                break;
            }
            out.push(dmi.unwrap_or_else(|| crate::classes::members::check::value_init(cx, &mty)));
            continue;
        }
        let c = first_node(its[*pos])?;
        if c.prod == "braced_init" {
            *pos += 1;
            let v = if mty.is_array() || mty.is_class() { aggregate(cx, c, &mty)? } else if items(c).is_empty() { Init::Zero } else { Init::Expr(scalar_list(cx, c, &mty, "LIST-2")?) };
            out.push(v);
            continue;
        }
        if mty.is_class() || mty.is_array() {
            let e = item_value(cx, its, cache, *pos)?;
            if e.ty.is_error() {
                return None;
            }
            // A value of the member's class, or a string for a character
            // array, initializes the member itself.
            if e.ty.nonref().unqual().t == mty.unqual().t && mty.is_class() {
                *pos += 1;
                out.push(Init::Expr(crate::declarations::initialization::forms::check::copy_init(cx, e, &mty, "INIT-7")?));
                continue;
            }
            if let (EK::Str(_), true) = (&e.k, mty.is_array()) {
                *pos += 1;
                out.push(string_init(cx, e, &mty)?);
                continue;
            }
            // LIST-13: an empty aggregate member takes its own braces.
            if mty.class().is_some_and(|c| cx.u.classes[c].fields.is_empty()) {
                cx.error(its[*pos].span, "LIST-13", "an empty aggregate member needs its own braces");
                return None;
            }
            // LIST-4: brace elision.
            let (v, _) = aggregate_from(cx, &mty, its, cache, pos, false)?;
            out.push(v);
            continue;
        }
        let e = item_value(cx, its, cache, *pos)?;
        *pos += 1;
        out.push(Init::Expr(braced_value(cx, e, &mty, "LIST-2")?));
    }
    let ty = match &ty.t {
        Type::Array(e, None) => QType::new(Type::Array(e.clone(), Some(out.len() as u64))).with_cv(ty.c, ty.v),
        _ => ty.clone(),
    };
    Some((Init::List(out), ty))
}

/// LIST-14: a list with designators `.member = value` names the class's
/// members in the order of their declarations, each at most once.
fn designated(cx: &mut Check, b: &Node, c: crate::sem::types::ClassId) -> Option<Init> {
    let fields = cx.u.classes[c].fields.clone();
    let mut given: Vec<Option<Init>> = vec![None; fields.len()];
    let mut last: Option<usize> = None;
    for it in items(b) {
        let Some(d) = it.child("designator").or_else(|| first_node(it).filter(|x| x.prod == "designator")) else {
            cx.error(it.span, "LIST-14", "a list mixes designators and values");
            return None;
        };
        let name = d.kids.iter().filter_map(|k| cx.token(k)).find(|t| t.kind == crate::lex::TK::Ident).map(|t| t.text.clone()).unwrap_or_default();
        let Some(k) = fields.iter().position(|&f| &*cx.u.entities[f].name == name) else {
            cx.error(d.span, "LIST-14", format!("the designator names no member '{}'", name));
            return None;
        };
        if last.is_some_and(|l| k <= l) {
            cx.error(d.span, "LIST-14", "the designators do not follow the order of the members");
            return None;
        }
        last = Some(k);
        let fty = cx.u.field(fields[k]).unwrap().ty.clone();
        let v = match d.kids.iter().filter_map(|k| match k {
            crate::syntax::Kid::Node(x) if x.prod != "designator" => Some(x),
            _ => None,
        }).last().or_else(|| it.kids.iter().filter_map(|k| match k {
            crate::syntax::Kid::Node(x) if x.prod != "designator" => Some(x),
            _ => None,
        }).last()) {
            Some(x) if x.prod == "braced_init" => {
                if fty.is_class() || fty.is_array() { aggregate(cx, x, &fty)? } else { Init::Expr(scalar_list(cx, x, &fty, "LIST-2")?) }
            }
            Some(x) => {
                let e = expr(cx, x);
                Init::Expr(braced_value(cx, e, &fty, "LIST-2")?)
            }
            None => return None,
        };
        given[k] = Some(v);
    }
    Some(Init::List(fields.iter().zip(given).map(|(&f, g)| {
        let x = cx.u.field(f).unwrap();
        g.or_else(|| x.init.clone()).unwrap_or_else(|| crate::classes::members::check::value_init(cx, &x.ty))
    }).collect()))
}

/// An array from a braced list (LIST-3, LIST-5, DCL-19): at most one
/// value per element, nested lists for nested arrays, and brace elision.
fn array_list(cx: &mut Check, b: &Node, el: &QType, bound: Option<u64>) -> Option<Init> {
    let its = items(b);
    if its.len() == 1 && el.is_integral() && matches!(el.int_kind(), Some(IntK::Char | IntK::SChar | IntK::UChar | IntK::Char8 | IntK::Char16 | IntK::Char32 | IntK::WChar)) {
        if let Some(c) = first_node(its[0]) {
            let e = expr(cx, c);
            if let EK::Str(_) = e.k {
                return string_init(cx, e, &QType::new(Type::Array(std::rc::Rc::new(el.clone()), bound)));
            }
        }
    }
    if its.is_empty() && bound.is_none() {
        cx.error(b.span, "DCL-19", "an array of unknown bound with an empty initializer");
        return None;
    }
    let mut out = Vec::new();
    for it in &its {
        let c = first_node(it)?;
        if c.prod == "designator" || it.child("designator").is_some() {
            cx.error(it.span, "LIST-14", "a designator in the list of an array");
            return None;
        }
        let v = if c.prod == "braced_init" {
            match &el.t {
                Type::Array(e2, n2) => array_list(cx, c, e2, *n2)?,
                _ => Init::Expr(scalar_list(cx, c, el, "LIST-2")?),
            }
        } else if el.is_array() {
            cx.unsupported(c.span, "brace elision in a nested array");
            return None;
        } else {
            let e = expr(cx, c);
            Init::Expr(braced_value(cx, e, el, "LIST-2")?)
        };
        out.push(v);
    }
    if let Some(n) = bound {
        if out.len() as u64 > n {
            cx.error(b.span, "DCL-19", format!("the initializer has {} elements for an array of {}", out.len(), n));
            return None;
        }
    }
    Some(Init::List(out))
}

/// A character array from a string literal (LIST-6, LIST-15).
pub fn string_init(cx: &mut Check, e: Expr, ty: &QType) -> Option<Init> {
    let (Type::Array(el, n), EK::Str(units)) = (&ty.t, &e.k) else { return None };
    let lit_el = e.ty.elem().and_then(|x| x.int_kind());
    let ok = match (el.int_kind(), lit_el) {
        (Some(IntK::Char | IntK::SChar | IntK::UChar), Some(IntK::Char)) => true,
        (Some(IntK::Char | IntK::UChar | IntK::Char8), Some(IntK::Char8)) => true,
        (Some(a), Some(b)) => a == b,
        _ => false,
    };
    if !ok {
        cx.error(e.span, "LIST-15", format!("a string literal of type {} does not initialize an array of {}", e.ty.show(), el.show()));
        return None;
    }
    if let Some(n) = n {
        if units.len() as u64 > *n {
            if units.len() as u64 - 1 > *n || true {
                cx.error(e.span, "LIST-15", format!("the string literal with its null character does not fit in {} elements", n));
                return None;
            }
        }
    }
    let items = units.iter().map(|&u| Init::Expr(Expr::int(u as i128, el.unqual(), e.span))).collect();
    Some(Init::List(items))
}
