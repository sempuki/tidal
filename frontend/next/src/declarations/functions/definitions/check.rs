//! The check part of `declarations/functions/definitions`: function
//! declarations and definitions, `main`, and the body of a function.

use crate::declarations::check::Specs;
use crate::declarations::declarators::check::{Declared, Param};
use crate::declarations::initialization::forms::check as forms;
use crate::declarations::storage_linkage::check as stl;
use crate::foundations::names::check as names;
use crate::sem::check::{Check, FnCtx};
use crate::sem::tree::{Expr, Stmt, SK};
use crate::sem::types::{FnType, QType, Type};
use crate::sem::{EntKind, EntityId, Func, Linkage, ScopeKind, Storage, Var};
use crate::source::Span;
use crate::syntax::{Kid, Node};
use std::rc::Rc;

/// Set whether declarations get C language linkage (LSP-1); gives the
/// earlier setting.
pub fn set_c_linkage(cx: &mut Check, c: bool) -> bool {
    std::mem::replace(&mut cx.lang_c, c)
}

/// Whether a function named `main` is the program's `main`.
fn is_main(cx: &Check, name: &str, scope: usize) -> bool {
    name == "main" && scope == cx.u.root
}

/// FN-4, TR-3: what `main` must be.
fn check_main(cx: &mut Check, s: &Specs, d: &Declared, span: Span) {
    let f = d.ty.func();
    if d.placeholder.is_some() || d.trailing && f.is_some_and(|f| f.ret.t != Type::int()) {
        cx.error_cause(span, "TR-3", Some("FN-4"), "the return type of main must be int, written out");
        return;
    }
    if f.is_some_and(|f| f.ret.unqual().t != Type::int()) {
        cx.error_cause(span, "TR-3", Some("FN-4"), "the return type of main must be int");
    }
    if s.inline || s.constexpr || s.consteval {
        cx.error_cause(span, "TR-3", Some("FN-4"), "main must not be inline, constexpr, or consteval");
    }
}

