//! The check part of `expressions/new-delete`: the type and initializer
//! of a `new` expression (NEW-1, NEW-2, NEW-7, NEW-11, NEW-16, NEW-17),
//! and the operand of `delete` (NEW-8).

use crate::expressions::check::{bad, expr, first_node, nodes};
use crate::foundations::conversions::check as conv;
use crate::foundations::constant_evaluation::check as ce;
use crate::foundations::values::check::rvalue;
use crate::machine::Val;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, Init, New, EK};
use crate::sem::types::{QType, Type};
use crate::source::Span;
use crate::syntax::{Kid, Node};

/// `new_expr`.
pub fn new_expr(cx: &mut Check, n: &Node) -> Expr {
    let span = n.span;
    if n.child("placement").is_some() {
        return unsupported(cx, span, "a new expression with placement arguments");
    }
    let init = n.child("new_init");
    // The type: a `new_type`, or a `type_id` in parentheses.
    let (mut ty, bounds) = if let Some(t) = n.child("new_type") {
        let specs: Vec<&Node> = t.children("type_specifier").collect();
        let (base, ph, _, unsup) = crate::foundations::types::check::type_specifiers(cx, &specs, t.span);
        if unsup {
            return bad(span);
        }
        let base = match (base, ph) {
            (Some(b), _) => b,
            (None, Some(p)) => {
                if p.decltype || t.child("new_ptr").is_some() {
                    return unsupported(cx, span, "a new expression with decltype(auto) or a pointer to auto");
                }
                if t.child("new_bounds").is_some() {
                    cx.error(span, "NEW-16", "an array of 'auto' in a new expression");
                    return bad(span);
                }
                return new_auto(cx, n, init, (p.c, p.v));
            }
            (None, None) => return bad(span),
        };
        let mut ty = base;
        for p in t.children("new_ptr") {
            ty = crate::declarations::declarators::check::ptr_part(cx, ty, p);
        }
        (ty, t.child("new_bounds"))
    } else {
        let Some(t) = n.child("type_id") else { return bad(span) };
        (crate::declarations::declarators::check::type_id(cx, t), None)
    };
    if ty.is_error() {
        return bad(span);
    }
    // The bounds after the first, innermost last: positive constants
    // (NEW-11, NEW-17).
    let mut first: Option<&Node> = None;
    let mut written = false;
    if let Some(b) = bounds {
        written = true;
        let exprs = nodes(b).into_iter().filter(|c| c.prod != "attribute_spec").collect::<Vec<_>>();
        let has_first = matches!(b.kids.get(1), Some(Kid::Node(c)) if c.prod != "attribute_spec");
        let (head, later) = if has_first { (exprs.first().copied(), &exprs[1..]) } else { (None, &exprs[..]) };
        first = head;
        let mut dims = Vec::new();
        for c in later {
            let e = expr(cx, c);
            let e = rvalue(cx, e);
            if e.ty.is_error() {
                return bad(span);
            }
            match ce::try_value(cx, &e) {
                Some(Val::Int(v)) if v > 0 => dims.push(v as u64),
                Some(Val::Int(_)) => {
                    cx.error(e.span, "NEW-17", "a bound after the first of a new expression is not positive");
                    return bad(span);
                }
                _ => {
                    cx.error(e.span, "NEW-11", "a bound after the first of a new expression is not a constant");
                    return bad(span);
                }
            }
        }
        for d in dims.into_iter().rev() {
            ty = QType::new(Type::Array(std::rc::Rc::new(ty), Some(d)));
        }
        if first.is_none() && init.is_none() {
            cx.error(b.span, "NEW-17", "an array new expression with no bound and no initializer");
            return bad(span);
        }
    }
    // A type that is itself an array, as through an alias, gives the
    // first bound.
    let mut fixed = None;
    if !written {
        if let Type::Array(el, Some(k)) = &ty.t {
            fixed = Some(*k);
            ty = (**el).clone();
            written = true;
        }
    }
    if !complete_object(cx, &ty, span) {
        return bad(span);
    }
    if !written {
        return single(cx, ty, init, span);
    }
    array(cx, ty, first, fixed, init, span)
}

