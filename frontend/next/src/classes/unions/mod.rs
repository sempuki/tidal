//! `classes/unions`: the page's module (`frontend/DESIGN-rewrite.md`, section 3).
//! Build step 2 holds its Syntax segment in `syntax.rs`; build step 4 adds
//! its check and run parts.

pub mod check;
pub mod run;
pub mod syntax;

#[cfg(test)]
mod tests;
