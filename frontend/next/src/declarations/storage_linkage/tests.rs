//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn storage() {
    assert!(ok("thread_local", storage_class));
    // The parser takes `register`, and analysis reports STO-5.
    assert!(ok("register", storage_class));
}
