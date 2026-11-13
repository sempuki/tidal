//! The run part of `statements/range-for`.

use crate::machine::{Machine, Ptr, R};
use crate::sem::EntityId;

/// RFOR-1: each pass declares the loop variable from the next element.
/// Analysis admits a variable of the element's type or a reference to
/// it, so the element's value or place is the variable's.
pub fn bind_element(m: &mut Machine, var: EntityId, elem: Ptr) -> R<()> {
    let Some(v) = m.u.var(var) else { return Ok(()) };
    let ty = v.ty.clone();
    if ty.is_ref() {
        if let Some(f) = m.frames.last_mut() {
            f.refs.insert(var, elem);
        }
        return Ok(());
    }
    // The read goes through the element, so it keeps the element's
    // qualifiers (a volatile element is read as volatile).
    let et = m.type_at(&elem).unwrap_or_else(|| ty.unqual());
    let val = crate::foundations::values::run::read(m, &elem, &et)?;
    let o = m.new_obj(&ty, val, &m.u.entities[var].name.clone());
    m.objs[o].konst = ty.is_const();
    if let Some(f) = m.frames.last_mut() {
        f.vars.insert(var, o);
        if let Some(b) = f.blocks.last_mut() {
            b.push(o);
        }
    }
    Ok(())
}
