//! The check part of `classes/comparisons`: defaulted comparison
//! operators (DCMP-1 to DCMP-8). A defaulted comparison gets a body made
//! of the comparisons of the bases and members, which the machine runs
//! as any other body. A comparison whose body cannot be made is
//! deleted (DCMP-2, DCMP-5).

use crate::sem::check::Check;
use crate::sem::tree::{BOp, Cat, Cond, Expr, Init, Stmt, EK, SK, UOp};
use crate::sem::types::{ClassId, QType, Type};
use crate::sem::{EntKind, EntityId, Linkage, ScopeKind, Storage, Var};
use crate::source::Span;
use std::rc::Rc;

/// The name of an operator that a class can default.
pub fn is_comparison(name: &str) -> bool {
    matches!(name, "operator==" | "operator!=" | "operator<" | "operator>" | "operator<=" | "operator>=" | "operator<=>")
}

/// The defaulted comparison `f`: DCMP-3 on its form, then its body once
/// its class is complete. `late` marks a comparison defaulted after its
/// class, which must not be one that would be deleted (DCMP-8).
pub fn defaulted(cx: &mut Check, f: EntityId, late: bool, span: Span) {
    let Some(func) = cx.u.func(f).cloned() else { return };
    if func.body.is_some() || func.deleted {
        return;
    }
    cx.u.defaulted.insert(f);
    let name = cx.u.entities[f].name.clone();
    let member = func.class.is_some() && !func.member_static;
    let params = func.ty.params.clone();
    // The class: the member's, or that of a friend's parameters.
    let Some(c) = func.class.or_else(|| params.first().and_then(|p| p.nonref().class())) else {
        cx.error(span, "DCMP-3", format!("the defaulted '{}' has no parameter of class type", name));
        return;
    };
    if !cx.u.classes[c].complete {
        return;
    }
    let cref = QType::lref(QType::new(Type::Class(c)).with_cv(true, false));
    let cval = QType::new(Type::Class(c));
    let form_ok = if member {
        params.len() == 1 && params[0] == cref && func.obj_cv.0 && !func.obj_cv.1 && func.ref_q != Some(true)
    } else {
        // A friend first declared outside the class is not one (DCMP-3).
        let friend = cx.u.friends.get(&c).is_some_and(|fs| fs.contains(&f)) && cx.u.hidden.contains(&f);
        params.len() == 2 && (params.iter().all(|p| *p == cref) || params.iter().all(|p| p.unqual() == cval)) && (friend || late)
    };
    if !form_ok {
        cx.error(span, "DCMP-3", format!("the defaulted '{}' is not a const member with a parameter of type const {}&, or a friend with two such parameters", name, cx.u.classes[c].name));
        return;
    }
    let three = &*name == "operator<=>";
    // LSUP-2, DCMP-10: a defaulted `<=>` needs std::strong_ordering.
    if three && crate::expressions::comparison::check::ordering_class(cx, "strong_ordering").is_none() {
        cx.error_cause(span, "DCMP-10", Some("LSUP-2"), "a defaulted three-way comparison needs a declaration of std::strong_ordering");
        return;
    }
    if !three && !func.ty.ret.is_bool() {
        cx.error(span, "DCMP-3", format!("the defaulted '{}' does not return bool", name));
        return;
    }
    // A defaulted `<=>` declares a defaulted `==` too, if the class
    // declares none (DCMP-4).
    if three && crate::foundations::names::check::declared_here(cx, cx.u.classes[c].scope, "operator==").is_empty() {
        implicit_eq(cx, f, c, span);
    }
    synthesize(cx, f, c, late, span);
}

/// The `==` that a defaulted `<=>` declares (DCMP-4): a member, or a
/// friend, of the same form, that returns `bool`.
fn implicit_eq(cx: &mut Check, three: EntityId, c: ClassId, span: Span) {
    let mut func = cx.u.func(three).unwrap().clone();
    let mut ty = (*func.ty).clone();
    ty.ret = QType::bool();
    func.ty = Rc::new(ty);
    func.body = None;
    func.params = Vec::new();
    func.placeholder = false;
    func.pending_return = false;
    func.unsupported = None;
    let scope = cx.u.entities[three].scope;
    let id = crate::foundations::names::check::declare_in(cx, scope, "operator==", EntKind::Func(func), span, cx.u.entities[three].linkage);
    if let Some(fs) = cx.u.friends.get_mut(&c) {
        if fs.contains(&three) {
            fs.push(id);
        }
    }
    if cx.u.hidden.contains(&three) {
        cx.u.hidden.insert(id);
    }
    if let Some(&a) = cx.u.access.get(&three) {
        cx.u.access.insert(id, a);
    }
    cx.u.defaulted.insert(id);
    synthesize(cx, id, c, false, span);
}

