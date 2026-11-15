//! The run part of `declarations/initialization/forms`: the value a new
//! object gets from its initializer.

use crate::machine::eval::{eval_any, place};
use crate::machine::{ub, Machine, Ptr, Val, R};
use crate::sem::tree::Init;
use crate::sem::types::{QType, Type};

/// INIT-1: a copy of an `unsigned char` that has no value, into an
/// `unsigned char`, through casts to `unsigned char`, either branch of
/// `?:`, and the right operand of a comma, is not undefined, and the copy
/// has no value. Constant evaluation rejects the copy (CE-22). None when
/// `e` has a value or is not such a copy.
pub fn indeterminate_copy(m: &mut Machine, ty: &QType, e: &crate::sem::tree::Expr) -> Option<R<Val>> {
    use crate::sem::tree::{Conv, EK};
    let uchar = |t: &QType| t.unqual().t == Type::Int(crate::sem::types::IntK::UChar);
    if !uchar(ty) || !uchar(&e.ty) {
        return None;
    }
    match &e.k {
        EK::Conv(Conv::LToR, x) if uchar(&x.ty) => {
            let p = match place(m, x) {
                Ok(p) => p,
                Err(s) => return Some(Err(s)),
            };
            if !matches!(m.slot(&p), Some(Val::Uninit)) || m.objs.get(p.obj).is_some_and(|o| !o.alive) {
                return None;
            }
            if m.constant {
                return Some(crate::foundations::constant_evaluation::run::no_value(m));
            }
            Some(Ok(Val::Uninit))
        }
        EK::Conv(Conv::Arith | Conv::NoOp, x) if uchar(&x.ty) => indeterminate_copy(m, ty, x),
        EK::Cond(c, a, b) => {
            let pick = match crate::machine::eval::eval(m, c) {
                Ok(v) => v.truthy(),
                Err(s) => return Some(Err(s)),
            };
            let r = indeterminate_copy(m, ty, if pick { a } else { b });
            r.or_else(|| Some(crate::machine::eval::eval(m, if pick { a } else { b })))
        }
        EK::Comma(a, b) => {
            if let Err(s) = crate::machine::eval::eval_discard(m, a) {
                return Some(Err(s));
            }
            indeterminate_copy(m, ty, b).or_else(|| Some(crate::machine::eval::eval(m, b)))
        }
        _ => None,
    }
}

/// An initializer that runs a constructor somewhere in it, so that the
/// object is initialized in its place.
pub fn constructs(init: &Init) -> bool {
    match init {
        Init::Ctor(..) => true,
        Init::List(xs) => xs.iter().any(constructs),
        // A default member initializer that uses `this` needs the object.
        Init::Expr(e) if e.any(&|x| matches!(x.k, crate::sem::tree::EK::InitThis)) => true,
        Init::Expr(e) => matches!(&e.k, crate::sem::tree::EK::List(i) if constructs(i)),
        _ => false,
    }
}

