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
    name == "main" && scope == 0
}

/// FN-4, TR-3: what `main` must be.
fn check_main(cx: &mut Check, s: &Specs, d: &Declared, span: Span) {
    let f = d.ty.func();
    if d.placeholder.is_some() || d.trailing && f.is_some_and(|f| f.ret.t != Type::int()) {
        cx.error(span, "TR-3", "the return type of main must be int, written out");
        return;
    }
    if f.is_some_and(|f| f.ret.unqual().t != Type::int()) {
        cx.error(span, "TR-3", "the return type of main must be int");
    }
    if s.inline || s.constexpr || s.consteval {
        cx.error(span, "TR-3", "main must not be inline, constexpr, or consteval");
    }
}

/// Declare a function, or find its earlier declaration (FN-14, DCL-14,
/// DEFARG-9). `body` is the function body of a definition.
pub fn declare_function(cx: &mut Check, s: &Specs, d: &Declared, body: Option<&Node>) -> Option<EntityId> {
    let (name, span) = d.name.clone()?;
    let scope = match d.qual {
        // A member of an inline namespace of the named one (NS-15).
        names::Qual::Scope(sc) => names::lookup_in(cx, sc, &name).iter().find(|&&e| matches!(cx.u.entities[e].kind, EntKind::Func(_))).map(|&e| cx.u.entities[e].scope).unwrap_or(sc),
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
        cx.error(span, "TR-3", "main has C language linkage");
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
    let linkage = if s.is_static { Linkage::Internal } else { stl::linkage(cx, s, false, false, false) };
    let deleted = body.is_some_and(|b| cx.words(b).contains(&"delete"));
    let defaulted = body.is_some_and(|b| cx.words(b).contains(&"default"));
    if defaulted {
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
                if f.ty.params != fty.params || f.ty.variadic != fty.variadic {
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
                    cx.error(span, "DCL-14", format!("the declarations of '{}' disagree on noexcept", name));
                    return None;
                }
                if body.is_some() && !deleted && f.defined {
                    cx.error(span, "FN-14", format!("the function '{}' is defined twice", name));
                    return None;
                }
                if deleted {
                    cx.error(span, "DEL-2", format!("'= delete' must be on the first declaration of '{}'", name));
                    return None;
                }
                if s.inline && !f.inline && f.defined {
                    crate::declarations::inline::check::misplaced(cx, d.span, "on a function after its definition");
                }
                let old_link = cx.u.entities[e].linkage;
                stl::redeclared_linkage(cx, span, old_link, s, &name);
                earlier = Some(e);
                break;
            }
            EntKind::Unsupported(_) | EntKind::Enum(_) => {}
            _ => {
                if !local {
                    cx.error(span, "NAM-3", format!("'{}' is already declared as a different kind of entity", name));
                    return None;
                }
            }
        }
    }
    let id = match earlier {
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
                c_linkage: cx.lang_c,
                pending_return: deduce,
                placeholder: deduce,
                defined: deleted,
                unsupported: if deduce { Some("a deduced return type that no definition gave".into()) } else { None },
            };
            let id = names::declare_in(cx, scope, &name, EntKind::Func(f), span, linkage);
            cx.u.entities[id].deprecated = s.attrs.deprecated.clone();
            id
        }
    };
    // DEFARG-8: after a parameter with a default argument, each later
    // one has one, from this declaration or an earlier one.
    let all = cx.u.func(id).map(|f| f.defaults.clone()).unwrap_or_default();
    if let Some(first) = all.iter().position(|x| x.is_some()) {
        if all[first..].iter().any(|x| x.is_none()) {
            cx.error(d.span, "DEFARG-8", "a parameter after one with a default argument has none");
        }
    }
    if (cx.lang_c && !s.is_static) || (scope == 0 && !cx.lang_c) {
        crate::declarations::linkage_specifications::check::c_name(cx, id, cx.lang_c, span);
    }
    if s.attrs.noreturn && earlier.is_some() && !cx.u.func(id).unwrap().noreturn {
        cx.error(span, "ATTR-2", "'noreturn' on a later declaration of a function first declared without it");
    }
    Some(id)
}

