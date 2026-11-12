//! The type table of analysis (`DESIGN-rewrite.md` section 2). A type is
//! a `QType`: a `Type` with its cv-qualifiers. Build step 3 has the
//! fundamental types, pointers, references, arrays, functions, and
//! enumerations. Classes, pointers to members, and the dependent forms of
//! model section 1 (`DepType`) come with steps 4 and 5.
//!
//! The rules on which types exist and how they relate sit in
//! `foundations::types`; this module holds the data and the queries that
//! every page uses. Sizes come from `target`.

use std::rc::Rc;

/// The integer types, `bool` and the character types included (TYP-3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IntK {
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
    LongLong,
    ULongLong,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FloatK {
    Float,
    Double,
    LongDouble,
}

/// An index into `Unit::enums`.
pub type EnumId = usize;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Void,
    Int(IntK),
    Float(FloatK),
    NullPtr,
    Ptr(Rc<QType>),
    /// A reference: the referred type, and true for an rvalue reference.
    Ref(Rc<QType>, bool),
    /// An array: the element type and the bound, if known.
    Array(Rc<QType>, Option<u64>),
    Func(Rc<FnType>),
    Enum(EnumId),
    /// The type of a construct after an error was reported, or of a
    /// construct the front end does not model. No check fires on it.
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct QType {
    pub t: Type,
    pub c: bool,
    pub v: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnType {
    pub ret: QType,
    pub params: Vec<QType>,
    pub variadic: bool,
    pub noexcept: bool,
}

impl IntK {
    pub fn signed(self) -> bool {
        use IntK::*;
        match self {
            Char => true,
            SChar | Short | Int | Long | LongLong => true,
            WChar => true,
            _ => false,
        }
    }
    /// The integer conversion rank (CONV rules): `bool` lowest.
    pub fn rank(self) -> u32 {
        use IntK::*;
        match self {
            Bool => 0,
            Char | SChar | UChar | Char8 => 1,
            Short | UShort | Char16 => 2,
            Int | UInt | WChar | Char32 => 3,
            Long | ULong => 4,
            LongLong | ULongLong => 5,
        }
    }
    pub fn unsigned_of(self) -> IntK {
        use IntK::*;
        match self {
            Char | SChar => UChar,
            Short => UShort,
            Int => UInt,
            Long => ULong,
            LongLong => ULongLong,
            k => k,
        }
    }
    pub fn is_char(self) -> bool {
        use IntK::*;
        matches!(self, Char | SChar | UChar | WChar | Char8 | Char16 | Char32)
    }
    pub fn name(self) -> &'static str {
        use IntK::*;
        match self {
            Bool => "bool",
            Char => "char",
            SChar => "signed char",
            UChar => "unsigned char",
            WChar => "wchar_t",
            Char8 => "char8_t",
            Char16 => "char16_t",
            Char32 => "char32_t",
            Short => "short",
            UShort => "unsigned short",
            Int => "int",
            UInt => "unsigned int",
            Long => "long",
            ULong => "unsigned long",
            LongLong => "long long",
            ULongLong => "unsigned long long",
        }
    }
}

impl FloatK {
    pub fn rank(self) -> u32 {
        match self {
            FloatK::Float => 1,
            FloatK::Double => 2,
            FloatK::LongDouble => 3,
        }
    }
}

impl Type {
    pub fn q(self) -> QType {
        QType { t: self, c: false, v: false }
    }
    pub fn int() -> Type {
        Type::Int(IntK::Int)
    }
    pub fn bool() -> Type {
        Type::Int(IntK::Bool)
    }
}

