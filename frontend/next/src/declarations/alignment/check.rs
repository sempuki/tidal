//! The check part of `declarations/alignment`: `alignas` on a variable.

use crate::foundations::constant_evaluation::check as ce;
use crate::sem::check::Check;
use crate::sem::types::QType;
use crate::syntax::Node;

/// The alignment that the `alignas` specifiers ask for (ALN-1, ALN-2):
/// a constant power of two, not weaker than the type's own alignment.
pub fn alignas(cx: &mut Check, specs: &[Node], ty: &QType) -> Option<u64> {
    let mut best: Option<u64> = None;
    for a in specs {
        let Some(e) = a.kids.iter().find_map(|k| match k {
            crate::syntax::Kid::Node(c) => Some(c),
            _ => None,
        }) else {
            continue;
        };
        if e.prod == "type_id" {
            let t = crate::declarations::declarators::check::type_id(cx, e);
            if let Some(al) = cx.u.target.align_of(&cx.u, &t) {
                best = Some(best.map_or(al, |b| b.max(al)));
            }
            continue;
        }
        let Some(v) = ce::integral_constant(cx, e, "ALN-2") else { continue };
        if v == 0 {
            continue;
        }
        if v < 0 || (v & (v - 1)) != 0 {
            cx.error(a.span, "ALN-2", format!("the alignment {} is not a power of two", v));
            continue;
        }
        best = Some(best.map_or(v as u64, |b| b.max(v as u64)));
    }
    if let (Some(b), Some(own)) = (best, cx.u.target.align_of(&cx.u, ty)) {
        if b < own {
            cx.error(specs[0].span, "ALN-2", "the alignment is weaker than the type's own alignment");
        }
    }
    best
}

/// ALN-3: the declarations of a variable agree on `alignas`.
pub fn redeclared(cx: &mut Check, span: crate::source::Span) {
    cx.error(span, "ALN-3", "a redeclaration with a different alignas");
}
