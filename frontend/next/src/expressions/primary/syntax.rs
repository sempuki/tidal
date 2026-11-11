//! The phrase productions of `expressions/primary`'s Syntax segment:
//! `primary`, `fold_expr`, `fold_op`, the names (`id_expression`,
//! `unqualified_id`, `qualified_name`, `nested_name`), and the
//! token-level `identifier`, `literal`, and `string_literal`.

use crate::classes::{members, operators};
use crate::classes::conversion_functions::syntax as cvf;
use crate::expressions::{casts, comma, decltype, lambdas};
use crate::foundations::keywords::is_keyword;
use crate::lex::TK;
use crate::syntax::names::{Kind, Qual, TKind};
use crate::syntax::{Kid, Node, PResult, Parser};
use crate::templates::{arguments, concepts};

/// `primary: literal | "this" | "(" expression ")" | id_expression
/// | lambda_expr | fold_expr | requires_expression`.
pub fn primary(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("primary") {
        return Ok(n);
    }
    let sp = p.span();
    if let Some(l) = p.opt(literal) {
        return Ok(p.node("primary", sp, vec![Kid::Node(l)]));
    }
    if p.at("this") {
        let k = p.bump();
        return Ok(p.node("primary", sp, vec![k]));
    }
    if p.at("(") {
        if paren_has_ellipsis(p) {
            if let Some(f) = p.opt(fold_expr) {
                return Ok(p.node("primary", sp, vec![Kid::Node(f)]));
            }
        }
        let open = p.bump();
        let e = p.bracketed(comma::syntax::expression)?;
        let close = p.expect(")")?;
        return Ok(p.node("primary", sp, vec![open, Kid::Node(e), close]));
    }
    if p.at("[") && !p.at_attr() {
        let l = lambdas::syntax::lambda_expr(p)?;
        return Ok(p.node("primary", sp, vec![Kid::Node(l)]));
    }
    if p.at("requires") {
        let r = concepts::syntax::requires_expression(p)?;
        return Ok(p.node("primary", sp, vec![Kid::Node(r)]));
    }
    let id = id_expression(p)?;
    Ok(p.node("primary", sp, vec![Kid::Node(id)]))
}

/// Whether a `...` sits at the top level of the parentheses that start here.
fn paren_has_ellipsis(p: &Parser) -> bool {
    let mut depth = 0i32;
    let mut k = 0;
    loop {
        let t = p.ahead(k);
        if p.ahead_kind(k) == TK::Eof {
            return false;
        }
        match t {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => {
                depth -= 1;
                if depth == 0 {
                    return false;
                }
            }
            "..." if depth == 1 => return true,
            _ => {}
        }
        k += 1;
    }
}

/// `fold_expr`: a unary right fold, a unary left fold, or a binary fold.
pub fn fold_expr(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("(")?];
    p.bracketed(|p| {
        if p.at("...") {
            kids.push(p.bump());
            kids.push(Kid::Node(fold_op(p)?));
            kids.push(Kid::Node(casts::syntax::cast_expr(p)?));
        } else {
            kids.push(Kid::Node(casts::syntax::cast_expr(p)?));
            kids.push(Kid::Node(fold_op(p)?));
            kids.push(p.expect("...")?);
            if !p.at(")") {
                kids.push(Kid::Node(fold_op(p)?));
                kids.push(Kid::Node(casts::syntax::cast_expr(p)?));
            }
        }
        Ok(())
    })?;
    kids.push(p.expect(")")?);
    Ok(p.node("fold_expr", sp, kids))
}

/// `fold_op`: the binary operators that a fold can use.
pub fn fold_op(p: &mut Parser) -> PResult<Node> {
    const OPS: &[&str] = &[
        "+", "-", "*", "/", "%", "^", "&", "|", "<<", ">>", "+=", "-=", "*=", "/=", "%=", "^=", "&=", "|=", "<<=",
        ">>=", "=", "==", "!=", "<", ">", "<=", ">=", "&&", "||", ",", ".*", "->*",
    ];
    let sp = p.span();
    if p.at_any(OPS) {
        let k = p.bump();
        return Ok(p.node("fold_op", sp, vec![k]));
    }
    Err(p.fail("a fold operator"))
}

