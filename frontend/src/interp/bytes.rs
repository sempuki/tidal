// The byte view of objects (STG-11 to STG-16, STG-23, STG-26, CAST-6,
// CAST-16, CLS-19).
//
// The value view (`Val`) stays the record of each object. The bytes of an
// object are computed from its value when a program reads them, and a
// write of a byte changes the value of the scalar that holds the byte.
// Bytes that the value view cannot hold (padding, and the bytes of a
// scalar that has no value yet) are kept in `shadow`.
//
// The text fixes the order of the bytes of an integer (STG-26), two's
// complement (TYP-4), and the floating formats (MUL-20). It does not fix
// the bytes of a pointer. The interpreter gives each object an address
// when a program first needs one (a pointer converted to an integer, or
// the bytes of a pointer), and a pointer's bytes are its address in the
// order of STG-26.

use super::*;

/// One byte of an object representation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Byte {
    Known(u8),
    /// A byte with no value: padding, or a byte of a scalar with no value.
    Indet,
}

/// The first address the interpreter gives an object, and the first
/// address of a function.
const FIRST_ADDR: u64 = 0x0800_0000;
const FIRST_FN_ADDR: u64 = 0x0040_0000;

impl<'a> Machine<'a> {
    /// `char`, `unsigned char`, or `std::byte` (STG-13, STG-14).
    pub fn is_byte_type(&self, t: &Ty) -> bool {
        match t {
            Ty::Char | Ty::UChar => true,
            Ty::Enum(e) => self.p.enums[*e].name == "byte",
            _ => false,
        }
    }

    pub fn size_of(&self, t: &Ty) -> R<i64> {
        match self.p.sizeof_ty(t) {
            Some(s) => Ok(s as i64),
            None => unsup("bytes of a type whose layout the front end does not compute"),
        }
    }

    /// The parts of a class object with their offsets: the value index,
    /// the offset, and the type. Members follow TYP-12; the pointer to the
    /// table of virtual functions comes first, as the front end's `sizeof`
    /// assumes.
    pub fn class_parts(&self, c: usize) -> R<Vec<(usize, i64, QTy)>> {
        let ci = &self.p.classes[c];
        let union = ci.key == 2;
        let mut off: i64 = if ci.has_virtual { if ilp32() { 4 } else { 8 } } else { 0 };
        let mut out = Vec::new();
        let nb = ci.bases.len();
        if nb > 0 {
            // CLS-19, CLS-30: an empty base shares the address of the
            // object. Other bases come first, in order. The text gives no
            // other placement of base subobjects (STG-25).
            if ci.has_virtual || ci.bases.iter().any(|b| b.virt) {
                return unsup("bytes of a class with virtual functions and base classes");
            }
            for (i, b) in ci.bases.iter().enumerate() {
                let Some(bc) = b.class else { return unsup("bytes of a class with a base of unknown type") };
                let t = QTy::new(Ty::Class(bc));
                if self.empty_class(bc, 0) {
                    out.push((i, 0, t));
                } else {
                    let s = self.size_of(&t.t)?;
                    let a = self.p.alignof_ty(&t.t).unwrap_or(1) as i64;
                    off = (off + a - 1) / a * a;
                    out.push((i, off, t));
                    off += s;
                }
            }
        }
        let mut empties: Vec<(usize, i64)> = Vec::new();
        for (_, fi) in self.fields(c) {
            if fi.bits.is_some() {
                return unsup("bytes of a class with bit-fields");
            }
            if fi.ty.t.is_ref() {
                return unsup("bytes of a class with a reference member");
            }
            if fi.nua && !union {
                if let Ty::Class(fc) = fi.ty.t {
                    if self.p.classes[fc].fields.is_empty() && self.p.classes[fc].bases.is_empty() && !self.p.classes[fc].has_virtual {
                        // CLS-31, ATTR-9: no bytes of its own; two of one
                        // type get different offsets.
                        let mut k = 0i64;
                        while empties.iter().any(|&(x, o)| x == fc && o == k) {
                            k += 1;
                        }
                        empties.push((fc, k));
                        out.push((nb + fi.index, k, fi.ty.clone()));
                        continue;
                    }
                }
            }
            let s = self.size_of(&fi.ty.t)?;
            let a = self.p.alignof_ty(&fi.ty.t).unwrap_or(1).max(fi.align) as i64;
            if union {
                out.push((nb + fi.index, 0, fi.ty.clone()));
            } else {
                off = (off + a - 1) / a * a;
                out.push((nb + fi.index, off, fi.ty.clone()));
                // CLS-31: the padding at the end of a no_unique_address
                // member can hold the next members.
                let used = match fi.ty.t {
                    Ty::Class(fc) if fi.nua && self.p.tail_reusable(fc) => self.p.class_layout(fc).map(|x| x.2 as i64).unwrap_or(s),
                    _ => s,
                };
                off += used;
            }
        }
        Ok(out)
    }

    /// A class with no non-static data members, no virtual functions, and
    /// only such bases.
    fn empty_class(&self, c: usize, depth: u32) -> bool {
        let ci = &self.p.classes[c];
        depth < 16 && ci.fields.iter().all(|&f| !matches!(self.p.decls[f].k, DK::Field(_))) && !ci.has_virtual && ci.bases.iter().all(|b| !b.virt && b.class.is_some_and(|bc| self.empty_class(bc, depth + 1)))
    }

