//! The check part of `classes/copy-move`, for build step 4 part a: the
//! copy and move constructors, user-declared or implicit, that a copy of
//! a class object uses (CPY-1 to CPY-11). Assignment operators are
//! operator functions, which a later piece models; the implicit copy
//! assignment stays a copy of the object's bytes.

use crate::classes::constructors::check::{ctors, has_ctors};
use crate::sem::check::Check;
use crate::sem::tree::{Arg, Cat, Expr, Init, EK};
use crate::sem::types::{ClassId, QType, Type};
use crate::sem::EntityId;
use crate::source::Span;

/// A copy or move constructor of `c` (CPY-1, CPY-2): its first and only
/// parameter is a reference to the class. True for a move constructor in
/// the second place.
pub fn copy_or_move(cx: &Check, c: ClassId, f: EntityId) -> Option<bool> {
    let func = cx.u.func(f)?;
    if func.ty.params.len() != 1 {
        return None;
    }
    match &func.ty.params[0].t {
        Type::Ref(t, rv) if t.class() == Some(c) => Some(*rv),
        _ => None,
    }
}

/// A copy or move assignment operator of `c` (CPY-3): a member
/// `operator=` with one parameter of the class or a reference to it.
/// True for a move assignment.
pub fn assignment_kind(cx: &Check, c: ClassId, f: EntityId) -> Option<bool> {
    let func = cx.u.func(f)?;
    if func.ty.params.len() != 1 || func.class != Some(c) {
        return None;
    }
    match &func.ty.params[0].t {
        Type::Ref(t, rv) if t.class() == Some(c) => Some(*rv),
        Type::Class(d) if *d == c => Some(false),
        _ => None,
    }
}

/// CPY-5: the class has an implicit move assignment.
pub fn implicit_move_assign(cx: &Check, c: ClassId) -> bool {
    user(cx, c).is_empty() && user_assignments(cx, c).iter().all(|(f, _)| cx.u.defaulted.contains(f)) && !has_dtor(cx, c)
}

/// The user-declared copy and move assignment operators of a class.
fn user_assignments(cx: &Check, c: ClassId) -> Vec<(EntityId, bool)> {
    crate::foundations::names::check::declared_here(cx, cx.u.classes[c].scope, "operator=").into_iter().filter_map(|f| assignment_kind(cx, c, f).map(|m| (f, m))).collect()
}

/// The user-declared copy and move constructors of a class.
fn user(cx: &Check, c: ClassId) -> Vec<(EntityId, bool)> {
    ctors(cx, c).into_iter().filter_map(|f| copy_or_move(cx, c, f).map(|m| (f, m))).collect()
}

/// The class, or a member of it at any depth, has a user-declared copy
/// or move constructor, so that a copy of its object runs code.
pub fn needs_copy_function(cx: &Check, c: ClassId) -> bool {
    !user(cx, c).is_empty() || !user_assignments(cx, c).is_empty() || cx.u.field_types(c).iter().any(|t| t.base_elem().class().is_some_and(|d| needs_copy_function(cx, d)))
}

/// A member at any depth whose copy or move constructor runs code, or
/// is deleted by a user-declared move assignment.
fn members_run_code(cx: &Check, c: ClassId) -> bool {
    cx.u.field_types(c).iter().any(|t| t.base_elem().class().is_some_and(|d| !user(cx, d).is_empty() || user_assignments(cx, d).iter().any(|&(_, m)| m) || members_run_code(cx, d)))
}

/// A user-declared destructor (CPY-5).
fn has_dtor(cx: &Check, c: ClassId) -> bool {
    !crate::foundations::names::check::declared_here(cx, cx.u.classes[c].scope, crate::classes::constructors::check::DTOR).is_empty()
}

