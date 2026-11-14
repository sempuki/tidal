//! The run part of `foundations/storage`: the byte view of objects.
//!
//! A pointer whose type does not match the object at its address is a
//! view: the byte offset in its complete object, the size of its
//! pointee, and whether the pointee is a byte type (STG-13). A pointer
//! to `char`, `unsigned char`, or `std::byte` reads and writes the bytes
//! of the object representation (STG-26). An access through any other
//! view reaches no object (STG-23, STG-29), and its arithmetic counts the
//! object as an array of one (ADD-14). Each object gets an address when
//! the program first needs one, so a pointer converts to an integer and
//! back (CAST-16) and its bytes are its address (STG-26).

use crate::machine::{ub, unsup, Machine, Ptr, Val, View, R};
use crate::sem::types::{FloatK, IntK, QType, Type};

/// The size of a type in bytes on the target.
pub fn size(m: &Machine, ty: &QType) -> i64 {
    m.target.size_of(m.u, ty).unwrap_or(1) as i64
}

/// `char`, `unsigned char`, or `std::byte`: the types whose glvalues
/// read and write the bytes of any object (STG-14).
pub fn is_byte(m: &Machine, ty: &QType) -> bool {
    match &ty.t {
        Type::Int(IntK::Char | IntK::UChar) => true,
        Type::Enum(e) => m.u.enums.get(*e).is_some_and(|i| &*i.name == "byte" && i.underlying == Some(IntK::UChar)),
        _ => false,
    }
}

/// The byte offset of a place in its complete object. An index into an
/// object that is not an array counts the object as an array of one.
pub fn offset(m: &Machine, p: &Ptr) -> i64 {
    if let Some(v) = &p.view {
        return v.off;
    }
    let mut t = m.objs[p.obj].ty.clone();
    let mut off = 0;
    for &i in &p.path {
        match t.t.clone() {
            Type::Array(e, _) => {
                off += i * size(m, &e);
                t = (*e).clone();
            }
            Type::Class(c) => {
                let offs = m.target.layout(m.u, c).map(|l| l.2).unwrap_or_default();
                off += offs.get(i as usize).copied().unwrap_or(0) as i64;
                t = m.u.field_types(c).get(i as usize).cloned().unwrap_or(t);
            }
            _ => off += i * size(m, &t),
        }
    }
    off
}

/// The same type up to cv-qualifiers, or the signed or unsigned integer
/// type that matches it (STG-14).
fn same_or_counterpart(a: &QType, b: &QType) -> bool {
    match (&a.unqual().t, &b.unqual().t) {
        (Type::Int(x), Type::Int(y)) => x == y || (x.unsigned_of() == y.unsigned_of() && *x != IntK::Char && *y != IntK::Char && *x != IntK::Bool),
        (x, y) => x == y,
    }
}

/// A place of type `want` (or its signed or unsigned counterpart, STG-14)
/// at byte `off` of the object, as a typed pointer, or None. The
/// position one past an array element counts.
pub fn reach(m: &Machine, obj: usize, off: i64, want: &QType) -> Option<Ptr> {
    fn walk(m: &Machine, t: &QType, off: i64, want: &QType, path: &mut Vec<i64>) -> Option<bool> {
        if off == 0 && same_or_counterpart(t, want) {
            return Some(false);
        }
        if let Type::Class(c) = &t.t {
            let offs = m.target.layout(m.u, *c)?.2;
            for (k, ft) in m.u.field_types(*c).iter().enumerate() {
                let fo = offs[k] as i64;
                if off >= fo && off < fo + size(m, ft).max(1) {
                    path.push(k as i64);
                    if let Some(a) = walk(m, ft, off - fo, want, path) {
                        return Some(a);
                    }
                    path.pop();
                }
            }
            return None;
        }
        let Type::Array(e, Some(n)) = &t.t else { return None };
        let es = size(m, e);
        if es == 0 {
            return None;
        }
        if same_or_counterpart(e, want) && off % es == 0 && off / es <= *n as i64 {
            path.push(off / es);
            return Some(true);
        }
        let k = off / es;
        if k < 0 || k >= *n as i64 {
            return None;
        }
        path.push(k);
        walk(m, e, off % es, want, path)
    }
    let mut path = Vec::new();
    let array = walk(m, &m.objs[obj].ty, off, want, &mut path)?;
    Some(Ptr { obj, path, array, view: None })
}

