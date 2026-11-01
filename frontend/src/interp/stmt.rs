// Statements on the abstract machine: blocks and the objects that end
// with them (OBJ-3), declarations, loops, `switch`, `goto`, and `return`.

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Seek {
    Case(i128),
    Default,
    Label(String),
}

impl<'a> Machine<'a> {
    /// Run a function body. The body is a block of its own.
    pub fn run_body(&mut self, body: &Stmt) -> R<Flow> {
        let mut seek = None;
        let r = self.exec(body, &mut seek)?;
        if let Flow::Goto(l) = &r {
            // A goto from the body's own block to a label in it.
            let mut sk = Some(Seek::Label(l.clone()));
            return self.exec(body, &mut sk);
        }
        Ok(r)
    }

    pub fn exec(&mut self, s: &Stmt, seek: &mut Option<Seek>) -> R<Flow> {
        self.step()?;
        if seek.is_some() {
            return self.exec_seek(s, seek);
        }
        match s {
            Stmt::Null | Stmt::Fallthrough(_) => Ok(Flow::Normal),
            // ATTR-10: an assumption that is false is undefined.
            Stmt::Assume(e) => {
                // ATTR-15: the expression is not evaluated, so its effects
                // do not happen. ATTR-10: if it would be false, the
                // behavior is undefined. The interpreter evaluates it and
                // then restores the objects.
                let saved = if self.constant { None } else { Some(self.objs.clone()) };
                let r = self.full(|m| m.eval_bool(e));
                if let Some(o) = saved {
                    self.objs = o;
                }
                match r {
                    Ok(false) => ub("ATTR-10", "assumption is false"),
                    _ => Ok(Flow::Normal),
                }
            }
            Stmt::Expr(e) => {
                self.full(|m| m.discard(e))?;
                Ok(Flow::Normal)
            }
            Stmt::Decl(ids) => {
                for &id in ids {
                    self.exec_decl(id)?;
                }
                Ok(Flow::Normal)
            }
            Stmt::Block(items) => self.in_block(|m| m.exec_items(items, seek)),
            Stmt::If { init, cond, then, els, kind } => self.in_block(|m| {
                if let Some(i) = init {
                    if let f @ (Flow::Return | Flow::Break | Flow::Continue | Flow::Goto(_)) = m.exec(i, seek)? {
                        return Ok(f);
                    }
                }
                let taken = match kind {
                    // IF-18: the then branch runs in constant evaluation,
                    // the else branch at run time (TR-5).
                    IfKind::Consteval(neg) => *neg != m.constant,
                    _ => m.eval_cond(cond.as_ref().unwrap())?,
                };
                if taken {
                    m.in_block(|m| m.exec(then, seek))
                } else if let Some(e) = els {
                    m.in_block(|m| m.exec(e, seek))
                } else {
                    Ok(Flow::Normal)
                }
            }),
            Stmt::While { cond, body } => loop {
                let r = self.in_block(|m| {
                    if !m.eval_cond(cond)? {
                        return Ok(None);
                    }
                    Ok(Some(m.in_block(|m| m.exec(body, seek))?))
                })?;
                match r {
                    None | Some(Flow::Break) => return Ok(Flow::Normal),
                    Some(f @ (Flow::Return | Flow::Goto(_))) => return Ok(f),
                    _ => {}
                }
            },
            Stmt::Do { body, cond } => loop {
                match self.in_block(|m| m.exec(body, seek))? {
                    Flow::Break => return Ok(Flow::Normal),
                    f @ (Flow::Return | Flow::Goto(_)) => return Ok(f),
                    _ => {}
                }
                if !self.full(|m| m.eval_bool(cond))? {
                    return Ok(Flow::Normal);
                }
            },
            Stmt::For { init, cond, inc, body } => self.in_block(|m| {
                if let Some(i) = init {
                    m.exec(i, seek)?;
                }
                loop {
                    let r = m.in_block(|m| {
                        if let Some(c) = cond {
                            if !m.eval_cond(c)? {
                                return Ok(None);
                            }
                        }
                        Ok(Some(m.in_block(|m| m.exec(body, seek))?))
                    })?;
                    match r {
                        None | Some(Flow::Break) => return Ok(Flow::Normal),
                        Some(f @ (Flow::Return | Flow::Goto(_))) => return Ok(f),
                        _ => {}
                    }
                    if let Some(i) = inc {
                        m.full(|m| m.discard(i))?;
                    }
                }
            }),
            Stmt::Switch { init, cond, body } => self.in_block(|m| {
                if let Some(i) = init {
                    m.exec(i, seek)?;
                }
                let v = match cond {
                    CondK::Expr(e) => m.full(|m| m.eval_int(e))?,
                    CondK::Decl(id, e) => {
                        m.exec_decl(*id)?;
                        m.full(|m| m.eval_int(e))?
                    }
                };
                let cases = collect_cases(body);
                let target = if cases.contains(&Some(v)) {
                    Seek::Case(v)
                } else if cases.contains(&None) {
                    Seek::Default
                } else {
                    return Ok(Flow::Normal);
                };
                let mut sk = Some(target);
                match m.exec(body, &mut sk)? {
                    Flow::Break => Ok(Flow::Normal),
                    f => Ok(f),
                }
            }),
            Stmt::Case(_, s) | Stmt::Default(s) | Stmt::Label(_, s) => self.exec(s, seek),
            Stmt::Break => Ok(Flow::Break),
            Stmt::Continue => Ok(Flow::Continue),
            Stmt::Return(e) => {
                if let Some(e) = e {
                    let (ret_ref, ret_class, slot) = {
                        let f = self.frame();
                        (f.ret_ref, f.ret_class, f.slot.clone())
                    };
                    // VC-9: the temporaries of the return expression end
                    // before the local variables.
                    let r = self.full(|m| {
                        if ret_ref && e.cat != Cat::PR {
                            Ok(Ret::Place(m.lvalue(&strip_bind(e))?))
                        } else if ret_ref && matches!(e.k, EK::Conv(Conv::Bind, _)) {
                            Ok(Ret::Place(m.bind_ref(e)?))
                        } else if ret_class {
                            let slot = slot.clone().unwrap();
                            let t = match m.frame().func.and_then(|f| m.p.func(f)) {
                                Some(f) => f.fty.ret.clone(),
                                None => e.ty.clone(),
                            };
                            m.init_into(&slot, e, &t)?;
                            Ok(Ret::InSlot)
                        } else if matches!(e.ty.t, Ty::Void) {
                            m.discard(e)?;
                            Ok(Ret::Val(Val::Void))
                        } else {
                            Ok(Ret::Val(m.eval(e)?))
                        }
                    })?;
                    self.frame_mut().ret = Some(r);
                }
                Ok(Flow::Return)
            }
            Stmt::Goto(l) => {
                if self.constant {
                    // CE-20: a goto in constant evaluation.
                    return limit("CE-20", "goto evaluated in constant evaluation");
                }
                Ok(Flow::Goto(l.clone()))
            }
            Stmt::Try(body, hs) => self.exec_try(body, hs, seek),
            Stmt::Unsupported(m) => unsup(m.clone()),
        }
    }

