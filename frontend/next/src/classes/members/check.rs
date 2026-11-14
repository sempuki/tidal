//! The check part of `classes/members`, for build step 4 part a: class
//! definitions whose members are non-static data members, the access of
//! those members, and aggregate initialization (`declarations/
//! initialization/list`). Member functions, static members, bases, and
//! the other kinds of member are unsupported until a later part, so a
//! class that has one gives no verdict.

use crate::expressions::check::{bad, expr, first_node};
use crate::foundations::names::check as names;
use crate::foundations::values::check::deref_ref;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Conv, Expr, Init, EK};
use crate::sem::types::{ClassId, QType, Type};
use crate::sem::{Access, ClassInfo, ClassKey, EntKind, EntityId, Field, Linkage, ScopeKind};
use crate::source::Span;
use crate::syntax::{Kid, Node};

/// The class that a class name of `n` names in the scope, if any.
fn declared_class(cx: &Check, name: &str) -> Option<ClassId> {
    names::declared_here(cx, cx.scope, name).into_iter().find_map(|e| match cx.u.entities[e].kind {
        EntKind::Class(c) => Some(c),
        _ => None,
    })
}

/// A new class of the scope, incomplete.
fn new_class(cx: &mut Check, name: &str, key: ClassKey, span: Span) -> ClassId {
    let scope = cx.u.new_scope(ScopeKind::Class, Some(cx.scope), name);
    let id = cx.u.classes.len();
    cx.u.classes.push(ClassInfo { name: name.into(), key, scope, fields: Vec::new(), complete: false, is_final: false });
    if !name.is_empty() {
        names::declare_in(cx, cx.scope, name, EntKind::Class(id), span, Linkage::None);
    }
    id
}

/// The key of a class head.
fn key_of(cx: &Check, head: &Node) -> ClassKey {
    match cx.first_word(head) {
        "class" => ClassKey::Class,
        "union" => ClassKey::Union,
        _ => ClassKey::Struct,
    }
}

/// `elaborated_type` with a class key: the class of the name, or a new
/// incomplete class when the scope has none (CLS rules).
pub fn elaborated_class(cx: &mut Check, n: &Node) -> QType {
    if n.child("nested_name").is_some() || n.child("template_id").is_some() || n.kids.iter().any(|k| matches!(k, Kid::Node(x) if matches!(x.prod, "attribute_spec" | "alignment_specifier"))) {
        return cx.unsupported(n.span, "a qualified or template name, or an attribute, after a class key");
    }
    let Some(name) = n.find("class_name").map(|c| cx.source(c)).or_else(|| n.kids.iter().rev().find_map(|k| cx.token(k).filter(|t| t.kind == crate::lex::TK::Ident).map(|t| t.text.clone()))) else {
        return cx.unsupported(n.span, "an elaborated type specifier of this form");
    };
    // TYN rules: the name finds a class and skips other entities.
    let found = names::lookup_where(cx, &name, |k| matches!(k, EntKind::Class(_) | EntKind::Unsupported(_) | EntKind::Typedef(_)));
    for e in found {
        match &cx.u.entities[e].kind {
            EntKind::Class(c) => return QType::new(Type::Class(*c)),
            EntKind::Typedef(_) => {
                cx.error(n.span, "TYN-10", format!("'{}' after a class key names a typedef", name));
                return QType::error();
            }
            EntKind::Unsupported(_) => return cx.unsupported(n.span, "a class that the front end does not model"),
            _ => {}
        }
    }
    // A new class goes to the nearest namespace or block, outside the
    // classes being defined.
    let saved = cx.scope;
    while cx.u.scopes[cx.scope].kind == ScopeKind::Class {
        cx.scope = cx.u.scopes[cx.scope].parent.unwrap_or(0);
    }
    let key = key_of(cx, n);
    let c = new_class(cx, &name, key, n.span);
    cx.scope = saved;
    QType::new(Type::Class(c))
}