    /// The offset of the subobject at `path` in the complete object `o`,
    /// and its type.
    pub fn path_offset(&self, o: usize, path: &[usize]) -> R<(i64, QTy)> {
        let Some(mut t) = self.obj_ty(o) else { return unsup("bytes of an object of unknown type") };
        let mut off = 0i64;
        for &i in path {
            t = match &t.t {
                Ty::Array(el, _) => {
                    off += i as i64 * self.size_of(&el.t)?;
                    (**el).clone()
                }
                Ty::Class(c) => {
                    let parts = self.class_parts(*c)?;
                    let Some((_, o2, ft)) = parts.into_iter().find(|x| x.0 == i) else { return unsup("bytes of a member with no storage") };
                    off += o2;
                    ft
                }
                _ => return unsup("subobject of a scalar"),
            };
        }
        Ok((off, t))
    }

    /// The complete object and byte offset that a pointer represents
    /// (STG-15). A pointer past an object gives the offset after it.
    pub fn byte_pos(&self, p: &Ptr) -> R<(usize, i64)> {
        if p.bytes {
            return Ok((p.obj, p.off));
        }
        if p.array {
            let (off, t) = self.path_offset(p.obj, &p.path)?;
            let el = match &t.t {
                Ty::Array(el, _) => self.size_of(&el.t)?,
                _ => return unsup("array pointer to an object that is not an array"),
            };
            return Ok((p.obj, off + p.off * el));
        }
        let (off, t) = self.path_offset(p.obj, &p.path)?;
        Ok((p.obj, off + if p.off != 0 { p.off * self.size_of(&t.t)? } else { 0 }))
    }

    /// The size of a complete object.
    pub fn obj_size(&self, o: usize) -> R<i64> {
        match self.obj_ty(o) {
            Some(t) => self.size_of(&t.t),
            None => unsup("bytes of an object of unknown type"),
        }
    }

    /// A byte pointer to the byte `k` of the complete object `o`.
    pub fn byte_ptr(o: usize, k: i64) -> Ptr {
        Ptr { obj: o, path: vec![], off: k, array: false, bytes: true }
    }

    // ---- addresses ----

    /// The address of the first byte of a complete object. An object made
    /// in the storage of another has the address of its bytes there.
    pub fn obj_addr(&mut self, o: usize) -> R<u64> {
        if let Some(&(h, lo)) = self.resident_of.get(&o) {
            return Ok(self.obj_addr(h)? + lo as u64);
        }
        if let Some(&a) = self.addr_of.get(&o) {
            return Ok(a);
        }
        let size = self.obj_ty(o).and_then(|t| self.p.sizeof_ty(&t.t)).unwrap_or(16).max(1);
        // TYP-2, ALN-4: the address meets the alignment of the type.
        let align = self.obj_ty(o).and_then(|t| self.p.alignof_ty(&t.t)).unwrap_or(16).max(16);
        // NEW-26: an aligned allocation function gives storage aligned to
        // its argument.
        let align = match self.raw_args.get(&o) {
            Some(&(_, Some(a))) if a > 0 && (a as u64) <= 1 << 20 => align.max(a as u64),
            _ => align,
        };
        let align = align.max(self.raw_align.get(&o).copied().unwrap_or(0));
        let base = self.next_addr.max(FIRST_ADDR).div_ceil(align) * align;
        self.next_addr = base + size + 16;
        self.addr_of.insert(o, base);
        self.addr_table.push((base, size, o));
        Ok(base)
    }

    pub fn ptr_addr(&mut self, p: &Ptr) -> R<u64> {
        let (o, k) = self.byte_pos(p)?;
        Ok(self.obj_addr(o)?.wrapping_add(k as u64))
    }

    fn fn_addr(&mut self, tu: usize, d: DeclId) -> u64 {
        let key = self.links.get(&(tu, d)).copied().unwrap_or((tu, d));
        let i = match self.fn_addrs.iter().position(|x| *x == key) {
            Some(i) => i,
            None => {
                self.fn_addrs.push(key);
                self.fn_addrs.len() - 1
            }
        };
        FIRST_FN_ADDR + 16 * i as u64
    }

    /// The address that a pointer value holds, as an integer (CAST-16).
    pub fn val_addr(&mut self, v: &Val) -> R<u64> {
        match v {
            Val::Null => Ok(0),
            Val::Ptr(p) => self.ptr_addr(p),
            Val::FnPtr(tu, d) => Ok(self.fn_addr(*tu, *d)),
            Val::Addr(a) => Ok(*a),
            Val::Int(x) => Ok(*x as u64),
            _ => unsup("address of a value that is not a pointer"),
        }
    }

