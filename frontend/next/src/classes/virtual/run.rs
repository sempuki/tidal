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
