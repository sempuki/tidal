//! The check part of `classes/virtual`, for build step 4 part a: virtual
//! functions in classes with non-virtual bases. A member function is
//! virtual when it is declared `virtual` or overrides a virtual function
//! of a base (VIRT-1). `Unit::virt` records each virtual function with
//! the functions it overrides, and the machine's dispatch finds the
//! final overrider of a call. `dynamic_cast` converts with a check at
//! run time (VIRT-10, VIRT-18). Virtual bases are unsupported.

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
            let same = func.special == Special::Dtor || (gf.ty.params == func.ty.params && gf.ty.variadic == func.ty.variadic && gf.obj_cv == func.obj_cv && gf.ref_q == func.ref_q);
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

/// VIRT-8: the class has a pure virtual function that it does not
/// override.
pub fn is_abstract(cx: &Check, c: ClassId) -> bool {
    !pure_left(cx, c).is_empty()
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
    // DER-12: no constexpr constructor or destructor in a class with a
    // virtual base.
    if !cx.u.classes[c].vbases.is_empty() {
        let mut fs = crate::classes::constructors::check::ctors(cx, c);
        fs.extend(crate::foundations::names::check::declared_here(cx, cx.u.classes[c].scope, dtor));
        for f in fs {
            if cx.u.func(f).is_some_and(|x| x.constexpr) {
                let what = if cx.u.func(f).is_some_and(|x| x.special == Special::Dtor) { "destructor" } else { "constructor" };
                cx.error(cx.u.entities[f].span, "DER-12", format!("a constexpr {} in the class '{}', which has a virtual base", what, cx.u.classes[c].name));
            }
        }
    }
    unique_overriders(cx, c, span);
    warnings(cx, c);
}

/// `e` is `v` or overrides it, directly or not.
fn overrides(cx: &Check, e: EntityId, v: EntityId) -> bool {
    e == v || cx.u.virt.get(&e).is_some_and(|x| x.overrides.iter().any(|&o| overrides(cx, o, v)))
}

/// VIRT-5: each virtual function of each base subobject has one final
/// overrider in `c`. Two paths that reach one subobject through a
/// virtual base can each override the function; then a class must
/// override it again, unless one overrider's class derives from the
/// other's.
fn unique_overriders(cx: &mut Check, c: ClassId, span: Span) {
    if cx.u.classes[c].vbases.is_empty() {
        return;
    }
    let declares = |cx: &Check, k: ClassId, v: EntityId| cx.u.scopes[cx.u.classes[k].scope].names.values().flatten().copied().find(|&e| cx.u.virt.contains_key(&e) && overrides(cx, e, v));
    for a in cx.u.classes[c].vbases.clone() {
        let virtuals: Vec<EntityId> = cx.u.scopes[cx.u.classes[a].scope].names.values().flatten().copied().filter(|e| cx.u.virt.contains_key(e)).collect();
        let paths = cx.u.base_paths(c, a);
        let Some(path) = paths.first() else { continue };
        if !crate::classes::derived::check::through_virtual(path) {
            continue;
        }
        for v in virtuals {
            // The overrider nearest to `c` along each path to the shared
            // subobject.
            let mut found: Vec<(ClassId, EntityId)> = Vec::new();
            for p in all_paths(cx, c, a) {
                if cx.u.path_key(c, &p) != cx.u.path_key(c, path) {
                    continue;
                }
                let mut k = c;
                let mut classes = vec![c];
                for &i in &p {
                    k = cx.u.step_class(k, i).unwrap();
                    classes.push(k);
                }
                if let Some(x) = classes.iter().find_map(|&k| declares(cx, k, v).map(|e| (k, e))) {
                    if !found.contains(&x) {
                        found.push(x);
                    }
                }
            }
            let finals: Vec<&(ClassId, EntityId)> = found.iter().filter(|x| !found.iter().any(|y| y.0 != x.0 && crate::classes::derived::check::is_base(cx, y.0, x.0))).collect();
            if finals.len() > 1 {
                cx.error(span, "VIRT-5", format!("'{}' has no unique final overrider in '{}'", cx.u.entities[v].name, cx.u.classes[c].name));
                return;
            }
        }
    }
}

