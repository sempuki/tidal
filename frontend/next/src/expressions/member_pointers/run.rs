//! The run part of `expressions/member-pointers`: a pointer to member
//! holds its member and the path of base subobjects from a class to the
//! class that declares the member, so that conversions keep its value
//! (MPTR-4). `.*` finds the subobject of that class in the object, or in
//! the object that holds it as a base (MPTR-3), and stops where there is
//! none or the pointer is null (MPTR-6).

use crate::machine::eval::eval;
use crate::machine::{ub, unsup, Machine, Ptr, Val, R};
use crate::sem::tree::{Expr, MemberRef};
use crate::sem::types::{ClassId, QType};
use crate::sem::EntityId;

/// The value of a pointer to member that is not null: the member, the
/// class `anchor`, and the positions of the base subobjects from the
/// anchor to the class that declares the member. The anchor is the class
/// of the pointer's type, or a class derived from it after a
/// `static_cast` to a base.
#[derive(Clone, Debug, PartialEq)]
pub struct MemVal {
    pub anchor: ClassId,
    pub path: Vec<usize>,
    pub member: MemberRef,
}

/// A conversion of the pointer to member `v` from type `from` to type
/// `to`. To a derived class, the anchor moves to that class along its
/// one path to the anchor. To a base, the anchor moves back where the
/// path starts with the base's subobject, and stays otherwise.
pub fn convert(m: &Machine, v: Val, from: &QType, to: &QType) -> Val {
    let Val::Mem(mut x) = v else { return v };
    let (Some((fc, _)), Some((tc, _))) = (from.member_ptr(), to.member_ptr()) else { return Val::Mem(x) };
    if fc == tc {
        return Val::Mem(x);
    }
    let down = m.u.base_paths(tc, x.anchor);
    if x.anchor != tc && down.len() == 1 {
        let mut p = down[0].clone();
        p.extend(x.path);
        x.path = p;
        x.anchor = tc;
        return Val::Mem(x);
    }
    let up = m.u.base_paths(x.anchor, tc);
    if up.len() == 1 && x.path.starts_with(&up[0]) {
        x.path.drain(..up[0].len());
        x.anchor = tc;
    }
    Val::Mem(x)
}

/// The subobject of class `anchor` for the object at `p`: the object
/// itself, one of its bases, or the object that holds `p` as a base.
pub fn locate(m: &Machine, p: &Ptr, anchor: ClassId) -> R<Ptr> {
    if p.view.is_some() {
        return unsup("a pointer to member applied through a pointer of another type");
    }
    let mut q = p.clone();
    loop {
        let Some(c) = m.type_at(&q).and_then(|t| t.class()) else { break };
        let ps = m.u.base_paths(c, anchor);
        if ps.len() == 1 {
            q.path.extend(ps[0].iter().map(|&i| i as i64));
            q.array = false;
            return Ok(q);
        }
        // The object that holds this one as a base subobject.
        let Some(&last) = q.path.last() else { break };
        let mut up = q.clone();
        up.path.pop();
        let base = m.type_at(&up).and_then(|t| t.class()).and_then(|u| m.u.classes[u].fields.get(last as usize).copied()).and_then(|f| m.u.field(f)).is_some_and(|f| f.base);
        if !base {
            break;
        }
        q = up;
    }
    ub("MPTR-6", "a pointer to member applied to an object that does not have the member")
}

/// The subobject of the class that declares the member, for the object
/// at `p`.
fn holder(m: &Machine, p: &Ptr, x: &MemVal) -> R<Ptr> {
    let mut q = locate(m, p, x.anchor)?;
    q.path.extend(x.path.iter().map(|&i| i as i64));
    Ok(q)
}

/// The value of a pointer to member operand.
fn member(m: &mut Machine, pm: &Expr) -> R<MemVal> {
    match eval(m, pm)? {
        Val::Mem(x) => Ok(x),
        Val::Null => ub("MPTR-6", "a null pointer to member applied to an object"),
        Val::Uninit => crate::declarations::initialization::forms::run::read_no_value("the pointer to member"),
        _ => unsup("a pointer to member of this form"),
    }
}

/// `o.*p` for a pointer to data member: the member's place.
pub fn place(m: &mut Machine, obj: &Expr, pm: &Expr) -> R<Ptr> {
    let p = crate::machine::eval::place(m, obj)?;
    let x = member(m, pm)?;
    let MemberRef::Field(i) = x.member else { return unsup("a pointer to member function applied as a data member") };
    let mut q = holder(m, &p, &x)?;
    q.path.push(i as i64);
    q.array = false;
    Ok(q)
}

/// The function that a call through a pointer to member function calls,
/// with the value of the pointer.
pub fn callee(m: &mut Machine, pm: &Expr) -> R<(MemVal, EntityId)> {
    let x = member(m, pm)?;
    let MemberRef::Func(f) = x.member else { return unsup("a pointer to data member called as a function") };
    // A pointer that a reinterpret_cast left with another type.
    let want = pm.ty.member_ptr().and_then(|(_, t)| t.func().map(|f| (f.params.clone(), f.ret.clone())));
    let have = m.u.func(f).map(|x| (x.ty.params.clone(), x.ty.ret.clone()));
    if want != have {
        return unsup("a call through a pointer to member function of another type");
    }
    Ok((x, f))
}

/// The object of a call through the pointer to member function `x`: the
/// subobject of the function's class.
pub fn object(m: &Machine, p: &Ptr, x: &MemVal) -> R<Ptr> {
    holder(m, p, x)
}

/// The class that declares the member of `x`.
fn home(m: &Machine, x: &MemVal) -> ClassId {
    let mut c = x.anchor;
    for &i in &x.path {
        c = m.u.field_types(c)[i].class().unwrap_or(c);
    }
    c
}

/// MPTR-5: two pointers to members are equal when both are null or both
/// name the same member of the same subobject. Two members of one union
/// with the same type count as one.
pub fn equal(m: &Machine, a: &Val, b: &Val) -> R<bool> {
    Ok(match (a, b) {
        (Val::Null, Val::Null) => true,
        (Val::Mem(_), Val::Null) | (Val::Null, Val::Mem(_)) => false,
        (Val::Mem(x), Val::Mem(y)) if x.anchor != y.anchor || x.path != y.path => false,
        (Val::Mem(x), Val::Mem(y)) => match (x.member, y.member) {
            (MemberRef::Field(i), MemberRef::Field(j)) if m.u.classes[home(m, x)].key == crate::sem::ClassKey::Union => {
                let tys = m.u.field_types(home(m, x));
                tys[i] == tys[j]
            }
            (p, q) => p == q,
        },
        _ => return unsup("a comparison of pointers to members of this form"),
    })
}
