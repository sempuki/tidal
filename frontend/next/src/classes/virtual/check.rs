//! The check part of `classes/virtual`, for build step 4 part a: virtual
//! functions in classes with non-virtual bases. A member function is
//! virtual when it is declared `virtual` or overrides a virtual function
//! of a base (VIRT-1). `Unit::virt` records each virtual function with
//! the functions it overrides, and the machine's dispatch finds the
//! final overrider of a call. Virtual bases and `dynamic_cast` are
//! unsupported.

use crate::sem::check::Check;
use crate::sem::types::{ClassId, QType, Type};
use crate::sem::{ClassKey, EntityId, Special, Virt};
use crate::source::Span;
use crate::syntax::Node;

/// The virt-specifiers after a member function's declarator: `override`
/// and `final`. None after VIRT-3 rejects a repeated one.
fn specifiers(cx: &mut Check, n: &Node) -> Option<(bool, bool)> {
    let Some(vs) = n.child("virt_specifiers") else { return Some((false, false)) };
    let words = cx.words(vs);
    let count = |w: &str| words.iter().filter(|&&x| x == w).count();
    if count("override") > 1 || count("final") > 1 {
        cx.error(vs.span, "VIRT-3", "a virt-specifier given twice");
        return None;
    }
    Some((count("override") == 1, count("final") == 1))
}

/// The virtual functions of the bases of `c`, direct or not, that a
/// member function `f` of `c` overrides (VIRT-1): those with its name,
/// its parameter types, and its cv-qualifiers.
pub fn overridden(cx: &Check, c: ClassId, f: EntityId) -> Vec<EntityId> {
    let Some(func) = cx.u.func(f) else { return Vec::new() };
    let name = cx.u.entities[f].name.clone();
    let mut out = Vec::new();
    let mut work: Vec<ClassId> = cx.u.bases(c).into_iter().map(|(_, b)| b).collect();
    let mut seen = Vec::new();
    while let Some(b) = work.pop() {
        if seen.contains(&b) {
            continue;
        }
        seen.push(b);
        for g in crate::foundations::names::check::declared_here(cx, cx.u.classes[b].scope, &name) {
            let Some(gf) = cx.u.func(g) else { continue };
            let same = func.special == Special::Dtor || (gf.ty.params == func.ty.params && gf.ty.variadic == func.ty.variadic && gf.obj_cv == func.obj_cv);
            if cx.u.virt.contains_key(&g) && same && !out.contains(&g) {
                out.push(g);
            }
        }
        work.extend(cx.u.bases(b).into_iter().map(|(_, d)| d));
    }
    out
}

/// A member function `f` of class `c` has been declared: record it when
/// it is virtual, and check the limits of VIRT-2, VIRT-3, VIRT-4,
/// VIRT-9, VIRT-14, VIRT-15, and VIRT-21. `n` holds the virt-specifiers
/// and `pure` the pure specifier, if any.
pub fn member_function(cx: &mut Check, c: ClassId, f: EntityId, declared_virtual: bool, n: &Node, pure: Option<&Node>, span: Span) {
    let Some((marked, is_final)) = specifiers(cx, n) else { return };
    let Some(func) = cx.u.func(f).cloned() else { return };
    let over = overridden(cx, c, f);
    if func.member_static {
        if declared_virtual {
            cx.error(span, "VIRT-2", "a static member function declared virtual");
        } else if !over.is_empty() {
            cx.error(span, "VIRT-21", format!("the static member function '{}' has the name and parameters of a virtual function of a base", cx.u.entities[f].name));
        }
        return;
    }
    let is_virtual = declared_virtual || !over.is_empty();
    if is_virtual && cx.u.classes[c].key == ClassKey::Union {
        cx.error(span, "VIRT-2", "a virtual function in a union");
        return;
    }
    if is_virtual && func.placeholder {
        cx.error(span, "VIRT-2", "a virtual function with a deduced return type");
        return;
    }
    if pure.is_some() && !is_virtual {
        cx.error(span, "VIRT-9", "a pure specifier on a function that is not virtual");
        return;
    }
    if is_final && !is_virtual {
        cx.error(span, "VIRT-3", "'final' on a function that is not virtual");
        return;
    }
    if marked && over.is_empty() {
        cx.error(span, "VIRT-3", format!("'{}' is marked override but overrides no virtual function of a base", cx.u.entities[f].name));
        return;
    }
    for &o in &over {
        if cx.u.virt.get(&o).is_some_and(|v| v.is_final) {
            cx.error(span, "VIRT-3", format!("'{}' overrides a final function", cx.u.entities[f].name));
            return;
        }
        let of = cx.u.func(o).cloned().unwrap();
        if of.consteval != func.consteval {
            cx.error(span, "VIRT-14", "a consteval function and a function that is not consteval override each other");
            return;
        }
        if func.special != Special::Dtor && !return_fits(cx, c, &func.ty.ret, &of.ty.ret, span) {
            return;
        }
    }
    if is_virtual {
        cx.u.virt.insert(f, Virt { pure: pure.is_some(), is_final, marked, overrides: over });
    }
}

