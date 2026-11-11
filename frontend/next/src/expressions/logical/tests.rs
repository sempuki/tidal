//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn levels() {
    assert!(sx("a || b && c", logical_or_expr).unwrap().starts_with("(logical_or_expr"));
    assert!(sx("a && b", logical_and_expr).unwrap().starts_with("(logical_and_expr"));
}