/// Every path from `from` to a base subobject of class `to`, also those
/// that reach one subobject twice.
fn all_paths(cx: &Check, from: ClassId, to: ClassId) -> Vec<Vec<usize>> {
    if from == to {
        return vec![Vec::new()];
    }
    let mut out = Vec::new();
    for (i, b) in cx.u.bases(from) {
        for mut p in all_paths(cx, b, to) {
            p.insert(0, i);
            out.push(p);
        }
    }
    out
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

/// Whether the destructor of class `c` is virtual: declared virtual, or
/// overriding the virtual destructor of a base (VIRT-7).
pub fn virtual_dtor(cx: &Check, c: ClassId) -> bool {
    let own = crate::foundations::names::check::declared_here(cx, cx.u.classes[c].scope, crate::classes::constructors::check::DTOR);
    own.iter().any(|d| cx.u.virt.contains_key(d)) || cx.u.bases(c).into_iter().any(|(_, b)| virtual_dtor(cx, b))
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

/// `dynamic_cast` (VIRT-10, VIRT-18). A cast to the same class or to an
/// accessible, unambiguous base converts as an implicit conversion does;
/// any other cast checks at run time and needs a polymorphic operand.
pub fn dynamic_cast(cx: &mut Check, n: &crate::syntax::Node) -> crate::sem::tree::Expr {
    use crate::expressions::check::{bad, expr, nodes};
    use crate::sem::tree::{Cat, Conv, Expr, EK};
    let span = n.span;
    let Some(tn) = n.child("type_id") else { return bad(span) };
    let to = crate::declarations::declarators::check::type_id(cx, tn);
    let Some(en) = nodes(n).into_iter().find(|c| c.prod != "type_id") else { return bad(span) };
    let e = expr(cx, en);
    if to.is_error() || e.ty.is_error() {
        return bad(span);
    }
    let fail = |cx: &mut Check, m: String| {
        cx.error(span, "VIRT-18", m);
        bad(span)
    };
    let complete = |cx: &Check, c: ClassId| cx.u.classes[c].complete;
    // The target's class, or None for `void`, and the operand's object.
    let (reference, rv, target) = match &to.t {
        Type::Ref(t, rv) => (true, *rv, (**t).clone()),
        Type::Ptr(t) => (false, false, (**t).clone()),
        _ => return fail(cx, format!("dynamic_cast to {}, which is not a pointer or a reference", to.show())),
    };
    let tc = target.class();
    if tc.is_none() && !(target.is_void() && !reference) {
        return fail(cx, format!("dynamic_cast to {}, which is not a class or void", to.show()));
    }
    if tc.is_some_and(|c| !complete(cx, c)) {
        return fail(cx, "dynamic_cast to an incomplete class".into());
    }
    let obj = if reference {
        let e = crate::foundations::values::check::deref_ref(e);
        if e.cat == Cat::PR {
            return fail(cx, "dynamic_cast of a prvalue to a reference".into());
        }
        e
    } else {
        let e = crate::foundations::values::check::rvalue(cx, e);
        if !e.ty.is_ptr() {
            return fail(cx, format!("dynamic_cast to a pointer of {}, which is not a pointer", e.ty.show()));
        }
        e
    };
    let src = if reference { obj.ty.clone() } else { obj.ty.pointee().cloned().unwrap_or_else(QType::error) };
    let Some(sc) = src.class() else {
        return fail(cx, format!("dynamic_cast of {}, which is not a class", src.show()));
    };
    if !complete(cx, sc) {
        return fail(cx, "dynamic_cast of an incomplete class".into());
    }
    if (src.c && !target.c) || (src.v && !target.v) {
        return fail(cx, "dynamic_cast removes a cv-qualifier".into());
    }
    let out_ty = if reference { target.clone() } else { to.unqual() };
    let cat = if reference { if rv { Cat::X } else { Cat::L } } else { Cat::PR };
    // To the same class, or up to a base.
    if let Some(t) = tc {
        if t == sc {
            return if reference { Expr::new(EK::Conv(Conv::NoOp, Box::new(obj)), out_ty, cat, span) } else { obj.conv(Conv::PtrNoOp, out_ty, Cat::PR) };
        }
        if crate::classes::derived::check::is_base(cx, sc, t) {
            if crate::classes::derived::check::paths(cx, sc, t).len() != 1 || !crate::classes::derived::check::base_is_accessible(cx, sc, t) {
                return fail(cx, format!("dynamic_cast to the base '{}', which is ambiguous or not accessible", cx.u.classes[t].name));
            }
            if reference {
                let Some(x) = crate::classes::derived::check::to_base(cx, obj, t, span) else { return bad(span) };
                return Expr::new(EK::Conv(Conv::NoOp, Box::new(x)), out_ty, cat, span);
            }
            return crate::classes::derived::check::ptr_to_base(cx, obj, out_ty, span).unwrap_or_else(|| bad(span));
        }
    }
    if !cx.u.polymorphic(sc) {
        return fail(cx, format!("dynamic_cast of '{}', which is not a polymorphic class", cx.u.classes[sc].name));
    }
    Expr::new(EK::DynCast(Box::new(obj), tc, reference), out_ty, cat, span)
}
