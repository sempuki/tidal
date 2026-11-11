//! The expressions chapter. Its index page holds the table of levels and
//! no production; each level's production sits on its operator's page:
//!
//! ```text
//! expression > assignment_expr > conditional_expr > logical_or_expr
//!   > logical_and_expr > or_expr > xor_expr > and_expr > equality_expr
//!   > relational_expr > three_way_expr > shift_expr > add_expr > mul_expr
//!   > pm_expr > cast_expr > unary_expr > postfix_expr > primary
//! ```

pub mod additive;
pub mod assignment;
pub mod bitwise;
pub mod casts;
pub mod comma;
pub mod comparison;
pub mod conditional;
pub mod decltype;
pub mod lambdas;
pub mod logical;
pub mod member_pointers;
pub mod multiplicative;
pub mod new_delete;
pub mod postfix;
pub mod primary;
pub mod shift;
pub mod unary;

use crate::syntax::{Kid, Node, PResult, Parser};

/// Whether `op` is an operator here. Inside a template argument list a
/// `>` closes the list, so `>`, `>=`, `>>`, and `>>=` are no operators
/// there until a bracket opens.
pub fn operator_here(p: &Parser, op: &str) -> bool {
    p.at(op) && (p.gt_ok || !op.starts_with('>'))
}

/// One level of the chain: `next {op next}`, left to right. A level
/// makes a node only when its operator is present.
pub fn binary(p: &mut Parser, prod: &'static str, ops: &[&str], next: fn(&mut Parser) -> PResult<Node>) -> PResult<Node> {
    let sp = p.span();
    let first = next(p)?;
    if !ops.iter().any(|o| operator_here(p, o)) {
        return Ok(first);
    }
    let mut left = first;
    while ops.iter().any(|o| operator_here(p, o)) {
        let op = p.bump();
        let right = next(p)?;
        left = p.node(prod, sp, vec![Kid::Node(left), op, Kid::Node(right)]);
    }
    Ok(left)
}
