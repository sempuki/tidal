//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn pointer_to_member_operators() {
    assert!(sx("o .* pm", pm_expr).unwrap().starts_with("(pm_expr"));
    assert!(ok("p ->* pm", pm_expr));
}
