//! The check part of `expressions/decltype`.

use crate::expressions::check::{expr, first_node};
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, EK};
use crate::sem::types::{QType, Type};
use crate::syntax::Node;
use std::rc::Rc;

/// The type `decltype` gives for a typed expression (DTY-1, DTY-2): the
/// declared type of an unparenthesized name, else `T&` for an lvalue,
/// `T&&` for an xvalue, and `T` for a prvalue.
pub fn decltype_of(e: &Expr) -> QType {
    decltype_with(e, None)
}

/// The type `decltype` gives for a typed expression, with the declared
/// type of an unparenthesized name of a variable or a function (DTY-1).
pub fn decltype_of_named(cx: &Check, e: &Expr) -> QType {
    let declared = match &e.k {
        EK::Var(id) => cx.u.var(*id).map(|v| if cx.bindings.contains(id) { v.ty.nonref() } else { v.ty.clone() }),
        EK::Func(id) => cx.u.func(*id).map(|f| QType::new(Type::Func(f.ty.clone()))),
        _ => None,
    };
    decltype_with(e, declared)
}

fn decltype_with(e: &Expr, declared: Option<QType>) -> QType {
    if let Some(t) = declared {
        if !e.paren {
            return t;
        }
    }
    match e.cat {
        Cat::L => QType::new(Type::Ref(Rc::new(e.ty.clone()), false)),
        Cat::X => QType::new(Type::Ref(Rc::new(e.ty.clone()), true)),
        Cat::PR => e.ty.clone(),
    }
}

/// `decltype_spec: "decltype" "(" expression ")"`. The operand is not
/// evaluated (UNEV-1).
pub fn decltype_spec(cx: &mut Check, n: &Node) -> QType {
    if cx.has_word(n, "auto") {
        return cx.unsupported(n.span, "decltype(auto) here");
    }
    let Some(mut c) = first_node(n) else { return QType::error() };
    if c.prod == "decltype_expr" {
        let Some(x) = first_node(c) else { return QType::error() };
        c = x;
    }
    cx.unevaluated += 1;
    let e = expr(cx, c);
    cx.unevaluated -= 1;
    unevaluated_effect_warning(cx, &e);
    if crate::expressions::primary::check::unresolved_set(&e) {
        crate::expressions::primary::check::no_target(cx, &e, "DTY-4");
    }
    if e.ty.is_error() {
        return QType::error();
    }
    let declared = match &e.k {
        EK::Var(id) => cx.u.var(*id).map(|v| if cx.bindings.contains(id) { v.ty.nonref() } else { v.ty.clone() }),
        EK::Func(id) => cx.u.func(*id).map(|f| QType::new(Type::Func(f.ty.clone()))),
        _ => None,
    };
    let unparenthesized_name = matches!(c.prod, "primary" | "id_expression") && !e.paren;
    decltype_with(&e, if unparenthesized_name { declared } else { None })
}

/// UNEV-3: whether the step that gives the value of a discarded-value
/// expression has an effect: it calls a function, writes an object,
/// reads a volatile object, or allocates storage. A cast to `void`
/// counts, so that it prevents the warning.
pub fn has_effect(e: &crate::sem::tree::Expr) -> bool {
    use crate::sem::tree::{Conv, EK};
    match &e.k {
        EK::Call(..) | EK::Assign(..) | EK::Compound(..) | EK::IncDec(..) | EK::Assume(_) => true,
        EK::Conv(Conv::ToVoid, _) => true,
        EK::Conv(_, x) => has_effect(x),
        EK::Comma(_, b) => has_effect(b),
        EK::Cond(_, a, b) => has_effect(a) || has_effect(b),
        EK::Var(_) | EK::Index(..) | EK::Deref(_) => e.ty.is_volatile(),
        _ => false,
    }
}

/// UNEV-4: warn about an unevaluated operand with an increment, a
/// decrement, or an assignment, whose effect does not occur. Clang
/// warns with no flag. An operand inside another unevaluated operand
/// is left to the outer one.
pub fn unevaluated_effect_warning(cx: &mut Check, e: &Expr) {
    let writes = e.any(&|x| matches!(x.k, EK::Assign(..) | EK::Compound(..) | EK::IncDec(..)));
    if cx.unevaluated == 0 && !e.ty.is_error() && writes {
        cx.warning(e.span, "UNEV-4", "an unevaluated operand has a side effect, which does not occur");
    }
}

/// UNEV-3 (and STMT-4, COMMA-5, which link to it): warn about a
/// discarded-value expression whose value step has no effect.
pub fn no_effect_warning(cx: &mut Check, e: &crate::sem::tree::Expr, rule: &'static str, what: &str) {
    if !e.ty.is_error() && !has_effect(e) && cx.wflag(&["-Wall", "-Wunused-value"]) {
        cx.warning(e.span, rule, format!("{} has no effect", what));
    }
}
