//! The check part of `classes/derived`, for build step 4 part a: classes
//! with non-virtual bases. Each direct base is a subobject that the class
//! holds as an unnamed data member before its declared members (DER-2),
//! so the layout, the initialization, the copies, and the destruction of
//! the members take the bases along. Virtual bases and virtual functions
//! are unsupported.

use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, EK};
use crate::sem::types::{ClassId, QType, Type};
use crate::sem::{Access, ClassKey, EntKind, EntityId, Field, Linkage};
use crate::source::Span;
use crate::syntax::{Kid, Node};

/// The base list of a class head (DER-1): a subobject for each base, in
/// the order of the list. False after an unsupported base.
pub fn base_clause(cx: &mut Check, c: ClassId, key: ClassKey, n: &Node) -> bool {
    if key == ClassKey::Union {
        cx.error(n.span, "DER-1", "a union has a base list");
        return true;
    }
    let mut seen: Vec<ClassId> = Vec::new();
    for b in n.children("base_spec") {
        if cx.has_word(b, "virtual") {
            cx.unsupported(b.span, "a virtual base (build step 4)");
            return false;
        }
        if b.kids.iter().any(|k| matches!(k, Kid::Tok(_)) && cx.text(k) == "...") || cx.has_word(n, "...") {
            cx.unsupported(b.span, "a pack expansion in a base list (build step 5)");
            return false;
        }
        let Some(bt) = b.child("base_type") else { return false };
        let ty = match base_type(cx, bt) {
            Some(t) => t,
            None => return false,
        };
        let Some(d) = ty.class() else {
            if !ty.is_error() {
                cx.error(bt.span, "DER-1", format!("the base {} is not a class", ty.show()));
            }
            return true;
        };
        if d == c {
            cx.error(bt.span, "DER-1", "a class is its own base");
            return true;
        }
        if !cx.u.classes[d].complete {
            cx.error(bt.span, "DER-1", format!("the base '{}' is incomplete", cx.u.classes[d].name));
            return true;
        }
        if cx.u.classes[d].key == ClassKey::Union {
            cx.error(bt.span, "DER-1", format!("the base '{}' is a union", cx.u.classes[d].name));
            return true;
        }
        if cx.u.classes[d].is_final {
            cx.error(bt.span, "DER-1", format!("the base '{}' is final", cx.u.classes[d].name));
            return true;
        }
        if seen.contains(&d) {
            cx.error(bt.span, "DER-1", format!("the base '{}' is named twice", cx.u.classes[d].name));
            return true;
        }
        if crate::expressions::lambdas::check::has_captures(cx, d) || cx.u.closures.contains_key(&d) {
            cx.unsupported(bt.span, "a closure type as a base");
            return false;
        }
        seen.push(d);
        // ACC-1: the access of a base is the specifier's, or the default
        // of the class key.
        let access = match b.child("access_specifier").map(|a| cx.first_word(a)) {
            Some("private") => Access::Private,
            Some("protected") => Access::Protected,
            Some(_) => Access::Public,
            None if key == ClassKey::Class => Access::Private,
            None => Access::Public,
        };
        let index = cx.u.classes[c].fields.len();
        let scope = cx.u.classes[c].scope;
        let f = cx.u.add(crate::sem::Entity {
            name: "".into(),
            kind: EntKind::Field(Field { ty: QType::new(Type::Class(d)), class: c, index, init: None, mutable: false, access, base: true, width: None }),
            span: bt.span,
            scope,
            linkage: Linkage::None,
            deprecated: None,
        });
        cx.u.classes[c].fields.push(f);
    }
    true
}

