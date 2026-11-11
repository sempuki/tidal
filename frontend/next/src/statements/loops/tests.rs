//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn loops() {
    assert!(ok("while (int k = f()) { }", while_stmt));
    assert!(ok("do x--; while (x);", do_stmt));
    assert!(ok("for (int i = 0; i < n; ++i) { }", for_stmt));
    assert!(ok("for (;;) ;", for_stmt));
}
