//! The statements chapter. Its index page holds `statement` and
//! `plain_stmt`, which sit in `syntax.rs` beside this file.

pub mod blocks;
pub mod if_stmt;
pub mod jumps;
pub mod loops;
pub mod range_for;
pub mod switch;
pub mod syntax;

pub use syntax::*;

#[cfg(test)]
mod tests;
