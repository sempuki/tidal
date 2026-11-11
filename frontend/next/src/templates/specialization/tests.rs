//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn specializations() {
    assert!(parses("template <class U> struct X {}; template <> struct X<int> { int y; };"));
    let _ = explicit_specialization;
}
