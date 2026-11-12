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
    if e.ty.is_error() {
        return QType::error();
    }
    let declared = match &e.k {
        EK::Var(id) => cx.u.var(*id).map(|v| v.ty.clone()),
        EK::Func(id) => cx.u.func(*id).map(|f| QType::new(Type::Func(f.ty.clone()))),
        _ => None,
    };
    let unparenthesized_name = matches!(c.prod, "primary" | "id_expression") && !e.paren;
    decltype_with(&e, if unparenthesized_name { declared } else { None })
}
