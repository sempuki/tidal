//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn declarators() {
    assert!(ok("*const* p", declarator));
    assert!(ok("S::* pm", declarator));
    assert!(ok("(*fp)(int)", declarator));
    assert!(ok("a[3][4]", declarator));
    assert!(ok("&&", ptr_part));
    assert!(ok("...args", direct_declarator));
    assert!(ok("N::operator==", declarator_id));
    assert!(ok("[]", array_suffix));
    assert!(ok("(int a, T b = 1, ...) const & noexcept -> int", function_suffix));
    assert!(ok("void", parameters));
    assert!(ok("volatile", cv));
    assert!(ok("&", ref_qualifier));
    assert!(ok("int (*)[3]", type_id));
    assert!(ok("(*)(int)", abstract_declarator));
    assert!(ok("[3]", abstract_direct));
}

#[test]
fn which_declarators_declare_functions() {
    let d = |src: &str| {
        let t = toks(src);
        let mut st = stub();
        let mut p = crate::syntax::Parser::new(&t, &mut st, &[]);
        declares_function(&declarator(&mut p).unwrap())
    };
    assert!(d("f(int)"));
    assert!(d("(f)(int)"));
    assert!(d("(*f(int))[3]"));
    assert!(!d("(*fp)(int)"));
    assert!(!d("a[3]"));
}