/// Declare a function, or find its earlier declaration (FN-14, DCL-14,
/// DEFARG-9). `body` is the function body of a definition.
pub fn declare_function(cx: &mut Check, s: &Specs, d: &Declared, body: Option<&Node>) -> Option<EntityId> {
    let (name, span) = d.name.clone()?;
    let scope = match d.qual {
        // A member of an inline namespace of the named one (NS-15).
        names::Qual::Scope(sc) => crate::declarations::namespaces::check::member_home(cx, sc, &name, |k| matches!(k, EntKind::Func(_))).unwrap_or(sc),
        names::Qual::None => cx.scope,
        _ => return None,
    };
    let local = cx.in_function();
    let main = is_main(cx, &name, scope);
    stl::function_specifiers(cx, s, local, main);
    if s.constinit {
        crate::declarations::constexpr::check::constinit_place(cx, d.span, "on a function");
    }
    if main {
        check_main(cx, s, d, span);
    }
    if name.as_ref() == "main" && cx.lang_c {
        cx.error_cause(span, "TR-3", Some("FN-4"), "main has C language linkage");
    }
    if s.inline && local {
        crate::declarations::inline::check::misplaced(cx, d.span, "on a function declaration in a block");
    }
    if let names::Qual::Scope(_) = d.qual {
        crate::declarations::namespaces::check::qualified_place(cx, scope, span, &name);
    }
    let Some(fty) = d.ty.func().cloned() else { return None };
    let deduce = d.placeholder.is_some();
    // A deduced return type is the error type until a return gives it.
    let known = if deduce { fty.params.iter().all(|p| !p.is_error()) } else { fty.is_error_free() };
    if !known {
        names::declare_unsupported(cx, &name, span);
        return None;
    }
    // Default arguments (DEFARG-8, DEFARG-10): checked at the declaration.
    let defaults = default_arguments(cx, &d.params);
    let linkage = if s.is_static && cx.member_of.is_none() { Linkage::Internal } else { stl::linkage(cx, s, false, false, false) };
    let deleted = body.is_some_and(|b| cx.words(b).contains(&"delete"));
    let defaulted = body.is_some_and(|b| cx.words(b).contains(&"default"));
    // OPR-2: the limits on an operator function.
    if crate::classes::operators::check::is_operator(&name) {
        let class = cx.member_of.or(match d.qual {
            names::Qual::Scope(sc) if cx.u.scopes[sc].kind == ScopeKind::Class => cx.u.classes.iter().position(|c| c.scope == sc),
            _ => None,
        });
        let defaults = d.params.iter().any(|p| p.default.is_some());
        if !crate::classes::operators::check::declaration(cx, &name, &d.ty, defaults, class, s.is_static, span) {
            return None;
        }
        if defaulted && matches!(&*name, "operator==" | "operator<=>" | "operator!=" | "operator<" | "operator>" | "operator<=" | "operator>=") {
            cx.unsupported(d.span, "a defaulted comparison operator (classes/comparisons)");
            return None;
        }
    }
    let assign_default = defaulted && &*name == "operator=";
    // CPY-17: a defaulted assignment operator takes a reference and
    // returns a reference to its class.
    if assign_default {
        let class = cx.member_of;
        let ret_ok = matches!(&fty.ret.t, crate::sem::types::Type::Ref(t, false) if t.class() == class && !t.c && !t.v);
        let param_ok = fty.params.len() == 1 && fty.params[0].is_ref();
        if !ret_ok || !param_ok {
            cx.error(d.span, "CPY-17", "a defaulted assignment operator must take a reference and return a reference to its class");
            return None;
        }
    }
    if defaulted && !assign_default {
        cx.error(d.span, "DEL-4", "'= default' on a function that is not a special member function");
        return None;
    }
    if deleted && main {
        cx.error(d.span, "DEL-2", "main must not be deleted");
    }
    let mut earlier = None;
    for e in names::declared_here(cx, scope, &name) {
        match cx.u.entities[e].kind.clone() {
            EntKind::Func(f) => {
                // Member functions that differ in the qualifiers of their
                // object are overloads.
                if f.ty.params != fty.params || f.ty.variadic != fty.variadic || (f.class.is_some() && f.obj_cv != d.obj_cv) {
                    if f.c_linkage && cx.lang_c {
                        cx.error(span, "LSP-7", format!("two C functions named '{}'", name));
                        return None;
                    }
                    continue;
                }
                if f.placeholder != deduce {
                    cx.error(span, "RET-4", format!("the declarations of '{}' differ in using a placeholder for the return type", name));
                    return None;
                }
                if !deduce && f.ty.ret != fty.ret {
                    cx.error(span, "FN-14", format!("the declarations of '{}' have different return types", name));
                    return None;
                }
                if f.ty.noexcept != fty.noexcept {
                    cx.error_cause(span, "DCL-14", Some("EXS-8"), format!("the declarations of '{}' disagree on noexcept", name));
                    return None;
                }
                if body.is_some() && !deleted && f.defined {
                    cx.error(span, "FN-14", format!("the function '{}' is defined twice", name));
                    return None;
                }
                if deleted && !cx.deferred.contains(&e) {
                    cx.error(span, "DEL-2", format!("'= delete' must be on the first declaration of '{}'", name));
                    return None;
                }
                if s.inline && !f.inline && f.defined {
                    crate::declarations::inline::check::misplaced(cx, d.span, "on a function after its definition");
                }
                let old_link = cx.u.entities[e].linkage;
                if cx.member_of.is_none() && cx.u.scopes[scope].kind != crate::sem::ScopeKind::Class {
                    stl::redeclared_linkage(cx, span, old_link, s, &name);
                }
                // ATTR-2: `noreturn` and `carries_dependency` belong on the
                // first declaration. ATTR-4: a later declaration can add
                // `deprecated`.
                // CX-5: every declaration agrees on constexpr and consteval.
                if s.constexpr != f.constexpr || s.consteval != f.consteval {
                    cx.error(span, "CX-5", format!("the declarations of '{}' differ in constexpr or consteval", name));
                    return None;
                }
                // LSP-7: a function first declared with C++ language
                // linkage cannot be given C language linkage later.
                if cx.lang_c && !f.c_linkage && f.class.is_none() {
                    cx.error(span, "LSP-7", format!("'{}' was first declared with C++ language linkage", name));
                    return None;
                }
                if s.attrs.noreturn && !f.noreturn {
                    cx.error(span, "ATTR-2", format!("'noreturn' on '{}', whose first declaration does not have it", name));
                }
                if s.attrs.carries_dependency {
                    cx.error(span, "ATTR-2", format!("'carries_dependency' on a declaration of '{}' after the first", name));
                }
                if s.attrs.deprecated.is_some() {
                    cx.u.entities[e].deprecated = s.attrs.deprecated.clone();
                }
                earlier = Some(e);
                break;
            }
            EntKind::Unsupported(_) | EntKind::Enum(_) | EntKind::Class(_) => {}
            _ => {
                if !local {
                    cx.error(span, "NAM-3", format!("'{}' is already declared as a different kind of entity", name));
                    return None;
                }
            }
        }
    }
    let id = match earlier {
        // The body of a member function that its class declared.
        Some(e) if cx.deferred.remove(&e) => {
            if deleted {
                if let Some(f) = cx.u.func_mut(e) {
                    f.deleted = true;
                }
            }
            e
        }
        Some(e) => {
            let old_defaults = cx.u.func(e).unwrap().defaults.clone();
            for (k, dflt) in defaults.into_iter().enumerate() {
                if let Some(x) = dflt {
                    if old_defaults.get(k).is_some_and(|o| o.is_some()) {
                        cx.error(span, "DEFARG-9", "a default argument is given again for the same parameter");
                        break;
                    }
                    if k < old_defaults.len() {
                        cx.u.func_mut(e).unwrap().defaults[k] = Some(x);
                    }
                }
            }
            let f = cx.u.func_mut(e).unwrap();
            f.inline |= s.inline;
            f.constexpr |= s.constexpr;
            f.consteval |= s.consteval;
            f.noreturn |= s.attrs.noreturn;
            f.nodiscard |= s.attrs.nodiscard;
            e
        }
        None => {
            if d.qual != names::Qual::None && d.qual != names::Qual::Scope(cx.scope) && names::declared_here(cx, scope, &name).is_empty() {
                crate::declarations::namespaces::check::member_not_declared(cx, span, &name);
                return None;
            }
            let f = Func {
                ty: Rc::new(fty.clone()),
                params: Vec::new(),
                defaults,
                body: None,
                constexpr: s.constexpr,
                consteval: s.consteval,
                inline: s.inline,
                is_static: s.is_static,
                deleted,
                noreturn: s.attrs.noreturn,
                nodiscard: s.attrs.nodiscard,
                c_linkage: cx.lang_c && cx.member_of.is_none(),
                pending_return: deduce,
                placeholder: deduce,
                used: false,
                defined: deleted,
                unsupported: if deduce { Some("a deduced return type that no definition gave".into()) } else { None },
                class: cx.member_of,
                special: crate::sem::Special::None,
                explicit: false,
                member_static: cx.member_of.is_some() && s.is_static,
                obj_cv: d.obj_cv,
            };
            let id = names::declare_in(cx, scope, &name, EntKind::Func(f), span, linkage);
            cx.u.entities[id].deprecated = s.attrs.deprecated.clone();
            id
        }
    };
    // A defaulted copy or move assignment is the implicit one (CPY-3).
    if assign_default {
        cx.u.defaulted.insert(id);
        cx.deferred.remove(&id);
        if let Some(f) = cx.u.func_mut(id) {
            f.defined = true;
            f.inline = true;
        }
    }
    // DEFARG-8: after a parameter with a default argument, each later
    // one has one, from this declaration or an earlier one.
    let all = cx.u.func(id).map(|f| f.defaults.clone()).unwrap_or_default();
    if let Some(first) = all.iter().position(|x| x.is_some()) {
        if all[first..].iter().any(|x| x.is_none()) {
            cx.error(d.span, "DEFARG-8", "a parameter after one with a default argument has none");
        }
    }
    if cx.member_of.is_none() && ((cx.lang_c && !s.is_static) || (scope == cx.u.root && !cx.lang_c)) {
        crate::declarations::linkage_specifications::check::c_name(cx, id, cx.lang_c, span);
    }
    if s.attrs.noreturn && earlier.is_some() && !cx.u.func(id).unwrap().noreturn {
        cx.error(span, "ATTR-2", "'noreturn' on a later declaration of a function first declared without it");
    }
    Some(id)
}

