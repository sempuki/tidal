//! The check part of `overloading/conversion-sequences`, for arguments
//! and parameters that are not classes (build step 3; step 4 adds the
//! user-defined sequences). A class operand gives no sequence here, and
//! the caller reports it as unsupported.

use crate::foundations::conversions::check as conv;
use crate::sem::check::Check;
use crate::sem::tree::{Cat, Expr};
use crate::sem::types::{FloatK, IntK, QType, Type};

/// The rank of a standard sequence (ICS-2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Exact,
    Promotion,
    Conversion,
}

/// An implicit conversion sequence of a non-class argument.
#[derive(Clone, Debug, PartialEq)]
pub struct Ics {
    pub rank: Rank,
    /// A qualification adjustment, which loses a tie to the identity
    /// (ICS-26).
    pub qual: bool,
    /// A reference binding: rvalue reference, binds an rvalue, and the
    /// referred type with its cv-qualifiers (ICS-28 to ICS-30).
    pub binds: Option<(bool, bool, QType)>,
    /// The argument matches an ellipsis (ICS-6).
    pub ellipsis: bool,
}

/// ICS-2: the rank of the conversion from a prvalue type to a target.
fn rank_of(cx: &Check, from: &QType, to: &QType, null: bool) -> Option<(Rank, bool)> {
    let (f, t) = (from.unqual(), to.unqual());
    if f.t == t.t {
        return Some((Rank::Exact, false));
    }
    match (&f.t, &t.t) {
        (Type::Int(_) | Type::Enum(_), Type::Int(_)) | (Type::Float(_), Type::Float(_)) => {
            if let Type::Enum(e) = f.t {
                if cx.u.enums[e].scoped {
                    return None;
                }
                // ICS-39: an enumeration with a fixed type promotes to it.
                if cx.u.enums[e].fixed && cx.u.enums[e].underlying.map(|k| Type::Int(k)) == Some(t.t.clone()) {
                    return Some((Rank::Promotion, false));
                }
            }
            if t.is_bool() && !f.is_bool() {
                return Some((Rank::Conversion, false));
            }
            if conv::promoted(cx, &f).t == t.t {
                return Some((Rank::Promotion, false));
            }
            if f.t == Type::Float(FloatK::Float) && t.t == Type::Float(FloatK::Double) {
                return Some((Rank::Promotion, false));
            }
            Some((Rank::Conversion, false))
        }
        (Type::Int(_) | Type::Enum(_), Type::Float(_)) | (Type::Float(_), Type::Int(_)) => {
            if let Type::Enum(e) = f.t {
                if cx.u.enums[e].scoped {
                    return None;
                }
            }
            Some((Rank::Conversion, false))
        }
        (Type::Ptr(_) | Type::NullPtr, Type::Int(IntK::Bool)) => Some((Rank::Conversion, false)),
        (_, Type::Ptr(_)) if null => Some((Rank::Conversion, false)),
        (_, Type::NullPtr) if null => Some((Rank::Exact, false)),
        (Type::Ptr(a), Type::Ptr(b)) => {
            if !conv::pointer_convertible(a, b) {
                return None;
            }
            if b.is_void() && !a.is_void() {
                return Some((Rank::Conversion, false));
            }
            if a.t == b.t || matches!((&a.t, &b.t), (Type::Ptr(_), Type::Ptr(_))) {
                return Some((Rank::Exact, true));
            }
            // A function pointer that loses `noexcept` is an exact match
            // (ICS-2's function pointer conversion).
            Some((Rank::Exact, true))
        }
        _ => None,
    }
}

/// The sequence from an argument to a parameter type, or None.
pub fn sequence(cx: &Check, arg: &Expr, param: &QType) -> Option<Ics> {
    if arg.ty.is_error() || param.is_error() {
        return Some(Ics { rank: Rank::Exact, qual: false, binds: None, ellipsis: false });
    }
    let null = conv::is_null_constant(arg);
    if let Type::Ref(inner, rv) = &param.t {
        let lvalue = arg.cat == Cat::L;
        let at = arg.ty.clone();
        // REF-1: a direct binding to an lvalue of a reference-compatible
        // type.
        let compatible = at.unqual().t == inner.unqual().t && (!at.c || inner.c) && (!at.v || inner.v);
        if compatible && ((lvalue && !rv) || (!lvalue && (*rv || (inner.c && !inner.v)))) {
            let qual = (inner.c && !at.c) || (inner.v && !at.v);
            return Some(Ics { rank: Rank::Exact, qual, binds: Some((*rv, !lvalue, (**inner).clone())), ellipsis: false });
        }
        if at.is_func() && inner.t == at.t {
            return Some(Ics { rank: Rank::Exact, qual: false, binds: Some((*rv, false, (**inner).clone())), ellipsis: false });
        }
        // REF-2, REF-3: a temporary, for a const lvalue reference or an
        // rvalue reference, from a value of another type.
        if (*rv || (inner.c && !inner.v)) && !(compatible && lvalue && *rv) {
            let from = crate::declarations::auto::check::decay(&at);
            let (rank, _) = rank_of(cx, &from, inner, null)?;
            return Some(Ics { rank, qual: false, binds: Some((*rv, true, (**inner).clone())), ellipsis: false });
        }
        return None;
    }
    let from = crate::declarations::auto::check::decay(&arg.ty);
    let (rank, qual) = rank_of(cx, &from, param, null)?;
    Some(Ics { rank, qual, binds: None, ellipsis: false })
}

/// ICS-21 to ICS-30: whether `a` is better than `b`.
pub fn better(a: &Ics, b: &Ics) -> bool {
    if a.ellipsis != b.ellipsis {
        return !a.ellipsis;
    }
    if a.rank != b.rank {
        return a.rank < b.rank;
    }
    if a.qual != b.qual && a.binds.is_none() && b.binds.is_none() {
        return !a.qual;
    }
    if let (Some((arv, arvalue, at)), Some((brv, brvalue, bt))) = (&a.binds, &b.binds) {
        if *arvalue && *brvalue && arv != brv {
            return *arv;
        }
        if at.t == bt.t {
            let ac = at.c as u8 + at.v as u8;
            let bc = bt.c as u8 + bt.v as u8;
            if ac < bc && (!at.c || bt.c) && (!at.v || bt.v) {
                return true;
            }
        } else if adjusts(at, bt) {
            // ICS-46: `int* const&` over `const int* const&`, and an
            // array of known bound over one of unknown bound.
            return true;
        }
    }
    false
}

/// Whether `a` converts to `b` by a qualification adjustment below the
/// top level: cv-qualifiers added under a pointer, or the bound of an
/// array dropped (ICS-46).
fn adjusts(a: &QType, b: &QType) -> bool {
    match (&a.t, &b.t) {
        (Type::Ptr(x), Type::Ptr(y)) => {
            (!x.c || y.c) && (!x.v || y.v) && (x.unqual().t == y.unqual().t && (x.c != y.c || x.v != y.v) || adjusts(x, y) && y.c)
        }
        (Type::Array(x, Some(_)), Type::Array(y, None)) => x == y,
        _ => false,
    }
}
