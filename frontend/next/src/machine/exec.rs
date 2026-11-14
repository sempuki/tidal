//! The execution of typed statements and the objects of variables. A
//! jump is a `Flow` that each enclosing statement passes on until the
//! statement it names (its `StmtId`, or the block that holds its label)
//! takes it. A `goto` into a block, and a `switch` to a label, run the
//! statements that hold the target in seeking mode until they reach it.
//! The rules of each statement sit in its page's `run.rs` part.

use super::eval::{eval, eval_any, eval_discard, init_value, place};
use super::{Flow, Machine, Ptr, Ret, Val, R};
use crate::sem::tree::{Cond, Init, Stmt, StmtId, SK};
use crate::sem::{EntKind, EntityId, Storage};
use crate::statements::{jumps, switch};
use std::rc::Rc;

/// The target of a seek: a label, or a `case` or `default` label of a
/// switch.
#[derive(Clone, Debug, PartialEq)]
pub enum Seek {
    Label(Rc<str>),
    Case(StmtId, Option<usize>),
}

/// Whether a statement holds the target.
pub fn contains(s: &Stmt, t: &Seek) -> bool {
    match &s.k {
        SK::Label(l, x) => matches!(t, Seek::Label(n) if n == l) || contains(x, t),
        SK::Case(id, k, x) => matches!(t, Seek::Case(i, j) if i == id && j == k) || contains(x, t),
        SK::Block(xs) => xs.iter().any(|x| contains(x, t)),
        SK::If { then, els, .. } => contains(then, t) || els.as_ref().is_some_and(|e| contains(e, t)),
        SK::Chosen(Some(x)) => contains(x, t),
        SK::Consteval { constant, runtime } => contains(constant, t) || contains(runtime, t),
        SK::While { body, .. } | SK::Do { body, .. } | SK::For { body, .. } | SK::RangeArray { body, .. } => contains(body, t),
        // A nested switch's own labels are not the outer switch's.
        SK::Switch { body, .. } => matches!(t, Seek::Label(_)) && contains(body, t),
        _ => false,
    }
}

pub fn exec(m: &mut Machine, s: &Stmt) -> R<Flow> {
    run(m, s, None)
}

fn end_block(m: &mut Machine) -> R<()> {
    let objs = m.frames.last_mut().and_then(|f| f.blocks.pop()).unwrap_or_default();
    crate::classes::constructors::run::end_objects(m, objs)
}

fn open_block(m: &mut Machine) {
    if let Some(f) = m.frames.last_mut() {
        f.blocks.push(Vec::new());
    }
}

/// The objects of a full expression end with it.
pub fn end_temps(m: &mut Machine, from: usize) -> R<()> {
    let objs: Vec<usize> = m.temps.drain(from.min(m.temps.len())..).collect();
    crate::classes::constructors::run::end_objects(m, objs)
}

