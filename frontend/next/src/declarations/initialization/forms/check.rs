//! The check part of `declarations/initialization/forms`: the
//! declaration of a variable with its initializer, and the
//! initialization of a parameter by an argument.

use crate::declarations::check::Specs;
use crate::declarations::declarators::check::Declared;
use crate::declarations::initialization::{list, references};
use crate::declarations::{alignment, attributes, auto, constexpr, storage_linkage};
use crate::expressions::check::{expr, first_node, nodes};
use crate::foundations::constant_evaluation::check as ce;
use crate::foundations::conversions::check as conv;
use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::tree::{Arg, Cat, Expr, Init, EK};
use crate::sem::types::{QType, Type};
use crate::sem::{EntKind, EntityId, Linkage, Storage, Var};
use crate::source::Span;
use crate::syntax::{Kid, Node};

/// Copy-initialization of an object of type `to` from an expression
/// (INIT-4): an implicit conversion. `rule` names the failure.
pub fn copy_init(cx: &mut Check, e: Expr, to: &QType, rule: &'static str) -> Option<Expr> {
    let e = select_overload(cx, e, to)?;
    if e.ty.is_error() || to.is_error() {
        return None;
    }
    // DER-5: an object of a derived class initializes a base object from
    // its base part.
    let e = match (e.ty.nonref().class(), to.class()) {
        (Some(d), Some(b)) if crate::classes::derived::check::is_base(cx, d, b) => {
            let e = crate::foundations::values::check::deref_ref(e);
            let e = if e.cat == crate::sem::tree::Cat::PR {
                let t = e.ty.clone();
                e.conv(crate::sem::tree::Conv::Materialize, t, crate::sem::tree::Cat::X)
            } else {
                e
            };
            let span = e.span;
            crate::classes::derived::check::to_base(cx, e, b, span)?
        }
        _ => e,
    };
    // CTOR-10: a converting constructor makes the object.
    if let Some(c) = to.class().filter(|&c| crate::classes::constructors::check::has_ctors(cx, c)) {
        if e.ty.nonref().unqual().t != Type::Class(c) {
            let span = e.span;
            if e.ty.nonref().class().is_some() {
                cx.unsupported(span, "a conversion between classes by a constructor (build step 4)");
                return None;
            }
            let i = crate::classes::constructors::check::construct_init(cx, to, vec![e], false, true, span)?;
            return Some(Expr::new(EK::List(Box::new(i)), to.unqual(), crate::sem::tree::Cat::PR, span));
        }
    }
    if to.is_array() {
        if let (EK::Str(_), Type::Array(..)) = (&e.k, &to.t) {
            return Some(e);
        }
        cx.error(e.span, "INIT-8", "an array initialized from another array");
        return None;
    }
    let span = e.span;
    match conv::implicit(cx, e, to, false) {
        Ok(x) => Some(x),
        Err(m) => {
            cx.error_cause(span, conv::rule_of(&m).unwrap_or(rule), conv::cause_of(&m), m);
            None
        }
    }
}

/// An overload set converted to a pointer to function selects the
/// function of the target's type (OVL-18, ICS-47); None after an error.
pub fn select_overload(cx: &mut Check, e: Expr, to: &QType) -> Option<Expr> {
    if let EK::AddrOf(inner) = &e.k {
        if crate::expressions::primary::check::unresolved_set(&e) {
            let f = select_overload(cx, (**inner).clone(), to)?;
            let t = QType::ptr_to(f.ty.clone());
            return Some(Expr::new(EK::AddrOf(Box::new(f)), t, Cat::PR, e.span));
        }
    }
    let EK::Func(_) = e.k else { return Some(e) };
    if !e.ty.is_error() {
        return Some(e);
    }
    let set = crate::expressions::primary::check::overload_set(cx, &e);
    let want = match &to.t {
        Type::Ptr(p) => p.func().cloned(),
        Type::Ref(p, _) => p.func().cloned(),
        _ => None,
    };
    let Some(want) = want else {
        crate::expressions::primary::check::no_target(cx, &e, "OVL-20");
        return None;
    };
    let pick: Vec<EntityId> = set.into_iter().filter(|&f| cx.u.func(f).is_some_and(|x| x.ty.params == want.params && x.ty.ret == want.ret)).collect();
    if pick.len() != 1 {
        cx.error(e.span, "OVL-20", "no single function of the overload set matches the target type");
        return None;
    }
    let f = pick[0];
    let ty = QType::new(Type::Func(cx.u.func(f).unwrap().ty.clone()));
    Some(Expr::new(EK::Func(f), ty, Cat::L, e.span))
}

