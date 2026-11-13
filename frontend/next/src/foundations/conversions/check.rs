//! The check part of `foundations/conversions`: the standard
//! conversions, integer promotion, the usual arithmetic conversions, and
//! narrowing.

use crate::foundations::values::check::rvalue;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Conv, Expr, EK};
use crate::sem::types::{FloatK, IntK, QType, Type};
use std::rc::Rc;

/// CONV-13: an array becomes a pointer to its first element, a function
/// a pointer to itself.
pub fn decay(_cx: &mut Check, e: Expr) -> Expr {
    match e.ty.t.clone() {
        Type::Array(el, _) => e.conv(Conv::ArrayDecay, QType::ptr_to((*el).clone()), Cat::PR),
        Type::Func(_) => {
            if let crate::sem::tree::EK::Func(f) = e.k {
                crate::declarations::constexpr::check::address_of(_cx, f, e.span);
            }
            let t = QType::ptr_to(e.ty.clone());
            e.conv(Conv::FuncDecay, t, Cat::PR)
        }
        _ => e,
    }
}

/// CONV-8: `nullptr`, or an integer literal with the value zero.
pub fn is_null_constant(e: &Expr) -> bool {
    if e.ty.is_nullptr() {
        return true;
    }
    fn lit(e: &Expr) -> bool {
        match &e.k {
            EK::Int(0) => e.ty.is_integral() && !e.ty.is_bool(),
            EK::Conv(Conv::LToR | Conv::NoOp, _) => false,
            _ => false,
        }
    }
    lit(e)
}

/// The integer type an unscoped enumeration's values have in arithmetic.
fn enum_int(cx: &Check, t: &QType) -> Option<IntK> {
    match t.t {
        Type::Enum(e) if !cx.u.enums[e].scoped => cx.u.enums[e].underlying.or(Some(IntK::Int)),
        _ => None,
    }
}

/// The type an operand of this type promotes to (CONV-1).
pub fn promoted(cx: &Check, t: &QType) -> QType {
    let tg = cx.u.target;
    let k = match (t.int_kind(), enum_int(cx, t)) {
        (Some(k), _) => k,
        (None, Some(k)) => {
            // An unscoped enumeration promotes as its underlying type.
            k
        }
        _ => return t.unqual(),
    };
    if k.rank() < IntK::Int.rank() || matches!(k, IntK::WChar | IntK::Char16 | IntK::Char32 | IntK::Char8) {
        if tg.int_min(k) >= tg.int_min(IntK::Int) && tg.int_max(k) <= tg.int_max(IntK::Int) {
            return QType::int();
        }
        return QType::new(Type::Int(IntK::UInt));
    }
    QType::new(Type::Int(k))
}

/// Integer promotion of an operand that is already a prvalue.
pub fn promote(cx: &mut Check, e: Expr) -> Expr {
    let p = promoted(cx, &e.ty);
    if p.t == e.ty.t {
        return e;
    }
    e.conv(Conv::Arith, p, Cat::PR)
}

/// CONV-19: the common type of two arithmetic or unscoped enumeration
/// operands after promotion.
pub fn common_type(cx: &Check, a: &QType, b: &QType) -> QType {
    match (&a.t, &b.t) {
        (Type::Float(x), Type::Float(y)) => return QType::new(Type::Float(if x.rank() >= y.rank() { *x } else { *y })),
        (Type::Float(_), _) => return a.unqual(),
        (_, Type::Float(_)) => return b.unqual(),
        _ => {}
    }
    let (pa, pb) = (promoted(cx, a), promoted(cx, b));
    let (Some(x), Some(y)) = (pa.int_kind(), pb.int_kind()) else { return QType::error() };
    if x == y {
        return pa;
    }
    let tg = cx.u.target;
    if x.signed() == y.signed() {
        return if x.rank() >= y.rank() { pa } else { pb };
    }
    let (s, u) = if x.signed() { (x, y) } else { (y, x) };
    if u.rank() >= s.rank() {
        return QType::new(Type::Int(u));
    }
    if tg.int_max(s) >= tg.int_max(u) {
        return QType::new(Type::Int(s));
    }
    QType::new(Type::Int(s.unsigned_of()))
}

/// The usual arithmetic conversions on two prvalue operands: both
/// converted to their common type.
pub fn arithmetic(cx: &mut Check, l: Expr, r: Expr) -> (Expr, Expr, QType) {
    let t = common_type(cx, &l.ty, &r.ty);
    let to = |e: Expr| if e.ty.t == t.t { e } else { e.conv(Conv::Arith, t.clone(), Cat::PR) };
    (to(l), to(r), t)
}

