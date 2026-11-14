//! The run part of `expressions/lambdas`: a use of a variable that a
//! closure captured by reference after the variable's lifetime ended
//! (LAM-19).

use crate::machine::{ub, Machine, Ptr, R};
use crate::sem::tree::{Expr, EK};

/// LAM-19: `x` is the pointer member of a capture by reference, and the
/// object `p` that it points to has ended.
pub fn dangling(m: &Machine, x: &Expr, p: &Ptr) -> R<()> {
    let capture = match &x.k {
        EK::Conv(_, y) => matches!(&y.k, EK::Member(o, _) if o.ty.class().is_some_and(|c| m.u.closures.contains_key(&c))),
        _ => false,
    };
    match m.objs.get(p.obj) {
        Some(o) if capture && !o.alive && !o.is_static => ub("LAM-19", format!("a use of {}, which a lambda captured by reference, after its lifetime ended", o.name)),
        _ => Ok(()),
    }
}
