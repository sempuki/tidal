//! The run part of `foundations/types`: the format of `long double`
//! (TYP-10). On the targets of the matrix, `long double` has a 64-bit
//! significand with an explicit integer bit and a 15-bit exponent, so
//! the machine computes it in software. Each operation rounds the exact
//! result to nearest, ties to even.

use std::cmp::Ordering;

/// A value of the extended format. A finite value is `m * 2^e`, with
/// `m` normalized so that its bit 63 is set, or smaller at the least
/// exponent (a subnormal value), or zero.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F80 {
    Finite { neg: bool, m: u64, e: i32 },
    Inf(bool),
    NaN,
}

/// A binary format: the bits of the significand, the least exponent of
/// its last bit, and the greatest exponent of its first bit.
struct Format {
    bits: u32,
    min_e: i32,
    max_top: i32,
}

const EXT: Format = Format { bits: 64, min_e: -16445, max_top: 16383 };
const DOUBLE: Format = Format { bits: 53, min_e: -1074, max_top: 1023 };
const SINGLE: Format = Format { bits: 24, min_e: -149, max_top: 127 };

/// Round `m * 2^e`, with `sticky` set when nonzero bits lie below `m`,
/// to the format: the significand and exponent, or None on overflow.
fn round(f: &Format, m: u128, e: i32, sticky: bool) -> Option<(u128, i32)> {
    if m == 0 {
        return Some((0, f.min_e));
    }
    let width = 128 - m.leading_zeros() as i32;
    let mut shift = width - f.bits as i32;
    if e + shift < f.min_e {
        shift = f.min_e - e;
    }
    let (mut m, mut e) = (m, e);
    if shift > 0 {
        // Up above half of the last place, and at exactly half when the
        // result is odd (ties to even).
        let (q, up) = if shift > 128 {
            (0, false)
        } else {
            let (q, lost, half) = if shift == 128 { (0, m, 1u128 << 127) } else { (m >> shift, m & ((1u128 << shift) - 1), 1u128 << (shift - 1)) };
            let up = match lost.cmp(&half) {
                Ordering::Greater => true,
                Ordering::Equal => sticky || q & 1 == 1,
                Ordering::Less => false,
            };
            (q, up)
        };
        m = q + up as u128;
        e += shift;
        if m >> f.bits != 0 {
            m >>= 1;
            e += 1;
        }
    } else if shift < 0 {
        m <<= -shift;
        e += shift;
    }
    if m != 0 && e + (128 - m.leading_zeros() as i32) - 1 > f.max_top {
        return None;
    }
    Some((m, e))
}

impl F80 {
    pub const ZERO: F80 = F80::Finite { neg: false, m: 0, e: EXT.min_e };

    fn make(neg: bool, m: u128, e: i32, sticky: bool) -> F80 {
        match round(&EXT, m, e, sticky) {
            Some((m, e)) => F80::Finite { neg, m: m as u64, e },
            None => F80::Inf(neg),
        }
    }

    pub fn from_int(x: i128) -> F80 {
        F80::make(x < 0, x.unsigned_abs(), 0, false)
    }

    pub fn from_f64(f: f64) -> F80 {
        if f.is_nan() {
            return F80::NaN;
        }
        if f.is_infinite() {
            return F80::Inf(f < 0.0);
        }
        let bits = f.to_bits();
        let neg = bits >> 63 == 1;
        let exp = ((bits >> 52) & 0x7ff) as i32;
        let frac = bits & ((1 << 52) - 1);
        let (m, e) = if exp == 0 { (frac, -1074) } else { (frac | 1 << 52, exp - 1075) };
        F80::make(neg, m as u128, e, false)
    }

