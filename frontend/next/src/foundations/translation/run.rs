//! The run part of `foundations/translation`: the run of a program.
//! The static variables are initialized in the order of their
//! definitions, then `main` runs, and its value is the status (TR-2).

use crate::machine::eval::call;
use crate::machine::{Machine, Ret, Stop, Val};
use crate::sem::Unit;

/// The global scope that holds a scope.
fn root_of(u: &Unit, mut s: crate::sem::ScopeId) -> crate::sem::ScopeId {
    while let Some(p) = u.scopes[s].parent {
        s = p;
    }
    s
}

/// The names of the namespaces around an entity, and its own name, or
/// None for an entity with no name or in an unnamed namespace. A
/// declaration in a block names the entity of its enclosing namespace
/// (LNK-9), so the blocks add no name.
fn qualified(u: &Unit, id: crate::sem::EntityId) -> Option<String> {
    let e = &u.entities[id];
    if e.name.is_empty() {
        return None;
    }
    let mut parts = vec![e.name.to_string()];
    let mut s = if u.scopes[e.scope].kind == crate::sem::ScopeKind::Class { e.scope } else { u.enclosing_namespace(e.scope) };
    while let Some(p) = u.scopes[s].parent {
        // The scope that binds a specialization's parameters adds no
        // name.
        if u.scopes[s].kind == crate::sem::ScopeKind::Template {
            s = p;
            continue;
        }
        if !matches!(u.scopes[s].kind, crate::sem::ScopeKind::Namespace | crate::sem::ScopeKind::Class) || u.scopes[s].name.is_empty() {
            return None;
        }
        parts.push(spelled_scope(u, s));
        s = p;
    }
    parts.reverse();
    Some(parts.join("::"))
}

/// The name of a scope, with the arguments of a class template
/// specialization, so that each specialization names its own members
/// (ODR-12).
fn spelled_scope(u: &Unit, s: crate::sem::ScopeId) -> String {
    use crate::sem::types::TArg;
    let name = u.scopes[s].name.to_string();
    if u.scopes[s].kind != crate::sem::ScopeKind::Class {
        return name;
    }
    fn arg(u: &Unit, a: &TArg) -> String {
        match a {
            TArg::Type(t) => match t.class() {
                Some(c) => format!("class {}", u.classes[c].name),
                None => t.show(),
            },
            TArg::Value(_, v) => v.to_string(),
            TArg::Pack(xs) => xs.iter().map(|x| arg(u, x)).collect::<Vec<_>>().join(","),
        }
    }
    for ct in &u.class_templates {
        if let Some((args, _)) = ct.specs.iter().find(|(_, c)| u.classes[*c].scope == s) {
            return format!("{}<{}>", name, args.iter().map(|a| arg(u, a)).collect::<Vec<_>>().join(","));
        }
    }
    name
}

