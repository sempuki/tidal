//! The Syntax segment of `expressions/postfix`.

use crate::classes::r#virtual::syntax::dynamic_cast;
use crate::declarations::initialization::list::syntax::braced_init;
use crate::expressions::assignment::syntax::assignment_expr;
use crate::expressions::casts;
use crate::expressions::primary::syntax::{id_expression, nested_name, primary, unqualified_id};
use crate::lex::TK;
use crate::syntax::names::Qual;
use crate::syntax::{Kid, Node, PResult, Parser};
use crate::templates::arguments::syntax::template_id;

/// `postfix_expr: primary | postfix_expr "[" [expression_list] "]"
/// | postfix_expr "(" [expression_list] ")"
/// | postfix_expr ("." | "->") ["template"] id_expression
/// | postfix_expr ("++" | "--") | functional_cast | named_cast | dynamic_cast`.
pub fn postfix_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("postfix_expr") {
        return Ok(n);
    }
    let sp = p.span();
    let base = if p.at_any(&["static_cast", "const_cast", "reinterpret_cast"]) {
        casts::syntax::named_cast(p)?
    } else if p.at("dynamic_cast") {
        dynamic_cast(p)?
    } else if let Some(f) = p.opt(casts::syntax::functional_cast) {
        f
    } else {
        primary(p)?
    };
    let mut left = base;
    loop {
        let kids = if p.at("[") && !p.at_attr() {
            let open = p.bump();
            let mut k = vec![Kid::Node(left), open];
            if !p.at("]") {
                k.push(Kid::Node(p.bracketed(expression_list)?));
            }
            k.push(p.expect("]")?);
            k
        } else if p.at("(") {
            let open = p.bump();
            let mut k = vec![Kid::Node(left), open];
            if !p.at(")") {
                k.push(Kid::Node(p.bracketed(expression_list)?));
            }
            k.push(p.expect(")")?);
            k
        } else if p.at(".") || p.at("->") {
            let op = p.bump();
            let mut k = vec![Kid::Node(left), op];
            let tkw = p.at("template");
            if tkw {
                k.push(p.bump());
            }
            k.push(Kid::Node(member_id(p, tkw)?));
            k
        } else if p.at("++") || p.at("--") {
            let op = p.bump();
            vec![Kid::Node(left), op]
        } else {
            return Ok(left);
        };
        left = p.node("postfix_expr", sp, kids);
    }
}

/// The `id_expression` after `.` or `->`. A name followed by `<` is a
/// template-id when `template` came before it, or when a member template
/// of that name is known (DEP-24); the parser does not know the object's
/// type.
fn member_id(p: &mut Parser, tkw: bool) -> PResult<Node> {
    let sp = p.span();
    if p.kind() == TK::Ident && p.ahead(1) == "<" && (tkw || p.names.member_template(p.peek())) {
        let (t, _) = template_id(p, Qual::Dependent, true)?;
        let u = p.node("unqualified_id", sp, vec![Kid::Node(t)]);
        return Ok(p.node("id_expression", sp, vec![Kid::Node(u)]));
    }
    if p.at("::") || p.ahead(1) == "::" {
        return id_expression(p);
    }
    if p.kind() == TK::Ident && p.ahead(1) != "::" {
        let u = unqualified_id(p, Qual::Dependent, false)?;
        return Ok(p.node("id_expression", sp, vec![Kid::Node(u)]));
    }
    if p.at("~") || p.at("operator") {
        let u = unqualified_id(p, Qual::Dependent, tkw)?;
        return Ok(p.node("id_expression", sp, vec![Kid::Node(u)]));
    }
    let _ = nested_name;
    id_expression(p)
}

/// `expression_list: argument ["..."] {"," argument ["..."]}`.
pub fn expression_list(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    loop {
        kids.push(Kid::Node(argument(p)?));
        if let Some(k) = p.eat("...") {
            kids.push(k);
        }
        match p.eat(",") {
            Some(k) => kids.push(k),
            None => break,
        }
    }
    Ok(p.node("expression_list", sp, kids))
}

/// `argument: assignment_expr | braced_init`.
pub fn argument(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let e = if p.at("{") { braced_init(p)? } else { assignment_expr(p)? };
    Ok(p.node("argument", sp, vec![Kid::Node(e)]))
}
