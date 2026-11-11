//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn typename_specifiers() {
    assert!(ok("typename T::type", typename_specifier));
    assert!(ok("typename T::template rebind<int>", typename_specifier));
    assert!(!ok("typename x", typename_specifier));
}
