//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn placeholders() {
    assert!(ok("auto", placeholder));
    assert!(ok("decltype(auto)", placeholder));
    assert!(ok("C<int> auto", placeholder));
    let t = toks("auto a = 1, b(2);");
    let mut names = crate::sem::names::Declare::default();
    let u = crate::syntax::parse_unit(&t, &mut names, &[]).unwrap();
    assert!(auto_var_decl(&u[0], &t));
    let d = u[0].find("init_declarator").unwrap();
    assert!(auto_declarator(d, &t) && auto_init(d.child("init").unwrap(), &t));
}
