//! The Syntax segment of `declarations/declarators`.

use crate::classes::members::syntax::class_name;
use crate::classes::operators::syntax::{literal_operator_id, operator_function_id};
use crate::classes::conversion_functions::syntax::conversion_function_id;
use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::exception_specifications::syntax::noexcept_specifier;
use crate::declarations::functions::default_arguments::syntax::parameter;
use crate::declarations::type_specifiers;
use crate::expressions::primary::syntax::{id_expression, nested_name, unqualified_id};
use crate::foundations::constant_evaluation::syntax::constant_expr;
use crate::syntax::names::{Enter, Qual};
use crate::syntax::{Kid, Node, PResult, Parser};

/// `declarator: ptr_part declarator | direct_declarator`.
pub fn declarator(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("declarator") {
        return Ok(n);
    }
    let sp = p.span();
    if let Some(pp) = p.opt(ptr_part) {
        let d = declarator(p)?;
        return Ok(p.node("declarator", sp, vec![Kid::Node(pp), Kid::Node(d)]));
    }
    let d = direct_declarator(p)?;
    Ok(p.node("declarator", sp, vec![Kid::Node(d)]))
}

fn attrs(p: &mut Parser, kids: &mut Vec<Kid>) -> PResult<()> {
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    Ok(())
}

/// `ptr_part: "*" {cv} {attribute_spec} | nested_name "*" {cv} {attribute_spec}
/// | "&" {attribute_spec} | "&&" {attribute_spec}`.
pub fn ptr_part(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if p.at("&") || p.at("&&") {
        kids.push(p.bump());
        attrs(p, &mut kids)?;
        return Ok(p.node("ptr_part", sp, kids));
    }
    if !p.at("*") {
        let (nn, _, _) = nested_name(p)?;
        kids.push(Kid::Node(nn));
    }
    kids.push(p.expect("*")?);
    while let Some(c) = p.opt(cv) {
        kids.push(Kid::Node(c));
    }
    attrs(p, &mut kids)?;
    Ok(p.node("ptr_part", sp, kids))
}

/// `direct_declarator: ["..."] declarator_id {attribute_spec}
/// | "(" declarator ")" | direct_declarator array_suffix
/// | direct_declarator function_suffix`.
pub fn direct_declarator(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    if p.at("(") {
        let open = p.bump();
        let d = p.bracketed(declarator)?;
        kids.extend([open, Kid::Node(d), p.expect(")")?]);
    } else {
        if let Some(k) = p.eat("...") {
            kids.push(k);
        }
        kids.push(Kid::Node(declarator_id(p)?));
        attrs(p, &mut kids)?;
    }
    let mut dd = p.node("direct_declarator", sp, kids);
    loop {
        let suffix = if p.at("[") && !p.at_attr() {
            array_suffix(p)?
        } else if p.at("(") {
            match p.attempt(function_suffix) {
                Ok(f) => f,
                Err(_) => break,
            }
        } else {
            break;
        };
        dd = p.node("direct_declarator", sp, vec![Kid::Node(dd), Kid::Node(suffix)]);
    }
    Ok(dd)
}

/// `declarator_id: id_expression | [nested_name] operator_function_id
/// | [nested_name] literal_operator_id | [nested_name] conversion_function_id
/// | [nested_name] "~" class_name`. Records the qualifier in
/// `Parser::decl_qual`.
pub fn declarator_id(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let m = p.mark();
    let mut q = Qual::None;
    let mut kids = Vec::new();
    if p.at("::") || p.ahead(1) == "::" || p.ahead(1) == "<" {
        if let Some((nn, qq, _)) = p.opt(nested_name) {
            kids.push(Kid::Node(nn));
            q = qq;
        }
    }
    p.decl_qual = q;
    if p.at("operator") {
        let n = if let Some(n) = p.opt(literal_operator_id) {
            n
        } else if let Some(n) = p.opt(operator_function_id) {
            n
        } else {
            conversion_function_id(p)?
        };
        kids.push(Kid::Node(n));
        return Ok(p.node("declarator_id", sp, kids));
    }
    if p.at("~") {
        kids.push(p.bump());
        kids.push(Kid::Node(class_name(p)?));
        return Ok(p.node("declarator_id", sp, kids));
    }
    // The other forms are an id_expression.
    if kids.is_empty() {
        let id = id_expression(p)?;
        return Ok(p.node("declarator_id", sp, vec![Kid::Node(id)]));
    }
    let tkw = p.at("template");
    let mut qk = kids;
    if tkw {
        qk.push(p.bump());
    }
    let u = unqualified_id(p, q, tkw).map_err(|f| {
        p.reset(m);
        f
    })?;
    qk.push(Kid::Node(u));
    let qn = p.node("qualified_name", sp, qk);
    let id = p.node("id_expression", sp, vec![Kid::Node(qn)]);
    p.decl_qual = q;
    Ok(p.node("declarator_id", sp, vec![Kid::Node(id)]))
}

/// `array_suffix: "[" [constant_expr] "]" {attribute_spec}`.
pub fn array_suffix(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("[")?];
    if !p.at("]") {
        kids.push(Kid::Node(p.bracketed(constant_expr)?));
    }
    kids.push(p.expect("]")?);
    attrs(p, &mut kids)?;
    Ok(p.node("array_suffix", sp, kids))
}

