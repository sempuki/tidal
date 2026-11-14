//! The run part of `foundations/constant-evaluation`: the limits that
//! constant evaluation adds to the machine. Each limit stops the
//! evaluation with `Stop::Limit` and its rule; analysis reports the
//! rule of the place that needed a constant with this rule as its cause.

use crate::machine::eval::init_value;
use crate::machine::{limit, Machine, Ptr, Val, R};
use crate::sem::types::QType;
use crate::sem::{EntityId, Var};

/// CE-10: the step limit of one evaluation.
pub fn too_many_steps<T>() -> R<T> {
    limit("CE-10", "the evaluation takes too many steps")
}

/// CE-10: the recursion limit of one evaluation.
pub fn too_deep<T>() -> R<T> {
    limit("CE-10", "the evaluation nests calls too deeply")
}

/// CE-3: a read of a variable that the evaluation did not create and
/// that is not usable in constant expressions; CE-12: a volatile read.
pub fn check_read(m: &mut Machine, p: &Ptr, ty: &QType) -> R<()> {
    let Some(o) = m.objs.get(p.obj) else { return Ok(()) };
    if !o.usable {
        // CV-11: a const object whose initializer is not constant, or a
        // volatile object.
        let rule = if o.ty.is_volatile() || o.ty.is_const() { "CV-11" } else { "CE-3" };
        return limit(rule, format!("{} is not usable in constant expressions", o.name));
    }
    if ty.v || o.ty.v {
        return limit("CE-12", format!("a read of the volatile object {}", o.name));
    }
    // CE-3: a mutable member of an object that the evaluation did not
    // create is not usable.
    if !o.own && through_mutable(m, p) && !inactive_mutable(m, p) {
        return limit("CE-3", "a read of a mutable member of an object from outside the evaluation");
    }
    Ok(())
}

/// CE-22: a copy of a whole union whose active member is not mutable,
/// which reads none of its members.
fn inactive_mutable(m: &mut Machine, p: &Ptr) -> bool {
    let Some(c) = m.type_at(p).and_then(|t| crate::classes::unions::run::union_of(m.u, &t)) else { return false };
    let active = match m.slot(p) {
        Some(crate::machine::Val::Agg(xs)) => xs.iter().position(|x| *x != crate::machine::Val::Inactive),
        _ => None,
    };
    !active.and_then(|k| m.u.classes[c].fields.get(k)).and_then(|&f| m.u.field(f)).is_some_and(|x| x.mutable)
}

/// The place goes through a mutable data member, or reads a whole class
/// object that has one.
fn through_mutable(m: &Machine, p: &Ptr) -> bool {
    fn has_mutable(m: &Machine, t: &QType) -> bool {
        match &t.t {
            crate::sem::types::Type::Array(e, _) => has_mutable(m, e),
            crate::sem::types::Type::Class(c) => m.u.classes[*c].fields.iter().any(|&f| m.u.field(f).is_some_and(|x| x.mutable || has_mutable(m, &x.ty))),
            _ => false,
        }
    }
    let mut t = m.objs[p.obj].ty.clone();
    for &i in &p.path {
        match &t.t.clone() {
            crate::sem::types::Type::Class(c) => {
                let Some(&f) = m.u.classes[*c].fields.get(i as usize) else { return false };
                let Some(x) = m.u.field(f) else { return false };
                if x.mutable {
                    return true;
                }
                t = x.ty.clone();
            }
            crate::sem::types::Type::Array(e, _) => t = (**e).clone(),
            _ => return false,
        }
    }
    has_mutable(m, &t)
}

/// CE-5: a change of an object that existed before the evaluation.
pub fn check_write(m: &mut Machine, p: &Ptr) -> R<()> {
    match m.objs.get(p.obj) {
        Some(o) if !o.own => limit("CE-5", format!("a change of {}, which the evaluation did not create", o.name)),
        _ => Ok(()),
    }
}

/// CE-22: a read of a scalar that has no value.
pub fn no_value<T>(_m: &mut Machine) -> R<T> {
    limit("CE-22", "a read of an object that has no value")
}

/// CE-4: `reinterpret_cast`.
pub fn reinterpret<T>() -> R<T> {
    limit("CE-4", "a reinterpret_cast")
}

/// CE-4: a conversion from `void*` to a pointer to an object type.
pub fn from_void_pointer<T>() -> R<T> {
    limit("CE-4", "a conversion from a pointer to void to a pointer to an object type")
}

/// CE-28: a floating-point division by zero.
pub fn float_divide_by_zero<T>() -> R<T> {
    limit("CE-28", "a floating-point division by zero")
}

/// CE-15: `<`, `>`, `<=`, or `>=` on pointers to unrelated objects.
pub fn unrelated_compare<T>() -> R<T> {
    limit("CE-15", "a comparison of pointers to unrelated objects")
}

/// CE-13: `==` on the addresses of two string literals.
pub fn string_compare<T>() -> R<T> {
    limit("CE-13", "an equality comparison of two string literals' addresses")
}

/// CE-2: a call of a function that is not `constexpr` or `consteval`.
pub fn not_constexpr<T>(name: &str) -> R<T> {
    limit("CE-2", format!("a call of '{}', which is not constexpr", name))
}

