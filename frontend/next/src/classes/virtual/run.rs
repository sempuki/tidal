//! The run part of `classes/virtual`: the final overrider of a virtual
//! call (VIRT-1, VIRT-6), the conversion of a covariant result (VIRT-4),
//! and the virtual call of a pure function (VIRT-12).

use crate::machine::{ub, unsup, Machine, Ptr, Ret, Val, R};
use crate::sem::types::{ClassId, Type};
use crate::sem::EntityId;

/// The final overrider of the virtual function `f` for the object of a
/// call, with the object converted to the subobject of the overrider's
/// class. The object's class is the class of its most derived object,
/// or, while a constructor or destructor of a class runs on a base
/// subobject, that class (VIRT-6).
pub fn dispatch(m: &Machine, f: EntityId, obj: Ret) -> R<(EntityId, Ret)> {
    let p = match &obj {
        Ret::Place(p) => p.clone(),
        Ret::Val(Val::Ptr(p)) => p.clone(),
        _ => return unsup("the object of a virtual call"),
    };
    if p.view.is_some() {
        return unsup("a virtual call on an object seen through a pointer of another type");
    }
    // The class at each prefix of the path, and where the most derived
    // object begins: after the last step that is not into a base.
    let mut ty = m.objs[p.obj].ty.clone();
    let mut classes: Vec<Option<ClassId>> = vec![ty.class()];
    let mut start = 0;
    for (k, &i) in p.path.iter().enumerate() {
        let base = ty.class().and_then(|c| m.u.classes[c].fields.get(i as usize).copied()).and_then(|x| m.u.field(x)).is_some_and(|x| x.base);
        ty = match &ty.t {
            Type::Class(c) => match m.u.field_types(*c).get(i as usize) {
                Some(t) => t.clone(),
                None => return unsup("the object of a virtual call"),
            },
            _ => match ty.elem() {
                Some(t) => t.clone(),
                None => return unsup("the object of a virtual call"),
            },
        };
        if !base {
            start = k + 1;
        }
        classes.push(ty.class());
    }
    let n = p.path.len();
    let from = m.structors.iter().filter(|(o, q)| *o == p.obj && q.len() >= start && q.len() <= n && p.path.starts_with(q)).map(|(_, q)| q.len()).max().unwrap_or(start);
    let name = &m.u.entities[f].name;
    for k in from..=n {
        let Some(c) = classes[k] else { continue };
        let scope = m.u.classes[c].scope;
        let found = m.u.scopes[scope].names.get(name).into_iter().flatten().copied().find(|&g| g == f || m.u.virt.get(&g).is_some_and(|v| v.overrides.contains(&f)));
        if let Some(g) = found {
            if m.u.virt.get(&g).is_some_and(|v| v.pure) {
                return ub("VIRT-12", format!("a virtual call of the pure virtual function '{}' of '{}'", name, m.u.classes[c].name));
            }
            let this = Ptr { obj: p.obj, path: p.path[..k].to_vec(), array: false, view: None };
            let obj = match obj {
                Ret::Place(_) => Ret::Place(this),
                _ => Ret::Val(Val::Ptr(this)),
            };
            return Ok((g, obj));
        }
    }
    Ok((f, obj))
}

/// VIRT-4: the result of the overrider `g` converted to the return type
/// of the function `f` that the call names, for a covariant return type.
pub fn covariant(m: &Machine, g: EntityId, f: EntityId, r: Ret) -> R<Ret> {
    let (Some(gf), Some(ff)) = (m.u.func(g), m.u.func(f)) else { return Ok(r) };
    let target = |t: &crate::sem::types::QType| match &t.t {
        Type::Ptr(x) | Type::Ref(x, _) => x.class(),
        _ => None,
    };
    let (Some(d), Some(b)) = (target(&gf.ty.ret), target(&ff.ty.ret)) else { return Ok(r) };
    if d == b {
        return Ok(r);
    }
    let Some(path) = m.u.base_paths(d, b).into_iter().next() else { return Ok(r) };
    let down = |mut q: Ptr| {
        q.path.extend(path.iter().map(|&i| i as i64));
        q.array = false;
        q
    };
    Ok(match r {
        Ret::Val(Val::Ptr(q)) if q.view.is_none() => Ret::Val(Val::Ptr(down(q))),
        Ret::Place(q) if q.view.is_none() => Ret::Place(down(q)),
        Ret::Val(Val::Null) => Ret::Val(Val::Null),
        _ => return unsup("a covariant result of this form"),
    })
}

