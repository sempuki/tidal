//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn static_asserts() {
    assert!(ok("static_assert(sizeof(int) == 4, \"int\");", static_assert_decl));
    assert!(ok("static_assert(true);", static_assert_decl));
}
