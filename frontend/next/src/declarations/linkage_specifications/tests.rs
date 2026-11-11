//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn linkage() {
    assert!(parses("extern \"C\" { int f(); } extern \"C\" int g();"));
    let _ = linkage_spec;
}