/// The class that a `base_type` names: a class name, possibly qualified,
/// or a type alias of a class.
fn base_type(cx: &mut Check, n: &Node) -> Option<QType> {
    if n.child("template_id").is_some() || n.child("decltype_expr").is_some() || cx.has_word(n, "template") {
        cx.unsupported(n.span, "a base named by a template-id or decltype (build step 5)");
        return None;
    }
    let name_node = n.child("class_name")?;
    let name = cx.source(name_node);
    let found = match n.child("nested_name") {
        Some(nn) => match crate::foundations::names::check::nested_name(cx, nn) {
            crate::foundations::names::check::Qual::Scope(s) => crate::foundations::names::check::lookup_in(cx, s, &name),
            _ => return None,
        },
        None => crate::foundations::names::check::lookup_where(cx, &name, |k| matches!(k, EntKind::Class(_) | EntKind::Typedef(_) | EntKind::Unsupported(_))),
    };
    for e in found {
        match &cx.u.entities[e].kind {
            EntKind::Class(d) => return Some(QType::new(Type::Class(*d))),
            EntKind::Typedef(t) => return Some(t.clone()),
            EntKind::Unsupported(_) => {
                cx.unsupported(n.span, format!("the base '{}', which the front end does not model", name));
                return None;
            }
            _ => {}
        }
    }
    cx.not_found(name_node.span, "NAM-8", &name);
    Some(QType::error())
}

/// The paths from a class to the subobjects of class `to` in it: each a
/// list of positions among the data members.
pub fn paths(cx: &Check, from: ClassId, to: ClassId) -> Vec<Vec<usize>> {
    cx.u.base_paths(from, to)
}

/// `to` is a base of `from`, direct or not, or `from` itself.
pub fn derives(cx: &Check, from: ClassId, to: ClassId) -> bool {
    !paths(cx, from, to).is_empty()
}

/// `to` is a proper base of `from`.
pub fn is_base(cx: &Check, from: ClassId, to: ClassId) -> bool {
    from != to && derives(cx, from, to)
}

/// DER-10: the one path to the base, or an error for an ambiguous base.
fn unique_path(cx: &mut Check, from: ClassId, to: ClassId, span: Span) -> Option<Vec<usize>> {
    let ps = paths(cx, from, to);
    if ps.len() > 1 {
        cx.error(span, "DER-10", format!("the base '{}' occurs as more than one subobject of '{}'", cx.u.classes[to].name, cx.u.classes[from].name));
        return None;
    }
    ps.into_iter().next()
}

/// The access of the base subobject along a path, the most restrictive
/// of the base specifiers.
fn path_access(cx: &Check, from: ClassId, path: &[usize]) -> Access {
    let mut c = from;
    let mut acc = Access::Public;
    for &i in path {
        let f = cx.u.classes[c].fields[i];
        let x = cx.u.field(f).unwrap();
        if x.access != Access::Public {
            acc = Access::Private;
        }
        c = x.ty.class().unwrap();
    }
    acc
}

/// The function being checked is a member of `c` or of a class derived
/// from it.
fn inside(cx: &Check, c: ClassId) -> bool {
    cx.func.as_ref().and_then(|f| f.id).and_then(|id| cx.u.func(id)).and_then(|f| f.class).is_some_and(|k| derives(cx, k, c))
}

/// DER-5: the base subobject of class `to` of the glvalue `e`.
pub fn to_base(cx: &mut Check, e: Expr, to: ClassId, span: Span) -> Option<Expr> {
    let from = e.ty.class()?;
    let path = unique_path(cx, from, to, span)?;
    // ACC-3: an inaccessible base does not convert.
    if !base_accessible(cx, from, &path) {
        cx.error(span, "ACC-3", format!("the base '{}' of '{}' is not accessible", cx.u.classes[to].name, cx.u.classes[from].name));
        return None;
    }
    Some(along(cx, e, from, &path))
}

/// The subobject at the end of a path, with the cv-qualifiers of `e`.
pub fn along(cx: &Check, e: Expr, from: ClassId, path: &[usize]) -> Expr {
    let (cv, cat, span) = ((e.ty.c, e.ty.v), e.cat, e.span);
    let mut out = e;
    let mut c = from;
    for &i in path {
        let f = cx.u.classes[c].fields[i];
        let x = cx.u.field(f).unwrap();
        let t = x.ty.clone().with_cv(cv.0, cv.1);
        c = x.ty.class().unwrap();
        out = Expr::new(EK::Member(Box::new(out), i), t, cat, span);
    }
    out
}

