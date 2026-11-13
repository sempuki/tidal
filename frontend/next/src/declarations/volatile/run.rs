//! The run part of `declarations/volatile`.

use crate::machine::{ub, Machine, Ptr, R};
use crate::sem::types::QType;

/// VOL-6: an access to a volatile object through a glvalue that is not
/// volatile is undefined.
pub fn check_access(m: &mut Machine, p: &Ptr, ty: &QType) -> R<()> {
    match m.objs.get(p.obj) {
        Some(o) if o.ty.is_volatile() && !ty.is_volatile() && !m.constructing.contains(&p.obj) => {
            ub("VOL-6", format!("an access to the volatile object {} through a glvalue that is not volatile", o.name))
        }
        _ => Ok(()),
    }
}
