//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn assignment() {
    assert!(sx("a = b = 1", assignment_expr).unwrap().starts_with("(assignment_expr"));
    assert!(ok("a += {1}", assignment_expr));
    assert!(sx("throw 1", assignment_expr).unwrap().starts_with("(throw_expr"));
    assert!(ok("<<=", assign_op));
}
