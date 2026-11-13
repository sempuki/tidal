//! The evaluation of typed expressions. Each operation whose rules can
//! stop a run is a function of its page's `run.rs` part, which this
//! module calls; the module itself names no rule.

use super::{unsup, Machine, Ptr, Ret, Val, R};
use crate::declarations::initialization::forms::run as init_run;
use crate::expressions::{additive, comparison, multiplicative, postfix, shift, unary};
use crate::foundations::{conversions, evaluation_order, objects, values};
use crate::sem::tree::{Arg, BOp, Cat, Conv, Expr, UOp, EK};
use crate::sem::types::{QType, Type};

/// The value of a prvalue expression.
pub fn eval(m: &mut Machine, e: &Expr) -> R<Val> {
    m.step()?;
    m.line = e.span.line;
    match &e.k {
        EK::Int(_) if e.ty.is_nullptr() || e.ty.is_ptr() => Ok(Val::Null),
        EK::Int(v) => Ok(Val::Int(*v)),
        EK::Float(f) if e.ty.t == Type::Float(crate::sem::types::FloatK::LongDouble) => Ok(Val::Ext(crate::foundations::types::run::F80::from_f64(*f))),
        EK::Float(f) => Ok(Val::Float(*f)),
        EK::LongDouble(x) => Ok(Val::Ext(*x)),
        EK::Str(_) | EK::Var(_) | EK::Deref(_) | EK::Index(..) => {
            // A glvalue in a value context: the place's value. Analysis
            // puts an lvalue-to-rvalue conversion here; a bare glvalue is
            // an array or function that decays.
            let p = place(m, e)?;
            values::run::read(m, &p, &e.ty)
        }
        EK::Func(id) => Ok(Val::Fn(*id)),
        EK::Conv(c, x) => conv(m, *c, x, &e.ty),
        EK::Unary(op, x) => {
            let v = eval(m, x)?;
            unary::run::unary(m, *op, v, &e.ty)
        }
        EK::Binary(op, l, r) => {
            let (a, b) = evaluation_order::run::operands(m, *op, l, r)?;
            binary(m, *op, a, b, &l.ty, &r.ty, &e.ty)
        }
        EK::PtrAdd(p, n, sub) => {
            let pv = eval(m, p)?;
            let nv = eval(m, n)?;
            additive::run::ptr_add(m, pv, nv, *sub, &p.ty)
        }
        EK::PtrDiff(a, b) => {
            let x = eval(m, a)?;
            let y = eval(m, b)?;
            additive::run::ptr_diff(m, x, y, &e.ty)
        }
        EK::Logical(and, l, r) => {
            let a = eval(m, l)?.truthy();
            if a != *and {
                return Ok(Val::Int(a as i128));
            }
            Ok(Val::Int(eval(m, r)?.truthy() as i128))
        }
        EK::Assign(..) | EK::Compound(..) => {
            let p = place(m, e)?;
            values::run::read(m, &p, &e.ty)
        }
        EK::IncDec(pre, inc, x) => {
            if *pre {
                let p = place(m, e)?;
                return values::run::read(m, &p, &e.ty);
            }
            let p = place(m, x)?;
            let old = values::run::read(m, &p, &x.ty)?;
            let new = postfix::run::increment(m, old.clone(), *inc, &x.ty)?;
            write(m, &p, &x.ty, new)?;
            Ok(old)
        }
        EK::Cond(c, a, b) => {
            if eval(m, c)?.truthy() {
                eval_any(m, a)
            } else {
                eval_any(m, b)
            }
        }
        EK::Comma(a, b) => {
            eval_discard(m, a)?;
            eval_any(m, b)
        }
        EK::Call(f, args) => match call_expr(m, f, args)? {
            Ret::Val(v) => Ok(v),
            Ret::Place(p) => values::run::read(m, &p, &e.ty),
        },
        EK::AddrOf(x) => match &x.k {
            EK::Func(id) => Ok(Val::Fn(*id)),
            // A reference to a function refers to a place that holds the
            // function.
            _ if x.ty.is_func() => {
                let p = place(m, x)?;
                match m.slot(&p) {
                    Some(Val::Fn(f)) => Ok(Val::Fn(*f)),
                    _ => unsup("the address of a function through this expression"),
                }
            }
            // `&a[n]` may form the pointer one past the end (PFX-4).
            EK::Index(p, i) => {
                let pv = eval(m, p)?;
                let iv = eval(m, i)?;
                additive::run::ptr_add(m, pv, iv, false, &p.ty)
            }
            _ => Ok(Val::Ptr(place(m, x)?)),
        },
        EK::Closure(id) => Ok(Val::Fn(*id)),
        EK::Unevaluated(_) => Ok(Val::Void),
        EK::List(i) => init_value(m, &e.ty, i),
        // ATTR-15: the expression is not evaluated. An expression with
        // no side effect is, so that a false one is found (ATTR-10).
        EK::Assume(c) => {
            if crate::declarations::attributes::run::has_side_effect(c) {
                return Ok(Val::Void);
            }
            let v = eval(m, c)?;
            crate::declarations::attributes::run::assume(m, v)
        }
        EK::EndLife(x) => {
            let p = place(m, x)?;
            crate::foundations::objects::run::end_life(m, &p)
        }
    }
}

