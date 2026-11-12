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
            Type::Void | Type::Array(_, None) | Type::Func(_) | Type::Error => return None,
        })
    }
    pub fn align_of(self, u: &Unit, q: &QType) -> Option<u64> {
        Some(match &q.t {
            Type::Int(IntK::LongLong | IntK::ULongLong) if self.ilp32 => 4,
            Type::Float(FloatK::Double | FloatK::LongDouble) if self.ilp32 => 4,
            Type::Array(e, _) => return self.align_of(u, e),
            Type::Ref(r, _) => return self.align_of(u, r),
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
