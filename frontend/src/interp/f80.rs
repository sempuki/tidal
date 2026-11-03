// The x87 extended format of `long double` on the matrix targets (TYP-10,
// MUL-20): a sign, a 15-bit exponent, and a 64-bit significand with an
// explicit integer bit. Arithmetic rounds to nearest, ties to even.
// Subnormal values are flushed to zero, an approximation.

use std::cmp::Ordering;

/// The largest and smallest exponents of a normal value.
const EMAX: i32 = 16383;
const EMIN: i32 = -16382;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F80 {
    Zero(bool),
    Inf(bool),
    NaN,
    /// The value `mant * 2^(exp - 63)`, with bit 63 of `mant` set.
    Fin { neg: bool, exp: i32, mant: u64 },
}

impl F80 {
    /// The value `m * 2^e`, rounded. The lowest bit of `m` holds the
    /// bits that a shift dropped (a sticky bit).
    fn from_parts(neg: bool, e: i32, m: u128) -> F80 {
        if m == 0 {
            return F80::Zero(neg);
        }
        let lz = m.leading_zeros() as i32;
        let m = m << lz;
        let mut exp = e - lz + 127;
        let mut mant = (m >> 64) as u64;
        let rest = m as u64;
        let half = 1u64 << 63;
        if rest > half || (rest == half && mant & 1 == 1) {
            let (n, carry) = mant.overflowing_add(1);
            mant = n;
            if carry {
                mant = 1 << 63;
                exp += 1;
            }
        }
        if exp > EMAX {
            return F80::Inf(neg);
        }
        if exp < EMIN {
            return F80::Zero(neg);
        }
        F80::Fin { neg, exp, mant }
    }

    pub fn from_i128(x: i128) -> F80 {
        F80::from_parts(x < 0, 0, x.unsigned_abs())
    }

    pub fn from_f64(x: f64) -> F80 {
        if x.is_nan() {
            return F80::NaN;
        }
        if x.is_infinite() {
            return F80::Inf(x < 0.0);
        }
        if x == 0.0 {
            return F80::Zero(x.is_sign_negative());
        }
        let bits = x.to_bits();
        let neg = bits >> 63 == 1;
        let be = ((bits >> 52) & 0x7ff) as i32;
        let frac = bits & ((1u64 << 52) - 1);
        let (m, e) = if be == 0 { (frac, -1074) } else { (frac | (1u64 << 52), be - 1075) };
        F80::from_parts(neg, e, m as u128)
    }

    pub fn to_f64(self) -> f64 {
        match self {
            F80::Zero(n) => {
                if n {
                    -0.0
                } else {
                    0.0
                }
            }
            F80::Inf(n) => {
                if n {
                    f64::NEG_INFINITY
                } else {
                    f64::INFINITY
                }
            }
            F80::NaN => f64::NAN,
            F80::Fin { neg, exp, mant } => {
                // The u64 conversion rounds to 53 bits; the scaling is exact
                // unless the result overflows or is subnormal.
                let mut v = mant as f64;
                let mut k = exp - 63;
                while k > 0 {
                    let s = k.min(1000);
                    v *= 2f64.powi(s);
                    k -= s;
                }
                while k < 0 {
                    let s = (-k).min(1000);
                    v /= 2f64.powi(s);
                    k += s;
                }
                if neg {
                    -v
                } else {
                    v
                }
            }
        }
    }

    /// The value truncated toward zero, or None when it does not fit.
    pub fn to_i128(self) -> Option<i128> {
        match self {
            F80::Zero(_) => Some(0),
            F80::Inf(_) | F80::NaN => None,
            F80::Fin { neg, exp, mant } => {
                if exp < 0 {
                    return Some(0);
                }
                if exp >= 126 {
                    return None;
                }
                let m = mant as u128;
                let v = if exp >= 63 { m << (exp - 63) } else { m >> (63 - exp) };
                let v = v as i128;
                Some(if neg { -v } else { v })
            }
        }
    }

    pub fn is_zero(self) -> bool {
        matches!(self, F80::Zero(_))
    }

    pub fn is_finite(self) -> bool {
        matches!(self, F80::Zero(_) | F80::Fin { .. })
    }