/// The default arguments of a declaration's parameters.
/// DEFARG-4: a local variable of an enclosing function, or a parameter,
/// that an evaluated part of a default argument names.
fn local_in(cx: &Check, e: &Expr) -> Option<EntityId> {
    use crate::sem::tree::EK;
    let sub = |x: &Expr| local_in(cx, x);
    match &e.k {
        EK::Var(v) => cx.u.var(*v).filter(|x| x.storage == Storage::Automatic || x.param).map(|_| *v),
        EK::Unevaluated(_) | EK::Int(_) | EK::Float(_) | EK::LongDouble(_) | EK::Str(_) | EK::Func(_) | EK::Virtual(_) | EK::Closure(_) | EK::This => None,
        EK::Conv(_, x) | EK::Unary(_, x) | EK::IncDec(_, _, x) | EK::Deref(x) | EK::AddrOf(x) | EK::Assume(x) | EK::EndLife(x) | EK::Member(x, _) | EK::BasePtr(x, _) | EK::DerivedPtr(x, _) => sub(x),
        EK::Binary(_, a, b) | EK::PtrAdd(a, b, _) | EK::PtrDiff(a, b) | EK::Logical(_, a, b) | EK::Assign(a, b) | EK::Compound(_, a, b, _) | EK::Comma(a, b) | EK::Index(a, b) => sub(a).or_else(|| sub(b)),
        EK::Cond(c, a, b) => sub(c).or_else(|| sub(a)).or_else(|| sub(b)),
        EK::Call(f, args, _) => sub(f).or_else(|| args.iter().find_map(|x| sub(&x.e))),
        EK::List(i) => {
            fn init(cx: &Check, i: &crate::sem::tree::Init) -> Option<EntityId> {
                use crate::sem::tree::Init;
                match i {
                    Init::Expr(e) | Init::Ref(e) => local_in(cx, e),
                    Init::List(xs) => xs.iter().find_map(|x| init(cx, x)),
                    Init::Ctor(_, args) => args.iter().find_map(|a| local_in(cx, &a.e)),
                    Init::Default | Init::Zero => None,
                }
            }
            init(cx, i)
        }
    }
}

