//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn operator_names() {
    assert!(ok("operator+=", operator_function_id));
    assert!(ok("operator()", operator_function_id));
    assert!(ok("new[]", overloadable_operator));
    assert!(ok("operator\"\" _km", literal_operator_id));
    assert!(ok("operator\"\"_km", literal_operator_id));
}