    /// The pointer that an address gives (CAST-16): the object whose
    /// storage holds the address, or an address with no object.
    pub fn addr_val(&mut self, a: u64, pointee: &QTy) -> R<Val> {
        if a == 0 {
            return Ok(Val::Null);
        }
        if (FIRST_FN_ADDR..FIRST_ADDR).contains(&a) && (a - FIRST_FN_ADDR) % 16 == 0 {
            let i = ((a - FIRST_FN_ADDR) / 16) as usize;
            if let Some(&(tu, d)) = self.fn_addrs.get(i) {
                return Ok(Val::FnPtr(tu, d));
            }
        }
        let mut hit = None;
        for &(base, size, o) in &self.addr_table {
            if a >= base && a < base + size {
                hit = Some((o, (a - base) as i64));
                break;
            }
            if a == base + size && hit.is_none() {
                hit = Some((o, size as i64));
            }
        }
        match hit {
            Some((o, k)) => Ok(Val::Ptr(self.retarget(Self::byte_ptr(o, k), pointee)?)),
            None => Ok(Val::Addr(a)),
        }
    }

    // ---- pointer conversions ----

    /// Whether a class is standard-layout, as far as CLS-19 lists the
    /// conditions.
    pub fn standard_layout(&self, c: usize, depth: u32) -> bool {
        let ci = &self.p.classes[c];
        if depth > 16 || ci.has_virtual || ci.bases.iter().any(|b| b.virt || b.class.is_none()) {
            return false;
        }
        let mut access = None;
        for &f in &ci.fields {
            if let DK::Field(fi) = &self.p.decls[f].k {
                let a = self.p.member_access.get(&f).copied().unwrap_or(0);
                if access.is_some_and(|x| x != a) {
                    return false;
                }
                access = Some(a);
                let mut t = &fi.ty.t;
                while let Ty::Array(el, _) = t {
                    t = &el.t;
                }
                if let Ty::Class(m) = t {
                    if !self.standard_layout(*m, depth + 1) {
                        return false;
                    }
                }
            }
        }
        ci.bases.iter().all(|b| b.class.is_some_and(|bc| self.standard_layout(bc, depth + 1)))
    }

    /// STG-16: the objects that are pointer-interconvertible with the
    /// object at `p`, and the first of them whose type is `to`.
    fn interconvertible(&self, p: &Ptr, to: &Ty) -> Option<Ptr> {
        let at = |path: Vec<usize>| Ptr { obj: p.obj, path, off: 0, array: false, bytes: false };
        // Up: a union member to the union, a first member or a base to a
        // standard-layout class.
        let mut path = full_path(p);
        if p.array {
            return None;
        }
        loop {
            if path.is_empty() {
                break;
            }
            let parent = path[..path.len() - 1].to_vec();
            let i = *path.last().unwrap();
            let up = match self.type_at(&at(parent.clone())).map(|q| q.t) {
                Some(Ty::Class(c)) => {
                    let ci = &self.p.classes[c];
                    let nb = self.nbases(c);
                    ci.key == 2 || (self.standard_layout(c, 0) && (i < nb || (i == nb && self.fields(c).first().is_some_and(|f| f.1.index == 0))))
                }
                _ => false,
            };
            if !up {
                break;
            }
            path = parent;
        }
        // Down from the top: each object along the same links.
        let mut stack = vec![path];
        while let Some(path) = stack.pop() {
            let q = at(path.clone());
            let Some(t) = self.type_at(&q) else { continue };
            if t.t == *to {
                return Some(q);
            }
            if let Ty::Class(c) = t.t {
                let ci = &self.p.classes[c];
                let nb = self.nbases(c);
                if ci.key == 2 {
                    for (_, fi) in self.fields(c) {
                        let mut s = path.clone();
                        s.push(nb + fi.index);
                        stack.push(s);
                    }
                } else if self.standard_layout(c, 0) {
                    for i in 0..nb {
                        let mut s = path.clone();
                        s.push(i);
                        stack.push(s);
                    }
                    if self.fields(c).first().is_some() {
                        let mut s = path.clone();
                        s.push(nb);
                        stack.push(s);
                    }
                }
            }
        }
        None
    }

    /// The object of type `to` that starts at byte `k` of the complete
    /// object `o`, outermost first.
    fn object_at(&self, o: usize, k: i64, to: &Ty) -> R<Option<Ptr>> {
        let Some(mut t) = self.obj_ty(o) else { return Ok(None) };
        let mut path: Vec<usize> = Vec::new();
        let mut k = k;
        let mut elem: Option<usize> = None;
        loop {
            if k == 0 && t.t == *to {
                return Ok(Some(match elem {
                    Some(i) => Ptr { obj: o, path: path[..path.len() - 1].to_vec(), off: i as i64, array: true, bytes: false },
                    None => Ptr { obj: o, path, off: 0, array: false, bytes: false },
                }));
            }
            elem = None;
            t = match &t.t {
                Ty::Array(el, Some(n)) => {
                    let es = self.size_of(&el.t)?;
                    if es == 0 || k / es >= *n as i64 {
                        return Ok(None);
                    }
                    let i = (k / es) as usize;
                    path.push(i);
                    k -= i as i64 * es;
                    elem = Some(i);
                    (**el).clone()
                }
                Ty::Class(c) => {
                    let c = *c;
                    let parts = self.class_parts(c)?;
                    let union = self.p.classes[c].key == 2;
                    let mut next = None;
                    for (i, off, ft) in parts {
                        let s = self.size_of(&ft.t)?;
                        if off <= k && k < off + s.max(1) {
                            if union {
                                // The member of that type, if any.
                                if ft.t == *to && k == 0 {
                                    next = Some((i, off, ft));
                                    break;
                                }
                                if next.is_none() {
                                    next = Some((i, off, ft));
                                }
                            } else {
                                next = Some((i, off, ft));
                                break;
                            }
                        }
                    }
                    let Some((i, off, ft)) = next else { return Ok(None) };
                    path.push(i);
                    k -= off;
                    ft
                }
                _ => return Ok(None),
            };
        }
    }

