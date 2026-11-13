//! The check part of `expressions/postfix`: calls, subscripts, and
//! postfix `++` and `--`. Member access comes with build step 4.

use crate::expressions::check::{bad, expr, first_node, nodes};
use crate::expressions::primary;
use crate::foundations::names::check as names;
use crate::foundations::values::check::{deref_ref, rvalue};
use crate::overloading::overload_resolution::check as ovl;
use crate::sem::check::Check;
use crate::sem::tree::{Arg, Cat, Expr, EK};
use crate::sem::types::{QType, Type};
use crate::sem::{EntKind, EntityId};
use crate::source::Span;
use crate::syntax::{Kid, Node};

/// `postfix_expr`.
pub fn postfix_expr(cx: &mut Check, n: &Node) -> Expr {
    let ns = nodes(n);
    let Some(base) = ns.first().copied() else { return bad(n.span) };
    let toks: Vec<&str> = cx.words(n);
    match toks.first().copied() {
        Some("(") => call(cx, base, ns.get(1).copied(), n.span),
        Some("[") => {
            let b = expr(cx, base);
            let Some(i) = ns.get(1) else {
                cx.error(n.span, "PFX-2", "a subscript with no index");
                return bad(n.span);
            };
            let parts = nodes(i);
            if parts.len() != 1 {
                cx.error(n.span, "COMMA-6", "a subscript with more than one expression");
                return bad(n.span);
            }
            let x = expr(cx, parts[0]);
            subscript(cx, b, x, n.span)
        }
        Some("++") | Some("--") => {
            let e = expr(cx, base);
            if e.ty.is_error() {
                return bad(n.span);
            }
            crate::expressions::unary::check::inc_dec(cx, e, false, toks[0] == "++", n.span, "PFX-13")
        }
        Some("." | "->") => {
            let b = expr(cx, base);
            if b.ty.is_error() {
                return bad(n.span);
            }
            if cx.source(n).contains('~') {
                if b.ty.nonref().is_scalar() {
                    cx.error(n.span, "DTOR-8", "a destructor name that is not called");
                    return bad(n.span);
                }
                let t = cx.unsupported(n.span, "a destructor call on a class (build step 4)");
                return Expr::new(EK::Int(0), t, Cat::PR, n.span);
            }
            let t = cx.error(n.span, "PFX-11", format!("member access on {}, which is not a class", b.ty.show()));
            Expr::new(EK::Int(0), t, Cat::PR, n.span)
        }
        _ => {
            let t = cx.unsupported(n.span, "a postfix expression of this form");
            Expr::new(EK::Int(0), t, Cat::PR, n.span)
        }
    }
}

/// `a[i]` (PFX-1, PFX-2): one operand a pointer or an array, the other
/// an integer or unscoped enumeration.
pub fn subscript(cx: &mut Check, a: Expr, i: Expr, span: Span) -> Expr {
    if a.ty.is_error() || i.ty.is_error() {
        return bad(span);
    }
    let (a, i) = (rvalue(cx, a), rvalue(cx, i));
    let (p, x) = if a.ty.is_ptr() { (a, i) } else if i.ty.is_ptr() { (i, a) } else {
        cx.error(span, "PFX-2", format!("a subscript of {}", a.ty.show()));
        return bad(span);
    };
    let int_ok = x.ty.is_integral() || matches!(x.ty.t, Type::Enum(en) if !cx.u.enums[en].scoped);
    if !int_ok {
        cx.error(span, "PFX-2", format!("a subscript of type {}", x.ty.show()));
        return bad(span);
    }
    let Some(el) = p.ty.pointee().cloned() else { return bad(span) };
    if el.is_void() || el.is_func() || el.is_incomplete() {
        cx.error(span, "PFX-2", "a subscript of a pointer to an incomplete or function type");
        return bad(span);
    }
    let x = crate::foundations::conversions::check::promote(cx, x);
    Expr::new(EK::Index(Box::new(p), Box::new(x)), el, Cat::L, span)
}

/// The argument nodes of a call.
fn arg_nodes(list: Option<&Node>) -> Vec<&Node> {
    list.map(|l| l.children("argument").collect()).unwrap_or_default()
}

/// The callee name of a call, through parentheses: its node, if the
/// callee is a name.
fn callee_name(n: &Node) -> Option<&Node> {
    match n.prod {
        "id_expression" => Some(n),
        "primary" => match n.kids.as_slice() {
            [Kid::Node(c)] if c.prod == "id_expression" => Some(c),
            _ => None,
        },
        _ => None,
    }
}

/// The member part of `x.~T` or `p->~T`, if the callee has that form.
fn destructor_name<'n>(cx: &Check, callee: &'n Node) -> Option<(&'n Node, bool, &'n Node)> {
    if callee.prod != "postfix_expr" {
        return None;
    }
    let arrow = match callee.kids.get(1) {
        Some(k @ Kid::Tok(_)) if matches!(cx.text(k), "." | "->") => cx.text(k) == "->",
        _ => return None,
    };
    let (Some(Kid::Node(base)), Some(Kid::Node(id))) = (callee.kids.first(), callee.kids.get(2)) else { return None };
    let u = match first_node(id)? {
        q if q.prod == "qualified_name" => q.child("unqualified_id")?,
        u => u,
    };
    cx.has_word(u, "~").then_some((base, arrow, id))
}