/// `class_def` (CLS-1, ACC-1): the class and its non-static data members,
/// then the default member initializers, where the class is complete
/// (CLS-2).
pub fn class_def(cx: &mut Check, n: &Node) -> QType {
    // A `union_def` is its own head, with no bases (`classes/unions`).
    let head = if n.prod == "union_def" { n } else {
        match n.child("class_head") {
            Some(h) => h,
            None => return QType::error(),
        }
    };
    if head.child("nested_name").is_some() || head.child("template_id").is_some() {
        return cx.unsupported(head.span, "a class with a qualified name (build step 4)");
    }
    if head.kids.iter().any(|k| matches!(k, Kid::Node(x) if matches!(x.prod, "attribute_spec" | "alignment_specifier"))) {
        return cx.unsupported(head.span, "an attribute or alignas on a class (build step 4)");
    }
    // The classes of namespace std have rules of their own (LSUP).
    if cx.u.scopes[cx.scope].name.as_ref() == "std" {
        return cx.unsupported(head.span, "a class of namespace std (build step 4)");
    }
    let key = key_of(cx, head);
    let name = head.child("class_name").map(|c| cx.source(c)).unwrap_or_default();
    let id = match declared_class(cx, &name).filter(|_| !name.is_empty()) {
        Some(c) if cx.u.classes[c].complete => {
            return cx.unsupported(head.span, format!("a second definition of the class '{}'", name));
        }
        Some(c) => c,
        None => new_class(cx, &name, key, head.span),
    };
    // TYN-10: `union` and the other class keys do not name one class.
    if (cx.u.classes[id].key == ClassKey::Union) != (key == ClassKey::Union) {
        cx.error(head.span, "TYN-10", format!("'{}' was declared with another class key", name));
        return QType::error();
    }
    cx.u.classes[id].key = key;
    cx.u.classes[id].is_final = cx.has_word(head, "final");
    // DER-1, DER-2: the base subobjects come first.
    if let Some(bc) = head.child("base_clause") {
        if !crate::classes::derived::check::base_clause(cx, id, key, bc) {
            cx.u.classes[id].complete = true;
            return QType::error();
        }
    }
    let saved = cx.scope;
    cx.scope = cx.u.classes[id].scope;
    let mut access = if key == ClassKey::Class { Access::Private } else { Access::Public };
    let mut inits: Vec<(EntityId, &Node)> = Vec::new();
    let mut bodies: Vec<&Node> = Vec::new();
    let mut ctor_bodies: Vec<(EntityId, &Node)> = Vec::new();
    let mut ok = true;
    let outer = cx.member_of.replace(id);
    for m in n.children("member_decl") {
        if !member(cx, id, m, &mut access, &mut inits, &mut bodies, &mut ctor_bodies) {
            ok = false;
            break;
        }
    }
    cx.u.classes[id].complete = true;
    // STG-1: the size fits in std::size_t.
    if ok {
        let max = cx.u.target.int_max(cx.u.target.size_t()) as u128;
        match cx.u.target.layout(&cx.u, id) {
            Some((size, _, _)) if (size as u128) <= max => {}
            _ => {
                cx.error(head.span, "STG-1", format!("the size of the class '{}' does not fit in std::size_t", name));
                ok = false;
            }
        }
    }
    // CLS-2: default member initializers and member function bodies see
    // the complete class.
    if ok {
        for (f, init) in inits {
            let Some(ty) = cx.u.field(f).map(|x| x.ty.clone()) else { continue };
            // CLS-2: `this` and the members are there; the machine runs
            // such an initializer with a later part.
            let this = QType::ptr_to(QType::new(Type::Class(id)));
            let saved_func = cx.func.replace(crate::sem::check::FnCtx { this: Some(this), ..Default::default() });
            let i = member_init(cx, &ty, init);
            cx.func = saved_func;
            if i.as_ref().is_some_and(|i| i.any(&|e| matches!(e.k, EK::This))) {
                cx.unsupported(init.span, "a default member initializer that uses this or another member (build step 4)");
                ok = false;
                break;
            }
            if let EntKind::Field(x) = &mut cx.u.entities[f].kind {
                x.init = i;
            }
        }
        if ok {
            crate::classes::r#virtual::check::complete(cx, id, head.span);
            for b in bodies {
                if b.prod == "friend_decl" {
                    crate::classes::access::check::definition(cx, b);
                } else {
                    crate::declarations::functions::definitions::check::function_definition(cx, b);
                }
            }
            for (f, b) in ctor_bodies {
                crate::classes::constructors::check::define(cx, f, b);
            }
            crate::classes::constructors::check::end_of_class(cx, id, head.span);
            crate::classes::unions::check::end_of_union(cx, id, head.span);
            // CLS-9: no member type or enumerator with the class's name.
            let own = cx.u.scopes[cx.u.classes[id].scope].names.get(name.as_str()).cloned().unwrap_or_default();
            if !name.is_empty() && own.iter().any(|&e| !matches!(cx.u.entities[e].kind, EntKind::Field(_) | EntKind::Func(_) | EntKind::Var(_))) {
                cx.error(head.span, "CLS-9", format!("a member type or enumerator has the name of the class '{}'", name));
            }
            crate::classes::r#virtual::check::after_bodies(cx, id);
        }
    }
    cx.member_of = outer;
    cx.scope = saved;
    QType::new(Type::Class(id))
}