    /// A pointer converted to a pointer to `to` (CAST-6, STG-16, STG-29):
    /// a byte pointer for `char`, `unsigned char`, and `std::byte`, the
    /// pointer-interconvertible object of type `to`, or else the same
    /// object.
    pub fn retarget(&mut self, p: Ptr, to: &QTy) -> R<Ptr> {
        let to = to.unqual().t;
        if to == Ty::Void || matches!(to, Ty::Unknown | Ty::Dep(_)) {
            return Ok(p);
        }
        if self.is_byte_type(&to) {
            if p.bytes {
                return Ok(p);
            }
            if self.type_at(&p).is_some_and(|t| t.t == to) {
                return Ok(p);
            }
            let (o, k) = self.byte_pos(&p)?;
            return Ok(Self::byte_ptr(o, k));
        }
        if p.bytes {
            if let Some(q) = self.object_at(p.obj, p.off, &to)? {
                return Ok(q);
            }
            if p.off == 0 && self.objs[p.obj].heap == Heap::Raw {
                return self.storage_pointer(Ptr { obj: p.obj, path: vec![], off: 0, array: true, bytes: false }, &QTy::new(to));
            }
            return Ok(p);
        }
        if self.type_at(&p).is_some_and(|t| t.t == to) {
            return Ok(p);
        }
        if let Some(q) = self.interconvertible(&p, &to) {
            return Ok(q);
        }
        if self.objs[p.obj].heap == Heap::Raw {
            // STG-8: storage from an allocation function creates the
            // objects that the program needs.
            return self.storage_pointer(p, &QTy::new(to));
        }
        Ok(p)
    }

    // ---- bytes ----

    /// The scalar that holds byte `k` of the object `o`: its path, its
    /// type, and the byte's offset in it. None for a padding byte.
    fn locate(&self, o: usize, k: i64) -> R<Option<(Vec<usize>, QTy, i64)>> {
        let Some(mut t) = self.obj_ty(o) else { return unsup("bytes of an object of unknown type") };
        let mut path: Vec<usize> = Vec::new();
        let mut k = k;
        loop {
            t = match &t.t {
                Ty::Array(el, Some(n)) => {
                    let es = self.size_of(&el.t)?;
                    if es == 0 || k / es >= *n as i64 {
                        return Ok(None);
                    }
                    let i = (k / es) as usize;
                    path.push(i);
                    k -= i as i64 * es;
                    (**el).clone()
                }
                Ty::Array(_, None) => return unsup("bytes of an array of unknown bound"),
                Ty::Class(c) => {
                    let c = *c;
                    let parts = self.class_parts(c)?;
                    let mut next = None;
                    if self.p.classes[c].key == 2 {
                        // The bytes of the active member.
                        let holder = Ptr { obj: o, path: path.clone(), off: 0, array: false, bytes: false };
                        if let Ok(Val::Agg(items)) = self.get_at(&holder) {
                            if let Some(a) = items.iter().position(|v| !matches!(v, Val::Inactive)) {
                                next = parts.into_iter().find(|x| x.0 == a);
                            }
                        }
                        if let Some((_, _, ft)) = &next {
                            if k >= self.size_of(&ft.t)? {
                                next = None;
                            }
                        }
                    } else {
                        for (i, off, ft) in parts {
                            let s = self.size_of(&ft.t)?;
                            if off <= k && k < off + s {
                                next = Some((i, off, ft));
                                break;
                            }
                        }
                    }
                    let Some((i, off, ft)) = next else { return Ok(None) };
                    path.push(i);
                    k -= off;
                    ft
                }
                Ty::Unknown | Ty::Dep(_) => return unsup("bytes of an object of unknown type"),
                _ => return Ok(Some((path, t, k))),
            };
        }
    }

    /// The object made in the storage of `o` that covers byte `k`, and the
    /// byte's offset in it.
    fn resident_at(&self, o: usize, k: i64) -> Option<(usize, i64)> {
        let v = self.residents.get(&o)?;
        v.iter().rev().find(|r| self.objs[r.2].alive && r.0 <= k && k < r.0 + r.1).map(|r| (r.2, k - r.0))
    }

    /// The bytes of a scalar value of type `t` (STG-26, TYP-4, MUL-20).
    pub fn encode(&mut self, v: &Val, t: &Ty) -> R<Vec<u8>> {
        let s = self.size_of(t)? as usize;
        let int = |x: i128| x.to_le_bytes()[..s].to_vec();
        Ok(match (v, t) {
            (Val::Float(f), Ty::Float) => (*f as f32).to_bits().to_le_bytes().to_vec(),
            (Val::Float(f), Ty::Double) => f.to_bits().to_le_bytes().to_vec(),
            (Val::LD(x), Ty::LDouble) => {
                // TYP-10: ten bytes of the x87 format, then padding.
                let mut b = x.to_bytes().to_vec();
                b.resize(s, 0);
                b
            }
            (Val::Float(f), Ty::LDouble) => {
                let mut b = super::f80::F80::from_f64(*f).to_bytes().to_vec();
                b.resize(s, 0);
                b
            }
            (Val::Float(_), _) => return unsup("bytes of a long double"),
            (Val::Int(x), _) => int(*x),
            (Val::Ptr(_) | Val::Null | Val::FnPtr(..) | Val::Addr(_), Ty::Ptr(_) | Ty::NullPtr) => {
                let a = self.val_addr(v)?;
                int(a as i128)
            }
            _ => return unsup("bytes of a value the interpreter does not encode"),
        })
    }

