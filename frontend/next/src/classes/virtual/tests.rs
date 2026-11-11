//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn virtual_parts() {
    assert!(ok("override final", virt_specifiers));
    assert!(ok("= 0", pure_specifier));
    assert!(!ok("= 1", pure_specifier));
    assert!(ok("dynamic_cast<S*>(p)", dynamic_cast));
}
