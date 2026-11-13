//! The check part of `expressions/casts`: `static_cast`, `const_cast`,
//! `reinterpret_cast`, the cast notation `(T)e`, and the functional
//! notation `T(e)` and `T{e}`, for types that are not classes.

use crate::expressions::check::{bad, expr, first_node, nodes};
use crate::foundations::conversions::check as conv;
use crate::foundations::values::check::{deref_ref, rvalue};
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Conv, Expr, EK};
use crate::sem::types::{QType, Type};
use crate::source::Span;
use crate::syntax::{Kid, Node};

/// The kinds of cast that `(T)e` tries in order (CAST-9).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Static,
    Const,
    Reinterpret,
}

/// Strip cv-qualifiers at every level of a pointer type, for the
/// similarity test of `const_cast`.
fn strip(t: &QType) -> QType {
    match &t.t {
        Type::Ptr(p) => QType::ptr_to(strip(p)),
        _ => t.unqual(),
    }
}

/// Whether a pointer conversion removes a cv-qualifier at any level
/// (casting away constness, CAST-2, CAST-7).
fn casts_away(from: &QType, to: &QType) -> bool {
    match (&from.t, &to.t) {
        (Type::Ptr(a), Type::Ptr(b)) => (a.c && !b.c) || (a.v && !b.v) || casts_away(a, b),
        _ => false,
    }
}

/// The result of a cast of kind `k`, or the reason it is not valid.
fn try_cast(cx: &mut Check, k: Kind, e: &Expr, to: &QType) -> Result<Expr, (&'static str, String)> {
    let span = e.span;
    let e = e.clone();
    // A cast to a reference gives an lvalue (or an xvalue for `&&`).
    if let Type::Ref(inner, rv) = &to.t {
        let e = deref_ref(e);
        let cat = if *rv && !inner.is_func() { Cat::X } else { Cat::L };
        let same = e.ty.unqual().t == inner.unqual().t;
        return match k {
            Kind::Const if same && e.cat != Cat::PR => Ok(Expr::new(EK::Conv(Conv::NoOp, Box::new(e)), (**inner).clone(), cat, span)),
            Kind::Static if same && (!e.ty.c || inner.c) && (!e.ty.v || inner.v) && (e.cat != Cat::PR || *rv || inner.c) => {
                let e = if e.cat == Cat::PR { e.conv(Conv::Materialize, (**inner).clone(), Cat::X) } else { e };
                Ok(Expr::new(EK::Conv(Conv::NoOp, Box::new(e)), (**inner).clone(), cat, span))
            }
            // CAST-6: a glvalue of one object type as a reference to
            // another is `*reinterpret_cast<T*>(&e)`.
            Kind::Reinterpret if e.cat != Cat::PR && (e.ty.is_func() || inner.is_func()) => Err(("", "a reinterpret_cast of a function to a reference".into())),
            Kind::Reinterpret if e.cat != Cat::PR && ((e.ty.c && !inner.c) || (e.ty.v && !inner.v)) => Err(("CAST-7", "the cast removes a cv-qualifier".into())),
            Kind::Reinterpret if e.cat != Cat::PR => {
                let from = QType::ptr_to(e.ty.clone());
                let addr = Expr::new(EK::AddrOf(Box::new(e)), from, Cat::PR, span);
                let p = addr.conv(Conv::PtrReinterpret, QType::ptr_to((**inner).clone()), Cat::PR);
                Ok(Expr::new(EK::Deref(Box::new(p)), (**inner).clone(), cat, span))
            }
            _ => Err(("CAST-19", format!("no {:?} cast from {} to {}", k, e.ty.show(), to.show()))),
        };
    }
    let null = conv::is_null_constant(&e);
    let e = rvalue(cx, e);
    let from = e.ty.clone();
    let target = to.unqual();
    match k {
        Kind::Const => {
            if target.pointee().is_some_and(|p| p.is_func()) {
                return Err(("CAST-4", format!("const_cast to {}, a pointer to a function", to.show())));
            }
            if !target.is_ptr() {
                return Err(("CAST-4", format!("const_cast to {}, which is not a pointer or reference", to.show())));
            }
            if from.is_ptr() && strip(&from).t == strip(&target).t {
                return Ok(e.conv(Conv::PtrNoOp, target, Cat::PR));
            }
            Err(("CAST-4", format!("const_cast from {} to {} changes more than cv-qualifiers", from.show(), to.show())))
        }
        Kind::Static => {
            if target.is_void() {
                return Ok(e.conv(Conv::ToVoid, target, Cat::PR));
            }
            let from_enum = matches!(from.t, Type::Enum(_));
            let to_enum = matches!(target.t, Type::Enum(_));
            if (from_enum && (target.is_arithmetic() || to_enum)) || (to_enum && (from.is_arithmetic() || from_enum)) {
                return Ok(e.conv(Conv::Arith, target, Cat::PR));
            }
            if from.is_arithmetic() && target.is_arithmetic() {
                let c = if target.is_bool() { Conv::ToBool } else { Conv::Arith };
                return Ok(e.conv(c, target, Cat::PR));
            }
            if let (Some(fp), Some(tp)) = (from.pointee(), target.pointee()) {
                if casts_away(&from, &target) {
                    return Err(("CAST-2", "the cast removes a cv-qualifier".into()));
                }
                if fp.is_void() && !tp.is_func() {
                    return Ok(e.conv(Conv::PtrNoOp, target, Cat::PR));
                }
                if conv::pointer_convertible(fp, tp) {
                    return Ok(e.conv(Conv::PtrNoOp, target, Cat::PR));
                }
                return Err(("CAST-2", format!("static_cast between unrelated pointer types {} and {}", from.show(), to.show())));
            }
            if target.is_ptr() && null {
                return Ok(e.conv(Conv::NullPtr, target, Cat::PR));
            }
            if target.is_bool() && (from.is_ptr() || from.is_nullptr()) {
                return Ok(e.conv(Conv::ToBool, target, Cat::PR));
            }
            if (from.is_ptr() && target.is_integral()) || (from.is_integral() && target.is_ptr()) {
                return Err(("CAST-2", format!("static_cast between a pointer and an integer ({} to {})", from.show(), to.show())));
            }
            match conv::implicit(cx, e, &target, true) {
                Ok(x) => Ok(x),
                Err(m) => Err(("CAST-19", m)),
            }
        }
        Kind::Reinterpret => {
            if from.is_ptr() && target.is_integral() {
                let need = cx.u.target.ptr_size();
                if cx.u.target.size_of(&cx.u, &target).unwrap_or(0) < need {
                    return Err(("CAST-7", format!("{} is too small to hold a pointer", to.show())));
                }
                return Ok(e.conv(Conv::PtrToInt, target, Cat::PR));
            }
            if (from.is_integral() || from.is_enum()) && target.is_ptr() {
                return Ok(e.conv(Conv::IntToPtr, target, Cat::PR));
            }
            if from.is_nullptr() && target.is_integral() {
                return Ok(e.conv(Conv::PtrToInt, target, Cat::PR));
            }
            if from.is_ptr() && target.is_ptr() {
                if casts_away(&from, &target) {
                    return Err(("CAST-7", "the cast removes a cv-qualifier".into()));
                }
                return Ok(e.conv(Conv::PtrReinterpret, target, Cat::PR));
            }
            if from.t == target.t && (from.is_integral() || from.is_ptr()) {
                return Ok(e);
            }
            Err(("CAST-19", format!("no reinterpret_cast from {} to {}", from.show(), to.show())))
        }
    }
}

