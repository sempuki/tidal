//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn conditional() {
    assert!(sx("c ? a : b", conditional_expr).unwrap().starts_with("(conditional_expr"));
    assert!(ok("c ? a , b : d", conditional_expr));
    assert!(!ok("c ? a", conditional_expr));
}