/// `id_expression: unqualified_id | qualified_name`.
pub fn id_expression(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at("::") || p.ahead(1) == "::" || p.at("decltype") || p.ahead(1) == "<" {
        if let Some(q) = p.opt(qualified_name) {
            return Ok(p.node("id_expression", sp, vec![Kid::Node(q)]));
        }
    }
    let u = unqualified_id(p, Qual::None, false)?;
    Ok(p.node("id_expression", sp, vec![Kid::Node(u)]))
}

/// `unqualified_id: identifier | "~" class_name | operator_function_id
/// | literal_operator_id | conversion_function_id | template_id`.
/// `q` is the qualifier before the name, and `tkw` says that the keyword
/// `template` came before it.
pub fn unqualified_id(p: &mut Parser, q: Qual, tkw: bool) -> PResult<Node> {
    let sp = p.span();
    if p.at("~") {
        let t = p.bump();
        let c = members::syntax::class_name(p)?;
        return Ok(p.node("unqualified_id", sp, vec![t, Kid::Node(c)]));
    }
    if p.at("operator") {
        let n = if let Some(n) = p.opt(operators::syntax::literal_operator_id) {
            n
        } else if let Some(n) = p.opt(operators::syntax::operator_function_id) {
            n
        } else {
            cvf::conversion_function_id(p)?
        };
        return Ok(p.node("unqualified_id", sp, vec![Kid::Node(n)]));
    }
    if p.ahead(1) == "<" && p.kind() == TK::Ident && p.ahead(0) == p.peek() {
        let name = p.peek();
        if tkw || p.find(q, name).is_template() {
            let (t, _) = arguments::syntax::template_id(p, q, tkw)?;
            return Ok(p.node("unqualified_id", sp, vec![Kid::Node(t)]));
        }
    }
    let id = identifier(p)?;
    Ok(p.node("unqualified_id", sp, vec![id]))
}

/// `qualified_name: nested_name ["template"] unqualified_id`.
pub fn qualified_name(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let (nn, q, _) = nested_name(p)?;
    let mut kids = vec![Kid::Node(nn)];
    let tkw = p.at("template");
    if tkw {
        kids.push(p.bump());
    }
    kids.push(Kid::Node(unqualified_id(p, q, tkw)?));
    Ok(p.node("qualified_name", sp, kids))
}

/// What a qualifier component names, for the next one.
pub fn resolve(p: &mut Parser, q: Qual, name: &str, dependent_args: bool) -> Qual {
    if q == Qual::Dependent || dependent_args {
        return Qual::Dependent;
    }
    let f = p.find(q, name);
    if f.param {
        return Qual::Dependent;
    }
    match (f.kind, f.scope) {
        (Kind::Namespace | Kind::Type | Kind::Template(TKind::Class), Some(s)) => Qual::Scope(s),
        _ => Qual::Dependent,
    }
}

/// The class a template-id with dependent arguments names when it is the
/// current instantiation: the template's own definition is open, or the
/// arguments are template parameters alone, as in the qualifier of a
/// member defined outside its class.
pub fn current_instantiation(p: &mut Parser, q: Qual, name: &str, tid: &Node) -> Option<u32> {
    if q == Qual::None {
        if let Some(s) = p.names.current(name) {
            return Some(s);
        }
    }
    let bare = tid.child("template_args").is_some_and(|a| {
        a.children("template_arg").all(|arg| {
            let mut ids = Vec::new();
            arg.tokens(&mut ids);
            ids.len() == 1 && p.toks[ids[0]].kind == TK::Ident && p.names.find(Qual::None, &p.toks[ids[0]].text).param
        })
    });
    let f = p.names.find(q, name);
    (bare && f.kind == Kind::Template(TKind::Class)).then_some(f.scope).flatten()
}

