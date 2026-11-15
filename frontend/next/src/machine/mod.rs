//! `machine`: the abstract machine that runs a checked unit and that
//! constant evaluation uses (`DESIGN-rewrite.md` sections 2 and 3). It
//! holds values, objects, places, frames, and the reasons a run stops.
//! It names no rule: the operations whose rules can stop a run sit in the
//! `run.rs` part of their pages, and the machine calls them.
//!
//! A run ends in a result (the status of `main`), a stop at undefined
//! behavior with the rule that names it, or `unsupported`. Constant
//! evaluation runs the same machine with `constant` set; an operation
//! that it does not allow stops with `Stop::Limit` and its rule.

pub mod eval;
pub mod exec;
#[cfg(test)]
mod tests;

use crate::sem::tree::Stmt;
use crate::sem::types::{FloatK, QType, Type};
use crate::sem::{EntityId, Unit};
use crate::target::Target;
use std::collections::HashMap;
use std::rc::Rc;

/// Why a run stopped before it gave a result.
#[derive(Debug, Clone, PartialEq)]
pub enum Stop {
    /// Undefined behavior: the rule that names the operation.
    Undefined(&'static str, String),
    /// An operation that constant evaluation does not allow.
    Limit(&'static str, String),
    /// A construct the machine does not model.
    Unsupported(String),
    /// The program ended with a status (`exit`, or the end of `main`).
    Exit(i128),
    /// An exception leaves the current operation: its exception object
    /// (`exceptions::throw::run`).
    Throw(usize),
    /// The program ends without returning from `main` (UNW-5).
    Terminate(String),
}

pub type R<T> = Result<T, Stop>;

pub fn ub<T>(rule: &'static str, msg: impl Into<String>) -> R<T> {
    Err(Stop::Undefined(rule, msg.into()))
}
pub fn limit<T>(rule: &'static str, msg: impl Into<String>) -> R<T> {
    Err(Stop::Limit(rule, msg.into()))
}
pub fn unsup<T>(msg: impl Into<String>) -> R<T> {
    Err(Stop::Unsupported(msg.into()))
}

/// A place: an object, the indexes from the object to a subobject, and
/// whether the last index is a position in an array (which can be one
/// past its end). A pointer to an object that is not an array element
/// counts that object as an array of one when it moves.
#[derive(Clone, Debug, PartialEq)]
pub struct Ptr {
    pub obj: usize,
    pub path: Vec<i64>,
    pub array: bool,
    /// A pointer whose type does not match the object at its address
    /// (`foundations::storage::run`). The path is then empty.
    pub view: Option<View>,
}

/// The byte view of a pointer: the byte offset in its complete object,
/// the size of its pointee, whether the pointee is a byte type, and the
/// offset where the view began, for ADD-14.
#[derive(Clone, Debug, PartialEq)]
pub struct View {
    pub off: i64,
    pub size: i64,
    pub bytes: bool,
    pub origin: i64,
}

impl Ptr {
    pub fn whole(obj: usize) -> Ptr {
        Ptr { obj, path: Vec::new(), array: false, view: None }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Int(i128),
    Float(f64),
    /// A `long double` value, in the extended format (TYP-10).
    Ext(crate::foundations::types::run::F80),
    Ptr(Ptr),
    /// A null pointer of any pointer type.
    Null,
    /// A pointer that holds an address where no object is.
    Addr(u64),
    Fn(EntityId),
    /// A pointer to member that is not null (`expressions::member_pointers::run`).
    Mem(crate::expressions::member_pointers::run::MemVal),
    /// An array: one value per element.
    Agg(Vec<Val>),
    /// A scalar with no value yet.
    Uninit,
    /// A member of a union that is not its active member
    /// (`classes::unions::run`).
    Inactive,
    Void,
}

impl Val {
    /// A part of the value has no value. A union's parts are those of its
    /// active member, and a union with no active member has none (CE-21).
    pub fn has_empty_part(&self) -> bool {
        match self {
            Val::Uninit => true,
            Val::Agg(xs) => xs.iter().any(|x| x.has_empty_part()),
            _ => false,
        }
    }
    pub fn int(&self) -> Option<i128> {
        match self {
            Val::Int(v) => Some(*v),
            _ => None,
        }
    }
    pub fn truthy(&self) -> bool {
        match self {
            Val::Int(v) => *v != 0,
            Val::Float(f) => *f != 0.0,
            Val::Ext(x) => !x.is_zero(),
            Val::Null => false,
            _ => true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Obj {
    pub val: Val,
    pub ty: QType,
    pub alive: bool,
    /// A complete object defined `const`.
    pub konst: bool,
    /// Created by the current constant evaluation, so it can change.
    pub own: bool,
    pub is_static: bool,
    /// The characters of a string literal.
    pub string: bool,
    /// Constant evaluation can read the object: false for a variable
    /// that is not usable in constant expressions.
    pub usable: bool,
    pub name: Rc<str>,
    /// Created by a `new` expression: whether it is an array, and whether
    /// a `delete` has freed it (`expressions::new_delete::run`).
    pub heap: Option<(bool, bool)>,
    /// The object whose storage this object takes, and the byte offset
    /// in it (STG-5, STG-6).
    pub host: Option<(usize, i64)>,
    /// Storage that a call of a global allocation function by name gave
    /// (NEW-19), whose objects the program creates in it (STG-8).
    pub raw: bool,
}

/// How a statement ended.
#[derive(Debug, Clone, PartialEq)]
pub enum Flow {
    Normal,
    Break(u32),
    Continue(u32),
    Return,
    Goto(Rc<str>),
}

/// The value a call returns: a value, or a place for a reference.
#[derive(Debug, Clone)]
pub enum Ret {
    Val(Val),
    Place(Ptr),
}

#[derive(Default)]
pub struct Frame {
    pub vars: HashMap<EntityId, usize>,
    pub refs: HashMap<EntityId, Ptr>,
    pub func: Option<EntityId>,
    /// The object of a non-static member function.
    pub this: Option<Ptr>,
    pub ret: Option<Ret>,
    /// The objects that end when each open block ends.
    pub blocks: Vec<Vec<usize>>,
}

pub struct Machine<'a> {
    pub u: &'a Unit,
    pub target: Target,
    pub constant: bool,
    /// The run is inside an initializer that the language tries as a
    /// constant, and that is one, so `if consteval` takes its constant
    /// branch (IF-25).
    pub constant_init: bool,
    pub objs: Vec<Obj>,
    pub frames: Vec<Frame>,
    pub globals: HashMap<EntityId, usize>,
    pub global_refs: HashMap<EntityId, Ptr>,
    /// Static variables whose initialization runs (STO rules).
    pub initializing: Vec<EntityId>,
    pub strings: HashMap<Vec<u32>, usize>,
    pub steps: u64,
    pub step_limit: u64,
    pub depth: u32,
    pub depth_limit: u32,
    /// The span of the expression being evaluated.
    pub line: u32,
    /// The temporaries of the full expression being evaluated.
    pub temps: Vec<usize>,
    /// The initializer being evaluated belongs to a variable with static
    /// storage, so its temporaries have static storage too (REF-4).
    pub static_temps: bool,
    /// The reads and writes made while unsequenced operands run, and the
    /// frame depths whose accesses are watched
    /// (`foundations::evaluation_order::run`).
    pub acc: Vec<Access>,
    pub seq_watch: Vec<usize>,
    /// The address of each object that has needed one, and the next free
    /// address (CAST-16, `foundations::storage::run`).
    pub addrs: HashMap<usize, u64>,
    pub next_addr: u64,
    /// The objects whose constructors run, which are not const or
    /// volatile yet (VOL-2, CTOR-11).
    pub constructing: Vec<usize>,
    /// The objects whose constructor runs the initializers of their bases,
    /// before the last base is initialized (CTOR-12).
    pub early: Vec<(usize, Vec<i64>)>,
    /// The places whose constructor or destructor runs, innermost last
    /// (`classes::virtual::run`, VIRT-6).
    pub structors: Vec<(usize, Vec<i64>)>,
    /// The handled exceptions, innermost last (THR-6), and the place that
    /// the parameter of the handler being entered binds to (TRY-2).
    pub handled: Vec<usize>,
    pub caught: Option<Ptr>,
    /// The objects whose members and bases ended before the handler of
    /// a constructor's or destructor's function try block (TRY-11).
    pub gone: Vec<(usize, Vec<i64>)>,
    /// The objects that an explicit destructor call destroyed (DTOR-7).
    pub destroyed: std::collections::HashSet<usize>,
    /// The objects in whose storage a `new` expression created an object
    /// (`foundations::storage::run`).
    pub hosting: std::collections::HashSet<usize>,
    /// The storage that an aligned allocation function gave (NEW-26).
    pub aligned_alloc: std::collections::HashSet<usize>,
    /// The prvalue being evaluated initializes an object, which owns it;
    /// otherwise a prvalue of a class that needs a destructor is a
    /// temporary of its full expression (OBJ-9).
    pub elide: bool,
    /// Every temporary of the initializer that a reference binds lives as
    /// long as the reference (the range of a range-based for, RFOR-4).
    pub keep_temps: bool,
    /// The temporaries that the reference members of the aggregate being
    /// declared bind, which live as long as the variable (REF-4). None
    /// outside such a declaration and inside each expression of it.
    pub extending: Option<Vec<usize>>,
    /// The subobjects whose life a pseudo-destructor call ended in
    /// constant evaluation, as object and path (OBJ-2).
    pub ended: Vec<(usize, Vec<i64>)>,
    /// The objects whose members default member initializers initialize,
    /// innermost last (`EK::InitThis`).
    pub init_this: Vec<Ptr>,
    /// The thread_local variables of the unit whose dynamic
    /// initialization waits for the first use of one of them (OBJ-11).
    pub thread_pending: Vec<(EntityId, usize)>,
    /// The static objects in the order their initialization completed,
    /// which their destruction reverses after main (OBJ-12).
    pub static_order: Vec<usize>,
    /// The objects of a block whose declarations the run has not reached
    /// yet, which get no destructor call if the block ends first.
    pub undeclared: std::collections::HashSet<usize>,
}

/// One read or write of a place while unsequenced operands run, with the
/// frame depth it was made at.
#[derive(Clone, Debug)]
pub struct Access {
    pub obj: usize,
    pub path: Vec<i64>,
    pub write: bool,
    pub depth: usize,
}

impl<'a> Machine<'a> {
    pub fn new(u: &'a Unit, constant: bool) -> Machine<'a> {
        Machine {
            u,
            target: u.target,
            constant,
            constant_init: false,
            objs: Vec::new(),
            frames: vec![Frame::default()],
            globals: HashMap::new(),
            global_refs: HashMap::new(),
            initializing: Vec::new(),
            strings: HashMap::new(),
            steps: 0,
            step_limit: if constant { 20_000_000 } else { 100_000_000 },
            depth: 0,
            depth_limit: if constant { 512 } else { 2000 },
            line: 0,
            temps: Vec::new(),
            static_temps: false,
            acc: Vec::new(),
            seq_watch: Vec::new(),
            addrs: HashMap::new(),
            next_addr: 0x1_0000,
            constructing: Vec::new(),
            structors: Vec::new(),
            handled: Vec::new(),
            caught: None,
            gone: Vec::new(),
            early: Vec::new(),
            destroyed: Default::default(),
            hosting: Default::default(),
            aligned_alloc: Default::default(),
            elide: false,
            keep_temps: false,
            extending: None,
            ended: Vec::new(),
            init_this: Vec::new(),
            thread_pending: Vec::new(),
            static_order: Vec::new(),
            undeclared: Default::default(),
        }
    }

    // -----------------------------------------------------------------
    // Objects and places

    /// Record a read or a write of a place when the current frame's
    /// accesses are watched.
    pub fn record(&mut self, p: &Ptr, write: bool) {
        let depth = self.frames.len();
        if self.seq_watch.contains(&depth) {
            self.acc.push(Access { obj: p.obj, path: p.path.clone(), write, depth });
        }
    }

    pub fn new_obj(&mut self, ty: &QType, val: Val, name: &str) -> usize {
        self.objs.push(Obj { val, ty: ty.clone(), alive: true, konst: ty.is_const(), own: true, is_static: false, string: false, usable: true, name: name.into(), heap: None, host: None, raw: false });
        self.objs.len() - 1
    }
    /// The machine holds one value per element; a larger object is
    /// unsupported.
    pub fn check_size(&self, ty: &QType) -> R<()> {
        fn count(t: &QType) -> u128 {
            match &t.t {
                Type::Array(e, Some(n)) => (*n as u128).saturating_mul(count(e)),
                _ => 1,
            }
        }
        if count(ty) > 1 << 22 {
            return unsup(format!("an object of type {} with more elements than the machine holds", ty.show()));
        }
        Ok(())
    }
    /// The value of an object of type `ty` with no initialization: no
    /// value for a scalar, and one such value per array element.
    pub fn uninit(&self, ty: &QType) -> Val {
        self.uninit_as(ty, true)
    }
    /// `uninit` for a complete object, or for a base subobject, which
    /// holds no virtual bases (`classes::derived::run`).
    pub fn uninit_as(&self, ty: &QType, complete: bool) -> Val {
        match &ty.t {
            Type::Array(e, Some(n)) => Val::Agg((0..*n).map(|_| self.uninit(e)).collect()),
            // A union has no active member.
            Type::Class(c) if self.u.classes[*c].key == crate::sem::ClassKey::Union => Val::Agg(self.u.field_types(*c).iter().map(|_| Val::Inactive).collect()),
            Type::Class(c) => Val::Agg(self.parts(*c, complete).iter().map(|(t, whole)| self.uninit_as(t, *whole)).collect()),
            _ => Val::Uninit,
        }
    }
    /// The types of the parts of an object of class `c`, each with
    /// whether it is a complete object: the data members, then the
    /// virtual bases of a complete object.
    pub fn parts(&self, c: crate::sem::types::ClassId, complete: bool) -> Vec<(QType, bool)> {
        let mut out: Vec<(QType, bool)> = self.u.classes[c].fields.iter().filter_map(|&f| self.u.field(f)).map(|x| (x.ty.clone(), !x.base)).collect();
        if complete {
            out.extend(self.u.classes[c].vbases.iter().map(|&v| (QType::new(Type::Class(v)), false)));
        }
        out
    }
    /// A static object's value after its initializer: the parts that
    /// default-initialization left without a value keep the zero of the
    /// zero-initialization before it (SINIT rules).
    pub fn fill_zero(&self, ty: &QType, v: Val) -> Val {
        self.fill_zero_as(ty, v, true)
    }
    fn fill_zero_as(&self, ty: &QType, v: Val, complete: bool) -> Val {
        match (v, &ty.t) {
            (Val::Uninit, _) => self.zero_as(ty, complete),
            (Val::Agg(xs), Type::Array(e, _)) => Val::Agg(xs.into_iter().map(|x| self.fill_zero(e, x)).collect()),
            // A union with no active member gets its first member zero.
            (Val::Agg(xs), Type::Class(_)) if !xs.is_empty() && xs.iter().all(|x| *x == Val::Inactive) => self.zero(ty),
            (Val::Agg(xs), Type::Class(c)) => {
                let parts = self.parts(*c, complete);
                Val::Agg(xs.into_iter().zip(parts.iter()).map(|(x, (t, whole))| self.fill_zero_as(t, x, *whole)).collect())
            }
            (v, _) => v,
        }
    }
    /// The zero value of a type (value-initialization of a scalar).
    pub fn zero(&self, ty: &QType) -> Val {
        self.zero_as(ty, true)
    }
    /// `zero` for a complete object or a base subobject.
    pub fn zero_as(&self, ty: &QType, complete: bool) -> Val {
        match &ty.t {
            Type::Array(e, Some(n)) => Val::Agg((0..*n).map(|_| self.zero(e)).collect()),
            // A union's first member is zero and active.
            Type::Class(c) if self.u.classes[*c].key == crate::sem::ClassKey::Union => Val::Agg(self.u.field_types(*c).iter().enumerate().map(|(k, t)| if k == 0 { self.zero(t) } else { Val::Inactive }).collect()),
            Type::Class(c) => Val::Agg(self.parts(*c, complete).iter().map(|(t, whole)| self.zero_as(t, *whole)).collect()),
            Type::Float(crate::sem::types::FloatK::LongDouble) => Val::Ext(crate::foundations::types::run::F80::ZERO),
            Type::Float(_) => Val::Float(0.0),
            Type::Ptr(_) | Type::NullPtr | Type::MemPtr(..) => Val::Null,
            _ => Val::Int(0),
        }
    }
    /// The type of the subobject a place names.
    pub fn type_at(&self, p: &Ptr) -> Option<QType> {
        let mut t = self.objs.get(p.obj)?.ty.clone();
        let n = p.path.len();
        for (k, &i) in p.path.iter().enumerate() {
            if k + 1 == n && p.array && !t.is_array() {
                // An object counted as an array of one.
                return Some(t);
            }
            t = match &t.t {
                Type::Class(c) => self.u.object_types(*c).get(i as usize)?.clone(),
                _ => t.elem()?.clone(),
            };
        }
        Some(t)
    }
    /// The number of elements of the array that a position of `p` is in:
    /// 1 for an object counted as an array of one.
    pub fn array_len(&self, p: &Ptr) -> Option<i64> {
        let mut q = p.clone();
        q.path.pop();
        q.array = false;
        let t = self.type_at(&q)?;
        Some(match &t.t {
            Type::Array(_, Some(n)) => *n as i64,
            Type::Array(_, None) => i64::MAX,
            _ => 1,
        })
    }
    /// The value slot a place names, if the place is inside its arrays.
    pub fn slot(&mut self, p: &Ptr) -> Option<&mut Val> {
        if p.view.is_some() {
            return None;
        }
        let single = p.array && {
            let mut q = p.clone();
            q.path.pop();
            q.array = false;
            !self.type_at(&q).is_some_and(|t| t.is_array())
        };
        let o = self.objs.get_mut(p.obj)?;
        let mut v = &mut o.val;
        let n = p.path.len();
        for (k, &i) in p.path.iter().enumerate() {
            if k + 1 == n && single {
                return if i == 0 { Some(v) } else { None };
            }
            match v {
                Val::Agg(xs) => {
                    if i < 0 || i as usize >= xs.len() {
                        return None;
                    }
                    v = &mut xs[i as usize];
                }
                _ => return None,
            }
        }
        Some(v)
    }
    pub fn step(&mut self) -> R<()> {
        self.steps += 1;
        if self.steps > self.step_limit {
            return if self.constant { crate::foundations::constant_evaluation::run::too_many_steps() } else { unsup("the run takes too many steps") };
        }
        Ok(())
    }
    /// Round a value to its floating type.
    pub fn round_float(&self, f: f64, k: FloatK) -> f64 {
        match k {
            FloatK::Float => f as f32 as f64,
            _ => f,
        }
    }

    // -----------------------------------------------------------------
    // Variables

    /// The object of a variable, made when the run first needs it for a
    /// static variable.
    pub fn var_place(&mut self, id: EntityId) -> R<Ptr> {
        // A declaration that another unit defines (linking).
        let id = self.u.links.get(&id).copied().unwrap_or(id);
        if let Some(f) = self.frames.last() {
            if let Some(&o) = f.vars.get(&id) {
                return Ok(Ptr::whole(o));
            }
            if let Some(p) = f.refs.get(&id) {
                return Ok(p.clone());
            }
        }
        // OBJ-11: the first use of a waiting thread_local variable.
        if self.thread_pending.iter().any(|&(i, _)| i == id) {
            crate::foundations::translation::run::start_thread_locals(self)?;
        }
        if let Some(&o) = self.globals.get(&id) {
            return Ok(Ptr::whole(o));
        }
        if let Some(p) = self.global_refs.get(&id) {
            return Ok(p.clone());
        }
        exec::static_var(self, id)
    }

    pub fn body(&self, id: EntityId) -> Option<Rc<Stmt>> {
        self.u.func(id).and_then(|f| f.body.clone())
    }
}