/// Make the body of the defaulted comparison `f` of class `c`.
fn synthesize(cx: &mut Check, f: EntityId, c: ClassId, late: bool, span: Span) {
    let func = cx.u.func(f).unwrap().clone();
    let name = cx.u.entities[f].name.clone();
    let saved_scope = cx.scope;
    cx.scope = if func.class.is_some() { cx.u.classes[c].scope } else { cx.u.entities[f].scope };
    let fs = cx.enter(ScopeKind::Function);
    let mut pids = Vec::new();
    for t in func.ty.params.iter() {
        let v = Var { ty: t.clone(), storage: Storage::Automatic, constexpr: false, constinit: false, inline: false, is_extern: false, is_static: false, param: true, defined: true, init: None, const_init: false, used: true, maybe_unused: true, align: None };
        pids.push(crate::foundations::names::check::declare_in(cx, fs, "", EntKind::Var(v), span, Linkage::None));
    }
    let var = |cx: &Check, id: EntityId| {
        let t = cx.u.var(id).unwrap().ty.clone();
        crate::foundations::values::check::deref_ref(Expr::new(EK::Var(id), t, Cat::L, span))
    };
    let class = QType::new(Type::Class(c));
    let lhs = if func.class.is_some() && !func.member_static {
        let this = QType::ptr_to(class.with_cv(true, false));
        Expr::new(EK::Deref(Box::new(Expr::new(EK::This, this, Cat::PR, span))), class.with_cv(true, false), Cat::L, span)
    } else {
        var(cx, pids[0])
    };
    let rhs = var(cx, *pids.last().unwrap());
    let saved_func = cx.func.replace(crate::sem::check::FnCtx { id: Some(f), this: func.class.map(|_| QType::ptr_to(class.with_cv(true, false))), constexpr: true, ..Default::default() });
    let marks = cx.u.diags.len();
    let body = match &name["operator".len()..] {
        "==" => eq_body(cx, c, lhs, rhs, span),
        "<=>" => three_body(cx, f, c, fs, lhs, rhs, span),
        "!=" => crate::classes::operators::check::binary(cx, "==", &lhs, &rhs, span).map(|e| {
            let b = crate::foundations::conversions::check::contextual_bool(cx, e, "DCMP-4");
            ret(Expr::new(EK::Unary(UOp::Not, Box::new(b)), QType::bool(), Cat::PR, span), span)
        }),
        op => crate::classes::operators::check::binary(cx, "<=>", &lhs, &rhs, span).map(|v| {
            let e = crate::classes::operators::check::compare_zero(cx, op, v, false, span);
            let b = crate::foundations::conversions::check::contextual_bool(cx, e, "DCMP-4");
            ret(b, span)
        }),
    };
    cx.func = saved_func;
    cx.scope = saved_scope;
    let new = &cx.u.diags[marks..];
    let unsupported = new.iter().any(|d| d.sev == crate::diag::Severity::Unsupported);
    let failed = body.is_none() || new.iter().any(|d| d.sev == crate::diag::Severity::Error) || body.as_ref().is_some_and(|b| has_error(b));
    if unsupported {
        if let Some(x) = cx.u.func_mut(f) {
            x.unsupported = Some("a defaulted comparison whose parts the front end does not model".into());
        }
        return;
    }
    if failed {
        cx.u.diags.truncate(marks);
        // DCMP-8: a comparison defaulted after its class is not deleted.
        if late {
            cx.error(span, "DCMP-8", format!("the '{}' defaulted after its class would be deleted", name));
        }
        if let Some(x) = cx.u.func_mut(f) {
            x.deleted = true;
            x.unsupported = None;
            x.pending_return = false;
        }
        return;
    }
    // EXS-4: the comparison throws only what its parts' comparisons throw.
    let body = body.unwrap();
    let throws = exprs(&body).iter().any(|e| crate::declarations::exception_specifications::check::can_throw(cx, e));
    if let Some(x) = cx.u.func_mut(f) {
        let mut t = (*x.ty).clone();
        t.noexcept = !throws;
        x.ty = Rc::new(t);
        x.body = Some(Rc::new(body));
        x.params = pids;
        x.defined = true;
        x.inline = true;
        x.constexpr = true;
        x.unsupported = None;
        x.pending_return = false;
    }
}

