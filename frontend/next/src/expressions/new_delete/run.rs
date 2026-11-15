//! The run part of `expressions/new-delete`: a `new` expression creates
//! an object or an array and initializes it (NEW-1, NEW-2, NEW-9), and
//! `delete` destroys it and frees it (NEW-4, NEW-9). The undefined uses
//! of `delete` stop the run (NEW-5, VIRT-12), and so does a read of an
//! object that `new` left without a value (NEW-3).

use crate::declarations::initialization::forms::run::construct;
use crate::machine::eval::eval;
use crate::machine::{limit, ub, unsup, Machine, Ptr, Ret, Val, R};
use crate::sem::tree::{Expr, Init, New};
use crate::sem::types::{QType, Type};

/// A `new` expression: its result points to the object, or to the first
/// element of the array.
pub fn new(m: &mut Machine, n: &New) -> R<Val> {
    // An allocation function that the program gives (NEW-10, NEW-13):
    // the placement arguments come first (NEW-12).
    let user = n.alloc.as_ref().filter(|a| !is_library(m, a.f));
    let mut place = Vec::new();
    if let Some(a) = user {
        if m.constant {
            return limit("NEW-37", "a new expression whose allocation function the program gives");
        }
        for arg in &a.args {
            place.push(if arg.by_ref { Ret::Place(crate::machine::eval::place(m, &arg.e)?) } else { Ret::Val(crate::machine::eval::eval_any(m, &arg.e)?) });
        }
    }
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
    let target = match user {
        Some(a) => {
            let mut vals = vec![Ret::Val(Val::Int(m.target.size_of(m.u, &ty).unwrap_or(1) as i128))];
            if a.align {
                vals.push(Ret::Val(Val::Int(m.target.align_of(m.u, &n.ty).unwrap_or(1) as i128)));
            }
            vals.extend(place);
            match crate::machine::eval::call(m, a.f, vals)? {
                // NEW-31: a null result initializes nothing; NEW-32: only a
                // non-throwing function can give one.
                Ret::Val(Val::Null) if m.u.func(a.f).is_some_and(|x| x.ty.noexcept) => return Ok(Val::Null),
                Ret::Val(Val::Null) => return ub("NEW-32", "an allocation function that can throw returns a null pointer"),
                Ret::Val(Val::Ptr(p)) => create_at(m, p, &ty, count.is_some())?,
                _ => return unsup("the result of an allocation function of this form"),
            }
        }
        None => {
            let o = m.new_obj(&ty, m.uninit(&ty), "the object that new created");
            m.objs[o].heap = Some((count.is_some(), false));
            m.objs[o].konst = false;
            Ptr::whole(o)
        }
    };
    match count {
        None => construct(m, &target, &ty, &n.init)?,
        Some(k) => {
            let Init::List(items) = &n.init else { return unsup("an array initializer of this form") };
            for i in 0..k as usize {
                let mut q = target.clone();
                q.path.push(i as i64);
                q.array = true;
                construct(m, &q, &n.ty, items.get(i).unwrap_or(&n.rest))?;
            }
        }
    }
    if target.path.is_empty() {
        m.objs[target.obj].konst = n.ty.is_const() && count.is_none();
    }
    let mut out = target;
    if count.is_some() {
        out.path.push(0);
        out.array = true;
    }
    Ok(Val::Ptr(out))
}

/// A global allocation or deallocation function that the program does
/// not define, whose body the implementation gives (NEW-19).
pub fn is_library(m: &Machine, f: crate::sem::EntityId) -> bool {
    let f = crate::expressions::postfix::run::defining(m, f);
    m.u.library.contains(&f) && m.body(f).is_none()
}

