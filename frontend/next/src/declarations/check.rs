//! The analysis of the declarations chapter's own productions
//! (`declarations/index`): the translation unit, a declaration, a
//! simple declaration, and its decl-specifiers. The index page has no
//! rules; each rule sits in the page that states it, and this module
//! calls that page's function.

use crate::declarations::{
    attributes, auto, constexpr, declarators, enumerations, functions, initialization, linkage_specifications,
    namespaces, static_assert, storage_linkage, type_names,
};
use crate::foundations::types;
use crate::sem::check::Check;
use crate::sem::tree::{Init, Stmt, SK};
use crate::sem::types::QType;
use crate::sem::EntityId;
use crate::source::Span;
use crate::syntax::{Kid, Node};

/// What the decl-specifiers of a declaration say.
#[derive(Clone, Debug, Default)]
pub struct Specs {
    /// The type the type specifiers give, before the declarator.
    pub ty: Option<QType>,
    /// `auto` or `decltype(auto)` (`declarations/auto`).
    pub placeholder: Option<auto::check::Placeholder>,
    pub typedef: bool,
    pub is_static: bool,
    pub is_extern: bool,
    pub thread_local: bool,
    pub mutable: bool,
    pub register: bool,
    pub constexpr: bool,
    /// `explicit` on a constructor (CTOR-9).
    pub explicit: bool,
    /// `virtual` on a member function (`classes/virtual`).
    pub is_virtual: bool,
    pub consteval: bool,
    pub constinit: bool,
    pub inline: bool,
    /// The attributes before the declaration and after the specifiers.
    pub attrs: attributes::check::Attrs,
    pub span: Span,
    /// The specifiers define an enumeration.
    pub defines_type: bool,
    /// A specifier that build step 3 does not model.
    pub unsupported: bool,
}

/// Check a translation unit's declarations in order.
pub fn translation_unit(cx: &mut Check, trees: &[Node]) {
    for t in trees {
        declaration(cx, t);
    }
    functions::definitions::check::end_of_unit(cx);
}

/// `declaration`: the kinds of declarations at namespace scope.
pub fn declaration(cx: &mut Check, n: &Node) {
    for k in &n.kids {
        let Kid::Node(c) = k else { continue };
        match c.prod {
            "block_decl" => {
                block_decl(cx, c, false);
            }
            "function_definition" => functions::definitions::check::function_definition(cx, c),
            "namespace_def" => namespaces::check::namespace_def(cx, c),
            "linkage_spec" => linkage_specifications::check::linkage_spec(cx, c),
            "attribute_spec" => {}
            "empty_decl" | "null_stmt" => {}
            _ => {
                cx.unsupported(c.span, format!("a declaration of the form {}", c.prod));
            }
        }
    }
    if n.kids.iter().all(|k| matches!(k, Kid::Tok(_))) {
        // `;` alone: an empty declaration.
    }
}

/// `block_decl` at namespace scope (`local` false) or in a block; in a
/// block it gives the statement that runs the declaration.
pub fn block_decl(cx: &mut Check, n: &Node, local: bool) -> Option<Stmt> {
    let c = n.kids.iter().find_map(|k| match k {
        Kid::Node(c) => Some(c),
        _ => None,
    })?;
    match c.prod {
        "simple_decl" => simple_decl(cx, c),
        "static_assert_decl" => {
            static_assert::check::static_assert_decl(cx, c);
            None
        }
        "alias_decl" => {
            type_names::check::alias_decl(cx, c);
            None
        }
        "using_directive" => {
            namespaces::check::using_directive(cx, c);
            None
        }
        "namespace_alias" => {
            namespaces::check::namespace_alias(cx, c);
            None
        }
        "using_decl" => {
            namespaces::check::using_decl(cx, c);
            None
        }
        "using_enum_decl" => {
            enumerations::check::using_enum_decl(cx, c);
            None
        }
        "enum_opaque" => {
            enumerations::check::enum_opaque(cx, c);
            None
        }
        "sb_decl" => crate::declarations::structured_bindings::check::sb_decl(cx, c),
        _ => {
            cx.unsupported(c.span, format!("a declaration of the form {}", c.prod));
            None
        }
    }
}

