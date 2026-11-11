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
