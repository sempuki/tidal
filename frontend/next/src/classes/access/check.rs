//! The check part of `classes/access`: friend declarations (ACC-5 to
//! ACC-7, ACC-17). A friend function that a class first declares is a
//! function of the enclosing namespace that only argument-dependent
//! lookup finds until the namespace declares it.

use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::types::ClassId;
use crate::sem::{EntKind, ScopeKind};
use crate::syntax::Node;

/// A `friend_decl` in class `c`. True when the declaration is valid;
/// false after an unsupported form.
pub fn friend_decl(cx: &mut Check, c: ClassId, n: &Node) -> bool {
    // `friend class X;`: the class X, or a new class of the nearest
    // namespace.
    if let Some(k) = n.child("class_key") {
        let _ = k;
        if n.child("nested_name").is_some() {
            cx.unsupported(n.span, "a friend class with a qualified name");
            return false;
        }
        let Some(name) = n.child("class_name").map(|x| cx.source(x)) else { return true };
        let found = names::lookup_where(cx, &name, |k| matches!(k, EntKind::Class(_)));
        let friend = match found.first().map(|&e| cx.u.entities[e].kind.clone()) {
            Some(EntKind::Class(f)) => f,
            _ => {
                let saved = cx.scope;
                cx.scope = cx.u.enclosing_namespace(cx.scope);
                let t = crate::classes::members::check::elaborated_class(cx, n);
                cx.scope = saved;
                let Some(f) = t.class() else { return true };
                f
            }
        };
        cx.u.friend_classes.entry(c).or_default().push(friend);
        return true;
    }
    // A friend function defined in its class: declared now, its body
    // checked with the class's other bodies (`definition`).
    let fd = n.child("function_definition");
    // A friend defined in a nested or local class waits, as the class's
    // member functions do.
    let parent = cx.u.scopes[cx.u.classes[c].scope].parent.unwrap_or(0);
    if fd.is_some() && cx.u.is_local(parent) {
        cx.unsupported(n.span, "a friend function defined in a local class (build step 4)");
        return false;
    }
    let Some(dn) = fd.unwrap_or(n).child("declarator") else {
        cx.unsupported(n.span, "a friend declaration of this form (build step 4)");
        return false;
    };
    let specs = match fd.unwrap_or(n).child("decl_specifiers") {
        Some(ds) => crate::declarations::check::decl_specifiers(cx, ds, Default::default()),
        None => {
            cx.unsupported(n.span, "a friend declaration of this form (build step 4)");
            return false;
        }
    };
    if specs.unsupported {
        return false;
    }
    // `friend X;` with a type: a class is a friend, another type is
    // ignored (ACC-5).
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    if d.unsupported {
        return false;
    }
    if !d.ty.is_func() {
        cx.unsupported(n.span, "a friend declaration that names a type (build step 4)");
        return false;
    }
    if let names::Qual::Scope(s) = d.qual {
        if fd.is_some() {
            cx.unsupported(n.span, "a friend definition with a qualified name");
            return false;
        }
        return qualified_friend(cx, c, s, &d, n);
    }
    if d.qual != names::Qual::None {
        return false;
    }
    // DEFARG-11: a friend declaration with a default argument is a
    // definition.
    if fd.is_none() && d.params.iter().any(|p| p.default.is_some()) {
        cx.error(n.span, "DEFARG-11", "a friend declaration with a default argument that is not a definition");
        return true;
    }
    // The function belongs to the nearest enclosing namespace.
    let ns = cx.u.enclosing_namespace(cx.scope);
    let saved_member = cx.member_of.take();
    let before = d.name.as_ref().map(|(name, _)| names::declared_here(cx, ns, name)).unwrap_or_default();
    cx.friend_decl = true;
    cx.friend_scope = Some(ns);
    // The default arguments of a friend defined in its class see the
    // complete class (DEFARG-3), so its definition gives them.
    let mut d = d;
    if fd.is_some() {
        for p in &mut d.params {
            p.default = None;
        }
    }
    let f = crate::declarations::functions::definitions::check::declare_function(cx, &specs, &d, None);
    cx.friend_decl = false;
    cx.friend_scope = None;
    cx.member_of = saved_member;
    let Some(f) = f else { return true };
    if !before.contains(&f) {
        cx.u.hidden.insert(f);
    }
    if fd.is_some() {
        if let Some(x) = cx.u.func_mut(f) {
            x.inline = true;
        }
    }
    cx.u.friends.entry(c).or_default().push(f);
    true
}

/// ACC-5: `friend R N::f(P);` or `friend R C::f(P);` makes the function
/// of that namespace or class with the same type a friend of class `c`.
/// A member function of another class needs access from `c` (ACC-15).
fn qualified_friend(cx: &mut Check, c: ClassId, s: crate::sem::ScopeId, d: &crate::declarations::declarators::check::Declared, n: &Node) -> bool {
    let Some((name, span)) = d.name.clone() else { return true };
    let Some(fty) = d.ty.func().cloned() else { return true };
    let found = names::declared_here(cx, s, &name).into_iter().find(|&e| cx.u.func(e).is_some_and(|x| x.ty.params == fty.params && x.ty.ret == fty.ret && x.obj_cv == d.obj_cv));
    let Some(f) = found else {
        cx.error(span, "ACC-5", format!("the friend declaration names '{}', which its scope does not declare with this type", name));
        return true;
    };
    if cx.u.func(f).is_some_and(|x| x.class.is_some()) && !crate::classes::members::check::accessible(cx, f) {
        cx.error(n.span, "ACC-15", format!("the friend declaration names the member function '{}', which is not accessible here", name));
        return true;
    }
    cx.u.friends.entry(c).or_default().push(f);
    true
}

/// ACC-5: the place being checked is a friend of class `c`: a friend
/// function's body, or a member or the definition of a friend class.
/// The friend of a friend is none (ACC-6).
pub fn befriended(cx: &Check, c: ClassId) -> bool {
    let func = cx.func.as_ref().and_then(|x| x.id);
    if func.is_some_and(|f| cx.u.friends.get(&c).is_some_and(|fs| fs.contains(&f))) {
        return true;
    }
    let Some(classes) = cx.u.friend_classes.get(&c) else { return false };
    if func.and_then(|f| cx.u.func(f)).and_then(|x| x.class).is_some_and(|k| classes.contains(&k)) {
        return true;
    }
    let mut s = Some(cx.scope);
    while let Some(x) = s {
        if cx.u.scopes[x].kind == ScopeKind::Class && classes.iter().any(|&k| cx.u.classes[k].scope == x) {
            return true;
        }
        s = cx.u.scopes[x].parent;
    }
    false
}

/// The body of a friend function defined in class `c`: a function of the
/// nearest namespace whose body looks names up in the class (ACC-7). It
/// is inline.
pub fn definition(cx: &mut Check, n: &Node) {
    let Some(fd) = n.child("function_definition") else { return };
    let ns = cx.u.enclosing_namespace(cx.scope);
    let saved_member = cx.member_of.take();
    cx.friend_decl = true;
    cx.friend_scope = Some(ns);
    crate::declarations::functions::definitions::check::function_definition(cx, fd);
    cx.friend_decl = false;
    cx.friend_scope = None;
    cx.member_of = saved_member;
}