/// `decl_specifiers`: the type and the other specifiers.
pub fn decl_specifiers(cx: &mut Check, n: &Node, attrs: attributes::check::Attrs) -> Specs {
    let mut s = Specs { span: n.span, attrs, ..Default::default() };
    let mut type_specs: Vec<&Node> = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for k in &n.kids {
        let Kid::Node(d) = k else { continue };
        if d.prod == "attribute_spec" {
            // An attribute among the specifiers applies to the type.
            let mut ta = attributes::check::Attrs::default();
            attributes::check::attribute_spec(cx, d, &mut ta);
            attributes::check::on_type(cx, &ta);
            continue;
        }
        let word = cx.first_word(d);
        match (word, d.kids.first()) {
            ("typedef", _) => s.typedef = true,
            ("constexpr", _) => s.constexpr = true,
            ("consteval", _) => s.consteval = true,
            ("constinit", _) => s.constinit = true,
            ("inline", _) => s.inline = true,
            ("explicit", _) if cx.member_of.is_some() && !cx.has_word(d, "(") => s.explicit = true,
            // VIRT-2: `virtual` only in a class definition.
            ("virtual", _) if cx.member_of.is_some() => s.is_virtual = true,
            ("virtual", _) => {
                cx.error(d.span, "VIRT-2", "'virtual' outside a class definition");
            }
            ("friend" | "explicit", _) => {
                s.unsupported = true;
                cx.unsupported(d.span, format!("the specifier '{}'", word));
            }
            _ => {
                if let Some(Kid::Node(x)) = d.kids.first() {
                    match x.prod {
                        // CTOR-9: `explicit` on a constructor.
                        "explicit_spec" if cx.member_of.is_some() && !cx.has_word(x, "(") => s.explicit = true,
                        "storage_class" if cx.first_word(x) == "register" => s.register = true,
                        "storage_class" => {
                            let w = cx.first_word(x);
                            if seen.contains(&w) || (!seen.is_empty() && !(seen == ["thread_local"] || w == "thread_local")) {
                                storage_linkage::check::two_storage_classes(cx, x.span, &seen, w);
                            }
                            seen.push(w);
                            match w {
                                "static" => s.is_static = true,
                                "extern" => s.is_extern = true,
                                "thread_local" => s.thread_local = true,
                                "mutable" => s.mutable = true,
                                _ => {}
                            }
                        }
                        "type_specifier" => type_specs.push(x),
                        _ => {
                            s.unsupported = true;
                            cx.unsupported(x.span, format!("a decl-specifier of the form {}", x.prod));
                        }
                    }
                }
            }
        }
    }
    if s.register {
        storage_linkage::check::register(cx, n.span);
    }
    if s.typedef && (s.is_static || s.is_extern || s.thread_local) {
        storage_linkage::check::typedef_with_storage(cx, n.span);
    }
    // The declaration's own specifiers, and not those of its parameters.
    if cx.implicit_extern {
        s.is_extern = true;
        cx.implicit_extern = false;
    }
    constexpr::check::specifier_pairs(cx, &s);
    let (ty, ph, defines, unsup) = types::check::type_specifiers(cx, &type_specs, n.span);
    s.ty = ty;
    s.placeholder = ph;
    s.defines_type = defines;
    s.unsupported |= unsup;
    s
}

/// The attributes and alignment specifiers before a declaration.
pub fn leading_attrs(cx: &mut Check, n: &Node) -> (attributes::check::Attrs, Vec<Node>) {
    let mut a = attributes::check::Attrs::default();
    let mut aligns = Vec::new();
    for k in &n.kids {
        if let Kid::Node(c) = k {
            match c.prod {
                "attribute_spec" => attributes::check::attribute_spec(cx, c, &mut a),
                "alignment_specifier" => aligns.push(c.clone()),
                _ => {}
            }
        }
    }
    (a, aligns)
}

