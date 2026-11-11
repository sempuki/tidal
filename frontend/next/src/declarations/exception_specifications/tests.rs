//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn noexcept_forms() {
    assert!(ok("noexcept", noexcept_specifier));
    assert!(ok("noexcept(sizeof(T) > 1)", noexcept_specifier));
}