/// One `member_decl`. False after an unsupported member. A member
/// function's body waits in `bodies` for the complete class.
#[allow(clippy::too_many_arguments)]
fn member<'n>(cx: &mut Check, id: ClassId, m: &'n Node, access: &mut Access, inits: &mut Vec<(EntityId, &'n Node)>, bodies: &mut Vec<&'n Node>, ctor_bodies: &mut Vec<(EntityId, &'n Node)>) -> bool {
    let Some(first) = first_node(m) else { return true };
    // A constructor or destructor: a declarator with the class's name.
    if first.prod == "member_declarator" {
        return match first.child("declarator") {
            Some(dn) if crate::classes::constructors::check::special_name(cx, id, dn).is_some() => {
                let ok = crate::classes::constructors::check::member(cx, id, None, dn, None, *access, ctor_bodies);
                if ok {
                    virtual_dtor(cx, id, false, first, dn);
                }
                ok
            }
            // A conversion function (CVF-1).
            Some(dn) if dn.find("conversion_function_id").is_some() && m.children("member_declarator").count() == 1 => {
                if first.kids.iter().skip(1).any(|k| !matches!(k, Kid::Node(x) if matches!(x.prod, "pure_specifier" | "virt_specifiers"))) {
                    cx.unsupported(first.span, "a pure, defaulted, or deleted member function (build step 4)");
                    return false;
                }
                let specs = crate::declarations::check::Specs::default();
                let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
                if d.unsupported || !d.ty.is_func() {
                    return !d.unsupported;
                }
                if !modeled_function(cx, id, dn, &d) {
                    return false;
                }
                if !valid_function(cx, id, &specs, &d) {
                    return true;
                }
                if let Some(f) = crate::declarations::functions::definitions::check::declare_function(cx, &specs, &d, None) {
                    cx.u.access.insert(f, *access);
                    crate::classes::r#virtual::check::member_function(cx, id, f, false, first, first.child("pure_specifier"), dn.span);
                }
                true
            }
            _ => {
                cx.unsupported(m.span, "a member declaration of this form (build step 4)");
                false
            }
        };
    }
    if first.prod == "function_definition" {
        if let Some(dn) = first.child("declarator").filter(|dn| crate::classes::constructors::check::special_name(cx, id, dn).is_some()) {
            let specs = match first.child("decl_specifiers") {
                Some(ds) => {
                    let s = crate::declarations::check::decl_specifiers(cx, ds, Default::default());
                    if s.ty.is_some() {
                        None
                    } else {
                        Some(s)
                    }
                }
                None => Some(Default::default()),
            };
            if let Some(specs) = specs {
                let ok = crate::classes::constructors::check::member(cx, id, Some(&specs), dn, Some(first), *access, ctor_bodies);
                if ok {
                    virtual_dtor(cx, id, specs.is_virtual, first, dn);
                }
                return ok;
            }
        }
    }
    match first.prod {
        "access_label" => {
            *access = match first.find("access_specifier").map(|a| cx.first_word(a)) {
                Some("private") => Access::Private,
                Some("protected") => Access::Protected,
                _ => Access::Public,
            };
            true
        }
        "function_definition" => {
            // The declaration now, the body when the class is complete.
            if let Some(e) = first.child("declarator").and_then(|d| d.find("exception_spec")).filter(|e| first_node(e).is_some()) {
                cx.unsupported(e.span, "a noexcept operand of a member function, which sees the complete class (build step 4)");
                return false;
            }
            let specs = match first.child("decl_specifiers") {
                Some(ds) => crate::declarations::check::decl_specifiers(cx, ds, Default::default()),
                // A conversion function (CVF-1).
                None if first.child("declarator").is_some_and(|d| d.find("conversion_function_id").is_some()) => Default::default(),
                None => {
                    cx.unsupported(first.span, "a constructor, destructor, or conversion function (build step 4)");
                    return false;
                }
            };
            if specs.unsupported {
                return false;
            }
            let Some(dn) = first.child("declarator") else { return true };
            let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
            if d.unsupported || !d.ty.is_func() {
                cx.unsupported(first.span, "a member function definition of this form (build step 4)");
                return false;
            }
            if !modeled_function(cx, id, dn, &d) {
                return false;
            }
            if !valid_function(cx, id, &specs, &d) {
                return true;
            }
            let Some(f) = crate::declarations::functions::definitions::check::declare_function(cx, &specs, &d, None) else { return true };
            cx.u.access.insert(f, *access);
            crate::classes::r#virtual::check::member_function(cx, id, f, specs.is_virtual, first, None, dn.span);
            cx.deferred.insert(f);
            // CLS rules: a member function defined in its class is inline.
            if let Some(x) = cx.u.func_mut(f) {
                x.inline = true;
            }
            bodies.push(first);
            true
        }
        "decl_specifiers" => {
            if let Some(ud) = crate::classes::unions::check::own_union(first) {
                // UNI-7: an anonymous union is an unnamed member whose
                // members are members of the class.
                let anonymous = ud.child("class_name").is_none() && ud.child("template_id").is_none() && m.children("member_declarator").next().is_none();
                if !anonymous {
                    cx.unsupported(m.span, "a nested union (build step 4)");
                    return false;
                }
                let specs = crate::declarations::check::decl_specifiers(cx, first, Default::default());
                if specs.unsupported {
                    return false;
                }
                let Some(u) = specs.ty.as_ref().and_then(|t| t.class()) else { return true };
                if specs.is_static {
                    cx.error(m.span, "UNI-9", "a static anonymous union member");
                    return true;
                }
                if !crate::classes::unions::check::anonymous(cx, u, m.span) {
                    return true;
                }
                let index = cx.u.classes[id].fields.len();
                let f = Field { ty: QType::new(Type::Class(u)), class: id, index, init: None, mutable: false, access: *access, base: false, width: None };
                let scope = cx.u.classes[id].scope;
                let fid = cx.u.add(crate::sem::Entity { name: "".into(), kind: EntKind::Field(f), span: m.span, scope, linkage: Linkage::None, deprecated: None });
                cx.u.classes[id].fields.push(fid);
                return true;
            }
            if let Some(e) = m.find("exception_spec").filter(|e| first_node(e).is_some()) {
                cx.unsupported(e.span, "a noexcept operand of a member function, which sees the complete class (build step 4)");
                return false;
            }
            if m.kids.iter().any(|k| matches!(k, Kid::Node(x) if !matches!(x.prod, "decl_specifiers" | "member_declarator" | "bitfield"))) {
                cx.unsupported(m.span, "a member of this form (build step 4)");
                return false;
            }
            let specs = crate::declarations::check::decl_specifiers(cx, first, Default::default());
            if specs.unsupported {
                return false;
            }
            // BF rules: the bit-fields of the declaration.
            if m.child("bitfield").is_some() {
                if m.child("member_declarator").is_some() {
                    cx.unsupported(m.span, "a bit-field and another member in one declaration (build step 4)");
                    return false;
                }
                for b in m.children("bitfield") {
                    match crate::classes::unions::check::bitfield(cx, id, &specs, b, *access, inits) {
                        Some(true) => {}
                        Some(false) => return true,
                        None => return false,
                    }
                }
                return true;
            }
            if specs.is_extern || specs.thread_local {
                cx.unsupported(m.span, "a member of this kind (extern or thread_local; build step 4)");
                return false;
            }
            // A nested class or enumeration: a member type with its access.
            if specs.defines_type && m.children("member_declarator").next().is_none() {
                // CLS-9: an unnamed class that declares no member.
                if specs.ty.as_ref().and_then(|t| t.class()).is_some_and(|c| cx.u.classes[c].name.is_empty()) {
                    cx.error(m.span, "CLS-9", "an unnamed class member that declares nothing");
                    return true;
                }
                if let Some(t) = &specs.ty {
                    let e = names::declared_here(cx, cx.scope, &type_name(cx, t)).into_iter().find(|&e| matches!(cx.u.entities[e].kind, EntKind::Class(_) | EntKind::Enum(_)));
                    if let Some(e) = e {
                        cx.u.access.insert(e, *access);
                    }
                }
                return true;
            }
            // A typedef member (TYN rules).
            if specs.typedef {
                for md in m.children("member_declarator") {
                    let Some(dn) = md.child("declarator") else { continue };
                    let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
                    if d.unsupported {
                        return false;
                    }
                    // TYN-2: a member typedef is declared once in its class.
                    if d.name.as_ref().is_some_and(|(n, _)| !names::declared_here(cx, cx.scope, n).is_empty()) {
                        cx.error(d.span, "TYN-2", "a member typedef declared again in its class");
                        return true;
                    }
                    crate::declarations::type_names::check::typedef_name(cx, &d);
                    if let Some((name, _)) = &d.name {
                        if let Some(e) = names::declared_here(cx, cx.scope, name).into_iter().find(|&e| matches!(cx.u.entities[e].kind, EntKind::Typedef(_))) {
                            cx.u.access.insert(e, *access);
                        }
                    }
                }
                return true;
            }
            if specs.explicit && specs.ty.is_some() {
                cx.error(m.span, "CTOR-9", "explicit on a member that is not a constructor");
                return true;
            }
            // CLS-9: no `mutable` with `const`.
            if specs.mutable && specs.ty.as_ref().is_some_and(|t| t.c) {
                cx.error_cause(m.span, "CLS-9", Some("STO-6"), "a mutable data member that is const");
                return true;
            }
            for md in m.children("member_declarator") {
                let Some(dn) = md.child("declarator") else {
                    cx.unsupported(md.span, "a member declarator of this form (build step 4)");
                    return false;
                };
                if (specs.ty.is_none() && crate::classes::constructors::check::special_name(cx, id, dn).is_some()) || crate::classes::constructors::check::special_name(cx, id, dn) == Some(true) {
                    if !crate::classes::constructors::check::member(cx, id, Some(&specs), dn, None, *access, ctor_bodies) {
                        return false;
                    }
                    virtual_dtor(cx, id, specs.is_virtual, md, dn);
                    continue;
                }
                if md.kids.iter().skip(1).any(|k| !matches!(k, Kid::Node(x) if matches!(x.prod, "member_init" | "pure_specifier" | "virt_specifiers"))) {
                    cx.unsupported(md.span, "a member declarator of this form (build step 4)");
                    return false;
                }
                let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
                if d.unsupported {
                    return false;
                }
                if d.ty.is_func() {
                    if md.child("member_init").is_some() {
                        cx.unsupported(md.span, "a pure, defaulted, or deleted member function (build step 4)");
                        return false;
                    }
                    if !modeled_function(cx, id, dn, &d) {
                        return false;
                    }
                    if !valid_function(cx, id, &specs, &d) {
                        return true;
                    }
                    if let Some(f) = crate::declarations::functions::definitions::check::declare_function(cx, &specs, &d, None) {
                        cx.u.access.insert(f, *access);
                        crate::classes::r#virtual::check::member_function(cx, id, f, specs.is_virtual, md, md.child("pure_specifier"), dn.span);
                    }
                    continue;
                }
                // VIRT-2: only a member function is virtual.
                if specs.is_virtual {
                    cx.error(d.name.as_ref().map(|n| n.1).unwrap_or(md.span), "VIRT-2", "a data member declared virtual");
                    return true;
                }
                if specs.is_static {
                    if !static_member(cx, id, &specs, &d, md.child("member_init"), *access) {
                        return false;
                    }
                    continue;
                }
                if specs.constexpr || specs.inline {
                    cx.unsupported(md.span, "constexpr or inline on a non-static data member (build step 4)");
                    return false;
                }
                // CLS-9: no `mutable` on a reference member.
                if specs.mutable && d.ty.is_ref() {
                    cx.error_cause(m.span, "CLS-9", Some("STO-6"), "a mutable reference member");
                    return true;
                }
                // CLS-9: no placeholder for a non-static data member.
                if specs.placeholder.is_some() {
                    cx.error(m.span, "CLS-9", "a data member declared with a placeholder type");
                    return true;
                }
                if d.ty.is_error() {
                    return true;
                }
                let Some((name, span)) = d.name.clone() else { continue };
                match crate::classes::unions::check::member(cx, id, &d.ty, &name, span) {
                    Some(true) => {}
                    Some(false) => return true,
                    None => return false,
                }
                // CLS-1: a data member has a complete type.
                if d.ty.is_incomplete() || d.ty.class().is_some_and(|c| !cx.u.classes[c].complete) {
                    cx.error(span, "CLS-1", format!("the data member '{}' has an incomplete type", name));
                    return true;
                }
                if !crate::classes::r#virtual::check::not_abstract(cx, &d.ty, span) {
                    return true;
                }
                let index = cx.u.classes[id].fields.len();
                let f = Field { ty: d.ty.clone(), class: id, index, init: None, mutable: specs.mutable, access: *access, base: false, width: None };
                let fid = names::declare_in(cx, cx.scope, &name, EntKind::Field(f), span, Linkage::None);
                cx.u.classes[id].fields.push(fid);
                if let Some(i) = md.child("member_init") {
                    inits.push((fid, i));
                }
            }
            true
        }
        "static_assert_decl" => {
            crate::declarations::static_assert::check::static_assert_decl(cx, first);
            true
        }
        "friend_decl" => {
            let ok = crate::classes::access::check::friend_decl(cx, id, first);
            if ok && first.child("function_definition").is_some() {
                bodies.push(first);
            }
            ok
        }
        // An alias member (TYN rules), with its access.
        "alias_decl" => {
            // TYN-2: a member alias is declared once in its class.
            let name = first.kids.iter().filter(|k| matches!(k, Kid::Tok(_))).map(|k| cx.text(k)).nth(1).unwrap_or("");
            if !name.is_empty() && !names::declared_here(cx, cx.scope, name).is_empty() {
                cx.error(first.span, "TYN-2", format!("the member alias '{}' is declared again in its class", name));
                return true;
            }
            let before = cx.u.entities.len();
            crate::declarations::type_names::check::alias_decl(cx, first);
            for e in before..cx.u.entities.len() {
                if matches!(cx.u.entities[e].kind, EntKind::Typedef(_)) {
                    cx.u.access.insert(e, *access);
                }
            }
            true
        }
        _ => {
            cx.unsupported(m.span, format!("a member of the form {} (build step 4)", first.prod));
            false
        }
    }
}