    pub fn neg(self) -> F80 {
        match self {
            F80::Zero(n) => F80::Zero(!n),
            F80::Inf(n) => F80::Inf(!n),
            F80::NaN => F80::NaN,
            F80::Fin { neg, exp, mant } => F80::Fin { neg: !neg, exp, mant },
        }
    }

    fn sign(self) -> bool {
        match self {
            F80::Zero(n) | F80::Inf(n) => n,
            F80::NaN => false,
            F80::Fin { neg, .. } => neg,
        }
    }

    pub fn add(self, o: F80) -> F80 {
        match (self, o) {
            (F80::NaN, _) | (_, F80::NaN) => F80::NaN,
            (F80::Inf(a), F80::Inf(b)) => {
                if a == b {
                    F80::Inf(a)
                } else {
                    F80::NaN
                }
            }
            (F80::Inf(a), _) | (_, F80::Inf(a)) => F80::Inf(a),
            (F80::Zero(a), F80::Zero(b)) => F80::Zero(a && b),
            (F80::Zero(_), x) | (x, F80::Zero(_)) => x,
            (F80::Fin { neg: na, exp: ea, mant: ma }, F80::Fin { neg: nb, exp: eb, mant: mb }) => {
                // Each significand with 63 guard bits below it.
                let (na, ea, ma, nb, eb, mb) = if ea >= eb { (na, ea, ma, nb, eb, mb) } else { (nb, eb, mb, na, ea, ma) };
                let a = (ma as u128) << 63;
                let full = (mb as u128) << 63;
                let d = (ea - eb) as u32;
                let b = if d >= 127 {
                    1
                } else {
                    let s = full >> d;
                    s | ((s << d != full) as u128)
                };
                if na == nb {
                    F80::from_parts(na, ea - 126, a + b)
                } else if a >= b {
                    if a == b {
                        return F80::Zero(false);
                    }
                    F80::from_parts(na, ea - 126, a - b)
                } else {
                    F80::from_parts(nb, ea - 126, b - a)
                }
            }
        }
    }

    pub fn sub(self, o: F80) -> F80 {
        self.add(o.neg())
    }

    pub fn mul(self, o: F80) -> F80 {
        let neg = self.sign() != o.sign();
        match (self, o) {
            (F80::NaN, _) | (_, F80::NaN) => F80::NaN,
            (F80::Inf(_), F80::Zero(_)) | (F80::Zero(_), F80::Inf(_)) => F80::NaN,
            (F80::Inf(_), _) | (_, F80::Inf(_)) => F80::Inf(neg),
            (F80::Zero(_), _) | (_, F80::Zero(_)) => F80::Zero(neg),
            (F80::Fin { exp: ea, mant: ma, .. }, F80::Fin { exp: eb, mant: mb, .. }) => F80::from_parts(neg, ea + eb - 126, ma as u128 * mb as u128),
        }
    }

    pub fn div(self, o: F80) -> F80 {
        let neg = self.sign() != o.sign();
        match (self, o) {
            (F80::NaN, _) | (_, F80::NaN) => F80::NaN,
            (F80::Inf(_), F80::Inf(_)) | (F80::Zero(_), F80::Zero(_)) => F80::NaN,
            (F80::Inf(_), _) => F80::Inf(neg),
            (_, F80::Inf(_)) => F80::Zero(neg),
            (F80::Zero(_), _) => F80::Zero(neg),
            (_, F80::Zero(_)) => F80::Inf(neg),
            (F80::Fin { exp: ea, mant: ma, .. }, F80::Fin { exp: eb, mant: mb, .. }) => {
                let num = (ma as u128) << 63;
                let d = mb as u128;
                let q1 = num / d;
                let r1 = num % d;
                let q2 = (r1 << 64) / d;
                let r2 = (r1 << 64) % d;
                let q = (q1 << 64) | q2 | ((r2 != 0) as u128);
                F80::from_parts(neg, ea - eb - 127, q)
            }
        }
    }

