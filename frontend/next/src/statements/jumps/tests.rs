//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn jumps() {
    assert!(ok("return {1, 2};", jump_stmt));
    assert!(ok("goto out;", jump_stmt));
    assert!(!ok("break out;", jump_stmt));
}
