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
    match init {
        Init::Ctor(f, args) => {
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
        }
        Init::Expr(e) if matches!(&e.k, crate::sem::tree::EK::List(i) if constructs(i)) => {
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
            for (k, t) in tys.iter().enumerate() {
                let mut q = p.clone();
                q.path.push(k as i64);
                q.array = ty.is_array();
                // VC-9: the initialization of each element of an array is a
                // full expression, whose temporaries end with it.
                let from = m.temps.len();
                match items.get(k) {
                    Some(i) => construct(m, &q, t, i)?,
                    None => set(m, &q, m.zero(t))?,
                }
                if ty.is_array() {
                    crate::machine::exec::end_temps(m, from)?;
                }
            }
            Ok(())
        }
        _ => {
            let v = value(m, ty, init)?;
            set(m, p, v)
        }
    }
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
    let Some(c) = m.u.func(f).and_then(|x| x.class) else { return Ok(()) };
    let ty = QType::new(Type::Class(c));
    match mi {
        crate::sem::MemInits::Delegate(i) => construct(m, &this, &ty, i),
        crate::sem::MemInits::Fields(inits) => {
            // Each member's initializer is a full expression (VC-9).
            let tys = m.u.field_types(c);
            // CTOR-12: until the last base is initialized, no member
            // function runs on the object.
            let last_base = m.u.bases(c).last().map(|&(i, _)| i);
            if last_base.is_some() {
                m.early.push((this.obj, this.path.clone()));
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
                crate::machine::exec::end_temps(m, from)?;
            }
            if last_base.is_some_and(|i| i + 1 == tys.len()) {
                m.early.pop();
            }
            Ok(())
        }
    }
}

/// The value of an object of type `ty` from its initializer (INIT-2 to
/// INIT-4): none for default-initialization of a scalar (INIT-1), zero
/// for value-initialization, the converted value, or the list.
pub fn value(m: &mut Machine, ty: &QType, init: &Init) -> R<Val> {
    match init {
        // A constructor runs on an object: a temporary, whose value is
        // the result.
        Init::Ctor(..) => {
            // The object that the value initializes owns it, so the
            // object the constructor ran on ends without destruction.
            let p = crate::machine::eval::temp(m, ty, m.uninit(ty))?;
            construct(m, &p, ty, init)?;
            m.temps.retain(|&o| o != p.obj);
            m.objs[p.obj].alive = false;
            Ok(m.objs[p.obj].val.clone())
        }
        Init::Default => Ok(m.uninit(ty)),
        Init::Zero => Ok(m.zero(ty)),
        Init::Expr(e) => {
            if let Some(v) = indeterminate_copy(m, ty, e) {
                return v;
            }
            m.elide = true;
            let v = eval_any(m, e)?;
            m.elide = false;
            match (&ty.t, v) {
                (Type::Float(k), v @ (Val::Float(_) | Val::Ext(_))) => Ok(crate::foundations::conversions::run::to_float(v, *k)),
                (_, v) => Ok(v),
            }
        }
        Init::Ref(e) => {
            let p = place(m, e)?;
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
        Init::List(items) if ty.is_class() => {
            let c = ty.class().unwrap();
            let tys = m.u.field_types(c);
            let mut out = Vec::new();
            for (k, t) in tys.iter().enumerate() {
                let v = match items.get(k) {
                    Some(i) => value(m, t, i)?,
                    None => m.zero(t),
                };
                // BF-10: a bit-field holds the value wrapped to its width.
                let width = m.u.classes[c].fields.get(k).and_then(|&f| m.u.field(f)).and_then(|x| x.width);
                out.push(match width {
                    Some(w) => crate::classes::unions::run::wrap(m, w, t, v),
                    None => v,
                });
            }
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
