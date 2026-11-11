//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn casts() {
    assert!(sx("(T) x", cast_expr).unwrap().starts_with("(cast_expr ( (type_id"));
    // `x` is no type, so this is a parenthesized expression.
    assert!(sx("(x) + y", crate::expressions::additive::syntax::add_expr).unwrap().starts_with("(add_expr"));
    assert!(ok("static_cast<int>(x)", named_cast));
    assert!(ok("reinterpret_cast", cast_keyword));
    assert!(ok("T(1, 2)", functional_cast));
    assert!(ok("T{1}", functional_cast));
    assert!(!ok("x(1)", functional_cast));
    assert!(ok("unsigned", cast_type));
}

#[test]
fn cast_types_of_item_gr() {
    // A class template's template-id is a cast type (G166); a function
    // template's is not, so the tokens stay a call.
    assert!(sx("V<int>{1}", functional_cast).unwrap().starts_with("(functional_cast (cast_type (template_id"));
    assert!(ok("N::V<int>(1)", functional_cast));
    assert!(!ok("f<int>(1)", functional_cast));
    // A typename specifier is a cast type.
    assert!(ok("typename T::type{3}", functional_cast));
    assert!(ok("typename T::template wrap<int>{5}", functional_cast));
}
