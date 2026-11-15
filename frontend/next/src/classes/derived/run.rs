//! The run part of `classes/derived`: the subobjects of virtual bases.
//! A complete object of a class holds its data members, then one
//! subobject for each virtual base, in the order of
//! `ClassInfo::vbases` (`Unit::object_types`). A base subobject holds
//! only its data members. A step of `VSTEP` in a path names a virtual
//! base, which the machine finds in the complete object that holds the
//! subobject (DER-6).

use crate::machine::{unsup, Machine, Ptr, R};
use crate::sem::types::{ClassId, Type};
use crate::sem::{Unit, VSTEP};

/// The part at position `i` of an object of class `c` is a base
/// subobject: a base that is not virtual, or a virtual base of a
/// complete object.
pub fn base_step(u: &Unit, c: ClassId, i: usize) -> bool {
    i >= u.classes[c].fields.len() || u.classes[c].fields.get(i).and_then(|&f| u.field(f)).is_some_and(|x| x.base)
}

/// The complete object that holds the subobject at `p`, and its class:
/// `p` without its last steps into base subobjects.
pub fn complete(m: &Machine, p: &Ptr) -> Option<(Ptr, ClassId)> {
    let mut ty = m.objs.get(p.obj)?.ty.clone();
    let (mut start, mut start_ty) = (0, ty.clone());
    let n = p.path.len();
    for (k, &i) in p.path.iter().enumerate() {
        if k + 1 == n && p.array && !ty.is_array() {
            break;
        }
        let base = ty.class().is_some_and(|c| base_step(m.u, c, i as usize));
        ty = match &ty.t {
            Type::Class(c) => m.u.object_types(*c).get(i as usize)?.clone(),
            _ => ty.elem()?.clone(),
        };
        if !base {
            start = k + 1;
            start_ty = ty.clone();
        }
    }
    Some((Ptr { obj: p.obj, path: p.path[..start].to_vec(), array: false, view: None }, start_ty.class()?))
}

/// The object at `p` is a complete object: no base subobject.
pub fn is_complete(m: &Machine, p: &Ptr) -> bool {
    complete(m, p).is_some_and(|(q, _)| q.path.len() == p.path.len())
}

/// The subobject of the virtual base `v` of the object at `p`.
pub fn virtual_base(m: &Machine, p: &Ptr, v: ClassId) -> R<Ptr> {
    if p.view.is_some() {
        return unsup("a virtual base of an object seen through a pointer of another type");
    }
    let Some((mut q, x)) = complete(m, p) else { return unsup("a virtual base of an object of this form") };
    let Some(k) = m.u.classes[x].vbases.iter().position(|&w| w == v) else {
        return crate::machine::ub("DER-17", format!("an access to the virtual base '{}' of an object that does not have it", m.u.classes[v].name));
    };
    q.path.push((m.u.classes[x].fields.len() + k) as i64);
    Ok(q)
}

/// The subobject at step `i` from the object at `p` of class `c`.
pub fn step(m: &Machine, mut p: Ptr, c: ClassId, i: usize) -> R<Ptr> {
    if i >= VSTEP {
        let Some(&v) = m.u.classes[c].vbases.get(i - VSTEP) else { return unsup("a virtual base step of this form") };
        return virtual_base(m, &p, v);
    }
    p.path.push(i as i64);
    p.array = false;
    Ok(p)
}

/// The base subobject at the end of `path` from the object at `p` of
/// class `c`.
pub fn along(m: &Machine, mut p: Ptr, mut c: ClassId, path: &[usize]) -> R<Ptr> {
    for &i in path {
        p = step(m, p, c, i)?;
        c = m.u.step_class(c, i).unwrap_or(c);
    }
    p.array = false;
    Ok(p)
}

/// The steps from the object at `from` to the subobject at `p` inside
/// it, as a path of `Unit::base_paths`: a step into a virtual base of
/// the complete object at `from` becomes a `VSTEP` step.
pub fn logical(m: &Machine, from: &Ptr, c: ClassId, p: &Ptr) -> Vec<usize> {
    let rel = &p.path[from.path.len().min(p.path.len())..];
    let len = m.u.classes[c].fields.len();
    rel.iter().enumerate().map(|(k, &i)| if k == 0 && i as usize >= len { VSTEP + (i as usize - len) } else { i as usize }).collect()
}