/// A pointer converted to a pointer to `to` (CAST rules, STG-29): the
/// typed place of that type at its address, or a view.
pub fn convert(m: &Machine, p: Ptr, to: &QType) -> Ptr {
    if to.is_void() || to.is_func() || p.obj >= m.objs.len() {
        return p;
    }
    let off = offset(m, &p);
    if let Some(q) = reach(m, p.obj, off, to) {
        return q;
    }
    Ptr { obj: p.obj, path: Vec::new(), array: false, view: Some(View { off, size: size(m, to).max(1), bytes: is_byte(m, to), origin: off }) }
}

/// A view moved by `d` elements: over the bytes of the complete object
/// for a byte type (STG-13), or within the object and one past it for
/// any other type (ADD-14).
pub fn moved(m: &Machine, p: &Ptr, d: i64) -> R<Ptr> {
    let v = p.view.clone().unwrap();
    let off = v.off + d * v.size;
    let ok = if v.bytes { off >= 0 && off <= size(m, &m.objs[p.obj].ty) } else { off >= v.origin && off <= v.origin + v.size };
    if !ok {
        return if v.bytes {
            ub("ADD-10", "the byte pointer leaves its object")
        } else {
            ub("ADD-14", "a pointer whose type differs from the object's moves past the object")
        };
    }
    let mut q = p.clone();
    q.view = Some(View { off, ..v });
    Ok(q)
}

/// The byte offsets of two pointers into one object, for a difference or
/// a comparison, when either is a view.
pub fn offsets(m: &Machine, a: &Ptr, b: &Ptr) -> Option<(i64, i64)> {
    (a.obj == b.obj).then(|| (offset(m, a), offset(m, b)))
}

/// The read of a place that a view names: one byte of the object
/// representation through a byte type (STG-13), the object of a listed
/// type at the address, or undefined (STG-23).
pub fn read(m: &mut Machine, p: &Ptr, ty: &QType) -> R<Val> {
    let v = p.view.clone().unwrap();
    if is_byte(m, ty) {
        let bytes = encode(m, &m.objs[p.obj].ty.clone(), &m.objs[p.obj].val.clone())?;
        let Some(b) = bytes.get(v.off as usize) else { return crate::foundations::objects::run::outside(m, p) };
        let Some(b) = *b else {
            return crate::declarations::initialization::forms::run::read_no_value("the byte");
        };
        return Ok(Val::Int(match ty.unqual().t {
            Type::Int(k) => m.target.wrap(k, b as i128),
            _ => b as i128,
        }));
    }
    match reach(m, p.obj, v.off, ty) {
        Some(q) => crate::foundations::values::run::read(m, &q, ty),
        None => no_object(ty),
    }
}

/// The write of a place that a view names.
pub fn write(m: &mut Machine, p: &Ptr, ty: &QType, val: Val) -> R<()> {
    let v = p.view.clone().unwrap();
    if is_byte(m, ty) {
        let Val::Int(x) = val else { return unsup("a byte write of this value") };
        let oty = m.objs[p.obj].ty.clone();
        let mut bytes = encode(m, &oty, &m.objs[p.obj].val.clone())?;
        let Some(slot) = bytes.get_mut(v.off as usize) else { return crate::foundations::objects::run::outside(m, p) };
        *slot = Some(x as u8);
        let new = decode(m, &oty, &bytes)?;
        m.objs[p.obj].val = new;
        return Ok(());
    }
    match reach(m, p.obj, v.off, ty) {
        Some(q) => crate::machine::eval::write(m, &q, ty, val),
        None => no_object(ty),
    }
}