    /// Run `f` in a block of its own: the objects it declares end when it
    /// ends, in reverse order (OBJ-3).
    pub fn in_block<T>(&mut self, f: impl FnOnce(&mut Self) -> R<T>) -> R<T> {
        self.blocks.push(Vec::new());
        self.block_item.push(0);
        let r = f(self);
        let l = self.blocks.pop().unwrap();
        self.block_item.pop();
        match r {
            Ok(v) => {
                self.cleanup(l)?;
                Ok(v)
            }
            // UNW-1: an exception ends the automatic objects of the block.
            Err(e) => Err(self.unwind(l, e)),
        }
    }

    /// A try block (TRY-1): the first handler that matches an exception
    /// that leaves the block runs.
    fn exec_try(&mut self, body: &Stmt, hs: &[Handler], seek: &mut Option<Seek>) -> R<Flow> {
        match self.in_block(|m| m.exec(body, seek)) {
            Err(Stop::Throw(x)) => self.handle(x, hs, seek),
            r => r,
        }
    }

    /// Run the handler of `hs` that matches the exception `x`, or pass the
    /// exception on.
    pub fn handle(&mut self, x: Exc, hs: &[Handler], seek: &mut Option<Seek>) -> R<Flow> {
        for h in hs {
            if !self.handler_matches(h.ty.as_ref(), &x.ty) {
                continue;
            }
            self.caught.push(x.clone());
            let r = self.in_block(|m| {
                if let (Some(var), Some(ht)) = (h.var, &h.ty) {
                    m.bind_handler(var, ht, &x)?;
                }
                m.exec(&h.body, seek)
            });
            self.caught.pop();
            return match r {
                // TRY-8, THR-6: a rethrow passes on the same object.
                Err(Stop::Throw(y)) if y.obj == x.obj => Err(Stop::Throw(y)),
                // THR-8: the exception object ends with the handler. A new
                // exception from the handler ends it too.
                Err(e @ Stop::Throw(_)) => {
                    self.end_exception(&x)?;
                    Err(e)
                }
                Err(e) => Err(e),
                Ok(f) => {
                    // THR-12: a throwing destructor of the exception object
                    // leaves from the end of the handler.
                    self.end_exception(&x)?;
                    Ok(f)
                }
            };
        }
        Err(Stop::Throw(x))
    }

