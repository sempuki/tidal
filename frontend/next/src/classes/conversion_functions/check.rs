//! The check part of `classes/conversion-functions`: the declaration of
//! a conversion function (CVF-1, CVF-7) and its use by a conversion
//! (CVF-2 to CVF-5, CVF-9 to CVF-12). A conversion function is a member
//! function named `operator T`, with the target type shown in its name,
//! so that one with the same target in a derived class hides it (CVF-5).

use crate::overloading::conversion_sequences::check::{better, sequence, Ics, Rank};
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, EK};
use crate::sem::types::{ClassId, QType, Type};
use crate::sem::{EntityId, Unit};
use crate::source::Span;
use crate::syntax::Node;

/// The name of the conversion function to `t`. A function type that is
/// `noexcept` is another type (CVF-5).
pub fn name(t: &QType) -> String {
    fn noexcept(t: &QType) -> bool {
        match &t.t {
            Type::Func(f) => f.noexcept || noexcept(&f.ret),
            Type::Ptr(x) | Type::Ref(x, _) | Type::Array(x, _) => noexcept(x),
            _ => false,
        }
    }
    if noexcept(t) {
        return format!("operator {} noexcept", t.show());
    }
    format!("operator {}", t.show())
}

/// Whether the function `f` is a conversion function.
pub fn is_conversion(u: &Unit, f: EntityId) -> bool {
    u.entities[f].name.starts_with("operator ") && u.func(f).is_some_and(|x| x.class.is_some())
}

/// The target type of a `conversion_function_id` (CVF-1, CVF-7): a type
/// name with `*` and `&` only, which is no array, function, or
/// placeholder.
pub fn target(cx: &mut Check, n: &Node) -> QType {
    let Some(ct) = n.child("conversion_type") else { return QType::error() };
    let specs: Vec<&Node> = ct.child("type_specifiers").map(|s| s.children("type_specifier").collect()).unwrap_or_default();
    let (ty, ph, _, unsup) = crate::foundations::types::check::type_specifiers(cx, &specs, ct.span);
    if unsup {
        return QType::error();
    }
    let Some(mut t) = ty else {
        if ph.is_some() {
            return cx.unsupported(ct.span, "a conversion function to a placeholder type");
        }
        return QType::error();
    };
    for p in ct.children("ptr_part") {
        t = crate::declarations::declarators::check::ptr_part(cx, t, p);
    }
    if t.is_array() || t.is_func() {
        return cx.error(ct.span, "CVF-7", format!("a conversion function to the {} type {}", if t.is_array() { "array" } else { "function" }, t.show()));
    }
    t
}

/// CVF-7: the limits on a conversion function's declaration. True when
/// it is valid.
pub fn declaration(cx: &mut Check, specs: &crate::declarations::check::Specs, ty: &QType, member: bool, span: Span) -> bool {
    if specs.ty.is_some() {
        cx.error(span, "CVF-7", "a conversion function with a return type before 'operator'");
        return false;
    }
    if !member {
        cx.error(span, "CVF-7", "a conversion function that is not a member function");
        return false;
    }
    if specs.is_static {
        cx.error(span, "CVF-7", "a static conversion function");
        return false;
    }
    if let Type::Func(ft) = &ty.t {
        if !ft.params.is_empty() || ft.variadic {
            cx.error(span, "CVF-7", "a conversion function with parameters");
            return false;
        }
        // CVF-6: a conversion to the class itself, a base, or void is
        // never used.
        let r = ft.ret.nonref();
        let c = cx.member_of;
        let never = r.is_void() || r.class().is_some_and(|d| Some(d) == c || c.is_some_and(|c| crate::classes::derived::check::is_base(cx, c, d)));
        if never {
            cx.warning(span, "CVF-6", format!("the conversion function to {} is never used by a conversion", r.show()));
        }
    }
    true
}

/// The conversion functions of class `c`: its own, and those of its bases
/// that one of its own does not hide (CVF-5).
pub fn functions(u: &Unit, c: ClassId) -> Vec<EntityId> {
    let mut own: Vec<(std::rc::Rc<str>, EntityId)> = Vec::new();
    for (n, fs) in &u.scopes[u.classes[c].scope].names {
        if n.starts_with("operator ") {
            own.extend(fs.iter().filter(|&&f| u.func(f).is_some()).map(|&f| (n.clone(), f)));
        }
    }
    let mut out: Vec<EntityId> = own.iter().map(|x| x.1).collect();
    for (_, b) in u.bases(c) {
        for f in functions(u, b) {
            if !own.iter().any(|(n, _)| *n == u.entities[f].name) && !out.contains(&f) {
                out.push(f);
            }
        }
    }
    out.sort();
    out
}

/// The class of an operand whose class has conversion functions.
pub fn converting_class(cx: &Check, e: &Expr) -> Option<ClassId> {
    e.ty.nonref().class().filter(|&c| cx.u.classes[c].complete && !functions(&cx.u, c).is_empty())
}

/// A candidate: the function, whether its object binding is exact, and
/// the sequence from its result to the target.
struct Cand {
    f: EntityId,
    exact_object: bool,
    second: Ics,
}