/// The object of type `ty` that a `new` expression creates in the
/// storage at `p` that its allocation function returned (STG-5, STG-8).
/// An object of the same type at the place is replaced in its place
/// (STG-9). Otherwise the new object takes the storage, and the objects
/// that it overlaps in the same storage end (STG-6).
fn create_at(m: &mut Machine, p: Ptr, ty: &QType, array: bool) -> R<Ptr> {
    // STG-20: no new object in the storage of a const complete object
    // with automatic, static, or thread storage duration.
    if m.objs[p.obj].konst && m.objs[p.obj].heap.is_none() {
        return ub("STG-20", format!("a new object in the storage of the const object {}", m.objs[p.obj].name));
    }
    // STG-17: the storage is aligned for the type.
    let align = m.target.align_of(m.u, ty).unwrap_or(1).max(1);
    if let Some(a) = crate::foundations::storage::run::address(m, &Val::Ptr(p.clone())) {
        if a % align != 0 {
            return ub("STG-17", format!("a new object of type {} at an address that is not aligned for it", ty.show()));
        }
    }
    if p.view.is_none() && !array && m.type_at(&p).is_some_and(|t| t.unqual() == ty.unqual()) {
        // The new object begins its life where an explicit destructor
        // call ended the old one.
        if p.path.is_empty() {
            m.destroyed.remove(&p.obj);
            m.objs[p.obj].alive = true;
        }
        // UNI-14: a member of a union becomes its active member.
        if let Some(&k) = p.path.last() {
            let mut u = p.clone();
            u.path.pop();
            if let Some(c) = m.type_at(&u).and_then(|t| crate::classes::unions::run::union_of(m.u, &t)) {
                crate::classes::unions::run::begin_member(m, &u, c, k as usize)?;
            }
        }
        return Ok(p);
    }
    m.hosting.insert(p.obj);
    let off = crate::foundations::storage::run::offset(m, &p);
    let size = crate::foundations::storage::run::size(m, ty);
    for k in 0..m.objs.len() {
        if let Some((h, o)) = m.objs[k].host {
            let n = crate::foundations::storage::run::size(m, &m.objs[k].ty.clone());
            if h == p.obj && o < off + size && off < o + n {
                end_hosted(m, k);
            }
        }
    }
    // STG-6: an object that does not host the new one ends.
    let host_ty = m.objs[p.obj].ty.clone();
    let hosts = m.objs[p.obj].raw || (host_ty.is_array() && crate::foundations::storage::run::is_byte(m, host_ty.base_elem()) && host_ty.base_elem().unqual().t != Type::Int(crate::sem::types::IntK::Char));
    if !hosts && p.path.is_empty() && p.view.is_none() {
        m.objs[p.obj].alive = false;
    }
    let o = m.new_obj(ty, m.uninit(ty), "the object that new created");
    m.objs[o].host = Some((p.obj, off));
    m.objs[o].heap = Some((array, false));
    m.objs[o].konst = false;
    Ok(Ptr::whole(o))
}

/// The object `obj` ended, and an object of another type took its
/// storage (STG-19, STG-21).
pub fn replaced(m: &Machine, obj: usize) -> bool {
    !m.objs[obj].alive && m.hosting.contains(&obj) && m.objs.iter().any(|o| o.alive && o.host == Some((obj, 0)) && o.ty.unqual() != m.objs[obj].ty.unqual())
}

/// The object `k` ends, with each object in its storage.
fn end_hosted(m: &mut Machine, k: usize) {
    m.objs[k].alive = false;
    for j in 0..m.objs.len() {
        if m.objs[j].host.is_some_and(|(h, _)| h == k) && m.objs[j].alive {
            end_hosted(m, j);
        }
    }
}

/// A call of a global allocation or deallocation function by name, whose
/// body the implementation gives (NEW-19, NEW-26): `operator new` gives
/// storage of the size, in which the program creates objects (STG-8);
/// `operator delete` frees the storage that the matching form gave, and
/// does nothing with a null pointer (NEW-41).
pub fn library(m: &mut Machine, f: crate::sem::EntityId, args: Vec<Ret>) -> R<Ret> {
    let name = m.u.entities[f].name.clone();
    let array = name.ends_with("[]");
    let arg = |k: usize| match args.get(k) {
        Some(Ret::Val(v)) => Some(v.clone()),
        _ => None,
    };
    if name.starts_with("operatornew") {
        if m.constant {
            return limit("LSUP-11", format!("a call of the global '{}' by name in a constant evaluation", name.replace("operator", "operator ")));
        }
        let Some(Val::Int(n)) = arg(0) else { return unsup("the size of an allocation of this form") };
        let ty = QType::new(Type::Array(std::rc::Rc::new(QType::new(Type::Int(crate::sem::types::IntK::UChar))), Some(n.max(1) as u64)));
        m.check_size(&ty)?;
        let o = m.new_obj(&ty, m.uninit(&ty), "the storage that operator new gave");
        m.objs[o].heap = Some((array, false));
        m.objs[o].raw = true;
        m.objs[o].konst = false;
        // NEW-26: an aligned form gives storage at a multiple of the
        // alignment.
        if let Some(Val::Int(a)) = arg(1).filter(|_| m.u.func(f).is_some_and(|x| x.ty.params.len() == 2)) {
            crate::foundations::storage::run::align_base(m, o, a.max(1) as u64);
            m.aligned_alloc.insert(o);
        }
        return Ok(Ret::Val(Val::Ptr(Ptr::whole(o))));
    }
    match arg(0) {
        Some(Val::Null) => Ok(Ret::Val(Val::Void)),
        Some(Val::Ptr(p)) => {
            // NEW-40: the partner of an aligned form takes the alignment.
            let aligned = m.u.func(f).is_some_and(|x| x.ty.params.last().is_some_and(|t| t.is_enum()));
            let root = crate::foundations::storage::run::root(m, p.obj);
            if m.objs[root].raw && m.aligned_alloc.contains(&root) != aligned {
                return ub("NEW-40", "a deallocation by a form that does not match the alignment of the allocation");
            }
            free(m, &p, array, "NEW-40")?;
            Ok(Ret::Val(Val::Void))
        }
        _ => unsup("the operand of a deallocation function of this form"),
    }
}