/// `named_cast`.
pub fn named_cast(cx: &mut Check, n: &Node) -> Expr {
    let span = n.span;
    let kw = n.child("cast_keyword").map(|k| cx.first_word(k)).unwrap_or("");
    let Some(tn) = n.child("type_id") else { return bad(span) };
    let to = crate::declarations::declarators::check::type_id(cx, tn);
    let Some(en) = nodes(n).into_iter().find(|c| c.prod != "cast_keyword" && c.prod != "type_id") else { return bad(span) };
    let e = expr(cx, en);
    let Some(e) = resolve_set(cx, e, &to) else { return bad(span) };
    if to.is_error() || e.ty.is_error() {
        return bad(span);
    }
    let k = match kw {
        "static_cast" => Kind::Static,
        "const_cast" => Kind::Const,
        _ => Kind::Reinterpret,
    };
    match try_cast(cx, k, &e, &to) {
        Ok(mut x) => {
            x.span = span;
            x
        }
        Err(("", m)) => {
            let t = cx.unsupported(span, m);
            Expr::new(EK::Int(0), t, Cat::PR, span)
        }
        Err((rule, m)) => {
            cx.error(span, rule, m);
            bad(span)
        }
    }
}

/// A cast in the notation `(T)e` or `T(e)` (CAST-9): the first of the
/// kinds that is valid.
pub fn explicit(cx: &mut Check, e: Expr, to: &QType, span: Span) -> Expr {
    let Some(e) = resolve_set(cx, e, to) else { return bad(span) };
    if to.is_error() || e.ty.is_error() {
        return bad(span);
    }
    let marks = cx.u.diags.len();
    let mut first_err = None;
    for k in [Kind::Const, Kind::Static, Kind::Reinterpret] {
        match try_cast(cx, k, &e, to) {
            Ok(mut x) => {
                cx.u.diags.truncate(marks);
                x.span = span;
                return x;
            }
            Err(err) => {
                cx.u.diags.truncate(marks);
                if first_err.is_none() || k == Kind::Static {
                    first_err = Some(err);
                }
            }
        }
    }
    // The static_cast followed by a const_cast (steps 3 and 5).
    if to.is_ptr() && rvalue(cx, e.clone()).ty.is_ptr() {
        let x = rvalue(cx, e);
        return x.conv(Conv::PtrReinterpret, to.unqual(), Cat::PR);
    }
    let (rule, m) = first_err.unwrap_or(("CAST-19", "no cast applies".into()));
    if rule.is_empty() {
        let t = cx.unsupported(span, m);
        return Expr::new(EK::Int(0), t, Cat::PR, span);
    }
    cx.error(span, "CAST-19", m);
    bad(span)
}