/// A member function declaration that build step 4's first pieces model:
/// no default argument or noexcept operand, which see the complete class,
/// in a class that is not local. True when it is modeled; otherwise an
/// unsupported diagnostic is out.
fn modeled_function(cx: &mut Check, id: ClassId, dn: &Node, d: &crate::declarations::declarators::check::Declared) -> bool {
    let parent = cx.u.scopes[cx.u.classes[id].scope].parent.unwrap_or(0);
    // A member function of a local or nested class waits for a later
    // part (access to the enclosing class, locals of the function).
    let local = cx.u.is_local(parent) || cx.u.scopes[parent].kind == ScopeKind::Class;
    let defaults = d.params.iter().any(|p| p.default.is_some());
    let operand = dn.find("exception_spec").is_some_and(|e| first_node(e).is_some());
    let qualified = d.ty.func().is_some_and(|f| f.qualified);
    if local || defaults || operand || qualified || d.ref_q {
        cx.unsupported(dn.span, "a member function of a local class, with a default argument, a noexcept operand, or a ref-qualifier (build step 4)");
        return false;
    }
    true
}

/// CLS-9, CLS-27, CTOR-1: a member function that has the class's name,
/// that is declared twice, or that is static with cv-qualifiers. True
/// when the declaration is valid.
fn valid_function(cx: &mut Check, id: ClassId, specs: &crate::declarations::check::Specs, d: &crate::declarations::declarators::check::Declared) -> bool {
    let Some((name, span)) = d.name.clone() else { return false };
    if *name == *cx.u.classes[id].name {
        cx.error_cause(span, "CTOR-1", Some("CLS-9"), "a member function with the class's name, which only a constructor has");
        return false;
    }
    if specs.is_static && (d.obj_cv.0 || d.obj_cv.1) {
        cx.error(span, "CLS-27", "a static member function with cv-qualifiers");
        return false;
    }
    let conversion = name.starts_with("operator ");
    if conversion && !crate::classes::conversion_functions::check::declaration(cx, specs, &d.ty, true, span) {
        return false;
    }
    let fty = d.ty.func().cloned();
    let twice = names::declared_here(cx, cx.scope, &name).into_iter().any(|e| cx.u.func(e).is_some_and(|f| Some(&*f.ty) == fty.as_ref() && f.obj_cv == d.obj_cv));
    if twice {
        let rule = if conversion { "CVF-7" } else { "CLS-9" };
        cx.error(span, rule, format!("the member function '{}' is declared twice in its class", name));
        return false;
    }
    true
}