/// The expressions of a body that `synthesize` made.
fn exprs(s: &Stmt) -> Vec<Expr> {
    match &s.k {
        SK::Block(xs) => xs.iter().flat_map(exprs).collect(),
        SK::Return(Some(e)) | SK::Expr(e) => vec![e.clone()],
        SK::Decl(vs) => vs.iter().filter_map(|(_, i)| match i {
            Init::Expr(e) => Some(e.clone()),
            _ => None,
        }).collect(),
        SK::If { cond: Cond::Expr(e), then, .. } => {
            let mut v = vec![e.clone()];
            v.extend(exprs(then));
            v
        }
        _ => Vec::new(),
    }
}

fn ret(e: Expr, span: Span) -> Stmt {
    Stmt::new(SK::Block(vec![Stmt::new(SK::Return(Some(e)), span)]), span)
}

/// A statement holds an expression of the error type.
fn has_error(s: &Stmt) -> bool {
    match &s.k {
        SK::Block(xs) => xs.iter().any(has_error),
        SK::Return(Some(e)) | SK::Expr(e) => e.any(&|x| x.ty.is_error()),
        SK::Decl(vs) => vs.iter().any(|(_, i)| i.any(&|x| x.ty.is_error())),
        SK::If { cond: Cond::Expr(e), then, .. } => e.any(&|x| x.ty.is_error()) || has_error(then),
        _ => false,
    }
}

/// The parts of two objects of class `c` that a defaulted comparison
/// compares, in order: the direct bases, then the data members, an array
/// element by element (DCMP-1). None when DCMP-2 deletes the comparison:
/// a reference member, a union, or an anonymous union member.
fn parts(cx: &mut Check, c: ClassId, l: &Expr, r: &Expr, span: Span) -> Option<Vec<(Expr, Expr)>> {
    if cx.u.classes[c].key == crate::sem::ClassKey::Union {
        return None;
    }
    let mut out = Vec::new();
    for f in cx.u.classes[c].fields.clone() {
        let x = cx.u.field(f)?.clone();
        if x.ty.is_ref() || x.ty.class().is_some_and(|k| cx.u.classes[k].key == crate::sem::ClassKey::Union && cx.u.classes[k].name.is_empty()) {
            return None;
        }
        if !x.base && cx.u.entities[f].name.is_empty() {
            continue;
        }
        let a = crate::classes::members::check::field_of(cx, l.clone(), f, span);
        let b = crate::classes::members::check::field_of(cx, r.clone(), f, span);
        elements(cx, a, b, span, &mut out);
    }
    Some(out)
}

/// An array part compares element by element.
fn elements(cx: &mut Check, a: Expr, b: Expr, span: Span, out: &mut Vec<(Expr, Expr)>) {
    let Type::Array(el, Some(n)) = a.ty.t.clone() else {
        out.push((a, b));
        return;
    };
    let size_t = QType::new(Type::Int(cx.u.target.size_t()));
    for k in 0..n {
        let at = |cx: &mut Check, x: &Expr| {
            let p = crate::foundations::conversions::check::decay(cx, x.clone());
            Expr::new(EK::Index(Box::new(p), Box::new(Expr::int(k as i128, size_t.clone(), span))), (*el).clone().with_cv(x.ty.c, x.ty.v), Cat::L, span)
        };
        let (x, y) = (at(cx, &a), at(cx, &b));
        elements(cx, x, y, span, out);
    }
}

/// `a == b` of two parts.
fn part_eq(cx: &mut Check, a: Expr, b: Expr, span: Span) -> Expr {
    if let Some(e) = crate::classes::operators::check::binary(cx, "==", &a, &b, span) {
        return e;
    }
    crate::expressions::comparison::check::operate(cx, BOp::Eq, a, b, span)
}

/// `a <=> b` of two parts.
fn part_three(cx: &mut Check, a: Expr, b: Expr, span: Span) -> Expr {
    if let Some(e) = crate::classes::operators::check::binary(cx, "<=>", &a, &b, span) {
        return e;
    }
    crate::expressions::comparison::check::three_way(cx, a, b, span)
}