/// Initialize the object at `p` of type `ty` in its place: a
/// constructor runs with the object as `this`, a prvalue of a list or a
/// constructor call initializes the object itself (the copy is elided),
/// and each member or element of a list is initialized in its place.
pub fn construct(m: &mut Machine, p: &Ptr, ty: &QType, init: &Init) -> R<()> {
    use crate::machine::Ret;
    virtual_base_constant(m, ty)?;
    match init {
        Init::Ctor(f, args) => {
            // A constructor's arguments are expressions, whose
            // temporaries the declared variable does not keep.
            let ext = m.extending.take();
            let r = (|| -> R<()> {
                let mut vals = vec![Ret::Place(p.clone())];
                for a in args {
                    if a.by_ref {
                        vals.push(Ret::Place(place(m, &a.e)?));
                    } else {
                        vals.push(Ret::Val(eval_any(m, &a.e)?));
                    }
                }
                m.constructing.push(p.obj);
                let r = crate::machine::eval::call(m, *f, vals);
                m.constructing.pop();
                r.map(|_| ())
            })();
            m.extending = ext;
            r
        }
        Init::Expr(e) if matches!(&e.k, crate::sem::tree::EK::List(i) if constructs(i) || has_vbases(m, ty)) => {
            let crate::sem::tree::EK::List(i) = &e.k else { unreachable!() };
            construct(m, p, ty, i)
        }
        // UNI-4: a union with the listed member active; a member whose
        // initializer is default-initialization stays inactive.
        Init::List(items) if crate::classes::unions::run::union_of(m.u, ty).is_some() => {
            let c = ty.class().unwrap();
            set(m, p, m.uninit(ty))?;
            let tys = m.u.field_types(c);
            for (k, i) in items.iter().enumerate() {
                if matches!(i, Init::Default) || k >= tys.len() {
                    continue;
                }
                crate::classes::unions::run::begin_member(m, p, c, k)?;
                let mut q = p.clone();
                q.path.push(k as i64);
                q.array = false;
                construct(m, &q, &tys[k], i)?;
            }
            Ok(())
        }
        Init::List(items) if ty.is_class() || ty.is_array() => {
            let tys: Vec<QType> = match &ty.t {
                Type::Class(c) => m.u.field_types(*c),
                Type::Array(e, Some(n)) => (0..*n).map(|_| (**e).clone()).collect(),
                _ => Vec::new(),
            };
            let mut built = Vec::new();
            let mut extended = Vec::new();
            let class = ty.class().is_some_and(|c| uses_init_this(m, c));
            if class {
                m.init_this.push(p.clone());
            }
            let r = (|| -> R<()> {
                // DER-7: a complete object initializes its virtual bases
                // first.
                if let Some(c) = ty.class() {
                    vbases_first(m, p, c, items, &mut built)?;
                }
                for (k, t) in tys.iter().enumerate() {
                    let mut q = p.clone();
                    q.path.push(k as i64);
                    q.array = ty.is_array();
                    // VC-9: the initialization of each element of an array
                    // is a full expression, whose temporaries end with it.
                    let from = m.temps.len();
                    let kept = m.extending.as_ref().map_or(0, |v| v.len());
                    match items.get(k) {
                        Some(i) => construct(m, &q, t, i)?,
                        None => set(m, &q, m.zero(t))?,
                    }
                    // UNW-2: a temporary that a reference member keeps
                    // ends in reverse order with the members built.
                    let now: Vec<usize> = m.extending.as_ref().map_or(Vec::new(), |v| v[kept..].to_vec());
                    for o in now {
                        built.push((Ptr::whole(o), m.objs[o].ty.clone()));
                        extended.push(o);
                    }
                    built.push((q, t.clone()));
                    if ty.is_array() {
                        crate::machine::exec::end_temps(m, from)?;
                    }
                }
                Ok(())
            })();
            if class {
                m.init_this.pop();
            }
            if matches!(r, Err(crate::machine::Stop::Throw(_))) {
                m.temps.retain(|o| !extended.contains(o));
                if let Some(v) = m.extending.as_mut() {
                    v.retain(|o| !extended.contains(o));
                }
            }
            undo(m, r, built)
        }
        _ => {
            let v = value(m, ty, init)?;
            set(m, p, v)
        }
    }
}

/// A default member initializer of class `c` uses `this`, so a list for
/// the class makes its object the one that `EK::InitThis` names. A list
/// of another class, such as a closure in such an initializer, leaves
/// `this` to the enclosing object.
fn uses_init_this(m: &Machine, c: crate::sem::types::ClassId) -> bool {
    m.u.classes[c].fields.iter().any(|&f| m.u.field(f).and_then(|x| x.init.as_ref()).is_some_and(|i| i.any(&|e| matches!(e.k, crate::sem::tree::EK::InitThis))))
}

/// Store an initial value: no check of `const`, which an initialization
/// does not modify.
fn set(m: &mut Machine, p: &Ptr, v: Val) -> R<()> {
    let v = crate::classes::unions::run::store(m, p, v);
    match m.slot(p) {
        Some(s) => {
            *s = v;
            Ok(())
        }
        None => crate::foundations::objects::run::outside(m, p),
    }
}

/// The member initializers of the constructor `f`, on the object of the
/// innermost frame (CTOR-5, CTOR-6).
pub fn mem_inits(m: &mut Machine, f: crate::sem::EntityId, mi: &crate::sem::MemInits) -> R<()> {
    let Some(this) = m.frames.last().and_then(|fr| fr.this.clone()) else { return crate::machine::unsup("a constructor without its object") };
    // The default member initializers name the object as `this`.
    m.init_this.push(this.clone());
    let r = mem_inits_of(m, f, mi, this);
    m.init_this.pop();
    r
}

