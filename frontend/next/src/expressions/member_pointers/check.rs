//! The check part of `expressions/member-pointers`: `&C::m` forms a
//! pointer to member (MPTR-2, MPTR-9), `.*` and `->*` apply one to an
//! object (MPTR-3, MPTR-10), and pointers to members convert (MPTR-4,
//! MPTR-11) and compare (MPTR-5, MPTR-12).

use crate::classes::derived::check as derived;
use crate::expressions::check::{bad, expr};
use crate::foundations::conversions::check as conv;
use crate::foundations::names::check::{self as names, Qual};
use crate::foundations::values::check::{deref_ref, rvalue};
use crate::sem::check::Check;
use crate::sem::tree::{BOp, Cat, Conv, Expr, MemberRef, EK};
use crate::sem::types::{ClassId, FnType, QType, Type};
use crate::sem::{EntKind, EntityId};
use crate::source::Span;
use crate::syntax::{Kid, Node};
use std::rc::Rc;

/// The type of a pointer to the member function `f` of class `c`: the
/// function's type with the qualifiers of its object.
pub fn func_type(cx: &Check, c: ClassId, f: EntityId) -> QType {
    let func = cx.u.func(f).unwrap();
    let ft = FnType { obj_cv: func.obj_cv, ref_q: func.ref_q, ..(*func.ty).clone() };
    QType::new(Type::MemPtr(c, Rc::new(QType::new(Type::Func(Rc::new(ft))))))
}

/// A pointer to the member function `f`.
pub fn func_ptr(cx: &mut Check, f: EntityId, span: Span) -> Expr {
    let c = cx.u.func(f).and_then(|x| x.class).unwrap();
    crate::declarations::functions::definitions::check::note_use(cx, f, span);
    let t = func_type(cx, c, f);
    Expr::new(EK::MemberPtr(c, MemberRef::Func(f)), t, Cat::PR, span)
}

/// MPTR-2, MPTR-9: `&C::m` with no parentheses, where `C` is a class and
/// `m` a non-static member. None for any other operand of `&`.
pub fn address(cx: &mut Check, operand: &Node, span: Span) -> Option<Expr> {
    if operand.prod != "primary" || operand.kids.len() != 1 {
        return None;
    }
    let q = operand.child("id_expression")?.child("qualified_name")?;
    let nn = q.child("nested_name")?;
    let u = q.child("unqualified_id")?;
    let [k @ Kid::Tok(_)] = u.kids.as_slice() else { return None };
    let name = cx.text(k).to_string();
    let marks = cx.u.diags.len();
    let class = match names::nested_name(cx, nn) {
        Qual::Scope(s) => cx.u.class_of_scope(s),
        _ => None,
    };
    cx.u.diags.truncate(marks);
    let class = class?;
    if *cx.u.classes[class].name == *name {
        cx.error(span, "MPTR-9", "a pointer to member of a constructor");
        return Some(bad(span));
    }
    let found = derived::member_lookup(cx, class, &name).ok()??;
    let ents = found.ents.clone();
    let owner = found.class;
    match ents.as_slice() {
        [e] if matches!(cx.u.entities[*e].kind, EntKind::Field(_)) => {
            let EntKind::Field(x) = cx.u.entities[*e].kind.clone() else { unreachable!() };
            if x.width.is_some() {
                cx.error(span, "MPTR-9", "a pointer to member of a bit-field");
                return Some(bad(span));
            }
            if x.ty.is_ref() {
                cx.error(span, "MPTR-8", "a pointer to member of reference type");
                return Some(bad(span));
            }
            if !derived::accessible_through(cx, *e, class) {
                cx.error(span, "MPTR-9", format!("a pointer to the member '{}', which is not accessible", name));
                return Some(bad(span));
            }
            if !protected_ok(cx, *e, class, owner, span) {
                return Some(bad(span));
            }
            let t = QType::new(Type::MemPtr(owner, Rc::new(x.ty.clone())));
            Some(Expr::new(EK::MemberPtr(owner, MemberRef::Field(x.index)), t, Cat::PR, span))
        }
        _ => {
            let funcs: Vec<EntityId> = ents.iter().copied().filter(|&e| cx.u.func(e).is_some_and(|f| !f.member_static)).collect();
            if funcs.is_empty() || funcs.len() != ents.len() {
                return None;
            }
            if funcs.iter().any(|&f| !derived::accessible_through(cx, f, class)) {
                cx.error(span, "MPTR-9", format!("a pointer to the member '{}', which is not accessible", name));
                return Some(bad(span));
            }
            if !protected_ok(cx, funcs[0], class, owner, span) {
                return Some(bad(span));
            }
            if funcs.len() == 1 {
                return Some(func_ptr(cx, funcs[0], span));
            }
            // OVL-18: the target type chooses one function of the set.
            let set = Expr::new(EK::Func(funcs[0]), QType::error(), Cat::L, span);
            Some(Expr::new(EK::AddrOf(Box::new(set)), QType::error(), Cat::PR, span))
        }
    }
}

