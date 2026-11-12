//! The check part of `overloading/overload-resolution` for calls whose
//! candidates are functions and whose arguments are not classes (build
//! step 3). Templates, members, and class arguments come with later
//! steps; a call that needs them is unsupported.

use crate::overloading::conversion_sequences::check::{better, sequence, Ics};
use crate::sem::check::Check;
use crate::sem::tree::Expr;
use crate::sem::EntityId;
use crate::source::Span;

/// The candidate that a call chooses, or None after an error.
pub fn resolve(cx: &mut Check, cands: &[EntityId], args: &[Expr], span: Span, name: &str) -> Option<EntityId> {
    let mut viable: Vec<(EntityId, Vec<Ics>)> = Vec::new();
    for &f in cands {
        let Some(func) = cx.u.func(f) else { continue };
        let ty = func.ty.clone();
        let defaults = func.defaults.iter().filter(|d| d.is_some()).count();
        // OVL-22: one parameter for each argument, with defaults or an
        // ellipsis for the rest.
        let n = ty.params.len();
        if args.len() > n && !ty.variadic {
            continue;
        }
        if args.len() + defaults < n {
            let first_default = func.defaults.iter().position(|d| d.is_some()).unwrap_or(n);
            if args.len() < first_default {
                continue;
            }
        }
        let mut seqs = Vec::new();
        let mut ok = true;
        for (k, a) in args.iter().enumerate() {
            if k >= n {
                seqs.push(Ics { rank: crate::overloading::conversion_sequences::check::Rank::Conversion, qual: false, binds: None, ellipsis: true });
                continue;
            }
            match sequence(cx, a, &ty.params[k]) {
                Some(s) => seqs.push(s),
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            viable.push((f, seqs));
        }
    }
    if viable.is_empty() {
        if cands.len() == 1 {
            // One candidate: the call's own checks name the fault
            // (PFX-5 for the count, the parameter's initialization for a
            // type).
            return Some(cands[0]);
        }
        cx.error(span, "OVL-35", format!("no function '{}' fits the arguments", name));
        return None;
    }
    // OVL-29: the best viable function is better than every other.
    let wins = |a: &Vec<Ics>, b: &Vec<Ics>| -> bool {
        let mut some = false;
        for (x, y) in a.iter().zip(b.iter()) {
            if better(y, x) {
                return false;
            }
            if better(x, y) {
                some = true;
            }
        }
        some
    };
    let mut best = 0;
    for i in 1..viable.len() {
        if wins(&viable[i].1, &viable[best].1) {
            best = i;
        }
    }
    for i in 0..viable.len() {
        if i != best && !wins(&viable[best].1, &viable[i].1) {
            cx.error(span, "OVL-36", format!("the call of '{}' is ambiguous", name));
            return None;
        }
    }
    Some(viable[best].0)
}
