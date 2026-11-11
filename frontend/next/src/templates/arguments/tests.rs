//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

use crate::syntax::names::Qual;

#[test]
fn arguments() {
    assert!(ok("V<int, 3>", |p| template_id(p, Qual::None, false).map(|x| x.0)));
    assert!(ok("V<V<int>>", |p| template_id(p, Qual::None, false).map(|x| x.0)));
    assert!(ok("V<(1 > 2)>", |p| template_id(p, Qual::None, false).map(|x| x.0)));
    assert!(ok("V", template_name));
    assert!(ok("T, 1, Ts...", template_args));
    assert!(sx("T*", template_arg).unwrap().contains("(type_id"));
    assert!(sx("Pair", template_arg).unwrap().contains("(template_name"));
    assert!(sx("n + 1", template_arg).unwrap().contains("(add_expr"));
}
