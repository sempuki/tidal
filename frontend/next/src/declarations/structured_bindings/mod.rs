//! `declarations/structured-bindings`: the page's module (`frontend/DESIGN-rewrite.md`, section 3).
//! The check part covers an array; a class type comes with build step 4.

pub mod check;
pub mod syntax;

#[cfg(test)]
mod tests;