/// The argument for a parameter of type `to` (PFX-6: as in a
/// copy-initialization).
pub fn init_parameter(cx: &mut Check, a: Expr, to: &QType) -> Option<Arg> {
    if to.is_ref() {
        let e = references::check::bind(cx, a, to, "INIT-7")?;
        return Some(Arg { e, by_ref: true });
    }
    let e = copy_init(cx, a, to, "INIT-7")?;
    Some(Arg { e, by_ref: false })
}

/// Record a use of a variable (ATTR-6 counts reads).
pub fn note_use(cx: &mut Check, id: EntityId, _span: Span) {
    if cx.unevaluated == 0 {
        if let Some(v) = cx.u.var_mut(id) {
            v.used = true;
        }
        *cx.uses.entry(id).or_insert(0) += 1;
    }
}

/// ATTR-6: the program reads the variable. A use as the left operand of
/// `=` is a write, so a variable whose every use is such a write is not
/// read.
pub fn is_read(cx: &Check, id: EntityId) -> bool {
    let used = cx.u.var(id).is_some_and(|v| v.used);
    let assigned = cx.assigned.get(&id).copied().unwrap_or(0);
    used && (assigned == 0 || cx.uses.get(&id).copied().unwrap_or(0) > assigned)
}

/// The initializer of a variable from its `init` node (INIT-2 to INIT-8;
/// lists in `list`, references in `references`).
fn initializer(cx: &mut Check, ty: &QType, init: &Node, name: &str) -> Option<Init> {
    let words = cx.words(init);
    let ns = nodes(init);
    if ty.is_ref() {
        return references::check::initializer(cx, ty, init);
    }
    // A class with constructors (CTOR-25): a parenthesized or braced list
    // chooses a constructor; `= e` copies a value of the class or
    // converts by a converting constructor.
    if ty.class().is_some_and(|c| crate::classes::constructors::check::has_ctors(cx, c)) {
        if let Some(b) = ns.first().filter(|c| c.prod == "braced_init") {
            let items: Vec<&Node> = b.children("init_clause").collect();
            let mut args = Vec::new();
            for it in items {
                let c = first_node(it)?;
                if c.prod == "braced_init" || c.prod == "designator" {
                    cx.unsupported(c.span, "a braced list as a constructor argument (build step 4)");
                    return None;
                }
                let e = expr(cx, c);
                if e.ty.is_error() {
                    return None;
                }
                args.push(e);
            }
            return crate::classes::constructors::check::construct_init(cx, ty, args, true, words.first() == Some(&"="), init.span);
        }
        if words.first() == Some(&"(") {
            let list: Vec<&Node> = ns.first().map(|l| l.children("argument").collect()).unwrap_or_default();
            let mut args = Vec::new();
            for a in list {
                let c = first_node(a)?;
                if c.prod == "braced_init" {
                    cx.unsupported(c.span, "a braced list as a constructor argument (build step 4)");
                    return None;
                }
                let e = expr(cx, c);
                if e.ty.is_error() {
                    return None;
                }
                args.push(e);
            }
            if args.len() == 1 && args[0].ty.nonref().unqual().t == ty.unqual().t {
                return copy_init(cx, args.remove(0), ty, "INIT-7").map(Init::Expr);
            }
            return crate::classes::constructors::check::construct_init(cx, ty, args, false, false, init.span);
        }
        let e = expr(cx, ns.first()?);
        return copy_init(cx, e, ty, "INIT-7").map(Init::Expr);
    }
    if let Some(b) = ns.first().filter(|c| c.prod == "braced_init") {
        let r = list::check::list_init(cx, b, ty, name);
        // ENUM-19: an integer in braces initializes an enumeration only by
        // direct-list-initialization, not after `=`.
        if words.first() == Some(&"=") && ty.is_enum() {
            if let Some(Init::Expr(e)) = &r {
                if matches!(&e.k, crate::sem::tree::EK::Conv(crate::sem::tree::Conv::Arith, x) if x.ty.t != ty.t) {
                    cx.error(init.span, "ENUM-19", "copy-list-initialization of an enumeration from an integer");
                    return None;
                }
            }
        }
        return r;
    }
    if words.first() == Some(&"(") {
        let args: Vec<&Node> = ns.first().map(|l| l.children("argument").collect()).unwrap_or_default();
        let other = ty.is_class() && args.len() == 1 && first_node(args[0]).is_some_and(|c| c.prod == "braced_init");
        if ty.base_elem().is_class() && (args.len() != 1 || ty.is_array() || other) {
            cx.unsupported(init.span, "a parenthesized initializer for a class (build step 4)");
            return None;
        }
        if let Type::Array(el, n) = &ty.t {
            // INIT-5: an array from a parenthesized list of values, with
            // no check for narrowing.
            let mut items = Vec::new();
            for a in &args {
                let Some(c) = first_node(a) else { return None };
                if c.prod == "braced_init" {
                    cx.unsupported(c.span, "a braced list in a parenthesized array initializer");
                    return None;
                }
                let e = expr(cx, c);
                items.push(Init::Expr(copy_init(cx, e, el, "INIT-7")?));
            }
            if n.is_some_and(|n| items.len() as u64 > n) {
                cx.error(init.span, "DCL-19", "the initializer has more elements than the array");
                return None;
            }
            return Some(Init::List(items));
        }
        if args.len() != 1 {
            cx.error(init.span, "INIT-7", "a parenthesized initializer for a scalar must hold exactly one expression");
            return None;
        }
        if first_node(args[0]).is_some_and(|c| c.prod == "braced_init") {
            cx.error(init.span, "INIT-7", "a braced list in parentheses for a scalar");
            return None;
        }
        let e = expr(cx, args[0]);
        if ty.is_array() {
            cx.error(init.span, "INIT-7", "an array initialized from a parenthesized expression");
            return None;
        }
        // A class from a value of another type needs a constructor, or
        // parenthesized aggregate initialization.
        if ty.is_class() && !e.ty.is_error() && e.ty.nonref().unqual().t != ty.unqual().t {
            cx.unsupported(init.span, "a class from a parenthesized value of another type (build step 4)");
            return None;
        }
        let span = e.span;
        return match conv::implicit(cx, e, ty, true) {
            Ok(x) => Some(Init::Expr(x)),
            Err(m) => {
                if !m.is_empty() {
                    cx.error_cause(span, conv::rule_of(&m).unwrap_or("INIT-7"), conv::cause_of(&m), m);
                }
                None
            }
        };
    }
    let e = expr(cx, ns.first()?);
    if ty.is_array() {
        if let EK::Str(_) = e.k {
            return list::check::string_init(cx, e, ty);
        }
        cx.error(init.span, "INIT-8", "an array initialized from another array or a value");
        return None;
    }
    copy_init(cx, e, ty, "INIT-7").map(Init::Expr)
}