/// Run a statement; with a seek, run from the target inside it.
pub fn run(m: &mut Machine, s: &Stmt, seek: Option<&Seek>) -> R<Flow> {
    m.step()?;
    m.line = s.span.line;
    if let Some(t) = seek {
        if !contains(s, t) {
            return Ok(Flow::Normal);
        }
    }
    match &s.k {
        SK::Null | SK::Fallthrough => Ok(Flow::Normal),
        SK::Expr(e) => {
            let t = m.temps.len();
            eval_discard(m, e)?;
            end_temps(m, t)?;
            Ok(Flow::Normal)
        }
        SK::Decl(vars) => {
            for (id, init) in vars {
                declare(m, *id, init)?;
            }
            Ok(Flow::Normal)
        }
        SK::Block(items) => block(m, items, seek),
        SK::If { init, cond, then, els } => {
            open_block(m);
            let r = (|| -> R<Flow> {
                if seek.is_none() {
                    if let Some(i) = init {
                        let f = run(m, i, None)?;
                        if f != Flow::Normal {
                            return Ok(f);
                        }
                    }
                    if condition(m, cond)?.truthy() {
                        run(m, then, None)
                    } else if let Some(e) = els {
                        run(m, e, None)
                    } else {
                        Ok(Flow::Normal)
                    }
                } else if contains(then, seek.unwrap()) {
                    run(m, then, seek)
                } else {
                    run(m, els.as_ref().unwrap(), seek)
                }
            })();
            end_block(m)?;
            r
        }
        SK::Chosen(x) => match x {
            Some(x) => run(m, x, seek),
            None => Ok(Flow::Normal),
        },
        // IF-18, IF-25: the branch for the mode the evaluation runs in.
        SK::Consteval { constant, runtime } => run(m, if m.constant || m.constant_init { constant } else { runtime }, seek),
        SK::While { id, cond, body } => {
            let mut seek = seek.cloned();
            loop {
                open_block(m);
                let r = (|| -> R<Option<Flow>> {
                    if seek.is_none() && !condition(m, cond)?.truthy() {
                        return Ok(Some(Flow::Normal));
                    }
                    let f = run(m, body, seek.as_ref())?;
                    seek = None;
                    Ok(loop_flow(f, *id))
                })();
                end_block(m)?;
                if let Some(f) = r? {
                    return Ok(f);
                }
            }
        }
        SK::Do { id, body, cond } => {
            let mut seek = seek.cloned();
            loop {
                let f = run(m, body, seek.as_ref())?;
                seek = None;
                if let Some(f) = loop_flow(f, *id) {
                    return Ok(f);
                }
                let t = m.temps.len();
                let c = eval(m, cond)?.truthy();
                end_temps(m, t)?;
                if !c {
                    return Ok(Flow::Normal);
                }
            }
        }
        SK::For { id, init, cond, step, body } => {
            open_block(m);
            let r = (|| -> R<Flow> {
                let mut seek = seek.cloned();
                if seek.is_none() {
                    if let Some(i) = init {
                        run(m, i, None)?;
                    }
                }
                loop {
                    open_block(m);
                    let r = (|| -> R<Option<Flow>> {
                        if seek.is_none() {
                            if let Some(c) = cond {
                                if !condition(m, c)?.truthy() {
                                    return Ok(Some(Flow::Normal));
                                }
                            }
                        }
                        let f = run(m, body, seek.as_ref())?;
                        seek = None;
                        let flow = loop_flow(f, *id);
                        // LOOP-3: the step runs before the variable of the
                        // condition is destroyed.
                        if flow.is_none() {
                            if let Some(st) = step {
                                let t = m.temps.len();
                                eval_discard(m, st)?;
                                end_temps(m, t)?;
                            }
                        }
                        Ok(flow)
                    })();
                    end_block(m)?;
                    if let Some(f) = r? {
                        return Ok(f);
                    }
                }
            })();
            end_block(m)?;
            r
        }
        SK::RangeArray { id, init, range, range_init, var, bound, body } => {
            open_block(m);
            let r = (|| -> R<Flow> {
                if let Some(i) = init {
                    run(m, i, None)?;
                }
                m.keep_temps = true;
                let d = declare(m, *range, range_init);
                m.keep_temps = false;
                d?;
                let base = m.var_place(*range)?;
                for k in 0..*bound as i64 {
                    open_block(m);
                    let mut elem = base.clone();
                    elem.path.push(k);
                    elem.array = true;
                    let r = (|| -> R<Option<Flow>> {
                        crate::statements::range_for::run::bind_element(m, *var, elem)?;
                        let f = run(m, body, None)?;
                        Ok(loop_flow(f, *id))
                    })();
                    end_block(m)?;
                    if let Some(f) = r? {
                        return Ok(f);
                    }
                }
                Ok(Flow::Normal)
            })();
            end_block(m)?;
            r
        }
        SK::Switch { id, init, cond, body, cases } => {
            open_block(m);
            let r = (|| -> R<Flow> {
                let target = if let Some(t) = seek {
                    t.clone()
                } else {
                    if let Some(i) = init {
                        run(m, i, None)?;
                    }
                    let v = condition(m, cond)?;
                    match switch::run::choose(m, v, cases, body, *id) {
                        Some(t) => t,
                        None => return Ok(Flow::Normal),
                    }
                };
                let f = run(m, body, Some(&target))?;
                Ok(match f {
                    Flow::Break(b) if b == *id => Flow::Normal,
                    f => f,
                })
            })();
            end_block(m)?;
            r
        }
        SK::Case(_, _, x) | SK::Label(_, x) => {
            // At the target the seek ends; a seek for a label further in
            // goes on.
            let here = match (&s.k, seek) {
                (SK::Case(i, k, _), Some(Seek::Case(a, b))) => i == a && k == b,
                (SK::Label(l, _), Some(Seek::Label(n))) => l == n,
                _ => seek.is_none(),
            };
            run(m, x, if here { None } else { seek })
        }
        SK::Break(id) => Ok(Flow::Break(*id)),
        SK::Continue(id) => Ok(Flow::Continue(*id)),
        SK::Return(e) => {
            let v = match e {
                Some(e) => {
                    let t = m.temps.len();
                    // The result object owns the returned prvalue.
                    m.elide = true;
                    let v = eval_any(m, e)?;
                    end_temps(m, t)?;
                    v
                }
                None => Val::Void,
            };
            if let Some(f) = m.frames.last_mut() {
                f.ret = Some(Ret::Val(v));
            }
            Ok(Flow::Return)
        }
        SK::ReturnRef(e) => {
            let p = place(m, e)?;
            if let Some(f) = m.frames.last_mut() {
                f.ret = Some(Ret::Place(p));
            }
            Ok(Flow::Return)
        }
        SK::Goto(l) => jumps::run::goto(m, l),
        SK::Unsupported(why) => super::unsup(why.clone()),
    }
}

