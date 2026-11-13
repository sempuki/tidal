//! The check part of `expressions/primary`: literals, names, and
//! parenthesized expressions.

use crate::expressions::check::{bad, expr, first_node};
use crate::foundations::names::check as names;
use crate::lex::{FSuffix, ISize, Lit, Prefix, TK};
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr, EK};
use crate::sem::types::{FloatK, IntK, QType, Type};
use crate::sem::{EntKind, EntityId};
use crate::source::Span;
use crate::syntax::{Kid, Node};
use std::rc::Rc;

/// `primary`.
pub fn primary(cx: &mut Check, n: &Node) -> Expr {
    match n.kids.as_slice() {
        [Kid::Node(c)] => match c.prod {
            "literal" => literal(cx, c),
            "id_expression" => id_expression(cx, c),
            _ => expr(cx, c),
        },
        [k @ Kid::Tok(_)] if cx.text(k) == "this" => {
            let t = cx.unsupported(n.span, "'this' outside a class (classes come with build step 4)");
            Expr::new(EK::Int(0), t, Cat::PR, n.span)
        }
        [_, Kid::Node(e), _] => {
            let mut x = expr(cx, e);
            x.paren = true;
            x
        }
        _ => {
            let t = cx.unsupported(n.span, "a primary expression of this form");
            Expr::new(EK::Int(0), t, Cat::PR, n.span)
        }
    }
}

/// The type of an integer literal (PRIM-1, PRIM-2): the first type of
/// its list that holds the value.
fn int_literal(cx: &mut Check, value: u128, decimal: bool, unsigned: bool, size: ISize, span: Span) -> Expr {
    let tg = cx.u.target;
    let list: &[IntK] = match size {
        ISize::None => &[IntK::Int, IntK::Long, IntK::LongLong],
        ISize::L => &[IntK::Long, IntK::LongLong],
        ISize::LL => &[IntK::LongLong],
        ISize::Z => &[tg.ptrdiff_t()],
    };
    for &k in list {
        let mut cands = Vec::new();
        if !unsigned {
            cands.push(k);
        }
        if unsigned || !decimal {
            cands.push(k.unsigned_of());
        }
        for c in cands {
            if value <= tg.int_max(c) as u128 {
                return Expr::int(value as i128, QType::new(Type::Int(c)), span);
            }
        }
    }
    cx.error(span, "PRIM-2", "integer literal too large for its candidate types");
    bad(span)
}

/// `literal`.
pub fn literal(cx: &mut Check, n: &Node) -> Expr {
    let span = n.span;
    let toks: Vec<&crate::lex::Token> = n.kids.iter().filter_map(|k| cx.token(k)).collect();
    let Some(t) = toks.first() else { return bad(span) };
    match t.text.as_str() {
        "true" if t.kind == TK::Ident => return Expr::int(1, QType::bool(), span),
        "false" if t.kind == TK::Ident => return Expr::int(0, QType::bool(), span),
        "nullptr" if t.kind == TK::Ident => return Expr::int(0, QType::new(Type::NullPtr), span),
        _ => {}
    }
    match &t.lit {
        Some(Lit::Int { value, decimal, unsigned, size }) => int_literal(cx, *value, *decimal, *unsigned, *size, span),
        Some(Lit::Float { value, suffix }) => {
            let k = match suffix {
                FSuffix::F => FloatK::Float,
                FSuffix::L => FloatK::LongDouble,
                FSuffix::None => FloatK::Double,
                FSuffix::Ext => {
                    let t = cx.unsupported(span, "an extended floating-point literal");
                    return Expr::new(EK::Int(0), t, Cat::PR, span);
                }
            };
            let v = if k == FloatK::Float { *value as f32 as f64 } else { *value };
            // PRIM-13: out of range; the compilers accept with a warning.
            // A long double literal keeps the range of the wider format.
            if k != FloatK::LongDouble && v.is_infinite() {
                cx.warning(span, "PRIM-13", "floating literal out of range");
            }
            Expr::new(EK::Float(v), QType::new(Type::Float(k)), Cat::PR, span)
        }
        Some(Lit::Char { prefix, value, multi }) => {
            let k = if *multi {
                cx.warning(span, "PRIM-13", "multi-character literal");
                IntK::Int
            } else {
                match prefix {
                    Prefix::None => IntK::Char,
                    Prefix::U8 => IntK::Char8,
                    Prefix::U16 => IntK::Char16,
                    Prefix::U32 => IntK::Char32,
                    Prefix::Wide => IntK::WChar,
                }
            };
            Expr::int(*value, QType::new(Type::Int(k)), span)
        }
        Some(Lit::Str { .. }) => string(cx, &toks, span),
        Some(Lit::UserDefined { suffix, .. }) => {
            // PRIM-16: a suffix that no literal operator of the unit has.
            // A unit that declares a literal operator waits for build
            // step 4.
            let declared = cx.toks.windows(2).any(|w| w[0].text == "operator" && matches!(w[1].lit, Some(Lit::Str { .. } | Lit::UserDefined { .. })));
            if !declared {
                cx.error(span, "PRIM-16", format!("no literal operator has the suffix '{}'", suffix));
                return bad(span);
            }
            let t = cx.unsupported(span, "a user-defined literal (classes and operators come with build step 4)");
            Expr::new(EK::Int(0), t, Cat::PR, span)
        }
        None => bad(span),
    }
}

