//! The check part of `statements/range-for`: a range that is an array
//! (RFOR-1, RFOR-5 to RFOR-7), or of class type, which the loop walks
//! with `begin` and `end` (RFOR-2 to RFOR-4).

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
        if re.prod == "braced_init" && !crate::foundations::names::check::std_declares(cx, "initializer_list") {
            cx.error_cause(re.span, "AUTO-21", Some("LSUP-1"), "a range-based for over a braced list needs a declaration of std::initializer_list");
            return Stmt::new(SK::Null, n.span);
        }
        // RFOR-1, ILC-5: a braced list is a std::initializer_list<E>, held
        // with its array by hidden variables of the loop.
        if re.prod == "braced_init" {
            let items: Vec<&Node> = re.children("init_clause").collect();
            let Some(lt) = crate::classes::initializer_lists::check::deduced(cx, &items, re.span) else { return Stmt::new(SK::Null, n.span) };
            let saved_ok = cx.backing_ok.replace(crate::sem::Storage::Automatic);
            cx.backing.clear();
            let made = crate::classes::initializer_lists::check::backed(cx, &lt, &items, crate::sem::Storage::Automatic, re.span);
            cx.backing_ok = saved_ok;
            let Some(li) = made else { return Stmt::new(SK::Null, n.span) };
            let il = cx.u.add(crate::sem::Entity { name: "".into(), kind: EntKind::Var(local_var(lt.clone(), Some(li.clone()))), span: re.span, scope: cs, linkage: Linkage::None, deprecated: None });
            let mut decls = std::mem::take(&mut cx.backing);
            decls.push((il, li));
            let decl = Stmt::new(SK::Decl(decls), re.span);
            let init = Some(match init {
                Some(s) => Box::new(Stmt::new(SK::Block(vec![*s, decl]), n.span)),
                None => Box::new(decl),
            });
            let range = Expr::new(EK::Var(il), lt, Cat::L, re.span);
            return class_range(cx, n, cs, &labels, init, rd, range);
        }
        let range = expr(cx, re);
        if range.ty.is_error() {
            return Stmt::new(SK::Null, n.span);
        }
        let range = crate::foundations::values::check::deref_ref(range);
        let Type::Array(el, Some(bound)) = range.ty.t.clone() else {
            if range.ty.is_ptr() || range.ty.is_arithmetic() || range.ty.is_enum() || range.ty.is_array() {
                cx.error(range.span, "RFOR-7", format!("the range has type {}, which is not a range", range.ty.show()));
                return Stmt::new(SK::Null, n.span);
            }
            if range.ty.class().is_some() {
                return class_range(cx, n, cs, &labels, init, rd, range);
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
        // CPY-9: a loop variable of a class whose copy runs code.
        if !ty.is_ref() && el.class().is_some_and(|c| crate::classes::copy_move::check::needs_copy_function(cx, c)) {
            let probe = Expr::new(EK::Int(0), (*el).clone(), elem.cat, range.span);
            if crate::classes::copy_move::check::copy(cx, probe, true, n.span).is_none() {
                return Stmt::new(SK::Null, n.span);
            }
            return unsupported(cx, n, "a loop variable of a class whose copy constructor runs code");
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
        // JMP-6: a jump into the body skips the initialization of the
        // range and of the loop variable.
        if let Some(f) = cx.func.as_mut() {
            f.pending_labels = labels.clone();
            f.live.extend([range_id, var]);
        }
        let id = push_target(cx, true);
        let body = n.child("statement").map(|b| sub_statement(cx, b, Some((vs, "RFOR-6"))));
        pop_target(cx);
        if let Some(f) = cx.func.as_mut() {
            f.live.truncate(f.live.len() - 2);
        }
        let Some(body) = body else { return Stmt::new(SK::Null, n.span) };
        Stmt::new(SK::RangeArray { id, init, range: range_id, range_init, var, bound, body: Box::new(body) }, n.span)
    })();
    cx.scope = saved;
    r
}

