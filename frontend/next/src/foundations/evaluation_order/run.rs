//! The run part of `foundations/evaluation-order`: the accesses of
//! unsequenced operands.

use crate::machine::eval::eval;
use crate::machine::{ub, Access, Machine, Val, R};
use crate::sem::tree::{BOp, Expr, EK};

/// Whether an operand modifies an object outside a call, so that it can
/// take part in a SEQ-4 conflict.
fn modifies(e: &Expr) -> bool {
    match &e.k {
        EK::Assign(..) | EK::Compound(..) | EK::IncDec(..) => true,
        EK::Conv(_, x) | EK::Unary(_, x) | EK::Deref(x) | EK::AddrOf(x) => modifies(x),
        EK::Binary(_, a, b) | EK::PtrAdd(a, b, _) | EK::PtrDiff(a, b) | EK::Index(a, b) | EK::Logical(_, a, b) | EK::Comma(a, b) => {
            modifies(a) || modifies(b)
        }
        EK::Cond(c, a, b) => modifies(c) || modifies(a) || modifies(b),
        _ => false,
    }
}

/// Two accesses conflict when they reach one memory location and at
/// least one writes it.
fn conflict(x: &Access, y: &Access) -> bool {
    x.obj == y.obj && (x.write || y.write) && (x.path.starts_with(&y.path) || y.path.starts_with(&x.path))
}

/// The values of the operands of a built-in binary operator, left first.
/// SEQ-4: the operands are unsequenced (the shifts excepted, SHF-8), so
/// a write in one and a read or a write of the same memory location in
/// the other is undefined. Accesses made inside a call are
/// indeterminately sequenced with the other operand, so only the
/// accesses at the operator's own frame count. Constant evaluation does
/// not watch.
pub fn operands(m: &mut Machine, op: BOp, l: &Expr, r: &Expr) -> R<(Val, Val)> {
    let watch = !m.constant && !matches!(op, BOp::Shl | BOp::Shr) && (modifies(l) || modifies(r));
    if !watch {
        return Ok((eval(m, l)?, eval(m, r)?));
    }
    let depth = m.frames.len();
    let mark = m.acc.len();
    m.seq_watch.push(depth);
    let res = (|| -> R<(Val, Val, usize)> {
        let a = eval(m, l)?;
        let mid = m.acc.len();
        let b = eval(m, r)?;
        Ok((a, b, mid))
    })();
    m.seq_watch.pop();
    let bad = match &res {
        Ok((_, _, mid)) => {
            let (x, y) = m.acc[mark..].split_at(mid - mark);
            x.iter().filter(|a| a.depth == depth).any(|a| y.iter().filter(|b| b.depth == depth).any(|b| conflict(a, b)))
        }
        Err(_) => false,
    };
    // An enclosing watch keeps the accesses for its own check.
    if !m.seq_watch.contains(&depth) {
        m.acc.truncate(mark);
    }
    let (a, b, _) = res?;
    if bad {
        return ub("SEQ-4", format!("unsequenced accesses to one memory location in the operands of '{}', at least one a write", op.spelling()));
    }
    Ok((a, b))
}
