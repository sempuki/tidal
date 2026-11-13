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
}

impl Ptr {
    pub fn whole(obj: usize) -> Ptr {
        Ptr { obj, path: Vec::new(), array: false }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Int(i128),
    Float(f64),
    Ptr(Ptr),
    /// A null pointer of any pointer type.
    Null,
    /// A pointer that holds an address where no object is.
    Addr(u64),
    Fn(EntityId),
    /// An array: one value per element.
    Agg(Vec<Val>),
    /// A scalar with no value yet.
    Uninit,
    Void,
}

impl Val {
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
        self.objs.push(Obj { val, ty: ty.clone(), alive: true, konst: ty.is_const(), own: true, is_static: false, string: false, usable: true, name: name.into() });
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
        match &ty.t {
            Type::Array(e, Some(n)) => Val::Agg((0..*n).map(|_| self.uninit(e)).collect()),
            _ => Val::Uninit,
        }
    }
    /// The zero value of a type (value-initialization of a scalar).
    pub fn zero(&self, ty: &QType) -> Val {
        match &ty.t {
            Type::Array(e, Some(n)) => Val::Agg((0..*n).map(|_| self.zero(e)).collect()),
            Type::Float(_) => Val::Float(0.0),
            Type::Ptr(_) | Type::NullPtr => Val::Null,
            _ => Val::Int(0),
        }
    }
    /// The type of the subobject a place names.
    pub fn type_at(&self, p: &Ptr) -> Option<QType> {
        let mut t = self.objs.get(p.obj)?.ty.clone();
        let n = p.path.len();
        for (k, _) in p.path.iter().enumerate() {
            if k + 1 == n && p.array && !t.is_array() {
                // An object counted as an array of one.
                return Some(t);
            }
            t = t.elem()?.clone();
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
        if let Some(f) = self.frames.last() {
            if let Some(&o) = f.vars.get(&id) {
                return Ok(Ptr::whole(o));
            }
            if let Some(p) = f.refs.get(&id) {
                return Ok(p.clone());
            }
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
