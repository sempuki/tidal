//! The check part of `foundations/evaluation-order`.

use crate::sem::check::Check;
use crate::sem::tree::{Expr, EK};
use crate::sem::EntityId;
use crate::source::Span;

/// The variables an expression modifies and reads, outside calls.
fn mods_reads(e: &Expr, mods: &mut Vec<EntityId>, reads: &mut Vec<EntityId>) {
    match &e.k {
        EK::Var(v) => reads.push(*v),
        EK::Assign(l, r) | EK::Compound(_, l, r, _) => {
            if let EK::Var(v) = l.k {
                mods.push(v);
            } else {
                mods_reads(l, mods, reads);
            }
            mods_reads(r, mods, reads);
        }
        EK::IncDec(_, _, x) => {
            if let EK::Var(v) = x.k {
                mods.push(v);
            } else {
                mods_reads(x, mods, reads);
            }
        }
        EK::Conv(_, x) | EK::Unary(_, x) | EK::Deref(x) | EK::AddrOf(x) => mods_reads(x, mods, reads),
        EK::Binary(_, a, b) | EK::PtrAdd(a, b, _) | EK::PtrDiff(a, b) | EK::Index(a, b) => {
            mods_reads(a, mods, reads);
            mods_reads(b, mods, reads);
        }
        // `&&`, `||`, `?:`, the comma operator, and calls sequence their
        // parts, so their accesses do not count here.
        _ => {}
    }
}

/// SEQ-7: an implementation should warn when the unsequenced operands of
/// a built-in operator both modify one variable, or one modifies it and
/// the other reads it.
pub fn unsequenced(cx: &mut Check, a: &Expr, b: &Expr, span: Span) {
    if !cx.wflag(&["-Wall", "-Wsequence-point", "-Wunsequenced"]) {
        return;
    }
    let (mut ma, mut ra, mut mb, mut rb) = (vec![], vec![], vec![], vec![]);
    mods_reads(a, &mut ma, &mut ra);
    mods_reads(b, &mut mb, &mut rb);
    if ma.iter().any(|v| mb.contains(v) || rb.contains(v)) || mb.iter().any(|v| ra.contains(v)) {
        cx.warning(span, "SEQ-7", "unsequenced modification and access to a variable");
    }
}