fn mem_inits_of(m: &mut Machine, f: crate::sem::EntityId, mi: &crate::sem::MemInits, this: Ptr) -> R<()> {
    let Some(c) = m.u.func(f).and_then(|x| x.class) else { return Ok(()) };
    let ty = QType::new(Type::Class(c));
    match mi {
        crate::sem::MemInits::Delegate(i) => construct(m, &this, &ty, i),
        crate::sem::MemInits::Fields(inits) => {
            // Each member's initializer is a full expression (VC-9).
            let tys = m.u.field_types(c);
            // CTOR-12: until the last base is initialized, no member
            // function runs on the object.
            let last_base = m.u.nv_bases(c).last().map(|&(i, _)| i);
            let vbases = !m.u.classes[c].vbases.is_empty();
            if last_base.is_some() || vbases {
                m.early.push((this.obj, this.path.clone()));
            }
            let mut built = Vec::new();
            let r = (|| -> R<()> {
            vbases_first(m, &this, c, inits, &mut built)?;
            if vbases && last_base.is_none() {
                m.early.pop();
            }
            let union = crate::classes::unions::run::union_of(m.u, &ty).is_some();
            for (k, t) in tys.iter().enumerate() {
                if Some(k) == last_base.map(|i| i + 1) {
                    m.early.pop();
                }
                let mut q = this.clone();
                q.path.push(k as i64);
                q.array = false;
                let from = m.temps.len();
                // UNI-4: a union's member without an initializer stays
                // inactive.
                if union {
                    match inits.get(k) {
                        Some(i) if !matches!(i, Init::Default) => {
                            crate::classes::unions::run::begin_member(m, &this, c, k)?;
                            construct(m, &q, t, i)?;
                        }
                        _ => {}
                    }
                    crate::machine::exec::end_temps(m, from)?;
                    continue;
                }
                match inits.get(k) {
                    Some(i) => construct(m, &q, t, i)?,
                    None => set(m, &q, m.zero(t))?,
                }
                built.push((q, t.clone()));
                crate::machine::exec::end_temps(m, from)?;
            }
            if last_base.is_some_and(|i| i + 1 == tys.len()) {
                m.early.pop();
            }
            Ok(())
            })();
            undo(m, r, built)
        }
    }
}

/// UNW-2: when the initialization of an object throws, the parts that
/// it built end, last first.
fn undo(m: &mut Machine, r: R<()>, built: Vec<(Ptr, QType)>) -> R<()> {
    if !matches!(r, Err(crate::machine::Stop::Throw(_))) {
        return r;
    }
    crate::exceptions::throw::run::unwind(m, r, |m| {
        for (q, t) in built.into_iter().rev() {
            if crate::classes::constructors::run::needs(m, &t) {
                crate::classes::constructors::run::destroy(m, &q, &t)?;
            }
        }
        Ok(())
    })
}

/// The value of an object of type `ty` from its initializer (INIT-2 to
/// INIT-4): none for default-initialization of a scalar (INIT-1), zero
/// for value-initialization, the converted value, or the list.
pub fn value(m: &mut Machine, ty: &QType, init: &Init) -> R<Val> {
    value_in(m, ty, init, true)
}