pub fn default_arguments(cx: &mut Check, params: &[Param]) -> Vec<Option<Expr>> {
    let saved = std::mem::replace(&mut cx.default_arg, true);
    let out = default_arguments_in(cx, params);
    cx.default_arg = saved;
    out
}

fn default_arguments_in(cx: &mut Check, params: &[Param]) -> Vec<Option<Expr>> {
    let mut out = Vec::new();
    let mut seen = false;
    for p in params {
        match &p.default {
            Some(n) => {
                seen = true;
                let e = crate::expressions::check::expr(cx, n);
                if let Some(v) = local_in(cx, &e) {
                    let name = cx.u.entities[v].name.clone();
                    cx.error(e.span, "DEFARG-4", format!("the default argument uses the local variable '{}'", name));
                }
                if p.ty.is_ref() {
                    out.push(crate::declarations::initialization::references::check::bind(cx, e, &p.ty, "DEFARG-10"));
                } else {
                    out.push(forms::copy_init(cx, e, &p.ty, "DEFARG-10"));
                }
            }
            None => out.push(None),
        }
    }
    out
}

/// `function_definition`.
pub fn function_definition(cx: &mut Check, n: &Node) {
    if cx.in_function() {
        cx.error(n.span, "FN-9", "a function definition inside a function body");
        return;
    }
    if n.child("decl_specifiers").is_none() && crate::classes::constructors::check::out_of_class(cx, n) {
        return;
    }
    let (attrs, _) = crate::declarations::check::leading_attrs(cx, n);
    let specs = match n.child("decl_specifiers") {
        Some(ds) => crate::declarations::check::decl_specifiers(cx, ds, attrs),
        None => {
            cx.unsupported(n.span, "a function definition with no decl-specifiers (a constructor or conversion function)");
            return;
        }
    };
    if specs.unsupported {
        return;
    }
    // VIRT-3: virt-specifiers only on the declaration in the class.
    if let Some(vs) = n.child("virt_specifiers").filter(|_| cx.member_of.is_none()) {
        cx.error(vs.span, "VIRT-3", "a virt-specifier on a definition outside its class");
        return;
    }
    let Some(dn) = n.child("declarator") else { return };
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    if d.unsupported {
        if let Some((name, sp)) = &d.name {
            names::declare_unsupported(cx, name, *sp);
        }
        return;
    }
    if !d.ty.is_func() {
        cx.unsupported(n.span, "a function definition whose declarator is not a function");
        return;
    }
    let body = n.child("function_body");
    let Some(id) = declare_function(cx, &specs, &d, body) else { return };
    let Some(body) = body else { return };
    if cx.words(body).contains(&"delete") || cx.words(body).contains(&"default") {
        return;
    }
    let Some(cs) = body.child("compound_stmt") else {
        mark_unsupported(cx, id, "a function try block (build step 4)");
        cx.unsupported(body.span, "a function try block (build step 4)");
        return;
    };
    define(cx, id, &d, cs);
}

fn mark_unsupported(cx: &mut Check, id: EntityId, why: &str) {
    if let Some(f) = cx.u.func_mut(id) {
        f.unsupported = Some(why.into());
    }
}

/// Check a function body with the parameters declared (FN-1, FN-12,
/// FN-13), and record it.
pub fn define(cx: &mut Check, id: EntityId, d: &Declared, cs: &Node) {
    define_with(cx, id, d, cs, None)
}

