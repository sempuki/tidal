//! The Syntax segment of `classes/conversion-functions`.

use crate::classes::constructors::syntax::parts;
use crate::declarations::declarators::syntax::ptr_part;
use crate::declarations::type_specifiers;
use crate::lex::Token;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `conversion_function_id: "operator" conversion_type`.
pub fn conversion_function_id(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let op = p.expect("operator")?;
    let t = conversion_type(p)?;
    Ok(p.node("conversion_function_id", sp, vec![op, Kid::Node(t)]))
}

/// `conversion_type: type_specifiers {ptr_part}`.
pub fn conversion_type(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![Kid::Node(type_specifiers(p)?)];
    while let Some(pp) = p.opt(ptr_part) {
        kids.push(Kid::Node(pp));
    }
    Ok(p.node("conversion_type", sp, kids))
}

/// `cvf_specifier: "constexpr" | "virtual" | "inline" | explicit_spec`.
pub fn cvf_specifier(word: &str) -> bool {
    matches!(word, "constexpr" | "virtual" | "inline" | "explicit")
}

/// `conversion_decl: {cvf_specifier} conversion_function_id "(" ")" [cv] [ref_qualifier]`:
/// a declaration whose declarator names a conversion function with no
/// parameters.
pub fn conversion_decl(n: &Node, toks: &[Token]) -> bool {
    let (specs, d) = parts(n);
    let specs_ok = specs.map_or(true, |s| {
        s.children("decl_specifier").all(|ds| {
            let mut ids = Vec::new();
            ds.tokens(&mut ids);
            ids.first().is_some_and(|&i| cvf_specifier(&toks[i].text))
        })
    });
    let Some(d) = d else { return false };
    specs_ok && d.find("conversion_function_id").is_some() && d.find("parameters").is_some_and(|ps| ps.kids.is_empty())
}