/// The default arguments of a declaration's parameters.
fn default_arguments(cx: &mut Check, params: &[Param]) -> Vec<Option<Expr>> {
    let mut out = Vec::new();
    let mut seen = false;
    for p in params {
        match &p.default {
            Some(n) => {
                seen = true;
                let e = crate::expressions::check::expr(cx, n);
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
    if cx.words(body).contains(&"delete") {
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
fn define(cx: &mut Check, id: EntityId, d: &Declared, cs: &Node) {
    let saved_scope = cx.scope;
    let decl_scope = cx.u.entities[id].scope;
    cx.scope = decl_scope;
    let fs = cx.enter(ScopeKind::Function);
    let fty = cx.u.func(id).unwrap().ty.clone();
    let mut params = Vec::new();
    for (k, p) in d.params.iter().enumerate() {
        let ty = fty.params.get(k).cloned().unwrap_or_else(QType::error);
        if ty.is_incomplete() && !ty.is_void() {
            cx.error(p.span, "FN-12", "a function definition with a parameter of incomplete type");
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
        params.push(pid);
    }
    let f = cx.u.func(id).unwrap().clone();
    let ret = fty.ret.clone();
    let deduce = d.placeholder.is_some();
    let ctx = FnCtx {
        id: Some(id),
        ret: if deduce { None } else { Some(ret.clone()) },
        deduce,
        constexpr: f.constexpr,
        consteval: f.consteval,
        is_main: cx.u.main() == Some(id) || (&*cx.u.entities[id].name == "main" && decl_scope == 0),
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
    let body = crate::statements::check::function_body(cx, cs);
    let ctx = cx.func.take().unwrap_or_default();
    cx.func = saved;
    cx.scope = saved_scope;
    crate::statements::jumps::check::end_of_function(cx, &ctx);
    if d.placeholder.as_ref().is_some_and(|p| p.decltype) && ctx.ret.is_some() {
        // decltype(auto) deduces by AUTO-10, which return deduction does
        // not apply yet.
        cx.unsupported(cx.u.entities[id].span, "a decltype(auto) return type");
    }
    if deduce && (ret.is_ref() || ret.is_ptr()) {
        if ctx.ret.is_none() {
            cx.error(cx.u.entities[id].span, "RET-3", "a reference or pointer placeholder return type needs a return statement with a value");
        } else {
            // Return deduction does not apply the declarator yet, so the
            // deduced type is not trusted and the unit is unsupported.
            cx.unsupported(cx.u.entities[id].span, "a deduced return type with a reference or pointer declarator");
        }
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
}

/// Each `return` or the end of the body: nothing to check here beyond
/// FN-3, which the machine reports.
fn flow_off_end_warning(_cx: &mut Check, _id: EntityId) {}

/// FN-4: a program must not call `main`.
pub fn called(cx: &mut Check, f: EntityId, span: Span) {
    if cx.u.main() == Some(f) || (&*cx.u.entities[f].name == "main" && cx.u.entities[f].scope == 0) {
        cx.error(span, "FN-4", "main is called");
    }
    if cx.u.func(f).is_some_and(|x| x.deleted) {
        cx.error(span, "DEL-1", format!("'{}' is deleted", cx.u.entities[f].name));
    }
}

/// FN-4: a program must not take the address of `main`.
pub fn address_taken(cx: &mut Check, f: EntityId, span: Span) {
    if &*cx.u.entities[f].name == "main" && cx.u.entities[f].scope == 0 {
        cx.error(span, "FN-4", "the address of main is taken");
    }
}

/// A use of a function by name: DEL-1 for a deleted function.
pub fn note_use(cx: &mut Check, f: EntityId, span: Span) {
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

/// The checks at the end of the unit: none yet in build step 3.
pub fn end_of_unit(_cx: &mut Check) {}

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
