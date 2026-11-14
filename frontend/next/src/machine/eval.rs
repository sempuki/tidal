//! The evaluation of typed expressions. Each operation whose rules can
//! stop a run is a function of its page's `run.rs` part, which this
//! module calls; the module itself names no rule.

use super::{unsup, Machine, Ptr, Ret, Stop, Val, R};
use crate::declarations::initialization::forms::run as init_run;
use crate::expressions::{additive, comparison, multiplicative, postfix, shift, unary};
use crate::foundations::{conversions, evaluation_order, objects, values};
use crate::sem::tree::{Arg, BOp, Cat, Conv, Expr, UOp, EK};
use crate::sem::types::{QType, Type};

/// The value of a prvalue expression.
pub fn eval(m: &mut Machine, e: &Expr) -> R<Val> {
    let owned = std::mem::take(&mut m.elide);
    m.step()?;
    m.line = e.span.line;
    match &e.k {
        EK::Int(_) if e.ty.is_nullptr() || e.ty.is_ptr() => Ok(Val::Null),
        EK::Int(v) => Ok(Val::Int(*v)),
        EK::Float(f) if e.ty.t == Type::Float(crate::sem::types::FloatK::LongDouble) => Ok(Val::Ext(crate::foundations::types::run::F80::from_f64(*f))),
        EK::Float(f) => Ok(Val::Float(*f)),
        EK::LongDouble(x) => Ok(Val::Ext(*x)),
        EK::This => match m.frames.last().and_then(|f| f.this.clone()) {
            Some(p) => Ok(Val::Ptr(p)),
            None => unsup("'this' outside a member function"),
        },
        // UN-5: `*p` for a pointer to a function names the function.
        EK::Deref(x) if e.ty.is_func() => match eval(m, x)? {
            Val::Fn(id) => Ok(Val::Fn(id)),
            v => unary::run::indirection(m, v, &e.ty).and_then(|_| unsup("indirection through this value")),
        },
        EK::Str(_) | EK::Var(_) | EK::Deref(_) | EK::Index(..) | EK::Member(..) => {
            // A glvalue in a value context: the place's value. Analysis
            // puts an lvalue-to-rvalue conversion here; a bare glvalue is
            // an array or function that decays.
            let p = place(m, e)?;
            values::run::read(m, &p, &e.ty)
        }
        // A function of another unit's declaration is its definition
        // (linking), so that its address compares equal.
        EK::Func(id) | EK::Virtual(id) => Ok(Val::Fn(postfix::run::defining(m, *id))),
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
            let pick = eval(m, c)?.truthy();
            m.elide = owned;
            if pick {
                eval_any(m, a)
            } else {
                eval_any(m, b)
            }
        }
        EK::Comma(a, b) => {
            eval_discard(m, a)?;
            m.elide = owned;
            eval_any(m, b)
        }
        EK::Call(f, args, rev) => match call_expr_in(m, f, args, *rev)? {
            Ret::Val(v) => temporary(m, &e.ty, v, owned),
            Ret::Place(p) => values::run::read(m, &p, &e.ty),
        },
        EK::AddrOf(x) => match &x.k {
            EK::Func(id) => Ok(Val::Fn(postfix::run::defining(m, *id))),
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
        // DER-5: the base subobject's address; null stays null.
        EK::BasePtr(p, path) => match eval(m, p)? {
            Val::Ptr(mut q) if q.view.is_none() => {
                q.path.extend(path.iter().map(|&i| i as i64));
                q.array = false;
                Ok(Val::Ptr(q))
            }
            Val::Null => Ok(Val::Null),
            _ => unsup("a conversion to a base of a pointer of this form"),
        },
        EK::DerivedPtr(p, path) => match eval(m, p)? {
            Val::Ptr(mut q) if q.view.is_none() => {
                let tail: Vec<i64> = path.iter().map(|&i| i as i64).collect();
                if q.path.len() < tail.len() || q.path[q.path.len() - tail.len()..] != tail[..] {
                    return crate::machine::ub("DER-17", "a static_cast to a derived class of an object that is not its base subobject");
                }
                let n = q.path.len() - tail.len();
                q.path.truncate(n);
                // The object that holds the base must be of the derived
                // class, not another class with a base on the same path.
                if m.type_at(&q).and_then(|t| t.class()) != e.ty.pointee().and_then(|t| t.class()) {
                    return crate::machine::ub("DER-17", "a static_cast to a derived class of an object that is not its base subobject");
                }
                Ok(Val::Ptr(q))
            }
            Val::Null => Ok(Val::Null),
            _ => unsup("a conversion to a derived class of a pointer of this form"),
        },
        EK::Unevaluated(_) => Ok(Val::Void),
        EK::List(i) => {
            let v = init_value(m, &e.ty, i)?;
            temporary(m, &e.ty, v, owned)
        }
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
        EK::New(n) => crate::expressions::new_delete::run::new(m, n),
        EK::Delete(x, array, virtual_dtor) => crate::expressions::new_delete::run::delete(m, x, *array, *virtual_dtor),
    }
}