fn unsupported(cx: &mut Check, span: Span, what: &str) -> Expr {
    let t = cx.unsupported(span, what);
    Expr::new(EK::Int(0), t, Cat::PR, span)
}

/// NEW-7: a complete object type, which is no abstract class.
fn complete_object(cx: &mut Check, ty: &QType, span: Span) -> bool {
    let what = if ty.is_void() {
        "void"
    } else if ty.is_ref() {
        "a reference"
    } else if ty.is_func() {
        "a function type"
    } else if cx.u.is_incomplete(ty) {
        "an incomplete type"
    } else {
        if let Some(c) = ty.base_elem().class().filter(|c| cx.u.abstract_classes.contains(c)) {
            cx.error_cause(span, "NEW-7", Some("VIRT-8"), format!("a new expression for the abstract class '{}'", cx.u.classes[c].name));
            return false;
        }
        return true;
    };
    cx.error(span, "NEW-7", format!("a new expression for {}", what));
    false
}

/// The result: a pointer to the object, or to the first element.
fn result(n: New, span: Span) -> Expr {
    let pt = QType::ptr_to(n.ty.clone());
    Expr::new(EK::New(Box::new(n)), pt, Cat::PR, span)
}

/// Whether an initializer is `()` or `{}`, which value-initializes.
fn empty(cx: &Check, init: &Node) -> bool {
    match nodes(init).first() {
        Some(b) if b.prod == "braced_init" => b.children("init_clause").next().is_none(),
        Some(_) => false,
        None => cx.words(init).first() == Some(&"("),
    }
}

/// `new T`, `new T(args)`, `new T{args}` (NEW-1).
fn single(cx: &mut Check, ty: QType, init: Option<&Node>, span: Span) -> Expr {
    let i = match init {
        None => {
            if ty.is_const() && ty.is_scalar() {
                cx.error(span, "NEW-11", "a new expression creates a const object of scalar type with no initializer");
                return bad(span);
            }
            if ty.is_const() && ty.is_class() {
                return unsupported(cx, span, "a new expression for a const object of class type with no initializer");
            }
            match crate::classes::constructors::check::default_init(cx, &ty, span) {
                Some(i) => i,
                None => return bad(span),
            }
        }
        Some(i) if empty(cx, i) => {
            if ty.is_class() && !ty.class().is_some_and(|c| crate::classes::constructors::check::has_ctors(cx, c)) && cx.words(i).first() == Some(&"(") {
                return unsupported(cx, span, "a new expression that value-initializes a class without constructors");
            }
            if ty.is_class() {
                match crate::declarations::initialization::forms::check::initializer(cx, &ty, i, "the object") {
                    Some(x) => x,
                    None => return bad(span),
                }
            } else {
                Init::Zero
            }
        }
        Some(i) => match crate::declarations::initialization::forms::check::initializer(cx, &ty, i, "the object") {
            Some(x) => x,
            None => return bad(span),
        },
    };
    result(New { ty, bound: None, init: i, rest: Init::Default }, span)
}

/// `new auto(e)` and `new auto{e}`: the type comes from the one
/// initializer (NEW-1, NEW-16).
fn new_auto(cx: &mut Check, n: &Node, init: Option<&Node>, cv: (bool, bool)) -> Expr {
    let span = n.span;
    let Some(i) = init else {
        cx.error(span, "NEW-16", "'new auto' with no initializer");
        return bad(span);
    };
    let items: Vec<&Node> = match nodes(i).first() {
        Some(b) if b.prod == "braced_init" => b.children("init_clause").filter_map(first_node).collect(),
        Some(l) => l.children("argument").filter_map(first_node).collect(),
        None => Vec::new(),
    };
    if items.len() != 1 || items[0].prod == "braced_init" {
        cx.error(span, "NEW-16", "'new auto' needs exactly one initializer");
        return bad(span);
    }
    let e = expr(cx, items[0]);
    let e = rvalue(cx, e);
    if e.ty.is_error() {
        return bad(span);
    }
    let ty = e.ty.unqual().with_cv(cv.0, cv.1);
    if !complete_object(cx, &ty, span) {
        return bad(span);
    }
    let Some(x) = crate::declarations::initialization::forms::check::copy_init(cx, e, &ty, "NEW-1") else { return bad(span) };
    result(New { ty, bound: None, init: Init::Expr(x), rest: Init::Default }, span)
}

