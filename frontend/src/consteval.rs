// Constant evaluation (specification/foundations/constant-evaluation.md) as a client
// of the interpreter (decision 0048). It runs the abstract machine of
// src/interp with `constant` set. The machine then stops at each
// operation that CE-2 to CE-25 forbid (`Stop::Limit`), and at each
// operation that an `undefined` rule names (CE-9, TR-8,
// `Stop::Undefined`). Both make the evaluation not constant.

use crate::ast::*;
use crate::interp::{has_unset, strip_bind, Heap, Machine, Ptr, Stop};
pub use crate::interp::Val;
use crate::parser::*;
use crate::types::*;

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

impl From<Stop> for CeErr {
    fn from(s: Stop) -> CeErr {
        match s {
            Stop::Undefined(r, m) | Stop::Limit(r, m) => CeErr::NotConst(r, m),
            Stop::Unsupported(m) => CeErr::Unsupported(m),
            // Constant evaluation stops at a throw (CE-8), so neither
            // reaches here.
            Stop::Throw(_) => CeErr::NotConst("CE-8", "an exception was thrown".into()),
            Stop::Terminate(r, m) => CeErr::NotConst(r, m),
        }
    }
}

/// CE-14, CE-21: the result of a constant expression must not point to
/// an automatic or temporary object, and every scalar part has a value.
fn check_result(m: &Machine, v: &Val) -> R<()> {
    match v {
        Val::Ptr(p) => {
            if !m.objs[p.obj].is_static {
                return nc("CE-14", "result points to an object that is not static");
            }
            Ok(())
        }
        Val::Agg(items) => {
            for i in items {
                check_result(m, i)?;
            }
            Ok(())
        }
        Val::Uninit => nc("CE-21", "result has a scalar part with no value"),
        _ => Ok(()),
    }
}

/// CE-7: storage from `new` must be freed in the same evaluation.
fn no_leak(m: &Machine) -> R<()> {
    if m.objs.iter().any(|o| o.heap != Heap::No && o.alive) {
        return nc("CE-7", "storage from new is not freed in the same constant evaluation");
    }
    Ok(())
}

fn as_lvalue(e: &Expr) -> Expr {
    if e.is_glvalue() && !matches!(e.ty.t, Ty::Array(..)) {
        Expr::new(EK::Conv(Conv::LToR, Box::new(e.clone())), e.ty.unqual(), Cat::PR, e.line)
    } else {
        e.clone()
    }
}

// ---- interface used by the parser ----

impl P {
    pub fn const_eval(&self, e: &Expr) -> R<Val> {
        let mut m = Machine::new(self, true);
        Ok(m.full(|m| m.eval(e))?)
    }

    /// The entity that a constant pointer or reference names, for a
    /// template argument (TARG-14): the variable or function, and the
    /// element for a pointer into an array (1 + index), or Ok(None) for a
    /// null pointer.
    pub fn const_addr(&self, e: &Expr) -> R<Option<(DeclId, u8)>> {
        let mut m = Machine::new(self, true);
        let v = m.full(|m| m.eval(e))?;
        match v {
            Val::Null => Ok(None),
            Val::FnPtr(_, d) => Ok(Some((d, 0))),
            Val::Ptr(p) => {
                let Some((&(_, d), _)) = m.globals.iter().find(|(_, &o)| o == p.obj) else { return unsup("pointer to an object without a name") };
                if !p.path.is_empty() {
                    return unsup("pointer to a subobject");
                }
                if p.array {
                    if p.off < 0 || p.off > 250 {
                        return unsup("pointer to an array element");
                    }
                    return Ok(Some((d, (p.off + 1) as u8)));
                }
                Ok(Some((d, 0)))
            }
            _ => unsup("not a pointer"),
        }
    }

    pub fn const_int(&self, e: &Expr) -> R<i128> {
        if e.ty.is_unknown() {
            if let EK::Unknown(w) = &e.k {
                return unsup(w.clone());
            }
        }
        let mut m = Machine::new(self, true);
        let e2 = as_lvalue(e);
        let v = m.full(|m| m.eval(&e2))?;
        no_leak(&m)?;
        match v {
            Val::Int(v) => Ok(v),
            Val::Float(_) => nc("CE-25", "a floating value where an integer constant is needed"),
            _ => unsup("non-integer constant"),
        }
    }

