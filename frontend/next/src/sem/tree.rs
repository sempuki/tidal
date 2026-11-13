//! The typed tree: what analysis makes from the parser's nodes, and what
//! the machine runs (`DESIGN-rewrite.md` section 2). Each node has its
//! type, its value category, and its span. Conversions are explicit
//! nodes, so the machine does no type reasoning of its own.
//!
//! The dependence marks of model section 1 come with step 5. Build step 3
//! makes nodes for constructs outside classes and templates.

use super::types::QType;
use super::EntityId;
use crate::source::Span;
use std::rc::Rc;

/// The value category (`foundations/values`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cat {
    L,
    X,
    PR,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOp {
    Mul,
    Div,
    Rem,
    Add,
    Sub,
    Shl,
    Shr,
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
    Spaceship,
    BitAnd,
    BitXor,
    BitOr,
}

impl BOp {
    pub fn from_str(s: &str) -> Option<BOp> {
        Some(match s {
            "*" => BOp::Mul,
            "/" => BOp::Div,
            "%" => BOp::Rem,
            "+" => BOp::Add,
            "-" => BOp::Sub,
            "<<" => BOp::Shl,
            ">>" => BOp::Shr,
            "<" => BOp::Lt,
            ">" => BOp::Gt,
            "<=" => BOp::Le,
            ">=" => BOp::Ge,
            "==" => BOp::Eq,
            "!=" => BOp::Ne,
            "<=>" => BOp::Spaceship,
            "&" => BOp::BitAnd,
            "^" => BOp::BitXor,
            "|" => BOp::BitOr,
            _ => return None,
        })
    }
    pub fn is_comparison(self) -> bool {
        matches!(self, BOp::Lt | BOp::Gt | BOp::Le | BOp::Ge | BOp::Eq | BOp::Ne)
    }
    pub fn spelling(self) -> &'static str {
        match self {
            BOp::Mul => "*",
            BOp::Div => "/",
            BOp::Rem => "%",
            BOp::Add => "+",
            BOp::Sub => "-",
            BOp::Shl => "<<",
            BOp::Shr => ">>",
            BOp::Lt => "<",
            BOp::Gt => ">",
            BOp::Le => "<=",
            BOp::Ge => ">=",
            BOp::Eq => "==",
            BOp::Ne => "!=",
            BOp::Spaceship => "<=>",
            BOp::BitAnd => "&",
            BOp::BitXor => "^",
            BOp::BitOr => "|",
        }
    }
}

/// An implicit or explicit conversion, as the machine performs it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Conv {
    /// Read the object a glvalue names (VC rules).
    LToR,
    ArrayDecay,
    FuncDecay,
    /// Between arithmetic and enumeration types, `bool` excluded as a
    /// target (CONV rules).
    Arith,
    /// To `bool`: from arithmetic, enumeration, pointer, or `nullptr`.
    ToBool,
    /// A null pointer constant to a pointer type.
    NullPtr,
    /// A pointer conversion with no change of the address: to `void*`,
    /// or adding qualifiers.
    PtrNoOp,
    ToVoid,
    /// Make a temporary object from a prvalue (VC rules), for a
    /// reference to bind.
    Materialize,
    /// A glvalue of one type seen as another with the same object, as a
    /// reference to a less qualified or the same type gives.
    NoOp,
    /// A pointer to an integer type (`reinterpret_cast`).
    PtrToInt,
    /// An integer to a pointer type (`reinterpret_cast`).
    IntToPtr,
    /// A pointer to an unrelated object type (`reinterpret_cast`).
    PtrReinterpret,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UOp {
    Plus,
    Neg,
    Not,
    BitNot,
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub k: EK,
    pub ty: QType,
    pub cat: Cat,
    pub span: Span,
    /// Written in parentheses, which `decltype` reads (DTY rules).
    pub paren: bool,
}

