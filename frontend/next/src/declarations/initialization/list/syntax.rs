//! The Syntax segment of `declarations/initialization/list`.

use crate::expressions::assignment::syntax::assignment_expr;
use crate::expressions::primary::syntax::identifier;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `braced_init: "{" [init_clause ["..."] {"," init_clause ["..."]} [","]] "}"`.
pub fn braced_init(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("{")?];
    p.bracketed(|p| -> PResult<()> {
        while !p.at("}") {
            kids.push(Kid::Node(init_clause(p)?));
            if let Some(k) = p.eat("...") {
                kids.push(k);
            }
            match p.eat(",") {
                Some(k) => kids.push(k),
                None => break,
            }
        }
        Ok(())
    })?;
    kids.push(p.expect("}")?);
    Ok(p.node("braced_init", sp, kids))
}

/// `init_clause: assignment_expr | braced_init
/// | designator "=" (assignment_expr | braced_init) | designator braced_init`.
pub fn init_clause(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if p.at(".") {
        kids.push(Kid::Node(designator(p)?));
        if p.at("{") {
            kids.push(Kid::Node(braced_init(p)?));
            return Ok(p.node("init_clause", sp, kids));
        }
        kids.push(p.expect("=")?);
    }
    kids.push(Kid::Node(if p.at("{") { braced_init(p)? } else { assignment_expr(p)? }));
    Ok(p.node("init_clause", sp, kids))
}

/// `designator: "." identifier`.
pub fn designator(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let dot = p.expect(".")?;
    let id = identifier(p)?;
    Ok(p.node("designator", sp, vec![dot, id]))
}
