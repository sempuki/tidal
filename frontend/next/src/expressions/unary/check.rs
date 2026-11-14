//! The check part of `expressions/unary`: the prefix operators,
//! `sizeof`, `alignof`, and `noexcept`.

use crate::expressions::check::{bad, expr, nodes};
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::{address_of_non_lvalue, deref_ref, rvalue};
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, UOp, EK};
use crate::sem::types::{QType, Type};
use crate::source::Span;
use crate::syntax::{Kid, Node};

/// `unary_expr`.
pub fn unary_expr(cx: &mut Check, n: &Node) -> Expr {
    let op = cx.first_word(n);
    let span = n.span;
    match op {
        "sizeof" | "alignof" => return size_or_align(cx, n, op == "alignof"),
        "noexcept" => {
            cx.unevaluated += 1;
            let e = nodes(n).first().map(|c| expr(cx, c));
            cx.unevaluated -= 1;
            let Some(e) = e.filter(|e| !e.ty.is_error()) else { return bad(span) };
            crate::expressions::decltype::check::unevaluated_effect_warning(cx, &e);
            let v = !crate::declarations::exception_specifications::check::can_throw(cx, &e);
            return Expr::int(v as i128, QType::bool(), span);
        }
        "co_await" => {
            let t = cx.unsupported(span, "a co_await expression");
            return Expr::new(EK::Int(0), t, Cat::PR, span);
        }
        _ => {}
    }
    if let Some(Kid::Node(c)) = n.kids.first() {
        match c.prod {
            "new_expr" => return crate::expressions::new_delete::check::new_expr(cx, c),
            "delete_expr" => return crate::expressions::new_delete::check::delete_expr(cx, c),
            _ => {}
        }
    }
    let Some(operand) = nodes(n).last().copied() else { return bad(span) };
    if op == "&" && operand.find("qualified_name").is_some() && operand.prod != "postfix_expr" {
        if let Some(q) = operand.find("qualified_name").and_then(|q| q.child("nested_name")) {
            let mut toks = Vec::new();
            q.tokens(&mut toks);
            let classes = toks.iter().rev().map(|&t| cx.toks[t].text.as_str()).find(|w| *w != "::").map(|w| crate::foundations::names::check::lookup_where(cx, w, |k| matches!(k, crate::sem::EntKind::Class(_)))).unwrap_or_default();
            // A static member function gives an ordinary pointer.
            let member = operand.find("qualified_name").and_then(|q| q.child("unqualified_id")).map(|u| cx.source(u));
            let all_static = classes.first().and_then(|&c| match cx.u.entities[c].kind {
                crate::sem::EntKind::Class(k) => Some(cx.u.classes[k].scope),
                _ => None,
            }).zip(member).is_some_and(|(s, m)| {
                let found = crate::foundations::names::check::declared_here(cx, s, &m);
                !found.is_empty() && found.iter().all(|&e| cx.u.func(e).is_some_and(|f| f.member_static) || cx.u.var(e).is_some())
            });
            if !classes.is_empty() && !all_static {
                let t = cx.unsupported(span, "the address of a member, a pointer to member (build step 4)");
                return Expr::new(EK::Int(0), t, Cat::PR, span);
            }
        }
    }
    let e = expr(cx, operand);
    if op == "&" && crate::expressions::primary::check::unresolved_set(&e) {
        return address_of(cx, e, span);
    }
    if e.ty.is_error() {
        return bad(span);
    }
    // OPR-6: an operand of class or enumeration type can select an
    // operator function.
    if matches!(op, "-" | "+" | "!" | "~" | "*" | "&" | "++" | "--") {
        if let Some(x) = crate::classes::operators::check::unary(cx, op, &e, false, span) {
            return x;
        }
    }
    match op {
        "&" if crate::classes::unions::check::width_of(&cx.u, &e).is_some() => {
            let rule = match &e.k {
                EK::Assign(..) | EK::Compound(..) => "ASN-10",
                EK::Cond(..) => "COND-9",
                _ => "BF-4",
            };
            cx.error(span, rule, "the address of a bit-field");
            bad(span)
        }
        "&" => address_of(cx, e, span),
        "*" => indirection(cx, e, span),
        "++" | "--" => inc_dec(cx, e, true, op == "++", span, "UN-11"),
        "+" | "-" => {
            // LAM-10: `+` on a closure applies to the converted pointer.
            let e = match crate::expressions::lambdas::check::to_pointer(cx, &e, None) {
                Some(Ok(p)) if op == "+" => p,
                _ => e,
            };
            let e = rvalue(cx, e);
            if let Type::Enum(en) = e.ty.t {
                if cx.u.enums[en].scoped {
                    cx.error(span, "UN-2", format!("unary '{}' on a scoped enumeration", op));
                    return bad(span);
                }
            }
            if e.ty.is_ptr() {
                if op == "+" {
                    return e;
                }
                cx.error(span, "UN-2", "unary '-' on a pointer");
                return bad(span);
            }
            if !e.ty.is_arithmetic() && !e.ty.is_enum() {
                cx.error(span, "UN-2", format!("unary '{}' on {}", op, e.ty.show()));
                return bad(span);
            }
            let e = conv::promote(cx, e);
            let t = e.ty.clone();
            Expr::new(EK::Unary(if op == "+" { UOp::Plus } else { UOp::Neg }, Box::new(e)), t, Cat::PR, span)
        }
        "!" => {
            let b = crate::expressions::logical::check::bool_operand(cx, e);
            if b.ty.is_error() {
                return bad(span);
            }
            Expr::new(EK::Unary(UOp::Not, Box::new(b)), QType::bool(), Cat::PR, span)
        }
        "~" => {
            let e = rvalue(cx, e);
            // BIT-7: `~` on a `bool` operand.
            if e.ty.is_bool() && cx.wflag(&["-Wall", "-Wbool-operation"]) {
                cx.warning(span, "BIT-7", "'~' on a bool operand");
            }
            let scoped = matches!(e.ty.t, Type::Enum(en) if cx.u.enums[en].scoped);
            if !(e.ty.is_integral() || e.ty.is_enum()) || scoped {
                cx.error(span, "BIT-3", format!("'~' on {}", e.ty.show()));
                return bad(span);
            }
            let e = conv::promote(cx, e);
            let t = e.ty.clone();
            Expr::new(EK::Unary(UOp::BitNot, Box::new(e)), t, Cat::PR, span)
        }
        _ => {
            let t = cx.unsupported(span, format!("the unary operator '{}'", op));
            Expr::new(EK::Int(0), t, Cat::PR, span)
        }
    }
}

