//! The run part of `foundations/objects`: the lifetime of an object and
//! the access checks of OBJ-2.

use crate::foundations::constant_evaluation;
use crate::machine::{ub, Machine, Ptr, R};
use crate::sem::types::QType;

/// OBJ-2: a read of an object outside its lifetime.
pub fn check_read(m: &mut Machine, p: &Ptr, _ty: &QType) -> R<()> {
    match m.objs.get(p.obj) {
        Some(o) if !o.alive => ub("OBJ-2", format!("a read of {} after its lifetime ended", o.name)),
        _ => Ok(()),
    }
}

/// OBJ-2: a write outside the lifetime; a write to a `const` object
/// goes to `declarations::declarators` (CV-5).
pub fn check_write(m: &mut Machine, p: &Ptr, ty: &QType) -> R<()> {
    let Some(o) = m.objs.get(p.obj) else { return Ok(()) };
    if !o.alive {
        return ub("OBJ-2", format!("a write to {} after its lifetime ended", o.name));
    }
    let (konst, string) = (o.konst, o.string);
    if m.constant {
        constant_evaluation::run::check_write(m, p)?;
    }
    let _ = ty;
    if konst || string {
        return crate::declarations::declarators::run::write_const(m, p);
    }
    Ok(())
}

/// OBJ-2: a scalar read before it has a value. A variable with automatic
/// storage and no initializer is the case of INIT-1, which its page
/// names.
pub fn no_value(m: &mut Machine, p: &Ptr) -> R<crate::machine::Val> {
    let name = m.objs.get(p.obj).map(|o| o.name.to_string()).unwrap_or_default();
    crate::declarations::initialization::forms::run::read_no_value(&name)
}

/// A place outside its object: an element past the end of its array.
pub fn outside<T>(_m: &mut Machine, _p: &Ptr) -> R<T> {
    ub("OBJ-2", "an access outside the object (past the end of an array)")
}
