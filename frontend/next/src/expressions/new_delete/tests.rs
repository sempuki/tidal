//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn new_and_delete() {
    assert!(ok("new int", new_expr));
    assert!(ok("new (buf) T(1)", new_expr));
    assert!(ok("new (T)", new_expr));
    assert!(ok("::new T[n][3]{}", new_expr));
    assert!(ok("(p, q)", placement));
    assert!(ok("const int * const", new_type));
    assert!(ok("[n][2]", new_bounds));
    assert!(ok("{1, 2}", new_init));
    assert!(ok("delete[] p", delete_expr));
    assert!(ok("::delete p", delete_expr));
}

#[test]
fn attributes_after_type_and_bound() {
    assert!(ok("int [[deprecated]]", new_type));
    assert!(ok("[3] [[deprecated]]", new_bounds));
    assert!(ok("[n] [[a]] [4] [[b]]", new_bounds));
}

#[test]
fn new_ptr_takes_attributes_after_cv() {
    assert!(ok("* const [[a]]", new_ptr));
    assert!(ok("int * [[a]] * const", new_type));
}