/// `nested_name: "::" | [nested_name] identifier "::"
/// | [nested_name] ["template"] template_id "::" | decltype_spec "::"`.
/// Gives the node, the scope it names, and the name of its last
/// component (for a constructor's declarator, `X::X`).
pub fn nested_name<'t>(p: &mut Parser<'t>) -> PResult<(Node, Qual, Option<&'t str>)> {
    let sp = p.span();
    let mut kids = Vec::new();
    let mut q;
    let mut last = None;
    if p.at("::") {
        kids.push(p.bump());
        q = Qual::Global;
    } else if p.at("decltype") {
        let d = decltype::syntax::decltype_spec(p)?;
        kids.push(Kid::Node(d));
        kids.push(p.expect("::")?);
        q = Qual::Dependent;
    } else {
        q = Qual::None;
    }
    let mut parts = 0;
    loop {
        let m = p.mark();
        let tkw = parts + kids.len() > 0 && p.at("template");
        let mut step = Vec::new();
        if tkw {
            step.push(p.bump());
        }
        if p.kind() != TK::Ident || is_keyword(p.peek()) {
            p.reset(m);
            break;
        }
        let name = p.tok().text.as_str();
        if p.ahead(1) == "::" {
            step.push(identifier(p)?);
            step.push(p.bump());
            q = resolve(p, q, name, false);
        } else if p.ahead(1) == "<" && (tkw || p.find(q, name).is_template()) {
            match p.attempt(|p| arguments::syntax::template_id(p, q, tkw)) {
                Ok((t, dep)) if p.at("::") => {
                    let cur = if dep && q != Qual::Dependent { current_instantiation(p, q, name, &t) } else { None };
                    step.push(Kid::Node(t));
                    step.push(p.bump());
                    q = match cur {
                        Some(s) => Qual::Scope(s),
                        None => resolve(p, q, name, dep),
                    };
                }
                _ => {
                    p.reset(m);
                    break;
                }
            }
        } else {
            p.reset(m);
            break;
        }
        last = Some(name);
        let start = p.node("nested_name", sp, std::mem::take(&mut kids));
        kids = if start.kids.is_empty() { step } else { std::iter::once(Kid::Node(start)).chain(step).collect() };
        parts += 1;
    }
    if kids.is_empty() {
        return Err(p.fail("a nested name"));
    }
    Ok((p.node("nested_name", sp, kids), q, last))
}

/// `identifier`: a name that is not a keyword (KW-1).
pub fn identifier(p: &mut Parser) -> PResult<Kid> {
    if p.kind() == TK::Ident && !is_keyword(p.peek()) {
        Ok(p.bump())
    } else {
        Err(p.fail("an identifier"))
    }
}

/// `literal: integer_literal | floating_literal | char_literal
/// | string_literal {string_literal} | "true" | "false" | "nullptr"`.
pub fn literal(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    match p.kind() {
        TK::Int | TK::Float | TK::Char => {
            let k = p.bump();
            Ok(p.node("literal", sp, vec![k]))
        }
        TK::Str => {
            let mut kids = vec![string_literal(p)?];
            while p.kind() == TK::Str {
                kids.push(string_literal(p)?);
            }
            Ok(p.node("literal", sp, kids))
        }
        _ if p.at_any(&["true", "false", "nullptr"]) => {
            let k = p.bump();
            Ok(p.node("literal", sp, vec![k]))
        }
        _ => Err(p.fail("a literal")),
    }
}

/// `string_literal`: one string literal token.
pub fn string_literal(p: &mut Parser) -> PResult<Kid> {
    if p.kind() == TK::Str {
        Ok(p.bump())
    } else {
        Err(p.fail("a string literal"))
    }
}
