//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn bindings() {
    assert!(ok("static const auto& [a, b] = p;", sb_decl));
    assert!(ok("auto [x] {p};", sb_decl));
    assert!(ok("(p)", sb_init));
}