/// `new T[n]`, with the initializers of the first elements (NEW-2,
/// NEW-11, NEW-17). `fixed` is the bound of an array type named as a
/// whole.
fn array(cx: &mut Check, el: QType, first: Option<&Node>, fixed: Option<u64>, init: Option<&Node>, span: Span) -> Expr {
    let size_t = QType::new(Type::Int(cx.u.target.size_t()));
    // The bound converts to the type of sizeof; its constant value must
    // not be negative or make the array too large.
    let mut known = fixed;
    let bound = match first {
        Some(b) => {
            let e = expr(cx, b);
            // CVF-2: a class object converts by its conversion function.
            let Some(e) = crate::classes::conversion_functions::check::builtin_operand(cx, e) else { return bad(span) };
            let e = rvalue(cx, e);
            if e.ty.is_error() {
                return bad(span);
            }
            if e.ty.is_class() {
                return unsupported(cx, span, "an array bound of class type");
            }
            let scoped = matches!(e.ty.t, Type::Enum(en) if cx.u.enums[en].scoped);
            if scoped || !(e.ty.is_arithmetic() || e.ty.is_enum()) {
                cx.error(e.span, "NEW-17", format!("the bound of an array new expression has type {}", e.ty.show()));
                return bad(span);
            }
            if e.ty.is_integral() || e.ty.is_enum() {
                if let Some(Val::Int(v)) = ce::try_value(cx, &e) {
                    if v < 0 {
                        cx.error(e.span, "NEW-17", "the bound of an array new expression is negative");
                        return bad(span);
                    }
                    known = Some(v as u64);
                }
            }
            let espan = e.span;
            match conv::implicit(cx, e, &size_t, false) {
                Ok(x) => Some(x),
                Err(m) => {
                    cx.error(espan, "NEW-17", m);
                    return bad(span);
                }
            }
        }
        None => None,
    };
    if let Some(k) = known {
        let elem = cx.u.target.size_of(&cx.u, &el).unwrap_or(1).max(1) as u128;
        let max = cx.u.target.int_max(cx.u.target.size_t()) as u128;
        if (k as u128).saturating_mul(elem) > max {
            cx.error(span, "NEW-17", "the array of a new expression is too large");
            return bad(span);
        }
    }
    // The listed initializers, and the initialization of the rest.
    let (items, rest) = match init {
        None => {
            if el.base_elem().is_scalar() && el.is_const() {
                cx.error(span, "NEW-11", "a new expression creates const objects of scalar type with no initializer");
                return bad(span);
            }
            match default_rest(cx, &el, span, known != Some(0)) {
                Some(r) => (Vec::new(), r),
                None => return bad(span),
            }
        }
        Some(i) => {
            let items = if empty(cx, i) {
                Vec::new()
            } else {
                let unknown = QType::new(Type::Array(std::rc::Rc::new(el.clone()), None));
                match crate::declarations::initialization::forms::check::initializer(cx, &unknown, i, "the array") {
                    Some(Init::List(xs)) => xs,
                    Some(_) => return unsupported(cx, span, "an initializer of this form in an array new expression"),
                    None => return bad(span),
                }
            };
            let short = known.is_none_or(|k| (items.len() as u64) < k);
            let rest = if !short {
                Init::Default
            } else {
                match value_rest(cx, &el, span) {
                    Some(r) => r,
                    None => return bad(span),
                }
            };
            (items, rest)
        }
    };
    if let Some(k) = known {
        if items.len() as u64 > k {
            cx.error(span, "NEW-17", "the initializer has more elements than the array of the new expression");
            return bad(span);
        }
    }
    if first.is_none() && fixed.is_none() {
        known = Some(items.len() as u64);
    }
    if el.base_elem().is_class() && !crate::classes::constructors::check::dtor_usable(cx, &el, span) {
        return bad(span);
    }
    let bound = bound.unwrap_or_else(|| Expr::int(known.unwrap_or(0) as i128, size_t, span));
    result(New { ty: el, bound: Some(bound), init: Init::List(items), rest }, span)
}