/// `define`, with the member initializer list of a constructor, which
/// sees the parameters (CTOR-5).
pub fn define_with(cx: &mut Check, id: EntityId, d: &Declared, cs: &Node, ctor_init: Option<&Node>) {
    let saved_scope = cx.scope;
    let decl_scope = cx.u.entities[id].scope;
    cx.scope = decl_scope;
    let fs = cx.enter(ScopeKind::Function);
    let fty = cx.u.func(id).unwrap().ty.clone();
    let mut params = Vec::new();
    for (k, p) in d.params.iter().enumerate() {
        let ty = fty.params.get(k).cloned().unwrap_or_else(QType::error);
        let ty = if p.cv.0 || p.cv.1 { ty.with_cv(p.cv.0 || ty.c, p.cv.1 || ty.v) } else { ty };
        if cx.u.is_incomplete(&ty) && !ty.is_void() {
            let class = ty.base_elem().is_class().then_some("TYN-7");
            cx.error_cause(p.span, "FN-12", class, "a function definition with a parameter of incomplete type");
        } else {
            crate::classes::r#virtual::check::not_abstract(cx, &ty, p.span);
        }
        let v = Var {
            ty,
            storage: Storage::Automatic,
            constexpr: false,
            constinit: false,
            inline: false,
            is_extern: false,
            is_static: false,
            param: true,
            defined: true,
            init: None,
            const_init: false,
            used: true,
            maybe_unused: p.maybe_unused,
            align: None,
        };
        let name = p.name.as_ref().map(|(n, _)| n.to_string()).unwrap_or_default();
        let span = p.name.as_ref().map(|(_, s)| *s).unwrap_or(p.span);
        if !name.is_empty() && !names::declared_here(cx, fs, &name).is_empty() {
            cx.error(span, "FN-13", format!("the parameter name '{}' occurs twice", name));
        }
        let pid = names::declare_in(cx, fs, &name, EntKind::Var(v), span, Linkage::None);
        if p.was_array {
            cx.array_params.insert(pid);
        }
        params.push(pid);
    }
    let f = cx.u.func(id).unwrap().clone();
    let ret = fty.ret.clone();
    // TYN-7: a definition does not return an incomplete class.
    if ret.class().is_some_and(|c| !cx.u.classes[c].complete) {
        cx.error(cx.u.entities[id].span, "TYN-7", "a function definition whose return type is an incomplete class");
    } else {
        let at = cx.u.entities[id].span;
        crate::classes::r#virtual::check::not_abstract(cx, &ret, at);
    }
    let deduce = d.placeholder.is_some();
    let ctx = FnCtx {
        id: Some(id),
        ret: if deduce { None } else { Some(ret.clone()) },
        deduce,
        placeholder: d.placeholder.clone().map(|p| {
            let op = match &ret.t {
                Type::Ref(_, false) => Some("&"),
                Type::Ref(_, true) => Some("&&"),
                Type::Ptr(_) => Some("*"),
                _ => None,
            };
            (p, op)
        }),
        constexpr: f.constexpr,
        consteval: f.consteval,
        // CLS rules: `this` points to the object, with the function's
        // cv-qualifiers.
        this: f.class.filter(|_| !f.member_static).map(|c| QType::ptr_to(QType::new(Type::Class(c)).with_cv(f.obj_cv.0, f.obj_cv.1))),
        is_main: cx.u.main() == Some(id) || (&*cx.u.entities[id].name == "main" && decl_scope == cx.u.root),
        ..Default::default()
    };
    if let Some(fm) = cx.u.func_mut(id) {
        fm.params = params;
        fm.defined = true;
        fm.unsupported = None;
    }
    let saved = cx.func.replace(ctx);
    let errors_before = cx.errors();
    let unsup_before = cx.u.diags.iter().filter(|d| d.sev == crate::diag::Severity::Unsupported).count();
    if cx.u.func(id).is_some_and(|f| f.special == crate::sem::Special::Ctor) {
        let mi = crate::classes::constructors::check::mem_inits(cx, id, ctor_init);
        if let Some(mi) = mi {
            cx.u.mem_inits.insert(id, mi);
        }
    }
    let body = crate::statements::check::function_body(cx, cs);
    let ctx = cx.func.take().unwrap_or_default();
    cx.func = saved;
    cx.scope = saved_scope;
    crate::statements::jumps::check::end_of_function(cx, &ctx);
    if deduce && (ret.is_ref() || ret.is_ptr()) && ctx.ret.is_none() && !ctx.failed_return {
        cx.error(cx.u.entities[id].span, "RET-3", "a reference or pointer placeholder return type needs a return statement with a value");
    }
    if deduce {
        let r = ctx.ret.clone().unwrap_or_else(QType::void);
        let fm = cx.u.func_mut(id).unwrap();
        let mut t = (*fm.ty).clone();
        t.ret = r;
        fm.ty = Rc::new(t);
        fm.pending_return = false;
    }
    let unsup_after = cx.u.diags.iter().filter(|d| d.sev == crate::diag::Severity::Unsupported).count();
    let fm = cx.u.func_mut(id).unwrap();
    fm.body = Some(Rc::new(body));
    if unsup_after > unsup_before || ctx.unsupported.is_some() {
        fm.unsupported = Some(ctx.unsupported.unwrap_or_else(|| "the body holds a construct the front end does not model".into()));
    }
    let _ = errors_before;
    flow_off_end_warning(cx, id);
    noreturn_returns(cx, id);
    unused_locals(cx, fs);
    if let Some(b) = cx.u.func(id).and_then(|f| f.body.clone()) {
        crate::declarations::attributes::check::fallthrough_checks(cx, &b);
    }
}

