//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn typename_specifiers() {
    assert!(ok("typename T::type", typename_specifier));
    assert!(ok("typename T::template rebind<int>", typename_specifier));
    assert!(!ok("typename x", typename_specifier));
}

#[test]
fn template_keyword_with_no_list() {
    // G179 (Q-0226).
    assert!(ok("typename T::template box", typename_specifier));
}
