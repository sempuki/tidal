//! The run part of `expressions/postfix`: calls and postfix `++` and
//! `--`.

use crate::foundations::constant_evaluation::run as ce;
use crate::machine::{ub, unsup, Machine, Ret, Val, R};
use crate::sem::types::{QType, Type};
use crate::sem::EntityId;

/// Postfix `++` and `--` give the new value to store. PFX-18: a signed
/// result out of range, or a pointer that leaves its array.
pub fn increment(m: &mut Machine, v: Val, inc: bool, ty: &QType) -> R<Val> {
    let d = if inc { 1 } else { -1 };
    match (v, &ty.t) {
        (Val::Int(x), Type::Int(k)) if *k == crate::sem::types::IntK::Bool => Ok(Val::Int(if inc { 1 } else { x })),
        (Val::Int(x), Type::Int(k)) => {
            let r = x + d;
            if k.signed() && k.rank() >= 3 && !m.target.fits(*k, r) {
                return ub("PFX-18", format!("{} {} 1 does not fit in {}", x, if inc { "+" } else { "-" }, k.name()));
            }
            Ok(Val::Int(m.target.wrap(*k, r)))
        }
        (Val::Float(f), Type::Float(k)) => Ok(Val::Float(m.round_float(f + d as f64, *k))),
        (Val::Ext(x), _) => Ok(Val::Ext(x.add(crate::foundations::types::run::F80::from_int(d)))),
        (Val::Ptr(p), Type::Ptr(_)) => match crate::expressions::additive::run::moved(m, p, d) {
            Some(p) => Ok(Val::Ptr(p)),
            None => ub("PFX-18", "the pointer moves outside its array"),
        },
        (Val::Null, Type::Ptr(_)) => ub("PFX-18", "an increment of a null pointer"),
        _ => unsup("an increment of this value"),
    }
}

/// The function a callee value names. PFX-8: a null function pointer.
pub fn callee(_m: &mut Machine, v: Val, _ty: &QType) -> R<EntityId> {
    match v {
        Val::Fn(id) => Ok(id),
        Val::Null => ub("PFX-8", "a call through a null function pointer"),
        Val::Uninit => crate::declarations::initialization::forms::run::read_no_value("the function pointer"),
        _ => unsup("a call through this value"),
    }
}

/// The function can run here: in constant evaluation it must be
/// `constexpr` or `consteval` (CE-2).
pub fn check_callable(m: &mut Machine, id: EntityId) -> R<()> {
    if !m.constant {
        return Ok(());
    }
    let f = m.u.func(id);
    if !f.is_some_and(|f| f.constexpr || f.consteval) {
        return ce::not_constexpr(&m.u.entities[id].name);
    }
    Ok(())
}

/// A call of a function with no definition in the unit.
/// The declaration of the same function that has its body: `id` itself
/// when it has one. A function declared in a block is the function of
/// that name and type in the enclosing namespace (LNK-9), and a function
/// with C language linkage is the one of its name with C language
/// linkage in any namespace (LSP-4).
pub fn defining(m: &Machine, id: EntityId) -> EntityId {
    let Some(f) = m.u.func(id) else { return id };
    if f.body.is_some() {
        return id;
    }
    let ent = &m.u.entities[id];
    let home = m.u.enclosing_namespace(ent.scope);
    let found = m.u.entities.iter().enumerate().find(|(k, e)| {
        let Some(g) = m.u.func(*k) else { return false };
        g.body.is_some()
            && e.name == ent.name
            && g.ty.params == f.ty.params
            && g.ty.ret == f.ty.ret
            && ((f.c_linkage && g.c_linkage) || (e.scope == home && ent.scope != home))
    });
    found.map_or(id, |(k, _)| k)
}

pub fn no_definition(m: &mut Machine, id: EntityId) -> R<Ret> {
    let name = m.u.entities[id].name.to_string();
    if m.constant {
        return ce::not_defined(&name);
    }
    unsup(format!("a call of '{}', which has no definition in this unit", name))
}

/// The run nests calls deeper than the machine follows.
pub fn too_deep(m: &mut Machine) -> R<Ret> {
    if m.constant {
        return ce::too_deep();
    }
    unsup("the run nests calls too deeply")
}

/// PFX-4: a subscript outside the array.
pub fn subscript(m: &mut Machine, p: Val, i: Val) -> R<crate::machine::Ptr> {
    let Val::Int(i) = i else { return unsup("a subscript with this value") };
    match p {
        Val::Ptr(p) => {
            let base = if p.array { p.clone() } else { crate::machine::Ptr { path: [p.path.clone(), vec![0]].concat(), array: true, ..p.clone() } };
            let n = m.array_len(&base).unwrap_or(1) as i128;
            let k = *base.path.last().unwrap_or(&0) as i128 + i;
            if k < 0 || k >= n {
                return ub("PFX-4", format!("the subscript {} is outside the array of {} elements", i, n));
            }
            let mut q = base;
            *q.path.last_mut().unwrap() = k as i64;
            Ok(q)
        }
        Val::Null => ub("PFX-4", "a subscript of a null pointer"),
        Val::Uninit => crate::declarations::initialization::forms::run::read_no_value("the pointer"),
        _ => unsup("a subscript of this value"),
    }
}
