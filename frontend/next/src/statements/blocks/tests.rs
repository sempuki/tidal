//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn blocks() {
    assert!(ok("f();", expression_stmt));
    assert!(ok(";", null_stmt));
    assert!(ok("{ int x = 1; x++; end: }", compound_stmt));
    assert!(ok("a: b: ;", labeled_stmt));
    assert!(ok("case 1:", label));
    assert!(ok("[[likely]] default:", label));
}
