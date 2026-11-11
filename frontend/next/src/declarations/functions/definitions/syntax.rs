//! The Syntax segment of `declarations/functions/definitions`.

use crate::classes::constructors::syntax::ctor_init;
use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::{function_definition_after, specs_here};
use crate::exceptions::handlers::syntax::function_try_block;
use crate::statements::blocks::syntax::compound_stmt;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `function_definition: {attribute_spec} [decl_specifiers] declarator
/// [virt_specifiers] function_body | {attribute_spec} [decl_specifiers]
/// declarator requires_clause function_body`. `declaration` and
/// `member_decl` read the same start for a simple declaration and call
/// `function_definition_rest` from there; this function reads the
/// production on its own.
pub fn function_definition(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut pre = Vec::new();
    while p.at_attr() {
        pre.push(Kid::Node(attribute_spec(p)?));
    }
    let specs = p.opt(specs_here);
    function_definition_after(p, sp, pre, specs)
}

/// `function_body: [ctor_init] compound_stmt | function_try_block
/// | "=" "delete" ";" | "=" "default" ";"`.
pub fn function_body(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("function_body") {
        return Ok(n);
    }
    let sp = p.span();
    let mut kids = Vec::new();
    if p.at("=") {
        kids.push(p.bump());
        if !(p.at("delete") || p.at("default")) {
            return Err(p.fail("'delete' or 'default'"));
        }
        kids.push(p.bump());
        kids.push(p.expect(";")?);
    } else if p.at("try") {
        kids.push(Kid::Node(function_try_block(p)?));
    } else {
        if p.at(":") {
            kids.push(Kid::Node(ctor_init(p)?));
        }
        let t = p.take_template();
        kids.push(Kid::Node(compound_stmt(p)?));
        p.template = t;
    }
    Ok(p.node("function_body", sp, kids))
}
