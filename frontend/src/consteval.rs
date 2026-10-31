// Constant evaluation (spec/foundations/constant-evaluation.md). The
// evaluator runs the abstract machine on the expression and statement
// trees and rejects the operations that CE-2 to CE-16 list, and every
// operation that an `undefined` rule names (CE-9, TR-8).

use crate::ast::*;
use crate::parser::*;
use crate::types::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum CeErr {
    /// Not a constant expression: the rule that says so and a message.
    NotConst(&'static str, String),
    /// The evaluator met something it does not model.
    Unsupported(String),
}

type R<T> = Result<T, CeErr>;

fn nc<T>(rule: &'static str, msg: impl Into<String>) -> R<T> {
    Err(CeErr::NotConst(rule, msg.into()))
}
fn unsup<T>(msg: impl Into<String>) -> R<T> {
    Err(CeErr::Unsupported(msg.into()))
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ptr {
    pub obj: usize,
    pub path: Vec<usize>,
    /// For an array pointer, the element index (can be one past the end).
    pub off: i64,
    pub array: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Int(i128),
    Float(f64),
    Ptr(Ptr),
    FnPtr(DeclId),
    Null,
    Agg(Vec<Val>),
    Uninit,
    /// A union member that is not the active member (CE-6).
    Inactive,
    Void,
    /// The value of a variable that constant evaluation must not read
    /// (CE-3).
    NotUsable(String),
}

#[derive(Clone, Debug)]
pub struct Obj {
    pub val: Val,
    pub alive: bool,
    /// Created by this evaluation, so it can be changed (CE-5).
    pub own: bool,
    pub volatile: bool,
    pub is_static: bool,
    pub string: bool,
    /// A complete object declared `const` (CAST-5, CV-5).
    pub konst: bool,
}

enum Flow {
    Normal,
    Break,
    Continue,
    Return(Option<Val>),
}

pub struct Eval<'a> {
    pub p: &'a P,
    pub objs: Vec<Obj>,
    frames: Vec<HashMap<DeclId, usize>>,
    refs: Vec<HashMap<DeclId, Ptr>>,
    globals: HashMap<DeclId, usize>,
    global_refs: HashMap<DeclId, Ptr>,
    steps: u64,
    depth: u32,
    this: Vec<Option<Ptr>>,
    in_consteval_fn: bool,
    locals_created: Vec<usize>,
    local_mark: Vec<usize>,
    /// Whether each active call returns a reference.
    ret_ref: Vec<bool>,
}

const STEP_LIMIT: u64 = 2_000_000;
const DEPTH_LIMIT: u32 = 512;

impl<'a> Eval<'a> {
    pub fn new(p: &'a P) -> Eval<'a> {
        Eval {
            p,
            objs: Vec::new(),
            frames: vec![HashMap::new()],
            refs: vec![HashMap::new()],
            globals: HashMap::new(),
            global_refs: HashMap::new(),
            steps: 0,
            depth: 0,
            this: vec![None],
            in_consteval_fn: false,
            locals_created: Vec::new(),
            local_mark: Vec::new(),
            ret_ref: Vec::new(),
        }
    }

    fn step(&mut self) -> R<()> {
        self.steps += 1;
        if self.steps > STEP_LIMIT {
            // CE-10: an implementation limit.
            return nc("CE-10", "constant evaluation step limit exceeded");
        }
        Ok(())
    }

    fn new_obj(&mut self, val: Val, own: bool) -> usize {
        self.objs.push(Obj { val, alive: true, own, volatile: false, is_static: false, string: false, konst: false });
        self.objs.len() - 1
    }

    // ---- places ----

    fn full_path(p: &Ptr) -> Vec<usize> {
        let mut v = p.path.clone();
        if p.array {
            v.push(p.off as usize);
        }
        v
    }

    fn get_at(&self, p: &Ptr) -> R<&Val> {
        let o = &self.objs[p.obj];
        if !o.alive {
            return nc("UN-7", "access to an object whose lifetime has ended");
        }
        let mut v = &o.val;
        let fp = Self::full_path(p);
        for &i in &fp {
            match v {
                Val::Agg(items) => {
                    if i >= items.len() {
                        // PFX-4 / ADD-10: outside the array.
                        return nc("PFX-4", "access outside the bounds of an array");
                    }
                    v = &items[i];
                }
                Val::NotUsable(m) => return nc("CE-3", m.clone()),
                Val::Uninit => return nc("INIT-1", "read of an uninitialized object"),
                _ => return unsup("subobject of a scalar"),
            }
        }
        Ok(v)
    }

    /// CAST-5, CV-5: a write to an object declared `const` is undefined.
    /// Such a write is only possible through a `const_cast`.
    fn check_const_write(&self, p: &Ptr) -> R<()> {
        if self.objs[p.obj].konst {
            return nc("CAST-5", "write to a const object through the result of a const_cast");
        }
        Ok(())
    }

    fn get_mut_at(&mut self, p: &Ptr) -> R<&mut Val> {
        let fp = Self::full_path(p);
        let o = &mut self.objs[p.obj];
        if !o.alive {
            return nc("UN-7", "access to an object whose lifetime has ended");
        }
        if !o.own {
            // CE-5: no change to an object from before the evaluation.
            return nc("CE-5", "modification of an object created outside the constant evaluation");
        }
        let mut v = &mut o.val;
        for &i in &fp {
            match v {
                Val::Agg(items) => {
                    if i >= items.len() {
                        return nc("PFX-4", "access outside the bounds of an array");
                    }
                    v = &mut items[i];
                }
                _ => return unsup("subobject of a scalar"),
            }
        }
        Ok(v)
    }

    fn read(&self, p: &Ptr, volatile: bool) -> R<Val> {
        if volatile || self.objs[p.obj].volatile {
            // CE-12: no read through a volatile lvalue.
            return nc("CE-12", "read of a volatile object");
        }
        if p.array {
            if let Ok(Val::Agg(items)) = self.get_at(&Ptr { array: false, off: 0, ..p.clone() }) {
                if p.off < 0 || p.off as usize >= items.len() {
                    return nc("UN-7", "read through a pointer past the end of an array");
                }
            }
        } else if p.off != 0 {
            return nc("UN-7", "read through a pointer past the end of an object");
        }
        let v = self.get_at(p)?.clone();
        match v {
            Val::Uninit => nc("INIT-1", "read of an uninitialized object"),
            Val::Inactive => nc("CE-6", "read of a union member that is not active"),
            // CE-22: a copy of an object reads each of its scalar parts.
            Val::Agg(_) if has_unset(&v) => nc("CE-22", "copy of an object that has a scalar part with no value"),
            Val::NotUsable(m) => nc("CE-3", m),
            v => Ok(v),
        }
    }

    // ---- variables ----

    fn var_place(&mut self, id: DeclId, line: u32) -> R<Ptr> {
        let _ = line;
        if let Some(r) = self.refs.last().unwrap().get(&id) {
            return Ok(r.clone());
        }
        if let Some(&o) = self.frames.last().unwrap().get(&id) {
            return Ok(Ptr { obj: o, path: vec![], off: 0, array: false });
        }
        if let Some(r) = self.global_refs.get(&id) {
            return Ok(r.clone());
        }
        if let Some(&o) = self.globals.get(&id) {
            return Ok(Ptr { obj: o, path: vec![], off: 0, array: false });
        }
        let Some(v) = self.p.var(id) else {
            return unsup("reference to something that is not a variable");
        };
        let v = v.clone();
        let name = self.p.decls[id].name.clone();
        if v.dependent {
            return unsup("value of a template parameter, structured binding, or init capture");
        }
        let usable_local = v.local && !v.param && v.init.is_some() && !v.ty.v && (v.constexpr || (v.ty.c && (v.ty.t.is_integral() || matches!(v.ty.t, Ty::Enum(_)))));
        if (v.param || (v.local && v.storage == Storage::Auto)) && !usable_local {
            // A variable of a caller that is not part of this evaluation.
            let o = self.new_obj(Val::NotUsable(format!("'{}' is not usable in constant expressions", name)), false);
            self.globals.insert(id, o);
            return Ok(Ptr { obj: o, path: vec![], off: 0, array: false });
        }
        // CE-3: usable if constexpr, or const integral/enum with a
        // constant initializer (CV-8).
        let usable_type = v.ty.c && !v.ty.v && (v.ty.t.is_integral() || matches!(v.ty.t, Ty::Enum(_)));
        let is_ref = v.ty.t.is_ref();
        let usable = (v.constexpr || usable_type || (is_ref && v.init.is_some())) && !v.ty.v;
        let val = if usable {
            match &v.init {
                Some(init) => {
                    let init = init.clone();
                    if is_ref {
                        // A reference: its object is the place of the initializer.
                        let place = self.with_fresh_frame(|ev| ev.lvalue(&strip_bind(&init)));
                        match place {
                            Ok(pl) => {
                                if !v.constexpr && !self.objs[pl.obj].is_static {
                                    return nc("CE-3", format!("reference '{}' is not usable in constant expressions", name));
                                }
                                self.global_refs.insert(id, pl.clone());
                                return Ok(pl);
                            }
                            Err(CeErr::NotConst(..)) => return nc("CE-3", format!("'{}' is not initialized with a constant expression", name)),
                            Err(e) => return Err(e),
                        }
                    }
                    match self.with_fresh_frame(|ev| ev.rvalue_init(&init, &v.ty)) {
                        Ok(x) => x,
                        Err(CeErr::NotConst(r, m)) => {
                            if v.constexpr {
                                return Err(CeErr::NotConst(r, m));
                            }
                            Val::NotUsable(format!("'{}' has an initializer that is not a constant expression", name))
                        }
                        Err(e) => return Err(e),
                    }
                }
                None => {
                    if matches!(v.ty.t, Ty::Class(_)) {
                        return unsup("default-initialized class object (constructor evaluation)");
                    }
                    if v.class.is_some() || v.storage == Storage::Extern {
                        Val::NotUsable(format!("'{}' has no initializer here", name))
                    } else {
                        Val::NotUsable(format!("'{}' has no initializer", name))
                    }
                }
            }
        } else {
            let why = if v.ty.v {
                format!("'{}' is volatile", name)
            } else if v.ty.t.is_floating() && v.ty.c {
                format!("'{}' is a const floating-point variable, not constexpr (CV-8)", name)
            } else {
                format!("'{}' is not constexpr or a const integer with a constant initializer", name)
            };
            Val::NotUsable(why)
        };
        let o = self.new_obj(val, false);
        self.objs[o].is_static = true;
        self.objs[o].volatile = v.ty.v;
        self.globals.insert(id, o);
        Ok(Ptr { obj: o, path: vec![], off: 0, array: false })
    }

    fn with_fresh_frame<T>(&mut self, f: impl FnOnce(&mut Self) -> R<T>) -> R<T> {
        self.frames.push(HashMap::new());
        self.refs.push(HashMap::new());
        self.this.push(None);
        let r = f(self);
        self.frames.pop();
        self.refs.pop();
        self.this.pop();
        r
    }

    // ---- expressions ----

    /// Evaluate a glvalue to its place.
    pub fn lvalue(&mut self, e: &Expr) -> R<Ptr> {
        self.step()?;
        match &e.k {
            EK::Var(id) => self.var_place(*id, e.line),
            EK::Unary(UOp::Deref, a) => {
                let v = self.eval(a)?;
                match v {
                    Val::Ptr(p) => {
                        // UN-7: the pointer must point to an object.
                        if p.array {
                            if let Ok(Val::Agg(items)) = self.get_at(&Ptr { array: false, off: 0, ..p.clone() }) {
                                if p.off < 0 || p.off as usize > items.len() {
                                    return nc("ADD-10", "pointer outside its array");
                                }
                            }
                        }
                        Ok(p)
                    }
                    Val::Null => nc("UN-7", "indirection through a null pointer"),
                    Val::FnPtr(_) => unsup("function lvalue"),
                    _ => unsup("indirection through a non-pointer"),
                }
            }
            EK::Unary(UOp::PreInc | UOp::PreDec, _) | EK::Assign(..) => {
                let _ = self.eval_effect(e)?;
                let target = match &e.k {
                    EK::Unary(_, a) => a,
                    EK::Assign(_, a, _) => a,
                    _ => unreachable!(),
                };
                // re-evaluating the place is safe only for simple names
                match &target.k {
                    EK::Var(_) | EK::Member(..) => self.lvalue_no_effects(target),
                    _ => unsup("lvalue of a complex assignment"),
                }
            }
            EK::Member(base, fid) => {
                let fi = match &self.p.decls[*fid].k {
                    DK::Field(f) => f.clone(),
                    _ => return unsup("member"),
                };
                let bp = if base.cat == Cat::PR {
                    let v = self.eval(base)?;
                    let o = self.new_obj(v, true);
                    Ptr { obj: o, path: vec![], off: 0, array: false }
                } else {
                    self.lvalue(base)?
                };
                let mut path = Self::full_path(&bp);
                if bp.array && false {
                    path.pop();
                }
                if !bp.array && bp.off != 0 {
                    return nc("UN-7", "member access through a pointer past the end");
                }
                // a union member must be the active one (CE-6)
                if self.p.classes[fi.class].fields.len() != self.p.classes[fi.class].fields.iter().filter(|&&f| matches!(self.p.decls[f].k, DK::Field(_))).count() {
                    return unsup("class layout");
                }
                if !self.p.classes[fi.class].bases.is_empty() {
                    return unsup("member of a class with bases");
                }
                path.push(fi.index);
                if fi.ty.t.is_ref() {
                    let v = self.read(&Ptr { obj: bp.obj, path: path.clone(), off: 0, array: false }, false)?;
                    if let Val::Ptr(p) = v {
                        return Ok(p);
                    }
                    return unsup("reference member");
                }
                Ok(Ptr { obj: bp.obj, path, off: 0, array: false })
            }
            EK::Index(a, i) => {
                let base = self.eval(a)?;
                let idx = self.eval_int(i)?;
                match base {
                    Val::Ptr(p) => self.ptr_add(&p, idx, true),
                    Val::Null => nc("UN-7", "subscript of a null pointer"),
                    _ => unsup("subscript"),
                }
            }
            EK::Str(units) => {
                let o = self.new_obj(Val::Agg(units.iter().map(|&u| Val::Int(u as i128)).chain(std::iter::once(Val::Int(0))).collect()), false);
                self.objs[o].string = true;
                self.objs[o].is_static = true;
                Ok(Ptr { obj: o, path: vec![], off: 0, array: false })
            }
            EK::Comma(a, b) => {
                self.eval(a)?;
                self.lvalue(b)
            }
            EK::Cond(c, a, b) => {
                if self.eval_bool(c)? {
                    self.lvalue(a)
                } else {
                    self.lvalue(b)
                }
            }
            EK::Conv(Conv::Bind, a) | EK::Conv(Conv::NoOp, a) => {
                if a.cat == Cat::PR {
                    // a temporary
                    let v = self.eval(a)?;
                    let o = self.new_obj(v, true);
                    return Ok(Ptr { obj: o, path: vec![], off: 0, array: false });
                }
                self.lvalue(a)
            }
            EK::Call(..) => {
                // a call returning a reference
                let v = self.eval_call(e, true)?;
                match v {
                    Val::Ptr(p) => Ok(p),
                    _ => unsup("call returning a reference"),
                }
            }
            EK::This => unsup("this as lvalue"),
            EK::Unknown(why) => self.unknown(why),
            _ => {
                if e.cat == Cat::PR {
                    let v = self.eval(e)?;
                    let o = self.new_obj(v, true);
                    return Ok(Ptr { obj: o, path: vec![], off: 0, array: false });
                }
                unsup(format!("lvalue of {:?}", std::mem::discriminant(&e.k)))
            }
        }
    }

    fn lvalue_no_effects(&mut self, e: &Expr) -> R<Ptr> {
        self.lvalue(e)
    }

    fn unknown<T>(&self, why: &str) -> R<T> {
        if let Some(rest) = why.strip_prefix("CE-4: ") {
            // CE-4: reinterpret_cast and void* to object pointer.
            return nc("CE-4", rest.to_string());
        }
        if let Some(rest) = why.strip_prefix("LAM-19: ") {
            return nc("LAM-19", rest.to_string());
        }
        unsup(why.to_string())
    }

    pub fn eval_int(&mut self, e: &Expr) -> R<i128> {
        match self.eval(e)? {
            Val::Int(v) => Ok(v),
            Val::Float(_) => unsup("float where integer expected"),
            _ => unsup("non-integer value"),
        }
    }

    pub fn eval_bool(&mut self, e: &Expr) -> R<bool> {
        match self.eval(e)? {
            Val::Int(v) => Ok(v != 0),
            Val::Float(f) => Ok(f != 0.0),
            Val::Null => Ok(false),
            Val::Ptr(_) | Val::FnPtr(_) => Ok(true),
            _ => unsup("non-scalar condition"),
        }
    }

    fn eval_effect(&mut self, e: &Expr) -> R<Val> {
        self.eval(e)
    }

    /// Evaluate a prvalue (or any expression for its value).
    pub fn eval(&mut self, e: &Expr) -> R<Val> {
        self.step()?;
        let ty = &e.ty.t;
        match &e.k {
            EK::Int(v) => Ok(Val::Int(*v)),
            EK::Float(f) => Ok(Val::Float(*f)),
            EK::NullPtr => Ok(Val::Null),
            EK::ZeroInit => Ok(self.zero(&e.ty)),
            EK::Str(_) => {
                let p = self.lvalue(e)?;
                Ok(Val::Ptr(p))
            }
            EK::Var(_) | EK::Member(..) | EK::Index(..) => {
                if e.cat == Cat::PR {
                    return unsup("prvalue member");
                }
                if let Ty::Class(c) = &e.ty.t {
                    if self.p.has_user_copy(*c) {
                        return unsup("copy with a user-provided copy or move constructor");
                    }
                }
                let p = self.lvalue(e)?;
                self.read(&p, e.ty.v)
            }
            EK::Func(f) => Ok(Val::FnPtr(*f)),
            EK::This => match self.this.last().cloned().flatten() {
                Some(p) => Ok(Val::Ptr(p)),
                None => nc("CE-3", "'this' of an object outside the evaluation"),
            },
            EK::Conv(k, a) => self.eval_conv(*k, a, &e.ty, e.line),
            EK::Unary(op, a) => self.eval_unary(*op, a, e),
            EK::Binary(op, a, b) => self.eval_binary(*op, a, b, &e.ty),
            EK::Assign(op, l, r) => {
                let rv = self.eval(r)?;
                let place = self.lvalue(l)?;
                let nv = match op {
                    None => rv,
                    Some((bop, ct)) => {
                        let old = self.read(&place, l.ty.v)?;
                        // convert the left value to the computation type
                        let lv = self.convert_val(old, &l.ty.t, &ct.t)?;
                        let res = self.arith(*bop, lv, rv, &ct.t, l.ty.t.clone())?;
                        self.convert_val(res, &ct.t, &l.ty.t)?
                    }
                };
                // BF-10: a bit-field keeps the low bits of the value.
                let mut nv = nv;
                if let EK::Member(_, fid) = &l.k {
                    if let DK::Field(fi) = &self.p.decls[*fid].k {
                        if let (Some(w), Val::Int(x)) = (fi.bits, &nv) {
                            if w > 0 && w < 64 && fi.ty.t.is_integral() && fi.ty.t != Ty::Bool {
                                let m = 1i128 << w;
                                let mut y = x.rem_euclid(m);
                                if fi.ty.t.is_signed() && y >= m / 2 {
                                    y -= m;
                                }
                                nv = Val::Int(y);
                            }
                        }
                    }
                }
                self.check_const_write(&place)?;
                let slot = self.get_mut_at(&place)?;
                *slot = nv.clone();
                // CE-6: an assignment to a union member makes it active.
                if let EK::Member(_, fid) = &l.k {
                    if let DK::Field(fi) = &self.p.decls[*fid].k {
                        if self.p.classes[fi.class].key == 2 && !place.array && !place.path.is_empty() {
                            let idx = fi.index;
                            let parent = Ptr { path: place.path[..place.path.len() - 1].to_vec(), ..place.clone() };
                            if let Ok(Val::Agg(items)) = self.get_mut_at(&parent) {
                                for (j, it) in items.iter_mut().enumerate() {
                                    if j != idx {
                                        *it = Val::Inactive;
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(nv)
            }
            EK::Cond(c, a, b) => {
                // COND-7: only the selected operand is evaluated.
                if self.eval_bool(c)? {
                    self.eval(a)
                } else {
                    self.eval(b)
                }
            }
            EK::Comma(a, b) => {
                self.eval(a)?;
                self.eval(b)
            }
            EK::Call(..) => self.eval_call(e, false),
            EK::InitList(items) => self.eval_list(items, &e.ty),
            EK::Throw(_) => nc("CE-8", "throw expression evaluated"),
            EK::Construct(cid, args) => {
                let Ty::Class(c) = e.ty.t else { return unsup("constructor of a non-class type") };
                self.construct(*cid, args, c)
            }
            EK::Unknown(why) => self.unknown(why),
            EK::Overloads(_) | EK::BoundMember(..) => unsup("overload set"),
        }
        .and_then(|v| {
            let _ = ty;
            Ok(v)
        })
    }

    fn eval_list(&mut self, items: &[Expr], ty: &QTy) -> R<Val> {
        match &ty.t {
            Ty::Array(el, Some(n)) => {
                let mut v = Vec::new();
                for it in items {
                    v.push(self.eval(it)?);
                }
                while (v.len() as u64) < *n {
                    v.push(self.zero(el));
                }
                Ok(Val::Agg(v))
            }
            Ty::Class(c) => {
                let fields = self.p.classes[*c].fields.clone();
                let mut v = Vec::new();
                for (i, &f) in fields.iter().enumerate() {
                    let DK::Field(fi) = &self.p.decls[f].k else { return unsup("class layout") };
                    let union = self.p.classes[*c].key == 2;
                    // UNI-4: with no value given, the member with a default
                    // member initializer is active, else the first.
                    let active = if items.is_empty() { fields.iter().position(|&g| matches!(&self.p.decls[g].k, DK::Field(x) if x.init.is_some())).unwrap_or(0) } else { 0 };
                    if union && i != active {
                        v.push(Val::Inactive);
                    } else if i < items.len() {
                        v.push(self.eval(&items[i])?);
                    } else if let Some(init) = &fi.init {
                        let init = init.clone();
                        v.push(self.eval(&init)?);
                    } else {
                        v.push(self.zero(&fi.ty));
                    }
                }
                Ok(Val::Agg(v))
            }
            _ => {
                if items.len() == 1 {
                    return self.eval(&items[0]);
                }
                unsup("initializer list")
            }
        }
    }

    pub fn zero(&self, t: &QTy) -> Val {
        match &t.t {
            Ty::Ptr(_) | Ty::NullPtr | Ty::MemPtr(..) => Val::Null,
            x if x.is_floating() => Val::Float(0.0),
            Ty::Array(el, Some(n)) => Val::Agg((0..*n).map(|_| self.zero(el)).collect()),
            Ty::Class(c) => {
                let mut v = Vec::new();
                let union = self.p.classes[*c].key == 2;
                for &f in &self.p.classes[*c].fields {
                    if let DK::Field(fi) = &self.p.decls[f].k {
                        if union && !v.is_empty() {
                            v.push(Val::Inactive);
                        } else {
                            v.push(self.zero(&fi.ty));
                        }
                    }
                }
                Val::Agg(v)
            }
            _ => Val::Int(0),
        }
    }

    fn default_val(&mut self, t: &QTy) -> R<Val> {
        // default-initialization: scalars have no value (INIT-1)
        Ok(match &t.t {
            Ty::Array(el, Some(n)) => {
                let mut v = Vec::new();
                for _ in 0..*n {
                    v.push(self.default_val(el)?);
                }
                Val::Agg(v)
            }
            Ty::Class(c) => {
                if self.p.class_uncertain(*c) {
                    return unsup("default constructor");
                }
                if !self.p.classes[*c].ctors.is_empty() {
                    return match self.p.default_ctor(*c) {
                        Some(cid) => self.construct(cid, &[], *c),
                        None => unsup("default constructor"),
                    };
                }
                let mut v = Vec::new();
                for &f in &self.p.classes[*c].fields.clone() {
                    if let DK::Field(fi) = &self.p.decls[f].k {
                        let fi = fi.clone();
                        match &fi.init {
                            Some(init) => v.push(self.eval(init)?),
                            None => v.push(self.default_val(&fi.ty)?),
                        }
                    }
                }
                Val::Agg(v)
            }
            _ => Val::Uninit,
        })
    }

    fn eval_conv(&mut self, k: Conv, a: &Expr, to: &QTy, line: u32) -> R<Val> {
        let _ = line;
        match k {
            Conv::LToR => {
                let p = self.lvalue(a)?;
                self.read(&p, a.ty.v)
            }
            Conv::ArrayDecay => {
                let p = self.lvalue(a)?;
                Ok(Val::Ptr(Ptr { obj: p.obj, path: Self::full_path(&p), off: 0, array: true }))
            }
            Conv::FuncDecay => match &a.k {
                EK::Func(f) => Ok(Val::FnPtr(*f)),
                _ => unsup("function decay"),
            },
            Conv::ToVoid => {
                self.eval(a)?;
                Ok(Val::Void)
            }
            Conv::ToBool => {
                let v = self.eval(a)?;
                Ok(Val::Int(match v {
                    Val::Int(x) => (x != 0) as i128,
                    Val::Float(f) => (f != 0.0) as i128,
                    Val::Null => 0,
                    Val::Ptr(_) | Val::FnPtr(_) => 1,
                    _ => return unsup("conversion to bool"),
                }))
            }
            Conv::NullPtr => {
                self.eval(a)?;
                Ok(Val::Null)
            }
            Conv::PtrConv | Conv::NoOp => {
                if a.cat != Cat::PR && !matches!(a.ty.t, Ty::Array(..)) {
                    return self.eval(a);
                }
                if let (Ty::Ptr(f), Ty::Ptr(t)) = (&a.ty.t, &to.t) {
                    if matches!((&f.t, &t.t), (Ty::Class(x), Ty::Class(y)) if x != y) {
                        return unsup("derived-to-base pointer conversion");
                    }
                }
                if let Ty::Array(..) = to.t {
                    return self.eval_array_init(a, to);
                }
                self.eval(a)
            }
            Conv::Arith => {
                let v = self.eval(a)?;
                self.convert_val(v, &a.ty.t, &to.t)
            }
            Conv::Bind => {
                let p = self.lvalue(a)?;
                self.read(&p, false)
            }
        }
    }

    fn eval_array_init(&mut self, a: &Expr, to: &QTy) -> R<Val> {
        // a char array from a string literal
        if let EK::Str(units) = &a.k {
            let Ty::Array(_, Some(n)) = &to.t else { return unsup("array init") };
            let mut v: Vec<Val> = units.iter().map(|&u| Val::Int(u as i128)).collect();
            while (v.len() as u64) < *n {
                v.push(Val::Int(0));
            }
            return Ok(Val::Agg(v));
        }
        unsup("array initialization")
    }

    /// Convert a scalar value between types (CONV-2, CONV-4, CONV-5).
    pub fn convert_val(&self, v: Val, from: &Ty, to: &Ty) -> R<Val> {
        let to_int = |t: &Ty| -> Option<Ty> {
            match t {
                Ty::Enum(e) => Some(self.p.enum_underlying(*e)),
                t if t.is_integral() => Some(t.clone()),
                _ => None,
            }
        };
        match v {
            Val::Int(x) => {
                if let Some(ti) = to_int(to) {
                    let r = wrap_int(x, &ti);
                    if let Ty::Enum(e) = to {
                        // ENUM-21: with a fixed underlying type the value
                        // converts to that type first.
                        let fixed = self.p.enums[*e].fixed.is_some() || self.p.enums[*e].scoped;
                        if !fixed && !matches!(from, Ty::Enum(f) if f == e) {
                            // ENUM-6, CAST-14: a value outside the range of
                            // an enumeration without a fixed type.
                            let (lo, hi) = self.p.enum_range(*e);
                            if x < lo || x > hi {
                                return nc("ENUM-6", format!("value {} is outside the range of the enumeration", x));
                            }
                            return Ok(Val::Int(x));
                        }
                    }
                    return Ok(Val::Int(r));
                }
                if to.is_floating() {
                    let f = x as f64;
                    return Ok(Val::Float(if *to == Ty::Float { f as f32 as f64 } else { f }));
                }
                Ok(Val::Int(x))
            }
            Val::Float(f) => {
                if let Some(ti) = to_int(to) {
                    if ti == Ty::Bool {
                        return Ok(Val::Int((f != 0.0) as i128));
                    }
                    let t = f.trunc();
                    if f.is_nan() || t < ti.int_min() as f64 || t > ti.int_max() as f64 {
                        // CONV-5
                        return nc("CONV-5", "floating value out of the range of the integer type");
                    }
                    return Ok(Val::Int(t as i128));
                }
                if *to == Ty::Float {
                    let r = f as f32;
                    if r.is_infinite() && f.is_finite() {
                        // CONV-11 is impl-defined; Clang and GCC reject in
                        // constant evaluation.
                        return nc("CONV-11", "floating conversion overflows the target type");
                    }
                    return Ok(Val::Float(r as f64));
                }
                Ok(Val::Float(f))
            }
            v => Ok(v),
        }
    }

    fn ptr_add(&self, p: &Ptr, n: i128, _for_index: bool) -> R<Ptr> {
        if p.array {
            let base = Ptr { array: false, off: 0, ..p.clone() };
            let len = match self.get_at(&base)? {
                Val::Agg(items) => items.len() as i128,
                _ => return unsup("pointer arithmetic"),
            };
            let no = p.off as i128 + n;
            if no < 0 || no > len {
                // ADD-10: outside the array and its one-past position.
                return nc("ADD-10", "pointer arithmetic outside the array");
            }
            Ok(Ptr { off: no as i64, ..p.clone() })
        } else {
            let no = p.off as i128 + n;
            if !(0..=1).contains(&no) {
                return nc("ADD-10", "pointer arithmetic outside an object (an array of one)");
            }
            Ok(Ptr { off: no as i64, ..p.clone() })
        }
    }

    fn eval_unary(&mut self, op: UOp, a: &Expr, e: &Expr) -> R<Val> {
        match op {
            UOp::Plus => self.eval(a),
            UOp::Neg => match self.eval(a)? {
                Val::Int(x) => {
                    let t = &e.ty.t;
                    if t.is_signed() {
                        if !fits(-x, t) {
                            // UN-4
                            return nc("UN-4", "negation overflows");
                        }
                        Ok(Val::Int(-x))
                    } else {
                        Ok(Val::Int(wrap_int(-x, t)))
                    }
                }
                Val::Float(f) => Ok(Val::Float(-f)),
                _ => unsup("negation"),
            },
            UOp::Not => Ok(Val::Int(!self.eval_bool(a)? as i128)),
            UOp::BitNot => {
                let x = self.eval_int(a)?;
                Ok(Val::Int(wrap_int(!x, &e.ty.t)))
            }
            UOp::Deref => {
                let p = self.lvalue(e)?;
                self.read(&p, e.ty.v)
            }
            UOp::AddrOf => {
                if let EK::Func(f) = &a.k {
                    return Ok(Val::FnPtr(*f));
                }
                let p = self.lvalue(a)?;
                Ok(Val::Ptr(p))
            }
            UOp::PreInc | UOp::PreDec | UOp::PostInc | UOp::PostDec => {
                let place = self.lvalue(a)?;
                let old = self.read(&place, a.ty.v)?;
                let delta = if matches!(op, UOp::PreInc | UOp::PostInc) { 1 } else { -1 };
                let nv = match &old {
                    Val::Int(x) => {
                        let t = &a.ty.t;
                        let r = x + delta;
                        if t.is_signed() && !fits(r, t) {
                            // UN-21
                            return nc("UN-21", "increment or decrement overflows");
                        }
                        Val::Int(wrap_int(r, t))
                    }
                    Val::Float(f) => Val::Float(f + delta as f64),
                    Val::Ptr(p) => Val::Ptr(self.ptr_add(p, delta, false)?),
                    Val::Null => return nc("ADD-10", "increment of a null pointer"),
                    _ => return unsup("increment"),
                };
                self.check_const_write(&place)?;
                *self.get_mut_at(&place)? = nv.clone();
                Ok(if matches!(op, UOp::PreInc | UOp::PreDec) { nv } else { old })
            }
        }
    }

    fn eval_binary(&mut self, op: BOp, a: &Expr, b: &Expr, rt: &QTy) -> R<Val> {
        if op == BOp::LAnd || op == BOp::LOr {
            // LOG-4, LOG-7: the right operand only if needed.
            let l = self.eval_bool(a)?;
            if (op == BOp::LAnd && !l) || (op == BOp::LOr && l) {
                return Ok(Val::Int(l as i128));
            }
            return Ok(Val::Int(self.eval_bool(b)? as i128));
        }
        let l = self.eval(a)?;
        let r = self.eval(b)?;
        let ct = a.ty.t.clone();
        match (&l, &r) {
            (Val::Ptr(_) | Val::Null, _) | (_, Val::Ptr(_) | Val::Null) | (Val::FnPtr(_), _) | (_, Val::FnPtr(_)) => {
                return self.ptr_binary(op, l, r, a, b, rt);
            }
            _ => {}
        }
        self.arith(op, l, r, &ct, rt.t.clone())
    }

    fn ptr_binary(&mut self, op: BOp, l: Val, r: Val, a: &Expr, _b: &Expr, rt: &QTy) -> R<Val> {
        match op {
            BOp::Add | BOp::Sub => match (&l, &r) {
                (Val::Ptr(p), Val::Int(n)) => {
                    let n = if op == BOp::Sub { -n } else { *n };
                    if let Ty::Ptr(pt) = &a.ty.t {
                        let _ = pt;
                    }
                    Ok(Val::Ptr(self.ptr_add(p, n, false)?))
                }
                (Val::Null, Val::Int(n)) => {
                    if *n == 0 {
                        Ok(Val::Null)
                    } else {
                        nc("ADD-10", "arithmetic on a null pointer")
                    }
                }
                (Val::Ptr(p), Val::Ptr(q)) => {
                    if p.obj != q.obj || p.path != q.path || p.array != q.array {
                        // ADD-11
                        return nc("ADD-11", "subtraction of pointers into different arrays");
                    }
                    Ok(Val::Int(p.off as i128 - q.off as i128))
                }
                (Val::Null, Val::Null) => Ok(Val::Int(0)),
                _ => unsup("pointer arithmetic"),
            },
            BOp::Eq | BOp::Ne => {
                let eq = match (&l, &r) {
                    (Val::Null, Val::Null) => true,
                    (Val::Null, _) | (_, Val::Null) => false,
                    (Val::FnPtr(x), Val::FnPtr(y)) => x == y,
                    (Val::Ptr(p), Val::Ptr(q)) => {
                        if self.objs[p.obj].string && self.objs[q.obj].string && p.obj != q.obj {
                            // CE-13
                            return nc("CE-13", "comparison of the addresses of string literals");
                        }
                        if p.obj != q.obj {
                            // one past the end of one object vs another: CMP-20
                            let past = |pp: &Ptr| -> bool {
                                if pp.array {
                                    matches!(self.get_at(&Ptr { array: false, off: 0, ..pp.clone() }), Ok(Val::Agg(items)) if pp.off as usize == items.len())
                                } else {
                                    pp.off == 1
                                }
                            };
                            if past(p) || past(q) {
                                return nc("CMP-20", "comparison of a past-the-end pointer with a pointer to another object");
                            }
                            false
                        } else {
                            Self::full_path(p) == Self::full_path(q) && p.off == q.off
                        }
                    }
                    _ => return unsup("pointer comparison"),
                };
                Ok(Val::Int((if op == BOp::Eq { eq } else { !eq }) as i128))
            }
            BOp::Lt | BOp::Gt | BOp::Le | BOp::Ge => match (&l, &r) {
                (Val::Ptr(p), Val::Ptr(q)) => {
                    if p.obj != q.obj {
                        // CE-15
                        return nc("CE-15", "relational comparison of pointers to unrelated objects");
                    }
                    // CMP-7: subobjects of one object compare in order.
                    let mut x: Vec<i64> = p.path.iter().map(|&v| v as i64).collect();
                    x.push(if p.array { p.off } else { p.off * i64::MAX / 2 });
                    let mut y: Vec<i64> = q.path.iter().map(|&v| v as i64).collect();
                    y.push(if q.array { q.off } else { q.off * i64::MAX / 2 });
                    if !p.array && !q.array && p.path.len() != q.path.len() && p.off == 0 && q.off == 0 {
                        return unsup("comparison of a subobject pointer with its enclosing object");
                    }
                    let res = match op {
                        BOp::Lt => x < y,
                        BOp::Gt => x > y,
                        BOp::Le => x <= y,
                        _ => x >= y,
                    };
                    Ok(Val::Int(res as i128))
                }
                _ => nc("CE-15", "relational comparison involving a null or function pointer"),
            },
            _ => {
                let _ = rt;
                unsup("pointer operation")
            }
        }
    }

    /// Arithmetic in the computation type `ct`, result type `rt`.
    fn arith(&self, op: BOp, l: Val, r: Val, ct: &Ty, rt: Ty) -> R<Val> {
        match (l, r) {
            (Val::Int(x), Val::Int(y)) => {
                let t = if matches!(op, BOp::Shl | BOp::Shr) {
                    rt.clone()
                } else if let Ty::Enum(e) = ct {
                    self.p.enum_underlying(*e)
                } else {
                    ct.clone()
                };
                let t = if t.is_integral() { t } else { rt.clone() };
                let signed = t.is_signed();
                let check = |v: i128, rule: &'static str| -> R<Val> {
                    if signed {
                        if !fits(v, &t) {
                            return nc(rule, format!("signed overflow: {} does not fit in {}", v, base_name(&t)));
                        }
                        Ok(Val::Int(v))
                    } else {
                        Ok(Val::Int(wrap_int(v, &t)))
                    }
                };
                match op {
                    BOp::Add => check(x + y, "ADD-9"),
                    BOp::Sub => check(x - y, "ADD-9"),
                    BOp::Mul => {
                        let v = x.checked_mul(y).unwrap_or(i128::MAX);
                        check(v, "MUL-10")
                    }
                    BOp::Div | BOp::Rem => {
                        if y == 0 {
                            // MUL-9
                            return nc("MUL-9", "division by zero");
                        }
                        let q = x / y; // rounds toward zero (MUL-6)
                        if signed && !fits(q, &t) {
                            return nc("MUL-10", "quotient does not fit in the type");
                        }
                        if op == BOp::Div {
                            check(q, "MUL-10")
                        } else {
                            Ok(Val::Int(wrap_int(x % y, &t)))
                        }
                    }
                    BOp::Shl | BOp::Shr => {
                        let w = t.int_bits() as i128;
                        if y < 0 || y >= w {
                            // SHF-7
                            return nc("SHF-7", "shift count negative or not less than the width");
                        }
                        if op == BOp::Shl {
                            // SHF-5: modulo 2^N, also for negative values
                            Ok(Val::Int(wrap_int(x.wrapping_shl(y as u32), &t)))
                        } else {
                            Ok(Val::Int(x >> y))
                        }
                    }
                    BOp::BitAnd => Ok(Val::Int(wrap_int(x & y, &t))),
                    BOp::BitOr => Ok(Val::Int(wrap_int(x | y, &t))),
                    BOp::BitXor => Ok(Val::Int(wrap_int(x ^ y, &t))),
                    BOp::Lt => Ok(Val::Int((x < y) as i128)),
                    BOp::Gt => Ok(Val::Int((x > y) as i128)),
                    BOp::Le => Ok(Val::Int((x <= y) as i128)),
                    BOp::Ge => Ok(Val::Int((x >= y) as i128)),
                    BOp::Eq => Ok(Val::Int((x == y) as i128)),
                    BOp::Ne => Ok(Val::Int((x != y) as i128)),
                    _ => unsup("operator"),
                }
            }
            (Val::Float(x), Val::Float(y)) => {
                let is_f32 = *ct == Ty::Float;
                let round = |v: f64| if is_f32 { v as f32 as f64 } else { v };
                let maxv = if is_f32 { f32::MAX as f64 } else { f64::MAX };
                let fin = |v: f64, rule: &'static str| -> R<Val> {
                    let r = round(v);
                    if (r.is_infinite() || v.abs() > maxv) && x.is_finite() && y.is_finite() {
                        return nc(rule, "floating result out of the range of its type");
                    }
                    Ok(Val::Float(r))
                };
                match op {
                    BOp::Add => fin(x + y, "ADD-17"),
                    BOp::Sub => fin(x - y, "ADD-17"),
                    BOp::Mul => fin(x * y, "MUL-12"),
                    BOp::Div => {
                        if y == 0.0 {
                            // MUL-11
                            return nc("MUL-11", "floating division by zero");
                        }
                        fin(x / y, "MUL-12")
                    }
                    BOp::Lt => Ok(Val::Int((x < y) as i128)),
                    BOp::Gt => Ok(Val::Int((x > y) as i128)),
                    BOp::Le => Ok(Val::Int((x <= y) as i128)),
                    BOp::Ge => Ok(Val::Int((x >= y) as i128)),
                    BOp::Eq => Ok(Val::Int((x == y) as i128)),
                    BOp::Ne => Ok(Val::Int((x != y) as i128)),
                    _ => unsup("floating operator"),
                }
            }
            (Val::Int(x), Val::Float(y)) => self.arith(op, Val::Float(x as f64), Val::Float(y), ct, rt),
            (Val::Float(x), Val::Int(y)) => self.arith(op, Val::Float(x), Val::Float(y as f64), ct, rt),
            _ => unsup("operands of an arithmetic operator"),
        }
    }

    // ---- calls ----

    fn eval_call(&mut self, e: &Expr, want_ref: bool) -> R<Val> {
        let EK::Call(callee, args, fid) = &e.k else { unreachable!() };
        let fid = match fid {
            Some(f) => *f,
            None => match self.eval(callee)? {
                Val::FnPtr(f) => f,
                Val::Null => return nc("PFX-8", "call through a null function pointer"),
                _ => return unsup("indirect call"),
            },
        };
        let Some(f) = self.p.func(fid) else { return unsup("call of a non-function") };
        let f = f.clone();
        let name = self.p.decls[fid].name.clone();
        if !f.constexpr && !f.consteval {
            // CE-2: only constexpr and consteval functions.
            return nc("CE-2", format!("call of non-constexpr function '{}'", name));
        }
        if f.template {
            return unsup("call of a function template");
        }
        if f.opaque {
            return unsup(format!("function '{}' has a body the front end did not parse", name));
        }
        if f.defaulted || f.deleted {
            return unsup("defaulted function");
        }
        let Some(body) = f.body.clone() else {
            if f.defined {
                return unsup(format!("body of '{}' is not available yet", name));
            }
            // CE-16: a declared but not yet defined function.
            return nc("CE-16", format!("'{}' is declared but not defined at this point", name));
        };
        if f.kind != FnKind::Normal && f.kind != FnKind::Operator {
            return unsup("special member function call");
        }
        self.depth += 1;
        if self.depth > DEPTH_LIMIT {
            self.depth -= 1;
            return nc("CE-10", "constant evaluation recursion limit exceeded");
        }
        // this
        let this = if f.class.is_some() && !f.is_static {
            if callee.cat == Cat::PR {
                let v = self.eval(callee)?;
                let o = self.new_obj(v, true);
                Some(Ptr { obj: o, path: vec![], off: 0, array: false })
            } else {
                Some(self.lvalue(callee)?)
            }
        } else {
            None
        };
        let (frame, refs) = match self.bind_args(&f, args) {
            Ok(x) => x,
            Err(e) => {
                self.depth -= 1;
                return Err(e);
            }
        };
        self.frames.push(frame);
        self.refs.push(refs);
        self.this.push(this);
        self.local_mark.push(self.locals_created.len());
        let save = self.in_consteval_fn;
        self.in_consteval_fn = f.consteval;
        self.ret_ref.push(f.fty.ret.t.is_ref() || f.fty.ret.is_unknown());
        let r = self.exec(&body, &mut None);
        self.ret_ref.pop();
        self.in_consteval_fn = save;
        // The function's automatic objects end with the call; a pointer
        // to one of them dangles (UN-7, CE-9).
        let mut ended: Vec<usize> = self.frames.last().unwrap().values().copied().collect();
        ended.extend(self.locals_created.drain(self.local_mark.pop().unwrap_or(0)..));
        for o in ended {
            if let Some(ob) = self.objs.get_mut(o) {
                ob.alive = false;
            }
        }
        self.frames.pop();
        self.refs.pop();
        self.this.pop();
        self.depth -= 1;
        match r? {
            Flow::Return(Some(v)) => Ok(v),
            Flow::Return(None) | Flow::Normal => {
                if f.fty.ret.t == Ty::Void {
                    Ok(Val::Void)
                } else {
                    // FN-3: flowing off the end of a value-returning function.
                    nc("FN-3", format!("control reaches the end of '{}' without a return", name))
                }
            }
            _ => unsup("break or continue outside a loop"),
        }
        .map(|v| {
            let _ = want_ref;
            v
        })
    }

    /// Bind the arguments of a call to the parameters, in the caller's
    /// frame, with default arguments for the rest.
    fn bind_args(&mut self, f: &FuncInfo, args: &[Expr]) -> R<(HashMap<DeclId, usize>, HashMap<DeclId, Ptr>)> {
        let mut frame = HashMap::new();
        let mut refs = HashMap::new();
        for (i, a) in args.iter().enumerate() {
            let Some(&pid) = f.params.get(i) else { continue };
            let pty = self.p.var(pid).map(|v| v.ty.clone()).unwrap_or(QTy::unknown());
            if pty.t.is_ref() {
                let pl = self.lvalue(&strip_bind(a))?;
                refs.insert(pid, pl);
            } else {
                let v = self.eval(a)?;
                let o = self.new_obj(v, true);
                frame.insert(pid, o);
            }
        }
        for i in args.len()..f.params.len() {
            let pid = f.params[i];
            let Some(mut v) = self.p.var(pid).cloned() else { continue };
            if v.init.is_none() {
                v.init = f.default_exprs.get(i).cloned().flatten();
            }
            match &v.init {
                Some(d) => {
                    if v.ty.t.is_ref() {
                        let pl = self.lvalue(&strip_bind(d))?;
                        refs.insert(pid, pl);
                    } else {
                        let val = self.eval(d)?;
                        let o = self.new_obj(val, true);
                        frame.insert(pid, o);
                    }
                }
                None => return unsup("missing argument"),
            }
        }
        Ok((frame, refs))
    }

    /// Run a constructor: initialize each member in order from its
    /// mem-initializer, its default member initializer, or by
    /// default-initialization, then run the body.
    fn construct(&mut self, cid: DeclId, args: &[Expr], c: usize) -> R<Val> {
        let Some(f) = self.p.func(cid) else { return unsup("constructor") };
        let f = f.clone();
        let cname = self.p.classes[c].name.clone();
        if !f.constexpr && !f.consteval && !f.defaulted {
            // CE-2: only constexpr constructors.
            return nc("CE-2", format!("call of non-constexpr constructor of '{}'", cname));
        }
        if self.p.class_uncertain(c) || !self.p.classes[c].bases.is_empty() {
            return unsup("constructor of a class with bases");
        }
        // The evaluator does not run destructors, so it gives up on a
        // class whose destructor does something.
        if let Some(d) = self.p.classes[c].dtor {
            let trivial = self.p.func(d).is_some_and(|df| df.defaulted || matches!(df.body.as_deref(), Some(Stmt::Block(v)) if v.is_empty()));
            if !trivial {
                return unsup("destructor evaluation");
            }
        }
        if f.opaque || f.mem_inits.iter().any(|m| m.field.is_none() || m.init.is_none()) {
            return unsup("constructor with an initializer the front end does not model");
        }
        let body = if f.defaulted {
            None
        } else {
            match f.body.clone() {
                Some(b) => Some(b),
                None if f.defined => return unsup(format!("body of the constructor of '{}' is not available yet", cname)),
                None => return nc("CE-16", format!("constructor of '{}' is declared but not defined at this point", cname)),
            }
        };
        let fields: Vec<DeclId> = self.p.classes[c].fields.clone();
        if fields.iter().any(|&fd| !matches!(self.p.decls[fd].k, DK::Field(_))) {
            return unsup("class layout");
        }
        let union = self.p.classes[c].key == 2;
        self.depth += 1;
        if self.depth > DEPTH_LIMIT {
            self.depth -= 1;
            return nc("CE-10", "constant evaluation recursion limit exceeded");
        }
        let o = self.new_obj(Val::Agg(vec![Val::Uninit; fields.len()]), true);
        let this = Ptr { obj: o, path: vec![], off: 0, array: false };
        let (frame, refs) = match self.bind_args(&f, args) {
            Ok(x) => x,
            Err(e) => {
                self.depth -= 1;
                return Err(e);
            }
        };
        self.frames.push(frame);
        self.refs.push(refs);
        self.this.push(Some(this));
        self.local_mark.push(self.locals_created.len());
        let r = (|| -> R<Flow> {
            for (i, &fd) in fields.iter().enumerate() {
                let DK::Field(fi) = &self.p.decls[fd].k else { unreachable!() };
                let fi = fi.clone();
                if fi.ty.t.is_ref() || fi.bits.is_some() {
                    return unsup("reference or bit-field member in a constructor");
                }
                let mi = f.mem_inits.iter().find(|m| m.field == Some(fd)).and_then(|m| m.init.clone());
                let v = if let Some(init) = mi.or(fi.init.clone()) {
                    self.rvalue_init(&init, &fi.ty)?
                } else if union {
                    Val::Inactive
                } else {
                    self.default_val(&fi.ty)?
                };
                if let Val::Agg(items) = &mut self.objs[o].val {
                    items[i] = v;
                }
            }
            match &body {
                Some(b) => {
                    self.ret_ref.push(false);
                    let r = self.exec(b, &mut None);
                    self.ret_ref.pop();
                    r
                }
                None => Ok(Flow::Normal),
            }
        })();
        let mut ended: Vec<usize> = self.frames.last().unwrap().values().copied().collect();
        ended.extend(self.locals_created.drain(self.local_mark.pop().unwrap_or(0)..));
        for x in ended {
            if let Some(ob) = self.objs.get_mut(x) {
                ob.alive = false;
            }
        }
        self.frames.pop();
        self.refs.pop();
        self.this.pop();
        self.depth -= 1;
        r?;
        Ok(self.objs[o].val.clone())
    }

    /// Initialize a variable's value from its converted initializer.
    pub fn rvalue_init(&mut self, init: &Expr, ty: &QTy) -> R<Val> {
        if let Ty::Array(..) = ty.t {
            if let EK::Conv(Conv::NoOp, a) = &init.k {
                return self.eval_array_init(a, ty);
            }
        }
        self.eval(init)
    }

    // ---- statements ----

    fn exec(&mut self, s: &Stmt, seek: &mut Option<Seek>) -> R<Flow> {
        self.step()?;
        if seek.is_some() {
            return self.exec_seek(s, seek);
        }
        match s {
            Stmt::Null | Stmt::Fallthrough(_) => Ok(Flow::Normal),
            // ATTR-10: an assumption that is false is undefined.
            Stmt::Assume(e) => match self.eval_bool(e) {
                Ok(false) => nc("ATTR-10", "assumption is false"),
                _ => Ok(Flow::Normal),
            },
            Stmt::Expr(e) => {
                self.eval(e)?;
                Ok(Flow::Normal)
            }
            Stmt::Decl(ids) => {
                for &id in ids {
                    self.exec_decl(id)?;
                }
                Ok(Flow::Normal)
            }
            Stmt::Block(items) => self.exec_block(items, seek),
            Stmt::If { init, cond, then, els, kind } => {
                if let Some(i) = init {
                    if let f @ (Flow::Return(_) | Flow::Break | Flow::Continue) = self.exec(i, seek)? {
                        return Ok(f);
                    }
                }
                let taken = match kind {
                    // IF-18: the then branch runs in constant evaluation.
                    IfKind::Consteval(neg) => !*neg,
                    _ => self.eval_cond(cond.as_ref().unwrap())?,
                };
                if taken {
                    self.exec(then, seek)
                } else if let Some(e) = els {
                    self.exec(e, seek)
                } else {
                    Ok(Flow::Normal)
                }
            }
            Stmt::While { cond, body } => loop {
                if !self.eval_cond(cond)? {
                    return Ok(Flow::Normal);
                }
                match self.exec(body, seek)? {
                    Flow::Break => return Ok(Flow::Normal),
                    f @ Flow::Return(_) => return Ok(f),
                    _ => {}
                }
            },
            Stmt::Do { body, cond } => loop {
                match self.exec(body, seek)? {
                    Flow::Break => return Ok(Flow::Normal),
                    f @ Flow::Return(_) => return Ok(f),
                    _ => {}
                }
                if !self.eval_bool(cond)? {
                    return Ok(Flow::Normal);
                }
            },
            Stmt::For { init, cond, inc, body } => {
                if let Some(i) = init {
                    self.exec(i, seek)?;
                }
                loop {
                    if let Some(c) = cond {
                        if !self.eval_cond(c)? {
                            return Ok(Flow::Normal);
                        }
                    }
                    match self.exec(body, seek)? {
                        Flow::Break => return Ok(Flow::Normal),
                        f @ Flow::Return(_) => return Ok(f),
                        _ => {}
                    }
                    if let Some(i) = inc {
                        self.eval(i)?;
                    }
                }
            }
            Stmt::Switch { init, cond, body } => {
                if let Some(i) = init {
                    self.exec(i, seek)?;
                }
                let v = match cond {
                    CondK::Expr(e) => self.eval_int(e)?,
                    CondK::Decl(id, e) => {
                        self.exec_decl(*id)?;
                        self.eval_int(e)?
                    }
                };
                let has_case = collect_cases(body).contains(&Some(v));
                let target = if has_case {
                    Some(Seek::Case(v))
                } else if collect_cases(body).contains(&None) {
                    Some(Seek::Default)
                } else {
                    return Ok(Flow::Normal);
                };
                let mut sk = target;
                match self.exec(body, &mut sk)? {
                    Flow::Break => Ok(Flow::Normal),
                    f => Ok(f),
                }
            }
            Stmt::Case(_, s) | Stmt::Default(s) | Stmt::Label(_, s) => self.exec(s, seek),
            Stmt::Break => Ok(Flow::Break),
            Stmt::Continue => Ok(Flow::Continue),
            Stmt::Return(e) => match e {
                None => Ok(Flow::Return(None)),
                Some(e) => {
                    let ret_ref = self.ret_ref.last().copied().unwrap_or(true);
                    if e.cat != Cat::PR && !matches!(e.k, EK::Conv(..)) && ret_ref {
                        // returning a reference
                        let p = self.lvalue(e)?;
                        return Ok(Flow::Return(Some(Val::Ptr(p))));
                    }
                    let v = self.eval(e)?;
                    Ok(Flow::Return(Some(v)))
                }
            },
            // CE-20: a goto in constant evaluation.
            Stmt::Goto(_) => nc("CE-20", "goto evaluated in constant evaluation"),
            Stmt::Unsupported(m) => unsup(m.clone()),
        }
    }

    fn exec_block(&mut self, items: &[Stmt], seek: &mut Option<Seek>) -> R<Flow> {
        for it in items {
            let f = self.exec(it, seek)?;
            if !matches!(f, Flow::Normal) {
                return Ok(f);
            }
        }
        Ok(Flow::Normal)
    }

    /// Skip statements until the case label the switch looks for.
    fn exec_seek(&mut self, s: &Stmt, seek: &mut Option<Seek>) -> R<Flow> {
        match s {
            Stmt::Case(v, inner) => {
                if *seek == Some(Seek::Case(*v)) {
                    *seek = None;
                    return self.exec(inner, seek);
                }
                self.exec_seek(inner, seek)
            }
            Stmt::Default(inner) => {
                if *seek == Some(Seek::Default) {
                    *seek = None;
                    return self.exec(inner, seek);
                }
                self.exec_seek(inner, seek)
            }
            Stmt::Label(_, inner) => self.exec_seek(inner, seek),
            Stmt::Block(items) => {
                for it in items {
                    let f = self.exec(it, seek)?;
                    if !matches!(f, Flow::Normal) {
                        return Ok(f);
                    }
                }
                Ok(Flow::Normal)
            }
            Stmt::Decl(ids) => {
                // skipped declarations: the variables exist without a value
                for &id in ids {
                    if let Some(v) = self.p.var(id) {
                        if v.storage == Storage::Auto && !v.ty.t.is_ref() {
                            let val = self.default_val(&v.ty.clone())?;
                            let o = self.new_obj(val, true);
                            self.frames.last_mut().unwrap().insert(id, o);
                        }
                    }
                }
                Ok(Flow::Normal)
            }
            Stmt::If { .. } | Stmt::While { .. } | Stmt::For { .. } | Stmt::Do { .. } => {
                if collect_cases_any(s) {
                    return unsup("case label inside a nested statement");
                }
                Ok(Flow::Normal)
            }
            _ => Ok(Flow::Normal),
        }
    }

    fn eval_cond(&mut self, c: &CondK) -> R<bool> {
        match c {
            CondK::Expr(e) => self.eval_bool(e),
            CondK::Decl(id, e) => {
                self.exec_decl(*id)?;
                self.eval_bool(e)
            }
        }
    }

    fn exec_decl(&mut self, id: DeclId) -> R<()> {
        let Some(v) = self.p.var(id) else {
            return Ok(());
        };
        let v = v.clone();
        if v.dependent {
            return Ok(()); // structured bindings: reads are unsupported
        }
        if v.storage != Storage::Auto {
            if v.constexpr {
                return Ok(());
            }
            // CE-20: a static or thread_local variable.
            return nc("CE-20", "definition of a static or thread_local variable in constant evaluation");
        }
        if self.p.literal_type(&v.ty) == Some(false) {
            // CE-20: no variable of a type that is not literal.
            return nc("CE-20", "definition of a variable of a type that is not a literal type");
        }
        if v.ty.t.is_ref() {
            let Some(init) = &v.init else { return unsup("reference without initializer") };
            let pl = self.lvalue(&strip_bind(init))?;
            self.refs.last_mut().unwrap().insert(id, pl);
            return Ok(());
        }
        let val = match &v.init {
            Some(init) => {
                let init = init.clone();
                // NAM-1: the variable is in scope in its own initializer,
                // without a value yet (INIT-1).
                let o = self.new_obj(Val::Uninit, true);
                self.frames.last_mut().unwrap().insert(id, o);
                let val = self.rvalue_init(&init, &v.ty)?;
                self.objs[o].val = val;
                self.objs[o].volatile = v.ty.v;
                // A class can have mutable members, so only scalars and arrays
                // of scalars are marked.
                self.objs[o].konst = v.ty.c && crate::decl2::elem_class(&v.ty.t).is_none();
                return Ok(());
            }
            None => self.default_val(&v.ty)?,
        };
        let o = self.new_obj(val, true);
        self.objs[o].volatile = v.ty.v;
        self.frames.last_mut().unwrap().insert(id, o);
        Ok(())
    }

    /// CE-14: the result of a constant expression must not point to an
    /// automatic or temporary object.
    pub fn check_result(&self, v: &Val) -> R<()> {
        match v {
            Val::Ptr(p) => {
                if !self.objs[p.obj].is_static {
                    return nc("CE-14", "result points to an object that is not static");
                }
                Ok(())
            }
            Val::Agg(items) => {
                for i in items {
                    self.check_result(i)?;
                }
                Ok(())
            }
            // CE-21: every scalar part of the result has a value.
            Val::Uninit => nc("CE-21", "result has a scalar part with no value"),
            _ => Ok(()),
        }
    }
}

/// A value with a scalar part that has no value (CE-21, CE-22).
fn has_unset(v: &Val) -> bool {
    match v {
        Val::Uninit => true,
        Val::Agg(items) => items.iter().any(has_unset),
        _ => false,
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Seek {
    Case(i128),
    Default,
}

fn collect_cases(s: &Stmt) -> Vec<Option<i128>> {
    let mut v = Vec::new();
    fn walk(s: &Stmt, v: &mut Vec<Option<i128>>) {
        match s {
            Stmt::Case(x, i) => {
                v.push(Some(*x));
                walk(i, v);
            }
            Stmt::Default(i) => {
                v.push(None);
                walk(i, v);
            }
            Stmt::Label(_, i) => walk(i, v),
            Stmt::Block(items) => {
                for it in items {
                    walk(it, v);
                }
            }
            Stmt::If { then, els, .. } => {
                walk(then, v);
                if let Some(e) = els {
                    walk(e, v);
                }
            }
            Stmt::While { body, .. } | Stmt::Do { body, .. } | Stmt::For { body, .. } => walk(body, v),
            _ => {}
        }
    }
    walk(s, &mut v);
    v
}

fn collect_cases_any(s: &Stmt) -> bool {
    !collect_cases(s).is_empty()
}

pub fn strip_bind(e: &Expr) -> Expr {
    match &e.k {
        EK::Conv(Conv::Bind, a) => (**a).clone(),
        _ => e.clone(),
    }
}

// ---- interface used by the parser ----

impl P {
    pub fn const_eval(&self, e: &Expr) -> R<Val> {
        let mut ev = Eval::new(self);
        let v = ev.eval(e)?;
        Ok(v)
    }

    pub fn const_int(&self, e: &Expr) -> R<i128> {
        if e.ty.is_unknown() {
            if let EK::Unknown(w) = &e.k {
                return unsup(w.clone());
            }
        }
        let mut ev = Eval::new(self);
        let e2 = if e.is_glvalue() && !matches!(e.ty.t, Ty::Array(..)) { Expr::new(EK::Conv(Conv::LToR, Box::new(e.clone())), e.ty.unqual(), Cat::PR, e.line) } else { e.clone() };
        match ev.eval(&e2)? {
            Val::Int(v) => Ok(v),
            Val::Float(_) => nc("CE-25", "a floating value where an integer constant is needed"),
            _ => unsup("non-integer constant"),
        }
    }

    pub fn const_int_quiet(&self, e: &Expr) -> R<i128> {
        self.const_int(e)
    }

    pub fn const_float_quiet(&self, e: &Expr) -> Option<f64> {
        let mut ev = Eval::new(self);
        let e2 = if e.is_glvalue() { Expr::new(EK::Conv(Conv::LToR, Box::new(e.clone())), e.ty.unqual(), Cat::PR, e.line) } else { e.clone() };
        match ev.eval(&e2) {
            Ok(Val::Float(f)) => Some(f),
            _ => None,
        }
    }

    pub fn const_eval_bool(&self, e: &Expr) -> R<bool> {
        let mut ev = Eval::new(self);
        ev.eval_bool(e)
    }

    /// Check that an expression (a consteval call) is constant.
    pub fn const_check(&self, e: &Expr) -> R<()> {
        let mut ev = Eval::new(self);
        let v = ev.eval(e)?;
        ev.check_result(&v)
    }

    /// Evaluate the initializer of a variable that needs a constant
    /// initializer (CX-1, CINIT-1, CLS-20).
    pub fn const_eval_init(&self, id: DeclId) -> R<()> {
        let Some(v) = self.var(id) else { return Ok(()) };
        let Some(init) = v.init.clone() else {
            // Default-initialization of a class object: CX-1 and CE-21 need
            // every scalar part to have a value.
            if matches!(v.ty.t, Ty::Class(_)) {
                let mut ev = Eval::new(self);
                let val = ev.default_val(&v.ty)?;
                return ev.check_result(&val);
            }
            return Ok(());
        };
        let mut ev = Eval::new(self);
        if v.ty.t.is_ref() {
            let p = ev.lvalue(&strip_bind(&init))?;
            // CX-2: a constexpr reference refers to a static object.
            if !ev.objs[p.obj].is_static {
                return nc("CX-2", "constexpr reference to an object without static storage");
            }
            return Ok(());
        }
        let val = ev.rvalue_init(&init, &v.ty)?;
        if let Val::Ptr(p) = &val {
            if !ev.objs[p.obj].is_static {
                return nc("CX-2", "constexpr pointer to an object without static storage");
            }
        }
        ev.check_result(&val)
    }

    /// A class with a copy or move constructor that is not defaulted.
    pub fn has_user_copy(&self, c: usize) -> bool {
        self.classes[c].ctors.iter().any(|&i| {
            self.func(i).is_some_and(|f| {
                !f.defaulted
                    && !f.params.is_empty()
                    && self.var(f.params[0]).is_some_and(|v| match &v.ty.t {
                        Ty::LRef(x) | Ty::RRef(x) => x.t == Ty::Class(c),
                        _ => false,
                    })
            })
        })
    }

    /// CE-23: whether a type is a literal type. None when the front end
    /// does not model the type well enough to say.
    pub fn literal_type(&self, t: &QTy) -> Option<bool> {
        match &t.t {
            Ty::Unknown => None,
            Ty::Array(el, _) => self.literal_type(el),
            Ty::Class(c) => {
                let c = *c;
                let ci = &self.classes[c];
                if self.class_uncertain(c) || !ci.complete {
                    return None;
                }
                // The destructor is implicit, defaulted, or constexpr.
                if let Some(d) = ci.dtor {
                    let f = self.func(d)?;
                    if !f.constexpr && !f.defaulted {
                        return Some(false);
                    }
                }
                // Bases and members have literal types that are not
                // volatile; a union needs one member that is not volatile.
                for b in &ci.bases {
                    let bc = b.class?;
                    if self.literal_type(&QTy::new(Ty::Class(bc)))? == false {
                        return Some(false);
                    }
                }
                let mut any_nonvolatile = false;
                let mut any_volatile = false;
                for &fd in &ci.fields {
                    let DK::Field(fi) = &self.decls[fd].k else { continue };
                    if self.literal_type(&fi.ty)? == false {
                        return Some(false);
                    }
                    let vol = fi.ty.v || matches!(&fi.ty.t, Ty::Array(el, _) if el.v);
                    any_volatile |= vol;
                    any_nonvolatile |= !vol;
                }
                if any_volatile && (ci.key != 2 || !any_nonvolatile) {
                    return Some(false);
                }
                Some(true)
            }
            _ => Some(true),
        }
    }

    pub fn eval_bool_const(&mut self, e: &Expr, line: u32, rule: &'static str) -> Option<bool> {
        if self.in_template() || e.ty.is_unknown() {
            return None;
        }
        let b = self.contextual_bool(e.clone(), rule);
        match self.const_eval_bool(&b) {
            Ok(v) => Some(v),
            Err(CeErr::NotConst(r, m)) => {
                self.error_because(line, rule, r, format!("not a constant expression ({}: {})", r, m));
                None
            }
            Err(CeErr::Unsupported(m)) => {
                self.unsupported(line, m);
                None
            }
        }
    }
}

pub fn _unused(_: Rc<()>) {}
