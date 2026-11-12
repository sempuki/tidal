//! The run part of `declarations/declarators`.

use crate::machine::{ub, Machine, Ptr, R};

/// CV-5: a change of an object that is defined `const`, as through a
/// cast that removes `const`. PRIM-8 is the case of a string literal.
pub fn write_const(m: &mut Machine, p: &Ptr) -> R<()> {
    let o = &m.objs[p.obj];
    if o.string {
        return crate::expressions::primary::run::write_string();
    }
    ub("CV-5", format!("a change of {}, which is defined const", o.name))
}