/// `&e` (UN-8, UN-9).
pub fn address_of(cx: &mut Check, e: Expr, span: Span) -> Expr {
    let e = deref_ref(e);
    if let EK::Func(f) = e.k {
        crate::declarations::functions::definitions::check::address_taken(cx, f, span);
        if e.ty.is_error() {
            // The address of an overload set: a target type chooses
            // the function (OVL-18).
            return Expr::new(EK::AddrOf(Box::new(e)), QType::error(), Cat::PR, span);
        }
    }
    if e.cat != Cat::L {
        address_of_non_lvalue(cx, &e);
        return bad(span);
    }
    let t = QType::ptr_to(e.ty.clone());
    Expr::new(EK::AddrOf(Box::new(e)), t, Cat::PR, span)
}

/// `*e` (UN-5, UN-6).
pub fn indirection(cx: &mut Check, e: Expr, span: Span) -> Expr {
    let e = rvalue(cx, e);
    match e.ty.t.clone() {
        Type::Ptr(p) => {
            if p.is_void() {
                cx.error(span, "UN-6", "indirection through a pointer to void");
                return bad(span);
            }
            Expr::new(EK::Deref(Box::new(e)), (*p).clone(), Cat::L, span)
        }
        _ => {
            cx.error(span, "UN-6", format!("indirection through {}, which is not a pointer", e.ty.show()));
            bad(span)
        }
    }
}

