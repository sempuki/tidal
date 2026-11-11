//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn handlers() {
    assert!(ok("try { f(); } catch (const T& e) { } catch (...) { }", try_block));
    assert!(!ok("try { }", try_block));
    assert!(ok("catch (int) { }", handler));
    assert!(ok("T* p", exception_decl));
    assert!(ok("try : a(1) { } catch (...) { }", function_try_block));
}