/// CE-16: a call of a `constexpr` function with no definition yet.
pub fn not_defined<T>(name: &str) -> R<T> {
    limit("CE-16", format!("'{}' has no definition at this point", name))
}

/// CE-17: a `goto`, or a static or thread variable of a block.
pub fn goto<T>() -> R<T> {
    limit("CE-17", "a goto statement")
}

/// The object of a variable with static storage during constant
/// evaluation. A variable usable in constant expressions (CE-3) gets its
/// value from its initializer; another variable is a place that the
/// evaluation cannot read (CE-3) or change (CE-5).
pub fn static_var(m: &mut Machine, id: EntityId, v: &Var) -> R<Ptr> {
    let ent = &m.u.entities[id];
    let name = ent.name.clone();
    let usable = usable_in_constant_expressions(m, v);
    if v.param {
        return limit("CE-30", format!("the parameter '{}' of the enclosing function", name));
    }
    if v.storage == crate::sem::Storage::Automatic && !usable {
        return limit(not_usable_rule(v), format!("'{}' is not usable in constant expressions", name));
    }
    if v.ty.is_ref() {
        let (true, Some(crate::sem::tree::Init::Ref(e))) = (usable, &v.init) else {
            return limit("CE-3", format!("the reference '{}' is not usable in constant expressions", name));
        };
        let saved = std::mem::replace(&mut m.static_temps, v.storage != crate::sem::Storage::Automatic);
        let temps = m.temps.len();
        let p = crate::machine::eval::place(m, e);
        m.static_temps = saved;
        let p = p?;
        // CE-3: a temporary that the reference extends is usable in
        // constant expressions only if its type is const. Another
        // evaluation can neither read nor change it.
        if m.temps[temps.min(m.temps.len())..].contains(&p.obj) && !v.ty.nonref().is_const() {
            let o = &mut m.objs[p.obj];
            o.usable = false;
            o.own = false;
            o.name = format!("the temporary that '{}' extends, which is not const,", name).into();
        }
        m.global_refs.insert(id, p.clone());
        return Ok(p);
    }
    if !usable {
        let val = m.uninit(&v.ty);
        let o = m.new_obj(&v.ty, val, &format!("'{}'", name));
        m.objs[o].own = false;
        m.objs[o].usable = false;
        m.objs[o].is_static = true;
        m.globals.insert(id, o);
        return Ok(Ptr::whole(o));
    }
    if m.initializing.contains(&id) {
        return limit("CE-3", format!("'{}' is read in its own initializer", name));
    }
    m.initializing.push(id);
    let frames = std::mem::replace(&mut m.frames, vec![Default::default()]);
    let val = match &v.init {
        Some(i) => init_value(m, &v.ty, i),
        None => Ok(m.zero(&v.ty)),
    };
    m.frames = frames;
    m.initializing.retain(|&x| x != id);
    let val = val?;
    let o = m.new_obj(&v.ty, val, &format!("'{}'", name));
    m.objs[o].own = false;
    // A usable local keeps automatic storage, so a reference or a
    // pointer to it is not a constant expression (CE-14).
    m.objs[o].is_static = v.storage != crate::sem::Storage::Automatic;
    m.globals.insert(id, o);
    Ok(Ptr::whole(o))
}

/// CE-3, or CV-11 for a `const` variable whose initializer is not
/// constant and for a `volatile` variable.
pub fn not_usable_rule(v: &Var) -> &'static str {
    if v.ty.is_volatile() || (v.ty.is_const() && !v.constexpr && !v.ty.is_ref()) {
        "CV-11"
    } else {
        "CE-3"
    }
}

/// CE-17: the evaluation reaches the declaration of a static or thread
/// variable of a block that is not usable in constant expressions.
pub fn reach_static_local(m: &mut Machine, id: EntityId) -> R<()> {
    let Some(v) = m.u.var(id) else { return Ok(()) };
    if !usable_in_constant_expressions(m, v) {
        return limit("CE-17", format!("the static variable '{}' of a block", m.u.entities[id].name));
    }
    Ok(())
}

/// CE-3 with CV-11: a `constexpr` variable, or a `const` variable of
/// integral or enumeration type that is not volatile and has a constant
/// initializer.
pub fn usable_in_constant_expressions(_m: &Machine, v: &Var) -> bool {
    if v.constexpr {
        return true;
    }
    let t = v.ty.nonref();
    if v.ty.is_ref() {
        return v.const_init;
    }
    t.c && !t.v && (t.is_integral() || t.is_enum()) && v.const_init
}

/// What a constant evaluation gave: a value, or the rule that stopped
/// it, with its message.
pub fn value_of(r: R<Val>) -> Result<Val, (&'static str, String)> {
    match r {
        Ok(v) => Ok(v),
        Err(crate::machine::Stop::Limit(rule, msg)) => Err((rule, msg)),
        // CE-9: an operation with undefined behavior.
        Err(crate::machine::Stop::Undefined(rule, msg)) => Err((rule, format!("undefined behavior: {}", msg))),
        Err(crate::machine::Stop::Unsupported(msg)) => Err(("", msg)),
        Err(crate::machine::Stop::Exit(_)) => Err(("CE-2", "a call that ends the program".into())),
    }
}
