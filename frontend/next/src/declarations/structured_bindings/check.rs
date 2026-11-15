//! The check part of `declarations/structured-bindings`, for an array
//! and for a class with public data members (SB-1, SB-2, SB-3, SB-4,
//! SB-7, SB-8, SB-12, SB-13).

use crate::expressions::check::{expr, first_node};
use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Conv, Expr, Init, Stmt, EK, SK};
use crate::sem::types::{QType, Type};
use crate::sem::{EntKind, EntityId, Linkage, Storage, Var};
use crate::source::Span;
use crate::syntax::{Kid, Node};
use std::rc::Rc;

fn var(ty: QType, storage: Storage, init: Option<Init>) -> Var {
    Var {
        ty,
        storage,
        constexpr: false,
        constinit: false,
        inline: false,
        is_extern: false,
        is_static: storage != Storage::Automatic,
        param: false,
        defined: true,
        init,
        const_init: false,
        used: true,
        maybe_unused: true,
        align: None,
    }
}

/// `sb_decl`: a hidden variable from the initializer, and one name for
/// each element of it. The statement declares the hidden variable and
/// then the names, or None at namespace scope.
pub fn sb_decl(cx: &mut Check, n: &Node) -> Option<Stmt> {
    let local = cx.in_function();
    let span = n.span;
    let words = cx.words(n);
    let head: Vec<&str> = words.iter().copied().take_while(|w| *w != "[").collect();
    let cvs: Vec<&str> = n.children("cv").map(|x| cx.first_word(x)).collect();
    let (c, v) = (head.contains(&"const") || cvs.contains(&"const"), head.contains(&"volatile") || cvs.contains(&"volatile"));
    if v {
        crate::declarations::volatile::check::structured_binding(cx, span);
    }
    let reference = head.iter().find(|w| matches!(**w, "&" | "&&")).copied();
    let storage = if head.contains(&"thread_local") {
        Storage::Thread
    } else if head.contains(&"static") || !local {
        Storage::Static
    } else {
        Storage::Automatic
    };
    let ids: Vec<(Rc<str>, Span)> = n
        .kids
        .iter()
        .skip_while(|k| !matches!(k, Kid::Tok(_)) || cx.text(k) != "[")
        .skip(1)
        .take_while(|k| !matches!(k, Kid::Tok(_)) || cx.text(k) != "]")
        .filter_map(|k| match k {
            Kid::Node(x) => Some((Rc::from(cx.source(x).as_str()), x.span)),
            Kid::Tok(_) if cx.text(k) != "," => Some((Rc::from(cx.text(k)), span)),
            _ => None,
        })
        .collect();
    // SB-8: the names differ, `_` included, and none is declared in the
    // scope already or occurs in the initializer.
    for (k, (name, sp)) in ids.iter().enumerate() {
        if ids[..k].iter().any(|(m, _)| m == name) {
            cx.error(*sp, "SB-8", format!("the name '{}' occurs twice in one structured binding declaration", name));
            return failed(local, span);
        }
        if !names::declared_here(cx, cx.scope, name).is_empty() {
            cx.error(*sp, "SB-8", format!("'{}' is already declared in this scope", name));
            return failed(local, span);
        }
    }
    let init = n.child("sb_init")?;
    let mut toks = Vec::new();
    init.tokens(&mut toks);
    if let Some((name, _)) = ids.iter().find(|(name, _)| toks.iter().any(|&t| cx.toks[t].text == **name)) {
        cx.error(init.span, "SB-8", format!("the name '{}' occurs in the initializer of its own declaration", name));
        return failed(local, span);
    }
    let e = expr(cx, first_node(init)?);
    if e.ty.is_error() {
        return failed(local, span);
    }
    let et = e.ty.nonref();
    let Type::Array(el, Some(bound)) = et.t.clone() else {
        if et.is_scalar() {
            cx.error(e.span, "SB-13", format!("a structured binding of the scalar type {}", et.show()));
            return failed(local, span);
        }
        if let Some(k) = et.class() {
            return class_bindings(cx, e, k, &ids, (c, v), reference, storage, local, span, init.span);
        }
        cx.unsupported(e.span, "a structured binding of this type");
        return failed(local, span);
    };
    // SB-2: one name for each element.
    if bound != ids.len() as u64 {
        cx.error(span, "SB-2", format!("{} names for an array of {} elements", ids.len(), bound));
        return failed(local, span);
    }
    // SB-1: the hidden variable has the written cv-qualifiers and
    // reference. Without a reference it is a copy of the array.
    let hidden_ty = QType { t: Type::Array(el.clone(), Some(bound)), c: false, v: false }.with_cv(c, v);
    let (hidden_ty, hidden_init) = match reference {
        None => {
            let copy = Expr::new(EK::Conv(Conv::LToR, Box::new(e)), et.unqual(), Cat::PR, init.span);
            (hidden_ty, Init::Expr(copy))
        }
        Some(r) => {
            // `auto&&` is a forwarding reference: an lvalue gives an
            // lvalue reference (AUTO-4).
            let rv = r == "&&" && e.cat != Cat::L;
            let rt = QType::new(Type::Ref(Rc::new(hidden_ty.clone()), rv));
            let x = crate::declarations::initialization::references::check::bind(cx, e, &rt, "INIT-7")?;
            (rt, Init::Ref(x))
        }
    };
    let elem = (*el).clone().with_cv(el.c || c, el.v || v);
    let scope = cx.scope;
    let hidden = cx.u.add(crate::sem::Entity { name: "".into(), kind: EntKind::Var(var(hidden_ty.clone(), storage, Some(hidden_init.clone()))), span, scope, linkage: Linkage::None, deprecated: None });
    let mut decls = vec![(hidden, hidden_init)];
    let size_t = QType::new(Type::Int(cx.u.target.size_t()));
    for (k, (name, sp)) in ids.iter().enumerate() {
        // SB-12: each name is an lvalue of an element of the hidden
        // variable.
        let base = Expr::new(EK::Var(hidden), hidden_ty.nonref(), Cat::L, *sp);
        let ptr = base.conv(Conv::ArrayDecay, QType::ptr_to(elem.clone()), Cat::PR);
        let at = Expr::new(EK::Index(Box::new(ptr), Box::new(Expr::int(k as i128, size_t.clone(), *sp))), elem.clone(), Cat::L, *sp);
        let i = Init::Ref(at);
        let id = names::declare_in(cx, scope, name, EntKind::Var(Var { used: false, ..var(QType::lref(elem.clone()), storage, Some(i.clone())) }), *sp, Linkage::None);
        cx.bindings.insert(id);
        decls.push((id, i));
    }
    if !local {
        cx.u.statics.extend(decls.iter().map(|(id, _)| *id));
        return None;
    }
    if storage != Storage::Automatic {
        return Some(Stmt::new(SK::Decl(decls), span));
    }
    if let Some(f) = cx.func.as_mut() {
        f.live.extend(decls.iter().map(|(id, _)| *id));
    }
    Some(Stmt::new(SK::Decl(decls), span))
}

