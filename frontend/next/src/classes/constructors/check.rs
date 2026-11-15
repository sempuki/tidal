//! The check part of `classes/constructors`, for build step 4 part a:
//! constructors declared and defined in their class or out of it, member
//! initializer lists and delegation, `= default` and `= delete`,
//! `explicit`, and the initialization of a class object by a constructor.
//! A destructor, and a user-declared copy or move constructor, are
//! unsupported until a later piece, so a unit that has one gives no
//! verdict.

use crate::declarations::check::Specs;
use crate::expressions::check::{expr, first_node};
use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, Init};
use crate::sem::types::{ClassId, QType, Type};
use crate::sem::{EntityId, MemInits, Special};
use crate::source::Span;
use crate::syntax::Node;
use std::rc::Rc;

/// The name under which a class holds its constructors, which no
/// identifier spells.
pub const CTOR: &str = "{ctor}";

/// The name under which a class holds its destructor.
pub const DTOR: &str = "{dtor}";

/// CTOR-4, CTOR-24: default-initialization gives each member of an
/// object of class `c` a value: the class has a user-provided default
/// constructor, or each base and data member has a default member
/// initializer or such a class, and a union one member with a default
/// member initializer. Err names the rule that fails.
pub fn const_default(cx: &Check, c: ClassId) -> Result<(), &'static str> {
    let provided = ctors(cx, c).iter().any(|&f| cx.u.func(f).is_some_and(|x| x.takes_no_argument()) && !cx.u.defaulted.contains(&f));
    if provided {
        return Ok(());
    }
    let fields: Vec<crate::sem::Field> = cx.u.classes[c].fields.iter().filter_map(|&f| cx.u.field(f).cloned()).collect();
    if crate::classes::unions::check::is_union(cx, c) {
        return if fields.is_empty() || fields.iter().any(|x| x.init.is_some()) { Ok(()) } else { Err("CTOR-24") };
    }
    for (k, x) in fields.iter().enumerate() {
        let unnamed = cx.u.entities[cx.u.classes[c].fields[k]].name.is_empty();
        let anonymous = unnamed && x.width.is_none();
        // An unnamed bit-field is no member (BF-12).
        if x.init.is_some() || (unnamed && x.width.is_some()) {
            continue;
        }
        match x.ty.base_elem().class() {
            Some(d) => {
                let r = const_default(cx, d);
                if r.is_err() {
                    return if anonymous { Err("CTOR-24") } else { Err("CTOR-4") };
                }
            }
            None => return Err("CTOR-4"),
        }
    }
    Ok(())
}

/// CE-23: the destructor of class `c` is constexpr: a declared one is
/// declared constexpr, and an implicit one is when the destructors of
/// its bases and members are.
pub fn dtor_constexpr(cx: &Check, c: ClassId) -> bool {
    match names::declared_here(cx, cx.u.classes[c].scope, DTOR).first().copied() {
        Some(d) if !cx.u.defaulted.contains(&d) => cx.u.func(d).is_some_and(|f| f.constexpr),
        _ => cx.u.classes[c].fields.iter().filter_map(|&f| cx.u.field(f)).all(|x| x.ty.base_elem().class().is_none_or(|k| !crate::classes::constructors::run::needs_in(&cx.u, &QType::new(Type::Class(k))) || dtor_constexpr(cx, k))),
    }
}

/// The constructors of a class.
pub fn ctors(cx: &Check, c: ClassId) -> Vec<EntityId> {
    names::declared_here(cx, cx.u.classes[c].scope, CTOR)
}

/// A class with a user-declared constructor, which is no aggregate and
/// has no implicit default constructor (CTOR-2).
pub fn has_ctors(cx: &Check, c: ClassId) -> bool {
    !ctors(cx, c).is_empty() || cx.u.inherits.contains_key(&c)
}

/// DER-8, OVL-17: the constructors that class `c` inherits for a call
/// with `n` arguments: those of the bases it names, other than copy and
/// move constructors, one that the class declares with the same
/// parameters, and, for one argument, one whose first parameter is a
/// reference to a class that `c` derives from.
fn inherited(cx: &Check, c: ClassId, n: usize) -> Vec<EntityId> {
    if n == 0 {
        return Vec::new();
    }
    let own: Vec<Vec<QType>> = ctors(cx, c).iter().filter_map(|&f| cx.u.func(f).map(|x| x.ty.params.clone())).collect();
    let mut out = Vec::new();
    for &b in cx.u.inherits.get(&c).map(|v| v.as_slice()).unwrap_or(&[]) {
        for f in ctors(cx, b) {
            let Some(x) = cx.u.func(f) else { continue };
            if crate::classes::copy_move::check::copy_or_move(cx, b, f).is_some() || own.contains(&x.ty.params) {
                continue;
            }
            let first = x.ty.params.first();
            if n == 1 && first.is_some_and(|p| p.is_ref() && p.nonref().class().is_some_and(|k| crate::classes::derived::check::derives(cx, c, k))) {
                continue;
            }
            out.push(f);
        }
    }
    out
}

/// The name of a declarator's declarator-id: the class's name for a
/// constructor, and true for a destructor.
pub fn special_name(cx: &Check, c: ClassId, dn: &Node) -> Option<bool> {
    let id = dn.find("declarator_id")?;
    let class = cx.u.classes[c].name.clone();
    if cx.has_word(id, "~") {
        return Some(true);
    }
    let u = id.find("unqualified_id")?;
    (cx.source(u) == *class).then_some(false)
}

/// Whether a constructor's parameters make it a copy or move
/// constructor: a first parameter that is a reference to the class.
fn copy_or_move(cx: &Check, c: ClassId, f: EntityId) -> bool {
    let Some(func) = cx.u.func(f) else { return false };
    func.ty.params.first().is_some_and(|p| p.is_ref() && p.nonref().class() == Some(c)) && func.ty.params.len() == 1
}

