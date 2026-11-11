//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn members() {
    assert!(parses("struct S { }; struct X : S { int a = 1, b{2}; virtual void f() = 0; X() : a(1) { } };"));
    assert!(ok("struct Y : S { int a = 1, b{2}; virtual void f() = 0; }", class_def));
    assert!(ok("S", class_name));
    assert!(ok("static constexpr int n = 3;", member_decl));
    assert!(ok("* p", member_declarator));
    assert!(ok("= {1}", member_init));
    assert!(parses("struct Q { void f() { g(); } void g() { } };"));
}