/// The type a name after `~`, or in its qualifier, names.
fn named_type(cx: &mut Check, n: &Node) -> Option<QType> {
    if n.child("decltype_expr").is_some() {
        return Some(crate::expressions::decltype::check::decltype_spec(cx, n));
    }
    let mut toks = Vec::new();
    n.tokens(&mut toks);
    let name = toks.iter().map(|&t| cx.toks[t].text.as_str()).rfind(|w| *w != "~" && *w != "::")?;
    let found = names::lookup_where(cx, name, |k| matches!(k, EntKind::Typedef(_)));
    match found.first().map(|&f| cx.u.entities[f].kind.clone()) {
        Some(EntKind::Typedef(t)) => Some(t),
        _ => None,
    }
}

/// A pseudo-destructor call `x.~T()` or `p->~T()` on a scalar (DTOR-6,
/// DTOR-8, NAM-16): it ends the life of the object. The names after `~`
/// and in its qualifier name the type of the object.
fn pseudo_destructor(cx: &mut Check, base: &Node, arrow: bool, id: &Node, args: bool, span: Span) -> Expr {
    let b = expr(cx, base);
    if b.ty.is_error() {
        return bad(span);
    }
    let obj = if arrow {
        let b = rvalue(cx, b);
        if !b.ty.is_ptr() {
            cx.error(span, "PFX-11", format!("'->' on {}, which is not a pointer", b.ty.show()));
            return bad(span);
        }
        crate::expressions::unary::check::indirection(cx, b, span)
    } else {
        deref_ref(b)
    };
    if obj.ty.is_error() {
        return bad(span);
    }
    if !obj.ty.is_scalar() {
        let t = cx.unsupported(span, "a destructor call on a class (build step 4)");
        return Expr::new(EK::Int(0), t, Cat::PR, span);
    }
    if args {
        cx.error(span, "DTOR-8", "a destructor call with arguments");
        return bad(span);
    }
    let mut parts: Vec<&Node> = Vec::new();
    match first_node(id) {
        Some(q) if q.prod == "qualified_name" => {
            parts.extend(q.child("nested_name"));
            parts.extend(q.child("unqualified_id"));
        }
        Some(u) => parts.push(u),
        None => return bad(span),
    }
    for p in parts {
        let Some(t) = named_type(cx, p) else {
            let t = cx.unsupported(p.span, "a name after '~' that is not a type alias or decltype");
            return Expr::new(EK::Int(0), t, Cat::PR, span);
        };
        if t.is_error() {
            return bad(span);
        }
        if t.unqual() != obj.ty.unqual() {
            cx.error(p.span, "DTOR-8", format!("the destructor name names {}, and the object has type {}", t.show(), obj.ty.show()));
            return bad(span);
        }
    }
    Expr::new(EK::EndLife(Box::new(obj)), QType::void(), Cat::PR, span)
}

/// A call (PFX-3 to PFX-8, OVL rules for a name with several functions).
pub fn call(cx: &mut Check, callee: &Node, list: Option<&Node>, span: Span) -> Expr {
    if let Some((base, arrow, id)) = destructor_name(cx, callee) {
        return pseudo_destructor(cx, base, arrow, id, !arg_nodes(list).is_empty(), span);
    }
    let mut args = Vec::new();
    for a in arg_nodes(list) {
        if first_node(a).is_some_and(|c| c.prod == "braced_init") {
            let t = cx.unsupported(a.span, "a braced list as an argument");
            return Expr::new(EK::Int(0), t, Cat::PR, span);
        }
        args.push(expr(cx, a));
    }
    if list.is_some_and(|l| cx.has_word(l, "...")) {
        let t = cx.unsupported(span, "a pack expansion in a call");
        return Expr::new(EK::Int(0), t, Cat::PR, span);
    }
    if args.iter().any(|a| a.ty.is_error() && !primary::check::unresolved_set(a)) {
        return bad(span);
    }
    // A name that finds functions: overload resolution chooses.
    if let Some(name) = callee_name(callee) {
        if let Some((mut found, rule, text, nspan)) = primary::check::find(cx, name) {
            // NAM-27, NAM-29: an unqualified name also finds the functions
            // of the arguments' associated namespaces, unless ordinary
            // lookup finds something other than a function at namespace
            // scope.
            if rule == "NAM-8" {
                let plain = found.iter().all(|&e| {
                    matches!(cx.u.entities[e].kind, EntKind::Func(_)) && cx.u.scopes[cx.u.entities[e].scope].kind == crate::sem::ScopeKind::Namespace
                });
                if plain {
                    let tys: Vec<QType> = args.iter().map(|a| a.ty.clone()).collect();
                    for e in crate::foundations::names::check::argument_dependent(cx, &text, &tys) {
                        if !found.contains(&e) {
                            found.push(e);
                        }
                    }
                }
            }
            if found.is_empty() {
                if !rule.is_empty() {
                    cx.not_found(nspan, rule, &text);
                }
                return bad(span);
            }
            let funcs: Vec<EntityId> = found.iter().copied().filter(|&e| matches!(cx.u.entities[e].kind, EntKind::Func(_))).collect();
            if !funcs.is_empty() && funcs.len() == found.len() {
                let Some(f) = ovl::resolve(cx, &funcs, &args, span, &text) else { return bad(span) };
                crate::declarations::attributes::check::use_entity(cx, f, nspan);
                crate::declarations::functions::definitions::check::note_use(cx, f, nspan);
                let fty = QType::new(Type::Func(cx.u.func(f).unwrap().ty.clone()));
                let callee = Expr::new(EK::Func(f), fty, Cat::L, nspan);
                return finish_call(cx, callee, Some(f), args, span);
            }
            if found.iter().any(|&e| matches!(cx.u.entities[e].kind, EntKind::Unsupported(_))) {
                let t = cx.unsupported(span, format!("a call of '{}', which the front end does not model", text));
                return Expr::new(EK::Int(0), t, Cat::PR, span);
            }
        }
    }
    let f = expr(cx, callee);
    if f.ty.is_error() {
        return bad(span);
    }
    let f = deref_ref(f);
    let target = match &f.k {
        EK::Func(id) => Some(*id),
        _ => None,
    };
    finish_call(cx, f, target, args, span)
}