/// SB-3, SB-13: a structured binding of an object of class `k`. The
/// names bind the non-static data members, which one class of `k` and
/// its bases declares, in their order.
#[allow(clippy::too_many_arguments)]
fn class_bindings(cx: &mut Check, e: Expr, k: crate::sem::types::ClassId, ids: &[(Rc<str>, Span)], cv: (bool, bool), reference: Option<&str>, storage: Storage, local: bool, span: Span, init_span: Span) -> Option<Stmt> {
    if crate::classes::unions::check::is_union(cx, k) {
        cx.error(e.span, "SB-13", "a structured binding of a union");
        return failed(local, span);
    }
    if !cx.u.classes[k].complete {
        cx.error(e.span, "SB-13", "a structured binding of an incomplete class");
        return failed(local, span);
    }
    // The classes of the hierarchy that declare data members.
    fn holders(cx: &Check, c: crate::sem::types::ClassId, out: &mut Vec<crate::sem::types::ClassId>) {
        let own = cx.u.classes[c].fields.iter().filter_map(|&f| cx.u.field(f).map(|x| (f, x))).any(|(f, x)| !x.base && !(x.width.is_some() && cx.u.entities[f].name.is_empty()));
        if own && !out.contains(&c) {
            out.push(c);
        }
        for (_, b) in cx.u.bases(c) {
            holders(cx, b, out);
        }
    }
    let mut hs = Vec::new();
    holders(cx, k, &mut hs);
    if hs.len() > 1 {
        cx.error(e.span, "SB-13", "the data members of the class are in more than one class of its hierarchy");
        return failed(local, span);
    }
    let members: Vec<EntityId> = hs.first().map(|&h| cx.u.classes[h].fields.iter().copied().filter(|&f| cx.u.field(f).is_some_and(|x| !x.base && !(x.width.is_some() && cx.u.entities[f].name.is_empty()))).collect()).unwrap_or_default();
    for &f in &members {
        let name = cx.u.entities[f].name.clone();
        if name.is_empty() {
            cx.error(e.span, "SB-13", "a structured binding of a class with an anonymous union member");
            return failed(local, span);
        }
        if !crate::classes::derived::check::accessible_through(cx, f, k) {
            cx.error(e.span, "SB-13", format!("the member '{}' that the structured binding names is not accessible", name));
            return failed(local, span);
        }
    }
    // SB-2: one name for each data member.
    if members.len() != ids.len() {
        cx.error(span, "SB-2", format!("{} names for a class with {} data members", ids.len(), members.len()));
        return failed(local, span);
    }
    // SB-1: the hidden variable has the written cv-qualifiers and
    // reference. Without a reference it is copied from the initializer.
    let et = e.ty.nonref();
    let hidden_ty = et.unqual().with_cv(cv.0, cv.1);
    let (hidden_ty, hidden_init) = match reference {
        None => {
            let x = crate::declarations::initialization::forms::check::copy_init(cx, e, &hidden_ty, "INIT-7")?;
            (hidden_ty, Init::Expr(x))
        }
        Some(r) => {
            let rv = r == "&&" && e.cat != Cat::L;
            let rt = QType::new(Type::Ref(Rc::new(hidden_ty.clone()), rv));
            let x = crate::declarations::initialization::references::check::bind(cx, e, &rt, "INIT-7")?;
            (rt, Init::Ref(x))
        }
    };
    let _ = init_span;
    let scope = cx.scope;
    let hidden = cx.u.add(crate::sem::Entity { name: "".into(), kind: EntKind::Var(var(hidden_ty.clone(), storage, Some(hidden_init.clone()))), span, scope, linkage: Linkage::None, deprecated: None });
    let mut decls = vec![(hidden, hidden_init)];
    for ((name, sp), &f) in ids.iter().zip(members.iter()) {
        // SB-3: each name is an lvalue of a member of the hidden variable.
        let base = Expr::new(EK::Var(hidden), hidden_ty.nonref(), Cat::L, *sp);
        let at = crate::classes::members::check::field_of(cx, base, f, *sp);
        if at.ty.is_error() {
            return failed(local, span);
        }
        let i = Init::Ref(at.clone());
        let id = names::declare_in(cx, scope, name, EntKind::Var(Var { used: false, ..var(QType::lref(at.ty.clone()), storage, Some(i.clone())) }), *sp, Linkage::None);
        cx.bindings.insert(id);
        // SB-4: `decltype` of the binding is the member's declared type.
        if let Some(t) = cx.u.field(f).map(|x| x.ty.clone()).filter(|t| t.is_ref()) {
            cx.binding_decl.insert(id, t);
        }
        // SB-10: a binding to a bit-field names the bit-field.
        if cx.u.field(f).is_some_and(|x| x.width.is_some()) {
            cx.bit_bindings.insert(id, at);
        }
        decls.push((id, i));
    }
    if !local {
        cx.u.statics.extend(decls.iter().map(|(id, _)| *id));
        return None;
    }
    if storage != Storage::Automatic {
        return Some(Stmt::new(SK::Decl(decls), span));
    }
    if let Some(f) = cx.func.as_mut() {
        f.live.extend(decls.iter().map(|(id, _)| *id));
    }
    Some(Stmt::new(SK::Decl(decls), span))
}

fn failed(local: bool, span: Span) -> Option<Stmt> {
    local.then(|| Stmt::new(SK::Null, span))
}

/// The names of structured bindings, whose `decltype` is the element
/// type (SB-4).
pub fn is_binding(cx: &Check, id: EntityId) -> bool {
    cx.bindings.contains(&id)
}