/// The variable expression of the hidden variable `id`, an lvalue of
/// type `ty`.
fn hidden(id: crate::sem::EntityId, ty: &QType, span: crate::source::Span) -> Expr {
    Expr::new(EK::Var(id), ty.clone(), Cat::L, span)
}

/// A hidden local variable of the loop in scope `scope`.
fn add_hidden(cx: &mut Check, scope: crate::sem::ScopeId, ty: QType, span: crate::source::Span) -> crate::sem::EntityId {
    cx.u.add(crate::sem::Entity { name: "".into(), kind: EntKind::Var(local_var(ty, None)), span, scope, linkage: Linkage::None, deprecated: None })
}

/// `begin` or `end` of the range `r` of class type (RFOR-2): the member
/// form when the class has members of both names, else the free
/// functions that argument-dependent lookup finds.
fn begin_end(cx: &mut Check, r: &Expr, name: &str, members: bool, span: crate::source::Span) -> Option<Expr> {
    if members {
        return crate::expressions::postfix::check::call_member(cx, r.clone(), name, &[], span);
    }
    let cands = crate::foundations::names::check::argument_dependent(cx, name, std::slice::from_ref(&r.ty));
    if cands.is_empty() {
        cx.error(span, "RFOR-7", format!("argument-dependent lookup finds no function '{}' for the range of type {}", name, r.ty.show()));
        return None;
    }
    let f = crate::overloading::overload_resolution::check::resolve(cx, &cands, std::slice::from_ref(r), span, name)?;
    crate::declarations::functions::definitions::check::note_use(cx, f, span);
    let fty = QType::new(Type::Func(cx.u.func(f)?.ty.clone()));
    let callee = Expr::new(EK::Func(f), fty, Cat::L, span);
    Some(crate::expressions::postfix::check::finish_call(cx, callee, Some(f), vec![r.clone()], span, None))
}

