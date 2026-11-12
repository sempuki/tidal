//! The run part of `declarations/storage-linkage`.

use crate::machine::{ub, Machine, R};
use crate::sem::EntityId;

/// STO-9: control reaches the declaration of a block-scope static
/// variable again while its initialization runs.
pub fn enter_init(m: &mut Machine, id: EntityId) -> R<()> {
    if m.initializing.contains(&id) {
        return ub("STO-9", format!("the initialization of '{}' reaches its own declaration again", m.u.entities[id].name));
    }
    m.initializing.push(id);
    Ok(())
}