    /// The value rounded to `double`, or to `float` when `single`.
    pub fn to_f64(self, single: bool) -> f64 {
        let (neg, m, e) = match self {
            F80::NaN => return f64::NAN,
            F80::Inf(n) => return if n { f64::NEG_INFINITY } else { f64::INFINITY },
            F80::Finite { neg, m, e } => (neg, m, e),
        };
        let f = if single { &SINGLE } else { &DOUBLE };
        let v = match round(f, m as u128, e, false) {
            None => f64::INFINITY,
            Some((m, e)) => {
                if single {
                    // Every float value is a double, so the scaling below
                    // is exact.
                    scale(m as f64, e)
                } else if m >> 52 == 0 {
                    f64::from_bits(m as u64)
                } else {
                    f64::from_bits(((e + 1075) as u64) << 52 | (m as u64 & ((1 << 52) - 1)))
                }
            }
        };
        if neg {
            -v
        } else {
            v
        }
    }

    /// The value toward zero as an integer, or None when it does not fit
    /// in 128 bits or is not finite.
    pub fn trunc(self) -> Option<i128> {
        let F80::Finite { neg, m, e } = self else { return None };
        let mag: u128 = if e >= 0 {
            if e > 63 {
                return None;
            }
            (m as u128) << e
        } else if e <= -64 {
            0
        } else {
            (m >> -e) as u128
        };
        let v = i128::try_from(mag).ok()?;
        Some(if neg { -v } else { v })
    }

    /// The 80 bits of the format: the significand with its integer bit,
    /// then 15 bits of biased exponent, then the sign.
    pub fn to_bits(self) -> u128 {
        let (neg, exp, mant): (bool, u128, u128) = match self {
            F80::NaN => (false, 0x7fff, 0xC000_0000_0000_0000),
            F80::Inf(n) => (n, 0x7fff, 0x8000_0000_0000_0000),
            F80::Finite { neg, m: 0, .. } => (neg, 0, 0),
            F80::Finite { neg, m, e } => {
                if m >> 63 == 0 {
                    (neg, 0, m as u128)
                } else {
                    (neg, (e + 63 + 16383) as u128, m as u128)
                }
            }
        };
        mant | exp << 64 | (neg as u128) << 79
    }

    pub fn from_bits(b: u128) -> F80 {
        let neg = b >> 79 & 1 == 1;
        let exp = (b >> 64 & 0x7fff) as i32;
        let m = b as u64;
        match exp {
            0x7fff if m << 1 == 0 => F80::Inf(neg),
            0x7fff => F80::NaN,
            0 => F80::Finite { neg, m, e: EXT.min_e },
            _ => F80::make(neg, m as u128, exp - 16383 - 63, false),
        }
    }

    pub fn is_finite(self) -> bool {
        matches!(self, F80::Finite { .. })
    }

    pub fn is_zero(self) -> bool {
        matches!(self, F80::Finite { m: 0, .. })
    }

    pub fn neg(self) -> F80 {
        match self {
            F80::Finite { neg, m, e } => F80::Finite { neg: !neg, m, e },
            F80::Inf(n) => F80::Inf(!n),
            F80::NaN => F80::NaN,
        }
    }

    pub fn add(self, o: F80) -> F80 {
        let ((an, am, ae), (bn, bm, be)) = match (self, o) {
            (F80::NaN, _) | (_, F80::NaN) => return F80::NaN,
            (F80::Inf(a), F80::Inf(b)) => return if a == b { F80::Inf(a) } else { F80::NaN },
            (F80::Inf(a), _) | (_, F80::Inf(a)) => return F80::Inf(a),
            (F80::Finite { neg: an, m: am, e: ae }, F80::Finite { neg: bn, m: bm, e: be }) => ((an, am, ae), (bn, bm, be)),
        };
        if am == 0 && bm == 0 {
            return F80::Finite { neg: an && bn, m: 0, e: EXT.min_e };
        }
        if am == 0 {
            return o;
        }
        if bm == 0 {
            return self;
        }
        // The operand with the greater exponent first, with 62 guard
        // bits, and the other aligned to it with its lost bits jammed
        // into the last bit.
        let ((xn, xm, xe), (yn, ym, ye)) = if ae >= be { ((an, am, ae), (bn, bm, be)) } else { ((bn, bm, be), (an, am, ae)) };
        let x = (xm as u128) << 62;
        let d = (xe - ye) as u32;
        let y_full = (ym as u128) << 62;
        let y = if d >= 127 { (y_full != 0) as u128 } else { (y_full >> d) | ((y_full & ((1u128 << d) - 1)) != 0) as u128 };
        let e = xe - 62;
        if xn == yn {
            return F80::make(xn, x + y, e, false);
        }
        match x.cmp(&y) {
            Ordering::Equal => F80::ZERO,
            Ordering::Greater => F80::make(xn, x - y, e, false),
            Ordering::Less => F80::make(yn, y - x, e, false),
        }
    }