    pub fn const_int_quiet(&self, e: &Expr) -> R<i128> {
        self.const_int(e)
    }

    pub fn const_float_quiet(&self, e: &Expr) -> Option<f64> {
        let mut m = Machine::new(self, true);
        let e2 = if e.is_glvalue() { Expr::new(EK::Conv(Conv::LToR, Box::new(e.clone())), e.ty.unqual(), Cat::PR, e.line) } else { e.clone() };
        match m.full(|m| m.eval(&e2)) {
            Ok(Val::Float(f)) => Some(f),
            _ => None,
        }
    }

    pub fn const_eval_bool(&self, e: &Expr) -> R<bool> {
        let mut m = Machine::new(self, true);
        let v = m.full(|m| m.eval_bool(e))?;
        no_leak(&m)?;
        Ok(v)
    }

    /// Check that an expression (a consteval call) is constant.
    pub fn const_check(&self, e: &Expr) -> R<()> {
        let mut m = Machine::new(self, true);
        let v = if matches!(e.ty.t, Ty::Class(_)) && e.cat == Cat::PR {
            let t = e.ty.unqual();
            let place = m.alloc(&t)?;
            m.full(|m| m.init_into(&place, e, &t))?;
            m.objs[place.obj].val.clone()
        } else {
            m.full(|m| m.eval(e))?
        };
        check_result(&m, &v)?;
        no_leak(&m)
    }

    /// Evaluate the initializer of a variable that needs a constant
    /// initializer (CX-1, CINIT-1, CLS-20).
    pub fn const_eval_init(&self, id: DeclId) -> R<()> {
        let Some(v) = self.var(id) else { return Ok(()) };
        let v = v.clone();
        let mut m = Machine::new(self, true);
        let is_static = !(v.local && v.storage == Storage::Auto);
        let Some(init) = v.init.clone() else {
            // Default-initialization of a class object: CX-1 and CE-21 need
            // every scalar part to have a value. A static object is zeroed
            // first (CE-24).
            if matches!(v.ty.t, Ty::Class(_)) {
                let place = m.alloc(&v.ty.unqual())?;
                if is_static && v.constinit && !v.constexpr {
                    // CE-24, Q-0200: a constinit object with no initializer
                    // keeps the values of its zero-initialization.
                    let z = m.zero(&v.ty);
                    m.write(&place, z, true)?;
                }
                m.default_init(&place, &v.ty)?;
                let val = m.objs[place.obj].val.clone();
                return check_result(&m, &val);
            }
            return Ok(());
        };
        if v.ty.t.is_ref() {
            let local = v.local && v.storage == Storage::Auto;
            let p: Ptr = if local { m.full(|m| m.lvalue(&strip_bind(&init)))? } else { m.bind_static_ref(&init)? };
            // CX-2: a constexpr reference refers to a static object.
            if !m.objs[p.obj].is_static {
                return nc("CX-2", "constexpr reference to an object without static storage");
            }
            return Ok(());
        }
        let place = if matches!(v.ty.t, Ty::Class(_) | Ty::Array(..)) {
            m.alloc(&v.ty.unqual())?
        } else {
            Ptr::whole(m.new_obj(Val::Uninit, true))
        };
        m.full(|m| {
            m.init_into(&place, &init, &v.ty)?;
            // CE-14, REF-4: a temporary that a reference member of a static
            // aggregate binds lives until the program ends.
            if is_static {
                for c in m.extended.split_off(0) {
                    m.objs[c.place.obj].is_static = true;
                }
            }
            Ok(())
        })?;
        let val = m.objs[place.obj].val.clone();
        if let Val::Ptr(p) = &val {
            if !m.objs[p.obj].is_static {
                return nc("CX-2", "constexpr pointer to an object without static storage");
            }
        }
        if let Val::Uninit = val {
            return nc("CE-21", "result has a scalar part with no value");
        }
        let _ = has_unset;
        check_result(&m, &val)?;
        no_leak(&m)
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
            Ty::Unknown | Ty::Dep(_) => None,
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
