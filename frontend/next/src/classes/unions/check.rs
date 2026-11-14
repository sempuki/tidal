//! The check part of `classes/unions`: the limits on the members of a
//! union (UNI-2, UNI-8) and the members that the rewrite models. A union
//! whose member has a non-trivial special member function (UNI-3,
//! UNI-11), an anonymous union (UNI-7, UNI-9), and a bit-field are
//! unsupported.

use crate::sem::check::Check;
use crate::sem::tree::Init;
use crate::sem::types::{ClassId, QType, Type};
use crate::sem::{ClassKey, Unit};
use crate::source::Span;

/// Whether class `c` is a union.
pub fn is_union(cx: &Check, c: ClassId) -> bool {
    cx.u.classes[c].key == ClassKey::Union
}

/// Whether the special member functions of a type are all trivial and
/// not declared by the program: no constructor, destructor, or
/// assignment that the program provides, no virtual function, and no
/// default member initializer, in the type and in its members.
fn plain(u: &Unit, ty: &QType) -> bool {
    match &ty.t {
        Type::Array(e, _) => plain(u, e),
        Type::Class(c) => {
            let names = &u.scopes[u.classes[*c].scope].names;
            let provided = |n: &str| names.get(n).is_some_and(|fs| fs.iter().any(|f| !u.defaulted.contains(f)));
            let ctor = crate::classes::constructors::check::CTOR;
            let dtor = crate::classes::constructors::check::DTOR;
            !u.polymorphic(*c) && !provided(ctor) && !provided(dtor) && !provided("operator=") && u.classes[*c].fields.iter().all(|&f| u.field(f).is_some_and(|x| x.init.is_none() && plain(u, &x.ty)))
        }
        _ => true,
    }
}

/// A non-static data member of the union `c`: no reference (UNI-2). True
/// when the member is valid; an unsupported member gives false after its
/// diagnostic too.
pub fn member(cx: &mut Check, c: ClassId, ty: &QType, name: &str, span: Span) -> Option<bool> {
    if !is_union(cx, c) {
        return Some(true);
    }
    if ty.is_ref() {
        cx.error(span, "UNI-2", format!("the union member '{}' has a reference type", name));
        return Some(false);
    }
    if !plain(&cx.u, ty) {
        cx.unsupported(span, "a union member of a class with special member functions that are not trivial (build step 4)");
        return None;
    }
    Some(true)
}

/// UNI-8: at most one variant member has a default member initializer.
pub fn end_of_union(cx: &mut Check, c: ClassId, span: Span) {
    if !is_union(cx, c) {
        return;
    }
    let n = cx.u.classes[c].fields.iter().filter(|&&f| cx.u.field(f).is_some_and(|x| x.init.is_some())).count();
    if n > 1 {
        cx.error(span, "UNI-8", "two members of a union have default member initializers");
    }
}

/// The initializer with each inactive member of a union in it (a member
/// whose initializer is default-initialization, UNI-4) taken as having a
/// value, for the checks of a constant initializer (CE-21).
pub fn without_inactive(cx: &Check, ty: &QType, init: &Init) -> Init {
    match (init, &ty.t) {
        (Init::List(xs), Type::Class(c)) => {
            let union = is_union(cx, *c);
            let tys = cx.u.field_types(*c);
            Init::List(xs.iter().enumerate().map(|(k, x)| match (x, tys.get(k)) {
                (Init::Default, _) if union => Init::Zero,
                (_, Some(t)) => without_inactive(cx, t, x),
                _ => x.clone(),
            }).collect())
        }
        (Init::List(xs), Type::Array(e, _)) => Init::List(xs.iter().map(|x| without_inactive(cx, e, x)).collect()),
        _ => init.clone(),
    }
}
