//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn heads_and_parameters() {
    assert!(ok("template <class U> struct X {};", template_decl));
    assert!(ok("template <class U, int N = 3> requires C<U>", template_head));
    assert!(ok("class... Us, int N, template <class> class TT", template_params));
    assert!(sx("typename U = int", template_param).unwrap().contains("(type_param"));
    assert!(ok("C U", type_param));
    assert!(ok("typename", type_key));
    assert!(ok("T* p", constant_param));
    assert!(ok("int... ns", constant_param));
    assert!(ok("template <class> class TT = V", template_template_param));
}

#[test]
fn template_template_default_with_template_keyword() {
    // G179.
    assert!(ok("template <class> class M = T::template box", template_template_param));
}
