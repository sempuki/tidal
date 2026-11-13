//! The Syntax segment of `declarations/storage-linkage`.

use crate::syntax::{Node, PResult, Parser};

/// `storage_class: "static" | "extern" | "thread_local" | "mutable"`.
/// The parser also takes the keyword `register` here, so that analysis
/// reports STO-5 in place of a stop.
pub fn storage_class(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at_any(&["static", "extern", "thread_local", "mutable", "register"]) {
        let k = p.bump();
        return Ok(p.node("storage_class", sp, vec![k]));
    }
    Err(p.fail("a storage class"))
}
