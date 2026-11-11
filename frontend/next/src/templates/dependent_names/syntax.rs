//! The Syntax segment of `templates/dependent-names`.

use crate::expressions::primary::syntax::{identifier, nested_name};
use crate::syntax::{Kid, Node, PResult, Parser};
use crate::templates::arguments::syntax::template_id;

/// `typename_specifier: "typename" nested_name identifier
/// | "typename" nested_name ["template"] template_id`.
pub fn typename_specifier(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kw = p.expect("typename")?;
    let (nn, q, _) = nested_name(p)?;
    let mut kids = vec![kw, Kid::Node(nn)];
    let tkw = p.at("template");
    if tkw {
        kids.push(p.bump());
    }
    if p.ahead(1) == "<" && (tkw || p.find(q, p.peek()).is_template()) {
        kids.push(Kid::Node(template_id(p, q, tkw)?.0));
    } else if tkw {
        return Err(p.fail("a template-id"));
    } else {
        kids.push(identifier(p)?);
    }
    Ok(p.node("typename_specifier", sp, kids))
}
