//! The run part of `declarations/enumerations`.

use crate::machine::{ub, Machine, R};
use crate::sem::types::EnumId;

/// ENUM-6: a conversion to an enumeration of a value that is not one of
/// its values (ENUM-22) is undefined. With a fixed underlying type, the
/// value converts to that type first (ENUM-21), so every result is a
/// value of the enumeration.
pub fn check_value(m: &mut Machine, en: EnumId, v: i128) -> R<()> {
    if m.u.enums[en].fixed {
        return Ok(());
    }
    let (lo, hi) = super::check::value_range(m.u, en);
    if v < lo || v > hi {
        return ub("ENUM-6", format!("{} is not a value of the enumeration {}, whose values are {} to {}", v, m.u.enums[en].name, lo, hi));
    }
    Ok(())
}
