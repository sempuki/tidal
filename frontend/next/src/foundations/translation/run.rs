//! The run part of `foundations/translation`: the run of a program.
//! The static variables are initialized in the order of their
//! definitions, then `main` runs, and its value is the status (TR-2).

use crate::machine::eval::call;
use crate::machine::{Machine, Ret, Stop, Val};
use crate::sem::Unit;

/// How a run ended.
#[derive(Debug, Clone, PartialEq)]
pub enum RunOutcome {
    /// The status of the program, which the environment receives modulo
    /// 256 (TR-11).
    Result(i128),
    Undefined(&'static str, String, u32),
    Unsupported(String, u32),
}

/// Run the program of one translation unit.
pub fn run_program(u: &Unit) -> RunOutcome {
    let mut m = Machine::new(u, false);
    let r = (|| -> Result<i128, Stop> {
        // SINIT rules: every static variable is zero-initialized or
        // constant-initialized first; then the dynamic initializers run in
        // the order of the definitions.
        let mut dynamic = Vec::new();
        for &id in &u.statics {
            let Some(v) = u.var(id) else { continue };
            let constant = v.const_init || matches!(v.init, None | Some(crate::sem::tree::Init::Default));
            if constant || v.ty.is_ref() {
                crate::machine::exec::static_var(&mut m, id)?;
            } else {
                m.check_size(&v.ty)?;
                let z = m.zero(&v.ty);
                let o = m.new_obj(&v.ty, z, &u.entities[id].name.clone());
                m.objs[o].is_static = true;
                m.objs[o].konst = false;
                m.globals.insert(id, o);
                dynamic.push((id, o));
            }
        }
        for (id, o) in dynamic {
            let v = u.var(id).unwrap();
            if let Some(init) = &v.init {
                let val = crate::machine::eval::init_value(&mut m, &v.ty, init)?;
                m.objs[o].val = val;
                m.objs[o].konst = v.ty.is_const();
                for t in m.temps.drain(..) {
                    m.objs[t].alive = false;
                }
            }
        }
        let Some(main) = u.main() else { return crate::machine::unsup("the program has no main function") };
        let args = main_arguments(&mut m, main);
        match call(&mut m, main, args)? {
            Ret::Val(Val::Int(v)) => Ok(v),
            _ => crate::machine::unsup("main returned no integer"),
        }
    })();
    match r {
        Ok(v) | Err(Stop::Exit(v)) => RunOutcome::Result(v),
        Err(Stop::Undefined(rule, msg)) => RunOutcome::Undefined(rule, msg, m.line),
        Err(Stop::Limit(rule, msg)) => RunOutcome::Unsupported(format!("[{}] {}", rule, msg), m.line),
        Err(Stop::Unsupported(msg)) => RunOutcome::Unsupported(msg, m.line),
    }
}

/// The arguments of `main` with parameters (TR-2): `argc` is 1, and
/// `argv` points to the program's name and a null pointer.
fn main_arguments(m: &mut Machine, main: crate::sem::EntityId) -> Vec<Ret> {
    use crate::sem::types::{IntK, QType, Type};
    let n = m.u.func(main).map_or(0, |f| f.ty.params.len());
    if n == 0 {
        return Vec::new();
    }
    let name: Vec<Val> = "a.out\0".chars().map(|c| Val::Int(c as i128)).collect();
    let ct = QType::new(Type::Int(IntK::Char));
    let st = QType::new(Type::Array(std::rc::Rc::new(ct.clone()), Some(name.len() as u64)));
    let so = m.new_obj(&st, Val::Agg(name), "the program's name");
    m.objs[so].is_static = true;
    let pt = QType::ptr_to(ct);
    let at = QType::new(Type::Array(std::rc::Rc::new(pt), Some(2)));
    let first = Val::Ptr(crate::machine::Ptr { obj: so, path: vec![0], array: true });
    let ao = m.new_obj(&at, Val::Agg(vec![first, Val::Null]), "argv");
    m.objs[ao].is_static = true;
    let argv = Val::Ptr(crate::machine::Ptr { obj: ao, path: vec![0], array: true });
    let mut out = vec![Ret::Val(Val::Int(1)), Ret::Val(argv)];
    if n > 2 {
        let eo = m.new_obj(&QType::new(Type::Array(std::rc::Rc::new(QType::ptr_to(QType::new(Type::Int(IntK::Char)))), Some(1))), Val::Agg(vec![Val::Null]), "envp");
        out.push(Ret::Val(Val::Ptr(crate::machine::Ptr { obj: eo, path: vec![0], array: true })));
    }
    out.truncate(n);
    out
}

/// The line that `tidal-next run` and the harness print for an outcome.
pub fn describe(o: &RunOutcome) -> String {
    match o {
        RunOutcome::Result(v) => format!("result: {}", v),
        RunOutcome::Undefined(r, msg, l) => format!("undefined [{}] at line {}: {}", r, l, msg),
        RunOutcome::Unsupported(msg, l) => format!("unsupported at line {}: {}", l, msg),
    }
}
