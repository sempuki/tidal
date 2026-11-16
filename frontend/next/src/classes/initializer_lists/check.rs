//! `classes/initializer-lists`, the Check segment: the
//! `std::initializer_list` objects that braced lists make (build step 5).

use crate::foundations::names::check as names;
use crate::sem::check::Check;
use crate::sem::tree::{Arg, Cat, Expr, Init, EK};
use crate::sem::types::{QType, TArg, Type};
use crate::sem::{EntKind, Linkage, Storage, Var};
use crate::source::Span;
use crate::syntax::Node;

/// The class template `std::initializer_list`, if the program declares
/// it (LSUP-1).
pub fn template(cx: &Check) -> Option<usize> {
    let s = cx.u.scopes[cx.u.root].names.get("std")?.iter().find_map(|&e| match cx.u.entities[e].kind {
        EntKind::Namespace(s) => Some(s),
        _ => None,
    })?;
    cx.u.scopes[s].names.get("initializer_list")?.iter().find_map(|&e| match cx.u.entities[e].kind {
        EntKind::ClassTemplate(v) => Some(v),
        _ => None,
    })
}

/// The element type `E` when `ty` is `std::initializer_list<E>`.
pub fn element(cx: &Check, ty: &QType) -> Option<QType> {
    let c = ty.nonref().class()?;
    let v = template(cx)?;
    let (args, _) = cx.u.class_templates[v].specs.iter().find(|(_, k)| *k == c)?;
    match args.as_slice() {
        [TArg::Type(e)] => Some(e.clone()),
        _ => None,
    }
}

/// The class `std::initializer_list<E>`.
pub fn list_of(cx: &mut Check, e: QType, span: Span) -> Option<QType> {
    let v = template(cx)?;
    let t = crate::templates::declarations::check::class_specialization(cx, v, vec![TArg::Type(e)], span);
    t.class().map(|_| t)
}

/// ILC-5: the initializer of an object of type `std::initializer_list<E>`
/// from the elements of a braced list: an array of `const E` that holds
/// a copy of each element, declared before the object and living as
/// long as it (`Check::backing`), and the object made from a pointer to
/// the array and its length. None after an error, or when the front end
/// does not model the form.
pub fn backed(cx: &mut Check, ty: &QType, items: &[&Node], storage: Storage, span: Span) -> Option<Init> {
    let e = element(cx, ty)?;
    let c = ty.class()?;
    // The array and its elements, each copied as an element of a list
    // (ILC-4: no narrowing).
    let mut inits = Vec::new();
    for it in items {
        let x = crate::expressions::check::first_node(it)?;
        if matches!(x.prod, "braced_init" | "designator") {
            cx.unsupported(x.span, "a nested braced list as an element of std::initializer_list");
            return None;
        }
        let v = crate::expressions::check::expr(cx, x);
        if v.ty.is_error() {
            return None;
        }
        if crate::foundations::conversions::check::is_narrowing(cx, &v, &e) {
            cx.error_cause(x.span, "ILC-4", Some("LIST-12"), format!("the element narrows from {} to {}", v.ty.show(), e.show()));
            return None;
        }
        let i = crate::declarations::initialization::forms::check::copy_init(cx, v, &e.unqual(), "INIT-7")?;
        inits.push(Init::Expr(i));
    }
    let n = inits.len() as u64;
    let array = QType::new(Type::Array(std::rc::Rc::new(e.unqual().with_cv(true, e.v)), Some(n)));
    let k = cx.backing_constexpr;
    let var = Var { ty: array.clone(), storage, constexpr: k, constinit: false, inline: false, is_extern: false, is_static: false, param: false, defined: true, init: Some(Init::List(inits.clone())), const_init: k, used: true, maybe_unused: true, align: None };
    let scope = cx.scope;
    let hidden = cx.u.add(crate::sem::Entity { name: "".into(), kind: EntKind::Var(var), span, scope, linkage: Linkage::None, deprecated: None });
    cx.backing.push((hidden, Init::List(inits)));
    // The object: the constructor that takes a pointer to `const E` and
    // a length.
    let ptr = QType::ptr_to(e.unqual().with_cv(true, e.v));
    let size = QType::new(Type::Int(cx.u.target.size_t()));
    let ctor = crate::classes::constructors::check::ctors(cx, c).into_iter().find(|&f| cx.u.func(f).is_some_and(|x| x.ty.params.len() == 2 && x.ty.params[0].unqual() == ptr && x.ty.params[1].is_integral()));
    let Some(ctor) = ctor else {
        cx.unsupported(span, "a std::initializer_list without a constructor from a pointer and a length");
        return None;
    };
    let a = Expr::new(EK::Var(hidden), array, Cat::L, span);
    let a = crate::foundations::values::check::rvalue(cx, a);
    Some(Init::Ctor(ctor, vec![Arg { e: a, by_ref: false }, Arg { e: Expr::int(n as i128, size, span), by_ref: false }]))
}

