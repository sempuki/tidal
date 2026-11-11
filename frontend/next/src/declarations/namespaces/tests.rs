//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn namespaces() {
    assert!(parses("namespace a::inline b { int x; } inline namespace c { } namespace { }"));
    assert!(ok("a::inline b", ns_path));
    assert!(parses("namespace m { int y; } namespace n = ::m;"));
    assert!(ok("using N::x, typename T::y...;", using_decl));
    assert!(ok("using namespace ::N;", using_directive));
    let _ = (namespace_def, namespace_alias);
}