/// The default-initialization of the elements without an initializer.
/// A class needs a default constructor (NEW-11).
fn default_rest(cx: &mut Check, el: &QType, span: Span, needed: bool) -> Option<Init> {
    if !needed {
        return Some(Init::Default);
    }
    if !has_default(cx, el) {
        cx.error(span, "NEW-11", "an array new expression for a class with no default constructor has no initializer for each element");
        return None;
    }
    crate::classes::constructors::check::default_init(cx, el, span)
}

/// The value-initialization of the elements without an initializer.
fn value_rest(cx: &mut Check, el: &QType, span: Span) -> Option<Init> {
    if !has_default(cx, el) {
        cx.error(span, "NEW-11", "an array new expression for a class with no default constructor has no initializer for each element");
        return None;
    }
    Some(crate::classes::members::check::value_init(cx, el))
}

/// A type whose objects can be initialized with no initializer: a class
/// with constructors needs one that takes no argument and is not deleted.
fn has_default(cx: &Check, ty: &QType) -> bool {
    match ty.base_elem().class() {
        Some(c) if crate::classes::constructors::check::has_ctors(cx, c) => crate::classes::constructors::check::ctors(cx, c).into_iter().any(|f| cx.u.func(f).is_some_and(|x| x.ty.params.is_empty() && !x.deleted)),
        _ => true,
    }
}

/// `delete_expr`: the operand is a pointer to an object type (NEW-8).
pub fn delete_expr(cx: &mut Check, n: &Node) -> Expr {
    let span = n.span;
    let array = cx.words(n).contains(&"[");
    let Some(o) = nodes(n).last().copied() else { return bad(span) };
    let e = expr(cx, o);
    let e = rvalue(cx, e);
    if e.ty.is_error() {
        return bad(span);
    }
    if e.ty.is_class() {
        return unsupported(cx, span, "a delete expression with an operand of class type");
    }
    let Some(pointee) = e.ty.pointee().cloned() else {
        cx.error(span, "NEW-8", format!("the operand of delete has type {}, not a pointer", e.ty.show()));
        return bad(span);
    };
    if pointee.is_void() || pointee.is_func() {
        cx.error(span, "NEW-8", format!("the operand of delete has type {}, not a pointer to an object type", e.ty.show()));
        return bad(span);
    }
    let mut virtual_dtor = false;
    if let Some(c) = pointee.class() {
        if cx.u.classes[c].complete {
            if !crate::classes::constructors::check::dtor_usable(cx, &pointee, span) {
                return bad(span);
            }
            virtual_dtor = crate::classes::r#virtual::check::virtual_dtor(cx, c);
            // VIRT-13: a delete through a pointer to a polymorphic class
            // whose destructor is not virtual.
            if !array && !virtual_dtor && cx.u.polymorphic(c) && !cx.u.classes[c].is_final && cx.wflag(&["-Wall", "-Wdelete-non-virtual-dtor"]) {
                cx.warning(span, "VIRT-13", format!("a delete through a pointer to the polymorphic class '{}', whose destructor is not virtual", cx.u.classes[c].name));
            }
        }
    }
    Expr::new(EK::Delete(Box::new(e), array, virtual_dtor), QType::void(), Cat::PR, span)
}
