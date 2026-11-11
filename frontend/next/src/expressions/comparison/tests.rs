//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn levels() {
    assert!(sx("a < b == c", equality_expr).unwrap().starts_with("(equality_expr (relational_expr"));
    assert!(sx("a <=> b", three_way_expr).unwrap().starts_with("(three_way_expr"));
    assert!(sx("a >= b", relational_expr).unwrap().starts_with("(relational_expr"));
}
