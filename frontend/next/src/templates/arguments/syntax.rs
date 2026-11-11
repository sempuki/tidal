//! The Syntax segment of `templates/arguments`.

use crate::classes::operators::syntax::{literal_operator_id, operator_function_id};
use crate::declarations::declarators::syntax::type_id;
use crate::declarations::initialization::list::syntax::braced_init;
use crate::expressions::primary::syntax::{identifier, nested_name};
use crate::foundations::constant_evaluation::syntax::constant_expr;
use crate::syntax::names::{Kind, Qual, TKind};
use crate::syntax::{Kid, Node, PResult, Parser};

/// `template_id: template_name "<" [template_args] ">"
/// | (operator_function_id | literal_operator_id) "<" [template_args] ">"`.
/// `q` is the qualifier before the name and `tkw` says that `template`
/// came before it. Gives the node and whether an argument names a
/// template parameter, which makes the template-id dependent.
pub fn template_id(p: &mut Parser, q: Qual, tkw: bool) -> PResult<(Node, bool)> {
    let _ = (q, tkw);
    let sp = p.span();
    let name = if p.at("operator") {
        match p.opt(literal_operator_id) {
            Some(n) => n,
            None => operator_function_id(p)?,
        }
    } else {
        template_name(p)?
    };
    let open = p.expect("<")?;
    let saw = std::mem::replace(&mut p.saw_param, false);
    let r = p.with(false, false, |p| -> PResult<Vec<Kid>> {
        let mut kids = vec![Kid::Node(name), open];
        if !p.peek().starts_with('>') {
            kids.push(Kid::Node(template_args(p)?));
        }
        kids.push(p.close_angle()?);
        Ok(kids)
    });
    let dep = p.saw_param;
    p.saw_param = saw || dep;
    Ok((p.node("template_id", sp, r?), dep))
}

/// `template_name: identifier`.
pub fn template_name(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let id = identifier(p)?;
    Ok(p.node("template_name", sp, vec![id]))
}

/// `template_args: template_arg ["..."] {"," template_arg ["..."]}`.
pub fn template_args(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    loop {
        kids.push(Kid::Node(template_arg(p)?));
        if let Some(k) = p.eat("...") {
            kids.push(k);
        }
        match p.eat(",") {
            Some(k) => kids.push(k),
            None => break,
        }
    }
    Ok(p.node("template_args", sp, kids))
}

fn arg_ends(p: &Parser) -> bool {
    p.at(",") || p.at("...") || p.peek().starts_with('>') || p.at_eof()
}

/// `template_arg: type_id | constant_expr | braced_init
/// | [nested_name] template_name`. A name of a class template with no
/// arguments is the last form; a form that is a type is a type (TARG
/// rules). A braced list is an argument for a constant parameter of class
/// type (TARG-16).
pub fn template_arg(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at("{") {
        let b = p.bracketed(braced_init)?;
        return Ok(p.node("template_arg", sp, vec![Kid::Node(b)]));
    }
    // A template template argument: a template's name and nothing more.
    if let Ok(n) = p.attempt(|p| {
        let mut kids = Vec::new();
        let mut q = Qual::None;
        if p.at("::") || p.ahead(1) == "::" {
            let (nn, qq, _) = nested_name(p)?;
            kids.push(Kid::Node(nn));
            q = qq;
        }
        let f = p.find(q, p.peek());
        let is_class_template = matches!(f.kind, Kind::Template(TKind::Class | TKind::Alias));
        if !(is_class_template && p.ahead(1) != "<") {
            return Err(p.fail("a template name"));
        }
        kids.push(Kid::Node(template_name(p)?));
        if !arg_ends(p) {
            return Err(p.fail("',' or '>'"));
        }
        Ok(kids)
    }) {
        return Ok(p.node("template_arg", sp, n));
    }
    if let Ok(t) = p.attempt(|p| {
        let t = type_id(p)?;
        if arg_ends(p) {
            Ok(t)
        } else {
            Err(p.fail("',' or '>'"))
        }
    }) {
        return Ok(p.node("template_arg", sp, vec![Kid::Node(t)]));
    }
    let e = constant_expr(p)?;
    Ok(p.node("template_arg", sp, vec![Kid::Node(e)]))
}
