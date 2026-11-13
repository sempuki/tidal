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

#[test]
fn base_types() {
    // G167 and G170: a template-id, a pack of template-ids, `decltype`,
    // and an attribute before the base.
    assert!(ok(": V<int>", base_clause));
    assert!(ok(": public N::V<T>...", base_clause));
    assert!(ok("decltype(x)", base_type));
    assert!(ok("[[deprecated]] public S", base_spec));
    assert!(sx("V<int>", base_type).unwrap().starts_with("(base_type (template_id"));
    assert!(parses("template <class T> struct B {}; struct D : B<int> { D() : B<int>() {} };"));
    assert!(parses("struct B {}; B b; struct D : decltype(b) { D() : decltype(b){} {} };"));
    // The class template being defined, named in its own base clause.
    assert!(parses("template <class D> struct C {}; template <class T> struct I : C<I<T>> {};"));
}
