//! The run part of `exceptions/throw`: a `throw` creates the exception
//! object from its operand (THR-1) and leaves through the enclosing
//! operations as `Stop::Throw`, which each of them passes on after its
//! own cleanup (UNW-1). A rethrow passes on the handled exception
//! (THR-6), and without one the program ends (THR-7).

use crate::machine::{Machine, Ptr, Stop, Val, R};
use crate::sem::tree::{Expr, Init};

/// `throw e` or `throw`.
pub fn throw(m: &mut Machine, e: Option<&Expr>) -> R<Val> {
    let Some(e) = e else {
        return match m.handled.last() {
            Some(&o) => Err(Stop::Throw(o)),
            None => terminate("a rethrow with no handled exception"),
        };
    };
    // CE-8: a throw ends a constant evaluation.
    if m.constant {
        return crate::machine::limit("CE-8", "a throw expression");
    }
    // THR-13: the storage of the exception object comes from the
    // implementation.
    let ty = e.ty.unqual();
    let o = m.new_obj(&ty, m.uninit(&ty), "the exception object");
    m.objs[o].konst = false;
    crate::declarations::initialization::forms::run::construct(m, &Ptr::whole(o), &ty, &Init::Expr(e.clone()))?;
    Err(Stop::Throw(o))
}

/// UNW-5: the program ends.
pub fn terminate<T>(why: impl Into<String>) -> R<T> {
    Err(Stop::Terminate(why.into()))
}

/// The result `r` of an operation, after `cleanup` ran: an exception
/// that leaves a cleanup during the unwinding of another ends the
/// program (UNW-5).
pub fn unwind<T>(m: &mut Machine, r: R<T>, cleanup: impl FnOnce(&mut Machine) -> R<()>) -> R<T> {
    match (r, cleanup(m)) {
        (Err(Stop::Throw(_)), Err(Stop::Throw(_))) => terminate("a destructor throws during the unwinding of an exception"),
        (Err(Stop::Throw(_)), Err(e)) => Err(e),
        (r, Ok(())) => r,
        (Ok(_), Err(e)) => Err(e),
        (Err(e), Err(_)) => Err(e),
    }
}

/// Q-0365: the result that a return statement of the current function
/// built ends when an exception leaves the function or reaches a handler
/// in it after the return.
pub fn drop_result(m: &mut Machine) -> R<()> {
    let Some(f) = m.frames.last_mut() else { return Ok(()) };
    if !matches!(f.ret, Some(crate::machine::Ret::Val(_))) {
        return Ok(());
    }
    let (Some(crate::machine::Ret::Val(v)), Some(id)) = (f.ret.take(), f.func) else { return Ok(()) };
    let Some(ty) = m.u.func(id).map(|x| x.ty.ret.clone()) else { return Ok(()) };
    if !crate::classes::constructors::run::needs(m, &ty) {
        return Ok(());
    }
    let o = m.new_obj(&ty, v, "the result");
    crate::classes::constructors::run::destroy(m, &Ptr::whole(o), &ty)?;
    m.objs[o].alive = false;
    Ok(())
}

/// The exception object `o` ends, after the last handler for it
/// (THR-8).
pub fn end_exception(m: &mut Machine, o: usize) -> R<()> {
    if m.handled.contains(&o) || !m.objs[o].alive {
        return Ok(());
    }
    let ty = m.objs[o].ty.clone();
    if crate::classes::constructors::run::needs(m, &ty) {
        crate::classes::constructors::run::destroy(m, &Ptr::whole(o), &ty)?;
    }
    m.objs[o].alive = false;
    Ok(())
}
