//! The Syntax segment of `expressions/casts`.

use crate::declarations::declarators::syntax::type_id;
use crate::declarations::initialization::list::syntax::braced_init;
use crate::declarations::type_start;
use crate::expressions::{comma, decltype, postfix, unary};
use crate::expressions::primary::syntax::{identifier, nested_name};
use crate::lex::TK;
use crate::syntax::names::Qual;
use crate::syntax::{Kid, Node, PResult, Parser};
use crate::templates::arguments::syntax::template_id;
use crate::templates::dependent_names::syntax::typename_specifier;

/// `cast_expr: unary_expr | "(" type_id ")" cast_expr`.
pub fn cast_expr(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("cast_expr") {
        return Ok(n);
    }
    let sp = p.span();
    if p.at("(") {
        if let Ok(kids) = p.attempt(|p| {
            let open = p.bump();
            if !type_start(p) {
                return Err(p.fail("a type"));
            }
            let t = p.bracketed(type_id)?;
            let close = p.expect(")")?;
            let e = cast_expr(p)?;
            Ok(vec![open, Kid::Node(t), close, Kid::Node(e)])
        }) {
            return Ok(p.node("cast_expr", sp, kids));
        }
    }
    unary::syntax::unary_expr(p)
}

/// `named_cast: cast_keyword "<" type_id ">" "(" expression ")"`.
pub fn named_cast(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let kw = cast_keyword(p)?;
    let lt = p.expect("<")?;
    let t = p.with(false, true, type_id)?;
    let gt = p.close_angle()?;
    let open = p.expect("(")?;
    let e = p.bracketed(comma::syntax::expression)?;
    let close = p.expect(")")?;
    Ok(p.node("named_cast", sp, vec![Kid::Node(kw), lt, Kid::Node(t), gt, open, Kid::Node(e), close]))
}

/// `cast_keyword: "static_cast" | "const_cast" | "reinterpret_cast"`.
pub fn cast_keyword(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at_any(&["static_cast", "const_cast", "reinterpret_cast"]) {
        let k = p.bump();
        return Ok(p.node("cast_keyword", sp, vec![k]));
    }
    Err(p.fail("a cast keyword"))
}

/// `functional_cast: cast_type "(" [expression_list] ")" | cast_type braced_init`.
pub fn functional_cast(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let t = cast_type(p)?;
    if p.at("{") {
        let b = braced_init(p)?;
        return Ok(p.node("functional_cast", sp, vec![Kid::Node(t), Kid::Node(b)]));
    }
    let open = p.expect("(")?;
    let mut kids = vec![Kid::Node(t), open];
    if !p.at(")") {
        kids.push(Kid::Node(p.bracketed(postfix::syntax::expression_list)?));
    }
    kids.push(p.expect(")")?);
    Ok(p.node("functional_cast", sp, kids))
}

const CAST_KEYWORDS: &[&str] = &[
    "bool", "void", "int", "short", "long", "signed", "unsigned", "float", "double", "char", "wchar_t", "char8_t",
    "char16_t", "char32_t", "auto",
];

/// `cast_type: [nested_name] (identifier | template_id) | decltype_spec
/// | typename_specifier | "auto" | "bool" | ...`. The name or the
/// template-id must name a type, as the oracle answers; otherwise the
/// same tokens are a call.
pub fn cast_type(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at_any(CAST_KEYWORDS) {
        let k = p.bump();
        return Ok(p.node("cast_type", sp, vec![k]));
    }
    if p.at("decltype") {
        let d = decltype::syntax::decltype_spec(p)?;
        return Ok(p.node("cast_type", sp, vec![Kid::Node(d)]));
    }
    if p.at("typename") {
        let t = typename_specifier(p)?;
        return Ok(p.node("cast_type", sp, vec![Kid::Node(t)]));
    }
    let mut kids = Vec::new();
    let mut q = Qual::None;
    if p.at("::") || p.ahead(1) == "::" || p.ahead(1) == "<" {
        if let Some((nn, qq, _)) = p.opt(nested_name) {
            kids.push(Kid::Node(nn));
            q = qq;
        }
    }
    let name = p.peek();
    if p.kind() == TK::Ident && p.ahead(1) == "<" {
        let f = p.find(q, name);
        let is_type = if q == Qual::Dependent { p.type_only } else { f.is_type() };
        if !is_type {
            return Err(p.fail("a type name"));
        }
        let (t, _) = template_id(p, q, false)?;
        kids.push(Kid::Node(t));
        return Ok(p.node("cast_type", sp, kids));
    }
    if !p.names_type(q, name) {
        return Err(p.fail("a type name"));
    }
    kids.push(identifier(p)?);
    Ok(p.node("cast_type", sp, kids))
}