/// Each `return` or the end of the body: nothing to check here beyond
/// FN-3, which the machine reports.
/// FN-7: an implementation should warn if control can reach the end of
/// a function that returns a value, other than `main`.
fn flow_off_end_warning(cx: &mut Check, id: EntityId) {
    let Some(f) = cx.u.func(id) else { return };
    let Some(body) = f.body.clone() else { return };
    if f.ty.ret.is_void() || f.ty.ret.is_error() || f.unsupported.is_some() || cx.u.main() == Some(id) || &*cx.u.entities[id].name == "main" {
        return;
    }
    if !cx.wflag(&["-Wall", "-Wreturn-type"]) {
        return;
    }
    let mut paths = Paths::default();
    if paths.completes(cx, &body) && !paths.unsupported {
        cx.warning(cx.u.entities[id].span, "FN-7", format!("control can reach the end of '{}', which returns a value", cx.u.entities[id].name));
    }
}

/// ATTR-14: a function declared `noreturn` whose end control can reach.
fn noreturn_returns(cx: &mut Check, id: EntityId) {
    let Some(f) = cx.u.func(id) else { return };
    let Some(body) = f.body.clone() else { return };
    if f.noreturn && f.unsupported.is_none() && can_complete(cx, &body) {
        cx.warning(cx.u.entities[id].span, "ATTR-14", format!("the noreturn function '{}' can return", cx.u.entities[id].name));
    }
}

/// ATTR-6: a local variable of the function that the program never
/// reads, unless it is declared `maybe_unused`.
fn unused_locals(cx: &mut Check, fs: crate::sem::ScopeId) {
    let inside = |cx: &Check, mut s: crate::sem::ScopeId| loop {
        if s == fs {
            return true;
        }
        match cx.u.scopes[s].parent {
            Some(p) => s = p,
            None => return false,
        }
    };
    let found: Vec<(Span, Rc<str>)> = cx.u.entities.iter().enumerate().skip(cx.u.first_entity).filter_map(|(id, e)| match &e.kind {
        EntKind::Var(v) if !v.param && !v.maybe_unused && v.storage == Storage::Automatic && !e.name.is_empty() && inside(cx, e.scope) && !crate::declarations::initialization::forms::check::is_read(cx, id) && !v.init.as_ref().is_some_and(|i| i.constructs()) && !crate::classes::constructors::run::needs_in(&cx.u, &v.ty) => Some((e.span, e.name.clone())),
        _ => None,
    }).collect();
    for (span, name) in found {
        crate::declarations::attributes::check::unused_variable(cx, span, &name);
    }
}

/// Whether control can reach the end of a statement (the analysis of
/// FN-7, which ATTR-7 shares).
pub fn can_complete(cx: &Check, s: &Stmt) -> bool {
    let mut paths = Paths::default();
    paths.completes(cx, s) || paths.unsupported
}

/// The exits that FN-7's check collects while it walks a body.
#[derive(Default)]
struct Paths {
    /// The statements that a reachable `break` leaves.
    breaks: Vec<crate::sem::tree::StmtId>,
    /// The labels that a reachable `goto` names.
    gotos: Vec<Rc<str>>,
    unsupported: bool,
}