    pub fn sub(self, o: F80) -> F80 {
        self.add(o.neg())
    }

    pub fn mul(self, o: F80) -> F80 {
        match (self, o) {
            (F80::NaN, _) | (_, F80::NaN) => F80::NaN,
            (F80::Inf(a), x) | (x, F80::Inf(a)) => {
                if x.is_zero() {
                    F80::NaN
                } else {
                    F80::Inf(a != x.sign())
                }
            }
            (F80::Finite { neg: an, m: am, e: ae }, F80::Finite { neg: bn, m: bm, e: be }) => F80::make(an != bn, am as u128 * bm as u128, ae + be, false),
        }
    }

    pub fn div(self, o: F80) -> F80 {
        let neg = self.sign() != o.sign();
        match (self, o) {
            (F80::NaN, _) | (_, F80::NaN) | (F80::Inf(_), F80::Inf(_)) => F80::NaN,
            (F80::Inf(_), _) => F80::Inf(neg),
            (_, F80::Inf(_)) => F80::Finite { neg, m: 0, e: EXT.min_e },
            (F80::Finite { m: am, e: ae, .. }, F80::Finite { m: bm, e: be, .. }) => {
                if bm == 0 {
                    return if am == 0 { F80::NaN } else { F80::Inf(neg) };
                }
                if am == 0 {
                    return F80::Finite { neg, m: 0, e: EXT.min_e };
                }
                // Normalize both significands to bit 63, then divide in
                // two steps for 72 bits of quotient and a sticky bit.
                let (sa, sb) = (am.leading_zeros(), bm.leading_zeros());
                let (a, b) = ((am << sa) as u128, (bm << sb) as u128);
                let (q1, r1) = ((a << 64) / b, (a << 64) % b);
                let (q2, r2) = ((r1 << 64) / b, (r1 << 64) % b);
                let low = q2 & ((1 << 56) - 1);
                let q = (q1 << 8) | (q2 >> 56) | (low != 0 || r2 != 0) as u128;
                F80::make(neg, q, ae - sa as i32 - (be - sb as i32) - 72, false)
            }
        }
    }

    fn sign(self) -> bool {
        match self {
            F80::Finite { neg, .. } => neg,
            F80::Inf(n) => n,
            F80::NaN => false,
        }
    }

    /// The order of two values, or None when either is NaN.
    pub fn cmp(self, o: F80) -> Option<Ordering> {
        let key = |x: F80| -> Option<(i8, i32, u64)> {
            match x {
                F80::NaN => None,
                F80::Inf(n) => Some((if n { -2 } else { 2 }, 0, 0)),
                F80::Finite { m: 0, .. } => Some((0, 0, 0)),
                F80::Finite { neg, m, e } => {
                    let lz = m.leading_zeros();
                    Some((if neg { -1 } else { 1 }, e + 63 - lz as i32, m << lz))
                }
            }
        };
        let (a, b) = (key(self)?, key(o)?);
        if a.0 != b.0 {
            return Some(a.0.cmp(&b.0));
        }
        let mag = (a.1, a.2).cmp(&(b.1, b.2));
        Some(if a.0 < 0 { mag.reverse() } else { mag })
    }

