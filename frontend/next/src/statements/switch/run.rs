//! The run part of `statements/switch`.

use crate::machine::exec::Seek;
use crate::machine::{Machine, Val};
use crate::sem::tree::{Stmt, StmtId, SK};

/// SW-1: control goes to the `case` label whose value equals the
/// condition, else to `default`, else past the statement.
pub fn choose(_m: &mut Machine, v: Val, cases: &[i128], body: &Stmt, id: StmtId) -> Option<Seek> {
    let x = v.int()?;
    if let Some(k) = cases.iter().position(|&c| c == x) {
        return Some(Seek::Case(id, Some(k)));
    }
    let d = Seek::Case(id, None);
    has(body, &d).then_some(d)
}

fn has(s: &Stmt, t: &Seek) -> bool {
    match &s.k {
        SK::Switch { .. } => false,
        _ => crate::machine::exec::contains(s, t),
    }
}