    /// End an exception object whose last handler ended (THR-8).
    fn end_exception(&mut self, x: &Exc) -> R<()> {
        if self.caught.iter().any(|c| c.obj == x.obj) || !self.objs[x.obj].alive {
            return Ok(());
        }
        let p = Ptr::whole(x.obj);
        self.destroy(&p, &x.ty)?;
        self.end_obj(x.obj);
        Ok(())
    }

    /// TRY-2: whether a handler for type `h` matches an exception object
    /// of type `e`. None is `...`.
    pub fn handler_matches(&self, h: Option<&QTy>, e: &QTy) -> bool {
        let Some(h) = h else { return true };
        let t = decay(&nonref(h));
        match (&t.t, &e.t) {
            (Ty::Class(tc), Ty::Class(ec)) => tc == ec || self.public_unique_base(*ec, *tc),
            (Ty::Ptr(tp), Ty::Ptr(ep)) => {
                let cv_ok = (tp.c || !ep.c) && (tp.v || !ep.v);
                if !cv_ok {
                    return false;
                }
                match (&tp.t, &ep.t) {
                    (Ty::Class(tc), Ty::Class(ec)) => tc == ec || self.public_unique_base(*ec, *tc),
                    // TRY-2: a pointer to an object converts to a
                    // pointer to `void` with the same or more qualifiers.
                    (Ty::Void, b) => !matches!(b, Ty::Func(_)),
                    (a, b) => a == b,
                }
            }
            (Ty::Ptr(_), Ty::NullPtr) => true,
            _ => same_unqual(&t, e),
        }
    }

    /// TRY-2: `to` is an accessible, unambiguous base of `from`. Access is
    /// judged as public along each path.
    fn public_unique_base(&self, from: usize, to: usize) -> bool {
        fn count(m: &Machine, c: usize, to: usize, public: bool, n: &mut (u32, bool), depth: u32) {
            if depth > 32 {
                return;
            }
            for b in &m.p.classes[c].bases {
                let Some(bc) = b.class else { continue };
                let p = public && b.access == 0;
                if bc == to {
                    n.0 += 1;
                    n.1 |= p;
                } else {
                    count(m, bc, to, p, n, depth + 1);
                }
            }
        }
        let mut n = (0, false);
        count(self, from, to, true, &mut n, 0);
        n.0 == 1 && n.1
    }

    /// Initialize a handler's parameter from the exception object (TRY-2).
    /// A copy that throws ends the program (TRY-18).
    fn bind_handler(&mut self, var: DeclId, ht: &QTy, x: &Exc) -> R<()> {
        let src = Ptr::whole(x.obj);
        if ht.t.is_ref() {
            let to = nonref(ht);
            let p = self.class_cast(src, &x.ty.t, &to.t)?;
            self.frame_mut().refs.insert(var, p);
            return Ok(());
        }
        let t = decay(ht).unqual();
        let place = self.alloc(&t)?;
        self.frame_mut().vars.insert(var, place.obj);
        let r = match &t.t {
            Ty::Class(c) => {
                let p = self.class_cast(src, &x.ty.t, &t.t)?;
                self.copy_class(&place, &p, *c, false)
            }
            _ => {
                let v = self.read(&src, false)?;
                let v = match (&x.ty.t, &t.t, v) {
                    (Ty::Ptr(a), Ty::Ptr(b), Val::Ptr(p)) => Val::Ptr(self.class_cast(p, &a.t, &b.t)?),
                    (_, Ty::Ptr(_), Val::Int(0)) => Val::Null,
                    (_, _, v) => v,
                };
                self.write(&place, v, true)
            }
        };
        match r {
            Err(Stop::Throw(_)) => return Err(Stop::Terminate("TRY-18", "the copy that initializes a handler parameter throws".into())),
            r => r?,
        }
        self.register_block(&place, &t);
        Ok(())
    }

