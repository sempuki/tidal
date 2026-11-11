//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn decltype_forms() {
    assert!(ok("decltype(x + 1)", decltype_spec));
    assert!(ok("decltype(auto)", decltype_spec));
}
