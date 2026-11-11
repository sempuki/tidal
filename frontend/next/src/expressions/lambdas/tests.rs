//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn lambdas() {
    assert!(ok("[] { return 1; }", lambda_expr));
    assert!(ok("[=, &x, this, y = 2] (int a) mutable noexcept -> int { return a; }", lambda_expr));
    assert!(ok("[]<class T>(T u) requires C<T> { return u; }", lambda_expr));
    assert!(ok("<class U>", lambda_template));
    assert!(ok("&, x", captures));
    assert!(ok("=", capture_default));
    assert!(ok("...xs = 1", capture));
    assert!(ok("*this", capture));
    assert!(ok("consteval", lambda_specifier));
}

#[test]
fn lambda_attributes() {
    // An attribute after the captures with no template head, and after a
    // template head.
    assert!(ok("[] [[nodiscard]] (int x) { return x; }", lambda_expr));
    assert!(ok("[] [[nodiscard]] { return 1; }", lambda_expr));
    assert!(ok("[] <class T> [[nodiscard]] (T x) { return x; }", lambda_expr));
}