/// `function_suffix: "(" parameters ")" {cv} [ref_qualifier]
/// [noexcept_specifier] {attribute_spec} ["->" type_id]`. The parameters
/// and the trailing return type share a prototype scope.
pub fn function_suffix(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let saved = p.decl_qual;
    // The parameters of a function with a qualified name see the scope
    // the qualifier names.
    let qual_scope = if let Qual::Scope(s) = saved { Some(s) } else { None };
    if let Some(s) = qual_scope {
        p.names.enter(Enter::Scope(s));
    }
    p.names.enter(Enter::Prototype);
    let r = (|| -> PResult<Vec<Kid>> {
        let mut kids = vec![p.expect("(")?];
        let t = p.type_only;
        kids.push(Kid::Node(p.with(true, t, parameters)?));
        kids.push(p.expect(")")?);
        while let Some(c) = p.opt(cv) {
            kids.push(Kid::Node(c));
        }
        if let Some(r) = p.opt(ref_qualifier) {
            kids.push(Kid::Node(r));
        }
        if p.at("noexcept") {
            kids.push(Kid::Node(noexcept_specifier(p)?));
        }
        attrs(p, &mut kids)?;
        if p.at("->") {
            kids.push(p.bump());
            kids.push(Kid::Node(p.with(true, true, type_id)?));
        }
        Ok(kids)
    })();
    p.names.leave();
    if qual_scope.is_some() {
        p.names.leave();
    }
    p.decl_qual = saved;
    Ok(p.node("function_suffix", sp, r?))
}

/// `parameters: [parameter {"," parameter} [[","] "..."]] | "..." | "void"`.
pub fn parameters(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if (p.at("void") || p.at("...")) && p.ahead(1) == ")" {
        let k = p.bump();
        return Ok(p.node("parameters", sp, vec![k]));
    }
    let mut kids = Vec::new();
    if p.at(")") {
        return Ok(p.node("parameters", sp, kids));
    }
    loop {
        kids.push(Kid::Node(parameter(p, false)?));
        if p.at("...") {
            kids.push(p.bump());
            break;
        }
        if p.at(",") && p.ahead(1) == "..." {
            kids.push(p.bump());
            kids.push(p.bump());
            break;
        }
        match p.eat(",") {
            Some(k) => kids.push(k),
            None => break,
        }
    }
    Ok(p.node("parameters", sp, kids))
}

/// `cv: "const" | "volatile"`.
pub fn cv(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at("const") || p.at("volatile") {
        let k = p.bump();
        return Ok(p.node("cv", sp, vec![k]));
    }
    Err(p.fail("'const' or 'volatile'"))
}

/// `ref_qualifier: "&" | "&&"`.
pub fn ref_qualifier(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at("&") || p.at("&&") {
        let k = p.bump();
        return Ok(p.node("ref_qualifier", sp, vec![k]));
    }
    Err(p.fail("'&' or '&&'"))
}

/// `type_id: type_specifiers [abstract_declarator]`.
pub fn type_id(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("type_id") {
        return Ok(n);
    }
    let sp = p.span();
    let t = type_specifiers(p)?;
    let mut kids = vec![Kid::Node(t)];
    if let Some(a) = p.opt(abstract_declarator) {
        kids.push(Kid::Node(a));
    }
    Ok(p.node("type_id", sp, kids))
}

/// `abstract_declarator: ptr_part [abstract_declarator] | abstract_direct | "..."`.
pub fn abstract_declarator(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    p.decl_qual = Qual::None;
    if let Some(pp) = p.opt(ptr_part) {
        let mut kids = vec![Kid::Node(pp)];
        if let Some(a) = p.opt(abstract_declarator) {
            kids.push(Kid::Node(a));
        }
        return Ok(p.node("abstract_declarator", sp, kids));
    }
    if p.at("...") {
        let k = p.bump();
        return Ok(p.node("abstract_declarator", sp, vec![k]));
    }
    let d = abstract_direct(p)?;
    Ok(p.node("abstract_declarator", sp, vec![Kid::Node(d)]))
}

/// `abstract_direct: "(" abstract_declarator ")" | [abstract_direct] array_suffix
/// | [abstract_direct] function_suffix`.
pub fn abstract_direct(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut cur: Option<Node> = None;
    if p.at("(") {
        if let Ok(kids) = p.attempt(|p| {
            let open = p.bump();
            let a = p.bracketed(abstract_declarator)?;
            Ok(vec![open, Kid::Node(a), p.expect(")")?])
        }) {
            cur = Some(p.node("abstract_direct", sp, kids));
        }
    }
    loop {
        let suffix = if p.at("[") && !p.at_attr() {
            array_suffix(p)?
        } else if p.at("(") {
            match p.attempt(function_suffix) {
                Ok(f) => f,
                Err(_) => break,
            }
        } else {
            break;
        };
        let mut kids: Vec<Kid> = cur.take().map(Kid::Node).into_iter().collect();
        kids.push(Kid::Node(suffix));
        cur = Some(p.node("abstract_direct", sp, kids));
    }
    cur.ok_or_else(|| p.fail("an abstract declarator"))
}

/// Whether a declarator declares a function: its name, alone or in
/// parentheses, is followed by a function suffix.
pub fn declares_function(d: &Node) -> bool {
    function_suffix_of(d).is_some()
}

/// The function suffix that makes a declarator a function declarator.
pub fn function_suffix_of(d: &Node) -> Option<&Node> {
    first_op(d).filter(|n| n.prod == "function_suffix")
}

/// The first derivation that applies to the declared name: a pointer
/// part, an array suffix, or a function suffix.
fn first_op(d: &Node) -> Option<&Node> {
    if let (Some(pp), Some(inner)) = (d.child("ptr_part"), d.child("declarator")) {
        return first_op(inner).or(Some(pp));
    }
    let mut dd = d.child("direct_declarator")?;
    let mut innermost: Option<&Node> = None;
    while let Some(inner) = dd.child("direct_declarator") {
        innermost = dd.child("function_suffix").or(dd.child("array_suffix"));
        dd = inner;
    }
    if let Some(inner) = dd.child("declarator") {
        return first_op(inner).or(innermost);
    }
    innermost
}