/// `simple_decl`: each declarator declares a variable, a function, or a
/// type alias. In a block, the result is the statement that initializes
/// the variables.
pub fn simple_decl(cx: &mut Check, n: &Node) -> Option<Stmt> {
    let (attrs, aligns) = leading_attrs(cx, n);
    let Some(ds) = n.child("decl_specifiers") else {
        cx.unsupported(n.span, "a declaration without decl-specifiers");
        return None;
    };
    // UNI-7: an anonymous union, whose members belong to the enclosing
    // scope.
    if n.child("init_declarator").is_none() && crate::classes::unions::check::own_union(ds).is_some_and(|u| u.child("class_name").is_none() && u.child("template_id").is_none()) {
        return anonymous_union(cx, n, ds, attrs);
    }
    let specs = decl_specifiers(cx, ds, attrs);
    let local = cx.in_function();
    let mut vars: Vec<(EntityId, Init)> = Vec::new();
    let decls: Vec<&Node> = n.children("init_declarator").collect();
    if decls.is_empty() {
        // CX-5, INL-2, STO-6: a declaration of a class alone takes no
        // such specifier.
        if specs.defines_type && specs.ty.as_ref().is_some_and(|t| t.is_class()) {
            if specs.constexpr {
                cx.error(n.span, "CX-5", "constexpr on the definition of a class");
            } else if specs.inline {
                cx.error(n.span, "INL-2", "inline on the definition of a class");
            } else if specs.is_static || specs.is_extern || specs.thread_local || specs.mutable {
                cx.error(n.span, "STO-6", "a storage class on the definition of a class");
            }
        }
        if specs.typedef && !specs.defines_type && !specs.unsupported {
            cx.error(n.span, "TYN-2", "a typedef declaration declares no name");
        } else if !specs.defines_type && !specs.unsupported {
            types::check::declares_nothing(cx, n.span);
        }
        return local.then(|| Stmt::new(SK::Null, n.span));
    }
    let mut first_auto: Option<QType> = None;
    for d in decls {
        let Some(dn) = d.child("declarator") else { continue };
        let declared = declarators::check::declarator(cx, &specs, dn);
        if specs.unsupported || declared.unsupported {
            if let Some((name, sp)) = &declared.name {
                crate::foundations::names::check::declare_unsupported(cx, name, *sp);
            }
            continue;
        }
        // ALN-2: alignas applies to a variable or a non-bit-field data
        // member, not to a typedef or a function.
        if !aligns.is_empty() && (specs.typedef || declared.ty.is_func()) {
            cx.error(d.span, "ALN-2", "alignas on a typedef or a function");
            continue;
        }
        if specs.typedef {
            // TYN-9: a class that a typedef names for linkage has no
            // default member initializer.
            if let Some(c) = declared.ty.class().filter(|&c| cx.u.classes[c].name.is_empty()) {
                let scope = cx.u.classes[c].scope;
                let functions = cx.u.scopes[scope].names.values().flatten().any(|&e| matches!(cx.u.entities[e].kind, crate::sem::EntKind::Func(_)));
                // A lambda expression anywhere in the class.
                let lambda = cx.u.closures.keys().any(|&k| {
                    let mut s = cx.u.scopes[cx.u.classes[k].scope].parent;
                    while let Some(x) = s {
                        if x == scope {
                            return true;
                        }
                        s = cx.u.scopes[x].parent;
                    }
                    false
                });
                let aliases = cx.u.scopes[scope].names.values().flatten().any(|&e| matches!(cx.u.entities[e].kind, crate::sem::EntKind::Typedef(_)));
                if functions || lambda || aliases || !cx.u.bases(c).is_empty() || cx.u.classes[c].fields.iter().any(|&f| cx.u.field(f).is_some_and(|x| x.init.is_some())) {
                    cx.error(d.span, "TYN-9", "a class named by a typedef for linkage has a member function, a default member initializer, or a lambda");
                    continue;
                }
            }
            if specs.inline {
                crate::declarations::inline::check::misplaced(cx, d.span, "on a typedef");
            }
            if d.child("init").is_some() {
                type_names::check::typedef_with_init(cx, d.span);
            }
            type_names::check::typedef_name(cx, &declared);
            continue;
        }
        if declarators::check::qualified_func(&declared.ty) {
            cx.error(declared.span, "DCL-21", "a function that is not a member has a cv-qualifier or a ref-qualifier");
            continue;
        }
        if declared.ty.is_func() {
            functions::definitions::check::declare_function(cx, &specs, &declared, None);
            continue;
        }
        let init = d.child("init");
        if let Some((id, i)) = initialization::forms::check::declare_variable(cx, &specs, &declared, init, &aligns, &mut first_auto) {
            if local {
                let automatic = cx.u.var(id).is_some_and(|v| v.storage == crate::sem::Storage::Automatic);
                let ty = cx.u.var(id).map(|v| v.ty.clone()).unwrap_or_else(QType::error);
                let trivial = crate::classes::members::check::trivial_default(&i) && !crate::classes::members::check::has_polymorphic_part(cx, &ty);
                if automatic && !matches!(i, Init::Default) && !trivial {
                    if let Some(f) = cx.func.as_mut() {
                        f.live.push(id);
                    }
                }
                vars.push((id, i));
            }
        }
    }
    if local {
        Some(Stmt::new(SK::Decl(vars), n.span))
    } else {
        None
    }
}

