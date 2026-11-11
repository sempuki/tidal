//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn range_for() {
    assert!(ok("for (auto x : xs) { }", range_for_stmt));
    assert!(ok("for (T t = f(); auto& [a, b] : t) ;", range_for_stmt));
    assert!(ok("const T& [a, b]", range_decl));
    assert!(ok("{1, 2}", range_init));
}