/// The place of the object that `dynamic_cast` gives for the subobject
/// at `p` (VIRT-10), or None when the check fails. A target of None is
/// `void`: the most derived object. During a constructor or destructor,
/// the object whose constructor runs counts as the most derived one.
fn dynamic_target(m: &Machine, p: &Ptr, target: Option<ClassId>) -> R<Option<Ptr>> {
    if p.view.is_some() {
        return unsup("a dynamic_cast of an object seen through a pointer of another type");
    }
    // CTOR-12: the object's constructor has not initialized its bases.
    if m.early.iter().any(|(o, q)| *o == p.obj && p.path.starts_with(q)) {
        return ub("CTOR-12", "a dynamic_cast of an object before its bases are initialized");
    }
    // The class at each prefix of the path, whether each step goes into a
    // public base, and where the most derived object begins.
    let mut ty = m.objs[p.obj].ty.clone();
    let mut classes: Vec<Option<ClassId>> = vec![ty.class()];
    let mut public: Vec<bool> = Vec::new();
    let mut start = 0;
    for (k, &i) in p.path.iter().enumerate() {
        let field = ty.class().and_then(|c| m.u.classes[c].fields.get(i as usize).copied()).and_then(|x| m.u.field(x));
        let base = field.is_some_and(|x| x.base);
        public.push(field.is_some_and(|x| x.base && x.access == crate::sem::Access::Public));
        ty = match &ty.t {
            Type::Class(c) => match m.u.field_types(*c).get(i as usize) {
                Some(t) => t.clone(),
                None => return unsup("the operand of a dynamic_cast"),
            },
            _ => match ty.elem() {
                Some(t) => t.clone(),
                None => return unsup("the operand of a dynamic_cast"),
            },
        };
        if !base {
            start = k + 1;
        }
        classes.push(ty.class());
    }
    let n = p.path.len();
    let from = m.structors.iter().filter(|(o, q)| *o == p.obj && q.len() >= start && q.len() <= n && p.path.starts_with(q)).map(|(_, q)| q.len()).max().unwrap_or(start);
    let at = |k: usize| Ptr { obj: p.obj, path: p.path[..k].to_vec(), array: false, view: None };
    let Some(t) = target else { return Ok(Some(at(from))) };
    // 1. The nearest object of class `t` that holds the subobject through
    // public bases.
    for k in (from..=n).rev() {
        if classes[k] == Some(t) {
            if public[k..].iter().all(|&x| x) {
                return Ok(Some(at(k)));
            }
            break;
        }
    }
    // 2. One public base subobject of class `t` of the most derived
    // object, which reaches the operand through public bases.
    if !public[from..].iter().all(|&x| x) {
        return Ok(None);
    }
    let Some(mc) = classes[from] else { return Ok(None) };
    let ps = m.u.base_paths(mc, t);
    if ps.len() != 1 {
        return Ok(None);
    }
    let mut c = mc;
    for &i in &ps[0] {
        let x = m.u.classes[c].fields.get(i).and_then(|&f| m.u.field(f));
        if !x.is_some_and(|x| x.access == crate::sem::Access::Public) {
            return Ok(None);
        }
        c = x.and_then(|x| x.ty.class()).unwrap_or(c);
    }
    let mut q = at(from);
    q.path.extend(ps[0].iter().map(|&i| i as i64));
    Ok(Some(q))
}

/// `dynamic_cast` to a pointer: a null pointer when the check fails.
pub fn dynamic_cast_ptr(m: &mut Machine, x: &crate::sem::tree::Expr, target: Option<ClassId>) -> R<Val> {
    match crate::machine::eval::eval(m, x)? {
        Val::Null => Ok(Val::Null),
        Val::Ptr(p) => Ok(match dynamic_target(m, &p, target)? {
            Some(q) => Val::Ptr(q),
            None => Val::Null,
        }),
        _ => unsup("a dynamic_cast of a pointer of this form"),
    }
}

/// `dynamic_cast` to a reference. A failed check throws `std::bad_cast`,
/// which constant evaluation rejects (CE-8).
pub fn dynamic_cast_ref(m: &mut Machine, x: &crate::sem::tree::Expr, target: Option<ClassId>) -> R<Ptr> {
    let p = crate::machine::eval::place(m, x)?;
    match dynamic_target(m, &p, target)? {
        Some(q) => Ok(q),
        None if m.constant => crate::machine::limit("CE-8", "a dynamic_cast to a reference fails, which throws"),
        None => unsup("the exception that a failed dynamic_cast to a reference throws"),
    }
}