/// VIRT-4, VIRT-15: the return type of an override in class `c` is the
/// type of the function it overrides, or a pointer or reference to a
/// class derived from that function's class. True when it fits.
fn return_fits(cx: &mut Check, c: ClassId, new: &QType, old: &QType, span: Span) -> bool {
    if new == old {
        return true;
    }
    let target = |t: &QType| -> Option<QType> {
        match &t.t {
            Type::Ptr(p) => Some((**p).clone()),
            Type::Ref(r, _) => Some((**r).clone()),
            _ => None,
        }
    };
    let same_kind = match (&new.t, &old.t) {
        (Type::Ptr(_), Type::Ptr(_)) => true,
        (Type::Ref(_, a), Type::Ref(_, b)) => a == b,
        _ => false,
    };
    let (Some(nt), Some(ot)) = (target(new), target(old)) else { return mismatch(cx, span) };
    let (Some(d), Some(b)) = (nt.class(), ot.class()) else { return mismatch(cx, span) };
    if !same_kind || new.c != old.c || new.v != old.v {
        return mismatch(cx, span);
    }
    if d != c && !cx.u.classes[d].complete {
        cx.error(span, "VIRT-15", format!("the covariant return type names the incomplete class '{}'", cx.u.classes[d].name));
        return false;
    }
    let paths = cx.u.base_paths(d, b);
    if paths.len() != 1 {
        let why = if paths.is_empty() { "is not derived from" } else { "has more than one base" };
        cx.error(span, "VIRT-15", format!("the return type is not covariant: '{}' {} '{}'", cx.u.classes[d].name, why, cx.u.classes[b].name));
        return false;
    }
    if !crate::classes::derived::check::base_accessible_in(cx, d, &paths[0], c) {
        cx.error(span, "VIRT-15", format!("the return type is not covariant: '{}' is not an accessible base of '{}'", cx.u.classes[b].name, cx.u.classes[d].name));
        return false;
    }
    if (nt.c && !ot.c) || (nt.v && !ot.v) {
        cx.error(span, "VIRT-15", "the covariant return type adds a cv-qualifier");
        return false;
    }
    true
}

fn mismatch(cx: &mut Check, span: Span) -> bool {
    cx.error(span, "VIRT-15", "the return type of an override differs from the return type of the function it overrides");
    false
}

/// The pure virtual functions of class `c` whose final overrider in
/// `c` is pure: its own, and those of its bases that it does not
/// override. The destructor of `c`, declared or implicit, overrides a
/// pure destructor of a base (VIRT-7).
fn pure_left(cx: &Check, c: ClassId) -> Vec<EntityId> {
    let own: Vec<EntityId> = cx.u.scopes[cx.u.classes[c].scope].names.values().flatten().copied().filter(|e| cx.u.virt.contains_key(e)).collect();
    let mut out: Vec<EntityId> = own.iter().copied().filter(|e| cx.u.virt[e].pure).collect();
    for (_, b) in cx.u.bases(c) {
        for p in pure_left(cx, b) {
            let dtor = cx.u.func(p).is_some_and(|f| f.special == Special::Dtor);
            if !dtor && !own.iter().any(|e| cx.u.virt[e].overrides.contains(&p)) && !out.contains(&p) {
                out.push(p);
            }
        }
    }
    out
}