/// `value` for a complete object, or for a base subobject, which holds
/// no virtual bases (`classes::derived::run`).
fn value_in(m: &mut Machine, ty: &QType, init: &Init, complete: bool) -> R<Val> {
    if !matches!(init, Init::Ref(_)) {
        virtual_base_constant(m, ty)?;
    }
    match init {
        // A constructor runs on an object: a temporary, whose value is
        // the result.
        Init::Ctor(..) => {
            // The object that the value initializes owns it, so the
            // object the constructor ran on ends without destruction.
            let p = crate::machine::eval::temp(m, ty, m.uninit(ty))?;
            let ext = m.extending.take();
            let r = construct(m, &p, ty, init);
            m.extending = ext;
            r?;
            m.temps.retain(|&o| o != p.obj);
            m.objs[p.obj].alive = false;
            Ok(m.objs[p.obj].val.clone())
        }
        Init::Default => Ok(m.uninit_as(ty, complete)),
        Init::Zero => Ok(m.zero_as(ty, complete)),
        Init::Expr(e) => {
            if let Some(v) = indeterminate_copy(m, ty, e) {
                return v;
            }
            m.elide = true;
            let ext = m.extending.take();
            let v = eval_any(m, e);
            m.extending = ext;
            let v = v?;
            m.elide = false;
            match (&ty.t, v) {
                (Type::Float(k), v @ (Val::Float(_) | Val::Ext(_))) => Ok(crate::foundations::conversions::run::to_float(v, *k)),
                (_, v) => Ok(v),
            }
        }
        Init::Ref(e) => {
            let ext = m.extending.take();
            let p = place(m, e);
            m.extending = ext;
            let p = p?;
            // REF-4: a reference member of the declared aggregate keeps
            // the temporary that it binds.
            if crate::declarations::initialization::references::run::extends(e) && m.temps.contains(&p.obj) {
                if let Some(v) = m.extending.as_mut() {
                    v.push(p.obj);
                }
            }
            Ok(Val::Ptr(p))
        }
        Init::List(items) if crate::classes::unions::run::union_of(m.u, ty).is_some() => {
            let tys = m.u.field_types(ty.class().unwrap());
            let mut out = vec![Val::Inactive; tys.len()];
            for (k, i) in items.iter().enumerate() {
                if !matches!(i, Init::Default) && k < tys.len() {
                    out[k] = value(m, &tys[k], i)?;
                }
            }
            Ok(Val::Agg(out))
        }
        // A list whose default member initializers use `this` is made in
        // a temporary object, whose value is the result.
        Init::List(_) if ty.is_class() && constructs(init) => {
            let p = crate::machine::eval::temp(m, ty, m.uninit(ty))?;
            let ext = m.extending.take();
            let r = construct(m, &p, ty, init);
            m.extending = ext;
            r?;
            m.temps.retain(|&o| o != p.obj);
            m.objs[p.obj].alive = false;
            Ok(m.objs[p.obj].val.clone())
        }
        Init::List(items) if ty.is_class() => {
            let c = ty.class().unwrap();
            let tys = m.u.field_types(c);
            // DER-7: the virtual bases of the complete object come first, and
            // a base subobject has none.
            let mut vals = Vec::new();
            let vbases = if complete { m.u.classes[c].vbases.clone() } else { Vec::new() };
            for (j, &v) in vbases.iter().enumerate() {
                let vt = QType::new(Type::Class(v));
                vals.push(match items.get(tys.len() + j) {
                    Some(i) => value(m, &vt, i)?,
                    None => m.zero_as(&vt, false),
                });
            }
            let mut out = Vec::new();
            for (k, t) in tys.iter().enumerate() {
                let base = m.u.classes[c].fields.get(k).and_then(|&f| m.u.field(f)).is_some_and(|x| x.base);
                let v = match items.get(k) {
                    Some(i) => value_in(m, t, i, !base)?,
                    None => m.zero_as(t, !base),
                };
                // BF-10: a bit-field holds the value wrapped to its width.
                let width = m.u.classes[c].fields.get(k).and_then(|&f| m.u.field(f)).and_then(|x| x.width);
                out.push(match width {
                    Some(w) => crate::classes::unions::run::wrap(m, w, t, v),
                    None => v,
                });
            }
            out.extend(vals);
            Ok(Val::Agg(out))
        }
        Init::List(items) => {
            let (Type::Array(et, Some(n))) = &ty.t else { return crate::machine::unsup("a list for a type that is not an array") };
            let mut out = Vec::new();
            for k in 0..*n as usize {
                out.push(match items.get(k) {
                    Some(i) => value(m, et, i)?,
                    None => m.zero(et),
                });
            }
            Ok(Val::Agg(out))
        }
    }
}

/// The type is a class with a virtual base.
fn has_vbases(m: &Machine, ty: &QType) -> bool {
    ty.class().is_some_and(|c| !m.u.classes[c].vbases.is_empty())
}

/// DER-7: the constructor of the complete object at `p` of class `c`
/// initializes each virtual base from its initializer in `inits`, after
/// those of the data members. A base subobject leaves them to the
/// complete object.
fn vbases_first(m: &mut Machine, p: &Ptr, c: crate::sem::types::ClassId, inits: &[Init], built: &mut Vec<(Ptr, QType)>) -> R<()> {
    if m.u.classes[c].vbases.is_empty() || !crate::classes::derived::run::is_complete(m, p) {
        return Ok(());
    }
    let n = m.u.classes[c].fields.len();
    for (j, v) in m.u.classes[c].vbases.clone().into_iter().enumerate() {
        let vt = QType::new(Type::Class(v));
        let mut q = p.clone();
        q.path.push((n + j) as i64);
        q.array = false;
        let from = m.temps.len();
        match inits.get(n + j) {
            Some(i) => construct(m, &q, &vt, i)?,
            None => set(m, &q, m.zero_as(&vt, false))?,
        }
        built.push((q, vt));
        crate::machine::exec::end_temps(m, from)?;
    }
    Ok(())
}

/// CE-2, DER-12: no constructor of a class with a virtual base is
/// constexpr, so a constant evaluation creates no object of one.
fn virtual_base_constant(m: &Machine, ty: &QType) -> R<()> {
    match ty.base_elem().class() {
        Some(c) if m.constant && !m.u.classes[c].vbases.is_empty() => crate::foundations::constant_evaluation::run::not_constexpr(&format!("{}::{}", m.u.classes[c].name, m.u.classes[c].name)),
        _ => Ok(()),
    }
}

/// INIT-1: a read of a variable with automatic storage before it has a
/// value.
pub fn read_no_value<T>(name: &str) -> R<T> {
    ub("INIT-1", format!("a read of {} before it has a value", name))
}

/// CTOR-12: a call of the member function `f` on an object whose
/// constructor has not initialized all its bases yet.
pub fn before_bases<T>(m: &Machine, f: crate::sem::EntityId) -> R<T> {
    ub("CTOR-12", format!("a call of the member function '{}' before the bases of its object are initialized", m.u.entities[f].name))
}