/// A constructor or destructor member (CTOR-1). `body` is the
/// function_definition of a definition, which waits for the complete
/// class in `bodies`. False after an unsupported member.
pub fn member<'n>(cx: &mut Check, c: ClassId, specs: Option<&Specs>, dn: &'n Node, def: Option<&'n Node>, access: crate::sem::Access, bodies: &mut Vec<(EntityId, &'n Node)>) -> bool {
    if special_name(cx, c, dn) == Some(true) {
        return dtor_member(cx, c, specs, dn, def, access, bodies);
    }
    let mut specs = specs.cloned().unwrap_or_default();
    if specs.is_virtual {
        cx.error(dn.span, "CTOR-1", "a constructor declared virtual");
        return true;
    }
    // CTOR-1: a constructor is not static and has no return type.
    if !specs.unsupported && (specs.is_static || specs.typedef || specs.ty.is_some()) {
        cx.error(dn.span, "CTOR-1", "a constructor declared static, typedef, or with a return type");
        return true;
    }
    if specs.unsupported {
        cx.unsupported(dn.span, "a constructor declaration of this form (build step 4)");
        return false;
    }
    specs.ty = Some(QType::void());
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    if d.params.first().is_some_and(|p| p.this_param) {
        cx.error(dn.span, "CTOR-1", "a constructor with an explicit object parameter");
        return true;
    }
    // CTOR-1, VOL-7: a constructor has no cv-qualifier and no
    // ref-qualifier.
    if !d.unsupported && d.ty.is_func() && (d.obj_cv != (false, false) || d.ref_q) {
        let cause = d.obj_cv.1.then_some("VOL-7");
        cx.error_cause(dn.span, "CTOR-1", cause, "a constructor with a cv-qualifier or a ref-qualifier");
        return true;
    }
    if d.unsupported || !d.ty.is_func() {
        cx.unsupported(dn.span, "a constructor declaration of this form (build step 4)");
        return false;
    }
    let local = cx.u.is_local(cx.u.scopes[cx.u.classes[c].scope].parent.unwrap_or(0));
    // An attribute before a constructor's definition applies to it.
    if let Some(def) = def {
        for a in def.children("attribute_spec") {
            crate::declarations::attributes::check::attribute_spec(cx, a, &mut specs.attrs);
        }
    }
    if local || specs.attrs.deprecated.is_some() || specs.attrs.noreturn || dn.find("attribute_spec").is_some() {
        cx.unsupported(dn.span, "a constructor of a local class or a constructor with attributes other than nodiscard (build step 4)");
        return false;
    }
    // CPY-15: a constructor whose first parameter is the class itself.
    if d.ty.func().and_then(|f| f.params.first().cloned()).is_some_and(|p| p.unqual().t == Type::Class(c)) {
        cx.error(dn.span, "CPY-15", "a constructor whose first parameter is its class by value");
        return true;
    }
    // CTOR-1, CLS-9: a constructor declared twice.
    let fty = d.ty.func().cloned();
    if ctors(cx, c).into_iter().any(|e| cx.u.func(e).is_some_and(|f| Some(&*f.ty) == fty.as_ref()) && !cx.deferred.contains(&e)) {
        cx.error_cause(dn.span, "CTOR-1", Some("CLS-9"), "a constructor declared twice in its class");
        return true;
    }
    let mut d2 = d.clone();
    d2.name = d.name.clone().map(|(_, s)| (Rc::from(CTOR), s));
    let Some(f) = crate::declarations::functions::definitions::check::declare_function(cx, &specs, &d2, None) else { return true };
    cx.u.access.insert(f, access);
    if let Some(x) = cx.u.func_mut(f) {
        x.special = Special::Ctor;
        x.explicit = specs.explicit;
        x.nodiscard |= specs.attrs.nodiscard;
    }
    let _ = copy_or_move;
    if let Some(def) = def {
        let body = def.child("function_body");
        let words = body.map(|b| cx.words(b)).unwrap_or_default();
        if words.contains(&"delete") {
            if let Some(x) = cx.u.func_mut(f) {
                x.deleted = true;
                x.defined = true;
            }
            return true;
        }
        if words.contains(&"default") {
            // CPY-14: a defaulted copy or move constructor is the
            // implicit one.
            if crate::classes::copy_move::check::copy_or_move(cx, c, f).is_some() {
                cx.u.defaulted.insert(f);
                if let Some(x) = cx.u.func_mut(f) {
                    x.inline = true;
                    x.defined = true;
                }
                crate::classes::copy_move::check::defaulted_in_class(cx, c, f);
                return true;
            }
            // CTOR-2: `= default` gives the implicit default constructor.
            if !d.params.is_empty() {
                cx.unsupported(dn.span, "a defaulted constructor that is not the default constructor (build step 4)");
                return false;
            }
            cx.u.defaulted.insert(f);
            if let Some(x) = cx.u.func_mut(f) {
                x.inline = true;
            }
            cx.deferred.insert(f);
            bodies.push((f, def));
            return true;
        }
        if let Some(x) = cx.u.func_mut(f) {
            x.inline = true;
        }
        cx.deferred.insert(f);
        bodies.push((f, def));
    }
    true
}

/// A destructor member (DTOR-1): the class's name after `~`, no
/// parameters, no return type, not static, no cv-qualifiers.
fn dtor_member<'n>(cx: &mut Check, c: ClassId, specs: Option<&Specs>, dn: &'n Node, def: Option<&'n Node>, access: crate::sem::Access, bodies: &mut Vec<(EntityId, &'n Node)>) -> bool {
    let mut specs = specs.cloned().unwrap_or_default();
    if specs.unsupported || cx.u.is_local(cx.u.scopes[cx.u.classes[c].scope].parent.unwrap_or(0)) || specs.consteval || def.is_some_and(|x| x.child("attribute_spec").is_some()) {
        cx.unsupported(dn.span, "a destructor of this form (build step 4)");
        return false;
    }
    let named = dn.find("class_name").map(|n| cx.source(n)).unwrap_or_default();
    if named != *cx.u.classes[c].name {
        cx.error(dn.span, "DTOR-1", "a destructor without the name of its class after '~'");
        return true;
    }
    if specs.ty.is_some() || specs.is_static || specs.explicit {
        cx.error(dn.span, "DTOR-1", "a destructor with a return type, static, or explicit");
        return true;
    }
    specs.ty = Some(QType::void());
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    if d.unsupported || !d.ty.is_func() {
        cx.unsupported(dn.span, "a destructor of this form (build step 4)");
        return false;
    }
    if !d.params.is_empty() || d.ty.func().is_some_and(|f| f.variadic) || d.obj_cv != (false, false) || d.ref_q {
        cx.error(dn.span, "DTOR-1", "a destructor with parameters or qualifiers");
        return true;
    }
    if !names::declared_here(cx, cx.scope, DTOR).is_empty() && !names::declared_here(cx, cx.scope, DTOR).iter().any(|e| cx.deferred.contains(e)) {
        cx.error_cause(dn.span, "DTOR-1", Some("CLS-9"), "a destructor declared twice");
        return true;
    }
    let mut d2 = d.clone();
    d2.name = d.name.clone().map(|(_, s)| (Rc::from(DTOR), s));
    let Some(f) = crate::declarations::functions::definitions::check::declare_function(cx, &specs, &d2, None) else { return true };
    cx.u.access.insert(f, access);
    if let Some(x) = cx.u.func_mut(f) {
        x.special = Special::Dtor;
    }
    if let Some(def) = def {
        let words = def.child("function_body").map(|b| cx.words(b)).unwrap_or_default();
        if let Some(x) = cx.u.func_mut(f) {
            x.inline = true;
        }
        if words.contains(&"delete") {
            if let Some(x) = cx.u.func_mut(f) {
                x.deleted = true;
                x.defined = true;
            }
            return true;
        }
        if words.contains(&"default") {
            cx.u.defaulted.insert(f);
        }
        cx.deferred.insert(f);
        bodies.push((f, def));
    }
    true
}

/// DTOR-4: the destructor of class `c` is non-throwing: as its exception
/// specification says, or for an implicit one, when those of its members
/// and bases are. A union's destructor runs none of its members'.
pub fn dtor_nothrow(cx: &Check, c: ClassId) -> bool {
    match names::declared_here(cx, cx.u.classes[c].scope, DTOR).first() {
        Some(&d) => cx.u.func(d).is_none_or(|f| f.ty.noexcept),
        None => parts_nothrow(cx, c),
    }
}

