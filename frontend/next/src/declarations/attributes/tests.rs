//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn attributes() {
    assert!(ok("[[nodiscard, deprecated(\"x\")]]", attribute_spec));
    assert!(ok("[[using gnu: hot]]", attribute_spec));
    assert!(ok("[[]]", attribute_spec));
    assert!(ok("ns::name(1, (2), [3])", attribute));
    assert!(ok("(a {b})", attribute_args));
    assert!(ok("{ x }", balanced_token));
    assert!(ok("+", token));
    assert!(!ok(")", token));
}
