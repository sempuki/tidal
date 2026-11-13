//! The check part of `declarations/linkage-specifications`.

use crate::sem::check::Check;
use crate::syntax::{Kid, Node};

/// `linkage_spec` (LSP-1, LSP-2, LSP-5): the declarations inside get the
/// language linkage of the string.
pub fn linkage_spec(cx: &mut Check, n: &Node) {
    let lang = n.kids.get(1).and_then(|k| cx.token(k)).and_then(|t| match &t.lit {
        Some(crate::lex::Lit::Str { units, .. }) => Some(units.iter().filter_map(|&u| char::from_u32(u)).collect::<String>()),
        _ => None,
    });
    if !matches!(lang.as_deref(), Some("C") | Some("C++")) {
        // LSP-5: the implementations in the matrix link with no other
        // language.
        cx.error(n.span, "LSP-5", format!("the language linkage \"{}\" is not supported", lang.unwrap_or_default()));
        return;
    }
    let c = lang.as_deref() == Some("C");
    let braces = cx.has_word(n, "{");
    let saved = crate::declarations::functions::definitions::check::set_c_linkage(cx, c);
    for k in &n.kids {
        if let Kid::Node(d) = k {
            if !braces {
                if let Some(sd) = d.find("simple_decl") {
                    if sd.child("decl_specifiers").is_some_and(|ds| cx.source(ds).split(' ').any(|w| w == "static")) {
                        cx.error(d.span, "LSP-2", "the form without braces declares a static entity");
                    }
                }
            }
            let saved = std::mem::replace(&mut cx.implicit_extern, !braces);
            crate::declarations::check::declaration(cx, d);
            cx.implicit_extern = saved;
        }
    }
    crate::declarations::functions::definitions::check::set_c_linkage(cx, saved);
}

/// LSP-4, LSP-7, LSP-8: a C name is one entity across namespaces, and a
/// variable or function of the global namespace with that name is the
/// same entity. Each declaration must agree in kind and type with the
/// others. `c` says that the new declaration has C language linkage.
pub fn c_name(cx: &mut Check, id: crate::sem::EntityId, c: bool, span: crate::source::Span) {
    use crate::sem::EntKind;
    let name = cx.u.entities[id].name.clone();
    let kind_ty = |cx: &Check, e: crate::sem::EntityId| -> Option<(bool, crate::sem::types::QType)> {
        match &cx.u.entities[e].kind {
            EntKind::Var(v) => Some((true, v.ty.clone())),
            EntKind::Func(f) => Some((false, crate::sem::types::QType::new(crate::sem::types::Type::Func(f.ty.clone())))),
            _ => None,
        }
    };
    let Some(mine) = kind_ty(cx, id) else { return };
    // A function of the global namespace is another entity than a C
    // name; a variable of the global namespace is the same entity.
    if !c && !mine.0 {
        return;
    }
    let mut others: Vec<crate::sem::EntityId> = Vec::new();
    if let Some(&e) = cx.c_names.get(&name) {
        others.push(e);
    }
    if c {
        others.extend(crate::foundations::names::check::declared_here(cx, cx.u.root, &name).into_iter().filter(|&e| matches!(cx.u.entities[e].kind, EntKind::Var(_))));
    }
    for e in others {
        if e == id {
            continue;
        }
        let Some(theirs) = kind_ty(cx, e) else { continue };
        if !c && cx.u.entities[e].scope == cx.u.root {
            continue;
        }
        if theirs.0 != mine.0 || theirs.1.t != mine.1.t {
            cx.error(span, "LSP-7", format!("the C name '{}' is declared as another entity", name));
            return;
        }
    }
    // LSP-7: a C name must not have the spelling of a global variable
    // with C++ language linkage, in either order.
    let first_c = cx.c_names.get(&name).copied();
    let clash = if c {
        crate::foundations::names::check::declared_here(cx, cx.u.root, &name).into_iter().any(|e| e != id && Some(e) != first_c && matches!(cx.u.entities[e].kind, EntKind::Var(_)))
    } else {
        mine.0 && cx.u.entities[id].scope == cx.u.root && first_c.is_some_and(|e| e != id)
    };
    if clash {
        cx.error(span, "LSP-7", format!("the C name '{}' has the spelling of a global variable with C++ language linkage", name));
        return;
    }
    if c {
        cx.c_names.entry(name).or_insert(id);
    }
}
