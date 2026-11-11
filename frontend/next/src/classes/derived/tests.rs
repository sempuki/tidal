//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn bases_and_heads() {
    assert!(ok(": public S, virtual private A", base_clause));
    assert!(ok("virtual protected N::S", base_spec));
    assert!(ok("struct X final : S", class_head));
    let t = toks("using S::S;");
    let mut st = stub();
    let mut p = crate::syntax::Parser::new(&t, &mut st, &[]);
    let n = crate::declarations::namespaces::syntax::using_decl(&mut p).unwrap();
    assert!(inheriting_decl(&n, &t));
}