/// The value of an initializer that is a single expression, for a
/// placeholder to deduce from.
fn init_expr(cx: &mut Check, init: &Node, p: &auto::check::Placeholder) -> Option<Expr> {
    let ns = nodes(init);
    let words = cx.words(init);
    let one = if let Some(b) = ns.first().filter(|c| c.prod == "braced_init") {
        let items: Vec<&Node> = b.children("init_clause").collect();
        if words.first() == Some(&"=") {
            if !names::std_declares(cx, "initializer_list") {
                cx.error_cause(init.span, "AUTO-21", Some("LSUP-1"), "'auto' with '= { ... }' needs a declaration of std::initializer_list");
                return None;
            }
            cx.unsupported(init.span, "auto with '= { ... }' (std::initializer_list, build step 4)");
            return None;
        }
        if items.len() != 1 {
            one_expression(cx, init.span, p);
            return None;
        }
        first_node(items[0])?
    } else if words.first() == Some(&"(") {
        let args: Vec<&Node> = ns.first().map(|l| l.children("argument").collect()).unwrap_or_default();
        if args.len() != 1 || first_node(args[0]).is_some_and(|c| c.prod == "braced_init") {
            one_expression(cx, init.span, p);
            return None;
        }
        args[0]
    } else {
        ns.first()?
    };
    Some(expr(cx, one))
}

