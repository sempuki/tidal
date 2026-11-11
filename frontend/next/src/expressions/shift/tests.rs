//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn shifts() {
    assert!(sx("a << 1 >> 2", shift_expr).unwrap().starts_with("(shift_expr (shift_expr"));
}