/// The destructors of the members and bases of class `c` are all
/// non-throwing.
pub fn parts_nothrow(cx: &Check, c: ClassId) -> bool {
    crate::classes::unions::check::is_union(cx, c) || cx.u.object_types(c).iter().all(|t| t.base_elem().class().is_none_or(|d| dtor_nothrow(cx, d)))
}

/// The destructor of class `c`, or of a member's class, is deleted or not
/// accessible: from the point of use when `top` is set, and those of its
/// members and bases from the class (DER-14: a base's protected one).
pub fn dtor_blocked(cx: &Check, c: ClassId, top: bool, base: bool) -> bool {
    blocked_in(cx, c, top, base, None, true)
}

/// `dtor_blocked` for the destructor of class `c` that the destructor
/// of class `from` runs, which a friend class can run; a complete object
/// also runs those of its virtual bases (DER-21).
fn blocked_in(cx: &Check, c: ClassId, top: bool, base: bool, from: Option<ClassId>, complete: bool) -> bool {
    let own = names::declared_here(cx, cx.u.classes[c].scope, DTOR).first().is_some_and(|&d| {
        let access = cx.u.access.get(&d).copied().unwrap_or(crate::sem::Access::Public);
        let friend = from.is_some_and(|k| cx.u.friend_classes.get(&c).is_some_and(|fs| fs.contains(&k)));
        let ok = if top { crate::classes::members::check::accessible(cx, d) } else { access == crate::sem::Access::Public || (base && access == crate::sem::Access::Protected) || friend };
        cx.u.func(d).is_some_and(|f| f.deleted) || !ok
    });
    // UNI-3: the destructor of a union runs none of its members'. A
    // variant member deletes an implicit one.
    if crate::classes::unions::check::is_union(cx, c) {
        let declared = !names::declared_here(cx, cx.u.classes[c].scope, DTOR).is_empty();
        return own || (!declared && crate::classes::unions::check::deleting_member(cx, c, crate::classes::unions::check::Special::Dtor).is_some());
    }
    let vbases = if complete { cx.u.classes[c].vbases.clone() } else { Vec::new() };
    // The members of an anonymous union are variant members of the
    // class, whose own destructor does not end them (UNI-3).
    let provided = names::declared_here(cx, cx.u.classes[c].scope, DTOR).first().is_some_and(|d| !cx.u.defaulted.contains(d));
    let anonymous = |cx: &Check, f: EntityId| cx.u.entities[f].name.is_empty() && cx.u.field(f).and_then(|x| x.ty.class()).is_some_and(|d| crate::classes::unions::check::is_union(cx, d));
    own || cx.u.classes[c].fields.clone().iter().filter(|&&f| !(provided && anonymous(cx, f))).any(|&f| cx.u.field(f).is_some_and(|x| x.ty.base_elem().class().is_some_and(|d| blocked_in(cx, d, false, x.base, Some(c), !x.base))))
        || vbases.into_iter().any(|v| blocked_in(cx, v, false, true, Some(c), false))
}

/// DTOR-3: the destructor of a class, or of a member's class, is
/// deleted or not accessible, so no object of the class can end. True
/// when it can.
pub fn dtor_usable(cx: &mut Check, ty: &QType, span: Span) -> bool {
    let Some(c) = ty.base_elem().class() else { return true };
    let declared = !names::declared_here(cx, cx.u.classes[c].scope, DTOR).is_empty();
    if !declared && !crate::classes::unions::check::usable(cx, c, crate::classes::unions::check::Special::Dtor, span) {
        return false;
    }
    if dtor_blocked(cx, c, true, false) {
        let cause = if cx.unevaluated > 0 { "UNEV-5" } else { "VC-11" };
        cx.error_cause(span, "DTOR-3", Some(cause), format!("the destructor of '{}' is deleted or not accessible", cx.u.classes[c].name));
        return false;
    }
    true
}

/// The checks at the end of a class with constructors: no delegation
/// cycle (CTOR-14), and no member with the class's name (CLS-9).
pub fn end_of_class(cx: &mut Check, c: ClassId, span: Span) {
    let all = ctors(cx, c);
    if all.is_empty() {
        return;
    }
    for &start in &all {
        let mut cur = start;
        let mut seen = vec![start];
        while let Some(MemInits::Delegate(Init::Ctor(next, _))) = cx.u.mem_inits.get(&cur) {
            if seen.contains(next) {
                cx.error(cx.u.entities[start].span, "CTOR-14", "the constructors delegate in a cycle");
                return;
            }
            seen.push(*next);
            cur = *next;
        }
    }
    let name = cx.u.classes[c].name.clone();
    if !names::declared_here(cx, cx.u.classes[c].scope, &name).is_empty() {
        cx.error(span, "CLS-9", format!("a member has the name '{}' of its class, which has a constructor", name));
    }
}

/// The body of a constructor that its class declared: the member
/// initializers and the compound statement, or the implicit body of a
/// defaulted default constructor.
pub fn define(cx: &mut Check, f: EntityId, def: &Node) {
    let Some(dn) = def.child("declarator") else { return };
    let mut specs = Specs { ty: Some(QType::void()), ..Default::default() };
    if let Some(ds) = def.child("decl_specifiers") {
        let s = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
        specs.constexpr = s.constexpr;
        specs.explicit = s.explicit;
    }
    let marks = cx.u.diags.len();
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    cx.u.diags.truncate(marks);
    cx.deferred.remove(&f);
    let Some(body) = def.child("function_body") else { return };
    if cx.words(body).contains(&"default") && cx.u.func(f).is_some_and(|x| x.special == Special::Dtor) {
        if let Some(x) = cx.u.func_mut(f) {
            x.body = Some(Rc::new(crate::sem::tree::Stmt::new(crate::sem::tree::SK::Block(Vec::new()), def.span)));
            x.defined = true;
        }
        return;
    }
    if cx.words(body).contains(&"default") {
        let c = cx.u.func(f).and_then(|x| x.class).unwrap();
        // CPY-14: a copy or move constructor defaulted outside its class
        // is the implicit one.
        if crate::classes::copy_move::check::copy_or_move(cx, c, f).is_some() {
            cx.u.defaulted.insert(f);
            if let Some(x) = cx.u.func_mut(f) {
                x.defined = true;
            }
            crate::classes::copy_move::check::defaulted_out_of_class(cx, c, f, def.span);
            return;
        }
        let fields = cx.u.classes[c].fields.clone();
        let union = crate::classes::unions::check::is_union(cx, c);
        let mut inits = Vec::new();
        for fid in fields {
            let x = cx.u.field(fid).unwrap().clone();
            if crate::classes::members::check::temporary_default(cx, fid, def.span) {
                return;
            }
            inits.push(match x.init {
                Some(i) => i,
                None if union => Init::Default,
                None => default_init(cx, &x.ty, def.span).unwrap_or(Init::Default),
            });
        }
        inits.extend(vbase_inits(cx, c, Vec::new(), def.span).unwrap_or_default());
        cx.u.mem_inits.insert(f, MemInits::Fields(inits));
        if let Some(x) = cx.u.func_mut(f) {
            x.body = Some(Rc::new(crate::sem::tree::Stmt::new(crate::sem::tree::SK::Block(Vec::new()), def.span)));
            x.defined = true;
        }
        return;
    }
    // TRY-10: a function try block on a constructor or destructor.
    if let Some(ftb) = body.child("function_try_block") {
        let Some(cs) = ftb.child("compound_stmt") else { return };
        let hs: Vec<&Node> = ftb.children("handler").collect();
        crate::declarations::functions::definitions::check::define_full(cx, f, &d, cs, ftb.child("ctor_init"), &hs);
        return;
    }
    let Some(cs) = body.child("compound_stmt") else { return };
    crate::declarations::functions::definitions::check::define_with(cx, f, &d, cs, body.child("ctor_init"));
}