/// The prvalue that copies or moves the object of the glvalue `e` of
/// class type (CPY-9): the user-declared constructor that overload
/// resolution chooses, or the implicit copy or move, which copies each
/// member (CPY-8). `copy_init` leaves out the explicit constructors
/// (CPY-24). None after an error.
pub fn copy(cx: &mut Check, e: Expr, copy_init: bool, span: Span) -> Option<Expr> {
    let c = e.ty.class()?;
    let ty = e.ty.unqual();
    let users = user(cx, c);
    let rvalue = e.cat != Cat::L;
    if users.is_empty() {
        // CPY-4: a user-declared move assignment deletes the implicit
        // copy constructor.
        if !rvalue && user_assignments(cx, c).iter().any(|&(_, m)| m) {
            cx.error_cause(span, "CPY-21", Some("CPY-24"), format!("the copy constructor of '{}' is deleted, because the class declares a move assignment operator", cx.u.classes[c].name));
            return None;
        }
        // CPY-13: an implicit copy in a class with a user-provided copy
        // assignment is deprecated.
        if user_assignments(cx, c).iter().any(|&(f, m)| !m && !cx.u.defaulted.contains(&f)) && cx.wflag(&["-Wextra", "-Wdeprecated-copy"]) {
            cx.warning(span, "CPY-13", format!("the implicit copy constructor of '{}' is deprecated, because the class has a user-provided copy assignment operator", cx.u.classes[c].name));
        }
        return implicit(cx, e, c, span);
    }
    let has_move = users.iter().any(|&(_, m)| m) || user_assignments(cx, c).iter().any(|&(_, m)| m);
    let has_copy = users.iter().any(|&(_, m)| !m);
    let mut cands: Vec<EntityId> = users.iter().map(|&(f, _)| f).collect();
    if copy_init {
        cands.retain(|&f| !cx.u.func(f).is_some_and(|x| x.explicit));
    }
    // CPY-7: a defaulted move constructor that a member's deleted move
    // deletes takes no part in overload resolution.
    if rvalue && move_deleted(cx, c) {
        cands.retain(|&f| !(cx.u.defaulted.contains(&f) && copy_or_move(cx, c, f) == Some(true)));
    }
    let viable: Vec<EntityId> = cands.iter().copied().filter(|&f| {
        let p = cx.u.func(f).unwrap().ty.params[0].clone();
        crate::overloading::conversion_sequences::check::sequence(cx, &e, &p).is_some()
    }).collect();
    let name = cx.u.classes[c].name.to_string();
    let f = match viable.len() {
        0 => {
            // CPY-4: a user-declared move constructor deletes the implicit
            // copy constructor; CPY-20: a copy constructor that takes a
            // non-const reference does not copy a const object.
            if e.ty.v {
                cx.error_cause(span, "VOL-11", Some("CPY-24"), format!("no copy constructor of '{}' copies a volatile object", name));
            } else if !rvalue && has_move && !has_copy {
                cx.error_cause(span, "CPY-21", Some("CPY-24"), format!("the copy constructor of '{}' is deleted, because the class declares a move constructor", name));
            } else if e.ty.c && has_copy {
                cx.error_cause(span, "CPY-20", Some("CPY-24"), format!("no copy constructor of '{}' copies a const object", name));
            } else {
                cx.error(span, "CPY-24", format!("no usable copy constructor of '{}'", name));
            }
            return None;
        }
        1 => viable[0],
        _ => crate::overloading::overload_resolution::check::resolve(cx, &viable, std::slice::from_ref(&e), span, &name)?,
    };
    let func = cx.u.func(f).unwrap().clone();
    // CPY-26: a defaulted copy or move constructor that CPY-14 deletes.
    if func.deleted && cx.u.defaulted.contains(&f) {
        cx.error(span, "CPY-26", format!("the defaulted constructor of '{}' is deleted, because its parameter differs from the implicit one", name));
        return None;
    }
    if func.deleted {
        let cause = if copy_or_move(cx, c, f) == Some(true) { "CPY-23" } else { "CPY-24" };
        cx.error_cause(span, "DEL-1", Some(cause), format!("the {} constructor of '{}' is deleted", if cause == "CPY-23" { "move" } else { "copy" }, name));
        return None;
    }
    if !crate::classes::members::check::accessible(cx, f) {
        cx.error(span, "ACC-2", format!("the constructor of '{}' is not public", name));
        return None;
    }
    // A defaulted copy or move constructor is the implicit one.
    if cx.u.defaulted.contains(&f) {
        return implicit(cx, e, c, span);
    }
    crate::declarations::functions::definitions::check::note_use(cx, f, span);
    Some(Expr::new(EK::List(Box::new(Init::Ctor(f, vec![Arg { e, by_ref: true }]))), ty, Cat::PR, span))
}

/// CPY-6: the implicit copy constructor takes a reference to const when
/// each member can be copied from a const object.
pub fn copies_const(cx: &Check, c: ClassId) -> bool {
    let users = user(cx, c);
    if users.iter().any(|&(_, m)| !m) {
        return users.iter().any(|&(f, m)| !m && cx.u.func(f).is_some_and(|x| x.ty.params[0].nonref().c));
    }
    cx.u.field_types(c).iter().all(|t| t.base_elem().class().is_none_or(|d| copies_const(cx, d)))
}