/// Whether `from` converts to `to` by a qualification conversion and the
/// pointer conversions of CONV-9, CONV-10, and CONV-16 (CV-10).
pub fn pointer_convertible(from: &QType, to: &QType) -> bool {
    if (from.c && !to.c) || (from.v && !to.v) {
        return false;
    }
    if to.is_void() {
        return !from.is_func();
    }
    if from.t == to.t {
        return true;
    }
    match (&from.t, &to.t) {
        (Type::Ptr(_), Type::Ptr(_)) => qual_level(from, to, true),
        (Type::Func(a), Type::Func(b)) => {
            let mut a2 = (**a).clone();
            a2.noexcept = b.noexcept;
            a2 == **b && (a.noexcept || !b.noexcept)
        }
        (Type::Array(..), Type::Array(..)) => qual_level(from, to, true),
        _ => false,
    }
}

/// Two types that differ at most in cv-qualifiers and array bounds at
/// each level of pointers and arrays.
fn similar(a: &QType, b: &QType) -> bool {
    match (&a.t, &b.t) {
        (Type::Ptr(x), Type::Ptr(y)) | (Type::Array(x, _), Type::Array(y, _)) => similar(x, y),
        (x, y) => x == y,
    }
}

/// CV-10: one level of a qualification conversion.
fn qual_level(from: &QType, to: &QType, const_above: bool) -> bool {
    if (from.c && !to.c) || (from.v && !to.v) {
        return false;
    }
    let added = (to.c && !from.c) || (to.v && !from.v);
    if added && !const_above {
        return false;
    }
    match (&from.t, &to.t) {
        (Type::Ptr(a), Type::Ptr(b)) => qual_level(a, b, const_above && to.c),
        (Type::Array(a, n), Type::Array(b, m)) => (n == m || m.is_none()) && qual_level(a, b, const_above),
        (a, b) => a == b,
    }
}

/// The implicit conversion of an expression to a type (CONV-20), or the
/// reason it does not exist. `direct` is a direct-initialization, where
/// `nullptr` converts to `bool` (CONV-12).
pub fn implicit(cx: &mut Check, e: Expr, to: &QType, direct: bool) -> Result<Expr, String> {
    if e.ty.is_error() || to.is_error() {
        return Ok(Expr::new(e.k, to.clone(), Cat::PR, e.span));
    }
    let null = is_null_constant(&e);
    let e = rvalue(cx, e);
    let target = to.unqual();
    if e.ty.t == target.t {
        return Ok(e);
    }
    let from = e.ty.clone();
    match (&from.t, &target.t) {
        (_, Type::Void) => Ok(e.conv(Conv::ToVoid, target, Cat::PR)),
        (Type::Void, _) => Err("a void value does not convert".into()),
        (Type::NullPtr, Type::Int(IntK::Bool)) => {
            if direct {
                Ok(e.conv(Conv::ToBool, target, Cat::PR))
            } else {
                Err("nullptr converts to bool only in direct initialization (CONV-12)".into())
            }
        }
        (Type::Enum(en), _) if cx.u.enums[*en].scoped => Err(format!("the scoped enumeration {} does not convert implicitly", from.show())),
        (f, Type::Int(IntK::Bool)) if matches!(f, Type::Int(_) | Type::Float(_) | Type::Enum(_) | Type::Ptr(_)) => Ok(e.conv(Conv::ToBool, target, Cat::PR)),
        (_, Type::Enum(_)) => Err(format!("no implicit conversion from {} to an enumeration", from.show())),
        (Type::Int(_) | Type::Float(_) | Type::Enum(_), Type::Int(_) | Type::Float(_)) => Ok(e.conv(Conv::Arith, target, Cat::PR)),
        (_, Type::Ptr(tp)) => {
            if null {
                return Ok(e.conv(Conv::NullPtr, target, Cat::PR));
            }
            match &from.t {
                Type::Ptr(fp) if pointer_convertible(fp, tp) => Ok(e.conv(Conv::PtrNoOp, target, Cat::PR)),
                // CONV-9: to or from a pointer to void. CV-10 with
                // CONV-9: a qualification conversion that is not valid.
                Type::Ptr(fp) if fp.is_void() || tp.is_void() => Err(format!("no implicit conversion from {} to {} (CONV-9)", from.show(), to.show())),
                Type::Ptr(fp) if similar(fp, tp) => Err(format!("no implicit conversion from {} to {} (CV-10, CONV-9)", from.show(), to.show())),
                Type::Ptr(_) => Err(format!("no implicit conversion from {} to {}", from.show(), to.show())),
                _ => Err(format!("no implicit conversion from {} to a pointer (CONV-8)", from.show())),
            }
        }
        (_, Type::NullPtr) => {
            if null {
                Ok(e.conv(Conv::NullPtr, target, Cat::PR))
            } else {
                Err("no conversion to std::nullptr_t".into())
            }
        }
        (Type::Ptr(_) | Type::NullPtr, Type::Int(_) | Type::Float(_)) => Err("a pointer does not convert to an integer (CONV-18)".into()),
        _ => Err(format!("no implicit conversion from {} to {}", from.show(), to.show())),
    }
}

