//! The declarations chapter. Its index page (`declarations/index.md`)
//! holds the start symbol and the general declaration productions, which
//! sit in `syntax.rs` beside this file.

pub mod alignment;
pub mod attributes;
pub mod auto;
pub mod declarators;
pub mod enumerations;
pub mod exception_specifications;
pub mod functions;
pub mod initialization;
pub mod linkage_specifications;
pub mod namespaces;
pub mod static_assert;
pub mod storage_linkage;
pub mod structured_bindings;
pub mod syntax;
pub mod type_names;

pub use syntax::*;

#[cfg(test)]
mod tests;
