//! Helpers for the unit tests of the `syntax.rs` parts.

use super::names::{Stub, TKind};
use super::{parse_unit, Node, PResult, Parser};
use crate::diag::Diags;
use crate::lex::{lex_file, Token};

/// The tokens of a source text.
pub fn toks(src: &str) -> Vec<Token> {
    let mut d = Diags::default();
    lex_file(src, None, &mut d).toks
}

/// The stub oracle of the tests: `T`, `S`, and `A` are types, `V` and
/// `Pair` are class templates, `f` is a function template, `v` a
/// variable template, `C` a concept, and `N` a namespace.
pub fn stub() -> Stub {
    let mut s = Stub::new(
        &["T", "S", "A", "B"],
        &[("V", TKind::Class), ("Pair", TKind::Class), ("f", TKind::Function), ("v", TKind::Variable), ("C", TKind::Concept)],
    );
    s.namespaces = vec!["N", "std"];
    s
}

/// Parse all of `src` with `f` and the stub oracle. Gives the tree as an
/// S-expression, or `None` when `f` fails or leaves tokens.
pub fn sx(src: &str, f: fn(&mut Parser) -> PResult<Node>) -> Option<String> {
    sx_with(src, f, &mut stub())
}

pub fn sx_with(src: &str, f: fn(&mut Parser) -> PResult<Node>, names: &mut Stub) -> Option<String> {
    let t = toks(src);
    let mut p = Parser::new(&t, names, &[]);
    let mut n = f(&mut p).ok()?;
    p.fill(&mut n);
    p.at_eof().then(|| n.sexp(&t))
}

/// Whether `f` parses all of `src`.
pub fn ok(src: &str, f: fn(&mut Parser) -> PResult<Node>) -> bool {
    sx(src, f).is_some()
}

/// Parse a whole unit with the declare pass of step 2.
pub fn unit(src: &str) -> Result<Vec<String>, String> {
    let t = toks(src);
    let mut names = crate::sem::names::Declare::default();
    match parse_unit(&t, &mut names, &[]) {
        Ok(ns) => Ok(ns.iter().map(|n| n.sexp(&t)).collect()),
        Err(s) => Err(super::stop_message(&t, &s)),
    }
}

/// Whether a whole unit parses.
pub fn parses(src: &str) -> bool {
    unit(src).is_ok()
}