/// DER-5: a pointer to a derived class converted to a pointer to its
/// base `to`; a null pointer stays null.
pub fn ptr_to_base(cx: &mut Check, e: Expr, to: QType, span: Span) -> Option<Expr> {
    let from = e.ty.pointee()?.class()?;
    let c = to.pointee()?.class()?;
    let path = unique_path(cx, from, c, span)?;
    if !base_accessible(cx, from, &path) {
        cx.error(span, "ACC-3", format!("the base '{}' of '{}' is not accessible", cx.u.classes[c].name, cx.u.classes[from].name));
        return None;
    }
    Some(Expr::new(EK::BasePtr(Box::new(e), path), to.unqual(), Cat::PR, span))
}

/// The result of a member lookup in a class (DER-3, DER-4): the members
/// that the class or its bases declare, the class that declares them, and
/// the path to its subobject. Err for an ambiguity.
#[derive(Clone, Debug)]
pub struct Found {
    pub ents: Vec<EntityId>,
    pub class: ClassId,
    pub path: Vec<usize>,
}

pub fn member_lookup(cx: &Check, c: ClassId, name: &str) -> Result<Option<Found>, ()> {
    let mut own = crate::foundations::names::check::declared_here(cx, cx.u.classes[c].scope, name);
    // NAM-35: a base's name is a member of it.
    if own.is_empty() && *cx.u.classes[c].name == *name {
        own.extend(injected(cx, c));
    }
    if !own.is_empty() {
        return Ok(Some(Found { ents: own, class: c, path: Vec::new() }));
    }
    let mut results: Vec<Found> = Vec::new();
    for (i, b) in cx.u.bases(c) {
        if let Some(mut f) = member_lookup(cx, b, name)? {
            f.path.insert(0, i);
            results.push(f);
        }
    }
    let Some(first) = results.first().cloned() else { return Ok(None) };
    // DER-4: one declaring class through every path, and for a
    // non-static member one path.
    let shared = |e: EntityId| match &cx.u.entities[e].kind {
        EntKind::Var(_) | EntKind::Enumerator { .. } | EntKind::Typedef(_) | EntKind::Class(_) | EntKind::Enum(_) => true,
        EntKind::Func(f) => f.member_static,
        _ => false,
    };
    for r in &results[1..] {
        if r.class != first.class || !first.ents.iter().all(|&e| shared(e)) {
            return Err(());
        }
    }
    Ok(Some(first))
}

/// The class that holds a member found in scope `s` through a class
/// `naming`, with the path to it, for a name that lookup found.
pub fn path_to(cx: &Check, naming: ClassId, member: EntityId) -> Option<Vec<usize>> {
    let owner = cx.u.class_of_scope(cx.u.entities[member].scope)?;
    let ps = paths(cx, naming, owner);
    (ps.len() == 1).then(|| ps.into_iter().next().unwrap())
}

/// ACC-3: the access of a member as a member of `naming`, reached along
/// `path` to its class where its access is `access`. Each base specifier
/// on the way limits it, and a private member of a base has none (None).
fn access_as_member(cx: &Check, naming: ClassId, path: &[usize], access: Access) -> Option<Access> {
    let rank = |a: Access| match a {
        Access::Public => 0,
        Access::Protected => 1,
        Access::Private => 2,
    };
    // The base specifiers from the naming class down to the owner.
    let mut specs = Vec::new();
    let mut c = naming;
    for &i in path {
        let f = cx.u.classes[c].fields[i];
        let x = cx.u.field(f).unwrap();
        specs.push(x.access);
        c = x.ty.class().unwrap();
    }
    let mut acc = access;
    for s in specs.into_iter().rev() {
        if acc == Access::Private {
            return None;
        }
        if rank(s) > rank(acc) {
            acc = s;
        }
    }
    Some(acc)
}

/// The class of the function being checked.
fn here(cx: &Check) -> Option<ClassId> {
    cx.func.as_ref().and_then(|f| f.id).and_then(|id| cx.u.func(id)).and_then(|f| f.class)
}

