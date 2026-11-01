// The interpreter (decision 0048). It runs a checked program on the
// abstract machine of the specification (spec/foundations/translation.md,
// objects.md, values.md, evaluation-order.md).
//
// A run ends in one of three ways:
// - a result: the exit status from `main` (TR-2),
// - a stop at undefined behavior, which names the rule and gives no
//   result (TR-8, decision 0047),
// - unsupported: the interpreter met a construct it does not model.
//
// Where the rules leave a choice open, the interpreter takes one fixed
// choice and records the choice point with its rule.
//
// Constant evaluation is a client (src/consteval.rs). It runs the same
// machine with `constant` set, which adds the limits of
// spec/foundations/constant-evaluation.md. An operation that constant
// evaluation does not allow stops the run with `Stop::Limit` and the rule
// that forbids it.

mod expr;
pub mod run;
mod stmt;

use crate::ast::*;
use crate::parser::*;
use crate::types::*;
use std::collections::HashMap;

/// Why a run stopped before it gave a result.
#[derive(Debug, Clone)]
pub enum Stop {
    /// Undefined behavior: the rule that names the operation.
    Undefined(&'static str, String),
    /// An operation that constant evaluation does not allow.
    Limit(&'static str, String),
    /// A construct the interpreter does not model.
    Unsupported(String),
    /// An exception on its way to a handler (THR rules, UNW-1).
    Throw(Exc),
    /// The program ends without returning from `main` (UNW-5, THR-7): the
    /// rules that say so.
    Terminate(&'static str, String),
}

/// An exception object and its type (THR-1, THR-4).
#[derive(Debug, Clone)]
pub struct Exc {
    pub obj: usize,
    pub ty: QTy,
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

/// A choice point that the run passed: the rule that leaves the choice
/// open, the line, and the choice the interpreter took.
#[derive(Clone, Debug, PartialEq)]
pub struct Choice {
    pub rule: &'static str,
    pub line: u32,
    pub what: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ptr {
    pub obj: usize,
    pub path: Vec<usize>,
    /// For an array pointer, the element index (can be one past the end).
    pub off: i64,
    pub array: bool,
}

impl Ptr {
    pub fn whole(obj: usize) -> Ptr {
        Ptr { obj, path: vec![], off: 0, array: false }
    }
    /// The place of a subobject.
    pub fn sub(&self, i: usize) -> Ptr {
        let mut path = full_path(self);
        path.push(i);
        Ptr { obj: self.obj, path, off: 0, array: false }
    }
}

pub fn full_path(p: &Ptr) -> Vec<usize> {
    let mut v = p.path.clone();
    if p.array {
        v.push(p.off as usize);
    }
    v
}

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Int(i128),
    Float(f64),
    Ptr(Ptr),
    /// A function, with the translation unit whose declaration it is.
    FnPtr(usize, DeclId),
    Null,
    Agg(Vec<Val>),
    Uninit,
    /// A union member that is not the active member (UNI-6, CE-6).
    Inactive,
    Void,
    /// The value of a variable that constant evaluation must not read
    /// (CE-3).
    NotUsable(String),
}

/// How an object was created by `new` (NEW-5).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Heap {
    No,
    Single,
    Array,
}

#[derive(Clone, Debug)]
pub struct Obj {
    pub val: Val,
    pub alive: bool,
    /// Created by this evaluation, so constant evaluation can change it
    /// (CE-5).
    pub own: bool,
    pub volatile: bool,
    pub is_static: bool,
    pub string: bool,
    /// A complete object declared `const` (CAST-5, CV-5).
    pub konst: bool,
    pub heap: Heap,
    /// The type of the complete object, for the dynamic type of its
    /// class subobjects.
    pub ty: Option<QTy>,
    /// The translation unit whose types `ty` uses.
    pub tu: usize,
}

/// An object to end when its block, full expression, or the program
/// ends.
#[derive(Clone, Debug)]
pub struct Cleanup {
    pub place: Ptr,
    pub ty: QTy,
    /// The item of the block that declared the object, for `goto`.
    pub item: usize,
}

pub enum Flow {
    Normal,
    Break,
    Continue,
    Return,
    Goto(String),
}

/// The result of a call.
pub enum Ret {
    Val(Val),
    /// A reference return: the place it refers to.
    Place(Ptr),
    /// A class return: the object is in the slot the caller gave.
    InSlot,
}

pub struct Frame {
    pub vars: HashMap<DeclId, usize>,
    pub refs: HashMap<DeclId, Ptr>,
    pub this: Option<Ptr>,
    pub func: Option<DeclId>,
    pub ret_ref: bool,
    pub ret_class: bool,
    pub slot: Option<Ptr>,
    pub ret: Option<Ret>,
    /// The number of blocks open when the call started.
    pub block_base: usize,
}

impl Frame {
    fn new() -> Frame {
        Frame { vars: HashMap::new(), refs: HashMap::new(), this: None, func: None, ret_ref: false, ret_class: false, slot: None, ret: None, block_base: 0 }
    }
}

pub struct Machine<'a> {
    /// The translation unit of the code that runs.
    pub p: &'a P,
    /// Every translation unit of the program, and the index of `p`.
    pub ps: Vec<&'a P>,
    pub tu: usize,
    /// Linked entities: a declaration in one unit and the definition that
    /// it names in another (TR-4, LNK rules).
    pub links: HashMap<(usize, DeclId), (usize, DeclId)>,
    /// Constant evaluation: the limits of constant-evaluation.md apply.
    pub constant: bool,
    pub objs: Vec<Obj>,
    pub frames: Vec<Frame>,
    /// The objects and references of variables with static storage
    /// duration, by translation unit and declaration.
    pub globals: HashMap<(usize, DeclId), usize>,
    pub global_refs: HashMap<(usize, DeclId), Ptr>,
    /// Block-scope statics whose initialization is running (STO-9).
    pub statics_running: std::collections::HashSet<(usize, DeclId)>,
    pub steps: u64,
    pub step_limit: u64,
    pub depth: u32,
    pub depth_limit: u32,
    pub in_consteval_fn: bool,
    /// Objects that end with their block, one list per open block.
    pub blocks: Vec<Vec<Cleanup>>,
    /// The item index of the statement each open block runs.
    pub block_item: Vec<usize>,
    /// Temporaries that end with their full expression.
    pub temps: Vec<Vec<Cleanup>>,
    /// Static objects, in the order their construction completed.
    pub static_dtors: Vec<Cleanup>,
    pub choices: Vec<Choice>,
    /// Constructors and destructors that run, with their object and class,
    /// for the dynamic type (VIRT rules).
    pub cdtor: Vec<(Ptr, usize)>,
    /// String literal objects by their contents (PRIM-9).
    pub strings: HashMap<Vec<u32>, usize>,
    /// The line of the expression being evaluated.
    pub line: u32,
    /// Places that synthetic argument expressions name, for the copy and
    /// move constructors that an implicit copy calls.
    pub synth: Vec<Ptr>,
    /// Temporaries that reference members of an aggregate bind, which live
    /// as long as the aggregate (REF-4). The declaration claims them.
    pub extended: Vec<Cleanup>,
    /// Depth of aggregate initializations from a braced list.
    pub in_aggregate: u32,
    /// The exceptions that active handlers handle, innermost last (TRY-8).
    pub caught: Vec<Exc>,
    /// Depth of unwinding (UNW-5).
    pub unwinding: u32,
}

impl<'a> Machine<'a> {
    pub fn new(p: &'a P, constant: bool) -> Machine<'a> {
        Machine {
            p,
            ps: vec![p],
            tu: 0,
            links: HashMap::new(),
            constant,
            objs: Vec::new(),
            frames: vec![Frame::new()],
            globals: HashMap::new(),
            global_refs: HashMap::new(),
            statics_running: Default::default(),
            steps: 0,
            step_limit: if constant { 2_000_000 } else { 20_000_000 },
            depth: 0,
            depth_limit: if constant { 512 } else { 2000 },
            in_consteval_fn: false,
            blocks: Vec::new(),
            block_item: Vec::new(),
            temps: Vec::new(),
            static_dtors: Vec::new(),
            choices: Vec::new(),
            cdtor: Vec::new(),
            strings: HashMap::new(),
            line: 0,
            synth: Vec::new(),
            extended: Vec::new(),
            in_aggregate: 0,
            caught: Vec::new(),
            unwinding: 0,
        }
    }

    pub fn step(&mut self) -> R<()> {
        self.steps += 1;
        if self.steps > self.step_limit {
            if self.constant {
                // CE-10: an implementation limit.
                return limit("CE-10", "constant evaluation step limit exceeded");
            }
            return unsup("step limit of the interpreter exceeded");
        }
        Ok(())
    }

    pub fn choice(&mut self, rule: &'static str, what: impl Into<String>) {
        let c = Choice { rule, line: self.line, what: what.into() };
        if !self.choices.contains(&c) {
            self.choices.push(c);
        }
    }

    pub fn new_obj(&mut self, val: Val, own: bool) -> usize {
        let tu = self.tu;
        self.objs.push(Obj { val, alive: true, own, volatile: false, is_static: false, string: false, konst: false, heap: Heap::No, ty: None, tu });
        self.objs.len() - 1
    }

    /// A new object of type `t` whose scalars have no value yet.
    pub fn alloc(&mut self, t: &QTy) -> R<Ptr> {
        let v = self.shape(t)?;
        let o = self.new_obj(v, true);
        self.objs[o].ty = Some(t.clone());
        self.objs[o].volatile = t.v;
        Ok(Ptr::whole(o))
    }

    /// The type of an object, if the unit that runs uses the same class
    /// numbering as the unit that made it.
    pub fn obj_ty(&self, o: usize) -> Option<QTy> {
        let ob = &self.objs[o];
        if ob.tu != self.tu {
            return None;
        }
        ob.ty.clone()
    }

    /// Run `f` with the code of translation unit `tu`.
    pub fn in_tu<T>(&mut self, tu: usize, f: impl FnOnce(&mut Self) -> R<T>) -> R<T> {
        if tu == self.tu {
            return f(self);
        }
        let (sp, st) = (self.p, self.tu);
        self.p = self.ps[tu];
        self.tu = tu;
        let r = f(self);
        self.p = sp;
        self.tu = st;
        r
    }

    pub fn frame(&self) -> &Frame {
        self.frames.last().unwrap()
    }
    pub fn frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    // ---- classes ----

    /// The number of base subobjects that come before the members in the
    /// value of a class object.
    pub fn nbases(&self, c: usize) -> usize {
        self.p.classes[c].bases.len()
    }

    pub fn class_ok(&self, c: usize) -> R<()> {
        let ci = &self.p.classes[c];
        if ci.opaque || ci.template || self.p.class_uncertain(c) {
            return unsup("class layout the front end does not model");
        }
        if ci.bases.iter().any(|b| b.virt || b.class.is_none()) {
            return unsup("virtual base class");
        }
        Ok(())
    }

    pub fn fields(&self, c: usize) -> Vec<(DeclId, FieldInfo)> {
        self.p.classes[c].fields.iter().filter_map(|&f| match &self.p.decls[f].k {
            DK::Field(fi) => Some((f, fi.clone())),
            _ => None,
        }).collect()
    }

    /// The subobject indices from class `from` down to its base `to`.
    pub fn base_path(&self, from: usize, to: usize) -> Option<Vec<usize>> {
        if from == to {
            return Some(vec![]);
        }
        for (i, b) in self.p.classes[from].bases.iter().enumerate() {
            if let Some(bc) = b.class {
                if let Some(mut rest) = self.base_path(bc, to) {
                    rest.insert(0, i);
                    return Some(rest);
                }
            }
        }
        None
    }

    /// The value of a new object of type `t`, with no scalar values yet.
    pub fn shape(&self, t: &QTy) -> R<Val> {
        Ok(match &t.t {
            Ty::Array(el, Some(n)) => {
                if *n > 1_000_000 {
                    return unsup("array too large for the interpreter");
                }
                let e = self.shape(el)?;
                Val::Agg(vec![e; *n as usize])
            }
            Ty::Array(_, None) => return unsup("array of unknown bound"),
            Ty::Class(c) => {
                let c = *c;
                self.class_ok(c)?;
                let mut v = Vec::new();
                for b in &self.p.classes[c].bases {
                    v.push(self.shape(&QTy::new(Ty::Class(b.class.unwrap())))?);
                }
                let union = self.p.classes[c].key == 2;
                for (_, fi) in self.fields(c) {
                    if union {
                        v.push(Val::Inactive);
                    } else if fi.ty.t.is_ref() {
                        v.push(Val::Uninit);
                    } else {
                        v.push(self.shape(&fi.ty)?);
                    }
                }
                Val::Agg(v)
            }
            Ty::Unknown => return unsup("object of unknown type"),
            Ty::Void | Ty::Func(_) => return unsup("object of void or function type"),
            _ => Val::Uninit,
        })
    }

    /// Zero-initialization (INIT rules): every scalar is zero, and a union
    /// has its first member active.
    pub fn zero(&self, t: &QTy) -> Val {
        match &t.t {
            Ty::Ptr(_) | Ty::NullPtr | Ty::MemPtr(..) => Val::Null,
            x if x.is_floating() => Val::Float(0.0),
            Ty::Array(el, Some(n)) => Val::Agg((0..*n).map(|_| self.zero(el)).collect()),
            Ty::Class(c) => {
                let mut v = Vec::new();
                for b in &self.p.classes[*c].bases {
                    if let Some(bc) = b.class {
                        v.push(self.zero(&QTy::new(Ty::Class(bc))));
                    }
                }
                let union = self.p.classes[*c].key == 2;
                let mut first = true;
                for (_, fi) in self.fields(*c) {
                    if union && !first {
                        v.push(Val::Inactive);
                    } else if fi.ty.t.is_ref() {
                        v.push(Val::Uninit);
                    } else {
                        v.push(self.zero(&fi.ty));
                    }
                    first = false;
                }
                Val::Agg(v)
            }
            _ => Val::Int(0),
        }
    }

    /// Whether ending an object of this type runs code.
    pub fn needs_dtor(&self, t: &Ty) -> bool {
        match t {
            Ty::Array(el, _) => self.needs_dtor(&el.t),
            Ty::Class(c) => {
                let ci = &self.p.classes[*c];
                if let Some(d) = ci.dtor {
                    if self.p.func(d).is_some_and(|f| !f.defaulted && !matches!(f.body.as_deref(), Some(Stmt::Block(v)) if v.is_empty())) {
                        return true;
                    }
                }
                if ci.key == 2 {
                    return false;
                }
                ci.bases.iter().any(|b| b.class.is_some_and(|bc| self.needs_dtor(&Ty::Class(bc)))) || self.fields(*c).iter().any(|(_, fi)| !fi.ty.t.is_ref() && self.needs_dtor(&fi.ty.t))
            }
            _ => false,
        }
    }

    // ---- places ----

    pub fn get_at(&self, p: &Ptr) -> R<&Val> {
        let o = &self.objs[p.obj];
        if !o.alive {
            return ub(self.ended_rule(false), "access to an object whose lifetime has ended");
        }
        let mut v = &o.val;
        for &i in &full_path(p) {
            match v {
                Val::Agg(items) => {
                    if i >= items.len() {
                        return ub("PFX-4", "access outside the bounds of an array");
                    }
                    v = &items[i];
                }
                Val::NotUsable(m) => return limit("CE-3", m.clone()),
                Val::Uninit => return ub("INIT-1", "read of an object with no value"),
                Val::Inactive => return ub("UNI-6", "access to a union member that is not active"),
                _ => return unsup("subobject of a scalar"),
            }
        }
        Ok(v)
    }

    /// The value at a place, for writing. `init` is true for an
    /// initialization, which can write a `const` object and, in constant
    /// evaluation, needs no own object.
    pub fn slot(&mut self, p: &Ptr, init: bool) -> R<&mut Val> {
        let fp = full_path(p);
        let constant = self.constant;
        let o = &mut self.objs[p.obj];
        if !o.alive {
            return ub(if constant { "OBJ-2" } else { "OBJ-2, DTOR-7" }, "write to an object whose lifetime has ended");
        }
        if !init && constant && !o.own {
            // CE-5: no change to an object from before the evaluation.
            return limit("CE-5", "modification of an object created outside the constant evaluation");
        }
        if !init && o.konst {
            // CAST-5, CV-5: a write to an object declared `const`.
            return ub("CAST-5", "write to a const object");
        }
        let mut v = &mut o.val;
        for &i in &fp {
            match v {
                Val::Agg(items) => {
                    if i >= items.len() {
                        return ub("PFX-4", "access outside the bounds of an array");
                    }
                    v = &mut items[i];
                }
                Val::Inactive => return ub("UNI-6", "access to a member of a union member that is not active"),
                _ => return unsup("subobject of a scalar"),
            }
        }
        Ok(v)
    }

    pub fn write(&mut self, p: &Ptr, v: Val, init: bool) -> R<()> {
        self.check_in_bounds(p)?;
        *self.slot(p, init)? = v;
        Ok(())
    }

    /// A pointer one past the end does not point to an object (ADD-10,
    /// UN-7).
    fn check_in_bounds(&self, p: &Ptr) -> R<()> {
        if p.array {
            if let Ok(Val::Agg(items)) = self.get_at(&Ptr { array: false, off: 0, ..p.clone() }) {
                if p.off < 0 || p.off as usize >= items.len() {
                    return ub("ADD-10", "access through a pointer past the end of an array");
                }
            }
        } else if p.off != 0 {
            return ub("ADD-10", "access through a pointer past the end of an object");
        }
        Ok(())
    }

    pub fn read(&mut self, p: &Ptr, volatile: bool) -> R<Val> {
        if self.constant && (volatile || self.objs[p.obj].volatile) {
            // CE-12: no read through a volatile lvalue.
            return limit("CE-12", "read of a volatile object");
        }
        self.check_in_bounds(p)?;
        let v = self.get_at(p)?.clone();
        match v {
            Val::Uninit if self.objs[p.obj].heap != Heap::No => ub("NEW-3", "read of an object from new that has no value"),
            Val::Uninit => ub("INIT-1", "read of an object with no value"),
            Val::Inactive => {
                if self.constant {
                    return limit("CE-6", "read of a union member that is not active");
                }
                ub("UNI-6", "read of a union member that is not active")
            }
            Val::Agg(_) if self.constant && has_unset(&v) => limit("CE-22", "copy of an object that has a scalar part with no value"),
            Val::NotUsable(m) => limit("CE-3", m),
            v => Ok(v),
        }
    }

    /// The rules that a use of an object after its lifetime breaks: OBJ-2,
    /// and DTOR-7 after a destructor. An indirection also breaks UN-7. A
    /// run names each of them; constant evaluation names the first.
    pub fn ended_rule(&self, indirection: bool) -> &'static str {
        match (self.constant, indirection) {
            (true, true) => "UN-7",
            (true, false) => "OBJ-2",
            (false, true) => "UN-7, OBJ-2, DTOR-7",
            (false, false) => "OBJ-2, DTOR-7",
        }
    }

    /// End the lifetime of a complete object.
    pub fn end_obj(&mut self, o: usize) {
        if let Some(ob) = self.objs.get_mut(o) {
            ob.alive = false;
        }
    }

    // ---- objects that end ----

    /// Run the destructor of the object at `place`, then end the objects
    /// of its members and bases (DTOR rules, OBJ-1, OBJ-6).
    pub fn destroy(&mut self, place: &Ptr, t: &QTy) -> R<()> {
        if !self.needs_dtor(&t.t) {
            return Ok(());
        }
        match &t.t {
            Ty::Array(el, Some(n)) => {
                for i in (0..*n as usize).rev() {
                    self.destroy(&place.sub(i), el)?;
                }
                Ok(())
            }
            Ty::Class(c) => {
                let c = *c;
                if let Some(d) = self.p.classes[c].dtor {
                    let f = self.p.func(d).cloned();
                    if let Some(f) = f {
                        if !f.defaulted {
                            if self.constant && !f.constexpr {
                                return unsup("destructor evaluation");
                            }
                            let Some(body) = f.body.clone() else {
                                // TR-4: a destructor defined in another unit.
                                if let Some(&(t2, d2)) = self.links.get(&(self.tu, d)) {
                                    if let Some(c2) = self.ps[t2].func(d2).and_then(|g| g.class) {
                                        return self.in_tu(t2, |m| m.destroy(place, &QTy::new(Ty::Class(c2))));
                                    }
                                }
                                return unsup("destructor whose body is not available");
                            };
                            if f.opaque {
                                return unsup("destructor body the front end did not parse");
                            }
                            self.enter_call(Some(d), Some(place.clone()), false, false, None)?;
                            self.cdtor.push((place.clone(), c));
                            let r = match f.fn_try.clone() {
                                Some(hs) => self.run_fn_try(&body, &hs, true),
                                None => self.run_body(&body),
                            };
                            self.cdtor.pop();
                            self.leave_call()?;
                            if let Err(Stop::Throw(x)) = r {
                                if !f.noexcept_false {
                                    // UNW-5: an exception leaves a destructor
                                    // that is not noexcept(false).
                                    return Err(Stop::Terminate("UNW-5", "an exception leaves a destructor that is not declared noexcept(false)".into()));
                                }
                                // UNW-6: the members and bases still end.
                                let e = Stop::Throw(x);
                                return match self.destroy_parts(place, c) {
                                    Ok(()) => Err(e),
                                    Err(Stop::Throw(_)) => Err(Stop::Terminate("UNW-5", "a destructor throws during the unwinding of another exception".into())),
                                    Err(o) => Err(o),
                                };
                            }
                            r?;
                        }
                    }
                }
                self.destroy_parts(place, c)
            }
            _ => Ok(()),
        }
    }

    /// End the members and bases of a class object, in reverse order
    /// (OBJ-1).
    pub fn destroy_parts(&mut self, place: &Ptr, c: usize) -> R<()> {
        if self.p.classes[c].key == 2 {
            return Ok(());
        }
        let nb = self.nbases(c);
        let fields = self.fields(c);
        for (_, fi) in fields.iter().rev() {
            if !fi.ty.t.is_ref() {
                self.destroy(&place.sub(nb + fi.index), &fi.ty)?;
            }
        }
        let bases: Vec<usize> = self.p.classes[c].bases.iter().filter_map(|b| b.class).collect();
        for (i, bc) in bases.iter().enumerate().rev() {
            self.destroy(&place.sub(i), &QTy::new(Ty::Class(*bc)))?;
        }
        Ok(())
    }

    /// End the objects of a list in reverse order.
    pub fn cleanup(&mut self, list: Vec<Cleanup>) -> R<()> {
        for c in list.into_iter().rev() {
            if self.objs[c.place.obj].alive {
                self.destroy(&c.place, &c.ty)?;
                if c.place.path.is_empty() {
                    self.end_obj(c.place.obj);
                }
            }
        }
        Ok(())
    }

    /// Run `f` as a full expression: its temporaries end after it, in the
    /// reverse order of their creation (VC-6, VC-9).
    pub fn full<T>(&mut self, f: impl FnOnce(&mut Self) -> R<T>) -> R<T> {
        self.temps.push(Vec::new());
        let mark = self.extended.len();
        let r = f(self);
        let mut list = self.temps.pop().unwrap();
        if self.extended.len() > mark {
            // No declaration claimed them: they end with the aggregate's
            // own full expression.
            list.extend(self.extended.split_off(mark));
        }
        match r {
            Ok(v) => {
                self.cleanup(list)?;
                Ok(v)
            }
            // UNW-9: an exception ends the temporaries of the full
            // expression.
            Err(e) => Err(self.unwind(list, e)),
        }
    }

    /// End objects while an exception passes (UNW-1, UNW-9). Other stops
    /// end nothing. A destructor that throws during the unwinding ends the
    /// program (UNW-5).
    pub fn unwind(&mut self, list: Vec<Cleanup>, e: Stop) -> Stop {
        if !matches!(e, Stop::Throw(_)) {
            return e;
        }
        self.unwinding += 1;
        let r = self.cleanup(list);
        self.unwinding -= 1;
        match r {
            Ok(()) => e,
            Err(Stop::Throw(_)) => Stop::Terminate("UNW-5", "a destructor throws during the unwinding of another exception".into()),
            Err(x) => x,
        }
    }

    /// Make a temporary object from a prvalue (VC-5, VC-8).
    pub fn materialize(&mut self, e: &Expr) -> R<Ptr> {
        let ty = e.ty.unqual();
        let place = if matches!(ty.t, Ty::Class(_) | Ty::Array(..)) { self.alloc(&ty)? } else {
            let o = self.new_obj(Val::Uninit, true);
            self.objs[o].ty = Some(ty.clone());
            Ptr::whole(o)
        };
        self.init_into(&place, e, &ty)?;
        self.add_temp(&place, &ty);
        Ok(place)
    }

    pub fn add_temp(&mut self, place: &Ptr, ty: &QTy) {
        let c = Cleanup { place: place.clone(), ty: ty.clone(), item: 0 };
        match self.temps.last_mut() {
            Some(l) => l.push(c),
            None => {
                // A temporary outside any full expression lives on.
            }
        }
    }

    /// A temporary that a reference binds lives as long as the reference
    /// (REF-4, OBJ-8). Moves it from the full expression to `to`.
    pub fn extend_temp(&mut self, obj: usize) -> Option<Cleanup> {
        for l in self.temps.iter_mut().rev() {
            if let Some(i) = l.iter().position(|c| c.place.obj == obj) {
                return Some(l.remove(i));
            }
        }
        None
    }

    // ---- calls ----

    /// Start a call: a new frame and block.
    pub fn enter_call(&mut self, func: Option<DeclId>, this: Option<Ptr>, ret_ref: bool, ret_class: bool, slot: Option<Ptr>) -> R<()> {
        self.depth += 1;
        if self.depth > self.depth_limit {
            self.depth -= 1;
            if self.constant {
                return limit("CE-10", "constant evaluation recursion limit exceeded");
            }
            return unsup("recursion depth of the interpreter exceeded");
        }
        let mut fr = Frame::new();
        fr.this = this;
        fr.func = func;
        fr.ret_ref = ret_ref;
        fr.ret_class = ret_class;
        fr.slot = slot;
        fr.block_base = self.blocks.len();
        self.frames.push(fr);
        self.blocks.push(Vec::new());
        self.block_item.push(0);
        Ok(())
    }

    /// End a call: the parameters and the blocks still open end.
    pub fn leave_call(&mut self) -> R<Option<Ret>> {
        let base = self.frame().block_base;
        let mut r = Ok(());
        while self.blocks.len() > base {
            let l = self.blocks.pop().unwrap();
            self.block_item.pop();
            if r.is_ok() {
                r = self.cleanup(l);
            }
        }
        let fr = self.frames.pop().unwrap();
        for (_, o) in fr.vars {
            // A parameter with a destructor ends with the caller's full
            // expression (VC-10).
            if !self.temps.iter().any(|l| l.iter().any(|c| c.place.obj == o)) {
                self.end_obj(o);
            }
        }
        self.depth -= 1;
        r?;
        Ok(fr.ret)
    }
}

/// A value with a scalar part that has no value (CE-21, CE-22).
pub fn has_unset(v: &Val) -> bool {
    match v {
        Val::Uninit => true,
        Val::Agg(items) => items.iter().any(has_unset),
        _ => false,
    }
}

pub fn strip_bind(e: &Expr) -> Expr {
    match &e.k {
        EK::Conv(Conv::Bind, a) => (**a).clone(),
        _ => e.clone(),
    }
}

/// Whether evaluating an expression can have a side effect: a call, an
/// assignment, an increment, or a construction.
pub fn has_effects(e: &Expr) -> bool {
    let mut any = false;
    walk_expr(e, &mut |x| {
        if matches!(x.k, EK::Call(..) | EK::Assign(..) | EK::Construct(..) | EK::Throw(_) | EK::Unary(UOp::PreInc | UOp::PreDec | UOp::PostInc | UOp::PostDec, _)) {
            any = true;
        }
    });
    any
}