/// An out-of-class definition `C::C(...) ...` of a constructor that the
/// class declared. True when `n` is one, handled.
pub fn out_of_class(cx: &mut Check, n: &Node) -> bool {
    let Some(dn) = n.child("declarator") else { return false };
    let Some(q) = dn.find("qualified_name") else { return false };
    let (Some(nn), Some(u)) = (q.child("nested_name"), q.child("unqualified_id")) else { return false };
    let mut toks = Vec::new();
    nn.tokens(&mut toks);
    let Some(cname) = toks.iter().rev().map(|&t| cx.toks[t].text.clone()).find(|w| w != "::") else { return false };
    let classes = names::lookup_where(cx, &cname, |k| matches!(k, crate::sem::EntKind::Class(_)));
    let Some(c) = classes.first().and_then(|&e| match cx.u.entities[e].kind {
        crate::sem::EntKind::Class(c) => Some(c),
        _ => None,
    }) else {
        return false;
    };
    if cx.source(u) != *cx.u.classes[c].name && !cx.has_word(u, "~") {
        return false;
    }
    if cx.has_word(u, "~") || cx.has_word(q, "~") {
        let found = names::declared_here(cx, cx.u.classes[c].scope, DTOR).first().copied();
        match found {
            Some(f) if !cx.u.func(f).unwrap().defined => {
                let saved = cx.scope;
                let outer = cx.member_of.replace(c);
                cx.scope = cx.u.classes[c].scope;
                define(cx, f, n);
                cx.scope = saved;
                cx.member_of = outer;
            }
            Some(_) => {
                cx.error(dn.span, "FN-14", format!("the destructor of '{}' is defined twice", cname));
            }
            None => {
                cx.error(dn.span, "NS-14", format!("the class '{}' declares no destructor", cname));
            }
        }
        return true;
    }
    let added = dn.find("parameter").is_some_and(|_| cx.source(dn).contains('='));
    let specs = Specs { ty: Some(QType::void()), ..Default::default() };
    let saved = cx.scope;
    let outer = cx.member_of.replace(c);
    cx.scope = cx.u.classes[c].scope;
    let marks = cx.u.diags.len();
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    cx.u.diags.truncate(marks);
    let ty = d.ty.func().cloned();
    let found = ctors(cx, c).into_iter().find(|&f| cx.u.func(f).is_some_and(|x| Some(&*x.ty) == ty.as_ref()));
    cx.scope = saved;
    // DEFARG-12: a default argument outside the class must not make the
    // constructor a default, copy, or move constructor.
    if added {
        if let (Some(f), Some(fty)) = (found, ty.as_ref()) {
            let before: Vec<bool> = (0..fty.params.len()).map(|k| cx.u.func(f).is_some_and(|x| x.defaults.get(k).is_some_and(|d| d.is_some()))).collect();
            let after: Vec<bool> = (0..fty.params.len()).map(|k| before[k] || d.params.get(k).is_some_and(|p| p.default.is_some())).collect();
            let special = |dflt: &[bool]| dflt.iter().all(|&x| x) || fty.params.first().is_some_and(|p| p.is_ref() && p.nonref().class() == Some(c)) && dflt[1..].iter().all(|&x| x);
            if special(&after) && !special(&before) {
                cx.error(dn.span, "DEFARG-12", format!("the default argument outside the class makes a constructor of '{}' a default, copy, or move constructor", cname));
                cx.member_of = outer;
                return true;
            }
        }
        cx.unsupported(dn.span, "a default argument on an out-of-class constructor definition (build step 4)");
        cx.member_of = outer;
        return true;
    }
    match found {
        Some(f) if !cx.u.func(f).unwrap().defined => {
            cx.scope = cx.u.classes[c].scope;
            define(cx, f, n);
            cx.scope = saved;
        }
        Some(_) => {
            cx.error(dn.span, "FN-14", format!("the constructor of '{}' is defined twice", cname));
        }
        None => {
            cx.error(dn.span, "NS-14", format!("the class '{}' declares no such constructor", cname));
        }
    }
    cx.member_of = outer;
    true
}

/// The arguments of a parenthesized list or a braced list.
fn arg_exprs(cx: &mut Check, n: Option<&Node>, braced: bool) -> Option<Vec<Expr>> {
    let Some(n) = n else { return Some(Vec::new()) };
    let items: Vec<&Node> = if braced { n.children("init_clause").collect() } else { n.children("argument").collect() };
    let mut out = Vec::new();
    for it in items {
        let c = first_node(it)?;
        if c.prod == "braced_init" || c.prod == "designator" {
            cx.unsupported(c.span, "a braced list as a constructor argument (build step 4)");
            return None;
        }
        let e = expr(cx, c);
        if e.ty.is_error() {
            return None;
        }
        out.push(e);
    }
    Some(out)
}