/// AUTO-20, with AUTO-11 for `decltype(auto)`.
fn one_expression(cx: &mut Check, span: Span, p: &auto::check::Placeholder) {
    if p.decltype {
        cx.error_cause(span, "AUTO-20", Some("AUTO-11"), "a placeholder needs exactly one expression in its initializer");
    } else {
        auto::check::one_expression(cx, span);
    }
}

/// Declare a variable of a simple declaration, with its initializer.
/// After a qualified declarator-id, the initializer sees the members of
/// the named scope (NAM-21).
pub fn declare_variable(cx: &mut Check, s: &Specs, d: &Declared, init: Option<&Node>, aligns: &[Node], first_auto: &mut Option<QType>) -> Option<(EntityId, Init)> {
    let names::Qual::Scope(q) = d.qual else { return declare_variable_in(cx, s, d, init, aligns, first_auto) };
    let saved = cx.scope;
    // A member of an inline namespace of the named one (NS-15).
    let mut d2 = d.clone();
    if let Some((name, _)) = &d.name {
        if let Some(&e) = names::lookup_in(cx, q, name).iter().find(|&&e| matches!(cx.u.entities[e].kind, EntKind::Var(_))) {
            d2.qual = names::Qual::Scope(cx.u.entities[e].scope);
        }
    }
    if let (names::Qual::Scope(sc), Some((name, span))) = (d2.qual, &d.name) {
        // ODR-15: a redeclaration outside its class of a constexpr static
        // data member, without an initializer, declares it again.
        if s.constexpr && init.is_none() && cx.u.scopes[sc].kind == crate::sem::ScopeKind::Class {
            if let Some(&e) = names::declared_here(cx, sc, name).iter().find(|&&e| cx.u.var(e).is_some_and(|v| v.constexpr && v.defined)) {
                let i = cx.u.var(e).and_then(|v| v.init.clone()).unwrap_or(Init::Default);
                return Some((e, i));
            }
        }
        crate::declarations::namespaces::check::qualified_place(cx, sc, *span, name);
        cx.scope = sc;
    }
    let r = declare_variable_in(cx, s, &d2, init, aligns, first_auto);
    cx.scope = saved;
    r
}

