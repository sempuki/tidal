//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn ifs() {
    assert!(ok("if (x) f(); else g();", if_stmt));
    assert!(ok("if constexpr (int n = 1; n) { }", if_stmt));
    assert!(ok("if !consteval { } else { }", if_stmt));
    assert!(!ok("if consteval f();", if_stmt));
    assert!(ok("T x = 1;", init_stmt));
    assert!(ok("x", condition));
    assert!(ok("T* p = q", cond_decl));
    assert!(ok("f();", branch));
    assert!(ok("{ }", block));
}

#[test]
fn structured_binding_init() {
    // G174.
    assert!(sx("auto [a, b] = s;", init_stmt).unwrap().starts_with("(init_stmt (sb_decl"));
    assert!(parses("struct P { int x, y; }; int f(P s) { if (auto [a, b] = s; b > 2) return a; for (auto [a, b] = s; a < b; ++a) {} return 0; }"));
}
