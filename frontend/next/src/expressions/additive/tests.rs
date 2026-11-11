//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn left_to_right() {
    assert_eq!(sx("a - b + c", add_expr).unwrap(), "(add_expr (add_expr (primary (id_expression (unqualified_id a))) - (primary (id_expression (unqualified_id b)))) + (primary (id_expression (unqualified_id c))))");
    assert!(ok("a * b + c", add_expr));
}