/// The member initializer list of a constructor (CTOR-5, CTOR-15,
/// CTOR-16, CTOR-8): an initializer for each data member, from its
/// `mem_init`, its default member initializer, or its
/// default-initialization; or a delegation to another constructor.
pub fn mem_inits(cx: &mut Check, f: EntityId, ctor_init: Option<&Node>) -> Option<MemInits> {
    let c = cx.u.func(f)?.class?;
    let fields = cx.u.classes[c].fields.clone();
    let vbases = cx.u.classes[c].vbases.clone();
    let mut given: Vec<Option<Init>> = vec![None; fields.len() + vbases.len()];
    let list: Vec<&Node> = ctor_init.map(|n| n.children("mem_init").collect()).unwrap_or_default();
    if let Some(ci) = ctor_init.filter(|n| cx.source(n).contains("__func__")) {
        cx.unsupported(ci.span, "__func__ in a member initializer (build step 4)");
        return None;
    }
    // CTOR-13: a member initializer list out of the members' order; an
    // implementation warns.
    if cx.wflag(&["-Wall", "-Wreorder"]) {
        let order: Vec<usize> = list.iter().filter_map(|mi| {
            let name = mi.child("mem_init_id").map(|x| cx.source(x)).unwrap_or_default();
            fields.iter().position(|&x| *cx.u.entities[x].name == *name)
        }).collect();
        if order.windows(2).any(|w| w[0] > w[1]) {
            cx.warning(ctor_init.unwrap().span, "CTOR-13", "the member initializers are not in the order of the members");
        }
    }
    for mi in &list {
        let name = mi.child("mem_init_id").map(|x| cx.source(x)).unwrap_or_default();
        let braced = mi.child("braced_init");
        let args = match braced {
            Some(b) => arg_exprs(cx, Some(b), true)?,
            None => arg_exprs(cx, mi.child("expression_list"), false)?,
        };
        let ty = QType::new(Type::Class(c));
        if name == *cx.u.classes[c].name {
            // CTOR-16: a delegation is the only mem-init.
            if list.len() > 1 {
                cx.error(mi.span, "CTOR-16", "a delegating constructor has another member initializer");
                return None;
            }
            let i = construct_init(cx, &ty, args, braced.is_some(), false, mi.span)?;
            if let Init::Ctor(g, _) = &i {
                if *g == f {
                    cx.error(mi.span, "CTOR-8", "a constructor delegates to itself");
                    return None;
                }
            }
            return Some(MemInits::Delegate(i));
        }
        // CTOR-5: a mem-init that names a direct base initializes its
        // subobject.
        // The name can be a type alias of the base (DER-1).
        let named_class = names::lookup_where(cx, &name, |k| matches!(k, crate::sem::EntKind::Class(_) | crate::sem::EntKind::Typedef(_))).first().and_then(|&e| match &cx.u.entities[e].kind {
            crate::sem::EntKind::Class(k) => Some(*k),
            crate::sem::EntKind::Typedef(t) => t.class(),
            _ => None,
        });
        let names_class = |b: ClassId| *cx.u.classes[b].name == *name || named_class == Some(b);
        // DER-7: a mem-init can also name a virtual base, direct or not.
        let base_k = fields.iter().position(|&x| cx.u.field(x).is_some_and(|y| y.base && y.ty.class().is_some_and(names_class))).or_else(|| vbases.iter().position(|&v| names_class(v)).map(|j| fields.len() + j));
        // UNI-7: a member of an anonymous union member.
        if base_k.is_none() && !fields.iter().any(|&x| *cx.u.entities[x].name == *name) {
            if let Some((k, j)) = crate::classes::unions::check::variant_member(&cx.u, c, &name) {
                if given[k].is_some() {
                    cx.error(mi.span, "UNI-8", "a constructor initializes two members of one anonymous union");
                    return None;
                }
                let anon = cx.u.field(fields[k]).and_then(|x| x.ty.class()).unwrap();
                let vty = cx.u.field(cx.u.classes[anon].fields[j]).unwrap().ty.clone();
                let i = direct_init(cx, &vty, args, braced.is_some(), mi.span)?;
                given[k] = Some(crate::classes::unions::check::variant_init(&cx.u, c, k, j, i));
                continue;
            }
        }
        let Some(k) = base_k.or_else(|| fields.iter().position(|&x| *cx.u.entities[x].name == *name)) else {
            cx.error(mi.span, "CTOR-15", format!("'{}' is not a non-static data member of the class", name));
            return None;
        };
        if given[k].is_some() {
            cx.error(mi.span, "CTOR-15", format!("the member '{}' has two initializers", name));
            return None;
        }
        // UNI-8: one mem-init for the members of a union.
        if crate::classes::unions::check::is_union(cx, c) && given.iter().any(|g| g.is_some()) {
            cx.error(mi.span, "UNI-8", "a constructor of a union initializes two members");
            return None;
        }
        let fty = match fields.get(k) {
            Some(&x) => cx.u.field(x).unwrap().ty.clone(),
            None => QType::new(Type::Class(vbases[k - fields.len()])),
        };
        given[k] = Some(direct_init(cx, &fty, args, braced.is_some(), mi.span)?);
    }
    let vgiven = given.split_off(fields.len());
    let mut out = Vec::new();
    // A union: the member that a mem-init names, or else the one with a
    // default member initializer, is active; the others are not (UNI-4).
    let union = crate::classes::unions::check::is_union(cx, c);
    let named = given.iter().any(|g| g.is_some());
    for (k, g) in given.into_iter().enumerate() {
        let x = cx.u.field(fields[k]).unwrap().clone();
        if union {
            out.push(g.or(if named { None } else { x.init.clone() }).unwrap_or(Init::Default));
            continue;
        }
        if g.is_none() && crate::classes::members::check::temporary_default(cx, fields[k], cx.u.entities[f].span) {
            return None;
        }
        out.push(match g.or(x.init.clone()) {
            Some(i) => i,
            None => {
                if x.ty.base_elem().c && x.ty.base_elem().is_scalar() {
                    cx.error(cx.u.entities[f].span, "CTOR-3", format!("the constructor leaves the const member '{}' without a value", cx.u.entities[fields[k]].name));
                    return None;
                }
                if x.ty.is_ref() {
                    cx.error(cx.u.entities[f].span, "CTOR-3", format!("the constructor leaves the reference member '{}' unbound", cx.u.entities[fields[k]].name));
                    return None;
                }
                default_init(cx, &x.ty, cx.u.entities[f].span)?
            }
        });
    }
    out.extend(vbase_inits(cx, c, vgiven, cx.u.entities[f].span)?);
    Some(MemInits::Fields(out))
}

/// Whether the elements of an array type are arrays.
fn el_is_array(ty: &QType) -> bool {
    ty.elem().is_some_and(|e| e.is_array())
}

/// Direct-initialization of an object of type `ty` from arguments in
/// parentheses or braces (INIT rules, LIST rules, CTOR-25).
pub fn direct_init(cx: &mut Check, ty: &QType, args: Vec<Expr>, list: bool, span: Span) -> Option<Init> {
    if let Some(c) = ty.class() {
        if has_ctors(cx, c) {
            return construct_init(cx, ty, args, list, false, span);
        }
        if args.is_empty() {
            return Some(crate::classes::members::check::value_init(cx, ty));
        }
        if args.len() == 1 && args[0].ty.nonref().class().is_some_and(|d| crate::classes::derived::check::derives(cx, d, c)) {
            return crate::declarations::initialization::forms::check::copy_init(cx, args[0].clone(), ty, "INIT-7").map(Init::Expr);
        }
        cx.unsupported(span, "an aggregate from a member initializer of this form (build step 4)");
        return None;
    }
    // A reference member binds its one value (CTOR-17).
    if ty.is_ref() && args.len() == 1 {
        return crate::classes::members::check::bind_member(cx, args[0].clone(), ty, span);
    }
    // An array member: from braces, each value initializes an element in
    // order (CTOR-21), and the rest are value-initialized; from empty
    // parentheses, each element is (INIT-8).
    if let (Type::Array(el, Some(n)), false) = (&ty.t, el_is_array(ty)) {
        if !list && !args.is_empty() {
            cx.error(span, "INIT-8", "an array member initialized from a value in parentheses");
            return None;
        }
        if args.len() as u64 > *n {
            cx.error(span, "CTOR-21", format!("{} values for an array of {} elements", args.len(), n));
            return None;
        }
        let mut out = Vec::new();
        for e in args {
            let i = if el.is_class() {
                Init::Expr(crate::declarations::initialization::forms::check::copy_init(cx, e, el, "CTOR-21")?)
            } else {
                Init::Expr(crate::declarations::initialization::list::check::braced_value(cx, e, el, "CTOR-21")?)
            };
            out.push(i);
        }
        while (out.len() as u64) < *n {
            out.push(crate::classes::members::check::value_init(cx, el));
        }
        return Some(Init::List(out));
    }
    if ty.is_ref() || ty.is_array() {
        cx.unsupported(span, "a member initializer for a reference or an array of arrays (build step 4)");
        return None;
    }
    match args.len() {
        0 => Some(Init::Zero),
        1 => {
            let e = args[0].clone();
            if list {
                return crate::declarations::initialization::list::check::braced_value(cx, e, ty, "CTOR-21").map(Init::Expr);
            }
            let span = e.span;
            match crate::foundations::conversions::check::implicit(cx, e, ty, true) {
                Ok(x) => Some(Init::Expr(x)),
                Err(m) => {
                    cx.error_cause(span, crate::foundations::conversions::check::rule_of(&m).unwrap_or("INIT-7"), crate::foundations::conversions::check::cause_of(&m), m);
                    None
                }
            }
        }
        _ => {
            cx.error(span, "INIT-7", "several values for an object of scalar type");
            None
        }
    }
}

