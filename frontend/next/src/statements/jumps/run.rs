//! The run part of `statements/jumps`. `break` and `continue` name the
//! statement they act on (JMP-1, JMP-2), so the machine passes their
//! flow to it; the objects of each block left end there (JMP-9).

use crate::machine::{Flow, Machine, R};
use std::rc::Rc;

/// JMP-5: `goto` passes control to its label. Constant evaluation must
/// not reach a `goto` (CE-17).
pub fn goto(m: &mut Machine, label: &Rc<str>) -> R<Flow> {
    if m.constant {
        return crate::foundations::constant_evaluation::run::goto();
    }
    Ok(Flow::Goto(label.clone()))
}
