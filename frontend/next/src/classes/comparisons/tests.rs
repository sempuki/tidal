//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn comparison_shapes() {
    assert!(comparison_op("<=>") && !comparison_op("+"));
    let src = "struct K { bool operator==(const K&) const = default; };";
    let t = toks(src);
    let mut names = crate::sem::names::Declare::default();
    let u = crate::syntax::parse_unit(&t, &mut names, &[]).unwrap();
    let m = u[0].find("member_decl").unwrap();
    assert!(comparison_decl(m, &t) && defaulted_comparison(m, &t));
}