/// The value that a call of `f` gives, as an expression of its type and
/// category, for the sequence after the conversion function.
fn result_of(cx: &Check, f: EntityId, span: Span) -> Expr {
    let ret = cx.u.func(f).map(|x| x.ty.ret.clone()).unwrap_or_else(QType::error);
    let (ty, cat) = match &ret.t {
        Type::Ref(inner, rv) => ((**inner).clone(), if *rv { Cat::X } else { Cat::L }),
        _ => (ret.unqual_top(), Cat::PR),
    };
    Expr::new(EK::Int(1), ty, cat, span)
}

/// The viable conversion functions from the object `e` to `to` (CVF-2,
/// CVF-3, CVF-4, CVF-9). With `direct`, an `explicit` function counts
/// when its result has the target type.
fn candidates(cx: &Check, e: &Expr, to: &QType, direct: bool) -> Vec<Cand> {
    let Some(c) = converting_class(cx, e) else { return Vec::new() };
    let obj = e.ty.nonref();
    let mut out = Vec::new();
    for f in functions(&cx.u, c) {
        let Some(func) = cx.u.func(f) else { continue };
        let ret = func.ty.ret.nonref();
        // CVF-4: no conversion to the class itself, a base, or void.
        if ret.is_void() || ret.class().is_some_and(|r| r == c || crate::classes::derived::check::is_base(cx, c, r)) {
            continue;
        }
        if func.explicit && !(direct && ret.unqual().t == to.nonref().unqual().t) {
            continue;
        }
        // CVF-9: a const object needs a const conversion function.
        if (obj.c && !func.obj_cv.0) || (obj.v && !func.obj_cv.1) {
            continue;
        }
        // CVF-4: the ref-qualifier takes the object by its category.
        let lvalue = e.cat == Cat::L;
        let binds = match func.ref_q {
            Some(true) => !lvalue,
            Some(false) => lvalue || (func.obj_cv.0 && !func.obj_cv.1),
            None => true,
        };
        if !binds {
            continue;
        }
        let r = result_of(cx, f, e.span);
        // A class target takes a result of the class itself.
        let second = if let Some(d) = to.nonref().class() {
            if r.ty.class() != Some(d) && !r.ty.class().is_some_and(|k| crate::classes::derived::check::is_base(cx, k, d)) {
                continue;
            }
            match sequence(cx, &r, to) {
                Some(s) => s,
                None if !to.is_ref() => Ics { rank: Rank::Exact, qual: false, binds: None, ellipsis: false, user: None },
                None => continue,
            }
        } else {
            match sequence(cx, &r, to) {
                Some(s) => s,
                None => continue,
            }
        };
        let exact_ref = func.ref_q.is_none_or(|rv| rv != lvalue);
        out.push(Cand { f, exact_object: obj.c == func.obj_cv.0 && obj.v == func.obj_cv.1 && exact_ref, second });
    }
    out
}

/// Why no candidate is the best.
#[derive(Clone, Copy, PartialEq)]
enum NoBest {
    /// Two candidates are as good (CVF-12).
    Ambiguous,
    /// The candidates differ in ways that the front end does not rank.
    Unranked,
}

/// The best candidate (CVF-4: the object binding, then the sequence
/// after the function), or why there is none.
fn choose(cx: &Check, e: &Expr, to: &QType, direct: bool) -> Result<Option<(EntityId, Ics)>, NoBest> {
    let cands = candidates(cx, e, to, direct);
    let wins = |a: &Cand, b: &Cand| (a.exact_object && !b.exact_object) || (a.exact_object == b.exact_object && better(&a.second, &b.second));
    let Some(b) = cands.iter().find(|a| cands.iter().all(|o| o.f == a.f || wins(a, o))) else {
        if cands.is_empty() {
            return Ok(None);
        }
        // CVF-12 holds for candidates whose sequences after the function
        // rank alike; others need ICS rules that the front end does not
        // weigh yet.
        let first = &cands[0];
        let arithmetic = |o: &Cand| cx.u.func(o.f).is_some_and(|x| x.ty.ret.nonref().is_arithmetic() || x.ty.ret.nonref().is_enum());
        if cands.iter().all(|o| o.exact_object == first.exact_object && o.second == first.second && arithmetic(o)) {
            return Err(NoBest::Ambiguous);
        }
        return Err(NoBest::Unranked);
    };
    Ok(Some((b.f, b.second.clone())))
}

