// Types. Sizes follow the x86-64 Linux targets of the test matrix (LP64),
// or ILP32 with `-m32`. The spec leaves sizes implementation-defined
// (UN-15, MUL-15), so these numbers are a stated choice.

use std::cell::Cell;

thread_local! {
    pub static ILP32: Cell<bool> = Cell::new(false);
}

pub fn ilp32() -> bool {
    ILP32.with(|c| c.get())
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    Void,
    Bool,
    Char,
    SChar,
    UChar,
    WChar,
    Char8,
    Char16,
    Char32,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    LLong,
    ULLong,
    Float,
    Double,
    LDouble,
    /// Extended floating types (PRIM-22); treated as opaque floats.
    ExtFloat,
    NullPtr,
    Ptr(Box<QTy>),
    LRef(Box<QTy>),
    RRef(Box<QTy>),
    Array(Box<QTy>, Option<u64>),
    Func(Box<FnTy>),
    Enum(usize),
    Class(usize),
    MemPtr(usize, Box<QTy>),
    /// A type the front end cannot model: dependent on a template
    /// parameter, or produced by a construct it does not understand.
    /// Checks never fire on it.
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub struct QTy {
    pub t: Ty,
    pub c: bool,
    pub v: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnTy {
    pub ret: QTy,
    pub params: Vec<QTy>,
    pub variadic: bool,
    pub noexcept: bool,
    pub cv_c: bool,
    pub cv_v: bool,
    pub refq: u8,
}

impl QTy {
    pub fn new(t: Ty) -> QTy {
        QTy { t, c: false, v: false }
    }
    pub fn unknown() -> QTy {
        QTy::new(Ty::Unknown)
    }
    pub fn int() -> QTy {
        QTy::new(Ty::Int)
    }
    pub fn bool_() -> QTy {
        QTy::new(Ty::Bool)
    }
    pub fn unqual(&self) -> QTy {
        QTy::new(self.t.clone())
    }
    pub fn with_const(mut self, c: bool) -> QTy {
        self.c |= c;
        self
    }
    pub fn is_unknown(&self) -> bool {
        self.t.contains_unknown()
    }
}

impl Ty {
    pub fn contains_unknown(&self) -> bool {
        match self {
            Ty::Unknown => true,
            Ty::Ptr(q) | Ty::LRef(q) | Ty::RRef(q) | Ty::Array(q, _) | Ty::MemPtr(_, q) => q.t.contains_unknown(),
            Ty::Func(f) => f.ret.t.contains_unknown() || f.params.iter().any(|p| p.t.contains_unknown()),
            _ => false,
        }
    }
    pub fn is_integral(&self) -> bool {
        matches!(
            self,
            Ty::Bool
                | Ty::Char
                | Ty::SChar
                | Ty::UChar
                | Ty::WChar
                | Ty::Char8
                | Ty::Char16
                | Ty::Char32
                | Ty::Short
                | Ty::UShort
                | Ty::Int
                | Ty::UInt
                | Ty::Long
                | Ty::ULong
                | Ty::LLong
                | Ty::ULLong
        )
    }
    pub fn is_floating(&self) -> bool {
        matches!(self, Ty::Float | Ty::Double | Ty::LDouble | Ty::ExtFloat)
    }
    pub fn is_arithmetic(&self) -> bool {
        self.is_integral() || self.is_floating()
    }
    pub fn is_pointer(&self) -> bool {
        matches!(self, Ty::Ptr(_))
    }
    pub fn is_ref(&self) -> bool {
        matches!(self, Ty::LRef(_) | Ty::RRef(_))
    }
    pub fn is_scalar(&self, scoped: &dyn Fn(usize) -> bool) -> bool {
        let _ = scoped;
        self.is_arithmetic() || matches!(self, Ty::Ptr(_) | Ty::NullPtr | Ty::Enum(_) | Ty::MemPtr(..))
    }
    pub fn is_signed(&self) -> bool {
        matches!(self, Ty::Char | Ty::SChar | Ty::Short | Ty::Int | Ty::Long | Ty::LLong | Ty::WChar)
    }
    /// Bit width of an integral type on the target.
    pub fn int_bits(&self) -> u32 {
        match self {
            Ty::Bool => 1,
            Ty::Char | Ty::SChar | Ty::UChar | Ty::Char8 => 8,
            Ty::Short | Ty::UShort | Ty::Char16 => 16,
            Ty::Int | Ty::UInt | Ty::WChar | Ty::Char32 => 32,
            Ty::Long | Ty::ULong => {
                if ilp32() {
                    32
                } else {
                    64
                }
            }
            Ty::LLong | Ty::ULLong => 64,
            _ => 64,
        }
    }
    /// Integer conversion rank, used by the usual arithmetic conversions.
    pub fn rank(&self) -> u32 {
        match self {
            Ty::Bool => 0,
            Ty::Char | Ty::SChar | Ty::UChar | Ty::Char8 => 1,
            Ty::Short | Ty::UShort | Ty::Char16 => 2,
            Ty::Int | Ty::UInt | Ty::WChar | Ty::Char32 => 3,
            Ty::Long | Ty::ULong => 4,
            Ty::LLong | Ty::ULLong => 5,
            _ => 0,
        }
    }
    pub fn int_min(&self) -> i128 {
        if self.is_signed() {
            -(1i128 << (self.int_bits() - 1))
        } else {
            0
        }
    }
    pub fn int_max(&self) -> i128 {
        if *self == Ty::Bool {
            return 1;
        }
        if self.is_signed() {
            (1i128 << (self.int_bits() - 1)) - 1
        } else {
            (1i128 << self.int_bits()) - 1
        }
    }
    pub fn to_unsigned(&self) -> Ty {
        match self {
            Ty::Char | Ty::SChar => Ty::UChar,
            Ty::Short => Ty::UShort,
            Ty::Int => Ty::UInt,
            Ty::Long => Ty::ULong,
            Ty::LLong => Ty::ULLong,
            t => t.clone(),
        }
    }
    pub fn size_t() -> Ty {
        if ilp32() {
            Ty::UInt
        } else {
            Ty::ULong
        }
    }
    pub fn ptrdiff_t() -> Ty {
        if ilp32() {
            Ty::Int
        } else {
            Ty::Long
        }
    }
    pub fn float_rank(&self) -> u32 {
        match self {
            Ty::Float => 1,
            Ty::Double => 2,
            Ty::LDouble => 3,
            Ty::ExtFloat => 2,
            _ => 0,
        }
    }
}

/// Add cv-qualifiers. On an array they apply to the elements (CV-6); on a
/// function or reference type they have no effect (CV-6, TYN-1).
pub fn add_cv(q: &QTy, c: bool, v: bool) -> QTy {
    match &q.t {
        Ty::Array(e, n) => {
            let e2 = add_cv(e, c, v);
            QTy { c: e2.c, v: e2.v, t: Ty::Array(Box::new(e2), *n) }
        }
        Ty::Func(_) | Ty::LRef(_) | Ty::RRef(_) => q.clone(),
        _ => QTy { t: q.t.clone(), c: q.c || c, v: q.v || v },
    }
}

/// Wrap an integer value into the range of an integer type (CONV-2).
pub fn wrap_int(v: i128, t: &Ty) -> i128 {
    if *t == Ty::Bool {
        return (v != 0) as i128;
    }
    let bits = t.int_bits();
    let m = 1i128 << bits;
    let mut r = v.rem_euclid(m);
    if t.is_signed() && r > t.int_max() {
        r -= m;
    }
    r
}

pub fn fits(v: i128, t: &Ty) -> bool {
    v >= t.int_min() && v <= t.int_max()
}

/// Strip references: the type of an expression never has reference type.
pub fn nonref(q: &QTy) -> QTy {
    match &q.t {
        Ty::LRef(i) | Ty::RRef(i) => (**i).clone(),
        _ => q.clone(),
    }
}

pub fn same_unqual(a: &QTy, b: &QTy) -> bool {
    a.t == b.t
}

pub fn base_name(t: &Ty) -> &'static str {
    match t {
        Ty::Void => "void",
        Ty::Bool => "bool",
        Ty::Char => "char",
        Ty::SChar => "signed char",
        Ty::UChar => "unsigned char",
        Ty::WChar => "wchar_t",
        Ty::Char8 => "char8_t",
        Ty::Char16 => "char16_t",
        Ty::Char32 => "char32_t",
        Ty::Short => "short",
        Ty::UShort => "unsigned short",
        Ty::Int => "int",
        Ty::UInt => "unsigned int",
        Ty::Long => "long",
        Ty::ULong => "unsigned long",
        Ty::LLong => "long long",
        Ty::ULLong => "unsigned long long",
        Ty::Float => "float",
        Ty::Double => "double",
        Ty::LDouble => "long double",
        Ty::ExtFloat => "extended float",
        Ty::NullPtr => "std::nullptr_t",
        Ty::Unknown => "<unknown>",
        _ => "",
    }
}

pub fn show(q: &QTy) -> String {
    let cv = format!("{}{}", if q.c { "const " } else { "" }, if q.v { "volatile " } else { "" });
    match &q.t {
        Ty::Ptr(i) => format!("{}{}*", cv, show(i)),
        Ty::LRef(i) => format!("{}&", show(i)),
        Ty::RRef(i) => format!("{}&&", show(i)),
        Ty::Array(i, n) => format!("{}[{}]", show(i), n.map(|x| x.to_string()).unwrap_or_default()),
        Ty::Func(f) => format!("{}({})", show(&f.ret), f.params.iter().map(show).collect::<Vec<_>>().join(", ")),
        Ty::Enum(i) => format!("{}enum#{}", cv, i),
        Ty::Class(i) => format!("{}class#{}", cv, i),
        Ty::MemPtr(c, i) => format!("{} class#{}::*", show(i), c),
        t => format!("{}{}", cv, base_name(t)),
    }
}

/// Integer promotion (glossary "integer promotion", CONV-1). Enumerations
/// are handled by the caller, which knows the underlying type.
pub fn promote(t: &Ty) -> Ty {
    match t {
        Ty::Bool | Ty::Char | Ty::SChar | Ty::UChar | Ty::Short | Ty::UShort | Ty::Char8 => Ty::Int,
        // glossary: wchar_t, char16_t, char32_t promote to the narrowest
        // standard type that holds all values, int before unsigned int.
        Ty::Char16 => Ty::Int,
        Ty::Char32 => Ty::UInt,
        Ty::WChar => Ty::Int,
        t => t.clone(),
    }
}

/// Usual arithmetic conversions (glossary), on promoted or floating types.
pub fn usual_arith(a: &Ty, b: &Ty) -> Ty {
    if a.is_floating() || b.is_floating() {
        if a.float_rank() >= b.float_rank() {
            return a.clone();
        }
        return b.clone();
    }
    let a = promote(a);
    let b = promote(b);
    if a == b {
        return a;
    }
    if a.is_signed() == b.is_signed() {
        return if a.rank() >= b.rank() { a } else { b };
    }
    let (s, u) = if a.is_signed() { (a, b) } else { (b, a) };
    if u.rank() >= s.rank() {
        return u;
    }
    if s.int_bits() > u.int_bits() {
        return s;
    }
    s.to_unsigned()
}

pub fn size_of(t: &Ty, classes: &dyn Fn(usize) -> Option<(u64, u64)>, enums: &dyn Fn(usize) -> Ty) -> Option<u64> {
    Some(match t {
        Ty::Bool | Ty::Char | Ty::SChar | Ty::UChar | Ty::Char8 => 1,
        Ty::Short | Ty::UShort | Ty::Char16 => 2,
        Ty::Int | Ty::UInt | Ty::WChar | Ty::Char32 | Ty::Float => 4,
        Ty::Long | Ty::ULong => {
            if ilp32() {
                4
            } else {
                8
            }
        }
        Ty::LLong | Ty::ULLong | Ty::Double => 8,
        Ty::LDouble => {
            if ilp32() {
                12
            } else {
                16
            }
        }
        Ty::Ptr(_) | Ty::NullPtr => {
            if ilp32() {
                4
            } else {
                8
            }
        }
        Ty::MemPtr(_, i) => {
            let p = if ilp32() { 4 } else { 8 };
            if matches!(i.t, Ty::Func(_)) {
                2 * p
            } else {
                p
            }
        }
        Ty::LRef(i) | Ty::RRef(i) => return size_of(&i.t, classes, enums),
        Ty::Array(i, Some(n)) => return size_of(&i.t, classes, enums).map(|s| s * n),
        Ty::Enum(e) => return size_of(&enums(*e), classes, enums),
        Ty::Class(c) => return classes(*c).map(|x| x.0),
        _ => return None,
    })
}

pub fn align_of(t: &Ty, classes: &dyn Fn(usize) -> Option<(u64, u64)>, enums: &dyn Fn(usize) -> Ty) -> Option<u64> {
    match t {
        Ty::Array(i, _) => align_of(&i.t, classes, enums),
        Ty::LRef(i) | Ty::RRef(i) => align_of(&i.t, classes, enums),
        Ty::Class(c) => classes(*c).map(|x| x.1),
        Ty::Enum(e) => align_of(&enums(*e), classes, enums),
        Ty::LDouble => Some(if ilp32() { 4 } else { 16 }),
        Ty::LLong | Ty::ULLong | Ty::Double if ilp32() => Some(4),
        Ty::MemPtr(..) => Some(if ilp32() { 4 } else { 8 }),
        t => size_of(t, classes, enums),
    }
}
