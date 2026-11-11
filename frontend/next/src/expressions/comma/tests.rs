//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn comma() {
    assert!(sx("a, b, c", expression).unwrap().starts_with("(expression"));
}