/// The call of a chosen function or of a function pointer: the
/// arguments initialize the parameters.
fn finish_call(cx: &mut Check, f: Expr, target: Option<EntityId>, args: Vec<Expr>, span: Span) -> Expr {
    let fty = match &f.ty.t {
        Type::Func(ft) => ft.clone(),
        Type::Ptr(p) => match &p.t {
            Type::Func(ft) => ft.clone(),
            _ => return not_callable(cx, &f, span),
        },
        Type::Int(_) | Type::Float(_) | Type::Enum(_) | Type::NullPtr | Type::Array(..) | Type::Void => return not_callable(cx, &f, span),
        _ => {
            let t = cx.unsupported(span, "a call of an object of this type");
            return Expr::new(EK::Int(0), t, Cat::PR, span);
        }
    };
    let f = if f.ty.is_func() { f } else { rvalue(cx, f) };
    let n = fty.params.len();
    // PFX-5: one argument for each parameter without a default.
    let defaults: Vec<Option<Expr>> = target.and_then(|t| cx.u.func(t)).map(|fi| fi.defaults.clone()).unwrap_or_default();
    if args.len() > n && !fty.variadic {
        cx.error(span, "PFX-5", format!("the call gives {} arguments for {} parameters", args.len(), n));
        return bad(span);
    }
    let mut out = Vec::new();
    for k in 0..n.max(args.len()) {
        if k < args.len() {
            let a = args[k].clone();
            if k < n {
                match crate::declarations::initialization::forms::check::init_parameter(cx, a, &fty.params[k]) {
                    Some(arg) => out.push(arg),
                    None => return bad(span),
                }
            } else {
                // An argument for the ellipsis is promoted (PFX-7).
                let a = rvalue(cx, a);
                let a = if a.ty.t == Type::Float(crate::sem::types::FloatK::Float) {
                    a.conv(crate::sem::tree::Conv::Arith, QType::new(Type::Float(crate::sem::types::FloatK::Double)), Cat::PR)
                } else {
                    crate::foundations::conversions::check::promote(cx, a)
                };
                out.push(Arg { e: a, by_ref: false });
            }
        } else {
            match defaults.get(k).cloned().flatten() {
                Some(d) => {
                    let by_ref = fty.params[k].is_ref();
                    out.push(Arg { e: d, by_ref });
                }
                None => {
                    // DEFARG-5: a call through a pointer or a reference
                    // has no default arguments.
                    let cause = target.is_none().then_some("DEFARG-5");
                    cx.error_cause(span, "PFX-5", cause, format!("the call gives {} arguments for {} parameters", args.len(), n));
                    return bad(span);
                }
            }
        }
    }
    let ret = fty.ret.clone();
    let (ty, cat) = match &ret.t {
        Type::Ref(inner, rv) => ((**inner).clone(), if *rv && !inner.is_func() { Cat::X } else { Cat::L }),
        _ => (ret.unqual_top(), Cat::PR),
    };
    if let Some(t) = target {
        crate::declarations::functions::definitions::check::called(cx, t, span);
    }
    let call = Expr::new(EK::Call(Box::new(f), out), ty, cat, span);
    if let Some(t) = target {
        return crate::declarations::constexpr::check::immediate_call(cx, t, call);
    }
    call
}

fn not_callable(cx: &mut Check, f: &Expr, span: Span) -> Expr {
    cx.error(span, "PFX-17", format!("a call of {}, which is not a function", f.ty.show()));
    bad(span)
}