    /// A function body with a function try block (TRY-9, TRY-10). For a
    /// constructor or destructor (`rethrow`), reaching the end of a
    /// handler rethrows.
    pub fn run_fn_try(&mut self, body: &Stmt, hs: &[Handler], rethrow: bool) -> R<Flow> {
        let mut seek = None;
        match self.run_body(body) {
            Err(Stop::Throw(x)) => {
                let r = self.handle_fn(x, hs, &mut seek, rethrow);
                r
            }
            r => r,
        }
    }

    pub fn handle_fn(&mut self, x: Exc, hs: &[Handler], seek: &mut Option<Seek>, rethrow: bool) -> R<Flow> {
        if !rethrow {
            return self.handle(x, hs, seek);
        }
        // TRY-10: the handler ends by rethrowing, unless it throws.
        for h in hs {
            if !self.handler_matches(h.ty.as_ref(), &x.ty) {
                continue;
            }
            self.caught.push(x.clone());
            let r = self.in_block(|m| {
                if let (Some(var), Some(ht)) = (h.var, &h.ty) {
                    m.bind_handler(var, ht, &x)?;
                }
                m.exec(&h.body, seek)
            });
            self.caught.pop();
            return match r {
                Ok(_) => Err(Stop::Throw(x)),
                Err(Stop::Throw(y)) if y.obj == x.obj => Err(Stop::Throw(y)),
                Err(e @ Stop::Throw(_)) => {
                    self.end_exception(&x)?;
                    Err(e)
                }
                Err(e) => Err(e),
            };
        }
        Err(Stop::Throw(x))
    }

    fn exec_items(&mut self, items: &[Stmt], seek: &mut Option<Seek>) -> R<Flow> {
        let mut i = 0;
        while i < items.len() {
            *self.block_item.last_mut().unwrap() = i;
            let f = self.exec(&items[i], seek)?;
            match f {
                Flow::Normal => i += 1,
                Flow::Goto(l) => {
                    // A label in this block: end the objects declared at or
                    // after the label's statement (JMP rules), and continue
                    // there.
                    match items.iter().position(|s| has_label(s, &l)) {
                        Some(k) => {
                            let list = self.blocks.last_mut().unwrap();
                            let mut ended = Vec::new();
                            list.retain(|c| {
                                if c.item >= k {
                                    ended.push(c.clone());
                                    false
                                } else {
                                    true
                                }
                            });
                            self.cleanup(ended)?;
                            *seek = Some(Seek::Label(l));
                            i = k;
                        }
                        None => return Ok(Flow::Goto(l)),
                    }
                }
                f => return Ok(f),
            }
        }
        Ok(Flow::Normal)
    }