/// A range of class type (RFOR-2 to RFOR-4): the loop as the plain `for`
/// that a hidden reference to the range, a hidden iterator, and a hidden
/// end make.
fn class_range(cx: &mut Check, n: &Node, cs: crate::sem::ScopeId, labels: &[std::rc::Rc<str>], init: Option<Box<Stmt>>, rd: &Node, range: Expr) -> Stmt {
    let span = n.span;
    let null = || Stmt::new(SK::Null, span);
    let Some(c) = range.ty.class() else { return null() };
    // RFOR-3: `auto&&` binds the range.
    let rv = range.cat != Cat::L;
    let range_ty = QType::new(Type::Ref(std::rc::Rc::new(range.ty.clone()), rv));
    let object_ty = range.ty.clone();
    let Some(range_init) = crate::declarations::initialization::references::check::bind(cx, range, &range_ty, "RFOR-1") else { return null() };
    let range_id = add_hidden(cx, cs, range_ty, span);
    let r = hidden(range_id, &object_ty, span);
    let has = |cx: &mut Check, name: &str| matches!(crate::classes::derived::check::member_lookup(cx, c, name), Ok(Some(_)));
    let members = has(cx, "begin") && has(cx, "end");
    let marks = cx.u.diags.len();
    let (Some(b), Some(e)) = (begin_end(cx, &r, "begin", members, span), begin_end(cx, &r, "end", members, span)) else {
        if cx.u.diags.len() == marks {
            cx.error(span, "RFOR-7", format!("the range of type {} has no begin and end", object_ty.show()));
        }
        return null();
    };
    if b.ty.is_error() || e.ty.is_error() {
        return null();
    }
    let iter_ty = |t: &QType| crate::declarations::auto::check::decay(t).unqual();
    let (bty, ety) = (iter_ty(&b.ty), iter_ty(&e.ty));
    let Some(binit) = crate::declarations::initialization::forms::check::copy_init(cx, b, &bty, "RFOR-1") else { return null() };
    let Some(einit) = crate::declarations::initialization::forms::check::copy_init(cx, e, &ety, "RFOR-1") else { return null() };
    let begin_id = add_hidden(cx, cs, bty.clone(), span);
    let end_id = add_hidden(cx, cs, ety.clone(), span);
    let (bv, ev) = (hidden(begin_id, &bty, span), hidden(end_id, &ety, span));
    // The loop compares by `!=`, advances by prefix `++`, and reads by
    // unary `*`.
    let ne = match crate::classes::operators::check::binary(cx, "!=", &bv, &ev, span) {
        Some(x) => x,
        None => crate::expressions::comparison::check::operate(cx, crate::sem::tree::BOp::Ne, bv.clone(), ev.clone(), span),
    };
    let step = match crate::classes::operators::check::unary(cx, "++", &bv, false, span) {
        Some(x) => x,
        None => crate::expressions::unary::check::inc_dec(cx, bv.clone(), true, true, span, "RFOR-7"),
    };
    let elem = match crate::classes::operators::check::unary(cx, "*", &bv, false, span) {
        Some(x) => x,
        None => crate::expressions::unary::check::indirection(cx, bv.clone(), span),
    };
    if ne.ty.is_error() || step.ty.is_error() || elem.ty.is_error() {
        return null();
    }
    let cond = crate::foundations::conversions::check::contextual_bool(cx, ne, "RFOR-2");
    // The loop variable.
    let Some(ds) = rd.child("decl_specifiers") else {
        return unsupported(cx, n, "a structured binding as the loop variable");
    };
    let Some(dn) = rd.child("declarator") else {
        return unsupported(cx, n, "a structured binding as the loop variable");
    };
    let specs = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
    if specs.is_static || specs.thread_local || specs.is_extern {
        cx.error(ds.span, "RFOR-5", "the loop variable must not be static, thread_local, or extern");
        return null();
    }
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    let ty = match &d.placeholder {
        Some(p) => match crate::declarations::auto::check::deduce(cx, p, dn, &elem) {
            Some(base) => crate::declarations::declarators::check::with_base(cx, base, dn),
            None => return null(),
        },
        None => d.ty.clone(),
    };
    if ty.is_error() {
        return null();
    }
    let vinit = if ty.is_ref() {
        crate::declarations::initialization::references::check::bind(cx, elem, &ty, "RFOR-1").map(Init::Ref)
    } else {
        crate::declarations::initialization::forms::check::copy_init(cx, elem, &ty, "RFOR-1").map(Init::Expr)
    };
    let Some(vinit) = vinit else { return null() };
    let Some((name, vspan)) = d.name.clone() else { return null() };
    let vs = cx.enter(ScopeKind::Block);
    let var = names::declare_in(cx, vs, &name, EntKind::Var(local_var(ty, None)), vspan, Linkage::None);
    // JMP-6: a jump into the body skips the initializations.
    if let Some(f) = cx.func.as_mut() {
        f.pending_labels = labels.to_vec();
        f.live.extend([range_id, begin_id, end_id, var]);
    }
    let id = push_target(cx, true);
    let body = n.child("statement").map(|b| sub_statement(cx, b, Some((vs, "RFOR-6"))));
    pop_target(cx);
    if let Some(f) = cx.func.as_mut() {
        f.live.truncate(f.live.len() - 4);
    }
    let Some(body) = body else { return null() };
    let inner = Stmt::new(SK::Block(vec![Stmt::new(SK::Decl(vec![(var, vinit)]), span), body]), span);
    let lp = Stmt::new(SK::For { id, init: None, cond: Some(crate::sem::tree::Cond::Expr(cond)), step: Some(step), body: Box::new(inner) }, span);
    let mut parts: Vec<Stmt> = init.into_iter().map(|b| *b).collect();
    parts.push(Stmt::new(SK::Decl(vec![(range_id, Init::Ref(range_init))]), span));
    parts.push(Stmt::new(SK::Decl(vec![(begin_id, Init::Expr(binit)), (end_id, Init::Expr(einit))]), span));
    parts.push(lp);
    Stmt::new(SK::Block(parts), span)
}