/// UNI-7, UNI-9: an anonymous union of a block or a namespace: an unnamed
/// variable of the union, whose members the scope names. At namespace
/// scope outside an unnamed namespace, the union is static (STO-11).
fn anonymous_union(cx: &mut Check, n: &Node, ds: &Node, attrs: crate::declarations::attributes::check::Attrs) -> Option<Stmt> {
    let specs = decl_specifiers(cx, ds, attrs);
    if specs.unsupported {
        return None;
    }
    let u = specs.ty.as_ref().and_then(|t| t.class())?;
    let local = cx.in_function();
    let ns = cx.u.scopes[cx.scope].kind == crate::sem::ScopeKind::Namespace;
    let unnamed = ns && cx.scope != cx.u.root && cx.u.scopes[cx.scope].name.is_empty();
    if ns && !specs.is_static && !unnamed {
        cx.error_cause(n.span, "UNI-9", Some("STO-11"), "an anonymous union at namespace scope that is not static");
        return None;
    }
    if specs.is_extern || specs.thread_local || specs.constexpr || specs.constinit || specs.typedef {
        cx.unsupported(n.span, "an anonymous union with this specifier");
        return None;
    }
    if !crate::classes::unions::check::anonymous(cx, u, n.span) {
        return None;
    }
    let storage = if local && !specs.is_static { crate::sem::Storage::Automatic } else { crate::sem::Storage::Static };
    let ty = QType::new(crate::sem::types::Type::Class(u));
    let init = crate::classes::members::check::default_init(cx, &ty);
    let v = crate::sem::Var { ty, storage, constexpr: false, constinit: false, inline: false, is_extern: false, is_static: specs.is_static, param: false, defined: true, init: Some(init.clone()), const_init: false, used: false, maybe_unused: false, align: None };
    let scope = cx.scope;
    let id = cx.u.add(crate::sem::Entity { name: "".into(), kind: crate::sem::EntKind::Var(v), span: n.span, scope, linkage: crate::sem::Linkage::None, deprecated: None });
    cx.u.anon_objects.insert(u, id);
    if !local {
        cx.u.statics.push(id);
        return None;
    }
    Some(Stmt::new(SK::Decl(vec![(id, init)]), n.span))
}
