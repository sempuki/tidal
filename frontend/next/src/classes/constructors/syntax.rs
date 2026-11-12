//! The Syntax segment of `classes/constructors`. `ctor_init`, `mem_init`,
//! `mem_init_id`, and `explicit_spec` are parse functions. The shapes
//! (`ctor_def`, `dtor_decl`, `dtor_call`) and the productions only they
//! use are recognizers over the trees that the general productions make.

use crate::classes::derived::syntax::base_type;
use crate::declarations::initialization::list::syntax::braced_init;
use crate::expressions::postfix::syntax::expression_list;
use crate::expressions::primary::syntax::identifier;
use crate::foundations::constant_evaluation::syntax::constant_expr;
use crate::lex::Token;
use crate::syntax::{Kid, Node, PResult, Parser};

/// `explicit_spec: "explicit" ["(" constant_expr ")"]`.
pub fn explicit_spec(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("explicit")?];
    if p.at("(") {
        kids.push(p.bump());
        kids.push(Kid::Node(p.bracketed(constant_expr)?));
        kids.push(p.expect(")")?);
    }
    Ok(p.node("explicit_spec", sp, kids))
}

/// `ctor_init: ":" mem_init ["..."] {"," mem_init ["..."]}`.
pub fn ctor_init(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect(":")?];
    loop {
        kids.push(Kid::Node(mem_init(p)?));
        if let Some(k) = p.eat("...") {
            kids.push(k);
        }
        match p.eat(",") {
            Some(k) => kids.push(k),
            None => break,
        }
    }
    Ok(p.node("ctor_init", sp, kids))
}

/// `mem_init: mem_init_id ("(" [expression_list] ")" | braced_init)`.
pub fn mem_init(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![Kid::Node(mem_init_id(p)?)];
    if p.at("{") {
        kids.push(Kid::Node(braced_init(p)?));
    } else {
        kids.push(p.expect("(")?);
        if !p.at(")") {
            kids.push(Kid::Node(p.bracketed(expression_list)?));
        }
        kids.push(p.expect(")")?);
    }
    Ok(p.node("mem_init", sp, kids))
}

/// `mem_init_id: identifier | base_type`. A qualified name, a
/// template-id, or `decltype` names a base; a lone identifier names a
/// member or a base.
pub fn mem_init_id(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if p.at("::") || p.ahead(1) == "::" || p.ahead(1) == "<" || p.at("decltype") {
        let b = base_type(p)?;
        return Ok(p.node("mem_init_id", sp, vec![Kid::Node(b)]));
    }
    let id = identifier(p)?;
    Ok(p.node("mem_init_id", sp, vec![id]))
}

// ---------------------------------------------------------------------
// Shapes

fn words(n: &Node, toks: &[Token]) -> Vec<String> {
    let mut ids = Vec::new();
    n.tokens(&mut ids);
    ids.iter().map(|&i| toks[i].text.clone()).collect()
}

/// `ctor_specifier: "constexpr" | "consteval" | "inline" | explicit_spec`.
pub fn ctor_specifier(word: &str) -> bool {
    matches!(word, "constexpr" | "consteval" | "inline" | "explicit")
}

/// `dtor_specifier: "constexpr" | "inline" | "virtual"`.
pub fn dtor_specifier(word: &str) -> bool {
    matches!(word, "constexpr" | "inline" | "virtual")
}

/// The declarator of a declaration node, and its specifiers.
pub fn parts(n: &Node) -> (Option<&Node>, Option<&Node>) {
    let specs = n.find("decl_specifiers");
    let d = n.find("declarator");
    (specs, d)
}

/// `ctor_decl: {ctor_specifier} class_name "(" parameters ")" [noexcept_specifier]`:
/// a declaration whose specifiers are constructor specifiers and whose
/// declarator names `class` with a function suffix.
pub fn ctor_decl(n: &Node, toks: &[Token], class: &str) -> bool {
    let (specs, d) = parts(n);
    let specs_ok = specs.map_or(true, |s| {
        s.children("decl_specifier").all(|ds| {
            let w = words(ds, toks);
            w.first().is_some_and(|f| ctor_specifier(f))
        })
    });
    let Some(d) = d else { return false };
    let w = words(d, toks);
    specs_ok && w.first().map(String::as_str) == Some(class) && w.get(1).map(String::as_str) == Some("(")
}

/// `ctor_def: ctor_decl [ctor_init] function_body`.
pub fn ctor_def(n: &Node, toks: &[Token], class: &str) -> bool {
    n.find("function_definition").is_some_and(|f| f.child("function_body").is_some() && ctor_decl(f, toks, class))
}

/// `dtor_decl: {dtor_specifier} "~" class_name "(" ["void"] ")" [noexcept_specifier]`.
pub fn dtor_decl(n: &Node, toks: &[Token], class: &str) -> bool {
    let (specs, d) = parts(n);
    let specs_ok = specs.map_or(true, |s| s.children("decl_specifier").all(|ds| words(ds, toks).first().is_some_and(|f| dtor_specifier(f))));
    let Some(d) = d else { return false };
    let w = words(d, toks);
    specs_ok && w.len() >= 4 && w[0] == "~" && w[1] == class && w[2] == "(" && (w[3] == ")" || (w[3] == "void" && w.get(4).map(String::as_str) == Some(")")))
}

/// `dtor_call: postfix_expr ("." | "->") [nested_name] "~"
/// (identifier | template_id | decltype_expr) "(" ")"`: a call with no
/// arguments of a member access whose name is `~` and an identifier, a
/// template-id, or a `decltype` specifier.
pub fn dtor_call(n: &Node, toks: &[Token]) -> bool {
    if n.prod != "postfix_expr" || n.kids.len() != 3 {
        return false;
    }
    let (Some(Kid::Node(callee)), Some(Kid::Tok(o)), Some(Kid::Tok(c))) = (n.kids.first(), n.kids.get(1), n.kids.get(2)) else {
        return false;
    };
    if toks[*o].text != "(" || toks[*c].text != ")" || callee.prod != "postfix_expr" {
        return false;
    }
    let Some(u) = callee.kids.last().and_then(|k| match k {
        Kid::Node(id) => id.find("unqualified_id"),
        _ => None,
    }) else {
        return false;
    };
    let starts_with_tilde = matches!(u.kids.first(), Some(Kid::Tok(t)) if toks[*t].text == "~");
    let named = match u.kids.get(1) {
        Some(Kid::Node(x)) => matches!(x.prod, "class_name" | "template_id" | "decltype_expr"),
        _ => false,
    };
    let w = words(callee, toks);
    starts_with_tilde && named && w.iter().any(|x| x == "." || x == "->")
}
