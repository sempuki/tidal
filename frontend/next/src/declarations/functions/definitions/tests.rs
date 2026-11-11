//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn definitions() {
    assert!(ok("int f(int x) { return x; }", function_definition));
    assert!(ok("= delete;", function_body));
    assert!(ok(": a(1) { }", function_body));
    assert!(parses("struct K { K() = default; int f() const & noexcept { return 1; } };"));
}