/// CPY-14: a copy or move constructor defaulted in its class whose
/// parameter differs from the implicit one's is deleted.
pub fn defaulted_in_class(cx: &mut Check, c: ClassId, f: EntityId) {
    let Some(mv) = copy_or_move(cx, c, f) else { return };
    let p = cx.u.func(f).unwrap().ty.params[0].nonref().clone();
    let members_const = cx.u.field_types(c).iter().all(|t| t.base_elem().class().is_none_or(|d| copies_const(cx, d)));
    let differs = if mv { p.c || p.v } else { p.v || (p.c && !members_const) };
    if differs {
        if let Some(x) = cx.u.func_mut(f) {
            x.deleted = true;
        }
    }
}

/// CPY-17: a copy constructor defaulted after its class must not be one
/// that a member's deleted copy deletes (CPY-7).
pub fn defaulted_out_of_class(cx: &mut Check, c: ClassId, f: EntityId, span: Span) {
    if copy_or_move(cx, c, f) != Some(false) {
        return;
    }
    let deleted = |cx: &Check, d: ClassId| {
        let users = user(cx, d);
        !users.is_empty() && users.iter().all(|&(g, m)| m || cx.u.func(g).is_some_and(|x| x.deleted))
    };
    if cx.u.field_types(c).iter().any(|t| t.base_elem().class().is_some_and(|d| deleted(cx, d))) {
        cx.error(span, "CPY-17", format!("the copy constructor of '{}' defaulted after its class would be deleted", cx.u.classes[c].name));
    }
}

/// A member at any depth whose move constructor is deleted.
fn move_deleted(cx: &Check, c: ClassId) -> bool {
    cx.u.field_types(c).iter().any(|t| {
        t.base_elem().class().is_some_and(|d| user(cx, d).iter().any(|&(f, m)| m && cx.u.func(f).is_some_and(|x| x.deleted)) || move_deleted(cx, d))
    })
}

/// The implicit copy or move constructor (CPY-4, CPY-5, CPY-8): a copy
/// of the object's bytes when no member runs code, otherwise each member
/// copied or moved in turn.
fn implicit(cx: &mut Check, e: Expr, c: ClassId, span: Span) -> Option<Expr> {
    let ty = e.ty.unqual();
    if !members_run_code(cx, c) {
        return Some(e.conv(crate::sem::tree::Conv::LToR, ty, Cat::PR));
    }
    // The object is named once for each member.
    if e.any(&|x| matches!(x.k, EK::Call(..) | EK::Assign(..) | EK::Compound(..) | EK::IncDec(..))) {
        cx.unsupported(span, "a memberwise copy of an object that an expression with side effects names");
        return None;
    }
    // CPY-5: without an implicit move constructor, an rvalue is copied;
    // CPY-7: so is one whose move a member's deleted move deletes.
    let mut e = e;
    if e.cat != Cat::L && (has_dtor(cx, c) || move_deleted(cx, c) || !user_assignments(cx, c).is_empty()) {
        e.cat = Cat::L;
    }
    let fields = cx.u.classes[c].fields.clone();
    let mut inits = Vec::new();
    for f in fields {
        let m = crate::classes::members::check::field_of(cx, e.clone(), f, span);
        inits.push(member_copy(cx, m, span)?);
    }
    Some(Expr::new(EK::List(Box::new(Init::List(inits))), ty, Cat::PR, span))
}

/// The copy of one member: an element at a time for an array.
fn member_copy(cx: &mut Check, m: Expr, span: Span) -> Option<Init> {
    match &m.ty.t {
        Type::Array(el, Some(n)) => {
            let el = (**el).clone();
            let mut out = Vec::new();
            for k in 0..*n {
                let size_t = QType::new(Type::Int(cx.u.target.size_t()));
                let p = crate::foundations::conversions::check::decay(cx, m.clone());
                let x = Expr::new(EK::Index(Box::new(p), Box::new(Expr::int(k as i128, size_t, span))), el.clone(), m.cat, span);
                out.push(member_copy(cx, x, span)?);
            }
            Some(Init::List(out))
        }
        Type::Class(_) => copy(cx, m, false, span).map(Init::Expr),
        _ => Some(Init::Expr(crate::foundations::values::check::rvalue(cx, m))),
    }
}

/// CPY-4: an assignment to an object of a class whose user-declared move
/// constructor deletes the implicit copy assignment. True when the
/// assignment is valid.
pub fn assignable(cx: &mut Check, c: ClassId, span: Span) -> bool {
    if user(cx, c).iter().any(|&(_, m)| m) && user_assignments(cx, c).is_empty() {
        cx.error(span, "CPY-21", format!("the copy assignment of '{}' is deleted, because the class declares a move constructor", cx.u.classes[c].name));
        return false;
    }
    let _ = has_ctors;
    true
}
