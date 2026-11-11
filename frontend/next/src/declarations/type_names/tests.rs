//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn type_names() {
    assert!(ok("using U [[deprecated]] = const int*;", alias_decl));
    assert!(ok("struct N::S", elaborated_type));
    assert!(ok("class V<int>", elaborated_type));
    assert!(ok("union", class_key));
    let t = toks("typedef int I, *P; struct Fwd;");
    let mut names = crate::sem::names::Declare::default();
    let u = crate::syntax::parse_unit(&t, &mut names, &[]).unwrap();
    assert!(typedef_decl(&u[0], &t) && !typedef_decl(&u[1], &t));
    assert!(forward_class(&u[1], &t) && !forward_class(&u[0], &t));
    let t = toks("template <class T> struct X; template <> struct X<int>; struct Y;");
    let mut names = crate::sem::names::Declare::default();
    let u = crate::syntax::parse_unit(&t, &mut names, &[]).unwrap();
    assert!(specialization_class(&u[1], &t) && !specialization_class(&u[2], &t));
    assert!(!forward_class(&u[1], &t));
}
