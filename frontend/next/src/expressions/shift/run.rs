//! The run part of `expressions/shift`.

use crate::machine::{ub, unsup, Machine, Val, R};
use crate::sem::types::QType;

/// `<<` and `>>` on the promoted left operand. SHF-7: a negative count,
/// or a count not less than the width.
pub fn shift(m: &mut Machine, left: bool, a: Val, b: Val, lt: &QType, _rt: &QType) -> R<Val> {
    let (Val::Int(x), Val::Int(n)) = (a, b) else { return unsup("a shift of these values") };
    let Some(k) = lt.int_kind() else { return unsup("a shift of this type") };
    let w = m.target.int_bits(k) as i128;
    if n < 0 {
        return ub("SHF-7", format!("the shift count {} is negative", n));
    }
    if n >= w {
        return ub("SHF-7", format!("the shift count {} is not less than the width {}", n, w));
    }
    // SHF rules: a left shift is modulo 2 to the width; a right shift of
    // a negative value rounds toward negative infinity.
    let r = if left { m.target.wrap(k, x << n) } else { x >> n };
    Ok(Val::Int(r))
}
