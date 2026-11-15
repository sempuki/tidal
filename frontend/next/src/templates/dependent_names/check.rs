//! `templates/dependent-names`, the Check segment: the type that a
//! `typename` specifier or a dependent `decltype` names (build step 5).

use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::types::{DepType, QType, Type};
use crate::sem::EntKind;
use crate::syntax::{Kid, Node};
use std::rc::Rc;

/// DEP-19: `typename Q::name`. In a template's definition, a qualifier
/// that names a parameter gives a dependent type that deduction does not
/// look into. Otherwise the name is looked up in the qualifier's scope.
pub fn typename_specifier(cx: &mut Check, n: &Node) -> QType {
    if let Some(nn) = n.child("nested_name") {
        if dependent(cx, nn) {
            let name: Rc<str> = cx.source(n).trim_start_matches("typename").trim().into();
            return QType::new(Type::Dep(Rc::new(DepType::Opaque { name })));
        }
    }
    if n.kids.iter().any(|k| matches!(k, Kid::Node(c) if c.prod != "nested_name")) {
        return cx.unsupported(n.span, "a typename specifier with a template-id after the qualifier");
    }
    crate::foundations::types::check::named_type(cx, n)
}

/// A part of a definition that names a template parameter of the
/// definition being checked: a type parameter, which is a dependent type
/// there, a constant parameter, which has no value there, or a variable
/// of a dependent type.
pub fn dependent(cx: &Check, nn: &Node) -> bool {
    let mut toks = Vec::new();
    nn.tokens(&mut toks);
    toks.iter().any(|&t| {
        names::lookup(cx, &cx.toks[t].text).into_iter().any(|e| match &cx.u.entities[e].kind {
            EntKind::Typedef(t) => crate::templates::deduction::check::has_dep(t),
            EntKind::Var(v) => crate::templates::deduction::check::has_dep(&v.ty) || (v.param && v.init.is_none() && cx.u.scopes[cx.u.entities[e].scope].kind == crate::sem::ScopeKind::Template),
            _ => false,
        })
    })
}