fn declare_variable_in(cx: &mut Check, s: &Specs, d: &Declared, init: Option<&Node>, aligns: &[Node], first_auto: &mut Option<QType>) -> Option<(EntityId, Init)> {
    let Some((name, span)) = d.name.clone() else {
        cx.unsupported(d.span, "a declaration with no declarator-id");
        return None;
    };
    let local = cx.in_function();
    attributes::check::on_variable(cx, &s.attrs);
    if s.inline && local {
        crate::declarations::inline::check::misplaced(cx, d.span, "on a variable in a block");
    }
    if name.as_ref() == "main" && (cx.lang_c || (cx.scope == cx.u.root && d.qual == names::Qual::None)) {
        cx.error_cause(span, "TR-3", Some("FN-4"), "a variable named main in the global namespace or with C language linkage");
    }
    storage_linkage::check::variable_specifiers(cx, s);
    if s.consteval {
        constexpr::check::consteval_variable(cx, d.span);
    }
    let storage = storage_linkage::check::storage(cx, s);
    if s.constinit && storage == Storage::Automatic {
        constexpr::check::constinit_place(cx, d.span, "on a variable with automatic storage");
    }
    if s.is_extern && local && init.is_some() {
        storage_linkage::check::block_extern_init(cx, d.span);
    }
    // The declared type, after a placeholder is deduced.
    let mut ty = d.ty.clone();
    let mut init_e: Option<Expr> = None;
    if let Some(p) = &d.placeholder {
        let Some(i) = init else {
            auto::check::needs_initializer(cx, span, &name);
            return None;
        };
        // AUTO-8: the variable is not visible in its own initializer.
        let mut toks = Vec::new();
        i.tokens(&mut toks);
        let own = toks.iter().enumerate().any(|(k, &t)| cx.toks[t].text == *name && (k == 0 || !matches!(cx.toks[toks[k - 1]].text.as_str(), "." | "->" | "::")));
        if own {
            auto::check::used_in_own_initializer(cx, span, &name);
            return None;
        }
        let e = init_expr(cx, i, p)?;
        let dn = d_node_hint(cx, d);
        let base = auto::check::deduce(cx, p, &dn, &e)?;
        // A forwarding reference already deduced the lvalue reference
        // that collapsing gives (AUTO-4).
        ty = if base.is_ref() { base.clone() } else { crate::declarations::declarators::check::with_base(cx, base.clone(), &dn) };
        if let Some(prev) = first_auto {
            if *prev != base {
                auto::check::differs(cx, d.span);
            }
        } else {
            *first_auto = Some(base);
        }
        init_e = Some(e);
    }
    if ty.is_error() {
        names::declare_unsupported(cx, &name, span);
        return None;
    }
    if ty.is_void() {
        cx.error(span, "DCL-20", format!("the variable '{}' has type void", name));
        return None;
    }
    if crate::declarations::declarators::check::is_qualified_function_type(&ty) {
        return None;
    }
    let mut constexpr = s.constexpr;
    if constexpr && !ty.is_ref() {
        ty = ty.with_cv(true, false);
    }
    // A qualified name defines a variable that its scope declared.
    let scope = match d.qual {
        names::Qual::Scope(sc) => match crate::declarations::namespaces::check::member_home(cx, sc, &name, |k| matches!(k, EntKind::Var(_))) {
            Some(home) => home,
            None => {
                crate::declarations::namespaces::check::member_not_declared(cx, span, &name);
                return None;
            }
        },
        names::Qual::None => cx.scope,
        _ => return None,
    };
    let linkage = storage_linkage::check::linkage(cx, s, true, ty.is_const(), ty.v);
    // LNK-9: a block-scope extern declaration names the variable of the
    // enclosing namespace, declared there if not yet.
    let scope = if local && s.is_extern && d.qual == names::Qual::None {
        let ns = cx.u.enclosing_namespace(cx.scope);
        if let Some(&e) = names::declared_here(cx, ns, &name).iter().find(|&&e| matches!(cx.u.entities[e].kind, EntKind::Var(_))) {
            if !names::declared_here(cx, cx.scope, &name).contains(&e) {
                let here = cx.scope;
                cx.u.scopes[here].names.entry(name.clone()).or_default().push(e);
            }
            return Some((e, Init::Default));
        }
        scope
    } else {
        scope
    };
    let earlier = redeclaration(cx, scope, &name, &ty, s, storage, init.is_some(), span, d.qual != names::Qual::None);
    let align = if aligns.is_empty() { None } else { alignment::check::alignas(cx, aligns, &ty) };
    let id = match earlier {
        Some(e) => {
            if let Some(v) = cx.u.var(e).cloned() {
                if v.align != align && (align.is_some() || !s.is_extern) && (v.align.is_some()) {
                    crate::declarations::alignment::check::redeclared(cx, span);
                }
                constexpr |= v.constexpr;
            }
            e
        }
        None => {
            if d.qual != names::Qual::None && d.qual != names::Qual::Scope(scope) {
                crate::declarations::namespaces::check::member_not_declared(cx, span, &name);
                return None;
            }
            let v = Var {
                ty: ty.clone(),
                storage,
                constexpr,
                constinit: s.constinit,
                inline: s.inline,
                is_extern: s.is_extern && init.is_none(),
                is_static: s.is_static,
                param: false,
                defined: !(s.is_extern && init.is_none()),
                init: None,
                const_init: false,
                used: false,
                maybe_unused: s.attrs.maybe_unused,
                align,
            };
            let id = names::declare_in(cx, scope, &name, EntKind::Var(v), span, linkage);
            cx.u.entities[id].deprecated = s.attrs.deprecated.clone();
            id
        }
    };
    // The point of declaration is before the initializer (NAM-1), so the
    // initializer can name the variable.
    let init_value: Init = match (init, init_e) {
        (Some(_), Some(e)) => {
            if e.ty.is_error() {
                return None;
            }
            if ty.is_ref() {
                Init::Ref(references::check::bind(cx, e, &ty, "INIT-7")?)
            } else if ty.is_array() {
                cx.error(span, "AUTO-15", "an array type deduced from an array");
                return None;
            } else {
                Init::Expr(copy_init(cx, e, &ty, "INIT-7")?)
            }
        }
        (Some(i), None) => {
            if cx.u.is_incomplete(&ty) && !ty.is_array() {
                cx.error(span, "INIT-7", format!("the variable '{}' has an incomplete type", name));
                return None;
            }
            if !crate::classes::r#virtual::check::not_abstract(cx, &ty, span) {
                return None;
            }
            let r = initializer(cx, &ty, i, &name);
            match r {
                Some(x) => {
                    // An array of unknown bound takes its bound from the
                    // initializer (DCL-5).
                    if let (Type::Array(el, None), Init::List(items)) = (&ty.t, &x) {
                        ty = QType::new(Type::Array(el.clone(), Some(items.len() as u64))).with_cv(ty.c, ty.v);
                    }
                    if let (Type::Array(el, None), Init::Expr(Expr { k: EK::Str(u), .. })) = (&ty.t, &x) {
                        ty = QType::new(Type::Array(el.clone(), Some(u.len() as u64)));
                    }
                    x
                }
                None => {
                    if let Some(v) = cx.u.var_mut(id) {
                        v.ty = QType::error();
                    }
                    return None;
                }
            }
        }
        (None, _) => {
            let defined = !(s.is_extern) || local && !s.is_extern;
            if ty.is_ref() && !s.is_extern {
                crate::declarations::declarators::check::reference_needs_init(cx, span, &name);
                return None;
            }
            if constexpr && ty.base_elem().is_class() {
                cx.unsupported(span, "a constexpr object of class type with no initializer (build step 4)");
                return None;
            }
            if constexpr {
                constexpr::check::needs_initializer(cx, span, &name);
                return None;
            }
            if ty.base_elem().is_class() && ty.is_const() && !s.is_extern {
                cx.unsupported(span, "a const object of class type with no initializer (build step 4)");
                return None;
            }
            if ty.is_const() && !s.is_extern && ty.base_elem().is_scalar() {
                crate::declarations::declarators::check::const_needs_init(cx, span, &name);
                return None;
            }
            if defined && !crate::classes::r#virtual::check::not_abstract(cx, &ty, span) {
                return None;
            }
            if defined && cx.u.is_incomplete(&ty) {
                if ty.is_array() {
                    cx.error(span, "DCL-19", format!("the array '{}' has no bound and no initializer", name));
                } else {
                    cx.error(span, "INIT-7", format!("the variable '{}' has an incomplete type", name));
                }
                return None;
            }
            // A class gets the default member initializers of its members.
            // A declaration that is not a definition initializes nothing.
            if !defined {
                Init::Default
            } else if ty.base_elem().class().is_some_and(|c| crate::classes::constructors::check::has_ctors(cx, c)) {
                if s.thread_local {
                    cx.unsupported(span, "a thread_local object of a class with constructors (build step 4)");
                    return None;
                }
                crate::classes::constructors::check::default_init(cx, &ty, span)?
            } else if ty.base_elem().is_class() {
                if !crate::classes::members::check::check_default_init(cx, &ty, span) {
                    return None;
                }
                crate::classes::members::check::default_init(cx, &ty)
            } else {
                Init::Default
            }
        }
    };
    // CE-24: a constant initializer makes the variable usable in
    // constant expressions when it is const (CE-3).
    let const_init = match &init_value {
        Init::Expr(e) => ce::is_constant(cx, e),
        Init::Ref(e) => ce::evaluate_ref(cx, e, storage != Storage::Automatic).is_ok(),
        Init::Zero => true,
        Init::List(items) => items.iter().all(|i| constant_part(cx, i, ty.base_elem().is_class())),
        Init::Default => storage != Storage::Automatic,
        Init::Ctor(..) => ce::is_constant(cx, &Expr::new(EK::List(Box::new(init_value.clone())), ty.unqual(), crate::sem::tree::Cat::PR, span)),
    };
    if storage != Storage::Static || !s.is_extern {
        if !crate::classes::constructors::check::dtor_usable(cx, &ty, span) {
            return None;
        }
    }
    // CE-23: a constexpr object has a literal type, whose destructor is
    // constexpr.
    if constexpr {
        if let Some(c) = ty.base_elem().class() {
            let user = names::declared_here(cx, cx.u.classes[c].scope, crate::classes::constructors::check::DTOR).first().copied();
            if crate::classes::constructors::run::needs_in(&cx.u, &ty) && !user.is_some_and(|d| cx.u.func(d).is_some_and(|f| f.constexpr)) {
                cx.error_cause(span, "CX-1", Some("CE-23"), format!("the constexpr variable '{}' has a type whose destructor is not constexpr", name));
                return None;
            }
        }
    }
    if let Some(v) = cx.u.var_mut(id) {
        // A later declaration without an initializer, such as `extern`
        // after a definition, keeps the definition's initializer.
        let keep = init.is_none() && v.defined && !matches!(v.init, None | Some(Init::Default));
        v.ty = ty.clone();
        if !keep {
            v.init = Some(init_value.clone());
            v.const_init = const_init;
            v.constexpr = constexpr;
        }
        v.defined = v.defined || init.is_some() || !s.is_extern;
        if init.is_some() || !s.is_extern {
            v.is_extern = false;
        }
    }
    if !local && ((cx.lang_c && !s.is_static) || (scope == cx.u.root && !cx.lang_c)) {
        crate::declarations::linkage_specifications::check::c_name(cx, id, cx.lang_c, span);
    }
    if constexpr || s.constinit {
        let rule = if constexpr { constexpr::check::VARIABLE } else { constexpr::check::CONSTINIT };
        check_constant_initializer(cx, &init_value, span, rule, storage != Storage::Automatic);
    }
    if storage != Storage::Automatic && !local || (storage != Storage::Automatic && local && false) {
        if !cx.u.statics.contains(&id) && init.is_some() || (!cx.u.statics.contains(&id) && !s.is_extern) {
            cx.u.statics.push(id);
        }
    }
    let _ = Linkage::None;
    Some((id, init_value))
}