/// Evaluate an operand whose value category can be either.
pub fn eval_any(m: &mut Machine, e: &Expr) -> R<Val> {
    if e.cat != Cat::PR && !matches!(e.k, EK::Cond(..) | EK::Comma(..)) {
        m.elide = false;
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
            let p = unary::run::indirection(m, v, &e.ty)?;
            crate::expressions::lambdas::run::dangling(m, x, &p)?;
            Ok(p)
        }
        // PFX-10: the member of the object, by its position.
        EK::Member(x, i) => {
            let mut p = place(m, x)?;
            if p.view.is_some() {
                // STG-23: no object of the class is at the address.
                return crate::machine::ub("STG-23", format!("an access through a member of {}, where no object of that class is", x.ty.show()));
            }
            p.path.push(*i as i64);
            p.array = false;
            Ok(p)
        }
        EK::Index(p, i) => {
            let pv = eval(m, p)?;
            let iv = eval(m, i)?;
            postfix::run::subscript(m, pv, iv)
        }
        EK::Assign(l, r) => {
            // The right operand is sequenced before the left (ASN rules).
            let v = match crate::declarations::initialization::forms::run::indeterminate_copy(m, &l.ty, r) {
                Some(v) => v?,
                None => eval(m, r)?,
            };
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
        EK::Call(f, args, rev) => match call_expr_in(m, f, args, *rev)? {
            Ret::Place(p) => Ok(p),
            Ret::Val(v) => temp(m, &e.ty, v),
        },
        EK::Conv(Conv::NoOp, x) => place(m, x),
        EK::Conv(Conv::Materialize, x) => {
            m.elide = true;
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
/// A class prvalue that no object owns: a temporary object that holds
/// it until the end of its full expression, when its destructor runs
/// (OBJ-9). Other prvalues are values only.
fn temporary(m: &mut Machine, ty: &QType, v: Val, owned: bool) -> R<Val> {
    if !owned && crate::classes::constructors::run::needs(m, ty) {
        temp(m, ty, v.clone())?;
    }
    Ok(v)
}

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
    if p.view.is_some() {
        return crate::foundations::storage::run::write(m, p, ty, v);
    }
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
        // A pointer converted to another pointee type names the place of
        // that type at its address, or a view (STG-29).
        Conv::PtrNoOp => match eval_any(m, x)? {
            Val::Ptr(p) => match to.pointee() {
                Some(t) if x.ty.pointee().is_some_and(|f| f.unqual().t != t.unqual().t) => Ok(Val::Ptr(crate::foundations::storage::run::convert(m, p, t))),
                _ => Ok(Val::Ptr(p)),
            },
            v => Ok(v),
        },
        Conv::NoOp => eval_any(m, x),
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
    call_expr_in(m, f, args, false)
}

/// A call whose arguments are evaluated from the last to the first when
/// `rev` holds (OPR-7).
pub fn call_expr_in(m: &mut Machine, f: &Expr, args: &[Arg], rev: bool) -> R<Ret> {
    let id = match &f.k {
        EK::Virtual(id) => *id,
        _ => {
            let callee = eval(m, f)?;
            postfix::run::callee(m, callee, &f.ty)?
        }
    };
    let mut vals = Vec::new();
    let order: Vec<usize> = if rev { (0..args.len()).rev().collect() } else { (0..args.len()).collect() };
    for k in order {
        let a = &args[k];
        if a.by_ref {
            let p = place(m, &a.e);
            // The object of a member call in constant evaluation can be a
            // reference whose object is unknown (CE-30); the machine does
            // not model such an object yet.
            if let (0, Err(Stop::Limit("CE-30", _))) = (k, &p) {
                if m.u.func(id).is_some_and(|f| f.class.is_some() && !f.member_static) {
                    return unsup("the object of a member call is a reference whose object constant evaluation does not know");
                }
            }
            vals.push(Ret::Place(p?));
        } else {
            // A parameter owns its argument's object.
            m.elide = true;
            vals.push(Ret::Val(eval_any(m, &a.e)?));
        }
    }
    if rev {
        vals.reverse();
    }
    if let EK::Virtual(_) = f.k {
        if vals.is_empty() {
            return unsup("a virtual call without its object");
        }
        let (g, obj) = crate::classes::r#virtual::run::dispatch(m, id, vals.remove(0))?;
        vals.insert(0, obj);
        let r = call(m, g, vals)?;
        return crate::classes::r#virtual::run::covariant(m, g, id, r);
    }
    call(m, id, vals)
}

/// Run a function with the given arguments.
pub fn call(m: &mut Machine, id: crate::sem::EntityId, args: Vec<Ret>) -> R<Ret> {
    // A declaration in a block, or a C function declared in another
    // namespace, names a function that another declaration defines
    // (LNK-9, LSP-4).
    let id = postfix::run::defining(m, id);
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
    // The object of a non-static member function comes first.
    let mut args = args;
    // LAM-10: a call through the pointer that a closure converts to has
    // no object.
    let through_pointer = func.class.is_some_and(|c| m.u.closures.contains_key(&c)) && args.len() == params.len();
    if func.class.is_some() && !func.member_static && !through_pointer {
        if args.is_empty() {
            return unsup("a member function called without its object");
        }
        match args.remove(0) {
            Ret::Place(p) => frame.this = Some(p),
            Ret::Val(Val::Ptr(p)) => frame.this = Some(p),
            _ => return unsup("the object of a member function call"),
        }
    }
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
    let special = func.special;
    if let (Some(p), crate::sem::Special::None) = (&frame.this, special) {
        if !func.member_static && func.class.is_some() {
            if m.early.iter().any(|(o, q)| *o == p.obj && p.path.starts_with(q)) {
                return crate::declarations::initialization::forms::run::before_bases(m, id);
            }
            // STG-22: no object of a polymorphic class is at the address.
            if p.view.is_some() && func.class.is_some_and(|c| m.u.polymorphic(c)) {
                return crate::foundations::storage::run::no_object_call(m, id);
            }
        }
    }
    // A constructor's bases see their own initializers only.
    let early = (special == crate::sem::Special::Ctor).then(|| std::mem::take(&mut m.early));
    // VIRT-6: while a constructor or destructor runs, its object is of
    // its class.
    let structor = match (&frame.this, func.special) {
        (Some(p), crate::sem::Special::Ctor | crate::sem::Special::Dtor) => {
            m.structors.push((p.obj, p.path.clone()));
            true
        }
        _ => false,
    };
    m.frames.push(frame);
    m.depth += 1;
    // CTOR-6: a constructor initializes the members before its body.
    let r = match m.u.mem_inits.get(&id) {
        Some(mi) => crate::declarations::initialization::forms::run::mem_inits(m, id, mi).and_then(|_| super::exec::exec(m, &body)),
        None => super::exec::exec(m, &body),
    };
    m.depth -= 1;
    if structor {
        m.structors.pop();
    }
    if let Some(e) = early {
        m.early = e;
    }
    let mut frame = m.frames.pop().unwrap_or_default();
    for b in frame.blocks.drain(..).rev() {
        if r.is_ok() {
            crate::classes::constructors::run::end_objects(m, b)?;
        } else {
            for o in b {
                m.objs[o].alive = false;
            }
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