/// Prefix and postfix `++` and `--` (UN-11, PFX-13, CV-4).
pub fn inc_dec(cx: &mut Check, e: Expr, prefix: bool, inc: bool, span: Span, rule: &'static str) -> Expr {
    let e = deref_ref(e);
    let sp = if inc { "++" } else { "--" };
    let t = e.ty.clone();
    if t.is_bool() {
        cx.error(span, rule, format!("'{}' on a bool", sp));
        return bad(span);
    }
    if t.is_enum() {
        cx.error_cause(span, rule, Some("ENUM-16"), format!("'{}' on an enumeration", sp));
        return bad(span);
    }
    if e.cat != Cat::L {
        cx.error(span, rule, format!("'{}' needs an lvalue", sp));
        return bad(span);
    }
    if t.c && crate::expressions::lambdas::check::copy_capture(cx, &e) {
        cx.error_cause(span, "LAM-23", Some(rule), format!("'{}' on a capture by copy in a lambda that is not mutable", sp));
        return bad(span);
    }
    if t.c {
        crate::declarations::declarators::check::modify_const(cx, span, &format!("'{}'", sp), rule);
        return bad(span);
    }
    match &t.t {
        Type::Int(_) | Type::Float(_) => {}
        Type::Ptr(p) if !p.is_void() && !p.is_func() && !cx.u.is_incomplete(p) => {}
        _ => {
            cx.error(span, rule, format!("'{}' on {}", sp, t.show()));
            return bad(span);
        }
    }
    crate::declarations::volatile::check::inc_dec(cx, &t, span);
    let ty = t.clone();
    Expr::new(EK::IncDec(prefix, inc, Box::new(e)), ty, if prefix { Cat::L } else { Cat::PR }, span)
}

/// `sizeof` and `alignof` (UN-13 to UN-17): a constant of the type of
/// `sizeof`.
fn size_or_align(cx: &mut Check, n: &Node, align: bool) -> Expr {
    let span = n.span;
    let st = QType::new(Type::Int(cx.u.target.size_t()));
    let ty = if let Some(t) = n.child("type_id") {
        crate::declarations::declarators::check::type_id(cx, t)
    } else if cx.has_word(n, "...") {
        let t = cx.unsupported(span, "sizeof... of a pack");
        return Expr::new(EK::Int(0), t, Cat::PR, span);
    } else {
        let Some(c) = nodes(n).last().copied() else { return bad(span) };
        if align {
            cx.error(span, "UN-17", "the operand of alignof must be a type");
            return bad(span);
        }
        cx.unevaluated += 1;
        let e = expr(cx, c);
        cx.unevaluated -= 1;
        crate::expressions::decltype::check::unevaluated_effect_warning(cx, &e);
        if crate::classes::unions::check::width_of(&cx.u, &e).is_some() {
            cx.error(span, "BF-4", "sizeof applied to a bit-field");
            return bad(span);
        }
        if let EK::Var(id) = &e.k {
            if cx.array_params.contains(id) {
                // UN-19: the parameter is a pointer (DCL-9).
                cx.warning(e.span, "UN-19", format!("sizeof of the parameter '{}', which is declared as an array and is a pointer", cx.u.entities[*id].name));
            }
        }
        e.ty.clone()
    };
    if ty.is_error() {
        return bad(span);
    }
    let rule = if align { "UN-17" } else { "UN-14" };
    let what = if align { "alignof" } else { "sizeof" };
    let t = ty.nonref();
    if t.is_func() || t.is_void() || (cx.u.is_incomplete(&t) && !(align && matches!(t.t, Type::Array(_, None)))) {
        cx.error(span, rule, format!("{} of {}", what, ty.show()));
        return bad(span);
    }
    let v = if align { cx.u.target.align_of(&cx.u, &t) } else { cx.u.target.size_of(&cx.u, &t) };
    match v {
        Some(v) => Expr::int(v as i128, st, span),
        None => {
            let t = cx.unsupported(span, format!("{} of {}", what, ty.show()));
            Expr::new(EK::Int(0), t, Cat::PR, span)
        }
    }
}