/// A static data member (CLS rules): a variable of the class's scope.
/// Without `inline` or `constexpr` the declaration in the class is not a
/// definition, unless it is a const integral member with an initializer
/// that the program does not take the address of.
fn static_member(cx: &mut Check, class: ClassId, specs: &crate::declarations::check::Specs, d: &crate::declarations::declarators::check::Declared, init: Option<&Node>, access: Access) -> bool {
    let Some((name, span)) = d.name.clone() else { return true };
    // CLS-9: no `mutable` on a static member.
    if specs.mutable {
        cx.error_cause(span, "CLS-9", Some("STO-6"), "a mutable static data member");
        return true;
    }
    // CLS-28: a local class has no static data member.
    let parent = cx.u.scopes[cx.u.classes[class].scope].parent.unwrap_or(0);
    if cx.u.is_local(parent) {
        cx.error(span, "CLS-28", format!("the local class has the static data member '{}'", name));
        return true;
    }
    // CLS-9: not in a class with no name or nested in one, and not with
    // the name of the class.
    let mut s = Some(cx.u.classes[class].scope);
    while let Some(x) = s {
        if cx.u.scopes[x].kind == ScopeKind::Class && cx.u.scopes[x].name.is_empty() {
            cx.error(span, "CLS-9", format!("the static data member '{}' is in a class with no name", name));
            return true;
        }
        s = cx.u.scopes[x].parent;
    }
    if *name == *cx.u.classes[class].name {
        cx.error(span, "CLS-9", format!("the static data member '{}' has the name of its class", name));
        return true;
    }
    let definition = specs.inline || specs.constexpr;
    // AUTO-17: a placeholder needs an initializer and inline, constexpr,
    // or const (AUTO-14).
    if let Some(p) = &specs.placeholder {
        if init.is_none() || !(definition || p.c || d.ty.c) {
            cx.error(span, "AUTO-17", format!("the static data member '{}' has a placeholder type without an initializer and inline, constexpr, or const", name));
            return true;
        }
    }
    // CLS-23: only a const static member of integer or enumeration type
    // that is not volatile has an initializer in the class without
    // inline or constexpr (CLS-3).
    let integral = |t: &QType| t.c && !t.v && (t.is_integral() || t.is_enum());
    if init.is_some() && !definition && specs.placeholder.is_none() && !integral(&d.ty) {
        cx.error(span, "CLS-23", format!("the static data member '{}' has an initializer in the class", name));
        return true;
    }
    // A declaration in the class that is not a definition acts as an
    // extern declaration: its type can be incomplete, and the definition
    // outside the class gives the object (CLS-3).
    let mut specs = specs.clone();
    specs.is_static = false;
    // A constexpr static data member is inline.
    specs.inline |= specs.constexpr;
    if !definition && init.is_none() {
        specs.is_extern = true;
    }
    let mut first = None;
    let Some((id, _)) = crate::declarations::initialization::forms::check::declare_variable(cx, &specs, d, init, &[], &mut first) else { return true };
    cx.u.access.insert(id, access);
    // CLS-20: the initializer of a const integral member is a constant.
    if init.is_some() && !definition && cx.u.var(id).is_some_and(|v| integral(&v.ty) && !v.const_init) {
        cx.error(span, "CLS-20", format!("the initializer of the static data member '{}' is not a constant expression", name));
        return true;
    }
    true
}