impl QType {
    pub fn new(t: Type) -> QType {
        QType { t, c: false, v: false }
    }
    pub fn error() -> QType {
        QType::new(Type::Error)
    }
    pub fn int() -> QType {
        QType::new(Type::int())
    }
    pub fn bool() -> QType {
        QType::new(Type::bool())
    }
    pub fn void() -> QType {
        QType::new(Type::Void)
    }
    pub fn ptr_to(q: QType) -> QType {
        QType::new(Type::Ptr(Rc::new(q)))
    }
    pub fn lref(q: QType) -> QType {
        QType::new(Type::Ref(Rc::new(q), false))
    }
    pub fn unqual(&self) -> QType {
        QType::new(self.t.clone())
    }
    pub fn with_cv(&self, c: bool, v: bool) -> QType {
        match &self.t {
            // A cv-qualifier on an array type applies to its elements.
            Type::Array(e, n) => QType::new(Type::Array(Rc::new(e.with_cv(c, v)), *n)),
            Type::Func(_) | Type::Ref(..) => self.clone(),
            _ => QType { t: self.t.clone(), c: self.c || c, v: self.v || v },
        }
    }
    /// Contains `Type::Error` anywhere.
    pub fn is_error(&self) -> bool {
        match &self.t {
            Type::Error => true,
            Type::Ptr(q) | Type::Ref(q, _) | Type::Array(q, _) => q.is_error(),
            Type::Func(f) => f.ret.is_error() || f.params.iter().any(|p| p.is_error()),
            _ => false,
        }
    }
    pub fn is_void(&self) -> bool {
        matches!(self.t, Type::Void)
    }
    pub fn int_kind(&self) -> Option<IntK> {
        match self.t {
            Type::Int(k) => Some(k),
            _ => None,
        }
    }
    pub fn is_bool(&self) -> bool {
        matches!(self.t, Type::Int(IntK::Bool))
    }
    pub fn is_integral(&self) -> bool {
        matches!(self.t, Type::Int(_))
    }
    pub fn is_floating(&self) -> bool {
        matches!(self.t, Type::Float(_))
    }
    pub fn is_arithmetic(&self) -> bool {
        self.is_integral() || self.is_floating()
    }
    pub fn is_enum(&self) -> bool {
        matches!(self.t, Type::Enum(_))
    }
    pub fn is_ptr(&self) -> bool {
        matches!(self.t, Type::Ptr(_))
    }
    pub fn is_ref(&self) -> bool {
        matches!(self.t, Type::Ref(..))
    }
    pub fn is_array(&self) -> bool {
        matches!(self.t, Type::Array(..))
    }
    pub fn is_func(&self) -> bool {
        matches!(self.t, Type::Func(_))
    }
    pub fn is_nullptr(&self) -> bool {
        matches!(self.t, Type::NullPtr)
    }
    /// A scalar type (TYP rules): arithmetic, enumeration, pointer, and
    /// `std::nullptr_t`.
    pub fn is_scalar(&self) -> bool {
        matches!(self.t, Type::Int(_) | Type::Float(_) | Type::Enum(_) | Type::Ptr(_) | Type::NullPtr)
    }
    /// The pointed-to type of a pointer.
    pub fn pointee(&self) -> Option<&QType> {
        match &self.t {
            Type::Ptr(q) => Some(q),
            _ => None,
        }
    }
    /// The referred type of a reference, or the type itself.
    pub fn nonref(&self) -> QType {
        match &self.t {
            Type::Ref(q, _) => (**q).clone(),
            _ => self.clone(),
        }
    }
    pub fn elem(&self) -> Option<&QType> {
        match &self.t {
            Type::Array(e, _) => Some(e),
            _ => None,
        }
    }
    pub fn func(&self) -> Option<&FnType> {
        match &self.t {
            Type::Func(f) => Some(f),
            _ => None,
        }
    }
    /// The innermost element type of an array, or the type.
    pub fn base_elem(&self) -> &QType {
        match &self.t {
            Type::Array(e, _) => e.base_elem(),
            _ => self,
        }
    }
    /// The type is const, or an array of const elements.
    pub fn is_const(&self) -> bool {
        self.c || self.elem().is_some_and(|e| e.is_const())
    }
    /// The same type up to top-level cv-qualifiers.
    pub fn same_unqual(&self, o: &QType) -> bool {
        self.t == o.t
    }
    /// An array whose bound is not known, or `void` (TYP rules on
    /// incomplete types; classes come with step 4).
    pub fn is_incomplete(&self) -> bool {
        match &self.t {
            Type::Void => true,
            Type::Array(e, n) => n.is_none() || e.is_incomplete(),
            _ => false,
        }
    }
    /// The type with the cv-qualifiers of every array level and of the
    /// elements removed.
    pub fn unqual_deep(&self) -> QType {
        match &self.t {
            Type::Array(e, n) => QType::new(Type::Array(Rc::new(e.unqual_deep()), *n)),
            _ => self.unqual(),
        }
    }
    /// The spelling of the type, for diagnostics.
    pub fn show(&self) -> String {
        show(self, "")
    }
}

fn cv(q: &QType) -> &'static str {
    match (q.c, q.v) {
        (true, true) => "const volatile ",
        (true, false) => "const ",
        (false, true) => "volatile ",
        _ => "",
    }
}

fn show(q: &QType, inner: &str) -> String {
    match &q.t {
        Type::Void => format!("{}void{}", cv(q), inner),
        Type::Int(k) => format!("{}{}{}", cv(q), k.name(), inner),
        Type::Float(FloatK::Float) => format!("{}float{}", cv(q), inner),
        Type::Float(FloatK::Double) => format!("{}double{}", cv(q), inner),
        Type::Float(FloatK::LongDouble) => format!("{}long double{}", cv(q), inner),
        Type::NullPtr => format!("{}std::nullptr_t{}", cv(q), inner),
        Type::Enum(e) => format!("{}enum#{}{}", cv(q), e, inner),
        Type::Error => "<error>".into(),
        Type::Ptr(p) => {
            let c = cv(q).trim_end();
            let s = if c.is_empty() { format!("*{}", inner) } else { format!("* {}{}", c, inner) };
            if matches!(p.t, Type::Array(..) | Type::Func(_)) {
                show(p, &format!("({})", s))
            } else {
                show(p, &format!(" {}", s.trim_start()).replace("  ", " "))
            }
        }
        Type::Ref(p, rv) => {
            let s = format!("{}{}", if *rv { "&&" } else { "&" }, inner);
            if matches!(p.t, Type::Array(..) | Type::Func(_)) {
                show(p, &format!("({})", s))
            } else {
                show(p, &format!(" {}", s))
            }
        }
        Type::Array(e, n) => show(e, &format!("{}[{}]", inner, n.map(|n| n.to_string()).unwrap_or_default())),
        Type::Func(f) => {
            let ps: Vec<String> = f.params.iter().map(|p| p.show()).collect();
            show(&f.ret, &format!("{}({}{})", inner, ps.join(", "), if f.variadic { ", ..." } else { "" }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spelling_of_compound_types() {
        let pi = QType::ptr_to(QType::int());
        assert_eq!(pi.show(), "int *");
        let arr = QType::new(Type::Array(Rc::new(QType::int()), Some(3)));
        assert_eq!(QType::ptr_to(arr.clone()).show(), "int(*)[3]");
        assert_eq!(arr.with_cv(true, false).show(), "const int[3]");
        assert!(arr.with_cv(true, false).is_const());
    }

    #[test]
    fn ranks_and_signedness() {
        assert!(IntK::Int.rank() < IntK::Long.rank());
        assert!(!IntK::UInt.signed());
        assert_eq!(IntK::Long.unsigned_of(), IntK::ULong);
        assert!(QType::error().is_error());
        assert!(QType::ptr_to(QType::error()).is_error());
    }
}
