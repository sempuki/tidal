//! The run part of `classes/constructors`: destructors. An object whose
//! class has a destructor, or a member that needs one, is destroyed when
//! its life ends (DTOR-5): the destructor's body runs, then the members
//! are destroyed, last declared first; the elements of an array go from
//! last to first. A second destruction is undefined (DTOR-7).

use crate::machine::{ub, Machine, Ptr, Ret, R};
use crate::sem::types::{QType, Type};

/// The destructor that a class declares, if any.
pub fn dtor(m: &Machine, c: crate::sem::types::ClassId) -> Option<crate::sem::EntityId> {
    m.u.scopes[m.u.classes[c].scope].names.get(crate::classes::constructors::check::DTOR)?.first().copied()
}

/// A type whose objects need a destructor run: a class with a
/// destructor that the program provides, or with a member that needs
/// one, and an array of such.
pub fn needs(m: &Machine, ty: &QType) -> bool {
    needs_in(m.u, ty)
}

/// `needs`, from the unit alone.
pub fn needs_in(u: &crate::sem::Unit, ty: &QType) -> bool {
    match &ty.t {
        Type::Array(e, _) => needs_in(u, e),
        Type::Class(c) => {
            let user = u.scopes[u.classes[*c].scope].names.get(crate::classes::constructors::check::DTOR).and_then(|v| v.first()).is_some_and(|&d| !u.defaulted.contains(&d));
            user || u.field_types(*c).iter().any(|t| needs_in(u, t))
        }
        _ => false,
    }
}

/// Destroy the object at `p` of type `ty` (DTOR-5).
pub fn destroy(m: &mut Machine, p: &Ptr, ty: &QType) -> R<()> {
    match &ty.t {
        Type::Array(e, Some(n)) => {
            for k in (0..*n as i64).rev() {
                let mut q = p.clone();
                q.path.push(k);
                q.array = true;
                destroy(m, &q, e)?;
            }
            Ok(())
        }
        Type::Class(c) => {
            if let Some(d) = dtor(m, *c).filter(|d| !m.u.defaulted.contains(d)) {
                m.constructing.push(p.obj);
                let r = crate::machine::eval::call(m, d, vec![Ret::Place(p.clone())]);
                m.constructing.pop();
                r?;
            }
            let tys = m.u.field_types(*c);
            for (k, t) in tys.iter().enumerate().rev() {
                if needs(m, t) {
                    let mut q = p.clone();
                    q.path.push(k as i64);
                    q.array = false;
                    destroy(m, &q, t)?;
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// The objects of a block or a full expression end, in the reverse
/// order of their creation: each that needs it is destroyed first.
pub fn end_objects(m: &mut Machine, objs: Vec<usize>) -> R<()> {
    for o in objs.into_iter().rev() {
        let ty = m.objs[o].ty.clone();
        // DTOR-7: also a trivial destructor runs a second time.
        if m.destroyed.contains(&o) && ty.base_elem().is_class() && !needs(m, &ty) {
            return ub("DTOR-7", format!("{} is destroyed a second time", m.objs[o].name));
        }
        if needs(m, &ty) {
            if m.destroyed.contains(&o) {
                return ub("DTOR-7", format!("{} is destroyed a second time", m.objs[o].name));
            }
            if m.objs[o].alive && !m.undeclared.remove(&o) {
                destroy(m, &Ptr::whole(o), &ty)?;
            }
        }
        m.objs[o].alive = false;
    }
    Ok(())
}

/// DTOR-6: an explicit destructor call runs the destructor of the object
/// at `p` and ends its life; DTOR-7: a second destruction is undefined,
/// also at the end of its block.
pub fn destroy_explicitly(m: &mut Machine, p: &Ptr) -> R<crate::machine::Val> {
    if !p.path.is_empty() || p.view.is_some() {
        return crate::machine::unsup("an explicit destructor call on a subobject");
    }
    if m.destroyed.contains(&p.obj) || m.objs.get(p.obj).is_some_and(|o| !o.alive) {
        return ub("DTOR-7", format!("{} is destroyed a second time", m.objs[p.obj].name));
    }
    if m.constant {
        crate::foundations::constant_evaluation::run::check_write(m, p)?;
    }
    let ty = m.objs[p.obj].ty.clone();
    if needs(m, &ty) {
        destroy(m, p, &ty)?;
    }
    m.objs[p.obj].alive = false;
    m.destroyed.insert(p.obj);
    Ok(crate::machine::Val::Void)
}
