//! The run part of `classes/unions`: the active member. A union's value
//! holds one value per member, and each member that is not active holds
//! `Val::Inactive`. An access to a member that is not active is undefined
//! (UNI-6). A built-in or trivial assignment through a chain of member
//! accesses makes each union member on its way active (UNI-5, UNI-13).

use crate::machine::{ub, unsup, Machine, Ptr, Val, R};
use crate::sem::types::{ClassId, QType, Type};
use crate::sem::{ClassKey, Unit};

/// The union class of a type, if it is one.
pub fn union_of(u: &Unit, ty: &QType) -> Option<ClassId> {
    ty.class().filter(|&c| u.classes[c].key == ClassKey::Union)
}

/// The union steps of a place: for each step into a member of a union,
/// the place of the union, its class, and the member's position.
fn union_steps(m: &Machine, p: &Ptr) -> Vec<(Ptr, ClassId, usize)> {
    let mut out = Vec::new();
    let Some(mut ty) = m.objs.get(p.obj).map(|o| o.ty.clone()) else { return out };
    for (k, &i) in p.path.iter().enumerate() {
        if let Some(c) = union_of(m.u, &ty) {
            out.push((Ptr { obj: p.obj, path: p.path[..k].to_vec(), array: false, view: None }, c, i as usize));
        }
        ty = match &ty.t {
            Type::Class(c) => match m.u.field_types(*c).get(i as usize) {
                Some(t) => t.clone(),
                None => break,
            },
            _ => match ty.elem() {
                Some(t) => t.clone(),
                None => break,
            },
        };
    }
    out
}

/// The name of member `i` of class `c`.
fn member_name(m: &Machine, c: ClassId, i: usize) -> String {
    m.u.classes[c].fields.get(i).map(|&f| m.u.entities[f].name.to_string()).unwrap_or_default()
}

/// Whether member `i` of the union at `q` is active.
fn active(m: &mut Machine, q: &Ptr, i: usize) -> Option<bool> {
    match m.slot(q)? {
        Val::Agg(xs) => xs.get(i).map(|x| *x != Val::Inactive),
        _ => None,
    }
}

/// UNI-6: the place is inside a member of a union that is not active.
pub fn check_active(m: &mut Machine, p: &Ptr) -> R<()> {
    if p.view.is_some() {
        return Ok(());
    }
    for (q, c, i) in union_steps(m, p) {
        if active(m, &q, i) == Some(false) {
            // UNI-10: a member of the common initial sequence of two
            // structs can be read through the other one.
            let tys = m.u.field_types(c);
            let other = match m.slot(&q) {
                Some(Val::Agg(xs)) => xs.iter().position(|x| *x != Val::Inactive),
                _ => None,
            };
            if other.is_some_and(|j| tys[j].is_class() && tys[i].is_class()) {
                return unsup("a read through the common initial sequence of two members of a union");
            }
            return ub("UNI-6", format!("an access to the member '{}' of a union, which is not its active member", member_name(m, c, i)));
        }
    }
    Ok(())
}

/// Whether the default constructor of a type is trivial, so that an
/// assignment can make a member of the type active (UNI-13, CTOR-23). A
/// default constructor deleted at its first declaration counts as
/// trivial.
pub fn trivial_default(u: &Unit, ty: &QType) -> bool {
    match &ty.t {
        Type::Array(e, _) => trivial_default(u, e),
        Type::Class(c) => {
            if u.polymorphic(*c) {
                return false;
            }
            let own = u.scopes[u.classes[*c].scope].names.get(crate::classes::constructors::check::CTOR).into_iter().flatten().copied();
            let provided = own.filter_map(|f| u.func(f).map(|x| (f, x))).any(|(f, x)| x.ty.params.is_empty() && !x.deleted && !u.defaulted.contains(&f));
            let dmi = u.classes[*c].fields.iter().any(|&f| u.field(f).is_some_and(|x| x.init.is_some()));
            !provided && !dmi && u.field_types(*c).iter().all(|t| trivial_default(u, t))
        }
        _ => true,
    }
}

