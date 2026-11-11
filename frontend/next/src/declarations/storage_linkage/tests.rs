//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn storage() {
    assert!(ok("thread_local", storage_class));
    assert!(!ok("register", storage_class));
}