    pub fn partial_cmp(self, o: F80) -> Option<Ordering> {
        let key = |x: F80| -> Option<(i8, i32, u64)> {
            match x {
                F80::NaN => None,
                F80::Zero(_) => Some((0, 0, 0)),
                F80::Inf(n) => Some((if n { -2 } else { 2 }, 0, 0)),
                F80::Fin { neg, exp, mant } => Some((if neg { -1 } else { 1 }, exp, mant)),
            }
        };
        let (a, b) = (key(self)?, key(o)?);
        if a.0 != b.0 {
            return Some(a.0.cmp(&b.0));
        }
        let c = (a.1, a.2).cmp(&(b.1, b.2));
        Some(if a.0 < 0 { c.reverse() } else { c })
    }

    fn pow10(n: u32) -> F80 {
        let mut r = F80::from_i128(1);
        let mut b = F80::from_i128(10);
        let mut n = n;
        while n > 0 {
            if n & 1 == 1 {
                r = r.mul(b);
            }
            b = b.mul(b);
            n >>= 1;
        }
        r
    }

    /// The value of a floating literal without its suffix. A decimal
    /// literal is computed in this format, so it can be off by a few
    /// units in the last place.
    pub fn parse(text: &str) -> Option<F80> {
        let t: String = text.chars().filter(|&c| c != '\'').collect();
        let t = t.as_str();
        if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
            let (mpart, epart) = match h.find(['p', 'P']) {
                Some(i) => (&h[..i], &h[i + 1..]),
                None => (h, "0"),
            };
            let mut m: u128 = 0;
            let mut e: i32 = epart.parse().ok()?;
            let mut frac = false;
            let mut sticky = false;
            for c in mpart.chars() {
                if c == '.' {
                    frac = true;
                    continue;
                }
                let d = c.to_digit(16)? as u128;
                if m >> 120 != 0 {
                    sticky |= d != 0;
                    if !frac {
                        e += 4;
                    }
                    continue;
                }
                m = m * 16 + d;
                if frac {
                    e -= 4;
                }
            }
            return Some(F80::from_parts(false, e, m | sticky as u128));
        }
        let (mpart, epart) = match t.find(['e', 'E']) {
            Some(i) => (&t[..i], &t[i + 1..]),
            None => (t, "0"),
        };
        let mut e10: i32 = epart.parse().ok()?;
        let mut m: u128 = 0;
        let mut frac = false;
        for c in mpart.chars() {
            if c == '.' {
                frac = true;
                continue;
            }
            let d = c.to_digit(10)? as u128;
            if m > u128::MAX / 100 {
                if !frac {
                    e10 += 1;
                }
                continue;
            }
            m = m * 10 + d;
            if frac {
                e10 -= 1;
            }
        }
        let v = F80::from_parts(false, 0, m);
        Some(if e10 >= 0 { v.mul(F80::pow10(e10 as u32)) } else { v.div(F80::pow10((-e10) as u32)) })
    }

    /// The ten bytes of the format, little-endian.
    pub fn to_bytes(self) -> [u8; 10] {
        let (sign, e, m): (bool, u16, u64) = match self {
            F80::Zero(n) => (n, 0, 0),
            F80::Inf(n) => (n, 0x7fff, 1 << 63),
            F80::NaN => (false, 0x7fff, 3 << 62),
            F80::Fin { neg, exp, mant } => (neg, (exp + EMAX) as u16, mant),
        };
        let mut out = [0u8; 10];
        out[..8].copy_from_slice(&m.to_le_bytes());
        let se = e | if sign { 0x8000 } else { 0 };
        out[8..].copy_from_slice(&se.to_le_bytes());
        out
    }

    pub fn from_bytes(b: &[u8]) -> F80 {
        let m = u64::from_le_bytes(b[..8].try_into().unwrap());
        let se = u16::from_le_bytes(b[8..10].try_into().unwrap());
        let neg = se & 0x8000 != 0;
        let e = (se & 0x7fff) as i32;
        if e == 0x7fff {
            return if m << 1 == 0 { F80::Inf(neg) } else { F80::NaN };
        }
        if m == 0 {
            return F80::Zero(neg);
        }
        if e == 0 {
            return F80::Zero(neg);
        }
        F80::from_parts(neg, e - EMAX - 63, m as u128)
    }
}
