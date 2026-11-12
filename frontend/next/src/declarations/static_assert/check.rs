//! The check part of `declarations/static-assert`.

use crate::foundations::constant_evaluation::check as ce;
use crate::sem::check::Check;
use crate::syntax::Node;

/// `static_assert_decl` (SA-1, SA-3, SA-7): the condition is a constant
/// expression that converts to `bool`, and it must be true.
pub fn static_assert_decl(cx: &mut Check, n: &Node) {
    let Some(c) = n.kids.iter().find_map(|k| match k {
        crate::syntax::Kid::Node(c) => Some(c),
        _ => None,
    }) else {
        return;
    };
    match ce::bool_constant(cx, c, "SA-7") {
        Some(true) | None => {}
        Some(false) => {
            let msg = n.kids.iter().filter_map(|k| cx.token(k)).find_map(|t| match &t.lit {
                Some(crate::lex::Lit::Str { units, .. }) => Some(units.iter().filter_map(|&u| char::from_u32(u)).collect::<String>()),
                _ => None,
            });
            let text = match msg {
                Some(m) => format!("static assertion failed: {}", m),
                None => "static assertion failed".into(),
            };
            cx.error(n.span, "SA-1", text);
        }
    }
}
