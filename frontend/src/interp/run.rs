// Running a whole program (TR-2, OBJ-4): static initialization, `main`,
// and the end of the static objects after `main` returns.

use super::*;

/// How a run ended.
#[derive(Debug, Clone)]
pub enum RunOutcome {
    /// The exit status, with the choice points the run passed.
    Result(i128, Vec<Choice>),
    /// Undefined behavior: the rule, a message, and the line.
    Undefined(&'static str, String, u32),
    /// A construct the interpreter does not model, and the line.
    Unsupported(String, u32),
    /// The program ended without returning from `main` (UNW-5, THR-7):
    /// the rules, a message, and the line. On the matrix targets the
    /// program then ends with a nonzero status.
    Terminated(&'static str, String, u32),
}

fn stop_outcome(s: Stop, line: u32) -> RunOutcome {
    match s {
        Stop::Undefined(r, m) => RunOutcome::Undefined(r, m, line),
        // A run has no constant-evaluation limits; a limit here is a
        // constant evaluation that ran during the run.
        Stop::Limit(r, m) => RunOutcome::Unsupported(format!("constant evaluation stopped at run time ({}: {})", r, m), line),
        Stop::Unsupported(m) => RunOutcome::Unsupported(m, line),
        Stop::Terminate(r, m) => RunOutcome::Terminated(r, m, line),
        Stop::Throw(_) => RunOutcome::Terminated("UNW-5", "an exception has no handler".into(), line),
        Stop::Exit(st, _) => RunOutcome::Result(st, Vec::new()),
    }
}

/// Run the program whose translation units are `ps`, each checked and
/// accepted, linked together (TR-4).
pub fn run_units(ps: &[&P]) -> RunOutcome {
    run_explored(ps).0
}

/// The most runs that the exploration of choice points makes.
pub const RUN_BOUND: usize = 64;

/// One run with the choices that `script` gives, the choices taken, and
/// the line of each.
fn run_once(ps: &[&P], script: &[u8]) -> (RunOutcome, Vec<(u8, u8)>, Vec<u32>) {
    let mut m = Machine::new(ps[0], false);
    m.ps = ps.to_vec();
    m.script = script.to_vec();
    m.link();
    let o = match m.run_all() {
        Ok(status) => RunOutcome::Result(status, m.choices.clone()),
        Err(s) => {
            let line = m.line;
            stop_outcome(s, line)
        }
    };
    (o, m.taken, m.taken_line)
}

/// A run with more choice points than this settles a line once another
/// choice at that line gave the outcome of the first run.
const SETTLE_AFTER: usize = 4096;

/// The runs of a program over its choice points (decision 0048): each
/// allowed choice at SEQ-1, SEQ-8, PRIM-9, CMP-8, and CMP-20, up to
/// `RUN_BOUND` runs. Returns the outcome to report, the outcomes of the
/// other runs, and, when the exploration left choices untried, the
/// reason: the bound, or a settled line.
///
/// The reported outcome is the first run's, unless another run stops at
/// undefined behavior: an allowed choice that leads to undefined behavior
/// makes the program's behavior undefined.
///
/// A run with very many choice points (a recursion through an operator
/// with unsequenced operands) explores each line once: when the other
/// option at the last choice point of a line gives the outcome of the
/// first run, the other choice points of that line are not explored.
pub fn run_explored(ps: &[&P]) -> (RunOutcome, Vec<RunOutcome>, Option<String>) {
    let (first, mut taken, mut lines) = run_once(ps, &[]);
    let mut others = Vec::new();
    let mut runs = 1;
    let mut bound = false;
    if matches!(first, RunOutcome::Unsupported(..)) {
        return (first, others, None);
    }
    let first_status = match &first {
        RunOutcome::Result(s, _) => Some(*s),
        _ => None,
    };
    let mut settled: std::collections::HashSet<u32> = Default::default();
    loop {
        // The next script: the last choice with an option left goes to
        // its next option, and the choices after it start again.
        let Some(i) = taken.iter().enumerate().rposition(|(k, &(c, n))| c + 1 < n && !settled.contains(&lines[k])) else { break };
        if runs >= RUN_BOUND {
            bound = true;
            break;
        }
        let mut script: Vec<u8> = taken[..i].iter().map(|x| x.0).collect();
        script.push(taken[i].0 + 1);
        let line = lines[i];
        let many = taken.len() > SETTLE_AFTER;
        let (o, t, l) = run_once(ps, &script);
        runs += 1;
        if many && matches!(&o, RunOutcome::Result(s, _) if Some(*s) == first_status) {
            settled.insert(line);
        }
        taken = t;
        lines = l;
        others.push(o);
    }
    // A settled line skips options, so the set is partial as well.
    let partial = if bound {
        Some(format!("the bound of {} runs stopped the exploration", RUN_BOUND))
    } else if !settled.is_empty() {
        let n = settled.len();
        Some(format!("{} {} settled after another choice there gave the first outcome", n, if n == 1 { "line was" } else { "lines were" }))
    } else {
        None
    };
    let ub = others.iter().find(|o| matches!(o, RunOutcome::Undefined(..))).cloned();
    match (ub, &first) {
        (Some(u), RunOutcome::Result(..)) => (u, others, partial),
        _ => (first, others, partial),
    }
}

impl<'a> Machine<'a> {
    /// The variables with static storage duration at namespace or class
    /// scope that this unit defines, in the order of their definitions.
    fn static_vars(&self) -> Vec<DeclId> {
        let mut out = Vec::new();
        for (id, d) in self.p.decls.iter().enumerate() {
            let DK::Var(v) = &d.k else { continue };
            if v.local || v.param || v.dependent {
                continue;
            }
            // A member of a class template's pattern, or a variable of a
            // type that depends on a template parameter, is no object.
            if v.class.is_some_and(|c| self.p.classes[c].template) || matches!(v.ty.t, Ty::Dep(_)) {
                continue;
            }
            if v.storage == Storage::Extern && v.init.is_none() {
                continue;
            }
            if v.class.is_some() && v.init.is_none() && !self.p.static_member_defs.contains(&id) && !v.defined {
                continue;
            }
            out.push(id);
        }
        // SINIT-7: a static data member defined outside its class takes
        // the place of its definition.
        out.sort_by_key(|&id| match self.p.static_member_pos.get(&id) {
            Some(&(n, k)) => (n, 0, k, id),
            None => (id, 1, 0, id),
        });
        out
    }

    /// The linkage name of a declaration: its qualified name, and for a
    /// function its parameter types, with classes and enumerations by
    /// name.
    fn link_name(p: &P, id: DeclId, tu: usize) -> String {
        // LSP-4: a C name is known by its spelling alone.
        // LSP-8: a variable of the global namespace and a C variable with
        // its spelling name one entity across units.
        let global_var = matches!(&p.decls[id].k, DK::Var(v) if v.class.is_none() && !v.local) && p.decls[id].scope == p.global;
        if is_c_name(p, id) || global_var {
            return format!("extern C {}", p.decls[id].name);
        }
        let mut parts = vec![p.decls[id].name.clone()];
        let mut s = Some(p.decls[id].scope);
        while let Some(x) = s {
            let sc = &p.scopes[x];
            match sc.kind {
                SK::Namespace if x != p.global => parts.push(if sc.name.is_empty() { "(unnamed)".into() } else { sc.name.clone() }),
                SK::Class(c) => parts.push(p.class_label_of(c)),
                _ => {}
            }
            s = sc.parent;
        }
        parts.reverse();
        let mut n = parts.join("::");
        if let Some((_, args)) = p.spec_of.get(&id) {
            // A specialization: its template arguments.
            n.push_str(&format!("<{}>", p.show_targs(args)));
        }
        if let Some(f) = p.func(id) {
            let ps: Vec<String> = f.fty.params.iter().map(|q| type_name(p, q)).collect();
            // LNK-1, LNK-5: a class of an unnamed namespace is a different
            // type in each unit, so a function with such a parameter is a
            // different function in each unit.
            fn internal(p: &P, t: &Ty, depth: u32) -> bool {
                match t {
                    Ty::Class(c) => p.in_unnamed_ns(p.classes[*c].scope),
                    Ty::Enum(e) => p.in_unnamed_ns(p.enums[*e].scope),
                    Ty::Ptr(q) | Ty::LRef(q) | Ty::RRef(q) | Ty::Array(q, _) => depth < 16 && internal(p, &q.t, depth + 1),
                    _ => false,
                }
            }
            if f.fty.params.iter().any(|q| internal(p, &q.t, 0)) {
                n.push_str(&format!("@unit{}", tu));
            }
            n.push_str(&format!("({}){}{}{}", ps.join(","), if f.fty.cv_c { " const" } else { "" }, if f.fty.cv_v { " volatile" } else { "" }, ["", " &", " &&"][f.fty.refq.min(2) as usize]));
        }
        n
    }

    /// TR-4, LNK rules: each declaration with external linkage names the
    /// first definition in the units' order. An inline function or
    /// variable is one entity in every unit.
    pub fn link(&mut self) {
        let ps = self.ps.clone();
        let linkable = |p: &P, id: DeclId| -> Option<bool> {
            match &p.decls[id].k {
                DK::Func(f) => {
                    if !is_c_name(p, id) && (p.internal_fns.contains(&id) || f.template || p.in_unnamed_ns(p.decls[id].scope)) {
                        return None;
                    }
                    Some(f.body.is_some() || f.defaulted)
                }
                DK::Var(v) => {
                    if v.local || v.param || v.dependent || (p.var_internal.contains(&id) && !is_c_name(p, id)) {
                        return None;
                    }
                    let defined = if v.class.is_some() { v.init.is_some() || p.static_member_defs.contains(&id) } else { v.storage != Storage::Extern || v.init.is_some() };
                    Some(defined)
                }
                _ => None,
            }
        };
        let mut defs: HashMap<String, (usize, DeclId)> = HashMap::new();
        for (t, p) in ps.iter().enumerate() {
            for id in 0..p.decls.len() {
                if linkable(p, id) == Some(true) {
                    defs.entry(Self::link_name(p, id, t)).or_insert((t, id));
                }
            }
        }
        for (t, p) in ps.iter().enumerate() {
            for id in 0..p.decls.len() {
                if linkable(p, id).is_some() {
                    if let Some(&d) = defs.get(&Self::link_name(p, id, t)) {
                        // Within one unit a declaration is its own entity.
                        if d.0 != t {
                            self.links.insert((t, id), d);
                        }
                    }
                }
            }
        }
    }

    /// Initialize the static objects of every unit, run `main`, and end
    /// the static objects.
    fn run_all(&mut self) -> R<i128> {
        match self.run_all_inner() {
            // `exit` ends the static objects that exist, and `_Exit` ends
            // none; the automatic objects do not end (decision 0035, the
            // C library list).
            Err(Stop::Exit(st, statics)) => {
                if statics {
                    let list = std::mem::take(&mut self.static_dtors);
                    self.static_dtors_running = true;
                    let r = self.cleanup(list);
                    self.static_dtors_running = false;
                    match r {
                        Err(Stop::Exit(st2, _)) => return Ok(st2),
                        Err(Stop::Throw(_)) => return Err(Stop::Terminate("UNW-5", "an exception leaves the destructor of a static object".into())),
                        r => r?,
                    }
                }
                Ok(st)
            }
            r => r,
        }
    }

    /// OBJ-11: the deferred `thread_local` constructions of unit `t`, run
    /// at the first use of one of them.
    pub fn start_deferred_tl(&mut self, t: usize) -> R<()> {
        let (now, rest): (Vec<(usize, DeclId)>, Vec<(usize, DeclId)>) = std::mem::take(&mut self.deferred_tl).into_iter().partition(|x| x.0 == t);
        self.deferred_tl = rest;
        for (t, id) in now {
            let r = self.in_tu(t, |m| {
                let v = m.p.var(id).unwrap().clone();
                m.init_static(id, &v)
            });
            if let Err(Stop::Throw(_)) = r {
                return Err(Stop::Terminate("UNW-5", "an exception leaves the initialization of a thread_local object".into()));
            }
            r?;
        }
        Ok(())
    }

    fn run_all_inner(&mut self) -> R<i128> {
        let mut main = None;
        for (t, p) in self.ps.clone().iter().enumerate() {
            for (id, d) in p.decls.iter().enumerate() {
                if let DK::Func(f) = &d.k {
                    if f.is_main && f.body.is_some() {
                        main = Some((t, id));
                    }
                }
            }
        }
        let Some((mt, main)) = main else {
            return unsup("no definition of main in the program (TR-4)");
        };
        let n = self.ps.len();
        // Zero-initialization of every static object comes first.
        let mut vars = Vec::new();
        for t in 0..n {
            let vs = self.in_tu(t, |m| Ok(m.static_vars()))?;
            for id in vs {
                if !self.links.contains_key(&(t, id)) {
                    vars.push((t, id));
                }
            }
        }
        for &(t, id) in &vars {
            self.in_tu(t, |m| m.zero_static(id))?;
        }
        // Constant initialization, then the dynamic initializations in the
        // order of the definitions (OBJ-4). SINIT-3: the order between
        // units is unspecified; the interpreter takes the units in order.
        let mut dynamic = Vec::new();
        // A constant initializer that reads another static variable runs
        // after it (a specialization's variable can be declared after the
        // variable that reads it).
        let order = self.const_order(&vars);
        let mut is_const = std::collections::HashSet::new();
        for &(t, id) in &order {
            if self.in_tu(t, |m| m.init_constant(id))? {
                is_const.insert((t, id));
            }
        }
        for &(t, id) in &vars {
            if !is_const.contains(&(t, id)) {
                dynamic.push((t, id));
            }
        }
        if n > 1 && dynamic.iter().any(|d| d.0 != dynamic[0].0) {
            self.choice("SINIT-3", "the translation units initialize their static objects in the order the units are given");
        }
        for (t, id) in dynamic {
            if self.ps[t].var(id).is_some_and(|v| v.storage == Storage::ThreadLocal && !v.local) {
                // OBJ-11: Clang and GCC defer the construction to the first
                // use of such a variable of the unit.
                self.deferred_tl.push((t, id));
                continue;
            }
            let r = self.in_tu(t, |m| {
                let v = m.p.var(id).unwrap().clone();
                m.init_static(id, &v)
            });
            if let Err(e) = r {
                if let Stop::Throw(_) = e {
                    // UNW-5: no handler can catch it.
                    return Err(Stop::Terminate("UNW-5", "an exception leaves the initialization of a static object".into()));
                }
                return Err(e);
            }
        }
        let status = self.in_tu(mt, |m| m.run_main_fn(main))?;
        // OBJ-4: the static objects end after main returns, in the reverse
        // of the order in which their construction completed.
        let list = std::mem::take(&mut self.static_dtors);
        self.static_dtors_running = true;
        let r = self.cleanup(list);
        self.static_dtors_running = false;
        match r {
            Err(Stop::Throw(_)) => return Err(Stop::Terminate("UNW-5", "an exception leaves the destructor of a static object".into())),
            r => r?,
        }
        Ok(status)
    }

    /// The static variables, each after the variables of its unit that
    /// its initializer names.
    fn const_order(&self, vars: &[(usize, DeclId)]) -> Vec<(usize, DeclId)> {
        let set: std::collections::HashSet<(usize, DeclId)> = vars.iter().copied().collect();
        let mut done: std::collections::HashSet<(usize, DeclId)> = std::collections::HashSet::new();
        let mut out = Vec::new();
        fn visit(m: &Machine, v: (usize, DeclId), set: &std::collections::HashSet<(usize, DeclId)>, done: &mut std::collections::HashSet<(usize, DeclId)>, out: &mut Vec<(usize, DeclId)>, depth: u32) {
            if done.contains(&v) || depth > 64 {
                return;
            }
            done.insert(v);
            let p = m.ps[v.0];
            if let Some(init) = p.var(v.1).and_then(|x| x.init.clone()) {
                let mut deps = Vec::new();
                crate::ast::walk_expr(&init, &mut |x| {
                    if let EK::Var(d) = &x.k {
                        deps.push(*d);
                    }
                });
                for d in deps {
                    if set.contains(&(v.0, d)) {
                        visit(m, (v.0, d), set, done, out, depth + 1);
                    }
                }
            }
            out.push(v);
        }
        for &v in vars {
            visit(self, v, &set, &mut done, &mut out, 0);
        }
        out
    }

    fn zero_static(&mut self, id: DeclId) -> R<()> {
        let v = self.p.var(id).unwrap().clone();
        if v.ty.t.is_ref() {
            return Ok(());
        }
        let place = self.alloc(&v.ty.unqual())?;
        let z = self.zero(&v.ty);
        self.write(&place, z, true)?;
        self.objs[place.obj].is_static = true;
        self.objs[place.obj].volatile = v.ty.v;
        self.globals.insert((self.tu, id), place.obj);
        if let Some(&a) = self.p.var_align.get(&id) {
            self.raw_align.insert(place.obj, a);
        }
        Ok(())
    }

    /// CE-24: a constant initializer runs before any dynamic one. Returns
    /// whether the variable had one.
    fn init_constant(&mut self, id: DeclId) -> R<bool> {
        let v = self.p.var(id).unwrap().clone();
        let aggregate = matches!(v.ty.t, Ty::Class(_) | Ty::Array(..));
        let constant = !v.ty.t.is_ref() && (v.constexpr || v.constinit || (v.init.is_some() && self.p.const_eval_init(id).is_ok() && !self.needs_dtor(&v.ty.t)));
        if !constant {
            return Ok(false);
        }
        // SINIT-1: a class object whose initializer is a constant
        // expression is constant-initialized too. If the run cannot do it
        // as a constant evaluation, it stays dynamic.
        let saved = if aggregate && !v.constexpr && !v.constinit {
            self.globals.get(&(self.tu, id)).map(|&o| (o, self.objs[o].val.clone()))
        } else {
            None
        };
        // The constant initializer runs as a constant evaluation, so
        // `if consteval` takes its first branch (IF-18).
        self.constant = true;
        let r = self.init_static(id, &v);
        self.constant = false;
        if let (Err(_), Some((o, val))) = (&r, saved) {
            self.objs[o].val = val;
            return Ok(false);
        }
        r?;
        Ok(true)
    }

    fn run_main_fn(&mut self, main: DeclId) -> R<i128> {
        // TR-2: execution begins with a call to main.
        let f = self.p.func(main).unwrap().clone();
        let mut frame = Frame::new();
        if f.params.len() == 3 || f.params.len() == 1 {
            // FN-6: the lists this front end accepts. The third parameter
            // points to the list of environment variables, which is empty
            // here.
            let argc = self.new_obj(Val::Int(1), true);
            frame.vars.insert(f.params[0], argc);
        }
        if f.params.len() == 2 || f.params.len() == 3 {
            // TR-9: one argument, the program's name.
            let name: Vec<u32> = "tidal-fe".chars().map(|c| c as u32).collect();
            let s = self.string_obj(&name);
            let s = Ptr { obj: s.obj, path: vec![], off: 0, array: true, bytes: false };
            let argv = self.new_obj(Val::Agg(vec![Val::Ptr(s), Val::Null]), true);
            let argc = self.new_obj(Val::Int(1), true);
            frame.vars.insert(f.params[0], argc);
            let pv = self.new_obj(Val::Ptr(Ptr { obj: argv, path: vec![], off: 0, array: true, bytes: false }), true);
            frame.vars.insert(f.params[1], pv);
            if f.params.len() == 3 {
                let env = self.new_obj(Val::Agg(vec![Val::Null]), true);
                let pe = self.new_obj(Val::Ptr(Ptr { obj: env, path: vec![], off: 0, array: true, bytes: false }), true);
                frame.vars.insert(f.params[2], pe);
            }
        } else if f.params.len() > 1 {
            return unsup("main with parameters other than (int, char**)");
        }
        let body = f.body.clone().unwrap();
        self.enter_call(Some(main), None, false, false, None)?;
        self.install_params(frame);
        let r = match f.fn_try.clone() {
            Some(hs) => self.run_fn_try(&body, &hs, false),
            None => self.run_body(&body),
        };
        let ret = self.leave_call();
        if let Err(Stop::Throw(_)) = &r {
            // UNW-7: with no matching handler, no automatic object ends
            // (the matrix table); the unwinding stops at the throw.
            return Err(Stop::Terminate("UNW-5", "an exception leaves main".into()));
        }
        let flow = r?;
        let ret = ret?;
        let status = match (flow, ret) {
            (Flow::Return, Some(Ret::Val(Val::Int(v)))) => v,
            (Flow::Return, Some(_)) => return unsup("main returned something other than an integer"),
            (Flow::Goto(l), _) => return unsup(format!("goto to label '{}' that the interpreter did not find", l)),
            // TR-2: reaching the end of main returns zero.
            _ => 0,
        };
        Ok(status)
    }
}

/// A type with classes and enumerations by name, for linkage names.
fn type_name(p: &P, q: &QTy) -> String {
    let cv = format!("{}{}", if q.c { "const " } else { "" }, if q.v { "volatile " } else { "" });
    let t = match &q.t {
        Ty::Class(c) => p.class_label_of(*c),
        Ty::Enum(e) => p.enums[*e].name.clone(),
        Ty::Ptr(x) => format!("{}*", type_name(p, x)),
        Ty::LRef(x) => format!("{}&", type_name(p, x)),
        Ty::RRef(x) => format!("{}&&", type_name(p, x)),
        Ty::Array(x, n) => format!("{}[{}]", type_name(p, x), n.map(|n| n.to_string()).unwrap_or_default()),
        _ => show(&QTy::new(q.t.clone())),
    };
    format!("{}{}", cv, t)
}

/// A function or variable with C language linkage that is not `static`
/// (LSP-4).
fn is_c_name(p: &P, id: DeclId) -> bool {
    p.c_names.get(&p.decls[id].name).is_some_and(|v| v.contains(&id))
}