#[derive(Clone, Debug)]
pub enum EK {
    Int(i128),
    Float(f64),
    /// A string literal: its code units with the terminating zero.
    Str(Rc<Vec<u32>>),
    Var(EntityId),
    /// A pseudo-destructor call: the life of the object ends (DTOR-6).
    EndLife(Box<Expr>),
    Func(EntityId),
    Conv(Conv, Box<Expr>),
    Unary(UOp, Box<Expr>),
    /// Both operands converted to the common type, or the promoted left
    /// and right operands of a shift.
    Binary(BOp, Box<Expr>, Box<Expr>),
    /// A pointer and an integer: `p + n`, `n + p`, `p - n`.
    PtrAdd(Box<Expr>, Box<Expr>, bool),
    PtrDiff(Box<Expr>, Box<Expr>),
    /// `&&` (true) or `||` (false), operands converted to `bool`.
    Logical(bool, Box<Expr>, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    /// `a op= b`: the operation runs in `ty` (the computation type), then
    /// the result converts back to the type of `a`.
    Compound(BOp, Box<Expr>, Box<Expr>, QType),
    /// `++` and `--`: prefix, increment.
    IncDec(bool, bool, Box<Expr>),
    Cond(Box<Expr>, Box<Expr>, Box<Expr>),
    Comma(Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Vec<Arg>),
    Deref(Box<Expr>),
    /// `a[i]` with the pointer operand and the integer operand: an
    /// lvalue of the element (`expressions/postfix`).
    Index(Box<Expr>, Box<Expr>),
    AddrOf(Box<Expr>),
    /// A lambda expression's closure (step 3 has lambdas with no
    /// captures that run as their function).
    Closure(EntityId),
    /// An expression that is not evaluated.
    Unevaluated(Box<Expr>),
    /// `[[assume(e)]]`: the condition, which must hold (`declarations/attributes`).
    Assume(Box<Expr>),
    /// An array prvalue from a braced list (`declarations/initialization/list`).
    List(Box<Init>),
}

/// An argument as the parameter receives it: a value for a parameter
/// of object type, or a place for a reference parameter.
#[derive(Clone, Debug)]
pub struct Arg {
    pub e: Expr,
    pub by_ref: bool,
}

impl Expr {
    pub fn new(k: EK, ty: QType, cat: Cat, span: Span) -> Expr {
        Expr { k, ty, cat, span, paren: false }
    }
    pub fn int(v: i128, ty: QType, span: Span) -> Expr {
        Expr::new(EK::Int(v), ty, Cat::PR, span)
    }
    pub fn is_glvalue(&self) -> bool {
        self.cat != Cat::PR
    }
    pub fn conv(self, c: Conv, ty: QType, cat: Cat) -> Expr {
        let span = self.span;
        Expr::new(EK::Conv(c, Box::new(self)), ty, cat, span)
    }
    /// Whether `f` holds for the expression or for one of its operands,
    /// at any depth.
    pub fn any(&self, f: &dyn Fn(&Expr) -> bool) -> bool {
        if f(self) {
            return true;
        }
        match &self.k {
            EK::Conv(_, x) | EK::Unary(_, x) | EK::Deref(x) | EK::AddrOf(x) | EK::IncDec(_, _, x) | EK::Unevaluated(x) | EK::Assume(x) | EK::EndLife(x) => x.any(f),
            EK::Binary(_, a, b) | EK::PtrAdd(a, b, _) | EK::PtrDiff(a, b) | EK::Logical(_, a, b) | EK::Assign(a, b) | EK::Compound(_, a, b, _) | EK::Comma(a, b) | EK::Index(a, b) => a.any(f) || b.any(f),
            EK::Cond(c, a, b) => c.any(f) || a.any(f) || b.any(f),
            EK::Call(c, args) => c.any(f) || args.iter().any(|a| a.e.any(f)),
            EK::List(i) => i.any(f),
            _ => false,
        }
    }
    /// The integer value of a literal node, through no-op conversions.
    pub fn as_int_literal(&self) -> Option<i128> {
        match &self.k {
            EK::Int(v) => Some(*v),
            EK::Conv(Conv::Arith | Conv::NoOp, e) => e.as_int_literal(),
            _ => None,
        }
    }
}

/// How a variable gets its first value (`declarations/initialization`).
#[derive(Clone, Debug)]
pub enum Init {
    /// No initializer: default-initialization (INIT rules).
    Default,
    /// `{}` or `()` on a scalar: value-initialization, zero.
    Zero,
    /// A prvalue of the variable's type.
    Expr(Expr),
    /// A glvalue the reference binds to (REF rules).
    Ref(Expr),
    /// An array from a list: one initializer per element, the rest
    /// value-initialized.
    List(Vec<Init>),
}

impl Init {
    /// Whether `f` holds for an expression of the initializer, at any
    /// depth (Expr::any).
    pub fn any(&self, f: &dyn Fn(&Expr) -> bool) -> bool {
        match self {
            Init::Expr(e) | Init::Ref(e) => e.any(f),
            Init::List(xs) => xs.iter().any(|x| x.any(f)),
            Init::Default | Init::Zero => false,
        }
    }
}

pub type StmtId = u32;

#[derive(Clone, Debug)]
pub enum Cond {
    Expr(Expr),
    /// A condition that declares a variable: the variable and its
    /// initializer, then the value converted to `bool` or the switch's
    /// type.
    Decl(EntityId, Init, Expr),
}

#[derive(Clone, Debug)]
pub struct Stmt {
    pub k: SK,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum SK {
    Null,
    Expr(Expr),
    /// A declaration of local variables, each with its initializer.
    Decl(Vec<(EntityId, Init)>),
    Block(Vec<Stmt>),
    If { init: Option<Box<Stmt>>, cond: Cond, then: Box<Stmt>, els: Option<Box<Stmt>> },
    /// `if consteval` and `if constexpr` that analysis already decided:
    /// the branch that runs, if any.
    Chosen(Option<Box<Stmt>>),
    /// `if consteval` and `if !consteval`: the branch that runs in
    /// constant evaluation, and the branch that runs at run time
    /// (IF-18), either one a null statement when the form has none.
    Consteval { constant: Box<Stmt>, runtime: Box<Stmt> },
    While { id: StmtId, cond: Cond, body: Box<Stmt> },
    Do { id: StmtId, body: Box<Stmt>, cond: Expr },
    For { id: StmtId, init: Option<Box<Stmt>>, cond: Option<Cond>, step: Option<Expr>, body: Box<Stmt> },
    /// A range-based `for` over an array: the hidden reference, the
    /// element variable, the bound, and the body.
    RangeArray { id: StmtId, init: Option<Box<Stmt>>, range: EntityId, range_init: Init, var: EntityId, bound: u64, body: Box<Stmt> },
    Switch { id: StmtId, init: Option<Box<Stmt>>, cond: Cond, body: Box<Stmt>, cases: Vec<i128> },
    /// The `k`th `case` label of a switch, or its `default` (None).
    Case(StmtId, Option<usize>, Box<Stmt>),
    Label(Rc<str>, Box<Stmt>),
    Break(StmtId),
    Continue(StmtId),
    Return(Option<Expr>),
    /// A return of a reference: the glvalue.
    ReturnRef(Expr),
    Goto(Rc<str>),
    /// `[[fallthrough]];`, which does nothing when it runs (ATTR-2).
    Fallthrough,
    /// A statement the machine does not run (unsupported at run time).
    Unsupported(String),
}

impl Stmt {
    pub fn new(k: SK, span: Span) -> Stmt {
        Stmt { k, span }
    }
}
