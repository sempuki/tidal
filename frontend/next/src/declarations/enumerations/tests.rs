//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn enumerations() {
    assert!(ok("enum class E : int { a, b = 2, }", enum_def));
    assert!(ok("enum { x }", enum_def));
    assert!(ok("enum struct E : long;", enum_opaque));
    assert!(ok("enum class N::E", enum_head));
    assert!(ok("a [[deprecated]] = 1", enumerator));
}