/// ACC-4: a pointer to a protected member that a class reaches as a
/// member of its base names the class itself, or a class derived from
/// it, as the qualifier. True when the qualifier `naming` is valid.
fn protected_ok(cx: &mut Check, member: EntityId, naming: ClassId, owner: ClassId, span: Span) -> bool {
    let access = match &cx.u.entities[member].kind {
        EntKind::Field(x) => x.access,
        _ => cx.u.access.get(&member).copied().unwrap_or(crate::sem::Access::Public),
    };
    let Some(h) = derived::here(cx) else { return true };
    if access != crate::sem::Access::Protected || h == owner || crate::classes::access::check::befriended(cx, owner) || derived::derives(cx, naming, h) {
        return true;
    }
    cx.error(span, "ACC-4", format!("a pointer to the protected member of '{}' names '{}', which is not '{}' or a class derived from it", cx.u.classes[owner].name, cx.u.classes[naming].name, cx.u.classes[h].name));
    false
}

/// The node of `o.*p` that a call's callee holds, inside parentheses.
pub fn called(callee: &Node) -> Option<&Node> {
    let mut n = callee;
    loop {
        if n.prod == "pm_expr" {
            return Some(n);
        }
        if n.prod != "primary" {
            return None;
        }
        n = match n.kids.as_slice() {
            [Kid::Tok(_), Kid::Node(inner), Kid::Tok(_)] => inner,
            _ => return None,
        };
    }
}

