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
    /// The object of a member access with a qualified name, as in
    /// `d.B::f()`: a call through it does not dispatch virtually.
    pub qualified: bool,
}

/// The allocation and deallocation functions of a `new` expression that
/// does not use only the library's global functions (NEW-10, NEW-13,
/// NEW-21 to NEW-24, NEW-31).
#[derive(Clone, Debug)]
pub struct NewAlloc {
    /// The allocation function; None for the library's global one.
    pub func: Option<DeclId>,
    /// The placement arguments, converted to the parameter types.
    pub args: Vec<Expr>,
    /// The allocation function is non-throwing (NEW-31, NEW-32).
    pub nothrow: bool,
    /// The deallocation function that frees the storage when the
    /// initialization throws (NEW-21).
    pub dealloc: Dealloc,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Dealloc {
    /// The library's global function.
    Library,
    /// A declared function, and whether it takes the size.
    Func(DeclId, bool),
    /// No function matches, and the storage is not freed (NEW-24).
    None,
}

#[derive(Clone, Debug)]
pub enum EK {
    Int(i128),
    Float(f64),
    /// A `long double` literal: its digits without the suffix (TYP-10).
    LFloat(Rc<str>),
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
    /// A `new` expression with the global allocation function, or with the
    /// non-allocating placement form and its argument as `void*` (STG-5):
    /// the type of the object or of each element, the number of elements
    /// of an array `new`, and the converted initializer (none for
    /// default-initialization).
    New { ty: QTy, count: Option<Box<Expr>>, init: Option<Box<Expr>>, place: Option<Box<Expr>>, alloc: Option<Rc<NewAlloc>> },
    /// A `delete` or `delete[]` expression.
    /// The third field says that the expression starts with `::`.
    Delete(bool, Box<Expr>, bool),
    /// A `dynamic_cast` whose checks passed; the type and category of the
    /// node are the target's.
    DynamicCast(Box<Expr>),
    /// A lambda expression: the closure class and the initializer of each
    /// of its members, one per capture (LAM rules). The flag says that a
    /// capture is by reference.
    Closure(usize, Vec<Expr>, bool),
    /// The name of a function template with explicit template
    /// arguments, before the call: the object of a member call, the
    /// functions, and the arguments as written (DED-2).
    TemplId(Option<Box<Expr>>, Vec<DeclId>, Rc<Vec<crate::templ::RawArg>>),
    /// An explicit destructor call `x.~T()` on the glvalue `x` (DTOR-8,
    /// STG-6).
    Destroy(Box<Expr>),
    /// An element of an aggregate's list for a member that a designated
    /// list does not name: the member is initialized as if the list
    /// ended before it (LIST-5).
    MemberDefault,
    /// A cast that reinterprets its operand (CAST-6, CAST-12, CAST-16,
    /// STG-16): `reinterpret_cast`, or a C-style cast that acts as one.
    /// The string says which kind, for the CE-4 stop in constant
    /// evaluation. The node's type and category are the target's.
    Reinterpret(Box<Expr>, String),
    /// A pointer to member constant `&C::m` (MPTR-1, MPTR-2).
    MemberPtr(DeclId),
    /// `obj.*mp`; `->*` has the indirection in `obj` (MPTR rules). The
    /// type is the member's, a function type for a member function.
    PtrMem(Box<Expr>, Box<Expr>),
    /// Something the front end parsed but cannot evaluate.
    Unknown(String),
}

impl Expr {
    pub fn new(k: EK, ty: QTy, cat: Cat, line: u32) -> Expr {
        Expr { k, ty, cat, line, paren: false, qualified: false }
    }
    pub fn unknown(line: u32, why: &str) -> Expr {
        Expr { k: EK::Unknown(why.to_string()), ty: QTy::unknown(), cat: Cat::PR, line, paren: false, qualified: false }
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

/// A handler of a try block: its parameter, if named, and its type
/// (None for `...`).
#[derive(Clone, Debug)]
pub struct Handler {
    pub var: Option<DeclId>,
    pub ty: Option<QTy>,
    pub body: Box<Stmt>,
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
    Try(Box<Stmt>, Vec<Handler>),
    Unsupported(String),
}

/// Call `f` on `e` and on each of its subexpressions.
pub fn walk_expr(e: &Expr, f: &mut dyn FnMut(&Expr)) {
    f(e);
    match &e.k {
        EK::BoundMember(a, _) | EK::Unary(_, a) | EK::Conv(_, a) | EK::Member(a, _) => walk_expr(a, f),
        EK::Binary(_, a, b) | EK::Assign(_, a, b) | EK::Comma(a, b) | EK::Index(a, b) | EK::PtrMem(a, b) => {
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
        EK::InitList(v) | EK::Construct(_, v) | EK::Closure(_, v, _) => {
            for x in v {
                walk_expr(x, f);
            }
        }
        EK::Throw(Some(a)) | EK::Delete(_, a, _) | EK::DynamicCast(a) | EK::Destroy(a) | EK::Reinterpret(a, _) => walk_expr(a, f),
        EK::New { count, init, place, alloc, .. } => {
            if let Some(p) = place {
                walk_expr(p, f);
            }
            if let Some(a) = alloc {
                for x in &a.args {
                    walk_expr(x, f);
                }
            }
            if let Some(c) = count {
                walk_expr(c, f);
            }
            if let Some(i) = init {
                walk_expr(i, f);
            }
        }
        _ => {}
    }
}

/// A compact form of an expression, for `tidal-fe dump`.
pub fn sexp(e: &Expr) -> String {
    let cat = match e.cat {
        Cat::L => "l",
        Cat::X => "x",
        Cat::PR => "",
    };
    let ty = crate::types::show(&e.ty);
    let inner = match &e.k {
        EK::MemberDefault => "member-default".into(),
        EK::Int(v) => format!("{}", v),
        EK::Float(v) => format!("{}", v),
        EK::LFloat(v) => format!("{}L", v),
        EK::NullPtr => "nullptr".into(),
        EK::Str(_) => "\"...\"".into(),
        EK::Var(d) => format!("var#{}", d),
        EK::Func(d) => format!("func#{}", d),
        EK::Overloads(v) => format!("overloads{:?}", v),
        EK::BoundMember(a, v) => format!("bound {} {:?}", sexp(a), v),
        EK::This => "this".into(),
        EK::Unary(op, a) => format!("{:?} {}", op, sexp(a)),
        EK::Binary(op, a, b) => format!("{} {} {}", op.spelling(), sexp(a), sexp(b)),
        EK::Assign(op, a, b) => format!("= {:?} {} {}", op.as_ref().map(|x| x.0), sexp(a), sexp(b)),
        EK::Cond(a, b, c) => format!("? {} {} {}", sexp(a), sexp(b), sexp(c)),
        EK::Comma(a, b) => format!(", {} {}", sexp(a), sexp(b)),
        EK::Call(a, args, f) => format!("call#{:?} {} [{}]", f, sexp(a), args.iter().map(sexp).collect::<Vec<_>>().join(" ")),
        EK::Conv(k, a) => format!("{:?} {}", k, sexp(a)),
        EK::Member(a, f) => format!("member#{} {}", f, sexp(a)),
        EK::Index(a, b) => format!("[] {} {}", sexp(a), sexp(b)),
        EK::InitList(v) => format!("{{{}}}", v.iter().map(sexp).collect::<Vec<_>>().join(" ")),
        EK::ZeroInit => "zero".into(),
        EK::Throw(a) => format!("throw {}", a.as_ref().map(|x| sexp(x)).unwrap_or_default()),
        EK::Construct(c, v) => format!("construct#{} [{}]", c, v.iter().map(sexp).collect::<Vec<_>>().join(" ")),
        EK::New { ty, count, init, .. } => format!("new {} [{}] {}", crate::types::show(ty), count.as_ref().map(|c| sexp(c)).unwrap_or_default(), init.as_ref().map(|c| sexp(c)).unwrap_or_default()),
        EK::Delete(a, x, g) => format!("{}delete{} {}", if *g { "::" } else { "" }, if *a { "[]" } else { "" }, sexp(x)),
        EK::DynamicCast(x) => format!("dynamic_cast {}", sexp(x)),
        EK::Closure(c, v, _) => format!("closure#{} [{}]", c, v.iter().map(sexp).collect::<Vec<_>>().join(" ")),
        EK::TemplId(_, v, _) => format!("templ-id{:?}", v),
        EK::Destroy(x) => format!("destroy {}", sexp(x)),
        EK::Reinterpret(x, w) => format!("reinterpret({}) {}", w, sexp(x)),
        EK::MemberPtr(d) => format!("memptr#{}", d),
        EK::PtrMem(a, b) => format!(".* {} {}", sexp(a), sexp(b)),
        EK::Unknown(w) => format!("UNKNOWN({})", w),
    };
    format!("({}{}{}:{})", if e.qualified { "Q " } else { "" }, inner, if cat.is_empty() { String::new() } else { format!(" {}", cat) }, ty)
}

/// A compact form of a statement, for `tidal-fe dump`.
pub fn sstmt(s: &Stmt, ind: usize) -> String {
    let pad = " ".repeat(ind);
    match s {
        Stmt::Block(v) => format!("{}{{\n{}{}}}\n", pad, v.iter().map(|x| sstmt(x, ind + 2)).collect::<String>(), pad),
        Stmt::Expr(e) => format!("{}{}\n", pad, sexp(e)),
        Stmt::Return(e) => format!("{}return {}\n", pad, e.as_ref().map(sexp).unwrap_or_default()),
        Stmt::Decl(ids) => format!("{}decl {:?}\n", pad, ids),
        Stmt::If { cond, then, els, .. } => format!(
            "{}if {}\n{}{}",
            pad,
            match cond {
                Some(CondK::Expr(e)) => sexp(e),
                Some(CondK::Decl(d, e)) => format!("decl#{} {}", d, sexp(e)),
                None => String::new(),
            },
            sstmt(then, ind + 2),
            els.as_ref().map(|e| format!("{}else\n{}", pad, sstmt(e, ind + 2))).unwrap_or_default()
        ),
        Stmt::While { body, .. } | Stmt::Do { body, .. } | Stmt::For { body, .. } | Stmt::Switch { body, .. } => format!("{}loop/switch\n{}", pad, sstmt(body, ind + 2)),
        other => format!("{}{:?}\n", pad, other).chars().take(300).collect(),
    }
}
