//! The Syntax segment of `classes/access`.

use crate::classes::members::syntax::class_name;
use crate::declarations::declarators::syntax::declares_function;
use crate::declarations::type_names::syntax::class_key;
use crate::declarations::{declarator_here, function_definition_rest, specs_here};
use crate::expressions::primary::syntax::nested_name;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `access_label: access_specifier ":"`.
pub fn access_label(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let a = access_specifier(p)?;
    let c = p.expect(":")?;
    Ok(p.node("access_label", sp, vec![Kid::Node(a), c]))
}

/// `access_specifier: "public" | "protected" | "private"`.
pub fn access_specifier(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at_any(&["public", "protected", "private"]) {
        let k = p.bump();
        return Ok(p.node("access_specifier", sp, vec![k]));
    }
    Err(p.fail("an access specifier"))
}

/// `friend_decl: "friend" [decl_specifiers] declarator ";"
/// | "friend" function_definition | "friend" class_key [nested_name] class_name ";"`.
pub fn friend_decl(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let fk = p.expect("friend")?;
    if p.at_any(&["class", "struct", "union"]) {
        if let Ok(kids) = p.attempt(|p| {
            let mut kids = vec![Kid::Node(class_key(p)?)];
            if p.at("::") || p.ahead(1) == "::" {
                kids.push(Kid::Node(nested_name(p)?.0));
            }
            kids.push(Kid::Node(class_name(p)?));
            kids.push(p.expect(";")?);
            Ok(kids)
        }) {
            let mut all = vec![fk];
            all.extend(kids);
            return Ok(p.node("friend_decl", sp, all));
        }
    }
    let specs = p.opt(specs_here);
    let d = declarator_here(p)?;
    if declares_function(&d) && !p.at(";") {
        let f = function_definition_rest(p, sp, vec![], specs, d)?;
        return Ok(p.node("friend_decl", sp, vec![fk, Kid::Node(f)]));
    }
    let _ = p.take_template();
    let mut kids = vec![fk];
    if let Some(s) = specs {
        kids.push(Kid::Node(s));
    }
    kids.push(Kid::Node(d));
    kids.push(p.expect(";")?);
    Ok(p.node("friend_decl", sp, kids))
}