/// The initialization of an object of class type by one of its
/// constructors (CTOR-25, CTOR-9): overload resolution over the
/// constructors, without the explicit ones for copy-initialization; a
/// deleted constructor is not used (DEL-1); a braced list does not
/// narrow (LIST-2).
pub fn construct_init(cx: &mut Check, ty: &QType, args: Vec<Expr>, list: bool, copy: bool, span: Span) -> Option<Init> {
    let c = ty.class()?;
    // A class whose only constructors are inherited keeps its implicit
    // default constructor (DER-8).
    if args.is_empty() && ctors(cx, c).is_empty() {
        return default_init(cx, ty, span);
    }
    let mut cands = ctors(cx, c);
    cands.extend(inherited(cx, c, args.len()));
    // CTOR-9: copy-initialization does not use an explicit constructor;
    // copy-list-initialization considers it and fails if it is chosen
    // (ILC-8).
    let dropped: Vec<EntityId> = if copy && !list { cands.iter().copied().filter(|&f| cx.u.func(f).unwrap().explicit).collect() } else { Vec::new() };
    cands.retain(|f| !dropped.contains(f));
    // CTOR-9 names the failure when only an explicit constructor was left
    // out.
    let none_left = |cx: &mut Check, dropped: bool| {
        let msg = format!("no constructor of '{}' takes these arguments", cx.u.classes[c].name);
        if dropped {
            cx.error_cause(span, "CTOR-9", Some("CTOR-25"), format!("{} in copy-initialization, which leaves out the explicit ones", msg));
        } else {
            cx.error(span, "CTOR-25", msg);
        }
    };
    if cands.is_empty() {
        none_left(cx, !dropped.is_empty());
        return None;
    }
    // CPY-9, CPY-10: an object of the class itself is copied or moved,
    // and a prvalue initializes the object directly.
    if args.len() == 1 && args[0].ty.nonref().class().is_some_and(|d| crate::classes::derived::check::derives(cx, d, c)) {
        let a = crate::foundations::values::check::deref_ref(args[0].clone());
        let a = if a.ty.class() != Some(c) {
            let a = if a.cat == Cat::PR {
                let t = a.ty.clone();
                a.conv(crate::sem::tree::Conv::Materialize, t, Cat::X)
            } else {
                a
            };
            crate::classes::derived::check::to_base(cx, a, c, span)?
        } else {
            a
        };
        if a.cat == Cat::PR {
            return Some(Init::Expr(a));
        }
        return crate::classes::copy_move::check::copy(cx, a, copy && !list, span).map(Init::Expr);
    }
    // A conversion function to the class can make the argument of a copy
    // or move constructor (OVL-40), which a later piece models.
    let to_class = args.iter().any(|a| {
        crate::classes::conversion_functions::check::converting_class(cx, a).is_some_and(|k| {
            crate::classes::conversion_functions::check::functions(&cx.u, k).iter().any(|&f| cx.u.func(f).is_some_and(|x| x.ty.ret.nonref().class().is_some_and(|r| crate::classes::derived::check::derives(cx, r, c))))
        })
    });
    if to_class && args.len() == 1 {
        return crate::classes::conversion_functions::check::class_from_class(cx, ty, args.into_iter().next().unwrap(), !copy, list, span);
    }
    if to_class {
        cx.unsupported(span, "a conversion function to the class in the argument of a constructor (build step 4)");
        return None;
    }
    // The copy and move constructors take no other argument here: their
    // parameter takes no user-defined conversion.
    cands.retain(|&f| crate::classes::copy_move::check::copy_or_move(cx, c, f).is_none());
    if cands.is_empty() {
        let dropped = dropped.iter().any(|&f| crate::classes::copy_move::check::copy_or_move(cx, c, f).is_none());
        none_left(cx, dropped);
        return None;
    }
    let name = cx.u.classes[c].name.to_string();
    let f = crate::overloading::overload_resolution::check::resolve(cx, &cands, &args, span, &name)?;
    let Some(func) = cx.u.func(f).cloned() else {
        cx.unsupported(span, format!("a constructor of '{}' that the front end does not model", name));
        return None;
    };
    if copy && list && func.explicit {
        cx.error_cause(span, "CTOR-9", Some("ILC-8"), format!("copy-list-initialization of '{}' chooses an explicit constructor", name));
        return None;
    }
    if func.deleted {
        cx.error(span, "DEL-1", format!("the constructor of '{}' that the initialization chooses is deleted", name));
        return None;
    }
    // DER-20: an inherited constructor has its access in the base.
    let base = func.class.filter(|&b| b != c);
    if !crate::classes::derived::check::accessible_through(cx, f, base.unwrap_or(c)) {
        cx.error(span, if base.is_some() { "DER-20" } else { "ACC-2" }, format!("the constructor of '{}' is not public", name));
        return None;
    }
    // The parameters after the arguments take their default arguments.
    let given = args.len();
    if given > func.ty.params.len() || (given..func.ty.params.len()).any(|k| func.defaults.get(k).is_none_or(|d| d.is_none())) {
        cx.error(span, "CTOR-25", format!("the constructor of '{}' takes {} arguments", name, func.ty.params.len()));
        return None;
    }
    let mut out = Vec::new();
    for (a, p) in args.into_iter().zip(func.ty.params.iter()) {
        if list && !p.is_ref() && crate::foundations::conversions::check::is_narrowing(cx, &a, p) {
            cx.error(a.span, "LIST-2", "a narrowing conversion in a braced list");
            return None;
        }
        // CTOR-22: the conversion that a converting constructor makes
        // takes standard conversions for its arguments.
        if copy && p.nonref().class().is_some_and(|d| has_ctors(cx, d) && a.ty.nonref().class() != Some(d)) {
            cx.error(a.span, "CTOR-22", "an implicit conversion uses two converting constructors");
            return None;
        }
        out.push(crate::declarations::initialization::forms::check::init_parameter(cx, a, p)?);
    }
    for k in given..func.ty.params.len() {
        if let Some(Some(d)) = func.defaults.get(k) {
            out.push(crate::sem::tree::Arg { e: d.clone(), by_ref: func.ty.params[k].is_ref() });
        }
    }
    crate::declarations::functions::definitions::check::note_use(cx, f, span);
    // DER-8: an inherited constructor initializes its base subobject,
    // and the other members as the implicit default constructor does.
    if let Some(b) = base {
        let k = cx.u.classes[c].fields.iter().position(|&x| cx.u.field(x).is_some_and(|y| y.base && y.ty.class() == Some(b)))?;
        let Init::List(mut items) = implicit_default_but(cx, c, Some(k), span)? else { return None };
        items[k] = Init::Ctor(f, out);
        return Some(Init::List(items));
    }
    immediate(cx, ty, Init::Ctor(f, out), span)
}

