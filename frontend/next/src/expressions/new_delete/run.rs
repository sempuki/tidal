//! The run part of `expressions/new-delete`: a `new` expression creates
//! an object or an array and initializes it (NEW-1, NEW-2, NEW-9), and
//! `delete` destroys it and frees it (NEW-4, NEW-9). The undefined uses
//! of `delete` stop the run (NEW-5, VIRT-12), and so does a read of an
//! object that `new` left without a value (NEW-3).

use crate::declarations::initialization::forms::run::construct;
use crate::machine::eval::eval;
use crate::machine::{limit, ub, unsup, Machine, Ptr, Val, R};
use crate::sem::tree::{Expr, Init, New};
use crate::sem::types::{QType, Type};

/// A `new` expression: its result points to the object, or to the first
/// element of the array.
pub fn new(m: &mut Machine, n: &New) -> R<Val> {
    let count = match &n.bound {
        Some(b) => {
            let Val::Int(k) = eval(m, b)? else { return unsup("an array bound of this form") };
            let Init::List(items) = &n.init else { return unsup("an array initializer of this form") };
            // NEW-35: an erroneous bound throws instead of allocating.
            let elem = m.target.size_of(m.u, &n.ty).unwrap_or(1).max(1) as i128;
            let max = m.target.int_max(m.target.size_t());
            let wrapped = m.target.wrap(m.target.ptrdiff_t(), k);
            if wrapped < 0 || k.saturating_mul(elem) > max || (k as usize) < items.len() {
                if m.constant {
                    return limit("NEW-39", "the bound of an array new expression is erroneous");
                }
                return unsup("the exception of an erroneous bound of an array new expression");
            }
            Some(k as u64)
        }
        None => None,
    };
    let ty = match count {
        Some(k) => QType::new(Type::Array(std::rc::Rc::new(n.ty.clone()), Some(k))),
        None => n.ty.clone(),
    };
    m.check_size(&ty)?;
    let o = m.new_obj(&ty, m.uninit(&ty), "the object that new created");
    m.objs[o].heap = Some((count.is_some(), false));
    m.objs[o].konst = false;
    match count {
        None => construct(m, &Ptr::whole(o), &ty, &n.init)?,
        Some(k) => {
            let Init::List(items) = &n.init else { return unsup("an array initializer of this form") };
            for i in 0..k as usize {
                let q = Ptr { obj: o, path: vec![i as i64], array: true, view: None };
                construct(m, &q, &n.ty, items.get(i).unwrap_or(&n.rest))?;
            }
        }
    }
    m.objs[o].konst = n.ty.is_const() && count.is_none();
    Ok(Val::Ptr(Ptr { obj: o, path: if count.is_some() { vec![0] } else { Vec::new() }, array: count.is_some(), view: None }))
}

/// A `delete` expression: `e` is the pointer, `array` marks `delete[]`,
/// and `virtual_dtor` says that the class the pointer points to has a
/// virtual destructor.
pub fn delete(m: &mut Machine, e: &Expr, array: bool, virtual_dtor: bool) -> R<Val> {
    let p = match eval(m, e)? {
        Val::Null => return Ok(Val::Void),
        Val::Ptr(p) => p,
        Val::Uninit => return crate::declarations::initialization::forms::run::read_no_value("the pointer"),
        _ => return unsup("the operand of delete of this form"),
    };
    let Some((was_array, freed)) = m.objs.get(p.obj).and_then(|o| o.heap) else {
        return ub("NEW-5", "a delete of a pointer that no new expression gave");
    };
    if p.view.is_some() {
        return unsup("a delete through a pointer of another type");
    }
    if freed {
        return ub("NEW-5", "a delete of an object that a delete already freed");
    }
    if was_array != array {
        return if array { ub("NEW-5", "a delete[] of an object that new created") } else { ub("NEW-5", "a delete of an array that new[] created") };
    }
    // The path from the object to the place the pointer points to: the
    // first element of an array, then bases only.
    let skip = if array {
        if p.path.first() != Some(&0) {
            return ub("NEW-5", "a delete[] of a pointer that is not the result of new[]");
        }
        1
    } else {
        0
    };
    let mut ty = m.objs[p.obj].ty.clone();
    if array {
        ty = ty.elem().cloned().unwrap_or(ty);
    }
    let mut cur = ty.clone();
    for &i in &p.path[skip..] {
        let base = cur.class().and_then(|c| m.u.classes[c].fields.get(i as usize).copied()).and_then(|x| m.u.field(x)).is_some_and(|x| x.base);
        if !base {
            return ub("NEW-5", "a delete of a pointer to a subobject of the object that new created");
        }
        cur = m.u.field_types(cur.class().unwrap())[i as usize].clone();
    }
    if p.path.len() > skip {
        if array {
            return ub("VIRT-12", "a delete[] of an array of a derived class through a pointer to a base");
        }
        if !virtual_dtor {
            return ub("VIRT-12", "a delete of an object of a derived class through a pointer to a base whose destructor is not virtual");
        }
    }
    if m.constant {
        crate::foundations::constant_evaluation::run::check_write(m, &Ptr::whole(p.obj))?;
    }
    let whole = m.objs[p.obj].ty.clone();
    // DTOR-7: an explicit destructor call already destroyed the object.
    if m.destroyed.contains(&p.obj) {
        return ub("DTOR-7", format!("{} is destroyed a second time", m.objs[p.obj].name));
    }
    if crate::classes::constructors::run::needs(m, &whole) {
        crate::classes::constructors::run::destroy(m, &Ptr::whole(p.obj), &whole)?;
    }
    let o = &mut m.objs[p.obj];
    o.alive = false;
    o.heap = Some((was_array, true));
    Ok(Val::Void)
}

/// NEW-6: at the end of a constant evaluation, each object that `new`
/// created has been deleted.
pub fn leaked(m: &Machine) -> R<()> {
    if m.objs.iter().any(|o| matches!(o.heap, Some((_, false)))) {
        return limit("NEW-6", "storage that a new expression allocated is not freed at the end of the evaluation");
    }
    Ok(())
}

/// NEW-3: a read of an object that `new` created without a value.
pub fn no_value<T>(m: &Machine, p: &Ptr) -> Option<R<T>> {
    m.objs.get(p.obj).and_then(|o| o.heap).map(|_| ub("NEW-3", "a read of an object that new created before it has a value"))
}
