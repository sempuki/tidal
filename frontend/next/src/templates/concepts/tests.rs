//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn concepts() {
    assert!(ok("template <class U> concept D = C<U> && true;", concept_definition));
    assert!(ok("requires C<T> || (sizeof(T) > 1)", requires_clause));
    assert!(ok("C<T> || true", constraint_or));
    assert!(ok("C<T> && true", constraint_and));
    assert!(ok("C<int>", type_constraint));
    assert!(!ok("D<int>", type_constraint));
    assert!(ok("requires (T t) { t + 1; typename T::type; { t } noexcept -> C; requires true; }", requires_expression));
    assert!(ok("f(x);", requirement));
    assert!(ok("x;", simple_requirement));
    assert!(ok("typename V<int>;", type_requirement));
    assert!(ok("{ x } -> C<int>;", compound_requirement));
    assert!(ok("requires C<T>;", nested_requirement));
}
