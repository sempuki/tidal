// The interpreter (decision 0048). It runs a checked program on the
// abstract machine of the specification (specification/foundations/translation.md,
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
// specification/foundations/constant-evaluation.md. An operation that constant
// evaluation does not allow stops the run with `Stop::Limit` and the rule
// that forbids it.

mod bytes;
mod expr;
pub mod f80;
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
    /// The program called `exit` (true: the static objects end) or
    /// `_Exit` (false: no object ends) with a status.
    Exit(i128, bool),
}

/// An entry of the catch stack (UNW-7): the handler types of a try
/// block (None for `...`), or the frame depth of a non-throwing function.
#[derive(Debug, Clone)]
pub enum CatchEntry {
    Try(Vec<Option<QTy>>),
    Boundary(u32),
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
    /// A pointer to a byte of an object's representation (STG-13): `off`
    /// is the byte offset in the complete object `obj`, and `path` is
    /// empty.
    pub bytes: bool,
}

impl Ptr {
    pub fn whole(obj: usize) -> Ptr {
        Ptr { obj, path: vec![], off: 0, array: false, bytes: false }
    }
    /// The place of a subobject.
    pub fn sub(&self, i: usize) -> Ptr {
        let mut path = full_path(self);
        path.push(i);
        Ptr { obj: self.obj, path, off: 0, array: false, bytes: false }
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
    /// A `long double` value (TYP-10).
    LD(f80::F80),
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
    /// A pointer that holds an address where no object is (CAST-16).
    Addr(u64),
    /// A pointer to a data member or a member function (MPTR rules), and
    /// the path of base subobjects from the class of the pointer's type to
    /// the class that declares the member (CMP-22). None when a cast to a
    /// base class left the path unknown.
    Member(DeclId, Option<Vec<usize>>),
}

/// How an object was created by `new` (NEW-5).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Heap {
    No,
    Single,
    Array,
    /// Storage from an allocation function called by name (STG-7).
    Raw,
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
    /// The static objects end after main (OBJ-4, OBJ-12).
    pub static_dtors_running: bool,
    /// The handlers and non-throwing function boundaries that an exception
    /// can reach, innermost last (UNW-7).
    pub catch_stack: Vec<CatchEntry>,
    /// The byte offset of each hosted object in the complete object
    /// that holds its byte array (STG-4, STG-6).
    pub resident_abs: HashMap<usize, i64>,
    /// Scalar variables whose initializer runs (OBJ-1, Q-0368).
    pub in_own_init: std::collections::HashSet<usize>,
    /// The aggregate being initialized is in parentheses (REF-5).
    pub paren_aggregate: bool,
    /// The exceptions that active handlers handle, innermost last (TRY-8).
    pub caught: Vec<Exc>,
    /// Objects on which a call of a member function is undefined: a class
    /// object whose bases are being initialized (CTOR-12, the object only),
    /// and an object whose parts ended before the handler of its function
    /// try block (TRY-11, the object and its subobjects).
    pub no_calls: Vec<(Ptr, &'static str)>,
    /// Depth of unwinding (UNW-5).
    pub unwinding: u32,
    /// Objects that placement `new` made in the storage of another object
    /// (STG-4, STG-5): for each host object, the first byte, the size, and
    /// the object.
    pub residents: HashMap<usize, Vec<(i64, i64, usize)>>,
    /// The host and first byte of each such object.
    pub resident_of: HashMap<usize, (usize, i64)>,
    /// Bytes of object representations that the values do not hold:
    /// padding, and the bytes of scalars with no value (STG-11).
    pub shadow: HashMap<(usize, i64), bytes::Byte>,
    /// The address of each object that has one, and the table of
    /// addresses, sizes, and objects (CAST-16).
    pub addr_of: HashMap<usize, u64>,
    pub addr_table: Vec<(u64, u64, usize)>,
    pub next_addr: u64,
    /// The functions whose addresses a program converted.
    pub fn_addrs: Vec<(usize, DeclId)>,
    /// Arrays that storage from an allocation function created
    /// implicitly (STG-8), and the elements of such arrays whose type is
    /// not an implicit-lifetime type and whose lifetime has not begun
    /// (STG-28).
    pub implicit: std::collections::HashSet<usize>,
    /// The implicitly created objects that the program read or wrote,
    /// and the objects that took their storage after such a use: no
    /// set of objects serves both uses (STG-18).
    pub implicit_used: std::collections::HashSet<usize>,
    pub no_set: std::collections::HashSet<usize>,
    pub unbuilt: std::collections::HashSet<(usize, usize)>,
    /// Complete objects that an explicit destructor call ended (DTOR-7).
    pub ended_by_call: std::collections::HashSet<usize>,
    /// Storage from `operator new[]` called by name (NEW-40).
    pub raw_array: std::collections::HashSet<usize>,
    /// The size and the alignment argument of each allocation by name,
    /// for the partner check of NEW-40.
    pub raw_args: HashMap<usize, (i128, Option<i128>)>,
    /// NEW-28: the alignment of storage that a new expression for an
    /// over-aligned type obtains.
    pub raw_align: HashMap<usize, u64>,
    /// Objects of `new` expressions whose allocation function the program
    /// declares, and the storage that the function returned.
    pub user_storage: HashMap<usize, Ptr>,
    /// The choices to take at the choice points, in order, and the choices
    /// taken with the number of options at each (decision 0048).
    pub script: Vec<u8>,
    pub taken: Vec<(u8, u8)>,
    /// The line of each choice point in `taken`.
    pub taken_line: Vec<u32>,
    /// SEQ-4: the reads and writes of objects while unsequenced operands
    /// run: the object, the path of the memory location, whether the
    /// access modifies it (or reads a volatile object), and the frame.
    pub acc: Vec<(usize, Vec<usize>, bool, usize)>,
    pub seq_watch: Vec<usize>,
    /// SEQ-6: the next call's arguments keep the left-to-right order.
    pub fixed_order: bool,
    /// PRIM-9: the choice made for each string literal object.
    pub prim9: HashMap<usize, u8>,
    /// UNI-10: places of the active member that a read through the
    /// common initial sequence reaches with the type of the other member.
    pub uni_alias: std::cell::RefCell<std::collections::HashSet<(usize, Vec<usize>)>>,
    /// The layouts of classes, by translation unit, class, and target.
    pub layouts: std::cell::RefCell<HashMap<(usize, usize, bool), Option<std::rc::Rc<crate::layout::Layout>>>>,
    /// The virtual bases of each class, by translation unit and class.
    pub vbases: std::cell::RefCell<HashMap<(usize, usize), std::rc::Rc<Vec<usize>>>>,
}

impl<'a> Machine<'a> {
    pub fn new(p: &'a P, constant: bool) -> Machine<'a> {
        Machine {
            p,
            ps: vec![p],
            tu: 0,
            links: HashMap::new(),
            uni_alias: Default::default(),
            layouts: Default::default(),
            vbases: Default::default(),
            constant,
            objs: Vec::new(),
            frames: vec![Frame::new()],
            globals: HashMap::new(),
            global_refs: HashMap::new(),
            statics_running: Default::default(),
            steps: 0,
            step_limit: if constant { 30_000_000 } else { 150_000_000 },
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
            static_dtors_running: false,
            catch_stack: Vec::new(),
            resident_abs: HashMap::new(),
            in_own_init: Default::default(),
            paren_aggregate: false,
            caught: Vec::new(),
            no_calls: Vec::new(),
            unwinding: 0,
            residents: HashMap::new(),
            resident_of: HashMap::new(),
            shadow: HashMap::new(),
            addr_of: HashMap::new(),
            addr_table: Vec::new(),
            next_addr: 0,
            fn_addrs: Vec::new(),
            implicit: Default::default(),
            implicit_used: Default::default(),
            no_set: Default::default(),
            unbuilt: Default::default(),
            ended_by_call: Default::default(),
            raw_array: Default::default(),
            raw_args: Default::default(),
            raw_align: Default::default(),
            user_storage: HashMap::new(),
            script: Vec::new(),
            taken: Vec::new(),
            taken_line: Vec::new(),
            acc: Vec::new(),
            seq_watch: Vec::new(),
            fixed_order: false,
            prim9: HashMap::new(),
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

    /// A choice point with `n` options: the option this run takes. The
    /// script gives the options; past its end the run takes option 0.
    pub fn decide(&mut self, n: u8) -> u8 {
        let k = self.taken.len();
        let c = self.script.get(k).copied().unwrap_or(0).min(n - 1);
        self.taken.push((c, n));
        self.taken_line.push(self.line);
        c
    }

    /// SEQ-4: record an access while unsequenced operands run.
    pub fn record(&mut self, p: &Ptr, write: bool) {
        if !self.seq_watch.contains(&self.frames.len()) {
            return;
        }
        let path = if p.bytes { vec![] } else { self.location(p) };
        let depth = self.frames.len();
        self.acc.push((p.obj, path, write, depth));
    }

    /// The memory location of a place: adjacent bit-fields share one
    /// (SEQ-4).
    fn location(&self, p: &Ptr) -> Vec<usize> {
        let mut path = full_path(p);
        if let Some(&last) = path.last() {
            let parent = Ptr { obj: p.obj, path: path[..path.len() - 1].to_vec(), off: 0, array: false, bytes: false };
            if let Some(QTy { t: Ty::Class(c), .. }) = self.type_at(&parent) {
                let nb = self.nbases(c);
                let fs = self.fields(c);
                if let Some(pos) = fs.iter().position(|(_, fi)| nb + fi.index == last) {
                    if fs[pos].1.bits.is_some() {
                        let mut first = pos;
                        while first > 0 && fs[first - 1].1.bits.is_some_and(|w| w > 0) {
                            first -= 1;
                        }
                        let n = path.len();
                        path[n - 1] = nb + fs[first].1.index;
                    }
                }
            }
        }
        path
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

    /// The type of the object or subobject at `p`.
    pub fn type_at(&self, p: &Ptr) -> Option<QTy> {
        let mut t = self.obj_ty(p.obj)?;
        for &i in &full_path(p) {
            t = match &t.t {
                Ty::Array(el, _) => (**el).clone(),
                Ty::Class(c) => {
                    let nb = self.nbases(*c);
                    if i < nb {
                        QTy::new(Ty::Class(self.p.classes[*c].bases[i].class?))
                    } else if i >= self.nslots(*c) {
                        QTy::new(Ty::Class(*self.vbases_of(*c).get(i - self.nslots(*c))?))
                    } else {
                        let f = self.fields(*c).into_iter().find(|(_, fi)| fi.index == i - nb)?;
                        f.1.ty
                    }
                }
                _ => return None,
            };
        }
        Some(t)
    }

    // ---- virtual bases (DER-6, DER-7, CLS-35) ----
    //
    // The value of a complete object of a class with virtual bases holds
    // one slot for each virtual base after its members, in the order of
    // `P::virtual_bases`. The entry of a direct virtual base in the base
    // list holds nothing. A base subobject has no such slots: the path
    // to a virtual base goes to the slot of the complete object that
    // contains the subobject (`canon`).

    /// The virtual bases of a class.
    pub fn vbases_of(&self, c: usize) -> std::rc::Rc<Vec<usize>> {
        let key = (self.tu, c);
        if let Some(v) = self.vbases.borrow().get(&key) {
            return v.clone();
        }
        let v = std::rc::Rc::new(self.p.virtual_bases(c));
        self.vbases.borrow_mut().insert(key, v.clone());
        v
    }

    /// The number of entries of the bases and members in the value of a
    /// class object.
    pub fn nslots(&self, c: usize) -> usize {
        self.nbases(c) + self.p.classes[c].fields.iter().filter(|&&f| matches!(self.p.decls[f].k, DK::Field(_))).count()
    }

    /// The entry of the virtual base `v` in a complete object of class `m`.
    pub fn vslot(&self, m: usize, v: usize) -> Option<usize> {
        self.vbases_of(m).iter().position(|&x| x == v).map(|j| self.nslots(m) + j)
    }

    /// Whether the place is a base subobject.
    pub fn is_base_place(&self, p: &Ptr) -> bool {
        if p.array || p.bytes || p.path.is_empty() {
            return false;
        }
        let n = p.path.len();
        let parent = Ptr { obj: p.obj, path: p.path[..n - 1].to_vec(), off: 0, array: false, bytes: false };
        match self.type_at(&parent).map(|t| t.t) {
            Some(Ty::Class(c)) => p.path[n - 1] < self.nbases(c) || p.path[n - 1] >= self.nslots(c),
            _ => false,
        }
    }

    /// The place with each step into a virtual base moved to the slot of
    /// the complete object that holds it.
    pub fn canon(&self, p: Ptr) -> Ptr {
        let Some(t0) = self.obj_ty(p.obj) else { return p };
        let mut t = t0.t;
        let mut out: Vec<usize> = Vec::new();
        let mut top = 0usize;
        let mut top_class = match t {
            Ty::Class(c) => Some(c),
            _ => None,
        };
        let mut changed = false;
        for &i in &p.path {
            t = match t {
                Ty::Array(el, _) => {
                    out.push(i);
                    top = out.len();
                    top_class = match el.t {
                        Ty::Class(c) => Some(c),
                        _ => None,
                    };
                    el.t
                }
                Ty::Class(c) => {
                    let nb = self.nbases(c);
                    if i < nb {
                        let b = &self.p.classes[c].bases[i];
                        let Some(bc) = b.class else { return p };
                        if b.virt {
                            let Some(m) = top_class else { return p };
                            let Some(slot) = self.vslot(m, bc) else { return p };
                            out.truncate(top);
                            out.push(slot);
                            changed = true;
                        } else {
                            out.push(i);
                        }
                        Ty::Class(bc)
                    } else if i >= self.nslots(c) {
                        let Some(&v) = self.vbases_of(c).get(i - self.nslots(c)) else { return p };
                        out.push(i);
                        Ty::Class(v)
                    } else {
                        let Some(f) = self.fields(c).into_iter().find(|(_, fi)| fi.index == i - nb) else { return p };
                        out.push(i);
                        top = out.len();
                        top_class = match f.1.ty.t {
                            Ty::Class(fc) => Some(fc),
                            _ => None,
                        };
                        f.1.ty.t
                    }
                }
                _ => return p,
            };
        }
        if !changed {
            return p;
        }
        Ptr { path: out, ..p }
    }

    /// The place of direct base `i` of the class object at `place`.
    pub fn base_sub(&self, place: &Ptr, c: usize, i: usize) -> Ptr {
        if self.p.classes[c].bases[i].virt {
            self.canon(place.sub(i))
        } else {
            place.sub(i)
        }
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
        if ci.bases.iter().any(|b| b.class.is_none()) {
            return unsup("base class of unknown type");
        }
        if self.constant && !self.vbases_of(c).is_empty() {
            // DER-12, CE-2: no constexpr constructor builds a class with a
            // virtual base.
            return limit("DER-12", format!("an object of '{}', a class with a virtual base, in constant evaluation", ci.name));
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
            Ty::Class(c) => self.shape_class(*c, true)?,
            Ty::Unknown | Ty::Dep(_) => return unsup("object of unknown type"),
            Ty::Void | Ty::Func(_) => return unsup("object of void or function type"),
            _ => Val::Uninit,
        })
    }

    /// The value of a new object of class `c`, a complete object or a
    /// base subobject.
    pub fn shape_class(&self, c: usize, complete: bool) -> R<Val> {
        Ok({
            {
                self.class_ok(c)?;
                let mut v = Vec::new();
                for b in &self.p.classes[c].bases {
                    if b.virt {
                        v.push(Val::Void);
                    } else {
                        v.push(self.shape_class(b.class.unwrap(), false)?);
                    }
                }
                let union = self.p.classes[c].key == 2;
                for (f, fi) in self.fields(c) {
                    if fi.bits.is_some() && self.p.decls[f].name.is_empty() && !union {
                        // BF-12: an unnamed bit-field is not a member and
                        // holds no value that a copy reads.
                        v.push(Val::Int(0));
                    } else if union {
                        v.push(Val::Inactive);
                    } else if fi.ty.t.is_ref() {
                        v.push(Val::Uninit);
                    } else {
                        v.push(self.shape(&fi.ty)?);
                    }
                }
                if complete {
                    for &vb in self.vbases_of(c).iter() {
                        v.push(self.shape_class(vb, false)?);
                    }
                }
                Val::Agg(v)
            }
        })
    }

    /// Zero-initialization (INIT rules): every scalar is zero, and a union
    /// has its first member active.
    /// The number of scalar parts of a type, stopping above 4096.
    pub fn leaves(&self, t: &QTy, depth: u32) -> u64 {
        if depth > 32 {
            return u64::MAX;
        }
        match &t.t {
            Ty::Array(el, Some(n)) => {
                if *n > 4096 {
                    return u64::MAX;
                }
                (*n as u64).saturating_mul(self.leaves(el, depth + 1))
            }
            Ty::Class(c) => {
                let mut k: u64 = 0;
                for b in &self.p.classes[*c].bases {
                    if let Some(bc) = b.class {
                        k = k.saturating_add(self.leaves(&QTy::new(Ty::Class(bc)), depth + 1));
                    }
                }
                for (_, fi) in self.fields(*c) {
                    k = k.saturating_add(self.leaves(&fi.ty, depth + 1));
                }
                k
            }
            _ => 1,
        }
    }

    pub fn zero(&self, t: &QTy) -> Val {
        match &t.t {
            Ty::Ptr(_) | Ty::NullPtr | Ty::MemPtr(..) => Val::Null,
            Ty::LDouble => Val::LD(f80::F80::Zero(false)),
            x if x.is_floating() => Val::Float(0.0),
            Ty::Array(el, Some(n)) => Val::Agg((0..*n).map(|_| self.zero(el)).collect()),
            Ty::Class(c) => self.zero_class(*c, true),
            _ => Val::Int(0),
        }
    }

    /// The zero value of a complete object or a base subobject of class
    /// `c`.
    pub fn zero_class(&self, c: usize, complete: bool) -> Val {
        let c = &c;
        {
            {
                let mut v = Vec::new();
                for b in &self.p.classes[*c].bases {
                    if let Some(bc) = b.class {
                        v.push(if b.virt { Val::Void } else { self.zero_class(bc, false) });
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
                if complete {
                    for &vb in self.vbases_of(*c).iter() {
                        v.push(self.zero_class(vb, false));
                    }
                }
                Val::Agg(v)
            }
        }
    }

    /// Whether ending an object of this type runs code.
    pub fn needs_dtor(&self, t: &Ty) -> bool {
        match t {
            Ty::Array(el, _) => self.needs_dtor(&el.t),
            Ty::Class(c) => {
                let ci = &self.p.classes[*c];
                if let Some(d) = ci.dtor {
                    // CE-2: in constant evaluation, a destructor that is
                    // not constexpr counts even with an empty body.
                    if self.p.func(d).is_some_and(|f| !f.defaulted && (!matches!(f.body.as_deref(), Some(Stmt::Block(v)) if v.is_empty()) || (self.constant && !f.constexpr))) {
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
        let writable = self.objs[p.obj].konst && !init && (self.mutable_on_path(p) || self.cdtor.iter().any(|(q, _)| q.obj == p.obj));
        let o = &mut self.objs[p.obj];
        if !o.alive {
            return ub(if constant { "OBJ-2" } else { "OBJ-2, DTOR-7" }, "write to an object whose lifetime has ended");
        }
        if !init && self.in_own_init.contains(&p.obj) {
            return ub("OBJ-2", "write to a variable inside its own initializer, before its lifetime begins");
        }
        let o = &mut self.objs[p.obj];
        if !init && constant && !o.own {
            // CE-5: no change to an object from before the evaluation.
            return limit("CE-5", "modification of an object created outside the constant evaluation");
        }
        if !init && o.konst && !writable {
            // CAST-5, CV-5: a write to an object declared `const`.
            return ub("CAST-5, CV-5", "write to a const object");
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

    /// Whether a place is in a mutable member (CV-5).
    pub fn mutable_on_path(&self, p: &Ptr) -> bool {
        let Some(mut t) = self.obj_ty(p.obj) else { return false };
        for &i in &full_path(p) {
            t = match &t.t {
                Ty::Array(el, _) => (**el).clone(),
                Ty::Class(c) => {
                    let nb = self.nbases(*c);
                    if i < nb {
                        match self.p.classes[*c].bases[i].class {
                            Some(b) => QTy::new(Ty::Class(b)),
                            None => return false,
                        }
                    } else {
                        match self.fields(*c).into_iter().find(|(_, fi)| fi.index == i - nb) {
                            Some((_, fi)) if fi.mutable => return true,
                            Some((_, fi)) => fi.ty,
                            None => return false,
                        }
                    }
                }
                _ => return false,
            };
        }
        false
    }

    /// STG-8, STG-18: the use of an implicitly created object.
    fn implicit_use(&mut self, p: &Ptr) -> R<()> {
        if p.bytes {
            return Ok(());
        }
        if self.no_set.contains(&p.obj) {
            return ub("STG-18", "access to storage as an object of another type than the implicitly created object that the program used");
        }
        if self.implicit.contains(&p.obj) {
            self.implicit_used.insert(p.obj);
        }
        Ok(())
    }

    pub fn write(&mut self, p: &Ptr, v: Val, init: bool) -> R<()> {
        if !init {
            self.record(p, true);
            self.implicit_use(p)?;
        }
        if p.bytes || self.hosted_byte(p).is_some() {
            // STG-13: a write of one byte of an object representation.
            let (o, k) = if p.bytes { (p.obj, p.off) } else { (p.obj, p.off) };
            let b = match v {
                Val::Int(x) => bytes::Byte::Known(x as u8),
                Val::Uninit => bytes::Byte::Indet,
                _ => return unsup("byte write of a value that is not an integer"),
            };
            return self.write_byte(o, k, b);
        }
        self.check_in_bounds(p)?;
        let mut v = v;
        if let Val::Agg(items) = &mut v {
            // A value with the slots of virtual bases written to a base
            // subobject, or the reverse: the slots follow the place.
            if let Ok(Val::Agg(old)) = self.get_at(p) {
                if old.len() != items.len() {
                    if let Some(Ty::Class(c)) = self.type_at(p).map(|t| t.t) {
                        if !self.vbases_of(c).is_empty() {
                            if old.len() < items.len() {
                                items.truncate(old.len());
                            } else {
                                items.extend(old[items.len()..].iter().cloned());
                            }
                        }
                    }
                }
            }
        }
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
        // SEQ-4 also makes two unsequenced reads of one volatile object
        // undefined. The run does not stop there: seven run tests read a
        // volatile object twice in one operator (finding G102).
        let _ = volatile;
        self.record(p, false);
        self.implicit_use(p)?;
        if p.bytes || self.hosted_byte(p).is_some() {
            // STG-13: a byte of an object representation, as `unsigned
            // char` (or as the element type of the host array).
            let t = if p.bytes { Ty::UChar } else { self.type_at(p).map(|t| t.t).unwrap_or(Ty::UChar) };
            let b = self.read_byte(p.obj, p.off)?;
            return match self.byte_val(b, &t) {
                Val::Uninit => ub("OBJ-2, INIT-1", "read of a byte with no value"),
                v => Ok(v),
            };
        }
        if self.constant && (volatile || self.objs[p.obj].volatile) {
            // CE-12: no read through a volatile lvalue.
            return limit("CE-12", "read of a volatile object");
        }
        self.check_in_bounds(p)?;
        let v = self.get_at(p)?.clone();
        match v {
            // OBJ-2 is the rule for a read of a scalar with no value.
            Val::Uninit if self.objs[p.obj].heap != Heap::No => ub("OBJ-2, NEW-3", "read of an object from new that has no value"),
            Val::Uninit => ub("OBJ-2, INIT-1", "read of an object with no value"),
            Val::Inactive => {
                if self.constant {
                    return limit("CE-6", "read of a union member that is not active");
                }
                ub("UNI-6", "read of a union member that is not active")
            }
            Val::Agg(_) if self.constant && first_not_usable(&v).is_some() => limit("CE-3", first_not_usable(&v).unwrap()),
            Val::Agg(_) if self.constant && has_unset(&v) => limit("CE-22", "copy of an object that has a scalar part with no value"),
            // OBJ-2: an implicit copy or move reads each scalar part.
            Val::Agg(_) if has_unset(&v) => ub("OBJ-2", "implicit copy of an object that has a scalar part with no value"),
            Val::NotUsable(m) => limit("CE-3", m),
            v => Ok(v),
        }
    }

    /// The rule that a use of an object after its lifetime breaks:
    /// OBJ-2, to which UN-7 and DTOR-7 refer.
    pub fn ended_rule(&self, indirection: bool) -> &'static str {
        let _ = indirection;
        "OBJ-2"
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
                                // CE-2: a call of a destructor that is not
                                // constexpr.
                                return limit("CE-2", format!("call of the destructor of '{}', which is not constexpr", self.p.classes[c].name));
                            }
                            if f.body.is_none() && self.library_exception(c) {
                                // LSUP-8: the library defines the destructor;
                                // it ends the object and does nothing else.
                                return self.destroy_parts(place, c);
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
                                // DTOR-4: a destructor without a specifier
                                // follows its bases and members.
                                if !f.noexcept_false && self.p.sm_throws(c, crate::special::Sm::Dtor) != Some(true) {
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
        let bases: Vec<(usize, bool)> = self.p.classes[c].bases.iter().filter_map(|b| b.class.map(|x| (x, b.virt))).collect();
        for (i, (bc, virt)) in bases.iter().enumerate().rev() {
            if !virt {
                self.destroy(&place.sub(i), &QTy::new(Ty::Class(*bc)))?;
            }
        }
        // DER-7: the virtual bases end last, with the complete object.
        if !self.is_base_place(place) {
            let vb = self.vbases_of(c);
            for (j, &v) in vb.iter().enumerate().rev() {
                self.destroy(&place.sub(self.nslots(c) + j), &QTy::new(Ty::Class(v)))?;
            }
        }
        Ok(())
    }

    /// End the objects of a list in reverse order.
    pub fn cleanup(&mut self, list: Vec<Cleanup>) -> R<()> {
        for c in list.into_iter().rev() {
            let nontrivial = self.needs_dtor(&c.ty.t) || matches!(&c.ty.t, Ty::Class(k) if self.p.classes[*k].dtor.is_some_and(|d| self.p.func(d).is_some_and(|f| !f.defaulted)));
            if !self.objs[c.place.obj].alive && c.place.path.is_empty() && nontrivial {
                if let Some(r) = self.residents.get(&c.place.obj).and_then(|v| v.iter().rev().find(|r| self.objs[r.2].alive)).map(|r| r.2) {
                    let rt = self.obj_ty(r).map(|q| q.unqual().t);
                    if rt != Some(c.ty.unqual().t) {
                        // STG-21: the end of the block destroys the variable
                        // as its declared type.
                        return ub("STG-21", "the storage of a variable holds an object of another type when its destructor runs");
                    }
                }
            }
            if !self.objs[c.place.obj].alive && c.place.path.is_empty() && self.resident_holder(c.place.obj).is_none() && self.ended_by_call.contains(&c.place.obj) {
                // DTOR-7: a second destruction of an object whose
                // destructor a program called (Q-0054).
                return ub("DTOR-7", "the end of the block destroys an object whose destructor already ran");
            }
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
            // UNW-1: an exception ends the temporaries of the full
            // expression.
            Err(e) => Err(self.unwind(list, e)),
        }
    }

    /// End objects while an exception passes (UNW-1). Other stops
    /// end nothing. A destructor that throws during the unwinding ends the
    /// program (UNW-5).
    pub fn unwind(&mut self, list: Vec<Cleanup>, e: Stop) -> Stop {
        let Stop::Throw(x) = &e else {
            return e;
        };
        // UNW-7: with no matching handler, no object is destroyed; at the
        // boundary of a non-throwing function, the objects of the functions
        // it calls are destroyed and its own objects are not (the matrix
        // table, Clang's choice for an unmatched try block).
        if !self.unwinding_reaches(&x.ty.clone()) {
            return Stop::Terminate("UNW-5, UNW-7", "an exception has no matching handler before main or a non-throwing function, and the objects are not destroyed".into());
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

    /// UNW-7: whether the unwinding of an exception of type `t` destroys
    /// the objects of the current frame.
    fn unwinding_reaches(&self, t: &QTy) -> bool {
        for c in self.catch_stack.iter().rev() {
            match c {
                CatchEntry::Try(hs) => {
                    if hs.iter().any(|h| self.handler_matches(h.as_ref(), t)) {
                        return true;
                    }
                }
                CatchEntry::Boundary(d) => return self.depth > *d,
            }
        }
        false
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
pub fn first_not_usable(v: &Val) -> Option<String> {
    match v {
        Val::NotUsable(m) => Some(m.clone()),
        Val::Agg(items) => items.iter().find_map(first_not_usable),
        _ => None,
    }
}

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

/// Whether an operand can take part in a SEQ-4 conflict: it modifies an
/// object outside a call.
pub fn may_conflict(e: &Expr) -> bool {
    let mut any = false;
    walk_expr(e, &mut |x| {
        if matches!(x.k, EK::Assign(..) | EK::Unary(UOp::PreInc | UOp::PreDec | UOp::PostInc | UOp::PostDec, _)) {
            any = true;
        }
    });
    any
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
