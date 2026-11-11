//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn access_and_friends() {
    assert!(ok("public:", access_label));
    assert!(ok("private", access_specifier));
    assert!(ok("friend class S;", friend_decl));
    assert!(ok("friend void g(S);", friend_decl));
    assert!(ok("friend bool operator==(const S&, const S&) { return true; }", friend_decl));
}