impl Paths {
    /// Whether control can reach the end of `s` from its start.
    fn completes(&mut self, cx: &Check, s: &Stmt) -> bool {
        use crate::sem::tree::Cond;
        let constant = |c: &Cond| match c {
            Cond::Expr(e) => constant_bool(e),
            Cond::Decl(..) => None,
        };
        match &s.k {
            SK::Null | SK::Fallthrough | SK::Decl(_) | SK::RangeArray { .. } => true,
            SK::Expr(e) => !ends_path(cx, e),
            SK::Block(items) => {
                let mut reachable = true;
                for it in items {
                    if matches!(it.k, SK::Label(..) | SK::Case(..)) {
                        reachable = true;
                    }
                    if reachable {
                        reachable = self.completes(cx, it);
                    }
                }
                reachable
            }
            SK::If { cond, then, els, .. } => match constant(cond) {
                Some(true) => self.completes(cx, then),
                Some(false) => els.as_ref().map_or(true, |e| self.completes(cx, e)),
                None => {
                    let a = self.completes(cx, then);
                    let b = els.as_ref().map_or(true, |e| self.completes(cx, e));
                    a || b
                }
            },
            SK::Chosen(c) => c.as_ref().map_or(true, |c| self.completes(cx, c)),
            SK::Consteval { constant, runtime } => {
                let a = self.completes(cx, constant);
                let b = self.completes(cx, runtime);
                a || b
            }
            SK::While { id, cond, body } => self.forever(cx, *id, body, constant(cond) == Some(true)),
            SK::For { id, cond, body, .. } => self.forever(cx, *id, body, cond.as_ref().map_or(true, |c| constant(c) == Some(true))),
            SK::Do { id, body, cond } => self.forever(cx, *id, body, constant_bool(cond) == Some(true)),
            SK::Switch { id, body, .. } => {
                // A switch without a default label can complete,
                // whatever its cases are.
                let mut has_default = false;
                walk(body, &mut |x| has_default |= matches!(x.k, SK::Case(sid, None, _) if sid == *id));
                let inner = self.completes(cx, body);
                !has_default || inner || self.breaks.contains(id)
            }
            SK::Case(_, _, inner) | SK::Label(_, inner) => self.completes(cx, inner),
            SK::Break(id) => {
                self.breaks.push(*id);
                false
            }
            SK::Goto(l) => {
                self.gotos.push(l.clone());
                false
            }
            SK::Continue(_) | SK::Return(_) | SK::ReturnRef(_) => false,
            SK::Unsupported(_) => {
                self.unsupported = true;
                true
            }
        }
    }

    /// A loop: it ends the path when its condition is a constant `true`,
    /// unless a `break` of it or a `goto` to a label outside it leaves
    /// it.
    fn forever(&mut self, cx: &Check, id: crate::sem::tree::StmtId, body: &Stmt, always: bool) -> bool {
        let gotos = self.gotos.len();
        self.completes(cx, body);
        if !always {
            return true;
        }
        let mut inside: Vec<Rc<str>> = Vec::new();
        walk(body, &mut |x| {
            if let SK::Label(l, _) = &x.k {
                inside.push(l.clone());
            }
        });
        self.breaks.contains(&id) || self.gotos[gotos..].iter().any(|l| !inside.contains(l))
    }
}

/// A condition whose value is a literal constant.
fn constant_bool(e: &Expr) -> Option<bool> {
    match &e.k {
        crate::sem::tree::EK::Int(v) => Some(*v != 0),
        crate::sem::tree::EK::Conv(_, x) => constant_bool(x),
        _ => None,
    }
}

/// An expression statement that ends a path: a call of a function
/// declared `[[noreturn]]`.
fn ends_path(cx: &Check, e: &Expr) -> bool {
    use crate::sem::tree::EK;
    match &e.k {
        EK::Conv(_, x) => ends_path(cx, x),
        EK::Call(f, _, _) => matches!(&f.k, EK::Func(id) if cx.u.func(*id).is_some_and(|x| x.noreturn)),
        _ => false,
    }
}

/// Each statement of `s`, `s` first.
fn walk(s: &Stmt, f: &mut impl FnMut(&Stmt)) {
    f(s);
    match &s.k {
        SK::Block(items) => items.iter().for_each(|x| walk(x, f)),
        SK::If { init, then, els, .. } => {
            init.iter().for_each(|x| walk(x, f));
            walk(then, f);
            els.iter().for_each(|x| walk(x, f));
        }
        SK::Chosen(c) => c.iter().for_each(|x| walk(x, f)),
        SK::Consteval { constant, runtime } => {
            walk(constant, f);
            walk(runtime, f);
        }
        SK::While { body, .. } | SK::Do { body, .. } => walk(body, f),
        SK::For { init, body, .. } | SK::RangeArray { init, body, .. } | SK::Switch { init, body, .. } => {
            init.iter().for_each(|x| walk(x, f));
            walk(body, f);
        }
        SK::Case(_, _, x) | SK::Label(_, x) => walk(x, f),
        _ => {}
    }
}

/// FN-4: a program must not call `main`.
pub fn called(cx: &mut Check, f: EntityId, span: Span) {
    if cx.u.main() == Some(f) || (&*cx.u.entities[f].name == "main" && cx.u.entities[f].scope == cx.u.root) {
        cx.error_cause(span, "FN-4", Some("TR-3"), "main is called");
    }
    if cx.u.func(f).is_some_and(|x| x.deleted) {
        cx.error(span, "DEL-1", format!("'{}' is deleted", cx.u.entities[f].name));
    }
}

