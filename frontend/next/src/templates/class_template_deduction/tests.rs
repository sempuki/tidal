//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn guides() {
    assert!(ok("Pair(int, T) -> Pair<int, T>;", deduction_guide));
    assert!(ok("explicit Pair(T) -> Pair<T, T>;", deduction_guide));
}
