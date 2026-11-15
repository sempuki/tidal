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
    // SB-5: a class that `std::tuple_size` gives a size for binds by the
    // tuple protocol.
    if let Some(r) = tuple_bindings(cx, &e, k, ids, cv, reference, storage, local, span) {
        return r;
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

/// The class template `name` of namespace `std`, if the program declares
/// one.
fn std_template(cx: &Check, name: &str) -> Option<usize> {
    let s = cx.u.scopes[cx.u.root].names.get("std")?.iter().find_map(|&e| match cx.u.entities[e].kind {
        EntKind::Namespace(s) => Some(s),
        _ => None,
    })?;
    cx.u.scopes[s].names.get(name)?.iter().find_map(|&e| match cx.u.entities[e].kind {
        EntKind::ClassTemplate(v) => Some(v),
        _ => None,
    })
}

/// The member `name` of class `c`.
fn member(cx: &Check, c: crate::sem::types::ClassId, name: &str) -> Option<EntityId> {
    names::declared_here(cx, cx.u.classes[c].scope, name).first().copied()
}

/// SB-5, SB-14: the tuple protocol. None when `std::tuple_size<E>` is not
/// complete or has no `value`, so the names refer to data members.
#[allow(clippy::too_many_arguments)]
fn tuple_bindings(cx: &mut Check, e: &Expr, k: crate::sem::types::ClassId, ids: &[(Rc<str>, Span)], cv: (bool, bool), reference: Option<&str>, storage: Storage, local: bool, span: Span) -> Option<Option<Stmt>> {
    use crate::sem::types::TArg;
    let size = std_template(cx, "tuple_size")?;
    // E: the hidden variable's type, with its cv-qualifiers.
    let ety = QType::new(Type::Class(k)).with_cv(cv.0 || e.ty.nonref().c, cv.1 || e.ty.nonref().v);
    let ct = cx.u.class_templates[size].clone();
    let args = vec![TArg::Type(ety.clone())];
    let complete = ct.defined || ct.specs.iter().any(|(a, _)| *a == args) || ct.partials.iter().any(|(ps, _, pat, _)| {
        let mut bound = vec![None; ps.len()];
        matches!(pat.as_slice(), [crate::sem::PatArg::Type(p)] if crate::templates::deduction::check::match_exact(&cx.u, p, &ety, &mut bound))
    });
    if !complete {
        return None;
    }
    let st = crate::templates::declarations::check::class_specialization(cx, size, args, span);
    let Some(sc) = st.class() else {
        return Some(failed(local, span));
    };
    let value = member(cx, sc, "value")?;
    let n = {
        let ty = cx.u.var(value)?.ty.clone();
        let x = Expr::new(EK::Var(value), ty, Cat::L, span);
        let x = crate::foundations::values::check::rvalue(cx, x);
        match crate::foundations::constant_evaluation::check::try_value(cx, &x).and_then(|v| v.int()) {
            Some(n) => n,
            None => {
                cx.unsupported(span, "a tuple_size whose value the front end does not evaluate");
                return Some(failed(local, span));
            }
        }
    };
    // SB-2: one name for each element.
    if n != ids.len() as i128 {
        cx.error(span, "SB-2", format!("{} names for a tuple-like type of {} elements", ids.len(), n));
        return Some(failed(local, span));
    }
    // SB-1: the hidden variable.
    let hidden_ty = e.ty.nonref().unqual().with_cv(cv.0, cv.1);
    let (hidden_ty, hidden_init) = match reference {
        None => {
            let x = crate::declarations::initialization::forms::check::copy_init(cx, e.clone(), &hidden_ty, "INIT-7")?;
            (hidden_ty, Init::Expr(x))
        }
        Some(r) => {
            let rv = r == "&&" && e.cat != Cat::L;
            let rt = QType::new(Type::Ref(Rc::new(hidden_ty.clone()), rv));
            let x = crate::declarations::initialization::references::check::bind(cx, e.clone(), &rt, "INIT-7")?;
            (rt, Init::Ref(x))
        }
    };
    let scope = cx.scope;
    let hidden = cx.u.add(crate::sem::Entity { name: "".into(), kind: EntKind::Var(var(hidden_ty.clone(), storage, Some(hidden_init.clone()))), span, scope, linkage: Linkage::None, deprecated: None });
    let mut decls = vec![(hidden, hidden_init)];
    // The member templates `get` whose head starts with a value, else the
    // `get` that argument-dependent lookup finds.
    let members: Vec<EntityId> = names::declared_here(cx, cx.u.classes[k].scope, "get").into_iter().filter(|&f| matches!(cx.u.entities[f].kind, EntKind::Template(t) if cx.u.templates[t].kinds.first().is_some_and(|x| x.is_some()))).collect();
    let any_member = !names::declared_here(cx, cx.u.classes[k].scope, "get").is_empty();
    if any_member && members.is_empty() && names::declared_here(cx, cx.u.classes[k].scope, "get").iter().any(|&f| !matches!(cx.u.entities[f].kind, EntKind::Template(_))) {
        cx.unsupported(span, "a member get that is not a template in the tuple protocol");
        return Some(failed(local, span));
    }
    let free: Vec<EntityId> = if members.is_empty() { names::argument_dependent(cx, "get", std::slice::from_ref(&ety)) } else { Vec::new() };
    let element = std_template(cx, "tuple_element");
    let size_t = QType::new(Type::Int(cx.u.target.size_t()));
    for (i, (name, sp)) in ids.iter().enumerate() {
        // Ti: std::tuple_element<i, E>::type (SB-14).
        let Some(te) = element else {
            cx.error(*sp, "SB-14", "the tuple protocol needs std::tuple_element");
            return Some(failed(local, span));
        };
        let tc = crate::templates::declarations::check::class_specialization(cx, te, vec![TArg::Value(size_t.clone(), i as i128), TArg::Type(ety.clone())], *sp);
        if tc.is_error() {
            return Some(failed(local, span));
        }
        let ti = match tc.class().and_then(|c| member(cx, c, "type")).map(|t| cx.u.entities[t].kind.clone()) {
            Some(EntKind::Typedef(t)) => t,
            _ => {
                cx.error(*sp, "SB-14", format!("std::tuple_element<{}, E> has no member type", i));
                return Some(failed(local, span));
            }
        };
        // The object: an lvalue for an lvalue reference, else an xvalue.
        let lref = matches!(hidden_ty.t, Type::Ref(_, false));
        let obj = Expr::new(EK::Var(hidden), hidden_ty.nonref(), if lref { Cat::L } else { Cat::X }, *sp);
        let explicit = [TArg::Value(size_t.clone(), i as i128)];
        let call = if !members.is_empty() {
            let specs = crate::templates::declarations::check::call_candidates_with(cx, &members, &explicit, &[]);
            match crate::overloading::overload_resolution::check::resolve_with(cx, &specs, Some(&obj), &[], *sp, "get") {
                Some(f) => {
                    let fty = QType::new(Type::Func(cx.u.func(f)?.ty.clone()));
                    let callee = Expr::new(EK::Func(f), fty, Cat::L, *sp);
                    crate::expressions::postfix::check::finish_call(cx, callee, Some(f), Vec::new(), *sp, Some(obj))
                }
                None => return Some(failed(local, span)),
            }
        } else {
            let specs = crate::templates::declarations::check::call_candidates_with(cx, &free, &explicit, std::slice::from_ref(&obj));
            if specs.is_empty() {
                cx.error(*sp, "SB-14", "no get for the tuple protocol");
                return Some(failed(local, span));
            }
            match crate::overloading::overload_resolution::check::resolve(cx, &specs, std::slice::from_ref(&obj), *sp, "get") {
                Some(f) => {
                    let fty = QType::new(Type::Func(cx.u.func(f)?.ty.clone()));
                    let callee = Expr::new(EK::Func(f), fty, Cat::L, *sp);
                    crate::expressions::postfix::check::finish_call(cx, callee, Some(f), vec![obj], *sp, None)
                }
                None => return Some(failed(local, span)),
            }
        };
        if call.ty.is_error() {
            return Some(failed(local, span));
        }
        // The name is a variable of type Ti& for an lvalue, else Ti&&.
        let rt = QType::new(Type::Ref(Rc::new(ti.clone()), call.cat != Cat::L));
        let x = crate::declarations::initialization::references::check::bind(cx, call, &rt, "INIT-7")?;
        let init = Init::Ref(x);
        let id = names::declare_in(cx, scope, name, EntKind::Var(Var { used: false, ..var(rt, storage, Some(init.clone())) }), *sp, Linkage::None);
        cx.bindings.insert(id);
        // SB-4: `decltype` of the name is Ti.
        cx.binding_decl.insert(id, ti);
        decls.push((id, init));
    }
    if !local {
        cx.u.statics.extend(decls.iter().map(|(id, _)| *id));
        return Some(None);
    }
    if storage != Storage::Automatic {
        return Some(Some(Stmt::new(SK::Decl(decls), span)));
    }
    if let Some(f) = cx.func.as_mut() {
        f.live.extend(decls.iter().map(|(id, _)| *id));
    }
    Some(Some(Stmt::new(SK::Decl(decls), span)))
}

fn failed(local: bool, span: Span) -> Option<Stmt> {
    local.then(|| Stmt::new(SK::Null, span))
}

/// The names of structured bindings, whose `decltype` is the element
/// type (SB-4).
pub fn is_binding(cx: &Check, id: EntityId) -> bool {
    cx.bindings.contains(&id)
}