/// A default member initializer (`member_init`) for a member of type
/// `ty`: `= expr`, `= {...}`, or `{...}`.
fn member_init(cx: &mut Check, ty: &QType, n: &Node) -> Option<Init> {
    let c = first_node(n)?;
    if ty.is_ref() {
        if c.prod == "braced_init" {
            cx.unsupported(c.span, "a braced default member initializer for a reference member");
            return None;
        }
        let e = expr(cx, c);
        if e.ty.is_error() {
            return None;
        }
        // CTOR-17 holds where a constructor uses the initializer, which
        // the front end does not track yet.
        let b = crate::declarations::initialization::references::check::bind(cx, e, ty, "INIT-7")?;
        if binds_temporary(&b) {
            cx.unsupported(n.span, "a default member initializer that binds a reference member to a temporary");
            return None;
        }
        return Some(Init::Ref(b));
    }
    if c.prod == "braced_init" {
        return crate::declarations::initialization::list::check::list_init(cx, c, ty, "");
    }
    let e = expr(cx, c);
    if e.ty.is_error() {
        return None;
    }
    crate::declarations::initialization::forms::check::copy_init(cx, e, ty, "INIT-7").map(Init::Expr)
}

/// A default-initialization that runs no code: each part default-
/// initialized, with no default member initializer (JMP-6, SW-5).
pub fn trivial_default(i: &Init) -> bool {
    match i {
        Init::Default => true,
        Init::List(xs) => !xs.is_empty() && xs.iter().all(trivial_default),
        _ => false,
    }
}

/// An object of type `ty` holds a polymorphic class object, whose
/// default constructor runs code (CTOR-23, VIRT-11).
pub fn has_polymorphic_part(cx: &Check, ty: &QType) -> bool {
    ty.base_elem().class().is_some_and(|c| cx.u.polymorphic(c) || cx.u.field_types(c).iter().any(|t| has_polymorphic_part(cx, t)))
}

/// A class whose default-initialization initializes each member: a
/// default member initializer for each, or a member of such a class
/// (CTOR-3, const-default-constructible).
fn const_default_ok(cx: &Check, c: ClassId) -> bool {
    cx.u.classes[c].fields.iter().all(|&f| {
        let x = cx.u.field(f).unwrap();
        x.init.is_some() || x.ty.base_elem().class().is_some_and(|d| const_default_ok(cx, d))
    })
}

/// A class with a const or reference data member, also through a
/// member of class type, whose implicit copy assignment is deleted
/// (CPY-7).
pub fn has_const_member(cx: &Check, c: ClassId) -> bool {
    cx.u.field_types(c).iter().any(|t| t.is_ref() || t.base_elem().c || t.base_elem().class().is_some_and(|d| has_const_member(cx, d)))
}

/// CTOR-3, CTOR-20: default-initialization of an object of type `ty`
/// uses the implicit default constructor, which a const member without a
/// default member initializer deletes. True when the use is valid.
pub fn check_default_init(cx: &mut Check, ty: &QType, span: Span) -> bool {
    let Some(c) = ty.base_elem().class() else { return true };
    // LAM-26: a closure type with captures has no default constructor.
    if crate::expressions::lambdas::check::has_captures(cx, c) {
        cx.error(span, "LAM-26", "the closure type of a lambda with captures has no default constructor");
        return false;
    }
    for f in cx.u.classes[c].fields.clone() {
        let x = cx.u.field(f).unwrap().clone();
        // A member whose destructor is deleted or not accessible.
        if x.ty.base_elem().class().is_some_and(|d| crate::classes::constructors::check::dtor_blocked(cx, d, false, x.base)) {
            cx.error(span, "CTOR-20", format!("the default constructor of the class is deleted: the destructor of the member '{}' is deleted or not accessible", cx.u.entities[f].name));
            return false;
        }
        if x.init.is_some() {
            continue;
        }
        // CTOR-3: a reference member without a default member initializer.
        if x.ty.is_ref() {
            cx.error(span, "CTOR-20", format!("the default constructor of the class is deleted: the reference member '{}' has no initializer", cx.u.entities[f].name));
            return false;
        }
        let base = x.ty.base_elem();
        // CTOR-3: a member whose class has no usable default constructor.
        if let Some(d) = base.class().filter(|&d| crate::classes::constructors::check::has_ctors(cx, d)) {
            // DER-14: a base's protected constructor is accessible.
            let ok = crate::classes::constructors::check::ctors(cx, d).into_iter().any(|g| cx.u.func(g).is_some_and(|y| y.ty.params.is_empty() && !y.deleted) && (accessible_from(cx, g, c) || (x.base && cx.u.access.get(&g) == Some(&Access::Protected))));
            if !ok {
                cx.error(span, "CTOR-20", format!("the default constructor of the class is deleted: the member '{}' has no usable default constructor", cx.u.entities[f].name));
                return false;
            }
            continue;
        }
        if base.c && (base.is_scalar() || base.class().is_some_and(|d| !const_default_ok(cx, d))) {
            cx.error(span, "CTOR-20", format!("the default constructor of the class is deleted: the const member '{}' has no initializer", cx.u.entities[f].name));
            return false;
        }
        if !check_default_init(cx, &x.ty, span) {
            return false;
        }
    }
    true
}

/// The virtual functions page for a destructor just declared: `n` holds
/// its virt-specifiers and pure specifier.
fn virtual_dtor(cx: &mut Check, id: ClassId, declared_virtual: bool, n: &Node, dn: &Node) {
    if crate::classes::constructors::check::special_name(cx, id, dn) != Some(true) {
        return;
    }
    let Some(&f) = names::declared_here(cx, cx.u.classes[id].scope, crate::classes::constructors::check::DTOR).last() else { return };
    crate::classes::r#virtual::check::member_function(cx, id, f, declared_virtual, n, n.child("pure_specifier"), dn.span);
}

