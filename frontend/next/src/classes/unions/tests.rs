//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn unions() {
    assert!(ok("union U { int a; float b; }", union_def));
    assert!(ok("x : 3 = 1", bitfield));
    assert!(ok(": 0", bitfield));
    let t = toks("static union { int a; };");
    let mut names = crate::sem::names::Declare::default();
    let u = crate::syntax::parse_unit(&t, &mut names, &[]).unwrap();
    assert!(anonymous_union(&u[0], &t));
}
