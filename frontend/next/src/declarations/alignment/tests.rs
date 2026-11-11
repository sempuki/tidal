//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn alignas_forms() {
    assert!(ok("alignas(8)", alignment_specifier));
    assert!(ok("alignas(T)", alignment_specifier));
    assert!(ok("alignas(Ts...)", alignment_specifier));
}