/// A class is an aggregate when every data member is public: build step
/// 4 part a has no constructors, bases, or virtual functions (LIST
/// rules).
pub fn is_aggregate(cx: &Check, c: ClassId) -> bool {
    !crate::classes::constructors::check::has_ctors(cx, c) && !cx.u.polymorphic(c) && cx.u.classes[c].fields.iter().all(|&f| cx.u.field(f).is_some_and(|x| x.access == Access::Public))
}

/// Default-initialization of an object of type `ty`: each data member
/// gets its default member initializer or its own default-initialization
/// (INIT rules).
pub fn default_init(cx: &Check, ty: &QType) -> Init {
    if ty.class().is_some_and(|c| crate::classes::constructors::check::has_ctors(cx, c)) {
        return value_init(cx, ty);
    }
    // A union: the member with a default member initializer, or none
    // (UNI-4).
    if let Some(c) = ty.class().filter(|&c| crate::classes::unions::check::is_union(cx, c)) {
        return Init::List(cx.u.classes[c].fields.iter().map(|&f| cx.u.field(f).and_then(|x| x.init.clone()).unwrap_or(Init::Default)).collect());
    }
    match &ty.t {
        Type::Class(c) => Init::List(cx.u.classes[*c].fields.iter().map(|&f| {
            let x = cx.u.field(f).unwrap();
            x.init.clone().unwrap_or_else(|| default_init(cx, &x.ty))
        }).collect()),
        Type::Array(e, Some(n)) if e.base_elem().is_class() => Init::List((0..*n).map(|_| default_init(cx, e)).collect()),
        _ => Init::Default,
    }
}

/// Value-initialization of an object of type `ty`: each data member gets
/// its default member initializer or its own value-initialization.
pub fn value_init(cx: &Check, ty: &QType) -> Init {
    if let Some(c) = ty.class().filter(|&c| crate::classes::constructors::check::has_ctors(cx, c)) {
        // The default constructor (CTOR-19 reports a missing one where
        // the class is initialized).
        let d = crate::classes::constructors::check::ctors(cx, c).into_iter().find(|&f| cx.u.func(f).is_some_and(|x| x.ty.params.is_empty() && !x.deleted));
        return match d {
            Some(f) => Init::Ctor(f, Vec::new()),
            None => Init::Default,
        };
    }
    // A union: the member with a default member initializer, or else the
    // first member, value-initialized (UNI-4).
    if let Some(c) = ty.class().filter(|&c| crate::classes::unions::check::is_union(cx, c)) {
        let fields = &cx.u.classes[c].fields;
        if fields.iter().any(|&f| cx.u.field(f).is_some_and(|x| x.init.is_some())) {
            return default_init(cx, ty);
        }
        return Init::List(fields.iter().take(1).map(|&f| value_init(cx, &cx.u.field(f).unwrap().ty)).collect());
    }
    match &ty.t {
        Type::Class(c) => Init::List(cx.u.classes[*c].fields.iter().map(|&f| {
            let x = cx.u.field(f).unwrap();
            x.init.clone().unwrap_or_else(|| value_init(cx, &x.ty))
        }).collect()),
        Type::Array(e, Some(n)) if e.base_elem().is_class() => Init::List((0..*n).map(|_| value_init(cx, e)).collect()),
        _ => Init::Zero,
    }
}

/// The object of a member access: the operand of `.`, or the class that
/// the operand of `->` points to.
pub fn member_access(cx: &mut Check, base: Expr, arrow: bool, id: &Node, span: Span) -> Expr {
    let obj = if arrow {
        let b = crate::foundations::values::check::rvalue(cx, base);
        match b.ty.pointee() {
            Some(p) if p.is_class() => crate::expressions::unary::check::indirection(cx, b, span),
            _ => {
                cx.error(span, "PFX-11", format!("'->' on {}, which is not a pointer to a class", b.ty.show()));
                return bad(span);
            }
        }
    } else {
        deref_ref(base)
    };
    if obj.ty.is_error() {
        return bad(span);
    }
    let Some(c) = obj.ty.class() else {
        cx.error(span, "PFX-11", format!("member access on {}, which is not a class", obj.ty.show()));
        return bad(span);
    };
    // PFX-11: inside its own definition, the class's earlier members.
    let defining = { let mut s = Some(cx.scope); let mut found = false; while let Some(x) = s { if x == cx.u.classes[c].scope { found = true; break; } s = cx.u.scopes[x].parent; } found };
    if !cx.u.classes[c].complete && !defining {
        cx.error(span, "PFX-11", "member access on an object of an incomplete class");
        return bad(span);
    }
    let name = match first_node(id) {
        Some(u) if u.prod == "unqualified_id" && u.kids.len() == 1 => cx.source(u),
        _ => return cx_unsupported(cx, span, "a member name of this form"),
    };
    let found = match crate::classes::derived::check::member_lookup(cx, c, &name) {
        Ok(Some(x)) => x.ents,
        Ok(None) => Vec::new(),
        Err(()) => {
            cx.error(span, "DER-4", format!("the member '{}' is in more than one base", name));
            return bad(span);
        }
    };
    let Some(&f) = found.first() else {
        cx.error(span, "PFX-11", format!("no member named '{}'", name));
        return bad(span);
    };
    match cx.u.entities[f].kind.clone() {
        EntKind::Field(_) => field_of(cx, obj, f, span),
        // A static data member: the object is not read.
        EntKind::Var(_) => {
            if !crate::classes::derived::check::accessible_through(cx, f, c) {
                cx.error(span, "ACC-2", format!("the member '{}' is not public", name));
                return bad(span);
            }
            let v = crate::expressions::primary::check::entity_expr(cx, &[f], &name, span);
            // PFX-10: the object expression is evaluated.
            if obj.any(&|x| matches!(x.k, EK::Call(..) | EK::Assign(..) | EK::Compound(..) | EK::IncDec(..))) {
                let (ty, cat) = (v.ty.clone(), v.cat);
                return Expr::new(EK::Comma(Box::new(obj), Box::new(v)), ty, cat, span);
            }
            v
        }
        _ => cx_unsupported(cx, span, "a member function named outside a call, or a member of this kind"),
    }
}

