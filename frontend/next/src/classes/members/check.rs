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
    cx.u.classes.push(ClassInfo { name: name.into(), key, scope, fields: Vec::new(), complete: false });
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
    let Some(head) = n.child("class_head") else { return QType::error() };
    if head.child("base_clause").is_some() || head.child("nested_name").is_some() || head.child("template_id").is_some() || cx.has_word(head, "final") {
        return cx.unsupported(head.span, "a class with a base, a qualified name, or 'final' (build step 4)");
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
    cx.u.classes[id].key = key;
    let saved = cx.scope;
    cx.scope = cx.u.classes[id].scope;
    let mut access = if key == ClassKey::Class { Access::Private } else { Access::Public };
    let mut inits: Vec<(EntityId, &Node)> = Vec::new();
    let mut ok = true;
    for m in n.children("member_decl") {
        if !member(cx, id, m, &mut access, &mut inits) {
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
    // CLS-2: a default member initializer sees the complete class.
    if ok {
        for (f, init) in inits {
            let Some(ty) = cx.u.field(f).map(|x| x.ty.clone()) else { continue };
            let i = member_init(cx, &ty, init);
            if let EntKind::Field(x) = &mut cx.u.entities[f].kind {
                x.init = i;
            }
        }
    }
    cx.scope = saved;
    QType::new(Type::Class(id))
}

/// One `member_decl`. False after an unsupported member.
fn member<'n>(cx: &mut Check, id: ClassId, m: &'n Node, access: &mut Access, inits: &mut Vec<(EntityId, &'n Node)>) -> bool {
    let Some(first) = first_node(m) else { return true };
    match first.prod {
        "access_label" => {
            *access = match first.find("access_specifier").map(|a| cx.first_word(a)) {
                Some("private") => Access::Private,
                Some("protected") => Access::Protected,
                _ => Access::Public,
            };
            true
        }
        "decl_specifiers" => {
            if m.kids.iter().any(|k| matches!(k, Kid::Node(x) if !matches!(x.prod, "decl_specifiers" | "member_declarator"))) {
                cx.unsupported(m.span, "a bit-field or a member of this form (build step 4)");
                return false;
            }
            let specs = crate::declarations::check::decl_specifiers(cx, first, Default::default());
            if specs.unsupported {
                return false;
            }
            if specs.is_static || specs.typedef || specs.is_extern || specs.thread_local || specs.constexpr || specs.inline {
                cx.unsupported(m.span, "a member of this kind (static, typedef, or inline; build step 4)");
                return false;
            }
            // CLS-9: no placeholder, and no `mutable` with `const`.
            if specs.placeholder.is_some() {
                cx.error(m.span, "CLS-9", "a data member declared with a placeholder type");
                return true;
            }
            if specs.mutable && specs.ty.as_ref().is_some_and(|t| t.c) {
                cx.error_cause(m.span, "CLS-9", Some("STO-6"), "a mutable data member that is const");
                return true;
            }
            if specs.is_static || specs.typedef || specs.is_extern || specs.thread_local || specs.constexpr || specs.inline || specs.defines_type && m.children("member_declarator").next().is_none() {
                cx.unsupported(m.span, "a member of this kind (static, typedef, or a nested type; build step 4)");
                return false;
            }
            for md in m.children("member_declarator") {
                let Some(dn) = md.child("declarator") else {
                    cx.unsupported(md.span, "a bit-field (build step 4)");
                    return false;
                };
                if md.kids.len() > 1 && md.child("member_init").is_none() {
                    cx.unsupported(md.span, "a member declarator of this form (build step 4)");
                    return false;
                }
                // A member function's declarator has rules of its own
                // (DCL-21 holds only outside a class).
                let marks = cx.u.diags.len();
                let d = crate::declarations::declarators::check::declarator(cx, &specs, dn);
                if d.ty.is_func() {
                    cx.u.diags.truncate(marks);
                    cx.unsupported(dn.span, "a member function (build step 4)");
                    return false;
                }
                if d.unsupported || d.ty.is_error() {
                    if !d.unsupported {
                        return true;
                    }
                    return false;
                }
                let Some((name, span)) = d.name.clone() else { continue };
                if d.ty.is_func() {
                    cx.unsupported(span, "a member function (build step 4)");
                    return false;
                }
                if d.ty.is_ref() {
                    cx.unsupported(span, "a reference data member (build step 4)");
                    return false;
                }
                // CLS-1: a data member has a complete type.
                if d.ty.is_incomplete() || d.ty.class().is_some_and(|c| !cx.u.classes[c].complete) {
                    cx.error(span, "CLS-1", format!("the data member '{}' has an incomplete type", name));
                    return true;
                }
                let index = cx.u.classes[id].fields.len();
                let f = Field { ty: d.ty.clone(), class: id, index, init: None, mutable: specs.mutable, access: *access };
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
        _ => {
            cx.unsupported(m.span, format!("a member of the form {} (build step 4)", first.prod));
            false
        }
    }
}

/// A default member initializer (`member_init`) for a member of type
/// `ty`: `= expr`, `= {...}`, or `{...}`.
fn member_init(cx: &mut Check, ty: &QType, n: &Node) -> Option<Init> {
    let c = first_node(n)?;
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

/// A class whose default-initialization initializes each member: a
/// default member initializer for each, or a member of such a class
/// (CTOR-3, const-default-constructible).
fn const_default_ok(cx: &Check, c: ClassId) -> bool {
    cx.u.classes[c].fields.iter().all(|&f| {
        let x = cx.u.field(f).unwrap();
        x.init.is_some() || x.ty.base_elem().class().is_some_and(|d| const_default_ok(cx, d))
    })
}

/// A class with a const data member, also through a member of class
/// type, whose implicit copy assignment is deleted (CPY-7).
pub fn has_const_member(cx: &Check, c: ClassId) -> bool {
    cx.u.field_types(c).iter().any(|t| t.base_elem().c || t.base_elem().class().is_some_and(|d| has_const_member(cx, d)))
}

/// CTOR-3, CTOR-20: default-initialization of an object of type `ty`
/// uses the implicit default constructor, which a const member without a
/// default member initializer deletes. True when the use is valid.
pub fn check_default_init(cx: &mut Check, ty: &QType, span: Span) -> bool {
    let Some(c) = ty.base_elem().class() else { return true };
    for f in cx.u.classes[c].fields.clone() {
        let x = cx.u.field(f).unwrap().clone();
        if x.init.is_some() {
            continue;
        }
        let base = x.ty.base_elem();
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

/// A class is an aggregate when every data member is public: build step
/// 4 part a has no constructors, bases, or virtual functions (LIST
/// rules).
pub fn is_aggregate(cx: &Check, c: ClassId) -> bool {
    cx.u.classes[c].fields.iter().all(|&f| cx.u.field(f).is_some_and(|x| x.access == Access::Public))
}

/// Default-initialization of an object of type `ty`: each data member
/// gets its default member initializer or its own default-initialization
/// (INIT rules).
pub fn default_init(cx: &Check, ty: &QType) -> Init {
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
    let scope = cx.u.classes[c].scope;
    let found = names::declared_here(cx, scope, &name);
    let Some(&f) = found.first() else {
        cx.error(span, "PFX-11", format!("no member named '{}'", name));
        return bad(span);
    };
    let Some(field) = cx.u.field(f).cloned() else {
        return cx_unsupported(cx, span, "a member that is not a data member");
    };
    // ACC-2: outside the class, only a public member can be named.
    if field.access != Access::Public {
        cx.error(span, "ACC-2", format!("the member '{}' is not public", name));
        return bad(span);
    }
    let obj = if obj.cat == Cat::PR {
        let t = obj.ty.clone();
        obj.conv(Conv::Materialize, t, Cat::X)
    } else {
        obj
    };
    let c_ = (obj.ty.c && !field.mutable) || field.ty.c;
    let v_ = obj.ty.v || field.ty.v;
    let ty = field.ty.clone().with_cv(c_, v_);
    let cat = obj.cat;
    Expr::new(EK::Member(Box::new(obj), field.index), ty, cat, span)
}

fn cx_unsupported(cx: &mut Check, span: Span, why: &str) -> Expr {
    let t = cx.unsupported(span, why.to_string());
    Expr::new(EK::Int(0), t, Cat::PR, span)
}
