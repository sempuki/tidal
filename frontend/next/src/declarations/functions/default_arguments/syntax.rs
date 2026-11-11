//! The Syntax segment of `declarations/functions/default-arguments`.

use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::declarators::syntax::{abstract_declarator, declarator};
use crate::declarations::decl_specifiers;
use crate::declarations::initialization::list::syntax::braced_init;
use crate::expressions::assignment::syntax::assignment_expr;
use crate::syntax::names::DeclCtx;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `parameter: {attribute_spec} ["this"] decl_specifiers [declarator | abstract_declarator]
/// ["=" (assignment_expr | braced_init)]`. `tparam` says that the
/// parameter is a template parameter. The name is declared before its
/// default argument.
pub fn parameter(p: &mut Parser, tparam: bool) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    if let Some(k) = p.eat("this") {
        kids.push(k);
    }
    let specs = decl_specifiers(p)?;
    kids.push(Kid::Node(specs));
    if let Some(d) = p.opt(|p| {
        let d = declarator(p)?;
        if p.at_any(&[",", ")", "=", "..."]) || p.peek().starts_with('>') || p.at_eof() {
            Ok(d)
        } else {
            Err(p.fail("',' or ')'"))
        }
    }) {
        kids.push(Kid::Node(d));
    } else if let Some(a) = p.opt(abstract_declarator) {
        kids.push(Kid::Node(a));
    }
    let partial = p.node("parameter", sp, kids.clone());
    p.declared(&partial, DeclCtx { specs: None, template: false, param: tparam });
    if p.at("=") {
        kids.push(p.bump());
        kids.push(Kid::Node(if p.at("{") { braced_init(p)? } else { assignment_expr(p)? }));
    }
    Ok(p.node("parameter", sp, kids))
}
