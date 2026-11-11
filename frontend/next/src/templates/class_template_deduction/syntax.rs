//! The Syntax segment of `templates/class-template-deduction`.

use crate::classes::constructors::syntax::explicit_spec;
use crate::declarations::declarators::syntax::parameters;
use crate::syntax::names::{Enter, Qual};
use crate::syntax::{Kid, Node, PResult, Parser};
use crate::templates::arguments::syntax::{template_id, template_name};

/// `deduction_guide: [explicit_spec] template_name "(" parameters ")" "->" template_id ";"`.
pub fn deduction_guide(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if p.at("explicit") {
        kids.push(Kid::Node(explicit_spec(p)?));
    }
    kids.push(Kid::Node(template_name(p)?));
    kids.push(p.expect("(")?);
    p.names.enter(Enter::Prototype);
    let r = p.with(true, true, parameters);
    p.names.leave();
    kids.push(Kid::Node(r?));
    kids.push(p.expect(")")?);
    kids.push(p.expect("->")?);
    kids.push(Kid::Node(template_id(p, Qual::None, true)?.0));
    kids.push(p.expect(";")?);
    let _ = p.take_template();
    Ok(p.node("deduction_guide", sp, kids))
}
