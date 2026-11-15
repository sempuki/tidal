//! `templates/concepts`: the page's module (`frontend/DESIGN-rewrite.md`, section 3).
//! Build step 2 holds its Syntax segment in `syntax.rs`, and build step
//! 5 its Check segment in `check.rs`.

pub mod check;
pub mod syntax;

#[cfg(test)]
mod tests;