    /// The value of a scalar of type `t` from its bytes.
    pub fn decode(&mut self, b: &[u8], t: &QTy) -> R<Val> {
        let mut u: u128 = 0;
        for (i, x) in b.iter().enumerate() {
            u |= (*x as u128) << (8 * i);
        }
        let it = match &t.t {
            Ty::Enum(e) => self.p.enum_underlying(*e),
            x => x.clone(),
        };
        if it.is_integral() {
            let bits = 8 * b.len() as u32;
            let mut x = u as i128;
            if it.is_signed() && bits < 128 && x >= 1i128 << (bits - 1) {
                x -= 1i128 << bits;
            }
            if it == Ty::Bool && x > 1 {
                return unsup("a bool whose byte holds a value other than 0 or 1");
            }
            return Ok(Val::Int(x));
        }
        match &t.t {
            Ty::Float => Ok(Val::Float(f32::from_bits(u as u32) as f64)),
            Ty::Double => Ok(Val::Float(f64::from_bits(u as u64))),
            Ty::LDouble if b.len() >= 10 => Ok(Val::LD(super::f80::F80::from_bytes(b))),
            Ty::NullPtr => Ok(Val::Null),
            Ty::Ptr(pt) => self.addr_val(u as u64, pt),
            _ => unsup("a value of this type from its bytes"),
        }
    }

    /// Byte `k` of the object representation of `o` (STG-11, STG-13).
    pub fn read_byte(&mut self, o: usize, k: i64) -> R<Byte> {
        if let Some((r, k2)) = self.resident_at(o, k) {
            return self.read_byte(r, k2);
        }
        if !self.objs[o].alive {
            return ub("OBJ-2, STG-22", "read of a byte of an object whose lifetime has ended");
        }
        let size = self.obj_size(o)?;
        if k < 0 || k >= size {
            return ub("ADD-10", "access through a byte pointer outside the object");
        }
        let Some((path, t, i)) = self.locate(o, k)? else {
            return Ok(self.shadow.get(&(o, k)).copied().unwrap_or(Byte::Indet));
        };
        let v = self.get_at(&Ptr { obj: o, path, off: 0, array: false, bytes: false })?.clone();
        match v {
            Val::Uninit => Ok(self.shadow.get(&(o, k)).copied().unwrap_or(Byte::Indet)),
            Val::NotUsable(m) => limit("CE-3", m),
            v => {
                let b = self.encode(&v, &t.t)?;
                Ok(Byte::Known(b[i as usize]))
            }
        }
    }

    /// Write byte `k` of the object representation of `o`: the scalar
    /// that holds it gets the value of its new bytes.
    pub fn write_byte(&mut self, o: usize, k: i64, b: Byte) -> R<()> {
        if let Some((r, k2)) = self.resident_at(o, k) {
            return self.write_byte(r, k2, b);
        }
        if !self.objs[o].alive {
            return ub("OBJ-2, STG-22", "write of a byte of an object whose lifetime has ended");
        }
        if self.objs[o].konst {
            return ub("CAST-5", "write to a byte of a const object");
        }
        let size = self.obj_size(o)?;
        if k < 0 || k >= size {
            return ub("ADD-10", "access through a byte pointer outside the object");
        }
        let Some((path, t, i)) = self.locate(o, k)? else {
            // A padding byte (STG-11): the value does not change.
            match b {
                Byte::Indet => self.shadow.remove(&(o, k)),
                b => self.shadow.insert((o, k), b),
            };
            return Ok(());
        };
        let place = Ptr { obj: o, path, off: 0, array: false, bytes: false };
        let s = self.size_of(&t.t)?;
        let first = k - i;
        let v = self.get_at(&place)?.clone();
        let mut bytes: Vec<Byte> = match &v {
            Val::Uninit => (0..s).map(|j| self.shadow.get(&(o, first + j)).copied().unwrap_or(Byte::Indet)).collect(),
            v => self.encode(v, &t.t)?.into_iter().map(Byte::Known).collect(),
        };
        bytes[i as usize] = b;
        if bytes.iter().all(|x| matches!(x, Byte::Known(_))) {
            let raw: Vec<u8> = bytes.iter().map(|x| if let Byte::Known(y) = x { *y } else { 0 }).collect();
            let nv = self.decode(&raw, &t)?;
            for j in 0..s {
                self.shadow.remove(&(o, first + j));
            }
            *self.slot(&place, true)? = nv;
        } else {
            for (j, x) in bytes.iter().enumerate() {
                match x {
                    Byte::Known(_) => self.shadow.insert((o, first + j as i64), *x),
                    Byte::Indet => self.shadow.remove(&(o, first + j as i64)),
                };
            }
            *self.slot(&place, true)? = Val::Uninit;
        }
        Ok(())
    }