/// ACC-2, ACC-3, ACC-4: a member named through the class `naming`, which
/// the class of the member or a class derived from it, is accessible
/// where the function being checked is.
pub fn accessible_through(cx: &Check, member: EntityId, naming: ClassId) -> bool {
    let access = match &cx.u.entities[member].kind {
        EntKind::Field(x) => x.access,
        _ => cx.u.access.get(&member).copied().unwrap_or(Access::Public),
    };
    let Some(owner) = cx.u.class_of_scope(cx.u.entities[member].scope) else { return true };
    let path = paths(cx, naming, owner).into_iter().next().unwrap_or_default();
    let x = here(cx);
    // ACC-5: a friend of the class that declares the member.
    if x == Some(owner) || crate::classes::access::check::befriended(cx, owner) {
        return true;
    }
    // A declaration in the class, or in a class nested in it (ACC-10).
    let home = cx.u.classes[owner].scope;
    let mut s = Some(cx.scope);
    while let Some(k) = s {
        if k == home {
            return true;
        }
        s = cx.u.scopes[k].parent;
    }
    match access_as_member(cx, naming, &path, access) {
        Some(Access::Public) => true,
        Some(Access::Protected) => x.is_some_and(|h| derives(cx, h, naming) || derives(cx, naming, h)),
        Some(Access::Private) => x == Some(naming),
        None => false,
    }
}

/// ACC-3: the base `to` of `from` is unambiguous and accessible.
pub fn base_is_accessible(cx: &Check, from: ClassId, to: ClassId) -> bool {
    let ps = paths(cx, from, to);
    ps.len() == 1 && base_accessible(cx, from, &ps[0])
}

/// ACC-3: the base subobject at the end of `path` is accessible as an
/// invented public member of it would be.
fn base_accessible(cx: &Check, naming: ClassId, path: &[usize]) -> bool {
    let x = here(cx);
    match access_as_member(cx, naming, path, Access::Public) {
        Some(Access::Public) => true,
        Some(Access::Protected) => x.is_some_and(|h| derives(cx, h, naming)),
        Some(Access::Private) => x == Some(naming),
        None => false,
    }
}

/// ACC-14: an unqualified name that finds the injected name of a base
/// is accessible as that base's public member, named through the class
/// whose scope finds it. False when a base on the way makes the name
/// inaccessible.
pub fn injected_accessible(cx: &Check, name: &str) -> bool {
    let mut s = Some(cx.scope);
    while let Some(sc) = s {
        if cx.u.scopes[sc].names.contains_key(name) {
            return true;
        }
        if let Some(c) = cx.u.class_of_scope(sc) {
            match member_lookup(cx, c, name) {
                Ok(Some(f)) if !f.path.is_empty() && f.ents.iter().any(|&e| injected(cx, f.class) == Some(e)) => {
                    // In a member declaration, the class being defined.
                    return match here(cx).or(cx.member_of) {
                        Some(h) => base_accessible_in(cx, c, &f.path, h),
                        None => base_accessible(cx, c, &f.path),
                    };
                }
                Ok(None) => {}
                _ => return true,
            }
        }
        s = cx.u.scopes[sc].parent;
    }
    true
}

/// ACC-3 from inside class `inside`: the base subobject at the end of
/// `path` from class `naming` is accessible there (VIRT-4).
pub fn base_accessible_in(cx: &Check, naming: ClassId, path: &[usize], inside: ClassId) -> bool {
    match access_as_member(cx, naming, path, Access::Public) {
        Some(Access::Public) => true,
        Some(Access::Protected) => derives(cx, inside, naming),
        Some(Access::Private) => inside == naming,
        None => false,
    }
}

/// The class entity of a class's injected name (NAM-33, NAM-35): the
/// declaration of the class in its enclosing scope.
pub fn injected(cx: &Check, c: ClassId) -> Option<EntityId> {
    let s = cx.u.scopes[cx.u.classes[c].scope].parent?;
    let name = cx.u.classes[c].name.clone();
    cx.u.scopes[s].names.get(&name)?.iter().copied().find(|&e| matches!(cx.u.entities[e].kind, EntKind::Class(k) if k == c))
}