/// Evaluate an operand whose value category can be either.
pub fn eval_any(m: &mut Machine, e: &Expr) -> R<Val> {
    if e.cat != Cat::PR && !matches!(e.k, EK::Cond(..) | EK::Comma(..)) {
        let p = place(m, e)?;
        return values::run::read(m, &p, &e.ty);
    }
    eval(m, e)
}

/// Evaluate an expression for its effects (an expression statement).
pub fn eval_discard(m: &mut Machine, e: &Expr) -> R<()> {
    if e.cat != Cat::PR {
        let p = place(m, e)?;
        if crate::expressions::decltype::run::reads_volatile(e) {
            values::run::read(m, &p, &e.ty)?;
        }
    } else {
        eval(m, e)?;
    }
    Ok(())
}

/// The place a glvalue names.
pub fn place(m: &mut Machine, e: &Expr) -> R<Ptr> {
    m.step()?;
    m.line = e.span.line;
    match &e.k {
        EK::Var(id) => m.var_place(*id),
        EK::Str(units) => {
            if let Some(&o) = m.strings.get(units.as_ref()) {
                return Ok(Ptr::whole(o));
            }
            let vals = units.iter().map(|&c| Val::Int(c as i128)).collect();
            let o = m.new_obj(&e.ty, Val::Agg(vals), "a string literal");
            m.objs[o].string = true;
            m.objs[o].konst = true;
            m.objs[o].is_static = true;
            m.objs[o].own = false;
            m.strings.insert(units.to_vec(), o);
            Ok(Ptr::whole(o))
        }
        EK::Deref(x) => {
            let v = eval(m, x)?;
            unary::run::indirection(m, v, &e.ty)
        }
        EK::Index(p, i) => {
            let pv = eval(m, p)?;
            let iv = eval(m, i)?;
            postfix::run::subscript(m, pv, iv)
        }
        EK::Assign(l, r) => {
            // The right operand is sequenced before the left (ASN rules).
            let v = eval(m, r)?;
            let p = place(m, l)?;
            write(m, &p, &l.ty, v)?;
            Ok(p)
        }
        EK::Compound(op, l, r, ct) => {
            let b = eval(m, r)?;
            let p = place(m, l)?;
            let a = values::run::read(m, &p, &l.ty)?;
            let a = conversions::run::convert(m, a, &l.ty, ct)?;
            let res = if *op == BOp::Add || *op == BOp::Sub {
                if l.ty.is_ptr() {
                    additive::run::ptr_add(m, a, b, *op == BOp::Sub, &l.ty)?
                } else {
                    binary(m, *op, a, b, ct, ct, ct)?
                }
            } else {
                binary(m, *op, a, b, ct, &r.ty, ct)?
            };
            let res = if l.ty.is_ptr() { res } else { conversions::run::convert(m, res, ct, &l.ty)? };
            write(m, &p, &l.ty, res)?;
            Ok(p)
        }
        EK::IncDec(true, inc, x) => {
            let p = place(m, x)?;
            let old = values::run::read(m, &p, &x.ty)?;
            let new = unary::run::increment(m, old, *inc, &x.ty)?;
            write(m, &p, &x.ty, new)?;
            Ok(p)
        }
        EK::Cond(c, a, b) => {
            if eval(m, c)?.truthy() {
                place(m, a)
            } else {
                place(m, b)
            }
        }
        EK::Comma(a, b) => {
            eval_discard(m, a)?;
            place(m, b)
        }
        EK::Call(f, args) => match call_expr(m, f, args)? {
            Ret::Place(p) => Ok(p),
            Ret::Val(v) => temp(m, &e.ty, v),
        },
        EK::Conv(Conv::NoOp, x) => place(m, x),
        EK::Conv(Conv::Materialize, x) => {
            let v = eval(m, x)?;
            temp(m, &e.ty, v)
        }
        _ => {
            let v = eval(m, e)?;
            temp(m, &e.ty, v)
        }
    }
}