/// Link the units of a program: each declaration with external linkage
/// that its unit does not define names the definition of the same name,
/// kind, and type in another unit. A function with C language linkage
/// matches by its name alone.
pub fn link(u: &mut Unit) {
    use crate::sem::{EntKind, Linkage};
    // A type's spelling, with each enumeration by its name, since each
    // unit has its own enumeration of a shared header (ODR-7).
    let named = |u: &Unit, t: String| -> String {
        let mut out = String::new();
        let mut rest = t.as_str();
        while let Some(i) = rest.find("enum#").into_iter().chain(rest.find("class#")).min() {
            out.push_str(&rest[..i]);
            let class = rest[i..].starts_with("class#");
            let skip = if class { 6 } else { 5 };
            let digits: String = rest[i + skip..].chars().take_while(|c| c.is_ascii_digit()).collect();
            let n: usize = digits.parse().unwrap_or(usize::MAX);
            out.push_str(&if class {
                u.classes.get(n).map_or(String::from("class"), |e| format!("class {}", e.name))
            } else {
                u.enums.get(n).map_or(String::from("enum"), |e| format!("enum {}", e.name))
            });
            rest = &rest[i + skip + digits.len()..];
        }
        out.push_str(rest);
        out
    };
    let sig = |u: &Unit, id: crate::sem::EntityId| -> Option<(bool, String)> {
        match &u.entities[id].kind {
            EntKind::Func(f) => Some((true, named(u, crate::sem::types::QType::new(crate::sem::types::Type::Func(f.ty.clone())).show()))),
            EntKind::Var(v) => Some((false, named(u, v.ty.show()))),
            _ => None,
        }
    };
    let defined = |u: &Unit, id: crate::sem::EntityId| match &u.entities[id].kind {
        EntKind::Func(f) => f.body.is_some(),
        EntKind::Var(v) => v.defined && !(v.is_extern && v.init.is_none()),
        _ => false,
    };
    let c_func = |u: &Unit, id: crate::sem::EntityId| u.func(id).is_some_and(|f| f.c_linkage);
    let mut defs: std::collections::HashMap<(String, bool, String), Vec<crate::sem::EntityId>> = Default::default();
    for id in 0..u.entities.len() {
        if u.entities[id].linkage != Linkage::External || !defined(u, id) {
            continue;
        }
        let (Some(name), Some((func, ty))) = (if c_func(u, id) { Some(u.entities[id].name.to_string()) } else { qualified(u, id) }, sig(u, id)) else { continue };
        defs.entry((name, func, ty)).or_default().push(id);
    }
    let inline = |u: &Unit, id: crate::sem::EntityId| match &u.entities[id].kind {
        EntKind::Func(f) => f.inline,
        EntKind::Var(v) => v.inline,
        _ => false,
    };
    let mut links = std::collections::HashMap::new();
    for id in 0..u.entities.len() {
        if u.entities[id].linkage != Linkage::External {
            continue;
        }
        // INL-1: an inline function or variable that several units
        // define is one entity, the first unit's definition.
        if defined(u, id) && !inline(u, id) {
            continue;
        }
        let (Some(name), Some((func, ty))) = (if c_func(u, id) { Some(u.entities[id].name.to_string()) } else { qualified(u, id) }, sig(u, id)) else { continue };
        let root = root_of(u, u.entities[id].scope);
        let found = defs.get(&(name, func, ty)).and_then(|ds| {
            if defined(u, id) {
                ds.first().copied().filter(|&d| d < id && root_of(u, u.entities[d].scope) != root)
            } else {
                ds.iter().copied().find(|&d| root_of(u, u.entities[d].scope) != root)
            }
        });
        if let Some(d) = found {
            links.insert(id, d);
        }
    }
    u.links = links;
}