/// What a loop does with the flow of its body: None to go on.
fn loop_flow(f: Flow, id: StmtId) -> Option<Flow> {
    match f {
        Flow::Normal => None,
        Flow::Continue(c) if c == id => None,
        Flow::Break(b) if b == id => Some(Flow::Normal),
        f => Some(f),
    }
}

fn block(m: &mut Machine, items: &[Stmt], seek: Option<&Seek>) -> R<Flow> {
    open_block(m);
    // The objects of the block's variables exist from its start, so a
    // jump past a declaration with no initializer finds them (JMP rules).
    for it in items {
        if let SK::Decl(vars) = &it.k {
            for (id, _) in vars {
                if let Some(v) = m.u.var(*id) {
                    if v.storage == Storage::Automatic && !v.ty.is_ref() {
                        m.check_size(&v.ty)?;
                        let val = m.uninit(&v.ty);
                        let o = m.new_obj(&v.ty, val, &m.u.entities[*id].name.clone());
                        m.objs[o].konst = false;
                        m.undeclared.insert(o);
                        if let Some(f) = m.frames.last_mut() {
                            f.vars.insert(*id, o);
                            if let Some(b) = f.blocks.last_mut() {
                                b.push(o);
                            }
                        }
                    }
                }
            }
        }
    }
    let r = (|| -> R<Flow> {
        let mut seek = seek.cloned();
        let mut i = match &seek {
            Some(t) => items.iter().position(|x| contains(x, t)).unwrap_or(items.len()),
            None => 0,
        };
        while i < items.len() {
            let f = run(m, &items[i], seek.as_ref())?;
            seek = None;
            match f {
                Flow::Normal => i += 1,
                Flow::Goto(l) => {
                    let t = Seek::Label(l.clone());
                    match items.iter().position(|x| contains(x, &t)) {
                        Some(k) => {
                            i = k;
                            seek = Some(t);
                        }
                        None => return Ok(Flow::Goto(l)),
                    }
                }
                f => return Ok(f),
            }
        }
        Ok(Flow::Normal)
    })();
    end_block(m)?;
    r
}

/// The value of a condition; a declared variable lives in the
/// statement's block.
fn condition(m: &mut Machine, c: &Cond) -> R<Val> {
    let t = m.temps.len();
    let v = match c {
        Cond::Expr(e) => eval(m, e)?,
        Cond::Decl(id, init, e) => {
            declare_new(m, *id, init)?;
            eval(m, e)?
        }
    };
    end_temps(m, t)?;
    Ok(v)
}

/// The run of a declaration statement for one variable.
fn declare(m: &mut Machine, id: EntityId, init: &Init) -> R<()> {
    let Some(v) = m.u.var(id) else { return Ok(()) };
    if v.storage != Storage::Automatic {
        // A block-scope static variable is initialized the first time
        // control passes its declaration.
        if m.constant {
            crate::foundations::constant_evaluation::run::reach_static_local(m, id)?;
        }
        static_var(m, id)?;
        return Ok(());
    }
    // IF-25: an initializer of a local variable that the language tries
    // as a constant, and that is one, runs as a constant evaluation.
    let constant = v.constexpr || (v.const_init && (v.ty.is_ref() || crate::foundations::constant_evaluation::run::usable_in_constant_expressions(m, v)));
    let was = m.constant_init;
    m.constant_init |= constant;
    let r = declare_automatic(m, id, init);
    m.constant_init = was;
    r
}

fn declare_automatic(m: &mut Machine, id: EntityId, init: &Init) -> R<()> {
    let Some(v) = m.u.var(id) else { return Ok(()) };
    let has = m.frames.last().is_some_and(|f| f.vars.contains_key(&id));
    if !has {
        return declare_new(m, id, init);
    }
    let ty = v.ty.clone();
    let t = m.temps.len();
    let o = m.frames.last().unwrap().vars[&id];
    m.undeclared.remove(&o);
    if crate::declarations::initialization::forms::run::constructs(init) {
        m.objs[o].val = m.uninit(&ty);
        m.objs[o].alive = true;
        crate::declarations::initialization::forms::run::construct(m, &Ptr::whole(o), &ty, init)?;
    } else {
        let val = init_value(m, &ty, init)?;
        m.objs[o].val = val;
        m.objs[o].alive = true;
    }
    m.objs[o].konst = ty.is_const();
    end_temps(m, t)?;
    Ok(())
}

