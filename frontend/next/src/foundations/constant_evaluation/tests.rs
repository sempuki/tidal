//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn constant_expression() {
    assert!(ok("N < 3 ? 1 : 2", constant_expr));
    assert!(!ok("a = 1", constant_expr));
}
