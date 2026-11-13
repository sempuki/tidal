//! The check part of `expressions/comma`.

use crate::expressions::check::{bad, expr, nodes};
use crate::foundations::values::check::deref_ref;
use crate::sem::check::Check;
use crate::sem::tree::{Expr, EK};
use crate::syntax::Node;

/// `expression`: the comma operator (COMMA-1 to COMMA-4). The result is
/// the right operand, with its type and value category.
pub fn expression(cx: &mut Check, n: &Node) -> Expr {
    let ns = nodes(n);
    let mut it = ns.into_iter();
    let Some(first) = it.next() else { return bad(n.span) };
    let mut acc = expr(cx, first);
    for c in it {
        if c.prod == "braced_init" {
            cx.error(c.span, "COMMA-9", "a braced list as an operand of the comma operator");
            return bad(n.span);
        }
        let r = deref_ref(expr(cx, c));
        if acc.ty.is_error() || r.ty.is_error() {
            acc = bad(n.span);
            continue;
        }
        // OPR-6: an operand of class or enumeration type can select an
        // operator function; a void operand cannot (COMMA-7).
        if !acc.ty.is_void() && !r.ty.is_void() {
            if let Some(e) = crate::classes::operators::check::binary(cx, ",", &acc, &r, n.span) {
                acc = e;
                continue;
            }
        }
        crate::expressions::decltype::check::no_effect_warning(cx, &acc, "COMMA-5", "the left operand of the comma operator");
        let (t, cat) = (r.ty.clone(), r.cat);
        acc = Expr::new(EK::Comma(Box::new(acc), Box::new(r)), t, cat, n.span);
    }
    acc
}