/// STG-23: no object of a type that STG-14 lists for the glvalue's type
/// is at the address.
fn no_object<T>(ty: &QType) -> R<T> {
    ub("STG-23", format!("an access through a glvalue of type {} where no object of a listed type is", ty.show()))
}

// ---------------------------------------------------------------------
// Addresses

/// The address of an object, which it gets when the program first needs
/// one. Objects lie apart, each at a multiple of 16.
pub fn base(m: &mut Machine, obj: usize) -> u64 {
    if let Some(&a) = m.addrs.get(&obj) {
        return a;
    }
    let a = m.next_addr;
    let n = size(m, &m.objs[obj].ty).max(1) as u64;
    m.next_addr = a + (n + 1).div_ceil(16) * 16;
    m.addrs.insert(obj, a);
    a
}

/// The address of a function: its own region, 16 bytes apart.
pub fn function_address(id: crate::sem::EntityId) -> u64 {
    0x40_0000 + 16 * id as u64
}

/// CAST-16: the integer for a pointer value.
pub fn address(m: &mut Machine, v: &Val) -> Option<u64> {
    match v {
        Val::Null => Some(0),
        Val::Addr(a) => Some(*a),
        Val::Fn(id) => Some(function_address(*id)),
        Val::Ptr(p) => {
            let off = offset(m, p);
            Some((base(m, p.obj) as i64 + off) as u64)
        }
        _ => None,
    }
}

/// CAST-16: the pointer to `to` for an integer: the place at that
/// address in an object that has an address, or the position one past
/// it, a function, or an address with no object (STG-15).
pub fn from_address(m: &Machine, a: u64, to: &QType) -> Val {
    if a == 0 {
        return Val::Null;
    }
    if a >= 0x40_0000 && (a - 0x40_0000) % 16 == 0 {
        let id = ((a - 0x40_0000) / 16) as usize;
        if m.u.func(id).is_some() {
            return Val::Fn(id);
        }
    }
    for (&obj, &b) in &m.addrs {
        let n = size(m, &m.objs[obj].ty) as u64;
        if a >= b && a <= b + n {
            let p = Ptr { obj, path: Vec::new(), array: false, view: Some(View { off: (a - b) as i64, size: 1, bytes: true, origin: (a - b) as i64 }) };
            return Val::Ptr(convert(m, p, to));
        }
    }
    Val::Addr(a)
}

// ---------------------------------------------------------------------
// Object representation (STG-26)

/// The bytes of a value of type `ty`, least significant first, with
/// None for a byte that has no value.
pub fn encode(m: &mut Machine, ty: &QType, v: &Val) -> R<Vec<Option<u8>>> {
    let n = size(m, ty) as usize;
    if matches!(v, Val::Uninit) {
        return Ok(vec![None; n]);
    }
    let int = |x: u128, n: usize| (0..n).map(|k| Some((x >> (8 * k)) as u8)).collect::<Vec<_>>();
    Ok(match (&ty.t, v) {
        (Type::Class(c), Val::Agg(xs)) => {
            if has_bitfield(m, *c) {
                return unsup("the bytes of a class with a bit-field");
            }
            let offs = m.target.layout(m.u, *c).map(|l| l.2).unwrap_or_default();
            let tys = m.u.field_types(*c);
            // Padding bytes have unspecified values, which a byte read
            // can copy (STG-26).
            let mut out = vec![Some(0); n];
            for (k, x) in xs.iter().enumerate() {
                // A union member that is not active has no bytes of its own.
                if *x == Val::Inactive {
                    continue;
                }
                let b = encode(m, &tys[k], x)?;
                let at = offs[k] as usize;
                out[at..at + b.len()].clone_from_slice(&b);
            }
            out
        }
        (Type::Array(e, _), Val::Agg(xs)) => {
            let mut out = Vec::new();
            for x in xs {
                out.extend(encode(m, e, x)?);
            }
            out.resize(n, None);
            out
        }
        (Type::Int(_) | Type::Enum(_), Val::Int(x)) => int(*x as u128, n),
        (Type::Float(FloatK::Float), Val::Float(f)) => int((*f as f32).to_bits() as u128, n),
        (Type::Float(_), Val::Float(f)) => int(f.to_bits() as u128, n),
        (Type::Float(_), Val::Ext(x)) => {
            let mut out = int(x.to_bits(), 10);
            out.resize(n, Some(0));
            out
        }
        (Type::Ptr(_) | Type::NullPtr, v) => match address(m, v) {
            Some(a) => int(a as u128, n),
            None => return unsup("the bytes of this pointer value"),
        },
        _ => return unsup(format!("the bytes of a value of type {}", ty.show())),
    })
}