/// FN-4: a program must not take the address of `main`.
pub fn address_taken(cx: &mut Check, f: EntityId, span: Span) {
    crate::declarations::constexpr::check::address_of(cx, f, span);
    if &*cx.u.entities[f].name == "main" && cx.u.entities[f].scope == 0 {
        cx.error_cause(span, "FN-4", Some("TR-3"), "the address of main is taken");
    }
}

/// A use of a function by name: DEL-1 for a deleted function.
pub fn note_use(cx: &mut Check, f: EntityId, span: Span) {
    // An unevaluated operand does not use the function (ODR-9).
    if cx.unevaluated == 0 {
        if let Some(x) = cx.u.func_mut(f) {
            x.used = true;
        }
    }
    if cx.u.func(f).is_some_and(|x| x.deleted) {
        cx.error(span, "DEL-1", format!("'{}' is deleted", cx.u.entities[f].name));
    }
    // RET-3: no use before a return statement gives the type.
    if cx.u.func(f).is_some_and(|x| x.pending_return) {
        cx.error(span, "RET-3", format!("'{}' is used before its return type is deduced", cx.u.entities[f].name));
    }
}

/// FN-5: `__func__` outside a function body.
pub fn func_name_outside(cx: &mut Check, span: Span) {
    cx.error(span, "FN-5", "__func__ outside a function body");
}

/// The checks at the end of the unit (ODR-9, INL-6).
pub fn end_of_unit(cx: &mut Check) {
    // ODR-9: a function with internal linkage that the unit uses is
    // defined in the unit. No diagnostic is required; the front end
    // gives one.
    let missing: Vec<(Span, Rc<str>)> = cx.u.entities.iter().skip(cx.u.first_entity).filter_map(|e| match &e.kind {
        EntKind::Func(f) if f.used && !f.defined && !f.deleted && f.unsupported.is_none() && e.linkage == Linkage::Internal => Some((e.span, e.name.clone())),
        _ => None,
    }).collect();
    for (span, name) in missing {
        cx.error(span, "ODR-9", format!("the function '{}' has internal linkage and is used, but the unit does not define it", name));
    }
    // INL-6: an inline variable that the unit uses is defined in the
    // unit. No diagnostic is required; the compilers give one for a
    // variable, and the front end does too. A missing inline function
    // fails at link time on Clang and GCC, so the front end accepts it.
    let missing: Vec<(Span, Rc<str>)> = cx.u.entities.iter().skip(cx.u.first_entity).filter_map(|e| match &e.kind {
        EntKind::Var(v) if v.inline && v.used && !v.defined => Some((e.span, e.name.clone())),
        _ => None,
    }).collect();
    for (span, name) in missing {
        cx.error(span, "INL-6", format!("the inline variable '{}' is used, but the unit does not define it", name));
    }
}

/// The `__func__` variable of a function body (FN-11): a static array
/// of `const char` with the function's name.
pub fn func_name_var(cx: &mut Check, name: &str) -> EntityId {
    let mut units: Vec<u32> = name.chars().map(|c| c as u32).collect();
    units.push(0);
    let n = units.len() as u64;
    let el = QType { t: Type::Int(crate::sem::types::IntK::Char), c: true, v: false };
    let ty = QType::new(Type::Array(Rc::new(el.clone()), Some(n)));
    let init = crate::sem::tree::Init::List(units.iter().map(|&u| crate::sem::tree::Init::Expr(Expr::int(u as i128, el.unqual(), Span::default()))).collect());
    let v = Var {
        ty,
        storage: Storage::Static,
        constexpr: false,
        constinit: false,
        inline: false,
        is_extern: false,
        is_static: true,
        param: false,
        defined: true,
        init: Some(init),
        const_init: true,
        used: true,
        maybe_unused: true,
        align: None,
    };
    let id = names::declare(cx, "__func__", EntKind::Var(v), Span::default(), Linkage::None);
    id
}

impl FnType {
    /// No part of the type is the error type.
    pub fn is_error_free(&self) -> bool {
        !self.ret.is_error() && !self.params.iter().any(|p| p.is_error())
    }
}

/// The statement of a function body that a block scope wraps.
pub fn body_block(items: Vec<Stmt>, span: Span) -> Stmt {
    Stmt::new(SK::Block(items), span)
}

/// The kids of a node, for callers that walk a function body.
pub fn kids(n: &Node) -> &[Kid] {
    &n.kids
}

/// The function has `[[nodiscard]]` (ATTR-5).
pub fn is_nodiscard(cx: &Check, f: EntityId) -> bool {
    cx.u.func(f).is_some_and(|x| x.nodiscard)
}
