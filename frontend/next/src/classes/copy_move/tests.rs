//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn copy_and_move_shapes() {
    let src = "struct K { K(const K&, int = 0); K(K&&); K& operator=(const K&); K& operator=(K&&); };";
    let t = toks(src);
    let mut names = crate::sem::names::Declare::default();
    let u = crate::syntax::parse_unit(&t, &mut names, &[]).unwrap();
    let ms: Vec<&crate::syntax::Node> = u[0].find("class_def").unwrap().children("member_decl").collect();
    assert!(copy_ctor(ms[0], &t, "K") && !move_ctor(ms[0], &t, "K"));
    assert!(move_ctor(ms[1], &t, "K"));
    assert!(copy_assign(ms[2], &t, "K") && !move_assign(ms[2], &t, "K"));
    assert!(move_assign(ms[3], &t, "K"));
    assert!(copy_param(&["const".to_string(), "K".to_string(), "&".to_string()], "K"));
    let p = ms[0].find("parameters").unwrap().children("parameter").nth(1).unwrap();
    assert!(default_param(p, &t));
}
