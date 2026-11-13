//! The analysis of an expression node: each production goes to the
//! check part of its page. The chapter's index page has no rules, so this
//! module names none. After a page makes the typed node, the node checks
//! of a proposal run on it (`DESIGN-rewrite.md` section 7).

use crate::expressions::{
    additive, assignment, bitwise, casts, comma, comparison, conditional, decltype, lambdas, logical, multiplicative, postfix,
    primary, shift, unary,
};
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, EK};
use crate::sem::types::QType;
use crate::syntax::{Kid, Node};

/// The typed expression of a node.
pub fn expr(cx: &mut Check, n: &Node) -> Expr {
    let e = match n.prod {
        "primary" => primary::check::primary(cx, n),
        "id_expression" => primary::check::id_expression(cx, n),
        "literal" => primary::check::literal(cx, n),
        "lambda_expr" => lambdas::check::lambda_expr(cx, n),
        "postfix_expr" => postfix::check::postfix_expr(cx, n),
        "unary_expr" => unary::check::unary_expr(cx, n),
        "cast_expr" => casts::check::cast_expr(cx, n),
        "named_cast" => casts::check::named_cast(cx, n),
        "functional_cast" => casts::check::functional_cast(cx, n),
        "mul_expr" => multiplicative::check::mul_expr(cx, n),
        "add_expr" => additive::check::add_expr(cx, n),
        "shift_expr" => shift::check::shift_expr(cx, n),
        "relational_expr" | "equality_expr" | "three_way_expr" => comparison::check::comparison(cx, n),
        "and_expr" | "xor_expr" | "or_expr" => bitwise::check::bitwise(cx, n),
        "logical_and_expr" | "logical_or_expr" => logical::check::logical(cx, n),
        "conditional_expr" => conditional::check::conditional_expr(cx, n),
        "assignment_expr" => assignment::check::assignment_expr(cx, n),
        "expression" => comma::check::expression(cx, n),
        "argument" | "init_clause" => match first_node(n) {
            Some(c) => expr(cx, c),
            None => error(cx, n),
        },
        "braced_init" => {
            let t = cx.unsupported(n.span, "a braced list as an operand");
            Expr::new(EK::Int(0), t, Cat::PR, n.span)
        }
        "decltype_spec" | "decltype_expr" => {
            let t = decltype::check::decltype_spec(cx, n);
            Expr::new(EK::Int(0), t, Cat::PR, n.span)
        }
        _ => {
            let t = cx.unsupported(n.span, format!("an expression of the form {}", n.prod));
            Expr::new(EK::Int(0), t, Cat::PR, n.span)
        }
    };
    cx.node_checks(n, &e);
    e
}

/// The first child node.
pub fn first_node(n: &Node) -> Option<&Node> {
    n.kids.iter().find_map(|k| match k {
        Kid::Node(c) => Some(c),
        _ => None,
    })
}

/// The child nodes of a node.
pub fn nodes(n: &Node) -> Vec<&Node> {
    n.kids
        .iter()
        .filter_map(|k| match k {
            Kid::Node(c) => Some(c),
            _ => None,
        })
        .collect()
}

/// The operator token of a binary node: its first direct token.
pub fn operator<'a>(cx: &Check<'a>, n: &Node) -> &'a str {
    cx.first_word(n)
}

/// An expression of the error type, for a construct after an error.
pub fn error(_cx: &mut Check, n: &Node) -> Expr {
    Expr::new(EK::Int(0), QType::error(), Cat::PR, n.span)
}

/// An expression with the error type that keeps the span.
pub fn bad(span: crate::source::Span) -> Expr {
    Expr::new(EK::Int(0), QType::error(), Cat::PR, span)
}

/// The left and right operands of a binary node.
pub fn operands(cx: &mut Check, n: &Node) -> Option<(Expr, Expr)> {
    let ns = nodes(n);
    if ns.len() != 2 {
        return None;
    }
    let l = expr(cx, ns[0]);
    let r = expr(cx, ns[1]);
    Some((l, r))
}

/// The deprecated mixes of the usual arithmetic conversions, which an
/// implementation should warn about: operands of two different
/// unscoped enumeration types, and (when `with_float`) an unscoped
/// enumeration with a floating-point operand. `rule` is the operator
/// page's rule (ADD-16, BIT-8, CMP-21, MUL-16).
pub fn mixed_enumerations(cx: &mut Check, l: &Expr, r: &Expr, span: crate::source::Span, rule: &'static str, with_float: bool) {
    use crate::sem::types::Type;
    let unscoped = |t: &QType| matches!(t.t, Type::Enum(e) if !cx.u.enums[e].scoped);
    match (&l.ty.t, &r.ty.t) {
        (Type::Enum(a), Type::Enum(b)) if a != b && unscoped(&l.ty) && unscoped(&r.ty) => {
            cx.warning(span, rule, "operands of two different enumeration types are deprecated");
        }
        (Type::Enum(_), Type::Float(_)) | (Type::Float(_), Type::Enum(_)) if with_float && (unscoped(&l.ty) || unscoped(&r.ty)) => {
            cx.warning(span, rule, "an enumeration operand with a floating-point operand is deprecated");
        }
        _ => {}
    }
}