/// The rule that names a failed implicit conversion's reason, when the
/// reason is one of this page's rules.
pub fn rule_of(msg: &str) -> Option<&'static str> {
    if msg.contains("(CV-10, CONV-9)") {
        Some("CV-10")
    } else if msg.contains("CONV-12") {
        Some("CONV-12")
    } else if msg.contains("CONV-18") {
        Some("CONV-18")
    } else if msg.contains("CONV-8") {
        Some("CONV-8")
    } else {
        None
    }
}

/// The rule that a failed implicit conversion's reason names as its
/// cause, beside the rule of the context or of `rule_of`.
pub fn cause_of(msg: &str) -> Option<&'static str> {
    msg.contains("CONV-9)").then_some("CONV-9")
}

/// CONV-21: the conversion of `e` to `to` is narrowing. A constant whose
/// value fits the target is not narrowing.
pub fn is_narrowing(cx: &mut Check, e: &Expr, to: &QType) -> bool {
    let from = e.ty.nonref();
    let from_t = match &from.t {
        Type::Enum(en) if !cx.u.enums[*en].scoped => Type::Int(cx.u.enums[*en].underlying.unwrap_or(IntK::Int)),
        t => t.clone(),
    };
    let to_t = match &to.t {
        Type::Enum(en) => Type::Int(cx.u.enums[*en].underlying.unwrap_or(IntK::Int)),
        t => t.clone(),
    };
    let tg = cx.u.target;
    let value = || -> Option<crate::machine::Val> { crate::foundations::constant_evaluation::check::try_value(cx, e) };
    match (&from_t, &to_t) {
        (Type::Float(_), Type::Int(_)) => true,
        (Type::Int(_), Type::Float(k)) => match value() {
            Some(crate::machine::Val::Int(v)) if *k == FloatK::LongDouble => crate::foundations::types::run::F80::from_int(v).trunc() != Some(v),
            Some(crate::machine::Val::Int(v)) => {
                let f = v as f64;
                let r = if *k == FloatK::Float { f as f32 as f64 } else { f };
                r as i128 != v
            }
            _ => true,
        },
        (Type::Float(a), Type::Float(b)) if b.rank() < a.rank() => match value() {
            Some(crate::machine::Val::Float(f)) => !(f.is_nan() || f.is_infinite() || (f.abs() <= f32::MAX as f64 || *b != FloatK::Float)),
            Some(crate::machine::Val::Ext(x)) => x.is_finite() && !x.to_f64(*b == FloatK::Float).is_finite(),
            _ => true,
        },
        (Type::Int(a), Type::Int(b)) => {
            if *b == IntK::Bool && *a != IntK::Bool {
                return !matches!(value(), Some(crate::machine::Val::Int(0 | 1)));
            }
            if tg.int_min(*a) >= tg.int_min(*b) && tg.int_max(*a) <= tg.int_max(*b) {
                return false;
            }
            match value() {
                Some(crate::machine::Val::Int(v)) => !tg.fits(*b, v),
                _ => true,
            }
        }
        (Type::Ptr(_), Type::Int(IntK::Bool)) => true,
        _ => false,
    }
}

/// A contextual conversion to `bool` (the glossary's term). `rule` names
/// the construct's own rule for a failure.
pub fn contextual_bool(cx: &mut Check, e: Expr, rule: &'static str) -> Expr {
    if e.ty.is_error() {
        return e;
    }
    let t = e.ty.nonref();
    if let Type::Enum(en) = t.t {
        if cx.u.enums[en].scoped {
            crate::declarations::enumerations::check::scoped_to_bool(cx, e.span, rule);
            return Expr::new(e.k, QType::error(), Cat::PR, e.span);
        }
    }
    let span = e.span;
    match implicit(cx, e, &QType::bool(), true) {
        Ok(x) => x,
        Err(m) => {
            cx.error(span, rule, m);
            Expr::new(EK::Int(0), QType::error(), Cat::PR, span)
        }
    }
}

/// A pointer to `t` with the same cv-qualifiers.
pub fn ptr(t: &QType) -> QType {
    QType::new(Type::Ptr(Rc::new(t.clone())))
}
