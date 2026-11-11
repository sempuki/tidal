//! The Syntax segment of `statements/index`.

use crate::declarations::attributes::syntax::attribute_spec;
use crate::declarations::block_decl;
use crate::exceptions::handlers::syntax::try_block;
use crate::lex::TK;
use crate::statements::blocks::syntax::{compound_stmt, expression_stmt, labeled_stmt, null_stmt};
use crate::statements::if_stmt::syntax::if_stmt;
use crate::statements::jumps::syntax::jump_stmt;
use crate::statements::loops::syntax::{do_stmt, for_stmt, while_stmt};
use crate::statements::range_for::syntax::range_for_stmt;
use crate::statements::switch::syntax::switch_stmt;
use crate::syntax::{Kid, Node, PResult, Parser};

/// The word after any attributes at the start of a statement.
fn word_after_attrs(p: &mut Parser) -> (&'static str, bool) {
    let m = p.mark();
    let mut any = false;
    while p.at_attr() {
        if attribute_spec(p).is_err() {
            break;
        }
        any = true;
    }
    let w = match p.peek() {
        "{" => "{",
        ";" => ";",
        "if" => "if",
        "switch" => "switch",
        "while" => "while",
        "do" => "do",
        "for" => "for",
        "return" | "break" | "continue" | "goto" => "jump",
        "try" => "try",
        "case" => "label",
        "default" if p.ahead(1) == ":" => "label",
        _ if p.kind() == TK::Ident && p.ahead(1) == ":" => "label",
        _ => "",
    };
    p.reset(m);
    (w, any)
}

/// `statement: labeled_stmt | block_decl | {attribute_spec} plain_stmt`.
/// A form that is a declaration and an expression is a declaration
/// (STMT-6), so the declaration is tried first.
pub fn statement(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("statement") {
        return Ok(n);
    }
    let sp = p.span();
    let (w, _) = word_after_attrs(p);
    if w == "label" {
        let l = labeled_stmt(p)?;
        return Ok(p.node("statement", sp, vec![Kid::Node(l)]));
    }
    if w.is_empty() {
        if let Ok(d) = p.attempt(block_decl) {
            return Ok(p.node("statement", sp, vec![Kid::Node(d)]));
        }
    }
    let mut kids = Vec::new();
    while p.at_attr() {
        kids.push(Kid::Node(attribute_spec(p)?));
    }
    kids.push(Kid::Node(plain_stmt(p)?));
    Ok(p.node("statement", sp, kids))
}

/// `plain_stmt: expression_stmt | null_stmt | compound_stmt | if_stmt
/// | switch_stmt | while_stmt | do_stmt | for_stmt | range_for_stmt
/// | jump_stmt | try_block`.
pub fn plain_stmt(p: &mut Parser) -> PResult<Node> {
    if let Some(n) = p.ext("plain_stmt") {
        return Ok(n);
    }
    let sp = p.span();
    let n = match p.peek() {
        "{" => compound_stmt(p)?,
        ";" => null_stmt(p)?,
        "if" => if_stmt(p)?,
        "switch" => switch_stmt(p)?,
        "while" => while_stmt(p)?,
        "do" => do_stmt(p)?,
        "for" => match p.attempt(range_for_stmt) {
            Ok(n) => n,
            Err(_) => for_stmt(p)?,
        },
        "return" | "break" | "continue" | "goto" => jump_stmt(p)?,
        "try" => try_block(p)?,
        _ => expression_stmt(p)?,
    };
    Ok(p.node("plain_stmt", sp, vec![Kid::Node(n)]))
}