    /// The value of a floating literal's spelling, without its suffix:
    /// decimal or hexadecimal, with digit separators.
    pub fn parse(text: &str) -> Option<F80> {
        let t: String = text.chars().filter(|&c| c != '\'').collect();
        let t = t.trim_end_matches(['l', 'L', 'f', 'F']);
        if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
            let (digits, exp) = h.split_once(['p', 'P'])?;
            let exp: i64 = exp.parse().ok()?;
            let (int, frac) = digits.split_once('.').unwrap_or((digits, ""));
            let mut n = Big::default();
            for c in int.chars().chain(frac.chars()) {
                n.mul_add(16, c.to_digit(16)?);
            }
            let e = exp - 4 * frac.len() as i64;
            return Some(from_big(n, e.clamp(-1 << 20, 1 << 20) as i32, 0));
        }
        let (mant, exp) = match t.split_once(['e', 'E']) {
            Some((m, x)) => (m, x.parse::<i64>().ok()?),
            None => (t, 0),
        };
        let (int, frac) = mant.split_once('.').unwrap_or((mant, ""));
        let mut n = Big::default();
        for c in int.chars().chain(frac.chars()) {
            n.mul_add(10, c.to_digit(10)?);
        }
        let k = exp - frac.len() as i64;
        Some(from_big(n, 0, k.clamp(-100_000, 100_000) as i32))
    }
}

/// `x * 2^e`, exact while the result is a double.
fn scale(x: f64, e: i32) -> f64 {
    let mut x = x;
    let mut e = e;
    while e > 0 {
        let s = e.min(1000);
        x *= f64::powi(2.0, s);
        e -= s;
    }
    while e < 0 {
        let s = (-e).min(1000);
        x *= f64::powi(2.0, -s);
        e += s;
    }
    x
}

/// `n * 2^e2 * 10^e10`, rounded to the extended format.
fn from_big(n: Big, e2: i32, e10: i32) -> F80 {
    if n.is_zero() {
        return F80::ZERO;
    }
    let mut n = n;
    if e10 >= 0 {
        for _ in 0..e10 {
            n.mul_add(10, 0);
        }
        let (top, sticky, shift) = n.top(66);
        return F80::make(false, top, e2 + shift, sticky);
    }
    // A quotient of at least 66 bits, and a sticky bit for the rest.
    let mut d = Big::from(1);
    for _ in 0..-e10 {
        d.mul_add(10, 0);
    }
    let s = (d.bits() as i32 - n.bits() as i32 + 67).max(0);
    n.shl(s as u32);
    let (q, rest) = n.div_small_quotient(&d);
    F80::make(false, q | rest as u128, e2 - s, false)
}

/// A nonnegative integer of any size, for the exact value of a literal.
#[derive(Clone, Default)]
struct Big(Vec<u32>);