/// CX-1, CINIT-1: the initializer must be a constant expression.
/// A part of a list initializer that is a constant: a constant
/// expression, zero, a list of such parts, or, in an array of scalars,
/// a default-initialized element. A default-initialized member of a class
/// gets no value (CE-21).
fn constant_part(cx: &Check, i: &Init, class: bool) -> bool {
    match i {
        Init::Expr(e) => ce::is_constant(cx, e),
        Init::Zero => true,
        Init::Default => !class,
        Init::List(xs) => xs.iter().all(|x| constant_part(cx, x, class)),
        Init::Ref(_) => false,
        Init::Ctor(..) => false,
    }
}

/// CE-21: a part of a class's value that default-initialization leaves
/// without a value.
fn has_default_part(i: &Init) -> bool {
    match i {
        Init::Default => true,
        Init::List(xs) => xs.iter().any(has_default_part),
        _ => false,
    }
}

fn check_constant_initializer(cx: &mut Check, init: &Init, span: Span, rule: &'static str, static_storage: bool) {
    // A constructor call: the value it gives has a value in each part
    // (CE-21).
    if let Init::Ctor(f, _) = init {
        let Some(c) = cx.u.func(*f).and_then(|x| x.class) else { return };
        let e = Expr::new(EK::List(Box::new(init.clone())), QType::new(Type::Class(c)), crate::sem::tree::Cat::PR, span);
        match ce::evaluate(cx, &e) {
            Ok(v) if v.has_empty_part() => ce::report(cx, span, rule, ("CE-21", "a part of the value has no value".into())),
            Ok(_) => {}
            Err(why) => ce::report(cx, span, rule, why),
        }
        return;
    }
    if let Init::List(_) = init {
        if has_default_part(init) {
            ce::report(cx, span, rule, ("CE-21", "a part of the value has no value".into()));
            return;
        }
    }
    // CX-2: a constexpr reference or pointer refers to an object with
    // static storage duration or to a function.
    let held = |rule: &'static str, why: &(&'static str, String)| if rule == constexpr::check::VARIABLE && matches!(why.0, "CE-3" | "CE-14") { "CX-2" } else { rule };
    if let Init::Ref(e) = init {
        if let Err(why) = ce::evaluate_ref(cx, e, static_storage) {
            ce::report(cx, span, held(rule, &why), why);
        }
        return;
    }
    fn exprs_of<'i>(i: &'i Init, out: &mut Vec<&'i Expr>) {
        match i {
            Init::Expr(e) => out.push(e),
            Init::List(xs) => xs.iter().for_each(|x| exprs_of(x, out)),
            _ => {}
        }
    }
    let mut exprs = Vec::new();
    exprs_of(init, &mut exprs);
    for e in exprs {
        if let Err(why) = ce::evaluate(cx, e) {
            let rule = if e.ty.is_ptr() { held(rule, &why) } else { rule };
            ce::report(cx, span, rule, why);
            return;
        }
    }
}

