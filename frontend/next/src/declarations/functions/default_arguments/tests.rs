//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn parameters() {
    assert!(ok("int x = 1", |p| parameter(p, false)));
    assert!(ok("this S& self", |p| parameter(p, false)));
    assert!(ok("T = {}", |p| parameter(p, false)));
}

#[test]
fn attribute_before_parameter() {
    // G170.
    assert!(ok("[[maybe_unused]] int p", |p| parameter(p, false)));
}
