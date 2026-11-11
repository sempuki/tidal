//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn lists() {
    assert!(ok("{1, {2, 3}, xs..., }", braced_init));
    assert!(ok("{.a = 1, .b{2}}", braced_init));
    assert!(ok(".a = 1", init_clause));
    assert!(ok(".x", designator));
}