    /// Skip statements until the label that the jump looks for.
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
            Stmt::Label(l, inner) => {
                if *seek == Some(Seek::Label(l.clone())) {
                    *seek = None;
                    return self.exec(inner, seek);
                }
                self.exec_seek(inner, seek)
            }
            Stmt::Block(items) => self.in_block(|m| {
                let start = items.iter().position(|it| m.seek_here(it, seek)).unwrap_or(items.len());
                for it in &items[..start] {
                    m.skip_decl(it)?;
                }
                if start == items.len() {
                    return Ok(Flow::Normal);
                }
                m.exec_items(&items[start..], seek)
            }),
            Stmt::Decl(_) => {
                self.skip_decl(s)?;
                Ok(Flow::Normal)
            }
            Stmt::If { .. } | Stmt::While { .. } | Stmt::For { .. } | Stmt::Do { .. } | Stmt::Switch { .. } => {
                if self.seek_here(s, seek) {
                    return unsup("jump into a nested statement");
                }
                Ok(Flow::Normal)
            }
            _ => Ok(Flow::Normal),
        }
    }

    fn seek_here(&self, s: &Stmt, seek: &Option<Seek>) -> bool {
        match seek {
            Some(Seek::Label(l)) => has_label(s, l),
            Some(Seek::Case(v)) => collect_cases(s).contains(&Some(*v)),
            Some(Seek::Default) => collect_cases(s).contains(&None),
            None => false,
        }
    }

    /// A declaration that a jump skips: the variables exist with no value.
    fn skip_decl(&mut self, s: &Stmt) -> R<()> {
        if let Stmt::Decl(ids) = s {
            for &id in ids {
                if let Some(v) = self.p.var(id) {
                    if v.storage == Storage::Auto && !v.ty.t.is_ref() {
                        let v = v.clone();
                        let place = self.alloc(&v.ty)?;
                        self.frame_mut().vars.insert(id, place.obj);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn eval_cond(&mut self, c: &CondK) -> R<bool> {
        match c {
            CondK::Expr(e) => self.full(|m| m.eval_bool(e)),
            CondK::Decl(id, e) => {
                self.exec_decl(*id)?;
                self.full(|m| m.eval_bool(e))
            }
        }
    }

    fn register_block(&mut self, place: &Ptr, ty: &QTy) {
        let item = self.block_item.last().copied().unwrap_or(0);
        if let Some(l) = self.blocks.last_mut() {
            l.push(Cleanup { place: place.clone(), ty: ty.clone(), item });
        }
    }

    pub fn exec_decl(&mut self, id: DeclId) -> R<()> {
        let Some(v) = self.p.var(id) else {
            return Ok(());
        };
        let v = v.clone();
        if v.dependent {
            if self.constant {
                return Ok(()); // structured bindings: reads are unsupported
            }
            return unsup("structured binding or init capture");
        }
        if v.storage != Storage::Auto {
            if self.constant {
                if v.constexpr {
                    return Ok(());
                }
                // CE-20: a static or thread_local variable.
                return limit("CE-20", "definition of a static or thread_local variable in constant evaluation");
            }
            if v.storage == Storage::Extern {
                return Ok(());
            }
            return self.local_static(id, &v);
        }
        if self.constant && self.p.literal_type(&v.ty) == Some(false) {
            // CE-20: no variable of a type that is not literal.
            return limit("CE-20", "definition of a variable of a type that is not a literal type");
        }
        if v.ty.t.is_ref() {
            let Some(init) = &v.init else { return unsup("reference without initializer") };
            let init = init.clone();
            // RFOR-4: every temporary of the range expression of a
            // range-based for lives until the loop ends.
            let range_var = self.p.decls[id].name == " range";
            let pl = self.full(|m| {
                let pl = m.bind_ref(&init)?;
                // REF-4: a temporary bound to the reference lives as long
                // as the reference.
                if range_var {
                    let all = std::mem::take(m.temps.last_mut().unwrap());
                    for c in all {
                        m.register_block(&c.place, &c.ty);
                    }
                } else if binds_temporary_p(m.p, &init) {
                    if let Some(c) = m.extend_temp(pl.obj) {
                        m.register_block(&c.place, &c.ty);
                    }
                }
                Ok(pl)
            })?;
            self.frame_mut().refs.insert(id, pl);
            return Ok(());
        }
        // NAM-1: the variable is in scope in its own initializer, without
        // a value yet (INIT-1).
        let place = self.alloc(&v.ty.unqual())?;
        self.objs[place.obj].volatile = v.ty.v;
        self.frame_mut().vars.insert(id, place.obj);
        match &v.init {
            Some(init) => {
                let init = init.clone();
                let mark = self.extended.len();
                self.full(|m| {
                    m.init_into(&place, &init, &v.ty)?;
                    // REF-4: a temporary that a reference member of the
                    // aggregate binds lives as long as the variable.
                    for c in m.extended.split_off(mark) {
                        m.register_block(&c.place, &c.ty);
                    }
                    Ok(())
                })?;
            }
            None => self.full(|m| m.default_init(&place, &v.ty))?,
        }
        // A class can have mutable members, so only scalars and arrays of
        // scalars are marked.
        self.objs[place.obj].konst = v.ty.c && crate::decl2::elem_class(&v.ty.t).is_none();
        self.register_block(&place, &v.ty);
        Ok(())
    }

    /// A block-scope static variable: initialized when control first
    /// reaches it (OBJ-4), ended after `main` returns.
    fn local_static(&mut self, id: DeclId, v: &VarInfo) -> R<()> {
        let key = (self.tu, id);
        if self.globals.contains_key(&key) || self.global_refs.contains_key(&key) {
            if self.statics_running.contains(&key) {
                // STO-9: control reaches the declaration again while its
                // initialization runs.
                return ub("STO-9", "recursive initialization of a block-scope static variable");
            }
            return Ok(());
        }
        self.statics_running.insert(key);
        let r = self.init_static(id, v);
        self.statics_running.remove(&key);
        if let Err(Stop::Throw(_)) = &r {
            // UNW-4: the variable is not initialized, and the next pass
            // tries again.
            if let Some(o) = self.globals.remove(&key) {
                self.end_obj(o);
            }
            self.global_refs.remove(&key);
        }
        r
    }

    /// Initialize a variable with static storage duration and record its
    /// destruction.
    pub fn init_static(&mut self, id: DeclId, v: &VarInfo) -> R<()> {
        if v.ty.t.is_ref() {
            let Some(init) = &v.init else { return unsup("reference without initializer") };
            let init = init.clone();
            // Visible to a recursive use (STO-9) while it runs.
            let dummy = self.new_obj(Val::Uninit, true);
            self.globals.insert((self.tu, id), dummy);
            let pl = self.full(|m| {
                let pl = m.bind_ref(&init)?;
                if binds_temporary_p(m.p, &init) {
                    if let Some(c) = m.extend_temp(pl.obj) {
                        m.objs[c.place.obj].is_static = true;
                        m.static_dtors.push(c);
                    }
                }
                Ok(pl)
            })?;
            self.globals.remove(&(self.tu, id));
            self.global_refs.insert((self.tu, id), pl);
            return Ok(());
        }
        let place = match self.globals.get(&(self.tu, id)) {
            Some(&o) => Ptr::whole(o),
            None => {
                let place = self.alloc(&v.ty.unqual())?;
                // Zero-initialization comes first for a static object.
                let z = self.zero(&v.ty);
                self.write(&place, z, true)?;
                self.objs[place.obj].is_static = true;
                self.objs[place.obj].volatile = v.ty.v;
                self.globals.insert((self.tu, id), place.obj);
                place
            }
        };
        match &v.init {
            Some(init) => {
                let init = init.clone();
                let mark = self.extended.len();
                self.full(|m| {
                    m.init_into(&place, &init, &v.ty)?;
                    for c in m.extended.split_off(mark) {
                        m.objs[c.place.obj].is_static = true;
                        m.static_dtors.push(c);
                    }
                    Ok(())
                })?;
            }
            None => self.full(|m| m.default_init(&place, &v.ty))?,
        }
        self.objs[place.obj].konst = v.ty.c && crate::decl2::elem_class(&v.ty.t).is_none();
        self.static_dtors.push(Cleanup { place, ty: v.ty.clone(), item: 0 });
        Ok(())
    }
}

pub fn has_label(s: &Stmt, l: &str) -> bool {
    match s {
        Stmt::Label(x, i) => x == l || has_label(i, l),
        Stmt::Case(_, i) | Stmt::Default(i) => has_label(i, l),
        Stmt::Block(items) => items.iter().any(|it| has_label(it, l)),
        Stmt::If { then, els, .. } => has_label(then, l) || els.as_ref().is_some_and(|e| has_label(e, l)),
        Stmt::While { body, .. } | Stmt::Do { body, .. } | Stmt::For { body, .. } | Stmt::Switch { body, .. } => has_label(body, l),
        _ => false,
    }
}

pub fn collect_cases(s: &Stmt) -> Vec<Option<i128>> {
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

/// Whether a reference initializer binds a temporary that the binding
/// makes, or a member of one, so that the temporary lives as long as the
/// reference (REF-4, OBJ-8). A reference that a function returns does not
/// extend anything (REF-5).
pub fn binds_temporary_p(p: &P, e: &Expr) -> bool {
    binds_temporary_in(e, &|f| matches!(&p.decls[*f].k, DK::Field(fi) if fi.bits.is_some()))
}

fn binds_temporary_in(e: &Expr, bitfield: &dyn Fn(&DeclId) -> bool) -> bool {
    let binds_temporary = |x: &Expr| binds_temporary_in(x, bitfield);
    match &e.k {
        EK::Conv(Conv::Bind | Conv::NoOp, a) if e.cat != Cat::PR => binds_temporary(a),
        // BF-4: a bit-field binds a temporary that holds its value.
        EK::Member(_, f) if bitfield(f) => true,
        EK::Member(b, _) => binds_temporary(b),
        // An element of an array member of a temporary.
        EK::Index(a, _) => match &a.k {
            EK::Conv(Conv::ArrayDecay, x) => binds_temporary(x),
            _ => false,
        },
        EK::Cond(_, a, b) => binds_temporary(a) || binds_temporary(b),
        EK::Comma(_, b) => binds_temporary(b),
        _ => e.cat == Cat::PR,
    }
}

/// The type of a handler parameter or exception object: an array or
/// function type becomes a pointer (TRY-5, THR-2).
pub fn decay(t: &QTy) -> QTy {
    match &t.t {
        Ty::Array(el, _) => QTy::new(Ty::Ptr(el.clone())),
        Ty::Func(_) => QTy::new(Ty::Ptr(Box::new(t.clone()))),
        _ => t.clone(),
    }
}
