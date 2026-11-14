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

/// The constructors of a class.
pub fn ctors(cx: &Check, c: ClassId) -> Vec<EntityId> {
    names::declared_here(cx, cx.u.classes[c].scope, CTOR)
}

/// A class with a user-declared constructor, which is no aggregate and
/// has no implicit default constructor (CTOR-2).
pub fn has_ctors(cx: &Check, c: ClassId) -> bool {
    !ctors(cx, c).is_empty()
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
    if specs.unsupported || specs.is_static || specs.typedef || specs.ty.is_some() {
        cx.unsupported(dn.span, "a constructor declaration of this form (build step 4)");
        return false;
    }
    specs.ty = Some(QType::void());
    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
    if d.unsupported || !d.ty.is_func() || d.obj_cv != (false, false) || d.ref_q {
        cx.unsupported(dn.span, "a constructor declaration of this form (build step 4)");
        return false;
    }
    if dn.find("exception_spec").is_some_and(|e| first_node(e).is_some()) {
        cx.unsupported(dn.span, "a constructor with a noexcept operand (build step 4)");
        return false;
    }
    let local = cx.u.is_local(cx.u.scopes[cx.u.classes[c].scope].parent.unwrap_or(0));
    if local || specs.consteval || specs.attrs.nodiscard || specs.attrs.deprecated.is_some() || specs.attrs.noreturn || dn.find("attribute_spec").is_some() || def.is_some_and(|x| x.child("attribute_spec").is_some()) {
        cx.unsupported(dn.span, "a constructor of a local class, a consteval constructor, or a constructor with attributes (build step 4)");
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
    if specs.unsupported || cx.u.is_local(cx.u.scopes[cx.u.classes[c].scope].parent.unwrap_or(0)) || specs.consteval || def.is_some_and(|x| x.child("attribute_spec").is_some()) || dn.find("exception_spec").is_some_and(|e| first_node(e).is_some()) {
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

/// The destructor of class `c`, or of a member's class, is deleted or not
/// accessible: from the point of use when `top` is set, and those of its
/// members and bases from the class (DER-14: a base's protected one).
pub fn dtor_blocked(cx: &Check, c: ClassId, top: bool, base: bool) -> bool {
    let own = names::declared_here(cx, cx.u.classes[c].scope, DTOR).first().is_some_and(|&d| {
        let access = cx.u.access.get(&d).copied().unwrap_or(crate::sem::Access::Public);
        let ok = if top { crate::classes::members::check::accessible(cx, d) } else { access == crate::sem::Access::Public || (base && access == crate::sem::Access::Protected) };
        cx.u.func(d).is_some_and(|f| f.deleted) || !ok
    });
    own || cx.u.classes[c].fields.clone().iter().any(|&f| cx.u.field(f).is_some_and(|x| x.ty.base_elem().class().is_some_and(|d| dtor_blocked(cx, d, false, x.base))))
}

/// DTOR-3: the destructor of a class, or of a member's class, is
/// deleted or not accessible, so no object of the class can end. True
/// when it can.
pub fn dtor_usable(cx: &mut Check, ty: &QType, span: Span) -> bool {
    let Some(c) = ty.base_elem().class() else { return true };
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
            inits.push(match x.init {
                Some(i) => i,
                None if union => Init::Default,
                None => default_init(cx, &x.ty, def.span).unwrap_or(Init::Default),
            });
        }
        cx.u.mem_inits.insert(f, MemInits::Fields(inits));
        if let Some(x) = cx.u.func_mut(f) {
            x.body = Some(Rc::new(crate::sem::tree::Stmt::new(crate::sem::tree::SK::Block(Vec::new()), def.span)));
            x.defined = true;
        }
        return;
    }
    let Some(cs) = body.child("compound_stmt") else {
        cx.unsupported(body.span, "a function try block (build step 4)");
        return;
    };
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
    if dn.find("parameter").is_some_and(|_| cx.source(dn).contains('=')) {
        cx.unsupported(dn.span, "a default argument on an out-of-class constructor definition (build step 4)");
        return true;
    }
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
    let mut given: Vec<Option<Init>> = vec![None; fields.len()];
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
        let base_k = fields.iter().position(|&x| cx.u.field(x).is_some_and(|y| y.base && (y.ty.class().is_some_and(|b| *cx.u.classes[b].name == *name) || (named_class.is_some() && y.ty.class() == named_class))));
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
        let fty = cx.u.field(fields[k]).unwrap().ty.clone();
        given[k] = Some(direct_init(cx, &fty, args, braced.is_some(), mi.span)?);
    }
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
    Some(MemInits::Fields(out))
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
    if ty.is_ref() && args.len() == 1 && !list {
        return crate::classes::members::check::bind_member(cx, args[0].clone(), ty, span);
    }
    if ty.is_ref() || ty.is_array() {
        cx.unsupported(span, "a member initializer for a reference or an array (build step 4)");
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
    let mut cands = ctors(cx, c);
    // CTOR-9: copy-initialization does not use an explicit constructor;
    // copy-list-initialization considers it and fails if it is chosen
    // (ILC-8).
    if copy && !list {
        cands.retain(|&f| !cx.u.func(f).unwrap().explicit);
    }
    if cands.is_empty() {
        cx.error(span, "CTOR-25", format!("no constructor of '{}' takes these arguments", cx.u.classes[c].name));
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
    if to_class {
        cx.unsupported(span, "a conversion function to the class in the argument of a constructor (build step 4)");
        return None;
    }
    // The copy and move constructors take no other argument here: their
    // parameter takes no user-defined conversion.
    cands.retain(|&f| crate::classes::copy_move::check::copy_or_move(cx, c, f).is_none());
    if cands.is_empty() {
        cx.error(span, "CTOR-25", format!("no constructor of '{}' takes these arguments", cx.u.classes[c].name));
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
    if !crate::classes::derived::check::accessible_through(cx, f, c) {
        cx.error(span, "ACC-2", format!("the constructor of '{}' is not public", name));
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
    Some(Init::Ctor(f, out))
}

/// The implicit default constructor's work: each member's default
/// member initializer or default-initialization.
fn implicit_default(cx: &mut Check, c: ClassId, span: Span) -> Option<Init> {
    let fields = cx.u.classes[c].fields.clone();
    let union = crate::classes::unions::check::is_union(cx, c);
    let mut out = Vec::new();
    for f in fields {
        let x = cx.u.field(f).unwrap().clone();
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
    Some(Init::List(out))
}

/// Default-initialization of an object of type `ty` (INIT rules,
/// CTOR-19, CTOR-20): the default constructor of a class with
/// constructors, each member's for a class without, each element's for
/// an array.
pub fn default_init(cx: &mut Check, ty: &QType, span: Span) -> Option<Init> {
    match &ty.t {
        Type::Class(c) if has_ctors(cx, *c) => {
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