/// Free the storage that `p` points to the start of: storage that a call
/// of a global allocation function gave, or an object that a `new`
/// expression created, after its objects end (NEW-5).
fn free(m: &mut Machine, p: &Ptr, array: bool, rule: &'static str) -> R<()> {
    let (mut obj, mut off) = (p.obj, crate::foundations::storage::run::offset(m, p));
    loop {
        if off != 0 {
            return ub(rule, "a deallocation of a pointer that is not the start of its storage");
        }
        let o = &m.objs[obj];
        if o.raw || (o.heap.is_some() && o.host.is_none()) {
            break;
        }
        match o.host {
            Some((h, at)) => (obj, off) = (h, at),
            None => return ub(rule, "a deallocation of storage that no allocation function gave"),
        }
    }
    let Some((was_array, freed)) = m.objs[obj].heap else { return ub(rule, "a deallocation of storage that no allocation function gave") };
    if freed {
        return ub(rule, "a deallocation of storage that a deallocation already freed");
    }
    if m.objs[obj].raw && was_array != array {
        return ub(rule, "a deallocation by the form that does not match the allocation");
    }
    end_hosted(m, obj);
    m.objs[obj].heap = Some((was_array, true));
    Ok(())
}

/// A `delete` expression: `e` is the pointer, `array` marks `delete[]`,
/// and `virtual_dtor` says that the class the pointer points to has a
/// virtual destructor.
pub fn delete(m: &mut Machine, e: &Expr, array: bool, virtual_dtor: bool, dealloc: Option<&crate::sem::tree::Dealloc>) -> R<Val> {
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
    // NEW-22: the deallocation function that the program gives gets the
    // pointer to the storage, and the size and the alignment it asks for.
    if let Some(d) = dealloc.filter(|d| !is_library(m, d.f)) {
        let dynamic = m.objs[p.obj].ty.base_elem().class();
        if virtual_dtor && dynamic != e.ty.pointee().and_then(|t| t.class()) && dynamic_delete(m, p.obj) {
            return unsup("a delete through a base whose dynamic class has its own deallocation function");
        }
        m.objs[p.obj].alive = false;
        let mut vals = vec![Ret::Val(Val::Ptr(Ptr::whole(p.obj)))];
        if d.sized {
            vals.push(Ret::Val(Val::Int(crate::foundations::storage::run::size(m, &whole) as i128)));
        }
        if d.align {
            vals.push(Ret::Val(Val::Int(m.target.align_of(m.u, whole.base_elem()).unwrap_or(1) as i128)));
        }
        crate::machine::eval::call(m, d.f, vals)?;
        if let Some((a, _)) = m.objs[p.obj].heap {
            if m.objs[p.obj].host.is_some() {
                m.objs[p.obj].heap = Some((a, true));
            }
        }
        return Ok(Val::Void);
    }
    if m.objs[p.obj].host.is_some() {
        m.objs[p.obj].alive = false;
        m.objs[p.obj].heap = None;
        return free(m, &Ptr::whole(p.obj), array, "NEW-5").map(|_| Val::Void);
    }
    let o = &mut m.objs[p.obj];
    o.alive = false;
    o.heap = Some((was_array, true));
    Ok(Val::Void)
}

/// The dynamic class of the object that `new` created, or a base of it,
/// declares its own deallocation function (NEW-21).
fn dynamic_delete(m: &Machine, obj: usize) -> bool {
    fn declares(u: &crate::sem::Unit, c: crate::sem::types::ClassId) -> bool {
        let names = &u.scopes[u.classes[c].scope].names;
        names.contains_key("operatordelete") || names.contains_key("operatordelete[]") || u.bases(c).iter().any(|&(_, b)| declares(u, b))
    }
    m.objs[obj].ty.base_elem().class().is_some_and(|c| declares(m.u, c))
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
