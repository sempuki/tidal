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

#[test]
fn namespace_names_of_item_gr() {
    // G168: a namespace with no qualifier.
    assert!(ok("using namespace N;", using_directive));
    assert!(ok("using namespace ::N::inner;", using_directive));
    assert!(ok("namespace n = N;", namespace_alias));
    // An attribute after `namespace` (ATTR-19).
    assert!(ok("namespace [[deprecated]] tools {}", namespace_def));
    assert!(ok("namespace [[deprecated]] {}", namespace_def));
    assert!(ok("namespace a::inline b {}", namespace_def));
    assert!(parses("namespace lib { int x; } using namespace lib; int y = x;"));
}
