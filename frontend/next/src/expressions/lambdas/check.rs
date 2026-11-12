//! The check part of `expressions/lambdas`. A lambda's closure is a
//! class, which comes with build step 4; build step 3 reports a lambda
//! expression as unsupported.

use crate::expressions::check::bad;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, EK};
use crate::sem::EntityId;
use crate::source::Span;
use crate::syntax::Node;

/// `lambda_expr`.
pub fn lambda_expr(cx: &mut Check, n: &Node) -> Expr {
    let t = cx.unsupported(n.span, "a lambda expression (its closure type is a class, build step 4)");
    let _ = bad;
    Expr::new(EK::Int(0), t, Cat::PR, n.span)
}

/// The check of a use of a variable inside a lambda body: none while
/// build step 3 has no lambdas.
pub fn capture_check(_cx: &mut Check, _var: EntityId, _span: Span) -> Option<Result<(), ()>> {
    None
}
