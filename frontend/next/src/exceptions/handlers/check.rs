//! The check part of `exceptions/handlers`: a try block and its
//! handlers (TRY-1 to TRY-7, TRY-14).

use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, Handler, Init, Stmt, EK, SK};
use crate::sem::types::{QType, Type};
use crate::sem::{EntKind, Linkage, ScopeKind, Storage, Var};
use crate::syntax::Node;

/// `try_block`.
pub fn try_block(cx: &mut Check, n: &Node) -> Stmt {
    let span = n.span;
    let Some(block) = n.child("compound_stmt") else { return Stmt::new(SK::Null, span) };
    let start = cx.u.entities.len();
    let body = region(cx, start, |cx| crate::statements::blocks::check::compound_stmt(cx, block, None));
    let hs: Vec<&Node> = n.children("handler").collect();
    let mut handlers = Vec::new();
    for (k, h) in hs.iter().enumerate() {
        let Some(ed) = h.child("exception_decl") else { continue };
        let all = cx.words(ed) == ["..."];
        // TRY-3: `catch (...)` comes last.
        if all && k + 1 != hs.len() {
            cx.error(h.span, "TRY-3", "'catch (...)' is not the last handler of its try block");
        }
        let saved = cx.scope;
        let scope = cx.enter(ScopeKind::Block);
        let hstart = cx.u.entities.len();
        let param = if all { Ok(None) } else { parameter(cx, ed) };
        let Ok(param) = param else {
            cx.scope = saved;
            return Stmt::new(SK::Null, span);
        };
        let Some(hb) = h.child("compound_stmt") else {
            cx.scope = saved;
            continue;
        };
        // TRY-14: the outermost block of the handler does not declare
        // the parameter again.
        let body = region(cx, hstart, |cx| crate::statements::blocks::check::compound_stmt(cx, hb, Some((scope, "TRY-14"))));
        cx.scope = saved;
        let (ty, param) = match param {
            Some((t, p)) => (Some(t), p),
            None => (None, None),
        };
        handlers.push(Handler { ty, param, body: Box::new(body) });
    }
    // TRY-4: a handler that an earlier handler of the try block always
    // shadows warns.
    for (k, h) in handlers.iter().enumerate() {
        let Some(t) = &h.ty else { continue };
        let shadowed = handlers[..k].iter().any(|e| e.ty.as_ref().is_some_and(|u| crate::exceptions::throw::check::catches(cx, u, t) && (u.nonref().is_class() || u.nonref().unqual() == t.nonref().unqual() || u.nonref().pointee().is_some())));
        if shadowed {
            cx.warning(h.body.span, "TRY-4", "the handler is shadowed by an earlier handler of the try block");
        }
    }
    Stmt::new(SK::Try { body: Box::new(body), handlers }, span)
}

/// Check a try block's body or a handler: no jump enters it (TRY-7), and
/// a throw inside it moves only a variable declared inside it (THR-1).
fn region(cx: &mut Check, start: usize, f: impl FnOnce(&mut Check) -> Stmt) -> Stmt {
    let id = cx.next_stmt_id();
    if let Some(fc) = cx.func.as_mut() {
        fc.constexpr_ifs.push(id);
        fc.try_regions.push(id);
        fc.tries.push(start);
    }
    let s = f(cx);
    if let Some(fc) = cx.func.as_mut() {
        fc.constexpr_ifs.pop();
        fc.tries.pop();
    }
    s
}

/// The parameter of a handler and its initialization (TRY-2).
type Param = (QType, Option<(crate::sem::EntityId, Init)>);

/// The handler parameter (TRY-5): its type, and its variable with the
/// initialization from the exception object, if it has a name. Err after
/// an error.
fn parameter(cx: &mut Check, ed: &Node) -> Result<Option<Param>, ()> {
    let span = ed.span;
    let specs: Vec<&Node> = ed.child("type_specifiers").map(|s| s.children("type_specifier").collect()).unwrap_or_default();
    let (base, ph, _, unsup) = crate::foundations::types::check::type_specifiers(cx, &specs, span);
    if unsup {
        return Err(());
    }
    if ph.is_some() {
        cx.error(span, "TRY-5", "a handler parameter with a placeholder type");
        return Err(());
    }
    let Some(base) = base else { return Err(()) };
    let (mut ty, name) = match ed.child("declarator") {
        Some(dn) => {
            let s = crate::declarations::check::Specs { ty: Some(base), ..Default::default() };
            let d = crate::declarations::declarators::check::declarator(cx, &s, dn);
            if d.unsupported {
                return Err(());
            }
            if d.qual != crate::foundations::names::check::Qual::None {
                cx.error(span, "TRY-5", "a handler parameter with a qualified name");
                return Err(());
            }
            (d.ty, d.name)
        }
        None => (crate::declarations::declarators::check::type_id(cx, ed), None),
    };
    if ty.is_error() {
        return Err(());
    }
    // An array or a function decays, as for a function parameter.
    match ty.t.clone() {
        Type::Array(e, _) => ty = QType::ptr_to((*e).clone()),
        Type::Func(_) => ty = QType::ptr_to(ty.clone()),
        _ => {}
    }
    if matches!(ty.t, Type::Ref(_, true)) {
        cx.error(span, "TRY-5", "a handler parameter of rvalue reference type");
        return Err(());
    }
    let inner = ty.nonref();
    let incomplete = |cx: &Check, t: &QType| t.is_void() || cx.u.is_incomplete(t) || matches!(t.t, Type::Array(_, None));
    if incomplete(cx, &inner) {
        cx.error(span, "TRY-5", format!("a handler parameter of the incomplete type {}", ty.show()));
        return Err(());
    }
    if let Some(p) = inner.pointee().filter(|p| !p.is_void() && !p.is_func()) {
        if incomplete(cx, p) {
            cx.error(span, "TRY-5", format!("a handler parameter that points to the incomplete type {}", p.show()));
            return Err(());
        }
    }
    if !ty.is_ref() && !crate::classes::r#virtual::check::not_abstract(cx, &ty, span) {
        return Err(());
    }
    // The initialization of the parameter from the exception object: a
    // reference binds to it, a class is copied (TRY-5), and a scalar
    // takes its value.
    let caught = Expr::new(EK::Caught, inner.clone(), Cat::L, span);
    let init = if ty.is_ref() {
        Init::Ref(caught)
    } else if ty.is_class() {
        match crate::classes::copy_move::check::copy(cx, caught, false, span) {
            Some(x) => Init::Expr(x),
            None => return Err(()),
        }
    } else {
        Init::Expr(crate::foundations::values::check::rvalue(cx, caught))
    };
    let Some((name, nspan)) = name else { return Ok(Some((ty, None))) };
    let var = Var {
        ty: ty.clone(),
        storage: Storage::Automatic,
        constexpr: false,
        constinit: false,
        inline: false,
        is_extern: false,
        is_static: false,
        param: false,
        defined: true,
        init: Some(init.clone()),
        const_init: false,
        used: false,
        maybe_unused: true,
        align: None,
    };
    let scope = cx.scope;
    let id = crate::foundations::names::check::declare_in(cx, scope, &name, EntKind::Var(var), nspan, Linkage::None);
    Ok(Some((ty, Some((id, init)))))
}
