//! The Syntax segment of `statements/if`.

use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::declarators::syntax::declarator;
use crate::declarations::initialization::list::syntax::braced_init;
use crate::declarations::type_names::syntax::alias_decl;
use crate::declarations::structured_bindings::syntax::sb_decl;
use crate::declarations::{decl_specifiers, looks_like_sb, simple_decl};
use crate::expressions::assignment::syntax::assignment_expr;
use crate::expressions::comma::syntax::expression;
use crate::statements::blocks::syntax::{compound_stmt, expression_stmt, null_stmt};
use crate::statements::statement;
use crate::syntax::names::{DeclCtx, Enter};
use crate::syntax::{Kid, Node, PResult, Parser};

/// `if_stmt: "if" ["constexpr"] "(" [init_stmt] condition ")" branch ["else" branch]
/// | "if" ["!"] "consteval" block ["else" block]`.
pub fn if_stmt(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = vec![p.expect("if")?];
    // IF-20: `!` goes only before `consteval`.
    if p.at("!") && p.ahead(1) != "consteval" {
        return Err(p.fail_rule("IF-20", "'!' after 'if' needs 'consteval'"));
    }
    if p.at("consteval") || (p.at("!") && p.ahead(1) == "consteval") {
        if p.at("!") {
            kids.push(p.bump());
        }
        kids.push(p.bump());
        kids.push(Kid::Node(block(p)?));
        if p.at("else") {
            kids.push(p.bump());
            kids.push(Kid::Node(block(p)?));
        }
        return Ok(p.node("if_stmt", sp, kids));
    }
    if let Some(k) = p.eat("constexpr") {
        kids.push(k);
    }
    p.names.enter(Enter::Block);
    let r = (|| -> PResult<()> {
        kids.extend(head(p)?);
        kids.push(Kid::Node(branch(p)?));
        if p.at("else") {
            kids.push(p.bump());
            kids.push(Kid::Node(branch(p)?));
        }
        Ok(())
    })();
    p.names.leave();
    r?;
    Ok(p.node("if_stmt", sp, kids))
}

/// `"(" [init_stmt] condition ")"`, shared with `switch_stmt`.
pub fn head(p: &mut Parser) -> PResult<Vec<Kid>> {
    let mut kids = vec![p.expect("(")?];
    p.bracketed(|p| -> PResult<()> {
        other_declaration(p, "IF-21")?;
        if let Ok(i) = p.attempt(|p| {
            let i = init_stmt(p)?;
            if p.at(")") {
                Err(p.fail("a condition"))
            } else {
                Ok(i)
            }
        }) {
            kids.push(Kid::Node(i));
        }
        kids.push(Kid::Node(condition(p)?));
        Ok(())
    })?;
    kids.push(p.expect(")")?);
    Ok(kids)
}

/// A declaration that is not an init-statement: `static_assert`, a
/// using-declaration, or a using-directive, which `rule` rejects.
pub fn other_declaration(p: &mut Parser, rule: &'static str) -> PResult<()> {
    if p.at("static_assert") || (p.at("using") && (p.ahead(1) == "namespace" || p.ahead(2) != "=")) {
        return Err(p.fail_rule(rule, "a static_assert, a using-declaration, or a using-directive is not an init-statement"));
    }
    Ok(())
}

/// `init_stmt: expression_stmt | null_stmt | simple_decl | sb_decl | alias_decl`.
pub fn init_stmt(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let sb = if looks_like_sb(p) { p.opt(sb_decl) } else { None };
    let n = if let Some(n) = sb {
        n
    } else if p.at(";") {
        null_stmt(p)?
    } else if p.at("using") {
        alias_decl(p)?
    } else {
        match p.attempt(simple_decl) {
            Ok(n) => n,
            Err(_) => expression_stmt(p)?,
        }
    };
    Ok(p.node("init_stmt", sp, vec![Kid::Node(n)]))
}

/// `condition: expression | cond_decl`.
pub fn condition(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let n = match p.attempt(cond_decl) {
        Ok(n) => n,
        Err(_) => expression(p)?,
    };
    Ok(p.node("condition", sp, vec![Kid::Node(n)]))
}

/// `cond_decl: {attribute_spec} decl_specifiers declarator
/// ("=" assignment_expr | "=" braced_init | braced_init)`.
pub fn cond_decl(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    let mut kids = Vec::new();
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    let specs = decl_specifiers(p)?;
    let d = declarator(p)?;
    if p.at(")") {
        return Err(p.fail_rule("IF-6", "a condition declaration needs an initializer"));
    }
    if !(p.at("=") || p.at("{")) {
        return Err(p.fail("'=' or '{'"));
    }
    p.declared(&d, DeclCtx { specs: Some(&specs), template: false, param: false });
    kids.push(Kid::Node(specs));
    kids.push(Kid::Node(d));
    if p.at("{") {
        kids.push(Kid::Node(braced_init(p)?));
    } else {
        kids.push(p.bump());
        kids.push(Kid::Node(if p.at("{") { braced_init(p)? } else { assignment_expr(p)? }));
    }
    if p.at(",") {
        // IF-6: one declarator. The rule's stop goes at the `)` that ends
        // the condition, where the reading as an init-statement stops.
        let m = p.mark();
        let mut depth = 0;
        while !p.at_eof() && !(depth == 0 && p.at(")")) {
            if p.at("(") {
                depth += 1;
            } else if p.at(")") {
                depth -= 1;
            }
            p.bump();
        }
        p.fail_rule("IF-6", "a condition declaration declares one variable");
        p.reset(m);
    }
    Ok(p.node("cond_decl", sp, kids))
}

/// `branch: statement`.
pub fn branch(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    p.names.enter(Enter::Block);
    let s = statement(p);
    p.names.leave();
    Ok(p.node("branch", sp, vec![Kid::Node(s?)]))
}

/// `block: compound_stmt`.
pub fn block(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if !p.at("{") {
        return Err(p.fail_rule("IF-20", "the branches of 'if consteval' are compound statements"));
    }
    let c = compound_stmt(p)?;
    Ok(p.node("block", sp, vec![Kid::Node(c)]))
}
