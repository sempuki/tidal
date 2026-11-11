//! Unit tests of `statements/index`'s productions.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn statements() {
    assert!(sx("x = 1;", statement).unwrap().contains("(expression_stmt"));
    assert!(sx("T* p = nullptr;", statement).unwrap().contains("(block_decl"));
    assert!(sx("[[fallthrough]];", statement).unwrap().contains("(attribute_spec"));
    assert!(sx("out: return;", statement).unwrap().contains("(labeled_stmt"));
    assert!(ok("while (x) { }", plain_stmt));
    assert!(ok("try { } catch (...) { }", plain_stmt));
}