/// A temporary object that holds a value.
pub fn temp(m: &mut Machine, ty: &QType, v: Val) -> R<Ptr> {
    let o = m.new_obj(&ty.unqual(), v, "a temporary");
    m.objs[o].konst = false;
    m.objs[o].is_static = m.static_temps;
    m.temps.push(o);
    Ok(Ptr::whole(o))
}

/// Write a value to a place (`foundations/objects`, the checks on a
/// write).
pub fn write(m: &mut Machine, p: &Ptr, ty: &QType, v: Val) -> R<()> {
    objects::run::check_write(m, p, ty)?;
    crate::declarations::volatile::run::check_access(m, p, ty)?;
    m.record(p, true);
    let v = match (&ty.t, v) {
        (Type::Float(k), v @ (Val::Float(_) | Val::Ext(_))) => conversions::run::to_float(v, *k),
        (_, v) => v,
    };
    match m.slot(p) {
        Some(s) => {
            *s = v;
            Ok(())
        }
        None => objects::run::outside(m, p),
    }
}

fn conv(m: &mut Machine, c: Conv, x: &Expr, to: &QType) -> R<Val> {
    match c {
        Conv::LToR => {
            let p = place(m, x)?;
            values::run::read(m, &p, &x.ty)
        }
        Conv::ArrayDecay => {
            let mut p = place(m, x)?;
            p.path.push(0);
            p.array = true;
            Ok(Val::Ptr(p))
        }
        Conv::FuncDecay => match &x.k {
            EK::Func(id) => Ok(Val::Fn(*id)),
            _ => eval(m, x),
        },
        Conv::Arith | Conv::ToBool => {
            let v = eval_any(m, x)?;
            conversions::run::convert(m, v, &x.ty, to)
        }
        Conv::NullPtr => {
            eval_any(m, x)?;
            Ok(Val::Null)
        }
        Conv::PtrNoOp if m.constant && x.ty.pointee().is_some_and(|p| p.is_void()) && to.pointee().is_some_and(|p| !p.is_void()) => {
            crate::foundations::constant_evaluation::run::from_void_pointer()
        }
        Conv::PtrNoOp | Conv::NoOp => eval_any(m, x),
        Conv::ToVoid => {
            eval_discard(m, x)?;
            Ok(Val::Void)
        }
        Conv::Materialize => eval(m, x),
        Conv::PtrToInt | Conv::IntToPtr | Conv::PtrReinterpret => {
            let v = eval_any(m, x)?;
            conversions::run::reinterpret(m, v, &x.ty, to)
        }
    }
}