    /// The value of a byte read through a glvalue of type `t`.
    pub fn byte_val(&self, b: Byte, t: &Ty) -> Val {
        match b {
            Byte::Known(x) => Val::Int(if *t == Ty::Char { x as i8 as i128 } else { x as i128 }),
            Byte::Indet => Val::Uninit,
        }
    }

    /// The host byte array and offset of an element place whose byte an
    /// object made by placement `new` covers.
    pub fn hosted_byte(&self, p: &Ptr) -> Option<(usize, i64)> {
        if !p.path.is_empty() || !p.array || p.bytes {
            return None;
        }
        self.resident_at(p.obj, p.off).map(|_| (p.obj, p.off))
    }

    // ---- access through a glvalue (STG-14, STG-23) ----

    /// Check that a glvalue of type `gl` can read or write the object of
    /// type `dt` (STG-14). Returns false for an access through the signed
    /// or unsigned counterpart or a byte type, which converts the value.
    fn access_ok(&self, dt: &Ty, gl: &Ty) -> R<bool> {
        fn strip(t: &Ty) -> Ty {
            match t {
                Ty::Ptr(q) => Ty::Ptr(Box::new(QTy::new(strip(&q.t)))),
                t => t.clone(),
            }
        }
        if strip(dt) == strip(gl) {
            return Ok(true);
        }
        if matches!(dt, Ty::Class(_) | Ty::Array(..) | Ty::Func(_) | Ty::Unknown | Ty::Dep(_) | Ty::Void) || matches!(gl, Ty::Class(_) | Ty::Array(..) | Ty::Func(_) | Ty::Unknown | Ty::Dep(_) | Ty::Void) {
            return Ok(true);
        }
        if self.is_byte_type(gl) {
            return Ok(false);
        }
        let counterpart = |t: &Ty| matches!(t, Ty::SChar | Ty::UChar | Ty::Short | Ty::UShort | Ty::Int | Ty::UInt | Ty::Long | Ty::ULong | Ty::LLong | Ty::ULLong);
        if counterpart(dt) && counterpart(gl) && dt.to_unsigned() == gl.to_unsigned() && *dt != Ty::UChar {
            return Ok(false);
        }
        ub("STG-23", format!("access to an object of type {} through a glvalue of type {}", crate::types::show(&QTy::new(dt.clone())), crate::types::show(&QTy::new(gl.clone()))))
    }

    /// The object type at a place, when the access check applies to it.
    fn checked_type(&self, p: &Ptr) -> Option<Ty> {
        if self.constant || p.bytes || self.uni_alias.borrow().contains(&(p.obj, p.path.clone())) {
            return None;
        }
        let t = self.type_at(p)?;
        Some(t.t)
    }

    /// Read the object at `p` through a glvalue of type `gl`.
    pub fn read_gl(&mut self, p: &Ptr, gl: &QTy) -> R<Val> {
        self.read_gl_inner(p, gl, false)
    }

    /// As `read_gl`, but a byte or `unsigned char` with no value gives
    /// `Val::Uninit` (INIT-1: a copy into an `unsigned char` or
    /// `std::byte`).
    pub fn read_gl_raw(&mut self, p: &Ptr, gl: &QTy) -> R<Val> {
        self.read_gl_inner(p, gl, true)
    }

    fn read_gl_inner(&mut self, p: &Ptr, gl: &QTy, indet: bool) -> R<Val> {
        let byte = |m: &mut Self, o: usize, k: i64, t: &Ty| -> R<Val> {
            let b = m.read_byte(o, k)?;
            match m.byte_val(b, t) {
                Val::Uninit if !indet => ub("OBJ-2, INIT-1", "read of a byte with no value"),
                v => Ok(v),
            }
        };
        if p.bytes {
            if !self.is_byte_type(&gl.t) {
                return ub("STG-23, STG-29", format!("access through a glvalue of type {} to a byte", crate::types::show(gl)));
            }
            return byte(self, p.obj, p.off, &gl.t);
        }
        if let Some((h, k)) = self.hosted_byte(p) {
            let et = self.type_at(p).map(|t| t.t).unwrap_or(Ty::UChar);
            return byte(self, h, k, &et);
        }
        if let Some(dt) = self.checked_type(p) {
            if !self.access_ok(&dt, &gl.t)? {
                if self.is_byte_type(&gl.t) {
                    let (o, k) = self.byte_pos(p)?;
                    return byte(self, o, k, &gl.t);
                }
                let v = self.read(p, gl.v)?;
                return Ok(match v {
                    Val::Int(x) => Val::Int(wrap_int(x, &gl.t)),
                    v => v,
                });
            }
        }
        if indet && !self.constant && self.is_byte_type(&gl.t) && matches!(self.get_at(p), Ok(Val::Uninit)) {
            return Ok(Val::Uninit);
        }
        self.read(p, gl.v)
    }

