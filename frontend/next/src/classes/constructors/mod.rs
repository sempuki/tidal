//! `classes/constructors`: the page's module (`frontend/DESIGN-rewrite.md`, section 3).
//! Build step 2 holds its Syntax segment in `syntax.rs`; the check and
//! run parts come with later steps.

pub mod check;
pub mod run;
pub mod syntax;

#[cfg(test)]
mod tests;
