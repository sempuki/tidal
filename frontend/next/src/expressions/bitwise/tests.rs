//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn levels() {
    assert!(sx("a | b ^ c & d", or_expr).unwrap().starts_with("(or_expr"));
    assert!(sx("a & b", and_expr).is_some());
    assert!(sx("a ^ b", xor_expr).unwrap().starts_with("(xor_expr"));
}
