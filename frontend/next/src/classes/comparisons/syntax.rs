//! The Syntax segment of `classes/comparisons`: shapes, as recognizers
//! over the trees that `member_decl` and `function_definition` make.

use crate::classes::constructors::syntax::parts;
use crate::lex::Token;
use crate::syntax::Node;

/// `comparison_op: "==" | "!=" | "<" | ">" | "<=" | ">=" | "<=>"`.
pub fn comparison_op(word: &str) -> bool {
    matches!(word, "==" | "!=" | "<" | ">" | "<=" | ">=" | "<=>")
}

/// `comparison_decl`: a member or friend declaration whose declarator
/// names `operator` and a comparison operator, with parameters.
pub fn comparison_decl(n: &Node, toks: &[Token]) -> bool {
    let (_, d) = parts(n);
    let Some(d) = d else { return false };
    let Some(op) = d.find("overloadable_operator") else { return false };
    let mut ids = Vec::new();
    op.tokens(&mut ids);
    ids.len() == 1 && comparison_op(&toks[ids[0]].text) && d.find("parameters").is_some()
}

/// `defaulted_comparison: comparison_decl "=" "default" ";"`.
pub fn defaulted_comparison(n: &Node, toks: &[Token]) -> bool {
    let Some(f) = n.find("function_definition") else { return false };
    let Some(b) = f.child("function_body") else { return false };
    let mut ids = Vec::new();
    b.tokens(&mut ids);
    let w: Vec<&str> = ids.iter().map(|&i| toks[i].text.as_str()).collect();
    w == ["=", "default", ";"] && comparison_decl(f, toks)
}