    /// Write `v` to the object at `p` through a glvalue of type `gl`.
    pub fn write_gl(&mut self, p: &Ptr, gl: &QTy, v: Val) -> R<()> {
        let as_byte = |v: &Val| match v {
            Val::Int(x) => Ok(Byte::Known(*x as u8)),
            Val::Uninit => Ok(Byte::Indet),
            _ => unsup("byte write of a value that is not an integer"),
        };
        if p.bytes {
            if !self.is_byte_type(&gl.t) {
                return ub("STG-23, STG-29", format!("access through a glvalue of type {} to a byte", crate::types::show(gl)));
            }
            let b = as_byte(&v)?;
            return self.write_byte(p.obj, p.off, b);
        }
        if let Some((h, k)) = self.hosted_byte(p) {
            let b = as_byte(&v)?;
            return self.write_byte(h, k, b);
        }
        if let Some(dt) = self.checked_type(p) {
            if !self.access_ok(&dt, &gl.t)? {
                if self.is_byte_type(&gl.t) {
                    let (o, k) = self.byte_pos(p)?;
                    let b = as_byte(&v)?;
                    return self.write_byte(o, k, b);
                }
                let v = match v {
                    Val::Int(x) => Val::Int(wrap_int(x, &dt)),
                    v => v,
                };
                return self.write(p, v, false);
            }
        }
        self.write(p, v, false)
    }

    /// STG-23 for a member access: the object at `p` must have the class
    /// `c` or a class derived from it.
    pub fn check_class_access(&self, p: &Ptr, c: usize) -> R<()> {
        if self.constant || p.bytes {
            if p.bytes && !self.constant {
                return ub("STG-23, STG-29", "member access through a pointer to a byte");
            }
            return Ok(());
        }
        let Some(t) = self.type_at(p) else { return Ok(()) };
        if self.uni_alias.borrow().contains(&(p.obj, full_path(p))) || self.uni_alias.borrow().contains(&(p.obj, p.path.clone())) {
            return Ok(());
        }
        match t.t {
            Ty::Class(x) if x != c && self.base_path(x, c).is_none() && self.base_path(c, x).is_none() => {
                ub("STG-23", format!("access to an object of class '{}' through a glvalue of class '{}'", self.p.classes[x].name, self.p.classes[c].name))
            }
            Ty::Int | Ty::UInt | Ty::Long | Ty::ULong | Ty::Char | Ty::UChar | Ty::Double | Ty::Float | Ty::Short | Ty::UShort | Ty::LLong | Ty::ULLong => {
                ub("STG-23", format!("access to an object of scalar type through a glvalue of class '{}'", self.p.classes[c].name))
            }
            _ => Ok(()),
        }
    }
}

impl<'a> Machine<'a> {
    /// A `reinterpret_cast` to a type that is not a reference, or a C-style
    /// cast that acts as one (CAST-6, CAST-12, CAST-16, STG-16).
    pub fn eval_reinterpret(&mut self, a: &Expr, what: &str, to: &QTy) -> R<Val> {
        if self.constant {
            // CE-4: no reinterpreting conversion in constant evaluation.
            return limit("CE-4", what.to_string());
        }
        let v = self.eval(a)?;
        let from = nonref(&a.ty).t;
        let to_t = &to.t;
        let int_like = |t: &Ty| t.is_integral() || matches!(t, Ty::Enum(_));
        match (&from, to_t) {
            // CAST-16: a pointer to an integer gives its address.
            (Ty::Ptr(_) | Ty::NullPtr, t) if int_like(t) => {
                let a = self.val_addr(&v)?;
                let it = match t {
                    Ty::Enum(e) => self.p.enum_underlying(*e),
                    t => t.clone(),
                };
                Ok(Val::Int(wrap_int(a as i128, &it)))
            }
            // CAST-16: an integer to a pointer gives the pointer whose
            // address it is.
            (f, Ty::Ptr(pt)) if int_like(f) => {
                let Val::Int(x) = v else { return unsup("integer to pointer of a value that is not an integer") };
                let a = wrap_int(x, &Ty::size_t()) as u64;
                if matches!(pt.t, Ty::Func(_)) {
                    return match self.addr_val(a, &QTy::new(Ty::Void))? {
                        f @ (Val::FnPtr(..) | Val::Null | Val::Addr(_)) => Ok(f),
                        _ => unsup("integer to function pointer that names an object"),
                    };
                }
                self.addr_val(a, pt)
            }
            (Ty::Ptr(_), Ty::Ptr(pt)) => match v {
                // CAST-6, CAST-12: a function pointer keeps its function
                // through any pointer type.
                Val::Null | Val::FnPtr(..) | Val::Addr(_) => Ok(v),
                Val::Ptr(p) => {
                    if matches!(pt.t, Ty::Func(_)) {
                        return unsup("object pointer converted to a function pointer");
                    }
                    Ok(Val::Ptr(self.retarget(p, pt)?))
                }
                _ => unsup("reinterpret_cast of a value that is not a pointer"),
            },
            (f, t) if int_like(f) && int_like(t) => Ok(v),
            (Ty::NullPtr, Ty::NullPtr) => Ok(Val::Null),
            // CAST-6, MPTR-4: a pointer to member keeps its member.
            (Ty::MemPtr(..), Ty::MemPtr(..)) => Ok(v),
            (Ty::MemPtr(..), _) | (_, Ty::MemPtr(..)) => unsup("reinterpret_cast of a pointer to member"),
            _ => unsup(format!("reinterpret_cast from {} to {}", crate::types::show(&QTy::new(from.clone())), crate::types::show(to))),
        }
    }
}