/// Adjacent string literals form one array of `const` characters with a
/// terminating zero (PRIM-6, PRIM-19).
fn string(cx: &mut Check, toks: &[&crate::lex::Token], span: Span) -> Expr {
    let mut prefix = Prefix::None;
    let mut units = Vec::new();
    for t in toks {
        if let Some(Lit::Str { prefix: p, units: u }) = &t.lit {
            if *p != Prefix::None {
                if prefix != Prefix::None && prefix != *p {
                    cx.error(t.span, "PRIM-19", "adjacent string literals have different prefixes");
                }
                prefix = *p;
            }
            units.extend_from_slice(u);
        }
    }
    units.push(0);
    let k = match prefix {
        Prefix::None => IntK::Char,
        Prefix::U8 => IntK::Char8,
        Prefix::U16 => IntK::Char16,
        Prefix::U32 => IntK::Char32,
        Prefix::Wide => IntK::WChar,
    };
    let el = QType { t: Type::Int(k), c: true, v: false };
    let ty = QType::new(Type::Array(Rc::new(el), Some(units.len() as u64)));
    Expr::new(EK::Str(Rc::new(units)), ty, Cat::L, span)
}

/// The name of an `id_expression` and its qualifier.
pub fn name_of<'a>(cx: &mut Check<'a>, n: &Node) -> Option<(names::Qual, &'a str, Span)> {
    let first = first_node(n)?;
    match first.prod {
        "unqualified_id" => match first.kids.as_slice() {
            [k @ Kid::Tok(_)] => Some((names::Qual::None, cx.text(k), cx.token(k)?.span)),
            _ => None,
        },
        "qualified_name" => {
            let q = names::nested_name(cx, first.child("nested_name")?);
            let u = first.child("unqualified_id")?;
            match u.kids.as_slice() {
                [k @ Kid::Tok(_)] => Some((q, cx.text(k), cx.token(k)?.span)),
                _ => None,
            }
        }
        _ => None,
    }
}

/// The entities a name finds.
pub fn find(cx: &mut Check, n: &Node) -> Option<(Vec<EntityId>, &'static str, String, Span)> {
    let (q, name, span) = name_of(cx, n)?;
    let found = match q {
        names::Qual::None => names::lookup(cx, name),
        names::Qual::Scope(s) => {
            let f = names::lookup_in(cx, s, name);
            if f.is_empty() {
                cx.not_found(span, "NAM-14", name);
                return Some((Vec::new(), "", name.to_string(), span));
            }
            f
        }
        _ => return Some((Vec::new(), "", name.to_string(), span)),
    };
    Some((found, if q == names::Qual::None { "NAM-8" } else { "NAM-14" }, name.to_string(), span))
}