/// A binary operator on two values of the operand types.
pub fn binary(m: &mut Machine, op: BOp, a: Val, b: Val, lt: &QType, rt: &QType, ty: &QType) -> R<Val> {
    match op {
        BOp::Mul | BOp::Div | BOp::Rem => multiplicative::run::binary(m, op, a, b, lt),
        BOp::Add | BOp::Sub => additive::run::binary(m, op, a, b, lt),
        BOp::Shl | BOp::Shr => shift::run::shift(m, op == BOp::Shl, a, b, lt, rt),
        BOp::Lt | BOp::Gt | BOp::Le | BOp::Ge | BOp::Eq | BOp::Ne | BOp::Spaceship => comparison::run::compare(m, op, a, b, lt),
        BOp::BitAnd | BOp::BitXor | BOp::BitOr => {
            let (Val::Int(x), Val::Int(y)) = (a, b) else { return unsup("a bitwise operator on values that are not integers") };
            let r = match op {
                BOp::BitAnd => x & y,
                BOp::BitXor => x ^ y,
                _ => x | y,
            };
            match ty.int_kind() {
                Some(k) => Ok(Val::Int(m.target.wrap(k, r))),
                None => Ok(Val::Int(r)),
            }
        }
    }
}

/// A call: the callee, then the arguments, then the body.
pub fn call_expr(m: &mut Machine, f: &Expr, args: &[Arg]) -> R<Ret> {
    let callee = eval(m, f)?;
    let id = postfix::run::callee(m, callee, &f.ty)?;
    let mut vals = Vec::new();
    for a in args {
        if a.by_ref {
            vals.push(Ret::Place(place(m, &a.e)?));
        } else {
            vals.push(Ret::Val(eval_any(m, &a.e)?));
        }
    }
    call(m, id, vals)
}

/// Run a function with the given arguments.
pub fn call(m: &mut Machine, id: crate::sem::EntityId, args: Vec<Ret>) -> R<Ret> {
    let Some(func) = m.u.func(id) else { return unsup("a call of an entity that is not a function") };
    if let Some(why) = &func.unsupported {
        return unsup(why.clone());
    }
    postfix::run::check_callable(m, id)?;
    let Some(body) = m.body(id) else {
        return postfix::run::no_definition(m, id);
    };
    if m.depth >= m.depth_limit {
        return postfix::run::too_deep(m);
    }
    let params = func.params.clone();
    let ptys: Vec<QType> = func.ty.params.clone();
    let ret_ty = func.ty.ret.clone();
    let mut frame = super::Frame { func: Some(id), ..Default::default() };
    let mut made = Vec::new();
    for (k, a) in args.into_iter().enumerate() {
        let Some(&pid) = params.get(k) else { break };
        match a {
            Ret::Place(p) => {
                frame.refs.insert(pid, p);
            }
            Ret::Val(v) => {
                let ty = ptys.get(k).cloned().unwrap_or_else(QType::error);
                let o = m.new_obj(&ty, v, "a parameter");
                made.push(o);
                frame.vars.insert(pid, o);
            }
        }
    }
    frame.blocks.push(made);
    m.frames.push(frame);
    m.depth += 1;
    let r = super::exec::exec(m, &body);
    m.depth -= 1;
    let mut frame = m.frames.pop().unwrap_or_default();
    for b in frame.blocks.drain(..) {
        for o in b {
            m.objs[o].alive = false;
        }
    }
    let flow = r?;
    match frame.ret.take() {
        Some(r) => Ok(r),
        None if ret_ty.is_void() => Ok(Ret::Val(Val::Void)),
        None => {
            let _ = flow;
            crate::declarations::functions::definitions::run::flow_off_end(m, id)
        }
    }
}

/// Initialize a new object of a variable from its initializer.
pub fn init_value(m: &mut Machine, ty: &QType, init: &crate::sem::tree::Init) -> R<Val> {
    init_run::value(m, ty, init)
}

/// A unary operator that is not `-` on a signed type: `+`, `!`, `~`.
pub fn simple_unary(m: &mut Machine, op: UOp, v: Val, ty: &QType) -> R<Val> {
    match (op, v) {
        (UOp::Plus, v) => Ok(v),
        (UOp::Not, v) => Ok(Val::Int(!v.truthy() as i128)),
        (UOp::BitNot, Val::Int(x)) => match ty.int_kind() {
            Some(k) => Ok(Val::Int(m.target.wrap(k, !x))),
            None => Ok(Val::Int(!x)),
        },
        _ => unsup("a unary operator on this value"),
    }
}