/// CEV-1: a call of a `consteval` constructor outside an immediate
/// function context is a constant expression.
pub fn immediate(cx: &mut Check, ty: &QType, init: Init, span: Span) -> Option<Init> {
    let Init::Ctor(f, _) = &init else { return Some(init) };
    if !cx.u.func(*f).is_some_and(|x| x.consteval) {
        return Some(init);
    }
    let marks = cx.u.diags.len();
    let e = Expr::new(crate::sem::tree::EK::List(Box::new(init.clone())), ty.unqual(), Cat::PR, span);
    crate::declarations::constexpr::check::immediate_call(cx, *f, e);
    if cx.u.diags[marks..].iter().any(|d| d.sev == crate::diag::Severity::Error) {
        return None;
    }
    Some(init)
}

/// The implicit default constructor's work: each member's default
/// member initializer or default-initialization.
fn implicit_default(cx: &mut Check, c: ClassId, span: Span) -> Option<Init> {
    implicit_default_but(cx, c, None, span)
}

/// The initializers of the virtual bases of `c`, after those of its
/// data members: the one that `given` holds, or the default
/// constructor's (DER-13). A constructor of an abstract class never
/// initializes them, because its class is never the most derived one.
fn vbase_inits(cx: &mut Check, c: ClassId, given: Vec<Option<Init>>, span: Span) -> Option<Vec<Init>> {
    let vbases = cx.u.classes[c].vbases.clone();
    let skip = crate::classes::r#virtual::check::is_abstract(cx, c);
    let mut out = Vec::new();
    for (j, v) in vbases.into_iter().enumerate() {
        if let Some(Some(i)) = given.get(j) {
            out.push(i.clone());
            continue;
        }
        if skip {
            out.push(Init::Default);
            continue;
        }
        let marks = cx.u.diags.len();
        match default_init(cx, &QType::new(Type::Class(v)), span) {
            Some(i) => out.push(i),
            None => {
                cx.u.diags.truncate(marks);
                cx.error(span, "DER-13", format!("the virtual base '{}' of '{}' has no default constructor and no initializer", cx.u.classes[v].name, cx.u.classes[c].name));
                return None;
            }
        }
    }
    Some(out)
}

/// `implicit_default`, with no initializer for the member at `skip`.
fn implicit_default_but(cx: &mut Check, c: ClassId, skip: Option<usize>, span: Span) -> Option<Init> {
    let fields = cx.u.classes[c].fields.clone();
    let union = crate::classes::unions::check::is_union(cx, c);
    let mut out = Vec::new();
    for (k, f) in fields.into_iter().enumerate() {
        if skip == Some(k) {
            out.push(Init::Default);
            continue;
        }
        let x = cx.u.field(f).unwrap().clone();
        if crate::classes::members::check::temporary_default(cx, f, span) {
            return None;
        }
        out.push(match x.init {
            Some(i) => i,
            // A union's member without an initializer stays inactive (UNI-4).
            None if union => Init::Default,
            // CTOR-3: a reference member without an initializer deletes the
            // default constructor.
            None if x.ty.is_ref() => {
                cx.error(span, "CTOR-20", format!("the default constructor of the class is deleted: the reference member '{}' has no initializer", cx.u.entities[f].name));
                return None;
            }
            None => default_init(cx, &x.ty, span)?,
        });
    }
    out.extend(vbase_inits(cx, c, Vec::new(), span)?);
    Some(Init::List(out))
}

/// Default-initialization of an object of type `ty` (INIT rules,
/// CTOR-19, CTOR-20): the default constructor of a class with
/// constructors, each member's for a class without, each element's for
/// an array.
pub fn default_init(cx: &mut Check, ty: &QType, span: Span) -> Option<Init> {
    match &ty.t {
        Type::Class(c) if has_ctors(cx, *c) && !ctors(cx, *c).is_empty() => {
            let defaults: Vec<EntityId> = ctors(cx, *c).into_iter().filter(|&f| cx.u.func(f).is_some_and(|x| x.takes_no_argument())).collect();
            if defaults.is_empty() {
                cx.error(span, "CTOR-19", format!("the class '{}' has no default constructor", cx.u.classes[*c].name));
                return None;
            }
            if defaults.iter().all(|f| cx.u.defaulted.contains(f)) && !defaults.iter().any(|&f| cx.u.func(f).unwrap().deleted) {
                return implicit_default(cx, *c, span);
            }
            construct_init(cx, ty, Vec::new(), false, false, span)
        }
        Type::Class(c) => {
            if !crate::classes::members::check::check_default_init(cx, ty, span) {
                return None;
            }
            implicit_default(cx, *c, span)
        }
        Type::Array(e, Some(n)) if e.base_elem().is_class() => {
            let one = default_init(cx, e, span)?;
            Some(Init::List((0..*n).map(|_| one.clone()).collect()))
        }
        _ => Some(Init::Default),
    }
}

/// CTOR-9: the condition of `explicit(c)` in the `explicit_spec` `n`, a
/// constant of type `bool` whose conversion does not narrow (CE-19).
/// `explicit` alone is true, and so is a condition with an error.
pub fn explicit_condition(cx: &mut Check, n: &Node) -> bool {
    let Some(c) = n.kids.iter().find_map(|k| match k {
        crate::syntax::Kid::Node(x) => Some(x),
        _ => None,
    }) else {
        return true;
    };
    let e = expr(cx, c);
    let e = crate::foundations::values::check::rvalue(cx, e);
    if e.ty.is_error() {
        return true;
    }
    let to = QType::bool();
    if e.ty.t != to.t {
        if !(e.ty.is_arithmetic() || e.ty.is_enum()) {
            cx.error_cause(e.span, "CTOR-9", Some("CE-11"), format!("the condition of 'explicit' has type {}, not bool", e.ty.show()));
            return true;
        }
        if crate::foundations::conversions::check::is_narrowing(cx, &e, &to) {
            cx.error(e.span, "CE-19", format!("the condition of 'explicit' narrows from {} to bool", e.ty.show()));
            return true;
        }
    }
    let b = crate::foundations::conversions::check::contextual_bool(cx, e, "CTOR-9");
    if b.ty.is_error() {
        return true;
    }
    match crate::foundations::constant_evaluation::check::evaluate(cx, &b) {
        Ok(v) => v.truthy(),
        Err(why) => {
            crate::foundations::constant_evaluation::check::report(cx, b.span, "CTOR-9", why);
            true
        }
    }
}