/// The class `c` is complete (its members are declared): VIRT-3 for a
/// final destructor of a base, which the destructor of `c` overrides
/// (VIRT-7), the abstract class (VIRT-8), and the warnings of VIRT-13.
pub fn complete(cx: &mut Check, c: ClassId, span: Span) {
    let dtor = crate::classes::constructors::check::DTOR;
    for (_, b) in cx.u.bases(c) {
        let fin = crate::foundations::names::check::declared_here(cx, cx.u.classes[b].scope, dtor).into_iter().any(|d| cx.u.virt.get(&d).is_some_and(|v| v.is_final));
        if fin {
            cx.error(span, "VIRT-3", format!("the destructor of '{}' overrides the final destructor of '{}'", cx.u.classes[c].name, cx.u.classes[b].name));
            return;
        }
    }
    if !pure_left(cx, c).is_empty() {
        cx.u.abstract_classes.insert(c);
    }
    warnings(cx, c);
}

/// VIRT-13: a member function that hides the virtual functions of a
/// base of its name and overrides none of them (under `-Wall`), and an
/// override with neither `override` nor `final` in a class where another
/// override has `override`.
fn warnings(cx: &mut Check, c: ClassId) {
    let dtor = crate::classes::constructors::check::DTOR;
    let mut names: Vec<(std::rc::Rc<str>, Vec<EntityId>)> = cx.u.scopes[cx.u.classes[c].scope].names.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    names.sort_by_key(|(_, v)| v.first().copied());
    let overrides = |cx: &Check, e: EntityId| cx.u.virt.get(&e).is_some_and(|v| !v.overrides.is_empty());
    if cx.wflag(&["-Wall", "-Woverloaded-virtual"]) {
        for (name, ents) in &names {
            if &**name == dtor || !ents.iter().any(|&e| cx.u.func(e).is_some()) {
                continue;
            }
            let hidden = cx.u.bases(c).into_iter().any(|(_, b)| matches!(crate::classes::derived::check::member_lookup(cx, b, name), Ok(Some(f)) if f.ents.iter().any(|e| cx.u.virt.contains_key(e))));
            if hidden && !ents.iter().any(|&e| overrides(cx, e)) {
                let at = cx.u.entities[ents[0]].span;
                cx.warning(at, "VIRT-13", format!("'{}' hides a virtual function of a base", name));
            }
        }
    }
    let marked = names.iter().flat_map(|(_, v)| v.iter()).any(|e| cx.u.virt.get(e).is_some_and(|v| v.marked));
    if marked {
        for (name, ents) in &names {
            if &**name == dtor {
                continue;
            }
            for &e in ents {
                if overrides(cx, e) && cx.u.virt.get(&e).is_some_and(|v| !v.marked && !v.is_final) {
                    let at = cx.u.entities[e].span;
                    cx.warning(at, "VIRT-13", format!("'{}' overrides a virtual function but is not marked override", name));
                }
            }
        }
    }
}

/// VIRT-19, after the bodies of the class's members: an override and the
/// function it overrides are both deleted or both not.
pub fn after_bodies(cx: &mut Check, c: ClassId) {
    let own: Vec<EntityId> = cx.u.scopes[cx.u.classes[c].scope].names.values().flatten().copied().filter(|e| cx.u.virt.contains_key(e)).collect();
    for f in own {
        let deleted = cx.u.func(f).is_some_and(|x| x.deleted);
        if cx.u.virt[&f].overrides.iter().any(|&o| cx.u.func(o).is_some_and(|x| x.deleted) != deleted) {
            let span = cx.u.entities[f].span;
            cx.error(span, "VIRT-19", format!("'{}' and the function it overrides differ in being deleted", cx.u.entities[f].name));
            return;
        }
    }
}

/// VIRT-8: no object of an abstract class. True when `ty` is no
/// abstract class or array of one.
pub fn not_abstract(cx: &mut Check, ty: &QType, span: Span) -> bool {
    match ty.base_elem().class() {
        Some(c) if !ty.is_ref() && cx.u.abstract_classes.contains(&c) => {
            cx.error(span, "VIRT-8", format!("an object of the abstract class '{}'", cx.u.classes[c].name));
            false
        }
        _ => true,
    }
}