impl Big {
    fn from(x: u32) -> Big {
        Big(vec![x])
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|&w| w == 0)
    }

    fn mul_add(&mut self, k: u32, a: u32) {
        let mut carry = a as u64;
        for w in self.0.iter_mut() {
            let v = *w as u64 * k as u64 + carry;
            *w = v as u32;
            carry = v >> 32;
        }
        if carry != 0 {
            self.0.push(carry as u32);
        }
    }

    fn bits(&self) -> u32 {
        for (i, &w) in self.0.iter().enumerate().rev() {
            if w != 0 {
                return i as u32 * 32 + 32 - w.leading_zeros();
            }
        }
        0
    }

    fn bit(&self, i: u32) -> bool {
        self.0.get((i / 32) as usize).is_some_and(|&w| w >> (i % 32) & 1 == 1)
    }

    /// The first `k` bits, whether any bit below them is set, and the
    /// exponent of the last of them.
    fn top(&self, k: u32) -> (u128, bool, i32) {
        let b = self.bits();
        if b <= k {
            let mut v = 0u128;
            for i in (0..b).rev() {
                v = v << 1 | self.bit(i) as u128;
            }
            return (v, false, 0);
        }
        let mut v = 0u128;
        for i in (b - k..b).rev() {
            v = v << 1 | self.bit(i) as u128;
        }
        let sticky = (0..b - k).any(|i| self.bit(i));
        (v, sticky, (b - k) as i32)
    }

    fn shl(&mut self, s: u32) {
        let (words, bits) = ((s / 32) as usize, s % 32);
        let mut out = vec![0u32; words];
        let mut carry = 0u32;
        for &w in &self.0 {
            out.push(if bits == 0 { w } else { w << bits | carry });
            carry = if bits == 0 { 0 } else { w >> (32 - bits) };
        }
        if carry != 0 {
            out.push(carry);
        }
        self.0 = out;
    }

    fn cmp_shifted(&self, d: &Big, s: u32) -> Ordering {
        let n = self.bits().max(d.bits() + s);
        for i in (0..n).rev() {
            let a = self.bit(i);
            let b = i >= s && d.bit(i - s);
            if a != b {
                return a.cmp(&b);
            }
        }
        Ordering::Equal
    }

    fn sub_shifted(&mut self, d: &Big, s: u32) {
        let mut t = d.clone();
        t.shl(s);
        let mut borrow = 0i64;
        for i in 0..self.0.len() {
            let v = self.0[i] as i64 - *t.0.get(i).unwrap_or(&0) as i64 - borrow;
            borrow = (v < 0) as i64;
            self.0[i] = v.rem_euclid(1 << 32) as u32;
        }
    }

    /// The quotient by `d`, which fits in 128 bits, and whether a
    /// remainder is left.
    fn div_small_quotient(mut self, d: &Big) -> (u128, bool) {
        let span = self.bits().saturating_sub(d.bits()) + 1;
        let mut q = 0u128;
        for s in (0..span.min(127)).rev() {
            q <<= 1;
            if self.cmp_shifted(d, s) != Ordering::Less {
                self.sub_shifted(d, s);
                q |= 1;
            }
        }
        (q, !self.is_zero())
    }
}

#[cfg(test)]
mod tests {
    use super::F80;

    #[test]
    fn extended_arithmetic() {
        let p = |s: &str| F80::parse(s).unwrap();
        // 2^53 + 1 is exact in 64 bits and not in 53.
        assert_eq!(p("9007199254740992.0").add(p("1.0")), p("9007199254740993.0"));
        assert_eq!(p("18446744073709551616.0").add(p("1.0")), p("18446744073709551616.0"));
        assert_ne!(p("1.0").add(p("0x1p-63")), p("1.0"));
        assert_eq!(p("1.0").add(p("0x1p-64")), p("1.0"));
        let big = p("0x1.fffffffffffffffep16383");
        assert!(big.is_finite());
        assert_eq!(big.div(p("0x1p16383")).cmp(p("1.99")), Some(std::cmp::Ordering::Greater));
        assert_eq!(p("1.1e4932").cmp(p("1.0e4932")), Some(std::cmp::Ordering::Greater));
        assert_eq!(p("1.2e4932"), F80::Inf(false));
        assert_eq!(p("0.1").to_f64(false), 0.1);
        assert_eq!(p("3.0").mul(p("0.5")).to_f64(false), 1.5);
        assert_eq!(p("1.0").div(p("3.0")).mul(p("3.0")).to_f64(false), 1.0);
        assert_eq!(F80::from_f64(-2.5).trunc(), Some(-2));
        assert_eq!(F80::from_int(-7).to_f64(true), -7.0);
        assert_eq!(p("0x1p-16445").cmp(F80::ZERO), Some(std::cmp::Ordering::Greater));
        assert_eq!(p("0x1p-16446"), F80::ZERO);
    }
}