/// AUTO-21: the type `std::initializer_list<E>` that `auto x = { ... }`
/// deduces, with `E` the one type of the elements after decay.
pub fn deduced(cx: &mut Check, items: &[&Node], span: Span) -> Option<QType> {
    let mut e: Option<QType> = None;
    for it in items {
        let x = crate::expressions::check::first_node(it)?;
        let marks = cx.u.diags.len();
        cx.unevaluated += 1;
        let v = crate::expressions::check::expr(cx, x);
        cx.unevaluated -= 1;
        cx.u.diags.truncate(marks);
        let t = crate::declarations::auto::check::decay(&v.ty.nonref()).unqual();
        if t.is_error() {
            return None;
        }
        match &e {
            None => e = Some(t),
            Some(prev) if *prev == t => {}
            Some(_) => {
                cx.error(span, "AUTO-21", "'auto' with a braced list whose elements have different types");
                return None;
            }
        }
    }
    let Some(e) = e else {
        cx.error(span, "AUTO-21", "'auto' with an empty braced list");
        return None;
    };
    let _ = names::lookup;
    list_of(cx, e, span)
}

/// An initializer-list constructor of class `c`: one whose first
/// parameter is `std::initializer_list<E>` or a reference to it.
pub fn has_list_ctor(cx: &Check, c: crate::sem::types::ClassId) -> bool {
    template(cx).is_some() && crate::classes::constructors::check::ctors(cx, c).iter().any(|&f| cx.u.func(f).is_some_and(|x| x.ty.params.first().is_some_and(|p| element(cx, p).is_some())))
}

/// A function of the overload set `found` with a parameter of type
/// `std::initializer_list<E>`.
pub fn takes_list(cx: &Check, found: &[crate::sem::EntityId]) -> bool {
    let listy = |p: &QType| element(cx, p).is_some() || p.nonref().class().is_some_and(|c| has_list_ctor(cx, c));
    template(cx).is_some() && found.iter().any(|&f| cx.u.func(f).is_some_and(|x| x.ty.params.iter().any(listy)) || matches!(cx.u.entities[f].kind, EntKind::Template(t) if cx.u.templates[t].fty.params.iter().any(|p| matches!(&p.nonref().t, Type::Dep(_)) || element(cx, p).is_some())))
}

/// ILC-2, ILC-3: the first phase of the initialization of class `ty`
/// from a braced list in a variable's declaration: the
/// initializer-list constructor that the elements convert to, called
/// with the list. Some(None) after an error; None when the first phase
/// does not apply, which leaves the second phase to the caller.
pub fn list_ctor_init(cx: &mut Check, ty: &QType, items: &[&Node], copy: bool, span: Span) -> Option<Option<Init>> {
    let c = ty.class()?;
    let storage = cx.backing_ok?;
    let ctors = crate::classes::constructors::check::ctors(cx, c);
    // ILC-2: `{}` takes the default constructor if there is one.
    if items.is_empty() && ctors.iter().any(|&f| cx.u.func(f).is_some_and(|x| x.takes_no_argument())) {
        return None;
    }
    let list_ctors: Vec<(crate::sem::EntityId, QType)> = ctors.iter().filter_map(|&f| {
        let x = cx.u.func(f)?;
        let e = element(cx, x.ty.params.first()?)?;
        (1..x.ty.params.len()).all(|k| x.defaults.get(k).is_some_and(|d| d.is_some())).then_some((f, e))
    }).collect();
    if list_ctors.is_empty() {
        return None;
    }
    // The elements, checked quietly, convert to `E` (narrowing keeps the
    // constructor viable, ILC-4).
    let mut elems = Vec::new();
    for it in items {
        let x = crate::expressions::check::first_node(it)?;
        if matches!(x.prod, "braced_init" | "designator") {
            return None;
        }
        let marks = cx.u.diags.len();
        let v = crate::expressions::check::expr(cx, x);
        cx.u.diags.truncate(marks);
        elems.push(v);
    }
    let viable: Vec<&(crate::sem::EntityId, QType)> = list_ctors.iter().filter(|(_, e)| elems.iter().all(|v| crate::overloading::conversion_sequences::check::sequence(cx, v, &e.unqual()).is_some())).collect();
    let [(f, e)] = viable.as_slice() else {
        if viable.len() > 1 {
            cx.unsupported(span, "a braced list that several initializer-list constructors take");
            return Some(None);
        }
        return None;
    };
    let (f, e) = (*f, e.clone());
    // ILC-8: copy list initialization does not take an explicit one.
    if copy && cx.u.func(f).is_some_and(|x| x.explicit) {
        cx.error(span, "ILC-8", "copy list initialization chooses an explicit initializer-list constructor");
        return Some(None);
    }
    let Some(lt) = list_of(cx, e, span) else { return Some(None) };
    let Some(init) = backed(cx, &lt, items, storage, span) else { return Some(None) };
    let arg = Expr::new(EK::List(Box::new(init)), lt.clone(), Cat::PR, span);
    let param = cx.u.func(f).map(|x| x.ty.params[0].clone()).unwrap_or(lt);
    let arg = if param.is_ref() {
        match crate::declarations::initialization::references::check::bind(cx, arg, &param, "INIT-7") {
            Some(a) => Arg { e: a, by_ref: true },
            None => return Some(None),
        }
    } else {
        Arg { e: arg, by_ref: false }
    };
    let n = cx.u.func(f).map(|x| x.ty.params.len()).unwrap_or(1);
    let mut args = vec![arg];
    for k in 1..n {
        let Some(Some(d)) = cx.u.func(f).and_then(|x| x.defaults.get(k).cloned()) else { return Some(None) };
        let pt = cx.u.func(f).map(|x| x.ty.params[k].clone()).unwrap_or_else(QType::error);
        args.push(Arg { e: d, by_ref: pt.is_ref() });
    }
    Some(Some(Init::Ctor(f, args)))
}
