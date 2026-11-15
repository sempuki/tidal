//! The run part of `exceptions/handlers`: a try block catches an
//! exception that leaves it with the first handler that matches
//! (TRY-1, TRY-2). The handler's parameter binds to, or copies, the
//! exception object; the handled exception lives until the handler
//! ends, through rethrows (THR-8).

use crate::exceptions::throw::run::end_exception;
use crate::machine::{Machine, Ptr, Stop, Val, R};
use crate::sem::tree::{Handler, Stmt};
use crate::sem::types::{QType, Type};

/// A try block.
pub fn try_block(m: &mut Machine, body: &Stmt, handlers: &[Handler], seek: Option<&crate::machine::exec::Seek>) -> R<crate::machine::Flow> {
    let t = m.temps.len();
    let o = match crate::machine::exec::run(m, body, seek) {
        Err(Stop::Throw(o)) => o,
        r => return r,
    };
    // UNW-1: the temporaries of the full expressions that the exception
    // left end first.
    crate::exceptions::throw::run::unwind(m, Ok(()), |m| crate::machine::exec::end_temps(m, t))?;
    catch(m, handlers, o)
}

/// The first of `handlers` that matches the exception object `o` runs
/// (TRY-1); with none, the exception goes on.
pub fn catch(m: &mut Machine, handlers: &[Handler], o: usize) -> R<crate::machine::Flow> {
    catch_keeping(m, handlers, o, false)
}

/// `catch`; `keep` keeps the exception object after a handler that
/// completes, for the rethrow at the end of a constructor's or
/// destructor's handler (TRY-10).
pub fn catch_keeping(m: &mut Machine, handlers: &[Handler], o: usize, keep: bool) -> R<crate::machine::Flow> {
    for h in handlers {
        let place = match &h.ty {
            None => Some(Ptr::whole(o)),
            Some(ty) => matches(m, ty, o)?,
        };
        let Some(place) = place else { continue };
        return handle(m, h, o, place, keep);
    }
    Err(Stop::Throw(o))
}

/// Run the handler `h` for the exception object `o`, whose parameter
/// binds to `place`.
fn handle(m: &mut Machine, h: &Handler, o: usize, place: Ptr, keep: bool) -> R<crate::machine::Flow> {
    m.handled.push(o);
    m.caught = Some(place);
    crate::machine::exec::open_block(m);
    let r = (|| {
        if let Some((id, init)) = &h.param {
            // TRY-18: a copy into the parameter that throws ends the
            // program.
            match crate::machine::exec::declare(m, *id, init) {
                Err(Stop::Throw(_)) => return crate::exceptions::throw::run::terminate("the copy into a handler parameter throws"),
                r => r?,
            }
        }
        crate::machine::exec::run(m, &h.body, None)
    })();
    // TRY-15: the handler's objects end, then its handled exception.
    let r = crate::exceptions::throw::run::unwind(m, r, crate::machine::exec::end_block);
    m.handled.pop();
    match r {
        // A rethrow passes the same object on.
        Err(Stop::Throw(x)) if x == o => Err(Stop::Throw(o)),
        Err(Stop::Throw(x)) => {
            crate::exceptions::throw::run::unwind(m, Ok(()), |m| end_exception(m, o))?;
            Err(Stop::Throw(x))
        }
        r => {
            if !keep {
                end_exception(m, o)?;
            }
            r
        }
    }
}

/// TRY-2: the place of the exception object `o` as a handler of type
/// `ty` sees it, or None when the handler does not match.
fn matches(m: &mut Machine, ty: &QType, o: usize) -> R<Option<Ptr>> {
    let want = ty.nonref().unqual();
    let have = m.objs[o].ty.unqual();
    if want == have {
        return Ok(Some(Ptr::whole(o)));
    }
    // A public base, one subobject of it.
    if let (Some(b), Some(d)) = (want.class(), have.class()) {
        let ps = m.u.base_paths(d, b);
        if ps.len() == 1 && m.u.path_public(d, &ps[0]) {
            return Ok(Some(crate::classes::derived::run::along(m, Ptr::whole(o), d, &ps[0])?));
        }
        return Ok(None);
    }
    // A pointer that converts by the conversions of TRY-2.
    let value = |m: &Machine| m.objs[o].val.clone();
    let converted = match (&want.t, &have.t) {
        (Type::Ptr(_) | Type::MemPtr(..), Type::NullPtr) => Some(Val::Null),
        (Type::Ptr(t), Type::Ptr(f)) => {
            let qual = |t: &QType, f: &QType| (t.c || !f.c) && (t.v || !f.v);
            if t.unqual() == f.unqual() && qual(t, f) {
                Some(value(m))
            } else if t.is_void() && !f.is_func() && qual(t, f) {
                Some(value(m))
            } else if let (Some(b), Some(d)) = (t.class(), f.class()) {
                let ps = m.u.base_paths(d, b);
                if ps.len() == 1 && m.u.path_public(d, &ps[0]) && qual(t, f) {
                    match value(m) {
                        Val::Ptr(p) => Some(Val::Ptr(crate::classes::derived::run::along(m, p, d, &ps[0])?)),
                        v => Some(v),
                    }
                } else {
                    None
                }
            } else if let (Type::Func(a), Type::Func(b)) = (&t.t, &f.t) {
                (a.params == b.params && a.ret == b.ret && a.variadic == b.variadic && b.noexcept).then(|| value(m))
            } else {
                None
            }
        }
        // A pointer to member by a qualification conversion.
        (Type::MemPtr(c, t), Type::MemPtr(d, f)) if c == d && t.unqual() == f.unqual() && (t.c || !f.c) && (t.v || !f.v) => Some(value(m)),
        // A pointer to a noexcept member function, for a handler without
        // noexcept.
        (Type::MemPtr(c, t), Type::MemPtr(d, f)) if c == d && t.is_func() && crate::foundations::conversions::check::pointer_convertible(f, t) => Some(value(m)),
        _ => None,
    };
    let Some(v) = converted else { return Ok(None) };
    let t = m.new_obj(&want, v, "the exception object as the handler sees it");
    Ok(Some(Ptr::whole(t)))
}
