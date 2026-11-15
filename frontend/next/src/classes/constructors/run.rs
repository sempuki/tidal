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
            user || u.object_types(*c).iter().any(|t| needs_in(u, t))
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
            let mut r = Ok(());
            if let Some(d) = dtor(m, *c).filter(|d| !m.u.defaulted.contains(d)) {
                m.constructing.push(p.obj);
                r = crate::machine::eval::call(m, d, vec![Ret::Place(p.clone())]).map(|_| ());
                m.constructing.pop();
            }
            // UNI-3: a union's destructor ends none of its members. A
            // destructor with a function try block ended them itself
            // (TRY-10).
            let guarded = dtor(m, *c).and_then(|d| m.u.func(d)).and_then(|f| f.body.clone()).is_some_and(|b| matches!(b.k, crate::sem::tree::SK::Try { .. }));
            if crate::classes::unions::run::union_of(m.u, ty).is_some() || guarded {
                return r;
            }
            // DER-7: a complete object destroys its virtual bases last.
            // UNW-2: the members end also after the body throws.
            let complete = crate::classes::derived::run::is_complete(m, p);
            let n = m.u.classes[*c].fields.len();
            let tys = if complete { m.u.object_types(*c) } else { m.u.field_types(*c) };
            let order = (0..n).rev().chain((n..tys.len()).rev());
            for (k, t) in order.map(|k| (k, &tys[k])) {
                if needs(m, t) {
                    let mut q = p.clone();
                    q.path.push(k as i64);
                    q.array = false;
                    r = crate::exceptions::throw::run::unwind(m, r, |m| destroy(m, &q, t));
                }
            }
            r
        }
        _ => Ok(()),
    }
}

/// UNW-2: the members and bases of the object at `p` of class `c`
/// end, last first, without the destructor of the class itself. A
/// complete object's virtual bases end last.
pub fn destroy_parts(m: &mut Machine, p: &Ptr, c: crate::sem::types::ClassId) -> R<()> {
    let complete = crate::classes::derived::run::is_complete(m, p);
    let n = m.u.classes[c].fields.len();
    let tys = if complete { m.u.object_types(c) } else { m.u.field_types(c) };
    let mut r = Ok(());
    for k in (0..n).rev().chain((n..tys.len()).rev()) {
        if needs(m, &tys[k]) {
            let mut q = p.clone();
            q.path.push(k as i64);
            q.array = false;
            r = crate::exceptions::throw::run::unwind(m, r, |m| destroy(m, &q, &tys[k]));
        }
    }
    r
}

/// The objects of a block or a full expression end, in the reverse
/// order of their creation: each that needs it is destroyed first.
pub fn end_objects(m: &mut Machine, objs: Vec<usize>) -> R<()> {
    // UNW-2: after a destructor throws, the other objects still end.
    let mut r = Ok(());
    for o in objs.into_iter().rev() {
        if matches!(r, Err(ref s) if !matches!(s, crate::machine::Stop::Throw(_))) {
            return r;
        }
        r = crate::exceptions::throw::run::unwind(m, r, |m| end_object(m, o));
    }
    r
}

/// One object of `end_objects` ends.
fn end_object(m: &mut Machine, o: usize) -> R<()> {
    let ty = m.objs[o].ty.clone();
    // STG-21: an object of another type took the storage of an object
    // whose destructor the end of its block runs.
    if crate::expressions::new_delete::run::replaced(m, o) && needs(m, &ty) {
        return ub("STG-21", format!("the end of the block destroys {}, whose storage holds an object of another type", m.objs[o].name));
    }
    // A trivial destructor of the ended object does not run.
    if crate::expressions::new_delete::run::replaced(m, o) {
        return Ok(());
    }
    // DTOR-7: also a trivial destructor runs a second time.
    if m.destroyed.contains(&o) && ty.base_elem().is_class() && !needs(m, &ty) {
        return ub("DTOR-7", format!("{} is destroyed a second time", m.objs[o].name));
    }
    if needs(m, &ty) {
        if m.destroyed.contains(&o) {
            return ub("DTOR-7", format!("{} is destroyed a second time", m.objs[o].name));
        }
        if m.objs[o].alive && !m.undeclared.remove(&o) {
            let r = destroy(m, &Ptr::whole(o), &ty);
            m.objs[o].alive = false;
            return r;
        }
    }
    m.objs[o].alive = false;
    Ok(())
}

/// DTOR-6: an explicit destructor call runs the destructor of the object
/// at `p` and ends its life; DTOR-7: a second destruction is undefined,
/// also at the end of its block.
pub fn destroy_explicitly(m: &mut Machine, p: &Ptr) -> R<crate::machine::Val> {
    // UNI-14: the active member of a union ends, and the union has no
    // active member.
    if p.view.is_none() && !p.path.is_empty() {
        let mut u = p.clone();
        u.path.pop();
        u.array = false;
        if m.type_at(&u).is_some_and(|t| crate::classes::unions::run::union_of(m.u, &t).is_some()) {
            crate::classes::unions::run::check_active(m, p)?;
            if m.constant {
                crate::foundations::constant_evaluation::run::check_write(m, p)?;
            }
            let Some(ty) = m.type_at(p) else { return crate::machine::unsup("an explicit destructor call on a subobject") };
            if needs(m, &ty) {
                destroy(m, p, &ty)?;
            }
            crate::classes::unions::run::end_members(m, &u);
            return Ok(crate::machine::Val::Void);
        }
    }
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
