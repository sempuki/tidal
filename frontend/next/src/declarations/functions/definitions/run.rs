//! The run part of `declarations/functions/definitions`.

use crate::machine::{ub, Machine, Ret, Val, R};
use crate::sem::EntityId;

/// FN-3: control reaches the end of a function that returns a value.
/// FN-8: the end of `main` returns 0.
pub fn flow_off_end(m: &mut Machine, id: EntityId) -> R<Ret> {
    if m.u.main() == Some(id) {
        return Ok(Ret::Val(Val::Int(0)));
    }
    ub("FN-3", format!("control reaches the end of '{}', which returns a value", m.u.entities[id].name))
}