/// The value of type `ty` that bytes represent: a value with no value
/// when a byte has none.
pub fn decode(m: &Machine, ty: &QType, bytes: &[Option<u8>]) -> R<Val> {
    if let Type::Class(c) = &ty.t {
        if has_bitfield(m, *c) || m.u.classes[*c].key == crate::sem::ClassKey::Union {
            return unsup("a value of a union or of a class with a bit-field from bytes");
        }
        let offs = m.target.layout(m.u, *c).map(|l| l.2).unwrap_or_default();
        let mut out = Vec::new();
        for (k, t) in m.u.field_types(*c).iter().enumerate() {
            let at = offs[k] as usize;
            out.push(decode(m, t, &bytes[at..at + size(m, t) as usize])?);
        }
        return Ok(Val::Agg(out));
    }
    if let Type::Array(e, Some(k)) = &ty.t {
        let es = size(m, e) as usize;
        let mut out = Vec::new();
        for i in 0..*k as usize {
            out.push(decode(m, e, &bytes[i * es..(i + 1) * es])?);
        }
        return Ok(Val::Agg(out));
    }
    if bytes.iter().any(|b| b.is_none()) {
        return Ok(Val::Uninit);
    }
    let raw: u128 = bytes.iter().enumerate().fold(0, |acc, (k, b)| acc | (b.unwrap() as u128) << (8 * k));
    let n = bytes.len();
    Ok(match &ty.t {
        Type::Int(k) => Val::Int(m.target.wrap(*k, raw as i128)),
        Type::Enum(e) => {
            let k = m.u.enums.get(*e).and_then(|i| i.underlying).unwrap_or(IntK::Int);
            Val::Int(m.target.wrap(k, raw as i128))
        }
        Type::Float(FloatK::Float) => Val::Float(f32::from_bits(raw as u32) as f64),
        Type::Float(FloatK::Double) => Val::Float(f64::from_bits(raw as u64)),
        Type::Float(FloatK::LongDouble) => Val::Ext(crate::foundations::types::run::F80::from_bits(raw & ((1 << 80) - 1))),
        Type::Ptr(p) => from_address(m, if n == 4 { raw as u32 as u64 } else { raw as u64 }, p),
        Type::NullPtr => Val::Null,
        _ => return unsup(format!("a value of type {} from bytes", ty.show())),
    })
}

/// STG-22: a call of a non-static member function `f` through a pointer
/// to storage that holds no object of its class.
pub fn no_object_call<T>(m: &Machine, f: crate::sem::EntityId) -> R<T> {
    ub("STG-22", format!("a call of the member function '{}' on storage that holds no object of its class", m.u.entities[f].name))
}

/// Whether a class has a bit-field, in it or in a member.
fn has_bitfield(m: &Machine, c: crate::sem::types::ClassId) -> bool {
    m.u.classes[c].fields.iter().any(|&f| m.u.field(f).is_some_and(|x| x.width.is_some() || x.ty.base_elem().class().is_some_and(|d| has_bitfield(m, d))))
}
