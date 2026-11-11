//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn switches() {
    assert!(ok("switch (int k = f(); k) { case 1: break; default: ; }", switch_stmt));
}