/// ICS-17: the sequence of the braced list `b` for the parameter `p`,
/// found by a trial initialization whose diagnostics are dropped. A list
/// of one element of the parameter's class, or of a derived class, has
/// that element's sequence. A class takes a list by a user-defined
/// conversion, which `no_user` rules out (the first parameter of a copy
/// or move constructor for a list of one element). None when the list
/// does not convert; an unsupported construct stays reported.
fn list_sequence(cx: &mut Check, b: &Node, p: &QType, no_user: bool) -> Option<crate::overloading::conversion_sequences::check::Ics> {
    use crate::overloading::conversion_sequences::check::{sequence, Ics, Rank};
    let its = crate::declarations::initialization::list::check::items(b);
    let target = p.nonref().clone();
    let marks = cx.u.diags.len();
    let one = (its.len() == 1).then(|| first_node(its[0])).flatten().filter(|c| c.prod != "braced_init" && c.prod != "designator").map(|c| expr(cx, c));
    if let (Some(e), Some(tc)) = (&one, target.class()) {
        if e.ty.nonref().class().is_some_and(|d| crate::classes::derived::check::derives(cx, d, tc)) {
            let r = sequence(cx, e, p);
            cx.u.diags.truncate(marks);
            return r;
        }
    }
    if target.is_class() && no_user {
        cx.u.diags.truncate(marks);
        return None;
    }
    let r = crate::declarations::initialization::list::check::scalar_list(cx, b, &target.unqual(), "INIT-7");
    let new = &cx.u.diags[marks..];
    if new.iter().any(|d| d.sev == crate::diag::Severity::Unsupported) {
        return None;
    }
    let ok = r.is_some() && !new.iter().any(|d| d.sev == crate::diag::Severity::Error);
    cx.u.diags.truncate(marks);
    if !ok {
        return None;
    }
    let plain = Ics { rank: Rank::Exact, qual: false, binds: None, ellipsis: false, user: None };
    if target.is_class() {
        return Some(Ics { rank: Rank::User, ..plain });
    }
    match &one {
        Some(e) => {
            let s = sequence(cx, e, &target);
            cx.u.diags.truncate(marks);
            s.or(Some(plain))
        }
        None => Some(plain),
    }
}

/// CTOR-25, ICS-17: the initialization of an object of class type `ty`
/// by a constructor whose arguments `nodes` hold braced lists. Each list
/// has the sequence of `list_sequence` for its parameter, and the best
/// constructor initializes each parameter from its list.
pub fn construct_braced(cx: &mut Check, ty: &QType, nodes: &[&Node], list: bool, copy: bool, span: Span) -> Option<Init> {
    use crate::overloading::conversion_sequences::check::{better, sequence, Ics};
    let c = ty.class()?;
    // ILC-7: a class that is not an aggregate takes no designator.
    if nodes.iter().any(|n| n.prod == "designator") {
        cx.error(span, "ILC-7", format!("a designated initializer for '{}', which is not an aggregate", cx.u.classes[c].name));
        return None;
    }
    let mut args: Vec<Result<Expr, &Node>> = Vec::new();
    for n in nodes {
        if n.prod == "braced_init" {
            args.push(Err(*n));
        } else {
            let e = expr(cx, n);
            if e.ty.is_error() {
                return None;
            }
            args.push(Ok(e));
        }
    }
    let mut cands = ctors(cx, c);
    cands.extend(inherited(cx, c, args.len()));
    let name = cx.u.classes[c].name.to_string();
    let mut viable: Vec<(EntityId, Vec<Ics>)> = Vec::new();
    for f in cands {
        let Some(g) = cx.u.func(f).cloned() else { continue };
        let n = g.ty.params.len();
        if args.len() > n || (args.len()..n).any(|k| g.defaults.get(k).is_none_or(|d| d.is_none())) {
            continue;
        }
        let copy_move = args.len() == 1 && crate::classes::copy_move::check::copy_or_move(cx, c, f).is_some();
        let mut seqs = Vec::new();
        for (k, a) in args.iter().enumerate() {
            let s = match a {
                Ok(e) => sequence(cx, e, &g.ty.params[k]),
                Err(b) => list_sequence(cx, b, &g.ty.params[k], copy_move && k == 0),
            };
            match s {
                Some(s) => seqs.push(s),
                None => break,
            }
        }
        if seqs.len() == args.len() {
            viable.push((f, seqs));
        }
    }
    if viable.is_empty() {
        cx.error(span, "CTOR-25", format!("no constructor of '{}' takes these arguments", name));
        return None;
    }
    let wins = |a: &[Ics], b: &[Ics]| !a.iter().zip(b).any(|(x, y)| better(y, x)) && a.iter().zip(b).any(|(x, y)| better(x, y));
    let mut best = 0;
    for j in 1..viable.len() {
        if wins(&viable[j].1, &viable[best].1) {
            best = j;
        }
    }
    if (0..viable.len()).any(|j| j != best && !wins(&viable[best].1, &viable[j].1)) {
        cx.error(span, "OVL-36", format!("the initialization of '{}' is ambiguous between its constructors", name));
        return None;
    }
    let f = viable[best].0;
    let g = cx.u.func(f)?.clone();
    if g.deleted {
        cx.error(span, "DEL-1", format!("the constructor of '{}' is deleted", name));
        return None;
    }
    if copy && list && g.explicit {
        cx.error_cause(span, "CTOR-9", Some("ILC-8"), format!("copy-list-initialization of '{}' chooses an explicit constructor", name));
        return None;
    }
    if !crate::classes::derived::check::accessible_through(cx, f, g.class.unwrap_or(c)) {
        cx.error(span, "ACC-2", format!("the constructor of '{}' is not public", name));
        return None;
    }
    let mut out = Vec::new();
    for (a, p) in args.into_iter().zip(g.ty.params.iter()) {
        let e = match a {
            Ok(e) => e,
            Err(b) => crate::declarations::initialization::list::check::scalar_list(cx, b, &p.nonref().unqual().with_cv(p.nonref().c, p.nonref().v), "INIT-7")?,
        };
        out.push(crate::declarations::initialization::forms::check::init_parameter(cx, e, p)?);
    }
    for k in out.len()..g.ty.params.len() {
        if let Some(Some(d)) = g.defaults.get(k) {
            out.push(crate::sem::tree::Arg { e: d.clone(), by_ref: g.ty.params[k].is_ref() });
        }
    }
    crate::declarations::functions::definitions::check::note_use(cx, f, span);
    if g.class.is_some_and(|b| b != c) {
        cx.unsupported(span, "an inherited constructor chosen for a braced argument");
        return None;
    }
    immediate(cx, ty, Init::Ctor(f, out), span)
}

