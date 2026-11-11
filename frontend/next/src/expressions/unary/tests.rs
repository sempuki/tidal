//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn unary() {
    assert!(sx("sizeof(T)", unary_expr).unwrap().contains("(type_id"));
    assert!(sx("sizeof x", unary_expr).unwrap().starts_with("(unary_expr sizeof"));
    assert!(ok("sizeof...(Ts)", unary_expr));
    assert!(ok("alignof(T)", unary_expr));
    assert!(!ok("alignof(x + 1)", unary_expr));
    assert!(ok("noexcept(f())", unary_expr));
    assert!(ok("-*p", unary_expr));
    assert!(ok("::new T", unary_expr));
}