/// `pm_expr` (MPTR-3, MPTR-10). `call` marks the callee of a call, the
/// one place where a pointer to member function can be applied.
pub fn pm_expr(cx: &mut Check, n: &Node, call: bool) -> Expr {
    let span = n.span;
    let [Kid::Node(l), op, Kid::Node(r)] = n.kids.as_slice() else { return bad(span) };
    let arrow = cx.text(op) == "->*";
    let l = expr(cx, l);
    let r = expr(cx, r);
    if l.ty.is_error() || r.ty.is_error() {
        return bad(span);
    }
    // OPR-6: an operand of class type of `->*` can select an operator
    // function.
    if arrow {
        if let Some(e) = crate::classes::operators::check::binary(cx, "->*", &l, &r, span) {
            return e;
        }
    }
    let r = rvalue(cx, r);
    let Some((c, mt)) = r.ty.member_ptr().map(|(c, m)| (c, m.clone())) else {
        cx.error(span, "MPTR-10", format!("the right operand of '{}' has type {}, which is not a pointer to member", cx.text(op), r.ty.show()));
        return bad(span);
    };
    let obj = if arrow {
        let l = rvalue(cx, l);
        let Some(p) = l.ty.pointee().cloned() else {
            cx.error(span, "MPTR-10", format!("the left operand of '->*' has type {}, which is not a pointer", l.ty.show()));
            return bad(span);
        };
        Expr::new(EK::Deref(Box::new(l)), p, Cat::L, span)
    } else {
        deref_ref(l)
    };
    let Some(d) = obj.ty.class() else {
        cx.error(span, "MPTR-10", format!("the object of '{}' has type {}, which is not a class", cx.text(op), obj.ty.show()));
        return bad(span);
    };
    if d != c {
        let ps = derived::paths(cx, d, c);
        if ps.is_empty() {
            cx.error(span, "MPTR-10", format!("the object of type {} has no member of class {}", obj.ty.show(), cx.u.classes[c].name));
            return bad(span);
        }
        if ps.len() > 1 {
            cx.error(span, "DER-10", format!("the base '{}' occurs as more than one subobject of '{}'", cx.u.classes[c].name, cx.u.classes[d].name));
            return bad(span);
        }
    }
    let obj = if obj.cat == Cat::PR {
        let t = obj.ty.clone();
        obj.conv(Conv::Materialize, t, Cat::X)
    } else {
        obj
    };
    if let Some(f) = mt.func() {
        if !call {
            cx.error(span, "MPTR-10", "the result of a pointer to member function is not called");
            return bad(span);
        }
        if (obj.ty.c && !f.obj_cv.0) || (obj.ty.v && !f.obj_cv.1) {
            cx.error(span, "MPTR-10", "the call through a pointer to member function drops a cv-qualifier of the object");
            return bad(span);
        }
        let lvalue = obj.cat == Cat::L;
        match f.ref_q {
            Some(false) if !lvalue && !(f.obj_cv.0 && !f.obj_cv.1) => {
                cx.error(span, "MPTR-10", "a call on an rvalue through a pointer to member function qualified '&'");
                return bad(span);
            }
            Some(true) if lvalue => {
                cx.error(span, "MPTR-10", "a call on an lvalue through a pointer to member function qualified '&&'");
                return bad(span);
            }
            _ => {}
        }
        return Expr::new(EK::PtrMem(Box::new(obj), Box::new(r)), mt, Cat::PR, span);
    }
    let t = mt.with_cv(mt.c || obj.ty.c, mt.v || obj.ty.v);
    let cat = if obj.cat == Cat::L { Cat::L } else { Cat::X };
    Expr::new(EK::PtrMem(Box::new(obj), Box::new(r)), t, cat, span)
}

/// A call through a pointer to member function: the callee and its
/// object. None for any other callee.
pub fn split_call(f: Expr) -> Result<(Expr, Expr), Expr> {
    match f.k {
        EK::PtrMem(o, p) if f.ty.is_func() => Ok((Expr::new(EK::PtrMemFn(p), f.ty, Cat::L, f.span), *o)),
        k => Err(Expr { k, ..f }),
    }
}

/// MPTR-4, MPTR-11: the implicit conversion of `e`, a prvalue, to the
/// pointer to member type `to`, or to `bool`. None when neither type is
/// a pointer to member.
pub fn implicit(cx: &mut Check, e: Expr, null: bool, to: &QType) -> Option<Result<Expr, String>> {
    if !e.ty.is_member_ptr() && !to.is_member_ptr() {
        return None;
    }
    let e = if e.cat == Cat::PR { e } else { rvalue(cx, e) };
    let from = e.ty.clone();
    if from.is_member_ptr() && to.is_bool() {
        return Some(Ok(e.conv(Conv::ToBool, to.clone(), Cat::PR)));
    }
    let (tc, tm) = to.member_ptr()?;
    if null {
        return Some(Ok(e.conv(Conv::NullPtr, to.clone(), Cat::PR)));
    }
    let Some((fc, fm)) = from.member_ptr() else {
        return Some(Err(format!("no implicit conversion from {} to {} (MPTR-11)", from.show(), to.show())));
    };
    let no = || Some(Err(format!("no implicit conversion from {} to {} (MPTR-11)", from.show(), to.show())));
    if !member_types(fm, tm) {
        return no();
    }
    if fc != tc {
        // A pointer to member of a base converts to one of a derived
        // class, through one path to a base that is accessible.
        let ps = derived::paths(cx, tc, fc);
        if ps.len() != 1 || !derived::base_is_accessible(cx, tc, fc) || derived::through_virtual(&ps[0]) {
            return no();
        }
    }
    Some(Ok(e.conv(Conv::PtrNoOp, to.clone(), Cat::PR)))
}