/// The declarator node, whose shape a placeholder's deduction reads.
fn d_node_hint(_cx: &mut Check, d: &Declared) -> Node {
    d.node.clone()
}

/// An earlier declaration of the same variable in the scope, after the
/// checks of NAM-3, ODR-1, LNK-10, and STO-12.
#[allow(clippy::too_many_arguments)]
fn redeclaration(cx: &mut Check, scope: usize, name: &str, ty: &QType, s: &Specs, storage: Storage, has_init: bool, span: Span, qualified: bool) -> Option<EntityId> {
    let local = cx.u.is_local(scope);
    for e in names::declared_here(cx, scope, name) {
        match cx.u.entities[e].kind.clone() {
            EntKind::Var(v) => {
                let same_type = v.ty.t == ty.t || v.ty.is_error() || matches!((&v.ty.t, &ty.t), (Type::Array(a, _), Type::Array(b, _)) if a == b);
                if cx.bindings.contains(&e) {
                    cx.error(span, "SB-8", format!("'{}' is already declared in this scope by a structured binding", name));
                    return None;
                }
                if v.param && local {
                    cx.error_cause(span, "NAM-3", Some("FN-13"), format!("'{}' redeclares a parameter", name));
                    return None;
                }
                if local && !(v.is_extern && s.is_extern) {
                    // STO-13: a block's extern declaration after a local
                    // of the name.
                    cx.error_cause(span, "NAM-3", s.is_extern.then_some("STO-13"), format!("'{}' is declared twice in one scope", name));
                    return None;
                }
                if !same_type {
                    cx.error(span, "NAM-3", format!("'{}' is redeclared with a different type", name));
                    return None;
                }
                let old_def = v.defined && (!v.is_extern);
                let new_def = has_init || !s.is_extern;
                if old_def && new_def && (!qualified || has_init) {
                    if v.inline && s.inline {
                        cx.error(span, "ODR-1", format!("'{}' is defined twice", name));
                    } else {
                        cx.error(span, "ODR-1", format!("'{}' is defined twice", name));
                    }
                    return None;
                }
                // CINIT-2: after a constinit declaration, the initializing
                // declaration has constinit too.
                if v.constinit && has_init && !s.constinit {
                    cx.error(span, "CINIT-2", format!("the initializing declaration of '{}' lacks constinit", name));
                    return None;
                }
                let old_link = cx.u.entities[e].linkage;
                storage_linkage::check::redeclared_linkage(cx, span, old_link, s, name);
                storage_linkage::check::thread_local_agrees(cx, span, v.storage, storage, name);
                return Some(e);
            }
            EntKind::Enum(_) | EntKind::Class(_) => continue,
            EntKind::Unsupported(_) => continue,
            k => {
                // TYN-2: a typedef name is not declared again as another
                // kind of entity.
                let cause = matches!(k, EntKind::Typedef(_)).then_some("TYN-2");
                cx.error_cause(span, "NAM-3", cause, format!("'{}' is already declared as a different kind of entity", name));
                return None;
            }
        }
    }
    None
}

/// The variable's name for a node, used by the parse of a list's
/// elements.
pub fn kid_text<'a>(cx: &Check<'a>, k: &Kid) -> &'a str {
    cx.text(k)
}

/// The category of an initializer expression after reference collapse.
pub fn cat_of(e: &Expr) -> Cat {
    e.cat
}
