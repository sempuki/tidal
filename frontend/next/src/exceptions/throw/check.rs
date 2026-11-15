//! The check part of `exceptions/throw`: the operand of `throw` and the
//! initialization of the exception object (THR-1 to THR-5).

use crate::expressions::check::{bad, expr, first_node};
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, EK};
use crate::sem::types::QType;
use crate::syntax::Node;

/// `throw_expr`: a prvalue of type `void` (THR-5).
pub fn throw_expr(cx: &mut Check, n: &Node) -> Expr {
    let span = n.span;
    let Some(op) = first_node(n) else { return Expr::new(EK::Throw(None), QType::void(), Cat::PR, span) };
    let e = expr(cx, op);
    if e.ty.is_error() {
        return bad(span);
    }
    // THR-2: an array or a function becomes a pointer, and the exception
    // object has no cv-qualifiers.
    let e = crate::foundations::values::check::deref_ref(e);
    let e = if e.ty.is_array() || e.ty.is_func() { crate::foundations::conversions::check::decay(cx, e) } else { e };
    let ty = e.ty.unqual();
    // THR-3: a complete type that is not abstract, or a pointer to one or
    // to void.
    let incomplete = |cx: &Check, t: &QType| t.is_void() || cx.u.is_incomplete(t) || matches!(t.t, crate::sem::types::Type::Array(_, None));
    if incomplete(cx, &ty) {
        cx.error(span, "THR-3", format!("the operand of throw has the incomplete type {}", ty.show()));
        return bad(span);
    }
    if let Some(p) = ty.pointee().filter(|p| !p.is_void() && !p.is_func()) {
        if incomplete(cx, p) {
            cx.error(span, "THR-3", format!("the operand of throw points to the incomplete type {}", p.show()));
            return bad(span);
        }
    }
    if !crate::classes::r#virtual::check::not_abstract(cx, &ty, span) || !crate::classes::constructors::check::dtor_usable(cx, &ty, span) {
        return bad(span);
    }
    // THR-1: a prvalue initializes the object directly; a local variable
    // that every enclosing try block declares is moved; another glvalue
    // is copied.
    let init = if ty.is_class() && e.cat != Cat::PR {
        let mut e = e;
        if movable(cx, &e) {
            e.cat = Cat::X;
        }
        match crate::classes::copy_move::check::copy(cx, e, true, span) {
            Some(x) => x,
            None => return bad(span),
        }
    } else if ty.is_class() {
        e
    } else {
        crate::foundations::values::check::rvalue(cx, e)
    };
    Expr::new(EK::Throw(Some(Box::new(init))), QType::void(), Cat::PR, span)
}

/// THR-1: the operand names a local variable or a parameter of an object
/// type that is not volatile, declared inside each try block around the
/// throw in the same function.
fn movable(cx: &Check, e: &Expr) -> bool {
    let v = match &e.k {
        EK::Var(v) => *v,
        EK::Deref(x) | EK::Conv(_, x) => match x.k {
            EK::Var(v) => v,
            _ => return false,
        },
        _ => return false,
    };
    let Some(var) = cx.u.var(v) else { return false };
    let local = var.storage == crate::sem::Storage::Automatic || var.param;
    // An rvalue reference to an object type counts, and the object it
    // names is moved.
    let object = match &var.ty.t {
        crate::sem::types::Type::Ref(t, true) => !t.v,
        crate::sem::types::Type::Ref(_, false) => false,
        _ => !var.ty.v,
    };
    let tries = cx.func.as_ref().map(|f| f.tries.clone()).unwrap_or_default();
    local && object && tries.iter().all(|&start| v >= start && !var.param)
}

