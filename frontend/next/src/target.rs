//! `target`: the sizes and alignments of the matrix targets, x86-64
//! Linux (LP64) and `-m32` (ILP32). The tables are those of TYP-1 and
//! TYP-2 (`foundations/types`); `foundations::types` names the rules,
//! and this core module holds the numbers. Class layout comes with step 4.

use crate::sem::types::{FloatK, IntK, QType, Type};
use crate::sem::Unit;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Target {
    /// `-m32`: the ILP32 column.
    pub ilp32: bool,
}

impl Target {
    pub fn int_size(self, k: IntK) -> u64 {
        use IntK::*;
        match k {
            Bool | Char | SChar | UChar | Char8 => 1,
            Short | UShort | Char16 => 2,
            Int | UInt | WChar | Char32 => 4,
            Long | ULong => {
                if self.ilp32 {
                    4
                } else {
                    8
                }
            }
            LongLong | ULongLong => 8,
        }
    }
    /// The width in bits: 8 times the size, and 1 for `bool`.
    pub fn int_bits(self, k: IntK) -> u32 {
        if k == IntK::Bool {
            1
        } else {
            8 * self.int_size(k) as u32
        }
    }
    pub fn int_min(self, k: IntK) -> i128 {
        if k.signed() {
            -(1i128 << (self.int_bits(k) - 1))
        } else {
            0
        }
    }
    pub fn int_max(self, k: IntK) -> i128 {
        if k.signed() {
            (1i128 << (self.int_bits(k) - 1)) - 1
        } else {
            (1i128 << self.int_bits(k)) - 1
        }
    }
    pub fn fits(self, k: IntK, v: i128) -> bool {
        v >= self.int_min(k) && v <= self.int_max(k)
    }
    /// The value modulo 2 to the width, in the range of `k` (CONV rules
    /// on integral conversions).
    pub fn wrap(self, k: IntK, v: i128) -> i128 {
        if k == IntK::Bool {
            return (v != 0) as i128;
        }
        let bits = self.int_bits(k);
        let m = 1i128 << bits;
        let mut r = v.rem_euclid(m);
        if k.signed() && r >= m / 2 {
            r -= m;
        }
        r
    }
    pub fn float_size(self, k: FloatK) -> u64 {
        match k {
            FloatK::Float => 4,
            FloatK::Double => 8,
            FloatK::LongDouble => {
                if self.ilp32 {
                    12
                } else {
                    16
                }
            }
        }
    }
    pub fn ptr_size(self) -> u64 {
        if self.ilp32 {
            4
        } else {
            8
        }
    }
    /// The type of `sizeof` and `alignof` (`std::size_t`).
    pub fn size_t(self) -> IntK {
        if self.ilp32 {
            IntK::UInt
        } else {
            IntK::ULong
        }
    }
    /// The type of the difference of two pointers.
    pub fn ptrdiff_t(self) -> IntK {
        if self.ilp32 {
            IntK::Int
        } else {
            IntK::Long
        }
    }
    /// The size of a complete object type, or None for a type with no
    /// size (incomplete, function, reference to it, or an error).
    pub fn size_of(self, u: &Unit, q: &QType) -> Option<u64> {
        Some(match &q.t {
            Type::Int(k) => self.int_size(*k),
            Type::Float(k) => self.float_size(*k),
            Type::NullPtr | Type::Ptr(_) => self.ptr_size(),
            Type::Ref(r, _) => return self.size_of(u, r),
            Type::Array(e, Some(n)) => self.size_of(u, e)?.checked_mul(*n)?,
            Type::Enum(e) => self.int_size(u.enums.get(*e)?.underlying?),
            Type::Class(c) => self.layout(u, *c)?.0,
            Type::Void | Type::Array(_, None) | Type::Func(_) | Type::Error => return None,
        })
    }
    /// The layout of a complete class (CLS rules, TYP-2): its size, its
    /// alignment, and the offset of each data member. Each member goes
    /// at the next multiple of its alignment, a union's members all at
    /// 0, and the size is a multiple of the alignment, at least 1.
    pub fn layout(self, u: &Unit, c: crate::sem::types::ClassId) -> Option<(u64, u64, Vec<u64>)> {
        let info = u.classes.get(c)?;
        if !info.complete {
            return None;
        }
        let union = info.key == crate::sem::ClassKey::Union;
        let (mut size, mut align) = (0u64, 1u64);
        // The empty classes among the subobjects placed so far, with their
        // offsets: two subobjects of one type never share an address.
        let mut placed: Vec<(crate::sem::types::ClassId, u64)> = Vec::new();
        let fields: Vec<&crate::sem::Field> = info.fields.iter().filter_map(|&f| u.field(f)).collect();
        let mut offs = vec![0u64; fields.len()];
        // A polymorphic class holds a pointer to its virtual functions at
        // offset 0, which its first polymorphic base shares (the matrix's
        // ABI, CLS-35).
        let mut order: Vec<usize> = (0..fields.len()).collect();
        if !union && u.polymorphic(c) {
            match (0..fields.len()).find(|&k| fields[k].base && fields[k].ty.class().is_some_and(|b| u.polymorphic(b))) {
                Some(p) => {
                    order.remove(p);
                    order.insert(0, p);
                }
                None => {
                    size = self.ptr_size();
                    align = self.ptr_size();
                }
            }
        }
        for k in order {
            let x = fields[k];
            let t = x.ty.clone();
            let inner = t.base_elem().class().map(|k| self.empty_subobjects(u, k)).unwrap_or_default();
            let clash = |at: u64, placed: &[(crate::sem::types::ClassId, u64)]| inner.iter().any(|&(k, o)| placed.contains(&(k, at + o)));
            // An empty base takes no space (DER-2), at the first offset
            // where it shares no address with a subobject of its type.
            if x.base && t.class().is_some_and(|b| self.is_empty(u, b)) {
                let mut at = 0;
                while clash(at, &placed) {
                    at += 1;
                }
                offs[k] = at;
                placed.extend(inner.iter().map(|&(e, o)| (e, at + o)));
                size = size.max(at.min(size));
                if at > 0 {
                    size = size.max(at + 1);
                }
                continue;
            }
            let (s, a) = (self.size_of(u, &t)?, self.align_of(u, &t)?);
            // The tail padding of a base that is not a POD holds the
            // members that follow (the matrix's ABI, CLS-35).
            let s = match t.class() {
                Some(b) if x.base && !self.pod(u, b) => self.data_size(u, b)?,
                _ => s,
            };
            align = align.max(a);
            if union {
                size = size.max(s);
            } else {
                let mut at = size.div_ceil(a).checked_mul(a)?;
                while clash(at, &placed) {
                    at = at.checked_add(a)?;
                }
                offs[k] = at;
                placed.extend(inner.iter().map(|&(e, o)| (e, at + o)));
                size = at.checked_add(s)?;
            }
        }
        let size = size.div_ceil(align).max(1).checked_mul(align)?;
        Some((size.max(1), align, offs))
    }
    /// The subobjects of empty class type in an object of class `c`, with
    /// their offsets, `c` itself included when it is empty.
    fn empty_subobjects(self, u: &Unit, c: crate::sem::types::ClassId) -> Vec<(crate::sem::types::ClassId, u64)> {
        let mut out = Vec::new();
        if self.is_empty(u, c) {
            out.push((c, 0));
        }
        let Some((_, _, offs)) = self.layout(u, c) else { return out };
        for (k, &f) in u.classes[c].fields.iter().enumerate() {
            let Some(x) = u.field(f) else { continue };
            if let Some(d) = x.ty.class() {
                out.extend(self.empty_subobjects(u, d).into_iter().map(|(e, o)| (e, o + offs[k])));
            }
        }
        out
    }
    /// The size of a class without its tail padding.
    fn data_size(self, u: &Unit, c: crate::sem::types::ClassId) -> Option<u64> {
        let (size, _, offs) = self.layout(u, c)?;
        let mut end = 0;
        for (k, &f) in u.classes[c].fields.iter().enumerate() {
            let x = u.field(f)?;
            if x.base && x.ty.class().is_some_and(|b| self.is_empty(u, b)) {
                continue;
            }
            let s = match x.ty.class() {
                Some(b) if x.base && !self.pod(u, b) => self.data_size(u, b)?,
                _ => self.size_of(u, &x.ty)?,
            };
            end = end.max(offs[k] + s);
        }
        Some(if end == 0 { size } else { end })
    }
    /// A POD class for the layout: no constructor, destructor, or
    /// assignment other than defaulted ones, no base, and public members
    /// of POD types.
    fn pod(self, u: &Unit, c: crate::sem::types::ClassId) -> bool {
        let names = &u.scopes[u.classes[c].scope].names;
        let provided = |n: &str| names.get(n).is_some_and(|fs| fs.iter().any(|f| !u.defaulted.contains(f)));
        if u.polymorphic(c) || provided("{ctor}") || provided("{dtor}") || provided("operator=") {
            return false;
        }
        u.classes[c].fields.iter().all(|&f| {
            u.field(f).is_some_and(|x| !x.base && x.init.is_none() && x.access == crate::sem::Access::Public && x.ty.base_elem().class().is_none_or(|d| self.pod(u, d)))
        })
    }
    /// A class with no data member other than empty bases.
    pub fn is_empty(self, u: &Unit, c: crate::sem::types::ClassId) -> bool {
        !u.polymorphic(c) && u.classes[c].fields.iter().all(|&f| u.field(f).is_some_and(|x| x.base && x.ty.class().is_some_and(|b| self.is_empty(u, b))))
    }
    pub fn align_of(self, u: &Unit, q: &QType) -> Option<u64> {
        Some(match &q.t {
            Type::Int(IntK::LongLong | IntK::ULongLong) if self.ilp32 => 4,
            Type::Float(FloatK::Double | FloatK::LongDouble) if self.ilp32 => 4,
            Type::Array(e, _) => return self.align_of(u, e),
            Type::Ref(r, _) => return self.align_of(u, r),
            Type::Class(c) => self.layout(u, *c)?.1,
            _ => self.size_of(u, q)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_follow_the_tables_of_typ_1() {
        let lp = Target { ilp32: false };
        let ilp = Target { ilp32: true };
        let u = Unit::default();
        assert_eq!(lp.int_size(IntK::Long), 8);
        assert_eq!(ilp.int_size(IntK::Long), 4);
        assert_eq!(ilp.float_size(FloatK::LongDouble), 12);
        assert_eq!(ilp.align_of(&u, &QType::new(Type::Int(IntK::LongLong))), Some(4));
        assert_eq!(lp.size_of(&u, &QType::ptr_to(QType::int())), Some(8));
        assert_eq!(lp.size_t(), IntK::ULong);
    }

    #[test]
    fn wrapping_is_modulo_the_width() {
        let t = Target::default();
        assert_eq!(t.wrap(IntK::UChar, 300), 44);
        assert_eq!(t.wrap(IntK::SChar, 200), -56);
        assert_eq!(t.wrap(IntK::UInt, -1), 4294967295);
        assert_eq!(t.wrap(IntK::Bool, 7), 1);
        assert!(t.fits(IntK::Int, 2147483647) && !t.fits(IntK::Int, 2147483648));
    }
}