/// The call of the conversion function `f` on the object `e`.
pub fn call(cx: &mut Check, f: EntityId, e: Expr, span: Span) -> Expr {
    if !crate::classes::members::check::accessible(cx, f) && !e.ty.nonref().class().is_some_and(|c| crate::classes::derived::check::accessible_through(cx, f, c)) {
        cx.error(span, "ACC-2", format!("the conversion function '{}' is not public", cx.u.entities[f].name));
        return crate::expressions::check::bad(span);
    }
    let Some(func) = cx.u.func(f).cloned() else { return crate::expressions::check::bad(span) };
    let kind = if cx.u.virt.contains_key(&f) { EK::Virtual(f) } else { EK::Func(f) };
    let callee = Expr::new(kind, QType::new(Type::Func(func.ty.clone())), Cat::L, span);
    let e = crate::foundations::values::check::deref_ref(e);
    // The object of a member of a base is the base subobject.
    let e = match (e.ty.class(), func.class) {
        (Some(d), Some(b)) if d != b => {
            let e = if e.cat == Cat::PR {
                let t = e.ty.clone();
                e.conv(crate::sem::tree::Conv::Materialize, t, Cat::X)
            } else {
                e
            };
            match crate::classes::derived::check::to_base(cx, e, b, span) {
                Some(x) => x,
                None => return crate::expressions::check::bad(span),
            }
        }
        _ => e,
    };
    crate::expressions::postfix::check::finish_call(cx, callee, Some(f), Vec::new(), span, Some(e))
}

/// CVF-2: the implicit conversion of a class object `e` to `to` by a
/// conversion function, or None when no conversion function of its
/// class can do it. `direct` allows an `explicit` one whose result has
/// the target type (CVF-3).
pub fn convert(cx: &mut Check, e: &Expr, to: &QType, direct: bool) -> Option<Result<Expr, String>> {
    converting_class(cx, e)?;
    let span = e.span;
    match choose(cx, e, to, direct) {
        Ok(Some((f, _))) => Some(Ok(call(cx, f, e.clone(), span))),
        // CVF-11: an explicit function that only direct initialization
        // can use.
        Ok(None) if !direct && matches!(choose(cx, e, to, true), Ok(Some(_))) => Some(Err(format!("the conversion from {} to {} needs an explicit conversion function (CVF-11)", e.ty.nonref().show(), to.show()))),
        Ok(None) => None,
        Err(NoBest::Ambiguous) => Some(Err(format!("the conversion from {} to {} is ambiguous (CVF-12)", e.ty.nonref().show(), to.show()))),
        Err(NoBest::Unranked) => {
            let t = cx.unsupported(span, "a choice between conversion functions that the front end does not rank");
            Some(Ok(Expr::new(EK::Int(0), t, Cat::PR, span)))
        }
    }
}

/// ICS-4: the user-defined sequence from a class object to a parameter
/// type by a conversion function. Two such sequences compare by the
/// sequence after the function when they use the same function.
pub fn user_sequence(cx: &Check, e: &Expr, to: &QType) -> Option<Ics> {
    converting_class(cx, e)?;
    match choose(cx, e, to, false) {
        Ok(Some((f, second))) => Some(Ics { rank: Rank::User, qual: false, binds: None, ellipsis: false, user: Some((f, second.rank)) }),
        Ok(None) => None,
        // ICS-8: an ambiguous conversion is a user-defined sequence that
        // is as good as any other.
        Err(_) => Some(Ics { rank: Rank::User, qual: false, binds: None, ellipsis: false, user: None }),
    }
}

/// An operand of class type of a built-in operator, converted by the one
/// conversion function of its class to an arithmetic, enumeration, or
/// pointer type (OVL-9, CVF-2). None after an unsupported form.
pub fn builtin_operand(cx: &mut Check, e: Expr) -> Option<Expr> {
    let Some(c) = converting_class(cx, &e) else { return Some(e) };
    let obj = e.ty.nonref();
    let fs: Vec<EntityId> = functions(&cx.u, c).into_iter().filter(|&f| {
        cx.u.func(f).is_some_and(|x| {
            let r = x.ty.ret.nonref();
            !x.explicit && (!obj.c || x.obj_cv.0) && (r.is_arithmetic() || r.is_enum() || r.is_ptr())
        })
    }).collect();
    // Functions with one result type differ in their object binding only
    // (CVF-4): an exact one wins.
    let fs = if fs.len() > 1 && fs.iter().all(|&f| cx.u.func(f).map(|x| x.ty.ret.nonref().unqual().t.clone()) == cx.u.func(fs[0]).map(|x| x.ty.ret.nonref().unqual().t.clone())) {
        let exact: Vec<EntityId> = fs.iter().copied().filter(|&f| cx.u.func(f).is_some_and(|x| x.obj_cv == (obj.c, obj.v))).collect();
        if exact.len() == 1 { exact } else { fs }
    } else {
        fs
    };
    match fs.as_slice() {
        // CVF-11: an explicit conversion function does not convert an
        // operand of a built-in operator.
        [] if functions(&cx.u, c).iter().any(|&f| cx.u.func(f).is_some_and(|x| x.explicit)) => {
            cx.error(e.span, "CVF-11", format!("an operand of class {} needs an explicit conversion function", obj.show()));
            None
        }
        [] => Some(e),
        [f] => {
            let span = e.span;
            Some(call(cx, *f, e, span))
        }
        _ => {
            cx.unsupported(e.span, "an operand of a built-in operator whose class has several conversion functions");
            None
        }
    }
}

/// `builtin_operand` for both operands of a binary operator.
pub fn builtin_operands(cx: &mut Check, l: Expr, r: Expr) -> Option<(Expr, Expr)> {
    let l = builtin_operand(cx, l)?;
    let r = builtin_operand(cx, r)?;
    Some((l, r))
}
