//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn the_page_holds_the_preprocessing_productions() {
    assert_eq!(include_directive("#include \"x.h\""), Some("x.h"));
    assert!(pragma_once("#pragma once"));
}