/// How a run ended.
#[derive(Debug, Clone, PartialEq)]
pub enum RunOutcome {
    /// The status of the program, which the environment receives modulo
    /// 256 (TR-11).
    Result(i128),
    Undefined(&'static str, String, u32),
    Unsupported(String, u32),
}

/// The dynamic initialization of the variable `id` with static or thread
/// storage, whose zero-initialized object is `o` (SINIT rules).
pub fn dynamic_init(m: &mut Machine, id: crate::sem::EntityId, o: usize) -> Result<(), Stop> {
    let u = m.u;
    let v = u.var(id).unwrap();
    if let Some(init) = &v.init {
        if crate::declarations::initialization::forms::run::constructs(init) {
            crate::declarations::initialization::forms::run::construct(m, &crate::machine::Ptr::whole(o), &v.ty, init)?;
            crate::machine::exec::end_temps(m, 0)?;
            m.static_order.push(o);
            return Ok(());
        }
        let val = crate::machine::eval::init_value(m, &v.ty, init)?;
        let val = m.fill_zero(&v.ty, val);
        m.objs[o].val = val;
        m.objs[o].konst = v.ty.is_const();
        for t in m.temps.drain(..) {
            m.objs[t].alive = false;
        }
    }
    Ok(())
}

/// OBJ-11: the first use of a thread_local variable that waits starts
/// every waiting thread_local variable of the unit, in the order of
/// their definitions.
pub fn start_thread_locals(m: &mut Machine) -> Result<(), Stop> {
    for (id, o) in std::mem::take(&mut m.thread_pending) {
        dynamic_init(m, id, o)?;
    }
    Ok(())
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
            // A reference whose initializer is not constant binds in the
            // order of the dynamic initializers (SINIT rules).
            if v.ty.is_ref() && !constant && v.storage != crate::sem::Storage::Thread {
                dynamic.push((id, usize::MAX));
            } else if constant || v.ty.is_ref() {
                crate::machine::exec::static_var(&mut m, id)?;
            } else {
                m.check_size(&v.ty)?;
                let z = m.zero(&v.ty);
                let o = m.new_obj(&v.ty, z, &u.entities[id].name.clone());
                crate::foundations::storage::run::var_aligned(&mut m, id, o);
                m.objs[o].is_static = true;
                m.objs[o].konst = false;
                m.globals.insert(id, o);
                // OBJ-11: a thread_local variable waits for the first use
                // of one of the unit's thread_local variables.
                if v.storage == crate::sem::Storage::Thread {
                    m.thread_pending.push((id, o));
                } else {
                    dynamic.push((id, o));
                }
            }
        }
        for (id, o) in dynamic {
            if o == usize::MAX {
                crate::machine::exec::static_var(&mut m, id)?;
                continue;
            }
            dynamic_init(&mut m, id, o)?;
        }
        let Some(main) = u.main() else { return crate::machine::unsup("the program has no main function") };
        let args = main_arguments(&mut m, main);
        let v = match call(&mut m, main, args)? {
            Ret::Val(Val::Int(v)) => v,
            _ => return crate::machine::unsup("main returned no integer"),
        };
        // OBJ-12: after main, the static objects end in the reverse order
        // of their construction, each that needs it destroyed.
        // Namespace-scope objects count in the order of their
        // definitions, also the constant-initialized ones.
        let waiting: Vec<usize> = m.thread_pending.iter().map(|&(_, o)| o).collect();
        let defined: Vec<usize> = u.statics.iter().filter_map(|id| m.globals.get(id).copied()).filter(|o| !waiting.contains(o)).collect();
        let mut order = defined.clone();
        order.extend(std::mem::take(&mut m.static_order).into_iter().filter(|o| !defined.contains(o)));
        crate::classes::constructors::run::end_objects(&mut m, order)?;
        Ok(v)
    })();
    match r {
        Ok(v) | Err(Stop::Exit(v)) => RunOutcome::Result(v),
        Err(Stop::Undefined(rule, msg)) => RunOutcome::Undefined(rule, msg, m.line),
        Err(Stop::Limit(rule, msg)) => RunOutcome::Unsupported(format!("[{}] {}", rule, msg), m.line),
        Err(Stop::Unsupported(msg)) => RunOutcome::Unsupported(msg, m.line),
        // UNW-5, UNW-10: an exception that no handler catches ends the
        // program as SIGABRT does, with the status 134.
        Err(Stop::Throw(_) | Stop::Terminate(_)) => RunOutcome::Result(134),
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
    let first = Val::Ptr(crate::machine::Ptr { obj: so, path: vec![0], array: true, view: None });
    let ao = m.new_obj(&at, Val::Agg(vec![first, Val::Null]), "argv");
    m.objs[ao].is_static = true;
    let argv = Val::Ptr(crate::machine::Ptr { obj: ao, path: vec![0], array: true, view: None });
    let mut out = vec![Ret::Val(Val::Int(1)), Ret::Val(argv)];
    if n > 2 {
        let eo = m.new_obj(&QType::new(Type::Array(std::rc::Rc::new(QType::ptr_to(QType::new(Type::Int(IntK::Char)))), Some(1))), Val::Agg(vec![Val::Null]), "envp");
        out.push(Ret::Val(Val::Ptr(crate::machine::Ptr { obj: eo, path: vec![0], array: true, view: None })));
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
