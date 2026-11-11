//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn operators() {
    assert!(sx("a * b / c % d", mul_expr).unwrap().starts_with("(mul_expr (mul_expr (mul_expr"));
    assert!(!ok("a * ", mul_expr));
}