/// A variable whose object the block did not make in advance: a
/// reference, or a variable of a condition or an init-statement.
fn declare_new(m: &mut Machine, id: EntityId, init: &Init) -> R<()> {
    let Some(v) = m.u.var(id) else { return Ok(()) };
    let ty = v.ty.clone();
    if let Init::Ref(e) = init {
        let t = m.temps.len();
        let p = place(m, e)?;
        // The temporary the reference keeps moves to the block.
        let kept = crate::declarations::initialization::references::run::kept_temporary(m, e, p.obj, t)?;
        if let Some(f) = m.frames.last_mut() {
            if let Some(b) = f.blocks.last_mut() {
                b.extend(kept);
            }
            f.refs.insert(id, p);
        }
        return Ok(());
    }
    m.check_size(&ty)?;
    let t = m.temps.len();
    let o = if crate::declarations::initialization::forms::run::constructs(init) {
        let o = m.new_obj(&ty, m.uninit(&ty), &m.u.entities[id].name.clone());
        crate::declarations::initialization::forms::run::construct(m, &Ptr::whole(o), &ty, init)?;
        o
    } else {
        let val = init_value(m, &ty, init)?;
        m.new_obj(&ty, val, &m.u.entities[id].name.clone())
    };
    end_temps(m, t)?;
    m.objs[o].konst = ty.is_const();
    if let Some(f) = m.frames.last_mut() {
        f.vars.insert(id, o);
        if let Some(b) = f.blocks.last_mut() {
            b.push(o);
        }
    }
    Ok(())
}

/// The object of a variable with static or thread storage, initialized
/// the first time the run needs it.
pub fn static_var(m: &mut Machine, id: EntityId) -> R<Ptr> {
    if let Some(&o) = m.globals.get(&id) {
        return Ok(Ptr::whole(o));
    }
    if let Some(p) = m.global_refs.get(&id) {
        return Ok(p.clone());
    }
    let Some(ent) = m.u.entities.get(id) else { return super::unsup("an unknown variable") };
    let EntKind::Var(v) = &ent.kind else { return super::unsup("a name that is not a variable") };
    let v = v.clone();
    let name = ent.name.clone();
    m.check_size(&v.ty)?;
    if m.constant {
        return crate::foundations::constant_evaluation::run::static_var(m, id, &v);
    }
    let init = match v.init.clone() {
        // A static variable with no initializer is zero-initialized.
        Some(Init::Default) => None,
        i => i,
    };
    let Some(init) = init else {
        // A static variable with no initializer is zero-initialized.
        let z = m.zero(&v.ty);
        let o = m.new_obj(&v.ty, z, &name);
        m.objs[o].is_static = true;
        m.objs[o].own = false;
        m.globals.insert(id, o);
        m.static_order.push(o);
        return Ok(Ptr::whole(o));
    };
    crate::declarations::storage_linkage::run::enter_init(m, id)?;
    let saved = std::mem::take(&mut m.temps);
    let frames = std::mem::replace(&mut m.frames, vec![super::Frame { blocks: vec![Vec::new()], ..Default::default() }]);
    let was = m.constant_init;
    m.constant_init |= v.const_init;
    let r = (|| -> R<Ptr> {
        if let Init::Ref(e) = &init {
            let p = place(m, e)?;
            m.temps.clear();
            m.global_refs.insert(id, p.clone());
            return Ok(p);
        }
        if crate::declarations::initialization::forms::run::constructs(&init) {
            let o = m.new_obj(&v.ty, m.zero(&v.ty), &name);
            m.objs[o].is_static = true;
            m.objs[o].own = false;
            m.globals.insert(id, o);
            crate::declarations::initialization::forms::run::construct(m, &Ptr::whole(o), &v.ty, &init)?;
            end_temps(m, 0)?;
            m.static_order.push(o);
            return Ok(Ptr::whole(o));
        }
        let val = init_value(m, &v.ty, &init)?;
        let val = m.fill_zero(&v.ty, val);
        let o = m.new_obj(&v.ty, val, &name);
        m.objs[o].is_static = true;
        m.objs[o].own = false;
        m.globals.insert(id, o);
        end_temps(m, 0)?;
        m.static_order.push(o);
        Ok(Ptr::whole(o))
    })();
    m.constant_init = was;
    m.frames = frames;
    m.temps = saved;
    m.initializing.retain(|&x| x != id);
    r
}
