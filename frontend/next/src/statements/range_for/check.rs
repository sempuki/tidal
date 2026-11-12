//! The check part of `statements/range-for`, for a range that is an
//! array (RFOR-1, RFOR-5 to RFOR-7). A range of class type comes with
//! build step 4.

use crate::expressions::check::{expr, first_node};
use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, Init, Stmt, EK, SK};
use crate::sem::types::{QType, Type};
use crate::sem::{EntKind, Linkage, ScopeKind, Storage, Var};
use crate::statements::check::sub_statement;
use crate::statements::loops::check::{pop_target, push_target};
use crate::syntax::Node;

fn unsupported(cx: &mut Check, n: &Node, why: &str) -> Stmt {
    cx.unsupported(n.span, why.to_string());
    if let Some(f) = cx.func.as_mut() {
        f.unsupported = Some(why.into());
    }
    Stmt::new(SK::Unsupported(why.into()), n.span)
}

fn local_var(ty: QType, init: Option<Init>) -> Var {
    Var {
        ty,
        storage: Storage::Automatic,
        constexpr: false,
        constinit: false,
        inline: false,
        is_extern: false,
        is_static: false,
        param: false,
        defined: true,
        init,
        const_init: false,
        used: true,
        maybe_unused: true,
        align: None,
    }
}

/// `range_for_stmt`.
pub fn range_for_stmt(cx: &mut Check, n: &Node) -> Stmt {
    let saved = cx.scope;
    let cs = cx.enter(ScopeKind::Block);
    let labels = cx.func.as_mut().map(|f| std::mem::take(&mut f.pending_labels)).unwrap_or_default();
    let r = (|| -> Stmt {
        let init = n.child("init_stmt").and_then(|i| crate::statements::if_stmt::check::init_stmt(cx, i));
        let Some(rd) = n.child("range_decl") else { return Stmt::new(SK::Null, n.span) };
        let Some(ri) = n.child("range_init") else { return Stmt::new(SK::Null, n.span) };
        let Some(re) = first_node(ri) else { return Stmt::new(SK::Null, n.span) };
        if re.prod == "braced_init" {
            return unsupported(cx, n, "a range-based for over a braced list (std::initializer_list, build step 4)");
        }
        let range = expr(cx, re);
        if range.ty.is_error() {
            return Stmt::new(SK::Null, n.span);
        }
        let range = crate::foundations::values::check::deref_ref(range);
        let Type::Array(el, Some(bound)) = range.ty.t.clone() else {
            if range.ty.is_ptr() || range.ty.is_arithmetic() || range.ty.is_enum() {
                cx.error(range.span, "RFOR-7", format!("the range has type {}, which is not a range", range.ty.show()));
                return Stmt::new(SK::Null, n.span);
            }
            return unsupported(cx, n, "a range-based for over a range of this type");
        };
        let Some(ds) = rd.child("decl_specifiers") else {
            return unsupported(cx, n, "a structured binding as the loop variable");
        };
        let Some(dn) = rd.child("declarator") else {
            return unsupported(cx, n, "a structured binding as the loop variable");
        };
        let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
        if specs.is_static || specs.thread_local || specs.is_extern {
            cx.error(ds.span, "RFOR-5", "the loop variable must not be static, thread_local, or extern");
            return Stmt::new(SK::Null, n.span);
        }
        let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
        let elem = Expr::new(EK::Int(0), (*el).clone(), if range.cat == Cat::PR { Cat::X } else { Cat::L }, range.span);
        let ty = match &d.placeholder {
            Some(p) => match crate::declarations::auto::check::deduce(cx, p, dn, &elem) {
                Some(base) => crate::declarations::declarators::check::with_base(cx, base, dn),
                None => return Stmt::new(SK::Null, n.span),
            },
            None => d.ty.clone(),
        };
        let fits = ty.nonref().unqual().t == el.unqual().t && (!ty.is_ref() || ((!el.c || ty.nonref().c) && (!el.v || ty.nonref().v)));
        if !fits {
            return unsupported(cx, n, "a loop variable whose type converts from the element type");
        }
        let Some((name, span)) = d.name.clone() else { return Stmt::new(SK::Null, n.span) };
        let range_ty = QType::lref(range.ty.clone());
        let range_id = cx.u.add(crate::sem::Entity { name: "".into(), kind: EntKind::Var(local_var(range_ty.clone(), None)), span: n.span, scope: cs, linkage: Linkage::None, deprecated: None });
        let range_init = match crate::declarations::initialization::references::check::bind(cx, range, &range_ty, "RFOR-1") {
            Some(e) => Init::Ref(e),
            None => return Stmt::new(SK::Null, n.span),
        };
        let vs = cx.enter(ScopeKind::Block);
        let var = names::declare_in(cx, vs, &name, EntKind::Var(local_var(ty, None)), span, Linkage::None);
        if let Some(f) = cx.func.as_mut() {
            f.pending_labels = labels.clone();
        }
        let id = push_target(cx, true);
        let body = n.child("statement").map(|b| sub_statement(cx, b, Some((vs, "RFOR-6"))));
        pop_target(cx);
        let Some(body) = body else { return Stmt::new(SK::Null, n.span) };
        Stmt::new(SK::RangeArray { id, init, range: range_id, range_init, var, bound, body: Box::new(body) }, n.span)
    })();
    cx.scope = saved;
    r
}
