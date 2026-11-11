//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn enumerations() {
    assert!(ok("enum class E : int { a, b = 2, }", enum_def));
    assert!(ok("enum { x }", enum_def));
    assert!(ok("enum struct E : long;", enum_opaque));
    assert!(ok("enum class N::E", enum_head));
    assert!(ok("a [[deprecated]] = 1", enumerator));
}

#[test]
fn using_enum_declarations() {
    assert!(ok("using enum E;", using_enum_decl));
    assert!(ok("using enum N::E;", using_enum_decl));
    // G169 and G172: in a block, at namespace scope, and in a class, with
    // an opaque declaration in a class.
    assert!(parses("enum class E { a }; using enum E; struct S { using enum E; enum class K : short; }; void f() { using enum E; }"));
}
