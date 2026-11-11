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

#[test]
fn arguments_of_item_gr() {
    // G178: a braced list as an argument.
    assert!(sx("{3, 4}", template_arg).unwrap().starts_with("(template_arg (braced_init"));
    // G171: an operator function's and a literal operator's template-id.
    assert!(ok("operator()<int>", |p| template_id(p, crate::syntax::names::Qual::None, false).map(|r| r.0)));
    assert!(ok("operator\"\"_n<'4', '0'>", |p| template_id(p, crate::syntax::names::Qual::None, false).map(|r| r.0)));
}