/// UNW-8: a `throw` in a non-throwing function that no enclosing try
/// block of the function can catch warns. A rethrow is caught only by
/// `catch (...)`, and a handler's own try block does not enclose it.
pub fn noexcept_throws(cx: &mut Check, body: &crate::sem::tree::Stmt) {
    let mut spans = Vec::new();
    stmt(cx, body, &mut Vec::new(), &mut spans);
    for span in spans {
        cx.warning(span, "UNW-8", "a throw in a non-throwing function that no handler of the function catches");
    }
}

/// The handler types of each enclosing try block, innermost last.
type Enclosing = Vec<Vec<Option<QType>>>;

fn stmt(cx: &Check, s: &crate::sem::tree::Stmt, tries: &mut Enclosing, out: &mut Vec<crate::source::Span>) {
    use crate::sem::tree::{Cond, Init, SK};
    let ex = |e: &Expr, tries: &Enclosing, out: &mut Vec<crate::source::Span>| {
        let found = std::cell::RefCell::new(Vec::new());
        e.any(&|x| {
            if let EK::Throw(op) = &x.k {
                let caught = tries.iter().any(|hs| hs.iter().any(|h| match (h, op) {
                    (None, _) => true,
                    (Some(_), None) => false,
                    (Some(t), Some(o)) => catches(cx, t, &o.ty),
                }));
                if !caught {
                    found.borrow_mut().push(x.span);
                }
            }
            false
        });
        out.extend(found.into_inner());
    };
    let cond = |c: &Cond| match c {
        Cond::Expr(e) => Some(e.clone()),
        Cond::Decl(_, Init::Expr(e), _) => Some(e.clone()),
        _ => None,
    };
    match &s.k {
        SK::Expr(e) | SK::Return(Some(e)) | SK::ReturnRef(e) => ex(e, tries, out),
        SK::Decl(vars) => {
            for (_, i) in vars {
                if let Init::Expr(e) | Init::Ref(e) = i {
                    ex(e, tries, out);
                }
            }
        }
        SK::Block(items) => items.iter().for_each(|x| stmt(cx, x, tries, out)),
        SK::If { init, cond: c, then, els } => {
            init.iter().for_each(|x| stmt(cx, x, tries, out));
            if let Some(e) = cond(c) {
                ex(&e, tries, out);
            }
            stmt(cx, then, tries, out);
            els.iter().for_each(|x| stmt(cx, x, tries, out));
        }
        SK::While { body, .. } | SK::Do { body, .. } | SK::For { body, .. } | SK::Switch { body, .. } | SK::RangeArray { body, .. } => stmt(cx, body, tries, out),
        SK::Case(_, _, x) | SK::Label(_, x) => stmt(cx, x, tries, out),
        SK::Chosen(Some(x)) => stmt(cx, x, tries, out),
        SK::Try { body, handlers } => {
            tries.push(handlers.iter().map(|h| h.ty.clone()).collect());
            stmt(cx, body, tries, out);
            tries.pop();
            handlers.iter().for_each(|h| stmt(cx, &h.body, tries, out));
        }
        _ => {}
    }
}

/// TRY-2 by the static types: a handler of type `t` catches an exception
/// of type `e`.
pub fn catches(cx: &Check, t: &QType, e: &QType) -> bool {
    let (t, e) = (t.nonref().unqual(), e.nonref().unqual());
    if t == e {
        return true;
    }
    if let (Some(b), Some(d)) = (t.class(), e.class()) {
        let ps = cx.u.base_paths(d, b);
        return ps.len() == 1 && cx.u.path_public(d, &ps[0]);
    }
    match (t.pointee(), e.pointee()) {
        (Some(tp), Some(ep)) => {
            let qual = (tp.c || !ep.c) && (tp.v || !ep.v);
            qual && (tp.unqual() == ep.unqual() || (tp.is_void() && !ep.is_func()) || matches!((tp.class(), ep.class()), (Some(b), Some(d)) if { let ps = cx.u.base_paths(d, b); ps.len() == 1 && cx.u.path_public(d, &ps[0]) }))
        }
        _ => t.pointee().is_some() && e.is_nullptr(),
    }
}
