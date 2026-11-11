//! The Syntax segment of `declarations/auto`. `placeholder` is a parse
//! function. `auto_var_decl` is a shape, and its productions are
//! recognizers over a simple declaration's tree.

use crate::lex::Token;
use crate::syntax::{Kid, Node, PResult, Parser};
use crate::templates::concepts::syntax::type_constraint;

/// `placeholder: [type_constraint] ("auto" | "decltype" "(" "auto" ")")`.
pub fn placeholder(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if !p.at("auto") && !p.at("decltype") {
        kids.push(Kid::Node(type_constraint(p)?));
    }
    if p.at("auto") {
        kids.push(p.bump());
    } else {
        kids.push(p.expect("decltype")?);
        kids.push(p.expect("(")?);
        kids.push(p.expect("auto")?);
        kids.push(p.expect(")")?);
    }
    Ok(p.node("placeholder", sp, kids))
}

/// `auto_var_decl: {decl_specifier} placeholder {decl_specifier}
/// auto_declarator {"," auto_declarator} ";"`: a simple declaration whose
/// specifiers hold a placeholder and whose declarators each have an
/// initializer of the forms of `auto_init`.
pub fn auto_var_decl(n: &Node, toks: &[Token]) -> bool {
    let Some(sd) = n.find("simple_decl") else { return false };
    let Some(specs) = sd.child("decl_specifiers") else { return false };
    let holes = specs.children("decl_specifier").filter(|d| d.find("placeholder").is_some()).count();
    let mut decls = sd.children("init_declarator").peekable();
    holes == 1 && decls.peek().is_some() && decls.all(|d| auto_declarator(d, toks))
}

/// `auto_declarator: declarator auto_init`.
pub fn auto_declarator(n: &Node, toks: &[Token]) -> bool {
    n.child("declarator").is_some() && n.child("init").is_some_and(|i| auto_init(i, toks))
}

/// `auto_init: "=" assignment_expr | "=" "{" [assignment_expr {"," assignment_expr}] "}"
/// | "(" assignment_expr ")" | "{" assignment_expr "}"`.
pub fn auto_init(n: &Node, toks: &[Token]) -> bool {
    let w = n.words(toks);
    let list = n.find("braced_init").map_or(0, |b| b.children("init_clause").count());
    match w.first().copied() {
        Some("=") => n.find("braced_init").is_none() || n.find("designator").is_none(),
        Some("(") => n.find("expression_list").is_some_and(|l| l.children("argument").count() == 1),
        _ => list == 1,
    }
}