/// `cast_expr: "(" type_id ")" cast_expr`.
pub fn cast_expr(cx: &mut Check, n: &Node) -> Expr {
    let Some(tn) = n.child("type_id") else { return bad(n.span) };
    let to = crate::declarations::declarators::check::type_id(cx, tn);
    let Some(en) = nodes(n).into_iter().find(|c| c.prod != "type_id") else { return bad(n.span) };
    let e = expr(cx, en);
    explicit(cx, e, &to, n.span)
}

/// The type a `cast_type` names.
fn cast_type(cx: &mut Check, n: &Node) -> QType {
    let words: Vec<&str> = n.kids.iter().filter_map(|k| matches!(k, Kid::Tok(_)).then(|| cx.text(k))).collect();
    if n.kids.iter().all(|k| matches!(k, Kid::Tok(_))) && words.iter().all(|w| crate::foundations::types::check::is_fundamental(w)) {
        return crate::foundations::types::check::keywords(cx, &words, n.span);
    }
    if words == ["auto"] {
        return cx.unsupported(n.span, "auto in a functional cast");
    }
    crate::foundations::types::check::named_type(cx, n)
}

/// `functional_cast: cast_type "(" [expression_list] ")" | cast_type
/// braced_init` (CAST-10, CAST-11).
pub fn functional_cast(cx: &mut Check, n: &Node) -> Expr {
    let span = n.span;
    let Some(ct) = n.child("cast_type") else { return bad(span) };
    if ct.kids.len() == 1 && cx.words(ct) == ["auto"] {
        return decay_copy(cx, n, span);
    }
    let to = cast_type(cx, ct);
    if to.is_error() {
        return bad(span);
    }
    if let Some(b) = n.child("braced_init") {
        let Some(e) = crate::declarations::initialization::list::check::scalar_list(cx, b, &to, "CAST-11") else { return bad(span) };
        return e;
    }
    let args: Vec<&Node> = n.child("expression_list").map(|l| l.children("argument").collect()).unwrap_or_default();
    match args.len() {
        0 => {
            if to.is_void() {
                return Expr::new(EK::Int(0), QType::void(), Cat::PR, span);
            }
            // `T()` value-initializes: zero.
            let z = match &to.t {
                Type::Float(_) => Expr::new(EK::Float(0.0), to.unqual(), Cat::PR, span),
                Type::Ptr(_) | Type::NullPtr => Expr::int(0, QType::new(Type::NullPtr), span).conv(Conv::NullPtr, to.unqual(), Cat::PR),
                _ => Expr::int(0, to.unqual(), span),
            };
            z
        }
        1 => {
            let e = expr(cx, args[0]);
            explicit(cx, e, &to, span)
        }
        _ => {
            cx.error(span, "CAST-10", format!("a functional cast to {} with several operands", to.show()));
            bad(span)
        }
    }
}

/// `auto(e)` and `auto{e}` (CAST-13): a prvalue copy of the one
/// operand, with the type that `auto` deduces from it (AUTO-20).
fn decay_copy(cx: &mut Check, n: &Node, span: Span) -> Expr {
    let ops: Vec<&Node> = match n.child("braced_init") {
        Some(b) => b.children("init_clause").filter_map(first_node).collect(),
        None => n.child("expression_list").map(|l| l.children("argument").collect()).unwrap_or_default(),
    };
    let [op] = ops.as_slice() else {
        crate::declarations::auto::check::one_expression(cx, span);
        return bad(span);
    };
    if op.prod == "braced_init" {
        crate::declarations::auto::check::one_expression(cx, span);
        return bad(span);
    };
    let e = expr(cx, op);
    if crate::expressions::primary::check::unresolved_set(&e) {
        crate::expressions::primary::check::no_target(cx, &e, "AUTO-8");
        return bad(span);
    }
    if e.ty.is_error() {
        return bad(span);
    }
    if e.ty.is_void() {
        cx.error(span, "AUTO-8", "'auto' deduced from an expression of type void");
        return bad(span);
    }
    let to = crate::declarations::auto::check::decay(&e.ty).unqual_top();
    let mut x = rvalue(cx, e);
    x.ty = to;
    x.span = span;
    x
}

/// An overload set as the operand of a cast: the target type chooses the
/// function (OVL-18), and a target that is not a pointer or a reference
/// to a function chooses none (OVL-20). None after an error.
fn resolve_set(cx: &mut Check, e: Expr, to: &QType) -> Option<Expr> {
    if !crate::expressions::primary::check::unresolved_set(&e) || to.is_error() {
        return Some(e);
    }
    crate::declarations::initialization::forms::check::select_overload(cx, e, to)
}

