//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn throws() {
    assert!(ok("throw", throw_expr));
    assert!(ok("throw T{}", throw_expr));
}
