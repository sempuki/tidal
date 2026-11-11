//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn instantiations() {
    assert!(parses("template <class U> struct X {}; template struct X<int>; extern template struct X<long>;"));
    let _ = explicit_instantiation;
}
