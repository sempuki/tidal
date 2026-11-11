//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn forms() {
    assert!(ok("= 1", init));
    assert!(ok("(1, 2)", init));
    assert!(ok("{1}", init));
    assert!(ok("= {1}", init));
}
