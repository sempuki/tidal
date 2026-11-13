//! The check part of `declarations/type-names`: `typedef` and alias
//! declarations (TYN-1, TYN-2).

use crate::declarations::declarators::check::Declared;
use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::types::QType;
use crate::sem::{EntKind, Linkage};
use crate::source::Span;
use crate::syntax::{Kid, Node};

/// Declare a type alias. TYN-2: a redeclaration in the same scope must
/// name the same type.
pub fn declare_alias(cx: &mut Check, name: &str, ty: QType, span: Span) {
    for e in names::declared_here(cx, cx.scope, name) {
        if let EntKind::Typedef(old) = &cx.u.entities[e].kind {
            if old.is_error() || ty.is_error() || *old == ty {
                return;
            }
            cx.error(span, "TYN-2", format!("the alias '{}' is declared again for a different type", name));
            return;
        }
    }
    names::declare(cx, name, EntKind::Typedef(ty), span, Linkage::None);
}

/// A `typedef` declarator.
pub fn typedef_name(cx: &mut Check, d: &Declared) {
    let Some((name, span)) = &d.name else {
        cx.error(d.span, "TYN-2", "a typedef with no name");
        return;
    };
    if d.placeholder.is_some() && !d.trailing {
        cx.error(d.span, "TYN-2", "a typedef with a placeholder type");
        return;
    }
    declare_alias(cx, name, d.ty.clone(), *span);
}

/// TYN-2: a typedef has no initializer.
pub fn typedef_with_init(cx: &mut Check, span: Span) {
    cx.error(span, "TYN-2", "a typedef with an initializer");
}

/// `alias_decl: "using" identifier {attribute_spec} "=" type_id ";"`.
pub fn alias_decl(cx: &mut Check, n: &Node) {
    let name = n.kids.iter().filter(|k| matches!(k, Kid::Tok(_))).map(|k| (cx.text(k), cx.token(k).map(|t| t.span))).nth(1);
    let Some((name, Some(span))) = name else { return };
    let Some(t) = n.child("type_id") else { return };
    let saved = std::mem::replace(&mut cx.naming_type, true);
    let ty = crate::declarations::declarators::check::type_id(cx, t);
    cx.naming_type = saved;
    declare_alias(cx, name, ty, span);
}

/// A declaration with specifiers and no declarator that declares no
/// type (TYN-7, the qualified class key; the same rule covers a bare
/// type).
pub fn declares_nothing(cx: &mut Check, span: Span) {
    cx.error(span, "TYN-7", "the declaration declares nothing");
}
