//! `declarations/functions/definitions`: the page's module (`frontend/DESIGN-rewrite.md`, section 3).
//! Build step 2 holds its Syntax segment in `syntax.rs`; build step 3 adds its
//! check and run parts.

pub mod syntax;
pub mod run;
pub mod check;

#[cfg(test)]
mod tests;