impl<'a> Machine<'a> {
    /// The place for placement `new` that a byte pointer gives: an element
    /// of a byte array, or the object that starts at the byte.
    pub fn byte_place(&self, p: &Ptr) -> R<Ptr> {
        if let Some(QTy { t: Ty::Array(el, Some(_)), .. }) = self.obj_ty(p.obj) {
            if self.is_byte_type(&el.t) {
                return Ok(Ptr { obj: p.obj, path: vec![], off: p.off, array: true, bytes: false });
            }
        }
        if p.off == 0 {
            return Ok(Ptr::whole(p.obj));
        }
        unsup("placement new at a byte inside an object")
    }

    /// Pointer arithmetic and comparison when a byte pointer or an address
    /// takes part: the addresses decide (STG-13, STG-15, CAST-16). None
    /// when no such pointer takes part.
    pub fn addr_binary(&mut self, op: BOp, l: &Val, r: &Val) -> R<Option<Val>> {
        let special = |v: &Val| matches!(v, Val::Addr(_)) || matches!(v, Val::Ptr(p) if p.bytes);
        if !special(l) && !special(r) {
            return Ok(None);
        }
        let ptrish = |v: &Val| matches!(v, Val::Ptr(_) | Val::Addr(_) | Val::Null | Val::FnPtr(..));
        if !(ptrish(l) && ptrish(r)) {
            if matches!(op, BOp::Add | BOp::Sub) {
                if let (Val::Addr(_), Val::Int(_)) | (Val::Int(_), Val::Addr(_)) = (l, r) {
                    return unsup("arithmetic on an address with no object");
                }
            }
            return Ok(None);
        }
        // The host of an object made in a byte array, so that the object
        // and the array compare by their bytes.
        let home = |m: &Self, v: &Val| -> Option<usize> {
            match v {
                Val::Ptr(p) => Some(m.resident_of.get(&p.obj).map(|x| x.0).unwrap_or(p.obj)),
                _ => None,
            }
        };
        let (hl, hr) = (home(self, l), home(self, r));
        let same = hl.is_some() && hl == hr;
        let (a, b) = (self.val_addr(l)?, self.val_addr(r)?);
        match op {
            BOp::Sub => {
                if !same {
                    return ub("ADD-11", "subtraction of pointers into different objects");
                }
                Ok(Some(Val::Int(a as i128 - b as i128)))
            }
            BOp::Eq | BOp::Ne => {
                let eq = a == b;
                Ok(Some(Val::Int((if op == BOp::Eq { eq } else { !eq }) as i128)))
            }
            BOp::Lt | BOp::Gt | BOp::Le | BOp::Ge => {
                if !same {
                    // CMP-8: the result is unspecified.
                    let rev = self.decide(2) == 1;
                    self.choice("CMP-8", if rev { "pointers to unrelated objects compare in the reverse order of their addresses" } else { "pointers to unrelated objects compare by their addresses" });
                    if rev {
                        let res = match op {
                            BOp::Lt => a > b,
                            BOp::Gt => a < b,
                            BOp::Le => a >= b,
                            _ => a <= b,
                        };
                        return Ok(Some(Val::Int(res as i128)));
                    }
                }
                let res = match op {
                    BOp::Lt => a < b,
                    BOp::Gt => a > b,
                    BOp::Le => a <= b,
                    _ => a >= b,
                };
                Ok(Some(Val::Int(res as i128)))
            }
            _ => Ok(None),
        }
    }
}

impl<'a> Machine<'a> {
    /// A class of the table of LSUP-8 that the program declares in `std`.
    pub fn std_class(&self, name: &str) -> Option<usize> {
        self.p.classes.iter().position(|ci| ci.name == name && ci.complete && !ci.template && self.p.scopes[ci.scope].parent.is_some_and(|s| self.p.scopes[s].name == "std"))
    }

    /// Whether a class is one of the classes of LSUP-8, whose destructor
    /// the library defines.
    pub fn library_exception(&self, c: usize) -> bool {
        let ci = &self.p.classes[c];
        matches!(ci.name.as_str(), "exception" | "bad_alloc" | "bad_array_new_length" | "bad_cast") && self.p.scopes[ci.scope].parent.is_some_and(|s| self.p.scopes[s].name == "std")
    }

    /// The exception that the language throws (LSUP-8): an object of the
    /// class `name` in `std`, if the program declares it. Otherwise an
    /// object that only `catch (...)` matches.
    pub fn throw_std(&mut self, name: &str) -> R<Stop> {
        if self.constant {
            return limit("CE-8", format!("the evaluation throws std::{}", name));
        }
        if let Some(c) = self.std_class(name) {
            let t = QTy::new(Ty::Class(c));
            let place = self.alloc(&t)?;
            self.default_init(&place, &t)?;
            return Ok(Stop::Throw(Exc { obj: place.obj, ty: t }));
        }
        let place = self.alloc(&QTy::int())?;
        self.write(&place, Val::Int(0), true)?;
        Ok(Stop::Throw(Exc { obj: place.obj, ty: QTy::new(Ty::Unknown) }))
    }
}
