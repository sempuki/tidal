//! The check part of `declarations/exception-specifications`.

use crate::declarations::declarators::check::Param;
use crate::foundations::constant_evaluation::check as ce;
use crate::sem::check::Check;
use crate::sem::tree::{Expr, EK};
use crate::sem::types::{QType, Type};
use crate::sem::{EntKind, Linkage, ScopeKind, Storage, Var};
use crate::syntax::Node;

/// `exception_spec`: whether the function is non-throwing (EXS-1,
/// EXS-2). `noexcept(e)` needs a constant that converts to `bool` with
/// no narrowing (EXS-7). The operand can name the parameters.
pub fn exception_spec(cx: &mut Check, n: &Node, params: &[Param]) -> bool {
    let Some(ns) = n.child("noexcept_specifier") else {
        // `throw()` is non-throwing (EXS-5).
        return true;
    };
    match ns.kids.iter().find_map(|k| match k {
        crate::syntax::Kid::Node(c) => Some(c),
        _ => None,
    }) {
        None => true,
        Some(e) if matches!(cx.source(e).as_str(), "true" | "false") => ce::bool_constant(cx, e, "EXS-3").unwrap_or(true),
        Some(e) => {
            let saved = cx.scope;
            let s = cx.enter(ScopeKind::Function);
            for p in params {
                if let Some((name, span)) = &p.name {
                    let v = Var { ty: p.ty.clone(), storage: Storage::Automatic, constexpr: false, constinit: false, inline: false, is_extern: false, is_static: false, param: true, defined: true, init: None, const_init: false, used: true, maybe_unused: true, align: None };
                    crate::foundations::names::check::declare_in(cx, s, name, EntKind::Var(v), *span, Linkage::None);
                }
            }
            let r = operand(cx, e);
            cx.scope = saved;
            r
        }
    }
}

fn operand(cx: &mut Check, n: &Node) -> bool {
    let e = crate::expressions::check::expr(cx, n);
    let e = crate::foundations::values::check::rvalue(cx, e);
    if e.ty.is_error() {
        return true;
    }
    if crate::foundations::conversions::check::is_narrowing(cx, &e, &QType::bool()) {
        cx.error(e.span, "EXS-7", format!("the operand of noexcept converts from {} to bool by narrowing", e.ty.show()));
        return true;
    }
    let b = crate::foundations::conversions::check::contextual_bool(cx, e, "EXS-7");
    if b.ty.is_error() {
        return true;
    }
    match ce::evaluate(cx, &b) {
        Ok(v) => v.truthy(),
        Err(why) => {
            ce::report(cx, b.span, "EXS-7", why);
            true
        }
    }
}

/// UN-13: whether an operand of the `noexcept` operator can throw. In
/// build step 3 only a call of a function whose type is not
/// non-throwing can (EXS-4).
pub fn can_throw(e: &Expr) -> bool {
    e.any(&|x| match &x.k {
        EK::Call(f, _) => {
            let t = f.ty.nonref();
            let t = match &t.t {
                Type::Ptr(q) => (**q).clone(),
                _ => t,
            };
            !matches!(&t.t, Type::Func(ft) if ft.noexcept)
        }
        _ => false,
    })
}
