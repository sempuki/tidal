//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn noexcept_forms() {
    assert!(ok("noexcept", noexcept_specifier));
    assert!(ok("noexcept(sizeof(T) > 1)", noexcept_specifier));
}

#[test]
fn exception_specs() {
    // G175: `throw()` (EXS-5, Q-0077).
    assert!(ok("throw()", exception_spec));
    assert!(ok("noexcept(true)", exception_spec));
    assert!(!ok("throw(int)", exception_spec));
    assert!(parses("void f() throw(); void g() noexcept;"));
}
