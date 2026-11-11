//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn constructors() {
    assert!(ok("explicit(true)", explicit_spec));
    assert!(ok(": a(1), S{2}, Ts(x)...", ctor_init));
    assert!(ok("a{}", mem_init));
    assert!(ok("N::S", mem_init_id));
    assert!(ctor_specifier("constexpr") && !ctor_specifier("virtual"));
    assert!(dtor_specifier("virtual"));
}

#[test]
fn shapes() {
    let src = "struct K { explicit K(int) { } ~K() { } void g(K* k) { k->~K(); } };";
    let t = toks(src);
    let mut names = crate::sem::names::Declare::default();
    let u = crate::syntax::parse_unit(&t, &mut names, &[]).unwrap();
    let members: Vec<&crate::syntax::Node> = {
        let cd = u[0].find("class_def").unwrap();
        cd.children("member_decl").collect()
    };
    assert!(ctor_decl(members[0], &t, "K") && ctor_def(members[0], &t, "K"));
    assert!(!ctor_decl(members[1], &t, "K"));
    assert!(dtor_decl(members[1], &t, "K"));
    let call = members[2].find("expression_stmt").unwrap().find("postfix_expr").unwrap();
    assert!(dtor_call(call, &t));
}

#[test]
fn member_initializer_names() {
    assert!(sx("x", mem_init_id).unwrap() == "(mem_init_id x)");
    assert!(ok("V<T>", mem_init_id));
    assert!(ok("decltype(x)", mem_init_id));
    assert!(ok("N::S", mem_init_id));
}

#[test]
fn destructor_calls_by_template_id_or_decltype() {
    for src in ["p->~V<T>()", "t.~decltype(t)()", "s.~S()"] {
        let t = toks(src);
        let mut st = stub();
        let mut p = crate::syntax::Parser::new(&t, &mut st, &[]);
        let n = crate::expressions::postfix::syntax::postfix_expr(&mut p).unwrap();
        assert!(dtor_call(&n, &t), "{}", src);
    }
}
