//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn postfix() {
    assert!(sx("a[1](2).b->c++", postfix_expr).unwrap().starts_with("(postfix_expr (postfix_expr"));
    assert!(ok("o.template f<int>()", postfix_expr));
    assert!(ok("g(1, {2}, xs...)", postfix_expr));
    assert!(ok("1", argument));
    assert!(ok("a, b...", expression_list));
}
