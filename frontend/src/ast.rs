// Expression and statement trees, kept for constant evaluation.

use crate::types::QTy;
use std::rc::Rc;

pub type DeclId = usize;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cat {
    L,
    X,
    PR,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UOp {
    Plus,
    Neg,
    Not,
    BitNot,
    Deref,
    AddrOf,
    PreInc,
    PreDec,
    PostInc,
    PostDec,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    LAnd,
    LOr,
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
            "&&" => BOp::LAnd,
            "||" => BOp::LOr,
            _ => return None,
        })
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
            BOp::LAnd => "&&",
            BOp::LOr => "||",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Conv {
    /// lvalue-to-rvalue: read the object.
    LToR,
    ArrayDecay,
    FuncDecay,
    /// Any arithmetic or enumeration conversion; the evaluator looks at
    /// the source and target types.
    Arith,
    ToBool,
    NullPtr,
    PtrConv,
    NoOp,
    ToVoid,
    /// Binding a reference: the operand is a glvalue, and the result is
    /// the same glvalue.
    Bind,
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub k: EK,
    pub ty: QTy,
    pub cat: Cat,
    pub line: u32,
    /// Written in parentheses (matters for decltype).
    pub paren: bool,
}

#[derive(Clone, Debug)]
pub enum EK {
    Int(i128),
    Float(f64),
    NullPtr,
    Str(Rc<Vec<u32>>),
    Var(DeclId),
    Func(DeclId),
    /// An unresolved overload set (functions found by lookup).
    Overloads(Vec<DeclId>),
    /// A member function selected through an object, before the call.
    BoundMember(Box<Expr>, Vec<DeclId>),
    This,
    Unary(UOp, Box<Expr>),
    Binary(BOp, Box<Expr>, Box<Expr>),
    /// Compound assignment keeps the operation and the type it is done in.
    Assign(Option<(BOp, QTy)>, Box<Expr>, Box<Expr>),
    Cond(Box<Expr>, Box<Expr>, Box<Expr>),
    Comma(Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>, Option<DeclId>),
    Conv(Conv, Box<Expr>),
    Member(Box<Expr>, DeclId),
    Index(Box<Expr>, Box<Expr>),
    InitList(Vec<Expr>),
    /// Value-initialization of a scalar, as in `int()`.
    ZeroInit,
    Throw(Option<Box<Expr>>),
    /// A constructor call: the constructor and its converted arguments.
    Construct(DeclId, Vec<Expr>),
    /// Something the front end parsed but cannot evaluate.
    Unknown(String),
}

impl Expr {
    pub fn new(k: EK, ty: QTy, cat: Cat, line: u32) -> Expr {
        Expr { k, ty, cat, line, paren: false }
    }
    pub fn unknown(line: u32, why: &str) -> Expr {
        Expr { k: EK::Unknown(why.to_string()), ty: QTy::unknown(), cat: Cat::PR, line, paren: false }
    }
    pub fn is_glvalue(&self) -> bool {
        self.cat != Cat::PR
    }
}

#[derive(Clone, Debug)]
pub enum CondK {
    Expr(Expr),
    /// A declared variable and the expression that tests it.
    Decl(DeclId, Expr),
}

#[derive(Clone, Debug)]
pub enum IfKind {
    Normal,
    Constexpr,
    Consteval(bool),
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Null,
    Expr(Expr),
    Decl(Vec<DeclId>),
    Block(Vec<Stmt>),
    /// `[[assume(e)]];` (ATTR-10, ATTR-15).
    Assume(Expr),
    /// `[[fallthrough]];` and its line (ATTR-2).
    Fallthrough(u32),
    If { init: Option<Box<Stmt>>, cond: Option<CondK>, then: Box<Stmt>, els: Option<Box<Stmt>>, kind: IfKind },
    While { cond: CondK, body: Box<Stmt> },
    Do { body: Box<Stmt>, cond: Expr },
    For { init: Option<Box<Stmt>>, cond: Option<CondK>, inc: Option<Expr>, body: Box<Stmt> },
    Switch { init: Option<Box<Stmt>>, cond: CondK, body: Box<Stmt> },
    Case(i128, Box<Stmt>),
    Default(Box<Stmt>),
    Break,
    Continue,
    Return(Option<Expr>),
    Goto(String),
    Label(String, Box<Stmt>),
    Unsupported(String),
}

/// Call `f` on `e` and on each of its subexpressions.
pub fn walk_expr(e: &Expr, f: &mut dyn FnMut(&Expr)) {
    f(e);
    match &e.k {
        EK::BoundMember(a, _) | EK::Unary(_, a) | EK::Conv(_, a) | EK::Member(a, _) => walk_expr(a, f),
        EK::Binary(_, a, b) | EK::Assign(_, a, b) | EK::Comma(a, b) | EK::Index(a, b) => {
            walk_expr(a, f);
            walk_expr(b, f);
        }
        EK::Cond(a, b, c) => {
            walk_expr(a, f);
            walk_expr(b, f);
            walk_expr(c, f);
        }
        EK::Call(a, args, _) => {
            walk_expr(a, f);
            for x in args {
                walk_expr(x, f);
            }
        }
        EK::InitList(v) | EK::Construct(_, v) => {
            for x in v {
                walk_expr(x, f);
            }
        }
        EK::Throw(Some(a)) => walk_expr(a, f),
        _ => {}
    }
}
