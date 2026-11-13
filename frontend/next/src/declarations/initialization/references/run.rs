//! The run part of `declarations/initialization/references`: which
//! temporary a reference variable keeps alive.

use crate::machine::Machine;
use crate::sem::tree::{Cat, Conv, Expr, EK};

/// REF-4: whether the initializer reaches a temporary that it makes
/// itself, through parentheses, the operands of `?:`, the right operand
/// of a comma, a cast with no user-defined conversion, or a subscript of
/// an array temporary. A returned reference or a reference that a
/// parameter holds reaches no such temporary (REF-5).
fn extends(e: &Expr) -> bool {
    match &e.k {
        EK::Conv(Conv::Materialize, _) => true,
        // The machine holds a function that a reference names in an
        // object of its own, which lives as long as the reference.
        EK::Func(_) => true,
        EK::Conv(Conv::NoOp | Conv::ArrayDecay, x) => extends(x),
        EK::Cond(_, a, b) => extends(a) || extends(b),
        EK::Comma(_, b) => extends(b),
        EK::Index(a, _) | EK::Member(a, _) => extends(a),
        _ => e.cat == Cat::PR,
    }
}

/// The temporaries of a reference variable's initializer, made from
/// `from` on: the one the reference binds to lives as long as the
/// reference (REF-4) and is returned; the others end with the full
/// expression (REF-5).
pub fn kept_temporary(m: &mut Machine, init: &Expr, bound: usize, from: usize) -> crate::machine::R<Option<usize>> {
    if m.keep_temps {
        let all: Vec<usize> = m.temps.drain(from..).collect();
        if let Some(b) = m.frames.last_mut().and_then(|f| f.blocks.last_mut()) {
            b.extend(all.iter().copied().filter(|&o| o != bound));
        }
        return Ok(all.contains(&bound).then_some(bound));
    }
    let keep = extends(init) && m.temps[from..].contains(&bound);
    let others: Vec<usize> = m.temps.drain(from..).filter(|&o| !(keep && o == bound)).collect();
    crate::classes::constructors::run::end_objects(m, others)?;
    Ok(keep.then_some(bound))
}
