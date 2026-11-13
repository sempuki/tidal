//! The Syntax segment of `declarations/static-assert`.

use crate::expressions::primary::syntax::string_literal;
use crate::foundations::constant_evaluation::syntax::constant_expr;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `static_assert_decl: "static_assert" "(" constant_expr
/// ["," string_literal {string_literal}] ")" ";"`.
pub fn static_assert_decl(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("static_assert")?, p.expect("(")?];
    kids.push(Kid::Node(p.bracketed(constant_expr)?));
    if p.at(",") {
        kids.push(p.bump());
        if p.kind() != crate::lex::TK::Str {
            return Err(p.fail_rule("SA-8", "the message of a static_assert is a string literal"));
        }
        kids.push(string_literal(p)?);
        while p.kind() == crate::lex::TK::Str {
            kids.push(string_literal(p)?);
        }
    }
    kids.push(p.expect(")")?);
    kids.push(p.expect(";")?);
    Ok(p.node("static_assert_decl", sp, kids))
}