/// The member type `f` converts to `t`: the same type, or a
/// qualification conversion (CV-10), where the pointer to member counts
/// as the outermost level.
fn member_types(f: &QType, t: &QType) -> bool {
    // A pointer to a noexcept member function converts to one without
    // noexcept (CONV-10).
    if f.is_func() || t.is_func() {
        return f.t == t.t || (f.is_func() && t.is_func() && conv::pointer_convertible(f, t));
    }
    conv::qual_level(f, t, true)
}

/// The `static_cast` or `reinterpret_cast` of `e`, a prvalue, to `to`
/// when one of them is a pointer to member (MPTR-4, MPTR-11). None when
/// neither is one.
pub fn cast(cx: &mut Check, reinterpret: bool, e: Expr, null: bool, to: &QType) -> Option<Result<Expr, (&'static str, String)>> {
    let from = e.ty.clone();
    if !from.is_member_ptr() && !to.is_member_ptr() {
        return None;
    }
    let err = |m: String| Some(Err(("MPTR-11", m)));
    if reinterpret {
        return match (from.member_ptr(), to.member_ptr()) {
            (Some((_, fm)), Some((_, tm))) if fm.is_func() == tm.is_func() => Some(Ok(e.conv(Conv::PtrNoOp, to.unqual(), Cat::PR))),
            _ => err(format!("no reinterpret_cast from {} to {}", from.show(), to.show())),
        };
    }
    if let (Some((fc, fm)), Some((tc, tm))) = (from.member_ptr(), to.member_ptr()) {
        // The reverse of a conversion from a base to a derived class.
        if fc != tc && member_types(fm, tm) && derived::is_base(cx, fc, tc) {
            let ps = derived::paths(cx, fc, tc);
            if ps.len() != 1 || !derived::base_is_accessible(cx, fc, tc) || derived::through_virtual(&ps[0]) {
                return err(format!("static_cast from {} to {} through a base that is ambiguous or not accessible", from.show(), to.show()));
            }
            return Some(Ok(e.conv(Conv::PtrNoOp, to.unqual(), Cat::PR)));
        }
    }
    Some(match implicit(cx, e, null, &to.unqual()) {
        Some(Ok(x)) => Ok(x),
        Some(Err(m)) => Err(("MPTR-11", m)),
        None => Err(("MPTR-11", format!("no static_cast from {} to {}", from.show(), to.show()))),
    })
}

/// MPTR-5, MPTR-12: `==` or `!=` with a pointer to member operand, after
/// the lvalue-to-rvalue conversion. The other operand converts to its
/// type.
pub fn compare(cx: &mut Check, op: BOp, l: Expr, r: Expr, nulls: (bool, bool), span: Span) -> Expr {
    if !matches!(op, BOp::Eq | BOp::Ne) {
        cx.error(span, "MPTR-12", "a relational comparison of pointers to members");
        return bad(span);
    }
    let mk = |l: Expr, r: Expr| Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), QType::bool(), Cat::PR, span);
    let null_of = |e: &Expr, n: bool| n || e.ty.is_nullptr();
    if l.ty.is_member_ptr() {
        let t = l.ty.unqual();
        let rn = null_of(&r, nulls.1);
        if let Some(Ok(r2)) = implicit(cx, r.clone(), rn, &t) {
            return mk(l, r2);
        }
    }
    if r.ty.is_member_ptr() {
        let t = r.ty.unqual();
        let ln = null_of(&l, nulls.0);
        if let Some(Ok(l2)) = implicit(cx, l.clone(), ln, &t) {
            return mk(l2, r);
        }
    }
    cx.error(span, "CMP-9", format!("invalid operands of types {} and {}", l.ty.show(), r.ty.show()));
    bad(span)
}