/// DCMP-1: the parts compare equal, the first that does not ending it.
fn eq_body(cx: &mut Check, c: ClassId, l: Expr, r: Expr, span: Span) -> Option<Stmt> {
    let mut e: Option<Expr> = None;
    for (a, b) in parts(cx, c, &l, &r, span)? {
        let x = part_eq(cx, a, b, span);
        let x = crate::foundations::conversions::check::contextual_bool(cx, x, "DCMP-2");
        e = Some(match e {
            None => x,
            Some(p) => Expr::new(EK::Logical(true, Box::new(p), Box::new(x)), QType::bool(), Cat::PR, span),
        });
    }
    Some(ret(e.unwrap_or_else(|| Expr::new(EK::Int(1), QType::bool(), Cat::PR, span)), span))
}

/// The rank of an ordering class, weakest first (DCMP-4).
fn rank(cx: &Check, t: &QType) -> Option<u8> {
    let c = t.class()?;
    let std = cx.u.scopes[cx.u.classes[c].scope].parent.filter(|&s| cx.u.scopes[s].name.as_ref() == "std")?;
    let _ = std;
    match &*cx.u.classes[c].name {
        "partial_ordering" => Some(0),
        "weak_ordering" => Some(1),
        "strong_ordering" => Some(2),
        _ => None,
    }
}

/// DCMP-4: the first result of the parts that is not equal, or equal.
fn three_body(cx: &mut Check, f: EntityId, c: ClassId, fs: crate::sem::ScopeId, l: Expr, r: Expr, span: Span) -> Option<Stmt> {
    let results: Vec<Expr> = parts(cx, c, &l, &r, span)?.into_iter().map(|(a, b)| part_three(cx, a, b, span)).collect();
    // LSUP-2: the result when every part is equal is the `equal` member of
    // std::strong_ordering.
    let strong = crate::expressions::comparison::check::ordering_class(cx, "strong_ordering")?;
    let equal = crate::expressions::comparison::check::ordering_members(cx, strong, false)?[1];
    let func = cx.u.func(f).unwrap().clone();
    let r_ty = if func.placeholder {
        // The weakest category of the parts; each part's result is one.
        let mut t = QType::new(Type::Class(strong));
        for x in &results {
            let k = rank(cx, &x.ty)?;
            if k < rank(cx, &t)? {
                t = x.ty.unqual();
            }
        }
        t
    } else {
        rank(cx, &func.ty.ret)?;
        func.ty.ret.clone()
    };
    let mut items = Vec::new();
    for x in results {
        let t = x.ty.unqual();
        let v = Var { ty: t.clone(), storage: Storage::Automatic, constexpr: false, constinit: false, inline: false, is_extern: false, is_static: false, param: false, defined: true, init: None, const_init: false, used: true, maybe_unused: true, align: None };
        let id = crate::foundations::names::check::declare_in(cx, fs, "", EntKind::Var(v), span, Linkage::None);
        items.push(Stmt::new(SK::Decl(vec![(id, Init::Expr(x))]), span));
        let ve = Expr::new(EK::Var(id), t.clone(), Cat::L, span);
        let ne = crate::classes::operators::check::binary(cx, "!=", &ve, &Expr::int(0, QType::int(), span), span)?;
        let cond = crate::foundations::conversions::check::contextual_bool(cx, ne, "LSUP-12");
        let out = crate::declarations::initialization::forms::check::copy_init(cx, ve, &r_ty, "LSUP-13")?;
        items.push(Stmt::new(SK::If { init: None, cond: Cond::Expr(cond), then: Box::new(Stmt::new(SK::Return(Some(out)), span)), els: None }, span));
    }
    let eq = Expr::new(EK::Var(equal), cx.u.var(equal)?.ty.clone(), Cat::L, span);
    let out = crate::declarations::initialization::forms::check::copy_init(cx, eq, &r_ty, "LSUP-13")?;
    items.push(Stmt::new(SK::Return(Some(out)), span));
    if func.placeholder {
        if let Some(x) = cx.u.func_mut(f) {
            let mut t = (*x.ty).clone();
            t.ret = r_ty;
            x.ty = Rc::new(t);
        }
    }
    Some(Stmt::new(SK::Block(items), span))
}
