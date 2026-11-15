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
            // OPR-3: `operator[]` takes any number of arguments.
            if b.ty.nonref().class().is_some_and(|c| !crate::foundations::names::check::declared_here(cx, cx.u.classes[c].scope, "operator[]").is_empty()) {
                let parts: Vec<&Node> = ns.get(1).map(|i| nodes(i)).unwrap_or_default();
                let args: Vec<Expr> = parts.into_iter().map(|p| expr(cx, p)).collect();
                if args.iter().any(|a| a.ty.is_error()) {
                    return bad(n.span);
                }
                return crate::classes::operators::check::subscript(cx, &b, &args, n.span).unwrap_or_else(|| bad(n.span));
            }
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
            // OPR-3: the subscript of a class object.
            if let Some(r) = crate::classes::operators::check::subscript(cx, &b, std::slice::from_ref(&x), n.span) {
                return r;
            }
            subscript(cx, b, x, n.span)
        }
        Some("++") | Some("--") => {
            let e = expr(cx, base);
            if e.ty.is_error() {
                return bad(n.span);
            }
            // OPR-4: the postfix form of an operator function.
            if let Some(x) = crate::classes::operators::check::unary(cx, toks[0], &e, true, n.span) {
                return x;
            }
            // CVF-2: a class operand converts by its conversion function.
            let Some(e) = crate::classes::conversion_functions::check::builtin_operand(cx, e) else { return bad(n.span) };
            crate::expressions::unary::check::inc_dec(cx, e, false, toks[0] == "++", n.span, "PFX-13")
        }
        Some("." | "->") => {
            let b = expr(cx, base);
            if b.ty.is_error() {
                return bad(n.span);
            }
            let arrow = toks.contains(&"->");
            // OPR-5: `->` on a class object calls its operator->.
            let b = if arrow && b.ty.nonref().is_class() { crate::classes::operators::check::arrow(cx, b, n.span) } else { b };
            if b.ty.is_error() {
                return bad(n.span);
            }
            let is_class = if arrow { b.ty.nonref().pointee().is_some_and(|p| p.is_class()) } else { b.ty.nonref().is_class() };
            if is_class && !cx.source(n).contains('~') {
                let Some(id) = ns.get(1).copied() else { return bad(n.span) };
                return crate::classes::members::check::member_access(cx, b, arrow, id, n.span);
            }
            if cx.source(n).contains('~') {
                if b.ty.nonref().is_scalar() {
                    cx.error(n.span, "DTOR-8", "a destructor name that is not called");
                    return bad(n.span);
                }
                // DTOR-8: a destructor name names no value without a call.
                if b.ty.nonref().is_class() {
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
    // PFX-1: the subscript of an array xvalue is an xvalue.
    let xvalue = [&a, &i].iter().any(|x| x.cat == Cat::X && x.ty.is_array());
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
    if el.is_void() || el.is_func() || cx.u.is_incomplete(&el) {
        cx.error(span, "PFX-2", "a subscript of a pointer to an incomplete or function type");
        return bad(span);
    }
    let x = crate::foundations::conversions::check::promote(cx, x);
    Expr::new(EK::Index(Box::new(p), Box::new(x)), el, if xvalue { Cat::X } else { Cat::L }, span)
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
    if args {
        cx.error(span, "DTOR-8", "a destructor call with arguments");
        return bad(span);
    }
    if let Some(c) = obj.ty.class() {
        return class_destructor_call(cx, obj, c, id, span);
    }
    if !obj.ty.is_scalar() {
        let t = cx.unsupported(span, "a destructor call on an object of this type");
        return Expr::new(EK::Int(0), t, Cat::PR, span);
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

/// An explicit destructor call on an object of class `c` (DTOR-6,
/// DTOR-8): the name after `~` names the class, by its name, a type
/// alias, or `decltype`.
fn class_destructor_call(cx: &mut Check, obj: Expr, c: crate::sem::types::ClassId, id: &Node, span: Span) -> Expr {
    let u = match first_node(id) {
        // DTOR-6: `x.B::~B()` ends the base subobject of class `B`,
        // without a virtual call.
        Some(q) if q.prod == "qualified_name" => {
            let Some(name) = q.find("class_name").map(|x| cx.source(x)) else { return bad(span) };
            let found = names::lookup_where(cx, &name, |k| matches!(k, EntKind::Class(_) | EntKind::Typedef(_)));
            let named = found.first().and_then(|&e| match &cx.u.entities[e].kind {
                EntKind::Class(k) => Some(*k),
                EntKind::Typedef(t) => t.class(),
                _ => None,
            });
            let Some(b) = named.filter(|&b| b == c || crate::classes::derived::check::is_base(cx, c, b)) else {
                cx.error(span, "DTOR-8", format!("the destructor name '~{}' does not name the class of the object or a base of it", name));
                return bad(span);
            };
            let obj = if b == c { obj } else {
                match crate::classes::derived::check::to_base(cx, obj, b, span) {
                    Some(x) => x,
                    None => return bad(span),
                }
            };
            if !crate::classes::constructors::check::dtor_usable(cx, &obj.ty, span) {
                return bad(span);
            }
            return Expr::new(EK::Destroy(Box::new(obj)), QType::void(), Cat::PR, span);
        }
        Some(u) => u,
        None => return bad(span),
    };
    if u.find("decltype_expr").is_some() || u.find("decltype_spec").is_some() {
        match named_type(cx, u) {
            Some(t) if t.unqual() == obj.ty.unqual() => {}
            Some(t) => {
                cx.error(u.span, "DTOR-8", format!("the destructor name names {}, and the object has type {}", t.show(), obj.ty.show()));
                return bad(span);
            }
            None => return bad(span),
        }
    } else {
        let mut toks = Vec::new();
        u.tokens(&mut toks);
        let Some(name) = toks.iter().map(|&t| cx.toks[t].text.clone()).rfind(|w| w != "~") else { return bad(span) };
        let found = names::lookup_where(cx, &name, |k| matches!(k, EntKind::Class(_) | EntKind::Typedef(_)));
        let named = found.first().and_then(|&e| match &cx.u.entities[e].kind {
            EntKind::Class(k) => Some(*k),
            EntKind::Typedef(t) => t.class(),
            _ => None,
        });
        if named != Some(c) && *name != *cx.u.classes[c].name {
            cx.error(u.span, "DTOR-8", format!("the destructor name '~{}' does not name the class of the object", name));
            return bad(span);
        }
    }
    if !crate::classes::constructors::check::dtor_usable(cx, &obj.ty, span) {
        return bad(span);
    }
    Expr::new(EK::Destroy(Box::new(obj)), QType::void(), Cat::PR, span)
}

/// A call (PFX-3 to PFX-8, OVL rules for a name with several functions).
pub fn call(cx: &mut Check, callee: &Node, list: Option<&Node>, span: Span) -> Expr {
    let top = cx.decltype_call == Some(span);
    let r = call_inner(cx, callee, list, span, top);
    cx.no_temporary = false;
    r
}

fn call_inner(cx: &mut Check, callee: &Node, list: Option<&Node>, span: Span, top: bool) -> Expr {
    if let Some((base, arrow, id)) = destructor_name(cx, callee) {
        return pseudo_destructor(cx, base, arrow, id, !arg_nodes(list).is_empty(), span);
    }
    if arg_nodes(list).iter().any(|a| first_node(a).is_some_and(|c| c.prod == "braced_init")) {
        return braced_call(cx, callee, list, span);
    }
    let mut args = Vec::new();
    for a in arg_nodes(list) {
        args.push(expr(cx, a));
    }
    if list.is_some_and(|l| cx.has_word(l, "...")) {
        let t = cx.unsupported(span, "a pack expansion in a call");
        return Expr::new(EK::Int(0), t, Cat::PR, span);
    }
    if args.iter().any(|a| a.ty.is_error() && !primary::check::unresolved_set(a)) {
        return bad(span);
    }
    cx.no_temporary = top;
    if let Some(r) = address_call(cx, callee, &args, span) {
        cx.no_temporary = false;
        return r;
    }
    if let Some(r) = member_call(cx, callee, &args, span) {
        cx.no_temporary = false;
        return r;
    }
    // A name that finds functions: overload resolution chooses.
    // TARG-1: a call of a function template with explicit arguments.
    if let Some(tid) = callee_name(callee).and_then(|id| id.find("template_id").filter(|_| id.find("nested_name").is_none())) {
        if let Some(r) = template_id_call(cx, tid, &args, span) {
            return r;
        }
    }
    if let Some(name) = callee_name(callee) {
        if let Some((mut found, rule, text, nspan)) = primary::check::find(cx, name) {
            // NAM-27, NAM-29: an unqualified name also finds the functions
            // of the arguments' associated namespaces, unless ordinary
            // lookup finds something other than a function at namespace
            // scope.
            if rule == "NAM-8" {
                let plain = found.iter().all(|&e| {
                    matches!(cx.u.entities[e].kind, EntKind::Func(_) | EntKind::Template(_)) && cx.u.scopes[cx.u.entities[e].scope].kind == crate::sem::ScopeKind::Namespace
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
            // DED-P3: a function template adds the specialization that its
            // deduced arguments give.
            // DEP-9: in a specialization, argument-dependent lookup at the
            // point of instantiation can find declarations after the
            // template, which the first piece of build step 5 does not
            // weigh.
            if cx.lookup_limit.is_some() && rule == "NAM-8" {
                let tys: Vec<QType> = args.iter().map(|a| a.ty.clone()).collect();
                if crate::foundations::names::check::late_argument_dependent(cx, &text, &tys) {
                    let t = cx.unsupported(span, format!("a call of '{}' in a specialization, which argument-dependent lookup after the template can change", text));
                    return Expr::new(EK::Int(0), t, Cat::PR, span);
                }
            }
            if found.iter().any(|&e| matches!(cx.u.entities[e].kind, EntKind::Template(_))) && found.iter().all(|&e| matches!(cx.u.entities[e].kind, EntKind::Func(_) | EntKind::Template(_))) {
                let plain: Vec<EntityId> = found.iter().copied().filter(|&e| matches!(cx.u.entities[e].kind, EntKind::Func(_))).collect();
                let specs = crate::templates::declarations::check::call_candidates(cx, &found, &[], &args);
                let Some(f) = choose_with_templates(cx, &plain, &specs, &args, span, &text) else { return bad(span) };
                crate::declarations::functions::definitions::check::note_use(cx, f, nspan);
                let fty = QType::new(Type::Func(cx.u.func(f).unwrap().ty.clone()));
                let callee = Expr::new(EK::Func(f), fty, Cat::L, nspan);
                return finish_call(cx, callee, Some(f), args, span, None);
            }
            let funcs: Vec<EntityId> = found.iter().copied().filter(|&e| matches!(cx.u.entities[e].kind, EntKind::Func(_))).collect();
            if !funcs.is_empty() && funcs.len() == found.len() {
                let Some(f) = ovl::resolve(cx, &funcs, &args, span, &text) else { return bad(span) };
                crate::declarations::attributes::check::use_entity(cx, f, nspan);
                crate::declarations::functions::definitions::check::note_use(cx, f, nspan);
                let fty = QType::new(Type::Func(cx.u.func(f).unwrap().ty.clone()));
                let callee = Expr::new(EK::Func(f), fty, Cat::L, nspan);
                return finish_call(cx, callee, Some(f), args, span, None);
            }
            if found.iter().any(|&e| matches!(cx.u.entities[e].kind, EntKind::Unsupported(_))) {
                let t = cx.unsupported(span, format!("a call of '{}', which the front end does not model", text));
                return Expr::new(EK::Int(0), t, Cat::PR, span);
            }
        }
    }
    let f = match crate::expressions::member_pointers::check::called(callee) {
        Some(pm) => crate::expressions::member_pointers::check::pm_expr(cx, pm, true),
        None => expr(cx, callee),
    };
    if f.ty.is_error() {
        return bad(span);
    }
    // MPTR-3: a call through a pointer to member function.
    let f = match crate::expressions::member_pointers::check::split_call(f) {
        Ok((callee, object)) => return finish_call(cx, callee, None, args, span, Some(object)),
        Err(f) => f,
    };
    call_value(cx, f, args, span)
}

/// OVL-21: a call `(&C::f)(a)` whose set `C::f` holds a member function
/// with an implicit object parameter. The call is by name with no
/// implied object argument, so only the other functions of the set are
/// candidates. With the qualified name in parentheses, `&(C::f)`, the
/// call is an error (OVL-44). None for a callee of another form.
fn address_call(cx: &mut Check, callee: &Node, args: &[Expr], span: Span) -> Option<Expr> {
    let ("primary", [_, Kid::Node(u), _]) = (callee.prod, callee.kids.as_slice()) else { return None };
    if u.prod != "unary_expr" || cx.first_word(u) != "&" {
        return None;
    }
    let operand = nodes(u).last().copied()?;
    let (inner, paren) = match (operand.prod, operand.kids.as_slice()) {
        ("primary", [_, Kid::Node(x), _]) => (x, true),
        _ => (operand, false),
    };
    let id = callee_name(inner)?;
    id.find("qualified_name")?;
    let (found, _, text, nspan) = primary::check::find(cx, id)?;
    let funcs: Vec<EntityId> = found.iter().copied().filter(|&e| cx.u.func(e).is_some()).collect();
    if funcs.is_empty() || funcs.len() != found.len() {
        return None;
    }
    let object = |cx: &Check, f: EntityId| cx.u.func(f).is_some_and(|x| x.class.is_some() && !x.member_static);
    if !funcs.iter().any(|&f| object(cx, f)) {
        return None;
    }
    let written = cx.source(id);
    if paren {
        cx.error_cause(span, "OVL-44", Some("MPTR-2"), format!("'&({})' names a member function with an implicit object parameter in parentheses", written));
        return Some(bad(span));
    }
    // The functions whose parameters the arguments can fill.
    let fits = |cx: &Check, f: EntityId| cx.u.func(f).is_some_and(|x| {
        let n = x.ty.params.len();
        let defaults = x.defaults.iter().filter(|d| d.is_some()).count();
        (args.len() <= n || x.ty.variadic) && args.len() + defaults >= n
    });
    let others: Vec<EntityId> = funcs.iter().copied().filter(|&f| !object(cx, f) && fits(cx, f)).collect();
    if others.is_empty() {
        cx.error(span, "OVL-21", format!("the call '(&{})(...)' has no implied object argument, so no function of the set fits", written));
        return Some(bad(span));
    }
    let f = ovl::resolve(cx, &others, args, span, &text)?;
    crate::declarations::functions::definitions::check::note_use(cx, f, nspan);
    let fty = QType::new(Type::Func(cx.u.func(f).unwrap().ty.clone()));
    let callee = Expr::new(EK::Func(f), fty, Cat::L, nspan);
    Some(finish_call(cx, callee, Some(f), args.to_vec(), span, None))
}

/// OVL-29: the best of the functions `plain` and the specializations
/// `specs` of function templates. A function that is not a
/// specialization wins a tie, which the front end takes as a function
/// that takes every argument by an exact match.
fn choose_with_templates(cx: &mut Check, plain: &[EntityId], specs: &[EntityId], args: &[Expr], span: Span, name: &str) -> Option<EntityId> {
    use crate::overloading::conversion_sequences::check::{sequence, Rank};
    let exact = plain.iter().copied().filter(|&f| cx.u.func(f).is_some_and(|x| x.ty.params.len() == args.len() && x.ty.params.iter().zip(args).all(|(p, a)| sequence(cx, a, p).is_some_and(|s| s.rank == Rank::Exact)))).collect::<Vec<_>>();
    if exact.len() == 1 {
        return Some(exact[0]);
    }
    // A call that no template's arguments deduce for: the first piece of
    // build step 5 deduces only some forms of parameters.
    if specs.is_empty() && plain.is_empty() {
        let t = cx.unsupported(span, format!("a call of the function template '{}' whose arguments the first piece of build step 5 does not deduce", name));
        let _ = t;
        return None;
    }
    let mut all: Vec<EntityId> = plain.to_vec();
    for &f in specs {
        if !all.contains(&f) {
            all.push(f);
        }
    }
    if all.is_empty() {
        cx.error(span, "OVL-35", format!("no function '{}' fits the arguments", name));
        return None;
    }
    ovl::resolve(cx, &all, args, span, name)
}

/// TARG-1: `f<A...>(args)`, a call of the function templates named `f`
/// with explicit type arguments. None for a callee that names no
/// function template, or arguments that are not types.
fn template_id_call(cx: &mut Check, tid: &Node, args: &[Expr], span: Span) -> Option<Expr> {
    let name = cx.source(tid.child("template_name")?);
    let found: Vec<EntityId> = crate::foundations::names::check::lookup(cx, &name).into_iter().filter(|&e| matches!(cx.u.entities[e].kind, EntKind::Template(_))).collect();
    if found.is_empty() {
        return None;
    }
    let mut explicit = Vec::new();
    if let Some(ta) = tid.child("template_args") {
        for a in ta.children("template_arg") {
            let t = a.child("type_id")?;
            let ty = crate::declarations::declarators::check::type_id(cx, t);
            if ty.is_error() {
                return Some(bad(span));
            }
            explicit.push(ty);
        }
    }
    let specs = crate::templates::declarations::check::call_candidates(cx, &found, &explicit, args);
    let f = choose_with_templates(cx, &[], &specs, args, span, &name)?;
    crate::declarations::functions::definitions::check::note_use(cx, f, tid.span);
    let fty = QType::new(Type::Func(cx.u.func(f).unwrap().ty.clone()));
    let callee = Expr::new(EK::Func(f), fty, Cat::L, tid.span);
    Some(finish_call(cx, callee, Some(f), args.to_vec(), span, None))
}

/// A call with a braced list as an argument, whose name finds one
/// function (PFX-6). The list copy-list-initializes its parameter, and
/// it adds no associated namespace (NAM-28). A list for a parameter of
/// several candidates needs the list sequences of ICS-17, which a later
/// piece weighs.
fn braced_call(cx: &mut Check, callee: &Node, list: Option<&Node>, span: Span) -> Expr {
    let unsupported = |cx: &mut Check, why: &str| {
        let t = cx.unsupported(span, why.to_string());
        Expr::new(EK::Int(0), t, Cat::PR, span)
    };
    let Some(name) = callee_name(callee) else { return unsupported(cx, "a braced list as an argument of a call that is not of a name") };
    let nodes = arg_nodes(list);
    let mut args: Vec<Option<Expr>> = Vec::new();
    for a in &nodes {
        if first_node(a).is_some_and(|c| c.prod == "braced_init") {
            args.push(None);
        } else {
            let e = expr(cx, a);
            if e.ty.is_error() {
                return bad(span);
            }
            args.push(Some(e));
        }
    }
    let Some((mut found, rule, text, nspan)) = primary::check::find(cx, name) else { return bad(span) };
    if rule == "NAM-8" && found.iter().all(|&e| matches!(cx.u.entities[e].kind, EntKind::Func(_)) && cx.u.scopes[cx.u.entities[e].scope].kind == crate::sem::ScopeKind::Namespace) {
        let tys: Vec<QType> = args.iter().flatten().map(|a| a.ty.clone()).collect();
        for e in crate::foundations::names::check::argument_dependent(cx, &text, &tys) {
            if !found.contains(&e) {
                found.push(e);
            }
        }
    }
    if found.is_empty() {
        if !rule.is_empty() {
            cx.not_found(nspan, rule, &text);
        }
        return bad(span);
    }
    let f = match found[..] {
        [f] => f,
        _ => match braced_choice(cx, &found, &args, &nodes, span, &text) {
            Some(f) => f,
            None => return bad(span),
        },
    };
    let Some(func) = cx.u.func(f).cloned() else { return unsupported(cx, "a braced list as an argument of a call of a name that is not a function") };
    if func.class.is_some() && !func.member_static {
        return unsupported(cx, "a braced list as an argument of a member function call");
    }
    let mut out = Vec::new();
    for (k, (a, n)) in args.into_iter().zip(&nodes).enumerate() {
        match a {
            Some(e) => out.push(e),
            None => {
                // ICS-6: a braced list has no sequence to an ellipsis.
                let Some(p) = func.ty.params.get(k) else {
                    cx.error(span, "PFX-5", format!("the call gives {} arguments for {} parameters", nodes.len(), func.ty.params.len()));
                    return bad(span);
                };
                let b = first_node(n).unwrap();
                // PFX-16: the parameter is copy-initialized, so an explicit
                // constructor takes no part.
                let n_errs = cx.u.diags.len();
                let Some(e) = crate::declarations::initialization::references::check::braced_argument(cx, b, p) else {
                    explicit_in_argument(cx, n_errs);
                    return bad(span);
                };
                out.push(e);
            }
        }
    }
    crate::declarations::attributes::check::use_entity(cx, f, nspan);
    crate::declarations::functions::definitions::check::note_use(cx, f, nspan);
    let callee = Expr::new(EK::Func(f), QType::new(Type::Func(func.ty.clone())), Cat::L, nspan);
    finish_call(cx, callee, Some(f), out, span, None)
}

/// PFX-16: a parameter is copy-initialized, so an explicit constructor
/// takes no part. The first error after `from` that names CTOR-9 names
/// PFX-16 instead.
fn explicit_in_argument(cx: &mut Check, from: usize) {
    if let Some(d) = cx.u.diags[from..].iter_mut().find(|d| d.sev == crate::diag::Severity::Error && d.rule == Some("CTOR-9")) {
        d.rule = Some("PFX-16");
        d.cause = Some("CTOR-9");
    }
}

/// OVL-29, ICS-17: the best of the functions `found` for arguments that
/// hold braced lists. A list has the sequence of a trial initialization
/// of its parameter and no sequence to an ellipsis (ICS-6). None after an
/// error.
fn braced_choice(cx: &mut Check, found: &[EntityId], args: &[Option<Expr>], nodes: &[&Node], span: Span, name: &str) -> Option<EntityId> {
    use crate::overloading::conversion_sequences::check::{better, sequence, Ics, Rank};
    let mut viable: Vec<(EntityId, Vec<Ics>)> = Vec::new();
    for &f in found {
        let Some(g) = cx.u.func(f).cloned() else {
            cx.unsupported(span, "a braced list as an argument of a call of a name that is not only functions");
            return None;
        };
        if g.class.is_some() && !g.member_static {
            cx.unsupported(span, "a braced list as an argument of a member function call");
            return None;
        }
        let n = g.ty.params.len();
        if (args.len() > n && !g.ty.variadic) || (args.len()..n).any(|k| g.defaults.get(k).is_none_or(|d| d.is_none())) {
            continue;
        }
        let mut seqs = Vec::new();
        for (k, a) in args.iter().enumerate() {
            let s = match (a, g.ty.params.get(k)) {
                (Some(e), Some(p)) => sequence(cx, e, p),
                (Some(_), None) => Some(Ics { rank: Rank::Conversion, qual: false, binds: None, ellipsis: true, user: None }),
                (None, Some(p)) => crate::classes::constructors::check::list_sequence(cx, first_node(nodes[k]).unwrap(), p, false),
                (None, None) => None,
            };
            match s {
                Some(s) => seqs.push(s),
                None => break,
            }
        }
        if seqs.len() == args.len() {
            viable.push((f, seqs));
        }
    }
    if viable.is_empty() {
        cx.error(span, "OVL-35", format!("no function '{}' fits the arguments", name));
        return None;
    }
    let wins = |a: &[Ics], b: &[Ics]| !a.iter().zip(b).any(|(x, y)| better(y, x)) && a.iter().zip(b).any(|(x, y)| better(x, y));
    let mut best = 0;
    for j in 1..viable.len() {
        if wins(&viable[j].1, &viable[best].1) {
            best = j;
        }
    }
    if (0..viable.len()).any(|j| j != best && !wins(&viable[best].1, &viable[j].1)) {
        cx.error(span, "OVL-36", format!("the call of '{}' is ambiguous", name));
        return None;
    }
    Some(viable[best].0)
}

/// A call of the value `f`: a function, a pointer to function, or a class
/// object with a call operator.
pub fn call_value(cx: &mut Check, f: Expr, args: Vec<Expr>, span: Span) -> Expr {
    let f = deref_ref(f);
    // OVL-6: a conversion function to a pointer or reference to function
    // gives a surrogate call function.
    let ops = crate::expressions::lambdas::check::call_operators(cx, &f);
    if let Some(c) = crate::classes::conversion_functions::check::converting_class(cx, &f) {
        let convs: Vec<EntityId> = crate::classes::conversion_functions::check::functions(&cx.u, c).into_iter().filter(|&g| cx.u.func(g).is_some_and(|x| !x.explicit && to_function(&x.ty.ret).is_some())).collect();
        if !convs.is_empty() {
            return surrogate_call(cx, f, &ops, &convs, args, span);
        }
    }
    // LAM-1, OPR-3: a call of a class object calls its call operator.
    if !ops.is_empty() {
        let Some(op) = ovl::resolve_with(cx, &ops, Some(&f), &args, span, "operator()") else { return bad(span) };
        return call_operator(cx, f, op, args, span);
    }
    // PFX-17: a class object without a call operator is not callable.
    if f.ty.is_class() {
        return not_callable(cx, &f, span);
    }
    let target = match &f.k {
        EK::Func(id) => Some(*id),
        _ => None,
    };
    finish_call(cx, f, target, args, span, None)
}

/// The function type that a conversion function's result `ret` calls:
/// a pointer to function, a reference to such a pointer, or a reference
/// to function (OVL-6).
fn to_function(ret: &QType) -> Option<crate::sem::types::FnType> {
    let r = ret.nonref();
    match (&r.t, r.pointee()) {
        (Type::Func(ft), _) => Some((**ft).clone()),
        (_, Some(p)) => match &p.t {
            Type::Func(ft) => Some((**ft).clone()),
            _ => None,
        },
        _ => None,
    }
}

/// OVL-6: a call of the class object `f` whose conversion functions
/// `convs` give surrogate call functions, weighed with the call
/// operators `ops`. A surrogate takes the object by a user-defined
/// sequence through its conversion function, and the arguments by the
/// parameters of the function type it converts to.
fn surrogate_call(cx: &mut Check, f: Expr, ops: &[EntityId], convs: &[EntityId], args: Vec<Expr>, span: Span) -> Expr {
    use crate::overloading::conversion_sequences::check::{better, sequence, Ics, Rank};
    let obj = f.ty.nonref();
    let lvalue = f.cat == Cat::L;
    // The sequences of each argument for parameters `params`, None when
    // the candidate is not viable (OVL-22).
    let fits = |cx: &Check, params: &[QType], defaults: usize, variadic: bool| -> Option<Vec<Ics>> {
        if (args.len() > params.len() && !variadic) || args.len() + defaults < params.len() {
            return None;
        }
        args.iter().enumerate().map(|(k, a)| match params.get(k) {
            Some(p) => sequence(cx, a, p),
            None => Some(Ics { rank: Rank::Conversion, qual: false, binds: None, ellipsis: true, user: None }),
        }).collect()
    };
    // A candidate: a call operator, or a surrogate by its conversion
    // function, with the object's sequence first.
    let mut viable: Vec<(EntityId, bool, Vec<Ics>)> = Vec::new();
    for &op in ops {
        let Some(func) = cx.u.func(op) else { continue };
        let skip = usize::from(func.explicit_object);
        let defaults = func.defaults[skip.min(func.defaults.len())..].iter().filter(|d| d.is_some()).count();
        let (params, variadic) = (func.ty.params[skip..].to_vec(), func.ty.variadic);
        let Some(o) = ovl::object_sequence(cx, op, &f) else { continue };
        if let Some(mut seqs) = fits(cx, &params, defaults, variadic) {
            seqs.insert(0, o);
            viable.push((op, false, seqs));
        }
    }
    for &g in convs {
        let Some(func) = cx.u.func(g) else { continue };
        // The cv-qualifiers of the function include the object's, and its
        // ref-qualifier takes the object's category.
        if (obj.c && !func.obj_cv.0) || (obj.v && !func.obj_cv.1) {
            continue;
        }
        let binds = match func.ref_q {
            Some(true) => !lvalue,
            Some(false) => lvalue || (func.obj_cv.0 && !func.obj_cv.1),
            None => true,
        };
        if !binds {
            continue;
        }
        let Some(ft) = to_function(&func.ty.ret) else { continue };
        if let Some(mut seqs) = fits(cx, &ft.params, 0, ft.variadic) {
            seqs.insert(0, Ics { rank: Rank::User, qual: false, binds: None, ellipsis: false, user: Some((g, Rank::Exact)) });
            viable.push((g, true, seqs));
        }
    }
    if viable.is_empty() {
        cx.error(span, "OVL-35", format!("no call operator or surrogate call function of {} fits the arguments", obj.show()));
        return bad(span);
    }
    // OVL-29: the best candidate is better than every other.
    let wins = |a: &[Ics], b: &[Ics]| !a.iter().zip(b).any(|(x, y)| better(y, x)) && a.iter().zip(b).any(|(x, y)| better(x, y));
    let mut best = 0;
    for i in 1..viable.len() {
        if wins(&viable[i].2, &viable[best].2) {
            best = i;
        }
    }
    if (0..viable.len()).any(|i| i != best && !wins(&viable[best].2, &viable[i].2)) {
        cx.error(span, "OVL-36", format!("the call of an object of {} is ambiguous between its call operators and surrogate call functions", obj.show()));
        return bad(span);
    }
    let (g, surrogate, _) = viable[best].clone();
    if !surrogate {
        return call_operator(cx, f, g, args, span);
    }
    let callee = crate::classes::conversion_functions::check::call(cx, g, f, span);
    if callee.ty.is_error() {
        return bad(span);
    }
    let callee = deref_ref(callee);
    let callee = if callee.ty.is_ptr() { rvalue(cx, callee) } else { callee };
    finish_call(cx, callee, None, args, span, None)
}

/// LAM-1, OPR-3: the call of the call operator `op` of the class object
/// `f`.
fn call_operator(cx: &mut Check, f: Expr, op: EntityId, args: Vec<Expr>, span: Span) -> Expr {
    {
        let func = cx.u.func(op).unwrap().clone();
        let fty = QType::new(Type::Func(func.ty.clone()));
        let callee = Expr::new(EK::Func(op), fty, Cat::L, f.span);
        let mut args = args;
        if func.explicit_object {
            args.insert(0, f);
            return finish_call(cx, callee, Some(op), args, span, None);
        }
        let object = if func.member_static { None } else { Some(f) };
        finish_call(cx, callee, Some(op), args, span, object)
    }
}

/// A call of a member function (PFX-10, ICS-13): `x.f(...)`, `p->f(...)`,
/// or a name that finds member functions inside a member function of
/// their class, whose object is `*this`. None for any other callee.
fn member_call(cx: &mut Check, callee: &Node, args: &[Expr], span: Span) -> Option<Expr> {
    // A parenthesized member access calls the member as without the
    // parentheses (OVL-2).
    let mut callee = callee;
    while let ("primary", [_, Kid::Node(inner), _]) = (callee.prod, callee.kids.as_slice()) {
        callee = inner;
    }
    // A qualified name calls the function it names, not its final
    // overrider (VIRT-1).
    let mut qualified = false;
    let (object, scope, name, nspan) = if callee.prod == "postfix_expr" && matches!(callee.kids.get(1).map(|k| cx.text(k)), Some("." | "->")) {
        let (Some(Kid::Node(base)), Some(Kid::Node(id))) = (callee.kids.first(), callee.kids.get(2)) else { return None };
        let arrow = cx.text(&callee.kids[1]) == "->";
        // Only a member that is a function: a callee of another kind
        // goes through member access.
        let mut u = first_node(id)?;
        // `x.B::f(...)`: the qualifier names the class whose function the
        // call takes, without dispatch (VIRT-1).
        let qual_node = if u.prod == "qualified_name" {
            let nn = u.child("nested_name")?;
            u = u.child("unqualified_id")?;
            Some(nn)
        } else {
            None
        };
        if u.prod != "unqualified_id" || u.kids.len() != 1 {
            return None;
        }
        // A conversion function called by its name (CVF-1).
        let name = match u.kids.first() {
            Some(Kid::Node(cf)) if cf.prod == "conversion_function_id" => {
                let t = crate::classes::conversion_functions::check::target(cx, cf);
                if t.is_error() {
                    return Some(bad(span));
                }
                crate::classes::conversion_functions::check::name(&t)
            }
            _ => crate::classes::operators::check::unqualified_name(cx, u)?.to_string(),
        };
        let b = expr(cx, base);
        if b.ty.is_error() {
            return Some(bad(span));
        }
        let obj = if arrow {
            // OPR-5: `->` on a class object calls its operator->.
            let b = if b.ty.nonref().is_class() { crate::classes::operators::check::arrow(cx, b, span) } else { b };
            if b.ty.is_error() {
                return Some(bad(span));
            }
            let b = rvalue(cx, b);
            match b.ty.pointee() {
                Some(p) if p.is_class() => crate::expressions::unary::check::indirection(cx, b, span),
                _ => {
                    cx.error(span, "PFX-11", format!("'->' on {}, which is not a pointer to a class", b.ty.show()));
                    return Some(bad(span));
                }
            }
        } else {
            deref_ref(b)
        };
        let Some(c) = obj.ty.class() else {
            cx.error(span, "PFX-11", format!("member access on {}, which is not a class", obj.ty.show()));
            return Some(bad(span));
        };
        if let Some(nn) = qual_node {
            let saved = std::mem::replace(&mut cx.scope, cx.u.classes[c].scope);
            let q = crate::foundations::names::check::nested_name(cx, nn);
            cx.scope = saved;
            let crate::foundations::names::check::Qual::Scope(s) = q else { return Some(bad(span)) };
            let k = cx.u.class_of_scope(s)?;
            let funcs = crate::foundations::names::check::lookup_in(cx, s, &name);
            if !crate::classes::derived::check::derives(cx, c, k) || funcs.is_empty() || !funcs.iter().all(|&e| matches!(cx.u.entities[e].kind, EntKind::Func(_))) {
                // Member access gives the error or the data member.
                let m = crate::classes::members::check::member_access(cx, obj, false, id, span);
                if m.ty.is_error() {
                    return Some(m);
                }
                return Some(call_value(cx, m, args.to_vec(), span));
            }
            let decl = cx.u.func(funcs[0]).and_then(|f| f.class).unwrap_or(k);
            return member_call_tail(cx, Some(obj), cx.u.classes[decl].scope, name, id.span, true, args, span);
        }
        let (found, decl) = match crate::classes::derived::check::member_lookup(cx, c, &name) {
            Ok(Some(x)) => (x.ents, x.class),
            Ok(None) => (Vec::new(), c),
            Err(()) => {
                cx.error(span, "DER-4", format!("the member '{}' is in more than one base", name));
                return Some(bad(span));
            }
        };
        if found.is_empty() || !found.iter().all(|&e| matches!(cx.u.entities[e].kind, EntKind::Func(_))) {
            // A data member that holds a function pointer, or no member.
            let m = crate::classes::members::check::member_access(cx, obj, false, id, span);
            if m.ty.is_error() {
                return Some(m);
            }
            return Some(call_value(cx, m, args.to_vec(), span));
        }
        (Some(obj), cx.u.classes[decl].scope, name, id.span)
    } else {
        let n = callee_name(callee)?;
        let (q, name, nspan) = primary::check::name_of(cx, n)?;
        qualified = !matches!(q, crate::foundations::names::check::Qual::None);
        let found = match q {
            crate::foundations::names::check::Qual::None => crate::foundations::names::check::lookup(cx, name),
            crate::foundations::names::check::Qual::Scope(s) => crate::foundations::names::check::lookup_in(cx, s, name),
            _ => return None,
        };
        let members: Vec<EntityId> = found.iter().copied().filter(|&e| cx.u.func(e).is_some_and(|f| f.class.is_some())).collect();
        if members.is_empty() || members.len() != found.len() {
            return None;
        }
        let class = cx.u.func(members[0]).unwrap().class.unwrap();
        // Inside a member function of the class, the object is `*this`.
        let this = cx.func.as_ref().and_then(|f| f.this.clone()).filter(|t| t.pointee().and_then(|p| p.class()).is_some_and(|k| crate::classes::derived::check::derives(cx, k, class)));
        let mut obj = this.map(|t| Expr::new(EK::Deref(Box::new(Expr::new(EK::This, t.clone(), Cat::PR, nspan))), t.pointee().unwrap().clone(), Cat::L, nspan));
        // LAM-8: in a lambda body, the object is the one that the capture
        // of `this` holds, captured at this use under a capture default.
        let outer = cx.lambdas.last().and_then(|l| l.outer_this.clone()).filter(|t| t.pointee().and_then(|p| p.class()).is_some_and(|k| crate::classes::derived::check::derives(cx, k, class)));
        if obj.is_none() && crate::expressions::lambdas::check::in_lambda_body(cx) && outer.is_some() && members.iter().any(|&f| !cx.u.func(f).unwrap().member_static) {
            let top = cx.lambdas.len().checked_sub(1);
            let Some(p) = crate::expressions::lambdas::check::this_at(cx, top, nspan) else { return Some(bad(span)) };
            let t = p.ty.pointee().cloned().unwrap_or_else(QType::error);
            obj = Some(Expr::new(EK::Deref(Box::new(p)), t, Cat::L, nspan));
        }
        let mixed = members.iter().any(|&f| cx.u.func(f).unwrap().member_static) && members.iter().any(|&f| !cx.u.func(f).unwrap().member_static);
        let other_this = cx.func.as_ref().is_some_and(|f| f.this.is_some());
        if obj.is_none() && (members.iter().any(|&f| cx.u.func(f).unwrap().ref_q.is_some()) || (mixed && (other_this || members.iter().any(|&f| cx.u.func(f).unwrap().defaults.iter().any(|d| d.is_some()))))) {
            return Some(contrived_call(cx, &members, args, nspan, span, name));
        }
        if obj.is_none() && members.iter().any(|&f| !cx.u.func(f).unwrap().member_static) && !members.iter().any(|&f| cx.u.func(f).unwrap().member_static) {
            let t = cx.unsupported(span, "a non-static member function called without an object (build step 4)");
            return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
        }
        (obj, cx.u.classes[class].scope, name.to_string(), nspan)
    };
    member_call_tail(cx, object, scope, name, nspan, qualified, args, span)
}

/// OVL-3: a call by name of member functions with no object to give
/// them. The implied object argument is a contrived object that every
/// candidate's object parameter takes, so the arguments decide. The best
/// candidate must be static (OVL-23); otherwise the call has no object.
fn contrived_call(cx: &mut Check, members: &[EntityId], args: &[Expr], nspan: Span, span: Span, name: &str) -> Expr {
    use crate::overloading::conversion_sequences::check::{better, sequence, Ics, Rank};
    let exact = Ics { rank: Rank::Exact, qual: false, binds: None, ellipsis: false, user: None };
    let mut viable: Vec<(EntityId, Vec<Ics>)> = Vec::new();
    for &f in members {
        let Some(g) = cx.u.func(f).cloned() else { continue };
        let skip = usize::from(g.explicit_object);
        let params = &g.ty.params[skip.min(g.ty.params.len())..];
        let defaults = &g.defaults[skip.min(g.defaults.len())..];
        if (args.len() > params.len() && !g.ty.variadic) || (args.len()..params.len()).any(|k| defaults.get(k).is_none_or(|d| d.is_none())) {
            continue;
        }
        let mut seqs = vec![exact.clone()];
        for (k, a) in args.iter().enumerate() {
            let s = match params.get(k) {
                Some(p) => sequence(cx, a, p),
                None => Some(Ics { rank: Rank::Conversion, qual: false, binds: None, ellipsis: true, user: None }),
            };
            match s {
                Some(s) => seqs.push(s),
                None => break,
            }
        }
        if seqs.len() == args.len() + 1 {
            viable.push((f, seqs));
        }
    }
    if viable.is_empty() {
        cx.error(span, "OVL-35", format!("no function '{}' fits the arguments", name));
        return bad(span);
    }
    let wins = |a: &[Ics], b: &[Ics]| !a.iter().zip(b).any(|(x, y)| better(y, x)) && a.iter().zip(b).any(|(x, y)| better(x, y));
    let mut best = 0;
    for j in 1..viable.len() {
        if wins(&viable[j].1, &viable[best].1) {
            best = j;
        }
    }
    if (0..viable.len()).any(|j| j != best && !wins(&viable[best].1, &viable[j].1)) {
        cx.error_cause(span, "OVL-36", Some("OVL-3"), format!("the call of '{}' is ambiguous", name));
        return bad(span);
    }
    let f = viable[best].0;
    let g = cx.u.func(f).unwrap().clone();
    if !g.member_static {
        cx.error_cause(span, "OVL-23", Some("OVL-3"), format!("the call of '{}' chooses a non-static member function, and there is no object", name));
        return bad(span);
    }
    crate::declarations::functions::definitions::check::note_use(cx, f, nspan);
    let callee = Expr::new(EK::Func(f), QType::new(Type::Func(g.ty.clone())), Cat::L, nspan);
    finish_call(cx, callee, Some(f), args.to_vec(), span, None)
}

/// A call `obj.name(args)` of a member that lookup by name finds in the
/// class of `obj`, as the range-based `for` makes (RFOR-2): a member
/// function, or a data member of a callable type. None when the class
/// has no such member.
pub fn call_member(cx: &mut Check, obj: Expr, name: &str, args: &[Expr], span: Span) -> Option<Expr> {
    let c = obj.ty.class()?;
    let (found, decl) = match crate::classes::derived::check::member_lookup(cx, c, name) {
        Ok(Some(x)) => (x.ents, x.class),
        Ok(None) => return None,
        Err(()) => {
            cx.error(span, "DER-4", format!("the member '{}' is in more than one base", name));
            return Some(bad(span));
        }
    };
    if !found.iter().all(|&e| matches!(cx.u.entities[e].kind, EntKind::Func(_))) {
        let [f] = found[..] else { return None };
        if !matches!(cx.u.entities[f].kind, EntKind::Field(_)) {
            let t = cx.unsupported(span, format!("a call of the member '{}' of this kind", name));
            return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
        }
        let m = crate::classes::members::check::field_of(cx, obj, f, span);
        if m.ty.is_error() {
            return Some(m);
        }
        return Some(call_value(cx, m, args.to_vec(), span));
    }
    member_call_tail(cx, Some(obj), cx.u.classes[decl].scope, name.to_string(), span, false, args, span)
}

/// The call of the member functions named `name` in `scope`, with the
/// object, if any.
#[allow(clippy::too_many_arguments)]
fn member_call_tail(cx: &mut Check, object: Option<Expr>, scope: crate::sem::ScopeId, name: String, nspan: Span, qualified: bool, args: &[Expr], span: Span) -> Option<Expr> {
    let mut cands: Vec<EntityId> = crate::foundations::names::check::declared_here(cx, scope, &name).into_iter().filter(|&e| matches!(cx.u.entities[e].kind, EntKind::Func(_))).collect();
    if let Some(c) = cx.u.class_of_scope(scope) {
        crate::classes::derived::check::hide_brought(cx, c, &mut cands);
    }
    let f = ovl::resolve_with(cx, &cands, object.as_ref(), args, span, &name)?;
    // OVL-3: a non-static member function chosen with no object.
    if object.is_none() && cx.u.func(f).is_some_and(|x| !x.member_static) {
        let t = cx.unsupported(span, "a non-static member function chosen without an object (build step 4)");
        return Some(Expr::new(EK::Int(0), t, Cat::PR, span));
    }
    let naming = object.as_ref().and_then(|o| o.ty.class()).or(cx.u.func(f).and_then(|x| x.class));
    let ok = match naming {
        Some(n) => crate::classes::derived::check::accessible_through(cx, f, n),
        None => crate::classes::members::check::accessible(cx, f),
    };
    if !ok {
        cx.error(span, "ACC-2", format!("the member '{}' is not public", name));
        return Some(bad(span));
    }
    crate::declarations::functions::definitions::check::note_use(cx, f, nspan);
    let func = cx.u.func(f).unwrap().clone();
    let fty = QType::new(Type::Func(func.ty.clone()));
    let kind = if cx.u.virt.contains_key(&f) && !qualified { EK::Virtual(f) } else { EK::Func(f) };
    let callee = Expr::new(kind, fty, Cat::L, nspan);
    // CLS-13: the object is the first argument of a function with an
    // explicit object parameter.
    let mut args = args.to_vec();
    if let Some(o) = object.clone().filter(|_| func.explicit_object) {
        args.insert(0, o);
    }
    let object = object.filter(|_| !func.member_static);
    // DER-3: the object of a member of a base is the base subobject.
    let object = match (object, func.class) {
        (Some(o), Some(k)) if o.ty.class().is_some_and(|oc| oc != k) => {
            let oc = o.ty.class().unwrap();
            let ps = crate::classes::derived::check::paths(cx, oc, k);
            if ps.len() != 1 {
                cx.error(span, "DER-4", format!("the member '{}' is in more than one base subobject", name));
                return Some(bad(span));
            }
            if !crate::classes::derived::check::accessible_through(cx, f, oc) {
                cx.error(span, "ACC-2", format!("the member '{}' is not public", name));
                return Some(bad(span));
            }
            Some(crate::classes::derived::check::along(cx, o, oc, &ps[0]))
        }
        (o, _) => o,
    };
    // ICS-13: the object binds the implicit object parameter.
    if let Some(o) = &object {
        if (o.ty.c && !func.obj_cv.0) || (o.ty.v && !func.obj_cv.1) {
            cx.error_cause(span, "OVL-35", Some("ICS-13"), format!("the member function '{}' does not take a {} object", name, if o.ty.c { "const" } else { "volatile" }));
            return Some(bad(span));
        }
        // CLS-6: `&&` takes an rvalue object, and `&` an lvalue unless it
        // is also const.
        let lvalue = o.cat == Cat::L;
        let binds = match func.ref_q {
            Some(true) => !lvalue,
            Some(false) => lvalue || (func.obj_cv.0 && !func.obj_cv.1),
            None => true,
        };
        if !binds {
            cx.error_cause(span, "OVL-35", Some("ICS-40"), format!("the member function '{}' does not take an {} object", name, if lvalue { "lvalue" } else { "rvalue" }));
            return Some(bad(span));
        }
    }
    Some(finish_call(cx, callee, Some(f), args, span, object))
}

/// The call of a chosen function or of a function pointer: the
/// arguments initialize the parameters.
pub fn finish_call(cx: &mut Check, f: Expr, target: Option<EntityId>, args: Vec<Expr>, span: Span, object: Option<Expr>) -> Expr {
    let fty = match &f.ty.t {
        Type::Func(ft) => ft.clone(),
        Type::Ptr(p) => match &p.t {
            Type::Func(ft) => ft.clone(),
            _ => return not_callable(cx, &f, span),
        },
        Type::Int(_) | Type::Float(_) | Type::Enum(_) | Type::NullPtr | Type::Array(..) | Type::Void => return not_callable(cx, &f, span),
        Type::MemPtr(..) => {
            cx.error(span, "MPTR-10", format!("a call of {}, a pointer to member, without '.*' or '->*'", f.ty.show()));
            return bad(span);
        }
        _ => {
            let t = cx.unsupported(span, "a call of an object of this type");
            return Expr::new(EK::Int(0), t, Cat::PR, span);
        }
    };
    let f = if f.ty.is_func() { f } else { rvalue(cx, f) };
    // PFX-14: no parameter or return type of incomplete class, outside
    // an unevaluated operand.
    let incomplete = |cx: &Check, t: &QType| t.class().is_some_and(|c| !cx.u.classes[c].complete);
    if cx.unevaluated == 0 && (incomplete(cx, &fty.ret) || fty.params.iter().any(|p| incomplete(cx, p))) {
        cx.error(span, "PFX-14", "a call of a function whose parameter or return type is an incomplete class");
        return bad(span);
    }
    // VIRT-8: the call makes its result and its arguments as objects.
    if cx.unevaluated == 0 && (!crate::classes::r#virtual::check::not_abstract(cx, &fty.ret, span) || fty.params.iter().any(|p| !crate::classes::r#virtual::check::not_abstract(cx, p, span))) {
        return bad(span);
    }
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
                let n_errs = cx.u.diags.len();
                match crate::declarations::initialization::forms::check::init_parameter(cx, a, &fty.params[k]) {
                    Some(arg) => out.push(arg),
                    None => {
                        explicit_in_argument(cx, n_errs);
                        return bad(span);
                    }
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
    let no_temporary = std::mem::take(&mut cx.no_temporary);
    if ret.is_class() && !no_temporary && !crate::classes::constructors::check::dtor_usable(cx, &ret, span) {
        return bad(span);
    }
    let (ty, cat) = match &ret.t {
        Type::Ref(inner, rv) => ((**inner).clone(), if *rv && !inner.is_func() { Cat::X } else { Cat::L }),
        // A class prvalue keeps its cv-qualifiers (VC rules).
        _ if ret.is_class() => (ret.clone(), Cat::PR),
        _ => (ret.unqual_top(), Cat::PR),
    };
    if let Some(t) = target {
        crate::declarations::functions::definitions::check::called(cx, t, span);
    }
    // The object of a member function comes first, as a place; a prvalue
    // object is materialized.
    if let Some(o) = object {
        let o = if o.cat == Cat::PR {
            let t = o.ty.clone();
            o.conv(crate::sem::tree::Conv::Materialize, t, Cat::X)
        } else {
            o
        };
        out.insert(0, Arg { e: o, by_ref: true });
    }
    let call = Expr::new(EK::Call(Box::new(f), out, false), ty, cat, span);
    if let Some(t) = target {
        return crate::declarations::constexpr::check::immediate_call(cx, t, call);
    }
    call
}

fn not_callable(cx: &mut Check, f: &Expr, span: Span) -> Expr {
    cx.error(span, "PFX-17", format!("a call of {}, which is not a function", f.ty.show()));
    bad(span)
}