/// The access of a member from the members of another class: public.
fn accessible_from(cx: &Check, f: EntityId, _from: ClassId) -> bool {
    cx.u.access.get(&f).copied().unwrap_or(Access::Public) == Access::Public
}

/// The access of a member (ACC-2): public, or named inside a member
/// function of its class.
pub fn accessible(cx: &Check, f: EntityId) -> bool {
    let access = match &cx.u.entities[f].kind {
        EntKind::Field(x) => x.access,
        _ => cx.u.access.get(&f).copied().unwrap_or(Access::Public),
    };
    if access == Access::Public {
        return true;
    }
    let home = cx.u.entities[f].scope;
    let inside = cx.func.as_ref().and_then(|x| x.id).and_then(|id| cx.u.func(id)).and_then(|x| x.class).is_some_and(|c| cx.u.classes[c].scope == home);
    // ACC-5: a friend of the class.
    if cx.u.class_of_scope(home).is_some_and(|c| crate::classes::access::check::befriended(cx, c)) {
        return true;
    }
    // A declaration in the class, or in a class nested in it (ACC-19).
    let mut s = Some(cx.scope);
    while let Some(x) = s {
        if x == home {
            return true;
        }
        s = cx.u.scopes[x].parent;
    }
    inside
}

/// The data member `f` of the object `obj` (PFX-10), with the access of
/// ACC-2.
pub fn field_of(cx: &mut Check, obj: Expr, f: EntityId, span: Span) -> Expr {
    let Some(field) = cx.u.field(f).cloned() else { return bad(span) };
    let naming = obj.ty.class().unwrap_or(field.class);
    let ok = if naming == field.class { accessible(cx, f) } else { crate::classes::derived::check::accessible_through(cx, f, naming) };
    if !ok {
        cx.error(span, "ACC-2", format!("the member '{}' is not public", cx.u.entities[f].name));
        return bad(span);
    }
    let obj = if obj.cat == Cat::PR {
        let t = obj.ty.clone();
        obj.conv(Conv::Materialize, t, Cat::X)
    } else {
        obj
    };
    // UNI-7: a member of an anonymous union member is a member of that
    // union.
    if naming != field.class {
        if let Some(k) = crate::classes::unions::check::holder(&cx.u, naming, field.class) {
            let ut = QType::new(Type::Class(field.class)).with_cv(obj.ty.c, obj.ty.v);
            let cat = obj.cat;
            let inner = Expr::new(EK::Member(Box::new(obj), k), ut, cat, span);
            return field_of(cx, inner, f, span);
        }
    }
    // DER-3: a member of a base is a member of the base subobject.
    let obj = if naming != field.class {
        let ps = crate::classes::derived::check::paths(cx, naming, field.class);
        if ps.len() != 1 {
            cx.error(span, "DER-4", format!("the member '{}' is in more than one base subobject", cx.u.entities[f].name));
            return bad(span);
        }
        crate::classes::derived::check::along(cx, obj, naming, &ps[0])
    } else {
        obj
    };
    // A reference member holds the place it refers to; naming it names
    // that place, an lvalue (REF rules).
    if let Type::Ref(inner, _) = &field.ty.t {
        let pt = QType::ptr_to((**inner).clone());
        let slot = Expr::new(EK::Member(Box::new(obj), field.index), pt.clone(), Cat::L, span);
        let p = Expr::new(EK::Conv(Conv::LToR, Box::new(slot)), pt, Cat::PR, span);
        return Expr::new(EK::Deref(Box::new(p)), (**inner).clone(), Cat::L, span);
    }
    let c_ = (obj.ty.c && !field.mutable) || field.ty.c;
    let v_ = obj.ty.v || field.ty.v;
    let ty = field.ty.clone().with_cv(c_, v_);
    let cat = obj.cat;
    Expr::new(EK::Member(Box::new(obj), field.index), ty, cat, span)
}

/// A reference member bound by a mem-init or a default member
/// initializer (CTOR-17): the binding of `e`, which must not be to a
/// temporary.
pub fn bind_member(cx: &mut Check, e: Expr, ty: &QType, span: Span) -> Option<Init> {
    let b = crate::declarations::initialization::references::check::bind(cx, e, ty, "INIT-7")?;
    if binds_temporary(&b) {
        cx.error(span, "CTOR-17", "a reference member bound to a temporary, which ends with the constructor");
        return None;
    }
    Some(Init::Ref(b))
}

/// Whether a reference binding makes a temporary.
pub fn binds_temporary(e: &Expr) -> bool {
    match &e.k {
        EK::Conv(Conv::Materialize, _) => true,
        EK::Conv(Conv::NoOp, x) | EK::BasePtr(x, _) => binds_temporary(x),
        _ => false,
    }
}

fn cx_unsupported(cx: &mut Check, span: Span, why: &str) -> Expr {
    let t = cx.unsupported(span, why.to_string());
    Expr::new(EK::Int(0), t, Cat::PR, span)
}

/// The name of a class or enumeration type, for its entity.
fn type_name(cx: &Check, t: &QType) -> String {
    match &t.t {
        Type::Class(c) => cx.u.classes[*c].name.to_string(),
        Type::Enum(e) => cx.u.enums[*e].name.to_string(),
        _ => String::new(),
    }
}
