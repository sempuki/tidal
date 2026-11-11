//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn conversions() {
    assert!(ok("operator int*", conversion_function_id));
    assert!(ok("const S&", conversion_type));
    assert!(cvf_specifier("explicit"));
    let src = "struct K { explicit operator bool() const; };";
    let t = toks(src);
    let mut names = crate::sem::names::Declare::default();
    let u = crate::syntax::parse_unit(&t, &mut names, &[]).unwrap();
    let m = u[0].find("member_decl").unwrap();
    assert!(conversion_decl(m, &t));
}

#[test]
fn conversion_to_member_pointer() {
    // G177.
    assert!(ok("int S::*", conversion_type));
    assert!(ok("const int* const&", conversion_type));
    assert!(parses("struct B { int m; }; struct S { operator int B::*() const; };"));
}