/// UNI-5, UNI-13: an assignment to the place `p`, whose left operand is a
/// chain of member accesses and subscripts, makes each union member on
/// the way active. The parts of a member that becomes active have no
/// value. A member whose default constructor is not trivial does not
/// become active, and the assignment uses an object that does not exist
/// (UNI-6).
pub fn activate(m: &mut Machine, p: &Ptr) -> R<()> {
    if p.view.is_some() {
        return Ok(());
    }
    let n = p.path.len();
    for k in 0..n {
        let steps = union_steps(m, &Ptr { obj: p.obj, path: p.path[..=k].to_vec(), array: false, view: None });
        let Some((q, c, i)) = steps.last().cloned().filter(|(q, _, _)| q.path.len() == k) else { continue };
        if active(m, &q, i) != Some(false) {
            continue;
        }
        let t = m.u.field_types(c)[i].clone();
        if !trivial_default(m.u, &t) {
            return ub("UNI-6", format!("an assignment to the member '{}' of a union, which is not active and whose default constructor is not trivial", member_name(m, c, i)));
        }
        begin_member(m, &q, c, i)?;
    }
    Ok(())
}

/// Make member `i` of the union at `q` its active member, with no value
/// yet; the other members are not active.
pub fn begin_member(m: &mut Machine, q: &Ptr, c: ClassId, i: usize) -> R<()> {
    let tys = m.u.field_types(c);
    let fresh = m.uninit(&tys[i]);
    match m.slot(q) {
        Some(Val::Agg(xs)) => {
            for x in xs.iter_mut() {
                *x = Val::Inactive;
            }
            if let Some(x) = xs.get_mut(i) {
                *x = fresh;
            }
            Ok(())
        }
        _ => crate::foundations::objects::run::outside(m, q),
    }
}

/// Whether the left operand of an assignment names its object through a
/// chain of member accesses and built-in subscripts (UNI-13).
pub fn is_chain(e: &crate::sem::tree::Expr) -> bool {
    use crate::sem::tree::{Conv, EK};
    match &e.k {
        EK::Member(..) => true,
        EK::Index(x, _) => match &x.k {
            EK::Conv(Conv::ArrayDecay, y) => is_chain(y),
            _ => false,
        },
        _ => false,
    }
}

/// The width of the bit-field that the place `p` names, with its type.
pub fn bitfield_at(m: &Machine, p: &Ptr) -> Option<(u64, QType)> {
    if p.view.is_some() || p.path.is_empty() {
        return None;
    }
    let parent = m.type_at(&Ptr { obj: p.obj, path: p.path[..p.path.len() - 1].to_vec(), array: false, view: None })?;
    let c = parent.class()?;
    let f = *m.u.classes[c].fields.get(*p.path.last()? as usize)?;
    let x = m.u.field(f)?;
    Some((x.width?, x.ty.clone()))
}

/// BF-10: the value that a bit-field of width `w` and type `ty` holds
/// after a store of `v`: the value in its range that differs from `v` by
/// a multiple of 2 to the width. Bits past the type's own are padding
/// (BF-3).
pub fn wrap(m: &Machine, w: u64, ty: &QType, v: Val) -> Val {
    let Val::Int(x) = v else { return v };
    let (bits, signed) = match &ty.t {
        Type::Int(k) => (m.target.int_bits(*k) as u64, k.signed()),
        Type::Enum(e) => match m.u.enums.get(*e).and_then(|i| i.underlying) {
            Some(k) => (m.target.int_bits(k) as u64, k.signed()),
            None => return Val::Int(x),
        },
        _ => return Val::Int(x),
    };
    let n = w.min(bits).min(127) as u32;
    if n == 0 {
        return Val::Int(0);
    }
    let modulus = 1i128 << n;
    let mut r = x.rem_euclid(modulus);
    if signed && r >= modulus / 2 {
        r -= modulus;
    }
    Val::Int(r)
}

/// A value stored at the place `p`, wrapped to the width of the
/// bit-field that `p` names (BF-10).
pub fn store(m: &Machine, p: &Ptr, v: Val) -> Val {
    match bitfield_at(m, p) {
        Some((w, ty)) => wrap(m, w, &ty, v),
        None => v,
    }
}
