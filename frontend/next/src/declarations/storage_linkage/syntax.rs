//! The Syntax segment of `declarations/storage-linkage`.

use crate::syntax::{Node, PResult, Parser};

/// `storage_class: "static" | "extern" | "thread_local" | "mutable"`.
pub fn storage_class(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at_any(&["static", "extern", "thread_local", "mutable"]) {
        let k = p.bump();
        return Ok(p.node("storage_class", sp, vec![k]));
    }
    Err(p.fail("a storage class"))
}