/// `id_expression` as an operand.
pub fn id_expression(cx: &mut Check, n: &Node) -> Expr {
    let Some((found, rule, name, span)) = find(cx, n) else {
        let t = cx.unsupported(n.span, "an operator function, a destructor, or a template-id as a name");
        return Expr::new(EK::Int(0), t, Cat::PR, n.span);
    };
    if found.is_empty() {
        if !rule.is_empty() {
            if name == "__func__" {
                crate::declarations::functions::definitions::check::func_name_outside(cx, span);
            } else {
                cx.not_found(span, rule, &name);
            }
        }
        return bad(span);
    }
    if names::check_ambiguous(cx, &found, &name, span) {
        return bad(span);
    }
    entity_expr(cx, &found, &name, span)
}

/// The expression that names one of the entities a lookup found.
pub fn entity_expr(cx: &mut Check, found: &[EntityId], name: &str, span: Span) -> Expr {
    let funcs: Vec<EntityId> = found.iter().copied().filter(|&e| matches!(cx.u.entities[e].kind, EntKind::Func(_))).collect();
    let pick = found.iter().copied().find(|&e| !matches!(cx.u.entities[e].kind, EntKind::Enum(_))).unwrap_or(found[0]);
    crate::declarations::attributes::check::use_entity(cx, pick, span);
    match cx.u.entities[pick].kind.clone() {
        EntKind::Var(v) => {
            crate::declarations::initialization::forms::check::note_use(cx, pick, span);
            if let Some(Err(())) = crate::expressions::lambdas::check::capture_check(cx, pick, span) {
                return bad(span);
            }
            let (ty, cat) = match &v.ty.t {
                Type::Ref(inner, _) => ((**inner).clone(), Cat::L),
                _ => (v.ty.clone(), Cat::L),
            };
            if ty.is_error() {
                return bad(span);
            }
            Expr::new(EK::Var(pick), ty, cat, span)
        }
        EntKind::Func(_) => {
            if funcs.len() > 1 {
                // An overload set: the call or the target type chooses.
                return Expr::new(EK::Func(pick), QType::error(), Cat::L, span);
            }
            crate::declarations::functions::definitions::check::note_use(cx, pick, span);
            let ty = QType::new(Type::Func(cx.u.func(pick).unwrap().ty.clone()));
            Expr::new(EK::Func(pick), ty, Cat::L, span)
        }
        EntKind::Enumerator { en, value, ty } => {
            let ty = if cx.u.enums[en].complete { QType::new(Type::Enum(en)) } else { QType::new(Type::Int(cx.u.enums[en].underlying.unwrap_or(ty))) };
            Expr::int(value, ty, span)
        }
        EntKind::Unsupported(why) => {
            let t = cx.unsupported(span, format!("the name '{}': {}", name, why));
            Expr::new(EK::Int(0), t, Cat::PR, span)
        }
        EntKind::Typedef(_) | EntKind::Enum(_) | EntKind::Namespace(_) => {
            let t = cx.unsupported(span, format!("the name '{}' of a type or namespace as an expression", name));
            Expr::new(EK::Int(0), t, Cat::PR, span)
        }
    }
}

/// The overload set a name expression names, if it names functions.
pub fn overload_set(cx: &Check, e: &Expr) -> Vec<EntityId> {
    let EK::Func(id) = e.k else { return Vec::new() };
    let ent = &cx.u.entities[id];
    let found = names::lookup_in(cx, ent.scope, &ent.name);
    let set: Vec<EntityId> = found.into_iter().filter(|&f| matches!(cx.u.entities[f].kind, EntKind::Func(_))).collect();
    if set.is_empty() {
        vec![id]
    } else {
        set
    }
}

/// A name of an overload set, or its address, that no target type has
/// resolved yet (OVL-18): it has the error type until a target chooses.
pub fn unresolved_set(e: &Expr) -> bool {
    e.ty.is_error()
        && match &e.k {
            EK::Func(_) => true,
            EK::AddrOf(x) => matches!(x.k, EK::Func(_)) && x.ty.is_error(),
            _ => false,
        }
}

/// OVL-20 and the rules that name it for one context (AUTO-8, DTY-4,
/// ICS-6): an overload set where no target type chooses a function.
pub fn no_target(cx: &mut Check, e: &Expr, rule: &'static str) {
    cx.error(e.span, rule, "an overload set with no target type that chooses one function");
}
