//! Unit tests of `declarations/index`'s productions.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn units_and_declarations() {
    assert!(ok("int x; ; namespace m { }", translation_unit));
    assert!(sx("int f() { return 0; }", declaration).unwrap().contains("(function_definition"));
    assert!(sx("int a = 1, *b;", declaration).unwrap().contains("(simple_decl"));
    assert!(ok("using U = int;", block_decl));
    assert!(ok("static const int n = 1;", simple_decl));
    assert!(parses("template <class U> U id(U u) { return u; } int k = id<int>(1);"));
}

#[test]
fn specifiers() {
    let mut st = Specs::default();
    assert!(sx("static constexpr unsigned long long", decl_specifiers).is_some());
    assert!(ok("const T", type_specifiers));
    assert!(sx("inline", |p| decl_specifier(p, &mut Specs::default())).is_some());
    assert!(sx("S", |p| type_specifier(p, &mut Specs::default())).is_some());
    assert!(ok("N::S", simple_type));
    assert!(ok("V<int>", simple_type));
    assert!(!ok("x", simple_type));
    st.named = true;
    let _ = st;
    // `T x`: the second name is the declarator.
    let s = sx("T x;", simple_decl).unwrap();
    assert!(s.contains("(init_declarator"), "{}", s);
}

#[test]
fn statement_or_declaration() {
    // STMT-6: `T(x);` is a declaration; `g(x);` an expression.
    assert!(sx("int f() { T(x); return 0; }", declaration).unwrap().contains("(block_decl"));
    let e = sx("int f() { g(x); return 0; }", declaration).unwrap();
    assert!(e.contains("(expression_stmt"), "{}", e);
    let _ = init_declarator;
}
