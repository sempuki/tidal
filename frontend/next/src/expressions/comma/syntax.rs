//! The Syntax segment of `expressions/comma`.

use crate::expressions::assignment::syntax::assignment_expr;
use crate::syntax::{Kid, Node, PResult, Parser};

/// COMMA-9: a braced list is no operand of the comma operator. `left`
/// says the list comes first, so a `,` after it marks the form.
fn braced_operand(p: &mut Parser, left: bool) -> PResult<()> {
    if !p.at("{") {
        return Ok(());
    }
    let mut depth = 0;
    let mut k = 0;
    loop {
        match p.ahead(k) {
            "{" => depth += 1,
            "}" => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            "" if p.ahead_kind(k) == crate::lex::TK::Eof => return Ok(()),
            _ => {}
        }
        k += 1;
    }
    if !left || p.ahead(k + 1) == "," {
        return Err(p.fail_rule("COMMA-9", "a braced list is not an operand of the comma operator"));
    }
    Ok(())
}

/// `expression: assignment_expr {"," assignment_expr}`.
pub fn expression(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("expression") {
        return Ok(n);
    }
    let sp = p.span();
    braced_operand(p, true)?;
    let first = assignment_expr(p)?;
    if !p.at(",") {
        return Ok(first);
    }
    let mut kids = vec![Kid::Node(first)];
    while p.at(",") {
        kids.push(p.bump());
        braced_operand(p, false)?;
        kids.push(Kid::Node(assignment_expr(p)?));
    }
    Ok(p.node("expression", sp, kids))
}
