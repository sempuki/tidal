// Expressions on the abstract machine: places, values, conversions,
// operators, calls, and initialization.

use super::*;

impl<'a> Machine<'a> {
    // ---- variables ----

    pub fn var_place(&mut self, id: DeclId) -> R<Ptr> {
        if let Some(r) = self.frame().refs.get(&id) {
            return Ok(r.clone());
        }
        if let Some(&o) = self.frame().vars.get(&id) {
            return Ok(Ptr::whole(o));
        }
        // A declaration that names a definition in another unit.
        let key = self.links.get(&(self.tu, id)).copied().unwrap_or((self.tu, id));
        if let Some(r) = self.global_refs.get(&key) {
            return Ok(r.clone());
        }
        if let Some(&o) = self.globals.get(&key) {
            return Ok(Ptr::whole(o));
        }
        // A variable that a lambda captured: the member of the closure. A
        // member of reference type refers to the variable (LAM rules).
        if let Some(op) = self.frame().func {
            if let Some(caps) = self.p.closure_caps.get(&op) {
                if let Some(&(_, idx, _)) = caps.iter().find(|c| c.0 == id) {
                    let Some(this) = self.frame().this.clone() else { return unsup("closure without an object") };
                    let place = this.sub(idx);
                    let c = self.p.func(op).and_then(|f| f.class);
                    let is_ref = c.and_then(|c| self.fields(c).into_iter().find(|(_, fi)| fi.index == idx)).is_some_and(|(_, fi)| fi.ty.t.is_ref());
                    if is_ref {
                        return match self.get_at(&place)?.clone() {
                            Val::Ptr(q) => Ok(q),
                            _ => unsup("reference capture"),
                        };
                    }
                    return Ok(place);
                }
            }
        }
        let Some(v) = self.p.var(id) else {
            return unsup("reference to something that is not a variable");
        };
        let v = v.clone();
        if v.dependent {
            return unsup("value of a template parameter, structured binding, or init capture");
        }
        if !self.constant {
            if v.local && v.storage == Storage::Auto {
                return unsup("variable of an enclosing function (local class or lambda)");
            }
            if v.local {
                return unsup("block-scope static variable used before its declaration ran");
            }
            return unsup(format!("variable '{}' has no definition in this translation unit", self.p.decls[id].name));
        }
        self.const_var_place(id, v)
    }

    /// A variable from outside the constant evaluation (CE-3).
    fn const_var_place(&mut self, id: DeclId, v: VarInfo) -> R<Ptr> {
        let name = self.p.decls[id].name.clone();
        let usable_local = v.local && !v.param && v.init.is_some() && !v.ty.v && (v.constexpr || (v.ty.c && (v.ty.t.is_integral() || matches!(v.ty.t, Ty::Enum(_)))));
        if (v.param || (v.local && v.storage == Storage::Auto)) && !usable_local {
            // A variable of a caller that is not part of this evaluation.
            let o = self.new_obj(Val::NotUsable(format!("'{}' is not usable in constant expressions", name)), false);
            self.globals.insert((self.tu, id), o);
            return Ok(Ptr::whole(o));
        }
        // CE-3: usable if constexpr, or const integral/enum with a
        // constant initializer (CV-8).
        let usable_type = v.ty.c && !v.ty.v && (v.ty.t.is_integral() || matches!(v.ty.t, Ty::Enum(_)));
        let is_ref = v.ty.t.is_ref();
        let usable = (v.constexpr || usable_type || (is_ref && v.init.is_some())) && !v.ty.v;
        let val = if usable {
            match &v.init {
                Some(init) => {
                    let init = init.clone();
                    if is_ref {
                        // A reference: its object is the place of the initializer.
                        let local = v.local && v.storage == Storage::Auto;
                        let place = self.with_fresh_frame(|m| if local { m.full(|m| m.bind_ref(&init)) } else { m.bind_static_ref(&init) });
                        match place {
                            Ok(pl) => {
                                if !v.constexpr && !self.objs[pl.obj].is_static {
                                    return limit("CE-3", format!("reference '{}' is not usable in constant expressions", name));
                                }
                                self.global_refs.insert((self.tu, id), pl.clone());
                                return Ok(pl);
                            }
                            Err(Stop::Limit(..) | Stop::Undefined(..)) => return limit("CE-3", format!("'{}' is not initialized with a constant expression", name)),
                            Err(e) => return Err(e),
                        }
                    }
                    let is_static = !(v.local && v.storage == Storage::Auto);
                    let r = self.with_fresh_frame(|m| {
                        let place = if matches!(v.ty.t, Ty::Class(_) | Ty::Array(..)) { m.alloc(&v.ty.unqual())? } else { Ptr::whole(m.new_obj(Val::Uninit, true)) };
                        m.full(|m| {
                            m.init_into(&place, &init, &v.ty)?;
                            // CE-14, REF-4: the temporaries that reference
                            // members of a static aggregate bind are static.
                            if is_static {
                                for c in m.extended.split_off(0) {
                                    m.objs[c.place.obj].is_static = true;
                                }
                            }
                            Ok(())
                        })?;
                        Ok(m.objs[place.obj].val.clone())
                    });
                    match r {
                        Ok(x) => x,
                        Err(Stop::Limit(r, msg) | Stop::Undefined(r, msg)) => {
                            if v.constexpr {
                                return Err(Stop::Limit(r, msg));
                            }
                            Val::NotUsable(format!("'{}' has an initializer that is not a constant expression", name))
                        }
                        Err(e) => return Err(e),
                    }
                }
                None => {
                    if matches!(v.ty.t, Ty::Class(_)) {
                        return unsup("default-initialized class object (constructor evaluation)");
                    }
                    if v.class.is_some() || v.storage == Storage::Extern {
                        Val::NotUsable(format!("'{}' has no initializer here", name))
                    } else {
                        Val::NotUsable(format!("'{}' has no initializer", name))
                    }
                }
            }
        } else {
            let why = if v.ty.v {
                format!("'{}' is volatile", name)
            } else if v.ty.t.is_floating() && v.ty.c {
                format!("'{}' is a const floating-point variable, not constexpr (CV-8)", name)
            } else {
                format!("'{}' is not constexpr or a const integer with a constant initializer", name)
            };
            Val::NotUsable(why)
        };
        let o = self.new_obj(val, false);
        self.objs[o].is_static = true;
        self.objs[o].volatile = v.ty.v;
        self.objs[o].ty = Some(v.ty.clone());
        self.globals.insert((self.tu, id), o);
        Ok(Ptr::whole(o))
    }

    pub fn with_fresh_frame<T>(&mut self, f: impl FnOnce(&mut Self) -> R<T>) -> R<T> {
        self.frames.push(Frame::new());
        let r = f(self);
        self.frames.pop();
        r
    }

    // ---- glvalues ----

    /// Bind a reference with static storage duration. A temporary that it
    /// binds lives until the program ends and counts as static (REF-4,
    /// CE-14).
    pub fn bind_static_ref(&mut self, init: &Expr) -> R<Ptr> {
        self.full(|m| {
            let pl = m.bind_ref(init)?;
            if super::stmt::binds_temporary_p(m.p, init) {
                if let Some(c) = m.extend_temp(pl.obj) {
                    m.objs[c.place.obj].is_static = true;
                    if !m.constant {
                        m.static_dtors.push(c);
                    }
                }
            }
            Ok(pl)
        })
    }

    /// The place of a reference's initializer: a glvalue's place, or a
    /// temporary for a prvalue (REF-4).
    pub fn bind_ref(&mut self, init: &Expr) -> R<Ptr> {
        if let EK::Conv(Conv::Bind, a) = &init.k {
            // A reference to a base binds the base subobject (REF rules).
            let (from, to) = (nonref(&a.ty).t, nonref(&init.ty).t);
            if matches!((&from, &to), (Ty::Class(x), Ty::Class(y)) if x != y) {
                let p = self.bind_ref(a)?;
                return self.class_cast(p, &from, &to);
            }
        }
        let e = strip_bind(init);
        if e.cat == Cat::PR {
            return self.materialize(&e);
        }
        // BF-4: a reference to const that binds a bit-field binds a
        // temporary that holds its value.
        if let EK::Member(_, fid) = &e.k {
            if matches!(&self.p.decls[*fid].k, DK::Field(fi) if fi.bits.is_some()) {
                let p = self.lvalue(&e)?;
                let v = self.read(&p, e.ty.v)?;
                let o = self.new_obj(v, true);
                self.objs[o].ty = Some(e.ty.unqual());
                let t = Ptr::whole(o);
                self.add_temp(&t, &e.ty.unqual());
                return Ok(t);
            }
        }
        self.lvalue(&e)
    }

    /// Evaluate a glvalue to its place.
    pub fn lvalue(&mut self, e: &Expr) -> R<Ptr> {
        self.step()?;
        self.line = e.line;
        match &e.k {
            EK::Var(id) => self.var_place(*id),
            EK::Unary(UOp::Deref, a) => {
                let v = self.eval(a)?;
                match v {
                    Val::Ptr(p) => {
                        if p.array {
                            if let Ok(Val::Agg(items)) = self.get_at(&Ptr { array: false, off: 0, ..p.clone() }) {
                                if p.off < 0 || p.off as usize > items.len() {
                                    return ub("ADD-10", "pointer outside its array");
                                }
                            }
                        }
                        if !self.objs[p.obj].alive {
                            if self.resident_holder(p.obj).is_some() {
                                // STG-19: the old pointer does not reach the
                                // object that took the storage.
                                return ub("STG-19, OBJ-2", "indirection through a pointer to an object whose storage another object took");
                            }
                            return ub(self.ended_rule(true), "indirection through a pointer to an object whose lifetime has ended");
                        }
                        Ok(p)
                    }
                    // UN-7: the operand must point to an object.
                    Val::Null => ub("UN-7", "indirection through a null pointer"),
                    Val::FnPtr(..) => unsup("function lvalue"),
                    _ => unsup("indirection through a non-pointer"),
                }
            }
            EK::Unary(UOp::PreInc | UOp::PreDec, a) => {
                let place = self.lvalue(a)?;
                self.incdec(&place, a, matches!(e.k, EK::Unary(UOp::PreInc, _)), true)?;
                Ok(place)
            }
            EK::Assign(..) => self.assign(e),
            EK::Member(base, fid) => self.member_place(base, *fid),
            EK::Index(..) => {
                let p = self.index_place(e)?;
                // PFX-4: a subscript that reads or writes outside the array.
                if self.is_past_end(&p) {
                    return ub("PFX-4", "subscript outside the array");
                }
                Ok(p)
            }
            EK::Str(units) => Ok(self.string_obj(units)),
            EK::Comma(a, b) => {
                self.discard(a)?;
                self.lvalue(b)
            }
            EK::Cond(c, a, b) => {
                if self.eval_bool(c)? {
                    self.lvalue(a)
                } else {
                    self.lvalue(b)
                }
            }
            EK::Conv(k @ (Conv::Bind | Conv::NoOp | Conv::PtrConv), a) => {
                if a.cat == Cat::PR {
                    return self.materialize(a);
                }
                let p = self.lvalue(a)?;
                if *k == Conv::PtrConv {
                    return Ok(p);
                }
                // A derived-to-base or base-to-derived conversion of a glvalue.
                self.class_cast(p, &nonref(&a.ty).t, &nonref(&e.ty).t)
            }
            EK::Call(..) => match self.call(e, None)? {
                Ret::Place(p) => Ok(p),
                _ => unsup("call that does not return a reference"),
            },
            EK::DynamicCast(a) => {
                let p = self.lvalue(a)?;
                match self.dynamic_target(&p, &e.ty.t)? {
                    Some(q) => Ok(q),
                    // VIRT-10: the failed cast of a reference throws an
                    // exception of a library type; the interpreter gives
                    // it a type that only `catch (...)` matches.
                    None => {
                        if self.constant {
                            return limit("CE-8", "a failed dynamic_cast of a reference throws");
                        }
                        let place = self.alloc(&QTy::int())?;
                        self.write(&place, Val::Int(0), true)?;
                        Err(Stop::Throw(Exc { obj: place.obj, ty: QTy::new(Ty::Unknown) }))
                    }
                }
            }
            EK::This => unsup("this as lvalue"),
            EK::Unknown(why) if why.starts_with('\u{1}') => {
                let i: usize = why[1..].parse().unwrap_or(0);
                Ok(self.synth[i].clone())
            }
            EK::Unknown(why) => self.unknown(why),
            _ => {
                if e.cat == Cat::PR {
                    return self.materialize(e);
                }
                unsup(format!("lvalue of {:?}", std::mem::discriminant(&e.k)))
            }
        }
    }

    /// The place a subscript names, which can be one past the end.
    fn index_place(&mut self, e: &Expr) -> R<Ptr> {
        let EK::Index(a, i) = &e.k else { unreachable!() };
        // SEQ-3: the array before the index.
        let base = self.eval(a)?;
        let idx = self.eval_int(i)?;
        match base {
            Val::Ptr(p) => self.ptr_add(&p, idx, "PFX-4"),
            Val::Null => ub("UN-7", "subscript of a null pointer"),
            _ => unsup("subscript"),
        }
    }

    /// The object of a string literal. A run gives each set of characters
    /// one object (PRIM-9: whether two literals share an object is
    /// unspecified). Constant evaluation makes a new object each time.
    pub fn string_obj(&mut self, units: &[u32]) -> Ptr {
        if !self.constant {
            if let Some(&o) = self.strings.get(units) {
                return Ptr::whole(o);
            }
        }
        let o = self.new_obj(Val::Agg(units.iter().map(|&u| Val::Int(u as i128)).chain(std::iter::once(Val::Int(0))).collect()), false);
        self.objs[o].string = true;
        self.objs[o].is_static = true;
        self.objs[o].konst = true;
        if !self.constant {
            self.strings.insert(units.to_vec(), o);
        }
        Ptr::whole(o)
    }

    fn member_place(&mut self, base: &Expr, fid: DeclId) -> R<Ptr> {
        let fi = match &self.p.decls[fid].k {
            DK::Field(f) => f.clone(),
            _ => return unsup("member"),
        };
        let bp = if base.cat == Cat::PR { self.materialize(base)? } else { self.lvalue(base)? };
        if !bp.array && bp.off != 0 {
            return ub("UN-7", "member access through a pointer past the end");
        }
        self.class_ok(fi.class)?;
        let bc = match nonref(&base.ty).t {
            Ty::Class(c) => c,
            _ => fi.class,
        };
        let mut p = Ptr { obj: bp.obj, path: full_path(&bp), off: 0, array: false };
        if bc != fi.class {
            match self.base_path(bc, fi.class) {
                Some(bp) => p.path.extend(bp),
                None => return unsup("member of a class that is not a base"),
            }
        }
        p.path.push(self.nbases(fi.class) + fi.index);
        self.common_initial(&mut p, &fi);
        if fi.ty.t.is_ref() {
            return match self.get_at(&p)?.clone() {
                Val::Ptr(q) => Ok(q),
                Val::Uninit => ub("OBJ-2", "use of a reference member before its initialization"),
                Val::NotUsable(m) => limit("CE-3", m),
                _ => unsup("reference member"),
            };
        }
        Ok(p)
    }

    /// UNI-10: a member of the common initial sequence of two structs in
    /// a union can be read through the member that is not active. The
    /// place moves to the matching member of the active struct.
    fn common_initial(&self, p: &mut Ptr, fi: &FieldInfo) {
        if self.constant || p.path.len() < 2 {
            return;
        }
        let n = p.path.len();
        let holder = Ptr { obj: p.obj, path: p.path[..n - 1].to_vec(), off: 0, array: false };
        let union = Ptr { obj: p.obj, path: p.path[..n - 2].to_vec(), off: 0, array: false };
        if !matches!(self.get_at(&holder), Ok(Val::Inactive)) {
            return;
        }
        let Some(uc) = self.class_at(&union) else { return };
        if self.p.classes[uc].key != 2 {
            return;
        }
        let Ok(Val::Agg(items)) = self.get_at(&union) else { return };
        let Some(active) = items.iter().position(|v| !matches!(v, Val::Inactive)) else { return };
        let uf = self.fields(uc);
        let Some((_, af)) = uf.iter().find(|(_, x)| x.index == active - self.nbases(uc)) else { return };
        let Ty::Class(ac) = af.ty.t else { return };
        if ac == fi.class || !self.p.classes[ac].bases.is_empty() || !self.p.classes[fi.class].bases.is_empty() {
            return;
        }
        let (a, b) = (self.fields(ac), self.fields(fi.class));
        let common = a.iter().zip(b.iter()).take_while(|(x, y)| same_unqual(&x.1.ty, &y.1.ty) && x.1.bits == y.1.bits).count();
        if fi.index < common {
            p.path[n - 2] = active;
        }
    }

    /// A pointer or glvalue conversion between a class and its base.
    pub fn class_cast(&mut self, p: Ptr, from: &Ty, to: &Ty) -> R<Ptr> {
        let (Ty::Class(f), Ty::Class(t)) = (from, to) else { return Ok(p) };
        if f == t {
            return Ok(p);
        }
        if let Some(bp) = self.base_path(*f, *t) {
            let mut q = Ptr { obj: p.obj, path: full_path(&p), off: 0, array: false };
            if !p.array && p.off != 0 {
                return unsup("base conversion of a pointer past the end");
            }
            self.class_ok(*f)?;
            q.path.extend(bp);
            return Ok(q);
        }
        if let Some(bp) = self.base_path(*t, *f) {
            // CAST-14, DER-17: the object must be a base subobject of a `to`.
            let fp = full_path(&p);
            let foreign = self.objs[p.obj].tu != self.tu;
            let ok = fp.len() >= bp.len() && fp[fp.len() - bp.len()..] == bp[..] && (foreign || self.class_at(&Ptr { obj: p.obj, path: fp[..fp.len() - bp.len()].to_vec(), off: 0, array: false }) == Some(*t));
            if !ok {
                return ub("DER-17", "cast to a derived class of an object that is not a base subobject of that class");
            }
            return Ok(Ptr { obj: p.obj, path: fp[..fp.len() - bp.len()].to_vec(), off: 0, array: false });
        }
        Ok(p)
    }

    /// The result of a `dynamic_cast` of the object at `p` to class `to`,
    /// or `void` for the most derived object; None when the check fails
    /// (VIRT rules): the most derived object must contain one public
    /// subobject of class `to`. During a constructor or destructor the
    /// object counts as an object of that constructor's class.
    pub fn dynamic_target(&mut self, p: &Ptr, to: &Ty) -> R<Option<Ptr>> {
        let Some((mdc, mdpath)) = self.dynamic_class(p) else {
            return unsup("dynamic type of an object the interpreter cannot see");
        };
        let at = |path: Vec<usize>| Ptr { obj: p.obj, path, off: 0, array: false };
        // The classes along the path from the most derived object to the
        // operand's subobject, and whether each edge is public.
        let fp = full_path(p);
        let mut chain: Vec<(usize, bool)> = vec![(mdc, true)];
        {
            let mut c = mdc;
            for &i in fp.iter().skip(mdpath.len()) {
                let Some(b) = self.p.classes[c].bases.get(i) else { break };
                let Some(bc) = b.class else { break };
                chain.push((bc, b.access == 0));
                c = bc;
            }
        }
        if let Ty::Class(tc) = to {
            // VIRT-10, step 1: the operand is a public base subobject of
            // exactly one `T` object on its path.
            let mut hits = Vec::new();
            for k in 0..chain.len().saturating_sub(1) {
                if chain[k].0 == *tc && chain[k + 1..].iter().all(|x| x.1) {
                    hits.push(k);
                }
            }
            if hits.len() == 1 {
                let k = hits[0];
                let mut path = mdpath.clone();
                path.extend(fp.iter().skip(mdpath.len()).take(k));
                return Ok(Some(at(path)));
            }
        }
        if !matches!(to, Ty::Void) && !chain.iter().all(|x| x.1) {
            // Step 2 needs the operand reached through public bases only.
            return Ok(None);
        }
        match to {
            Ty::Void => Ok(Some(at(mdpath))),
            Ty::Class(tc) => {
                if *tc == mdc {
                    return Ok(Some(at(mdpath)));
                }
                let mut found = Vec::new();
                fn walk(m: &Machine, c: usize, tc: usize, path: &mut Vec<usize>, out: &mut Vec<Vec<usize>>, depth: u32) {
                    if depth > 32 {
                        return;
                    }
                    for (i, b) in m.p.classes[c].bases.iter().enumerate() {
                        let Some(bc) = b.class else { continue };
                        if b.access != 0 {
                            continue;
                        }
                        path.push(i);
                        if bc == tc {
                            out.push(path.clone());
                        } else {
                            walk(m, bc, tc, path, out, depth + 1);
                        }
                        path.pop();
                    }
                }
                walk(self, mdc, *tc, &mut vec![], &mut found, 0);
                if found.len() != 1 {
                    return Ok(None);
                }
                let mut path = mdpath;
                path.extend(found.pop().unwrap());
                Ok(Some(at(path)))
            }
            _ => unsup("dynamic_cast target"),
        }
    }

    /// The class of the subobject at a place, from the type of its
    /// complete object.
    pub fn class_at(&self, p: &Ptr) -> Option<usize> {
        let mut t = self.obj_ty(p.obj)?.t;
        for &i in &full_path(p) {
            t = match t {
                Ty::Array(el, _) => el.t,
                Ty::Class(c) => {
                    let nb = self.nbases(c);
                    if i < nb {
                        Ty::Class(self.p.classes[c].bases[i].class?)
                    } else {
                        let f = self.fields(c).into_iter().find(|(_, fi)| fi.index == i - nb)?;
                        f.1.ty.t
                    }
                }
                _ => return None,
            };
        }
        match t {
            Ty::Class(c) => Some(c),
            _ => None,
        }
    }

    /// The dynamic type of the object at a place: the most derived object
    /// that contains it as a base, or the class whose constructor or
    /// destructor runs (VIRT rules).
    pub fn dynamic_class(&self, p: &Ptr) -> Option<(usize, Vec<usize>)> {
        let fp = full_path(p);
        // A constructor or destructor that runs for this object or an
        // enclosing one.
        let mut best: Option<(usize, Vec<usize>)> = None;
        for (cp, c) in self.cdtor.iter() {
            let cpp = full_path(cp);
            if cp.obj == p.obj && fp.len() >= cpp.len() && fp[..cpp.len()] == cpp[..] && best.as_ref().is_none_or(|b| b.1.len() <= cpp.len()) {
                best = Some((*c, cpp));
            }
        }
        // The most derived object along the path.
        let mut t = self.obj_ty(p.obj)?.t;
        let mut md: Option<(usize, Vec<usize>)> = match t {
            Ty::Class(c) => Some((c, vec![])),
            _ => None,
        };
        for (k, &i) in fp.iter().enumerate() {
            t = match t {
                Ty::Array(el, _) => {
                    if let Ty::Class(c) = el.t {
                        md = Some((c, fp[..=k].to_vec()));
                    }
                    el.t
                }
                Ty::Class(c) => {
                    let nb = self.nbases(c);
                    if i < nb {
                        Ty::Class(self.p.classes[c].bases[i].class?)
                    } else {
                        let f = self.fields(c).into_iter().find(|(_, fi)| fi.index == i - nb)?;
                        if let Ty::Class(mc) = f.1.ty.t {
                            md = Some((mc, fp[..=k].to_vec()));
                        }
                        f.1.ty.t
                    }
                }
                _ => return None,
            };
        }
        match (best, md) {
            (Some(b), Some(m)) => Some(if b.1.len() >= m.1.len() { b } else { m }),
            (b, m) => b.or(m),
        }
    }

    pub fn unknown<T>(&self, why: &str) -> R<T> {
        if why.starts_with('\u{1}') {
            return unsup("synthetic place used as a value");
        }
        if self.constant {
            if let Some(rest) = why.strip_prefix("CE-4: ") {
                // CE-4: reinterpret_cast and void* to object pointer.
                return limit("CE-4", rest.to_string());
            }
        }
        if let Some(rest) = why.strip_prefix("LAM-19: ") {
            return ub("LAM-19", rest.to_string());
        }
        unsup(why.to_string())
    }

    // ---- prvalues ----

    pub fn eval_int(&mut self, e: &Expr) -> R<i128> {
        match self.eval(e)? {
            Val::Int(v) => Ok(v),
            Val::Float(_) => unsup("float where integer expected"),
            _ => unsup("non-integer value"),
        }
    }

    pub fn eval_bool(&mut self, e: &Expr) -> R<bool> {
        match self.eval(e)? {
            Val::Int(v) => Ok(v != 0),
            Val::Float(f) => Ok(f != 0.0),
            Val::Null => Ok(false),
            Val::Ptr(_) | Val::FnPtr(..) => Ok(true),
            _ => unsup("non-scalar condition"),
        }
    }

    /// Evaluate an expression and drop its value.
    pub fn discard(&mut self, e: &Expr) -> R<()> {
        if e.cat != Cat::PR {
            let p = self.lvalue(e)?;
            // UNEV-2: a discarded name, member access, subscript, or
            // indirection that refers to a volatile object reads it.
            let mut x = e;
            while let EK::Comma(_, b) = &x.k {
                x = b;
            }
            if e.ty.v && !matches!(e.ty.t, Ty::Class(_) | Ty::Array(..)) && matches!(x.k, EK::Var(_) | EK::Member(..) | EK::Index(..) | EK::Unary(UOp::Deref, _)) {
                self.read(&p, true)?;
            }
            return Ok(());
        }
        if matches!(e.ty.t, Ty::Class(_)) {
            // VC-5: a discarded class prvalue makes a temporary.
            self.materialize(e)?;
            return Ok(());
        }
        self.eval(e)?;
        Ok(())
    }

    /// Evaluate a prvalue, or read a glvalue.
    pub fn eval(&mut self, e: &Expr) -> R<Val> {
        self.step()?;
        self.line = e.line;
        match &e.k {
            EK::Int(v) => Ok(Val::Int(*v)),
            EK::Float(f) => Ok(Val::Float(*f)),
            EK::NullPtr => Ok(Val::Null),
            EK::ZeroInit => Ok(self.zero(&e.ty)),
            EK::Str(_) => {
                let p = self.lvalue(e)?;
                Ok(Val::Ptr(p))
            }
            EK::Var(_) | EK::Member(..) | EK::Index(..) => {
                if e.cat == Cat::PR {
                    return unsup("prvalue member");
                }
                if let Ty::Class(c) = &e.ty.t {
                    if self.copy_runs_code(*c) {
                        return self.copy_to_temp(e, *c);
                    }
                }
                let p = self.lvalue(e)?;
                self.read(&p, e.ty.v)
            }
            EK::Func(f) => Ok(Val::FnPtr(self.tu, *f)),
            EK::This => match self.frame().this.clone() {
                // LAM rules: `this` in a lambda is the captured `this`.
                Some(p) if self.frame().func.is_some_and(|f| self.p.closure_this.contains_key(&f)) => {
                    let (idx, copy) = self.p.closure_this[&self.frame().func.unwrap()];
                    let place = p.sub(idx);
                    if copy {
                        return Ok(Val::Ptr(place));
                    }
                    self.read(&place, false)
                }
                Some(p) => Ok(Val::Ptr(p)),
                None => {
                    if self.constant {
                        return limit("CE-3", "'this' of an object outside the evaluation");
                    }
                    unsup("'this' outside a member function")
                }
            },
            EK::Conv(k, a) => self.eval_conv(*k, a, &e.ty),
            EK::Unary(op, a) => self.eval_unary(*op, a, e),
            EK::Binary(op, a, b) => self.eval_binary(*op, a, b, &e.ty),
            EK::Assign(_, l, _) => {
                let p = self.assign(e)?;
                if matches!(l.ty.t, Ty::Class(_)) {
                    return Ok(self.get_at(&p)?.clone());
                }
                self.read(&p, l.ty.v)
            }
            EK::Cond(c, a, b) => {
                // COND-7: only the selected operand is evaluated.
                if self.eval_bool(c)? {
                    self.eval(a)
                } else {
                    self.eval(b)
                }
            }
            EK::Comma(a, b) => {
                self.discard(a)?;
                self.eval(b)
            }
            EK::Call(..) => {
                if matches!(e.ty.t, Ty::Class(_)) && e.cat == Cat::PR {
                    let p = self.materialize(e)?;
                    return Ok(self.get_at(&p)?.clone());
                }
                match self.call(e, None)? {
                    Ret::Val(v) => Ok(v),
                    Ret::Place(p) => self.read(&p, e.ty.v),
                    Ret::InSlot => unsup("class result without a slot"),
                }
            }
            EK::InitList(_) | EK::Construct(..) | EK::Closure(..) => {
                let p = self.materialize(e)?;
                Ok(self.get_at(&p)?.clone())
            }
            EK::Throw(a) => {
                if self.constant {
                    return limit("CE-8", "throw expression evaluated");
                }
                match a {
                    Some(a) => Err(Stop::Throw(self.make_exception(a)?)),
                    // TRY-8, THR-7: a rethrow with no exception being
                    // handled ends the program.
                    None => match self.caught.last() {
                        Some(x) => Err(Stop::Throw(x.clone())),
                        None => Err(Stop::Terminate("THR-7, TRY-8", "rethrow when no exception is being handled".into())),
                    },
                }
            }
            EK::DynamicCast(a) => {
                if e.cat != Cat::PR {
                    let p = self.lvalue(e)?;
                    return self.read(&p, e.ty.v);
                }
                match self.eval(a)? {
                    Val::Null => Ok(Val::Null),
                    Val::Ptr(p) => {
                        let to = match &e.ty.t {
                            Ty::Ptr(x) => x.t.clone(),
                            _ => return unsup("dynamic_cast target"),
                        };
                        Ok(match self.dynamic_target(&p, &to)? {
                            Some(q) => Val::Ptr(q),
                            None => Val::Null,
                        })
                    }
                    _ => unsup("dynamic_cast operand"),
                }
            }
            EK::Destroy(x) => {
                let mut p = self.lvalue(x)?;
                if !self.objs[p.obj].alive {
                    return ub("STG-22, OBJ-2", "destructor call on an object whose lifetime has ended");
                }
                let mut t = nonref(&x.ty);
                // A virtual destructor called through a base ends the
                // complete object.
                if let Ty::Class(c) = t.t {
                    let virt = self.p.classes[c].dtor.is_some_and(|d| self.is_virtual_fn(d));
                    if virt && !p.path.is_empty() && !p.array {
                        if let Some(ot) = self.obj_ty(p.obj) {
                            if matches!(ot.t, Ty::Class(_)) {
                                p = Ptr::whole(p.obj);
                                t = ot;
                            }
                        }
                    }
                }
                if self.constant && !self.objs[p.obj].own {
                    // CE-5: only an object that the evaluation created.
                    return limit("CE-5", "explicit destructor call on an object that the evaluation did not create");
                }
                self.destroy(&p, &t)?;
                if p.path.is_empty() && !p.array {
                    self.end_obj(p.obj);
                }
                Ok(Val::Void)
            }
            EK::New { ty, count, init, place: None } => self.eval_new(ty, count.as_deref(), init.as_deref()),
            EK::New { ty, count, init, place: Some(pl) } => self.eval_placement_new(ty, count.as_deref(), init.as_deref(), pl),
            EK::Delete(arr, x) => {
                self.eval_delete(*arr, x)?;
                Ok(Val::Void)
            }
            EK::Unknown(why) => self.unknown(why),
            EK::Overloads(_) | EK::BoundMember(..) | EK::TemplId(..) => unsup("overload set"),
        }
    }

    /// The exception object of `throw a` (THR-1, THR-2, THR-4): the static
    /// type of the operand without cv-qualifiers, an array or function as a
    /// pointer. A prvalue initializes the object directly, a local variable
    /// is moved, and another lvalue is copied.
    fn make_exception(&mut self, a: &Expr) -> R<Exc> {
        let mut x = a;
        while let EK::Conv(Conv::LToR | Conv::NoOp, b) = &x.k {
            if !matches!(b.ty.t, Ty::Class(_)) {
                break;
            }
            x = b;
        }
        let t = super::stmt::decay(&nonref(&x.ty)).unqual();
        if t.is_unknown() {
            return unsup("throw of an operand of unknown type");
        }
        let place = self.alloc(&t)?;
        match &t.t {
            Ty::Class(c) if x.cat != Cat::PR => {
                let src = self.lvalue(x)?;
                let local = matches!(&x.k, EK::Var(id) if self.p.var(*id).is_some_and(|v| v.local && v.storage == Storage::Auto && !v.ty.t.is_ref()));
                self.copy_class_cv(&place, &src, *c, local || x.cat == Cat::X, nonref(&x.ty).c)?;
            }
            Ty::Ptr(_) if matches!(nonref(&x.ty).t, Ty::Array(..) | Ty::Func(_)) => {
                let d = Expr::new(EK::Conv(if matches!(nonref(&x.ty).t, Ty::Func(_)) { Conv::FuncDecay } else { Conv::ArrayDecay }, Box::new(x.clone())), t.clone(), Cat::PR, x.line);
                let v = self.eval(&d)?;
                self.write(&place, v, true)?;
            }
            _ => self.init_into(&place, a, &t)?,
        }
        Ok(Exc { obj: place.obj, ty: t })
    }

    /// A `new` expression (NEW rules): a new object with dynamic storage
    /// duration, initialized from `init` or by default-initialization.
    fn eval_new(&mut self, ty: &QTy, count: Option<&Expr>, init: Option<&Expr>) -> R<Val> {
        let elem = ty.unqual();
        match count {
            None => {
                let place = self.alloc(&elem)?;
                self.objs[place.obj].heap = Heap::Single;
                if !matches!(elem.t, Ty::Class(_) | Ty::Array(..)) {
                    self.objs[place.obj].val = Val::Uninit;
                }
                // REF-5: a reference member of an aggregate from `new` does
                // not extend a temporary.
                let mark = self.extended.len();
                match init {
                    Some(e) => self.init_into(&place, e, &elem)?,
                    None => self.default_init(&place, &elem)?,
                }
                for c in self.extended.split_off(mark) {
                    self.add_temp(&c.place, &c.ty);
                }
                Ok(Val::Ptr(place))
            }
            Some(n) => {
                let n = match self.eval(n)? {
                    Val::Int(v) => v,
                    _ => return unsup("array size"),
                };
                if n < 0 {
                    return unsup("negative array size in new (an exception)");
                }
                let at = QTy::new(Ty::Array(Box::new(elem.clone()), Some(n as u64)));
                let place = self.alloc(&at)?;
                self.objs[place.obj].heap = Heap::Array;
                match init {
                    Some(e) => self.init_into(&place, e, &at)?,
                    None => self.default_init(&place, &at)?,
                }
                Ok(Val::Ptr(Ptr { obj: place.obj, path: vec![], off: 0, array: true }))
            }
        }
    }

    /// A global allocation or deallocation function called by name
    /// (STG-7, NEW-5, NEW-19): storage of bytes with no object yet.
    fn call_allocation(&mut self, name: &str, args: &[Expr]) -> R<Ret> {
        if self.constant {
            return limit("CE-2", "call of an allocation function");
        }
        if name.starts_with("operatornew") {
            let n = match self.eval(&args[0])? {
                Val::Int(v) => v,
                _ => return unsup("allocation size"),
            };
            if !(0..=1 << 20).contains(&n) {
                return unsup("allocation of a size the interpreter does not model");
            }
            let t = QTy::new(Ty::Array(Box::new(QTy::new(Ty::UChar)), Some(n.max(1) as u64)));
            let p = self.alloc(&t)?;
            self.objs[p.obj].heap = Heap::Raw;
            self.residents.entry(p.obj).or_default();
            return Ok(Ret::Val(Val::Ptr(Ptr { obj: p.obj, path: vec![], off: 0, array: true })));
        }
        let p = match self.eval(&args[0])? {
            Val::Null => return Ok(Ret::Val(Val::Void)),
            Val::Ptr(p) => p,
            _ => return unsup("deallocation of a value that is not a pointer"),
        };
        let host = match self.resident_of.get(&p.obj) {
            Some(&(h, 0)) => h,
            _ => p.obj,
        };
        if self.objs[host].heap != Heap::Raw && self.objs[host].heap == Heap::No {
            return ub("NEW-5", "deallocation of storage that an allocation function did not give");
        }
        if let Some(v) = self.residents.get(&host).cloned() {
            for (_, _, o) in v {
                self.objs[o].alive = false;
            }
        }
        self.objs[host].alive = false;
        Ok(Ret::Val(Val::Void))
    }

    /// STG-7, STG-18: an implicit-lifetime type, whose objects an
    /// allocation function creates as the program needs them.
    fn implicit_lifetime(&self, t: &Ty) -> bool {
        match t {
            Ty::Array(..) => true,
            Ty::Class(c) => {
                let ci = &self.p.classes[*c];
                if ci.has_virtual || ci.bases.iter().any(|b| b.virt) {
                    return false;
                }
                let dtor_ok = ci.dtor.is_none_or(|d| self.p.func(d).is_some_and(|f| f.defaulted));
                let aggregate = ci.ctors.is_empty();
                let trivial_ctor = ci.ctors.iter().any(|&k| self.p.func(k).is_some_and(|f| f.defaulted && f.params.is_empty()));
                dtor_ok && (aggregate || trivial_ctor)
            }
            Ty::Void | Ty::Func(_) => false,
            _ => true,
        }
    }

    /// A pointer into storage, converted to a pointer to `to`: the object
    /// of that type that lives there, or one that the storage creates
    /// implicitly (STG-7).
    pub fn storage_pointer(&mut self, p: Ptr, to: &QTy) -> R<Ptr> {
        if !self.residents.contains_key(&p.obj) || !p.path.is_empty() || !p.array {
            return Ok(p);
        }
        let off = p.off;
        let t = to.unqual();
        if let Some(v) = self.residents.get(&p.obj) {
            if let Some(&(_, _, o)) = v.iter().rev().find(|r| r.0 == off && self.objs[r.2].alive) {
                match self.obj_ty(o) {
                    Some(q) if q.unqual().t == t.t => return Ok(Ptr::whole(o)),
                    Some(QTy { t: Ty::Array(el, _), .. }) if el.unqual().t == t.t => return Ok(Ptr { obj: o, path: vec![], off: 0, array: true }),
                    _ => return Ok(p),
                }
            }
        }
        if self.objs[p.obj].heap == Heap::Raw && off == 0 && self.implicit_lifetime(&t.t) {
            // STG-8: the objects that give the program defined behavior.
            // An array of the type that fills the storage serves both one
            // object and arithmetic over several.
            let host_n = match self.obj_ty(p.obj) {
                Some(QTy { t: Ty::Array(_, Some(n)), .. }) => n,
                _ => return Ok(p),
            };
            if let Some(size) = self.p.sizeof_ty(&t.t).filter(|&s| s > 0) {
                let n = host_n / size;
                if n >= 1 {
                    let at = QTy::new(Ty::Array(Box::new(t.clone()), Some(n)));
                    let r = self.alloc(&at)?;
                    if !matches!(t.t, Ty::Class(_) | Ty::Array(..)) {
                        self.objs[r.obj].val = Val::Agg(vec![Val::Uninit; n as usize]);
                    }
                    self.residents.entry(p.obj).or_default().push((0, (n * size) as i64, r.obj));
                    self.resident_of.insert(r.obj, (p.obj, 0));
                    return Ok(Ptr { obj: r.obj, path: vec![], off: 0, array: true });
                }
            }
        }
        Ok(p)
    }

    /// The object that lives in the storage of the object `o`.
    pub fn resident_holder(&self, o: usize) -> Option<usize> {
        self.residents.get(&o)?.iter().rev().find(|r| self.objs[r.2].alive).map(|r| r.2)
    }

    /// The host object and byte offset of a pointer into a byte array
    /// that holds objects, or to such an object.
    pub fn byte_address(&self, p: &Ptr) -> Option<(usize, i64)> {
        if let Some(&(h, off)) = self.resident_of.get(&p.obj) {
            if p.path.is_empty() {
                let el = if p.array {
                    match self.obj_ty(p.obj) {
                        Some(QTy { t: Ty::Array(e, _), .. }) => self.p.sizeof_ty(&e.t)? as i64 * p.off,
                        _ => return None,
                    }
                } else {
                    0
                };
                return Some((h, off + el));
            }
            return None;
        }
        if self.residents.contains_key(&p.obj) && p.path.is_empty() && p.array {
            return Some((p.obj, p.off));
        }
        None
    }

    /// Placement `new` with the non-allocating form (STG-4, STG-5,
    /// STG-9, STG-17, STG-20).
    fn eval_placement_new(&mut self, ty: &QTy, count: Option<&Expr>, init: Option<&Expr>, place: &Expr) -> R<Val> {
        if self.constant {
            return limit("CE-4", "placement new");
        }
        let p = match self.eval(place)? {
            Val::Ptr(p) => p,
            Val::Null => return unsup("placement new at a null pointer"),
            _ => return unsup("placement new at a value that is not a pointer"),
        };
        let elem = ty.unqual();
        let t = match count {
            None => elem.clone(),
            Some(n) => {
                let n = match self.eval(n)? {
                    Val::Int(v) => v,
                    _ => return unsup("array size"),
                };
                if n < 0 {
                    return unsup("negative array size in new (an exception)");
                }
                QTy::new(Ty::Array(Box::new(elem.clone()), Some(n as u64)))
            }
        };
        let at = self.type_at(&p);
        let host = self.type_at(&Ptr { obj: p.obj, path: p.path.clone(), off: 0, array: false });
        let byte_host0 = matches!(&host, Some(QTy { t: Ty::Array(el, Some(_)), .. }) if matches!(el.t, Ty::UChar | Ty::Char) || matches!(&el.t, Ty::Enum(e) if self.p.enums[*e].name == "byte")) && p.array;
        let (size, align) = match (self.p.sizeof_ty(&t.t), self.p.alignof_ty(&t.t)) {
            (Some(s), Some(a)) => (s, a),
            _ if !byte_host0 => (0, 1),
            _ => return unsup("placement new of a type with a layout the interpreter does not compute"),
        };
        let byte_host = matches!(&host, Some(QTy { t: Ty::Array(el, Some(_)), .. }) if matches!(el.t, Ty::UChar | Ty::Char) || matches!(&el.t, Ty::Enum(e) if self.p.enums[*e].name == "byte")) && p.array;
        if byte_host {
            // STG-4, STG-17: the storage of a byte array, large enough and
            // aligned for the type.
            let Some(QTy { t: Ty::Array(_, Some(n)), .. }) = host else { return unsup("placement new") };
            let avail = n as i64 - p.off;
            if (size as i64) > avail {
                return ub("STG-17", format!("placement new of {} bytes in storage of {} bytes", size, avail.max(0)));
            }
            if p.off % align as i64 != 0 {
                return ub("STG-17", "placement new at an address that is not aligned for the type");
            }
            let r = self.alloc(&t)?;
            if !matches!(t.t, Ty::Class(_) | Ty::Array(..)) {
                self.objs[r.obj].val = Val::Uninit;
            }
            // A new object ends the objects whose storage it takes.
            let (lo, hi) = (p.off, p.off + size as i64);
            if let Some(v) = self.residents.get(&p.obj).cloned() {
                for (o, s2, obj) in v {
                    if o < hi && lo < o + s2 {
                        self.objs[obj].alive = false;
                    }
                }
            }
            self.residents.entry(p.obj).or_default().push((lo, size as i64, r.obj));
            self.resident_of.insert(r.obj, (p.obj, lo));
            let target = if count.is_some() { r.clone() } else { r.clone() };
            match init {
                Some(e) => self.init_into(&target, e, &t)?,
                None => self.default_init(&target, &t)?,
            }
            return Ok(Val::Ptr(if count.is_some() { Ptr { obj: r.obj, path: vec![], off: 0, array: true } } else { r }));
        }
        // STG-9: a new object of the same type in the place of an old one
        // replaces it.
        let Some(at) = at else { return unsup("placement new in storage of unknown type") };
        let whole0 = p.path.is_empty() && !p.array;
        if !p.path.is_empty() && !p.array && count.is_none() {
            let parent = Ptr { obj: p.obj, path: p.path[..p.path.len() - 1].to_vec(), off: 0, array: false };
            if let Some(QTy { t: Ty::Class(pc), .. }) = self.type_at(&parent) {
                if *p.path.last().unwrap() < self.nbases(pc) {
                    // STG-19: a new object in the place of a base subobject
                    // replaces nothing, and the enclosing object ends.
                    let r = self.alloc(&t)?;
                    self.objs[p.obj].alive = false;
                    self.residents.entry(p.obj).or_default().push((0, size as i64, r.obj));
                    self.resident_of.insert(r.obj, (p.obj, 0));
                    match init {
                        Some(e) => self.init_into(&r, e, &t)?,
                        None => self.default_init(&r, &t)?,
                    }
                    return Ok(Val::Ptr(r));
                }
            }
        }
        if whole0 && self.objs[p.obj].konst && self.objs[p.obj].heap == Heap::No {
            return ub("STG-20", "placement new in the storage of a const complete object");
        }
        if at.unqual().t != t.t || count.is_some() {
            if !whole0 || count.is_some() {
                return unsup("placement new of another type in the storage of an object");
            }
            if self.p.sizeof_ty(&at.t).is_some_and(|h| h < size) {
                return unsup("placement new of a larger type in the storage of an object");
            }
            // STG-19: a new object of another type; the old object ends and
            // its names keep the old object.
            let r = self.alloc(&t)?;
            if !matches!(t.t, Ty::Class(_) | Ty::Array(..)) {
                self.objs[r.obj].val = Val::Uninit;
            }
            self.objs[p.obj].alive = false;
            self.residents.entry(p.obj).or_default().push((0, size as i64, r.obj));
            self.resident_of.insert(r.obj, (p.obj, 0));
            match init {
                Some(e) => self.init_into(&r, e, &t)?,
                None => self.default_init(&r, &t)?,
            }
            return Ok(Val::Ptr(r));
        }
        let whole = p.path.is_empty() && !p.array;
        if whole && self.objs[p.obj].konst && self.objs[p.obj].heap == Heap::No {
            // STG-20: no new object in the storage of a const complete
            // object whose storage is not dynamic.
            return ub("STG-20", "placement new in the storage of a const complete object");
        }
        if whole {
            self.objs[p.obj].alive = true;
        }
        let blank = self.shape(&t)?;
        let blank = if matches!(t.t, Ty::Class(_) | Ty::Array(..)) { blank } else { Val::Uninit };
        self.write(&p, blank, true)?;
        match init {
            Some(e) => self.init_into(&p, e, &t)?,
            None => self.default_init(&p, &t)?,
        }
        Ok(Val::Ptr(p))
    }

    /// A `delete` or `delete[]` expression (NEW-5, VIRT-12).
    fn eval_delete(&mut self, arr: bool, x: &Expr) -> R<()> {
        let p = match self.eval(x)? {
            Val::Null => return Ok(()),
            Val::Ptr(p) => p,
            _ => return unsup("delete of a value that is not a pointer"),
        };
        let o = &self.objs[p.obj];
        if o.heap == Heap::No {
            return ub("NEW-5", "delete of a pointer that new did not give");
        }
        if !o.alive {
            return ub("NEW-5", "delete of an object that was already deleted");
        }
        let whole = if arr { p.array && p.off == 0 && p.path.is_empty() } else { !p.array && p.off == 0 };
        if (o.heap == Heap::Array) != arr {
            return ub("NEW-5", if arr { "delete[] of an object from new" } else { "delete of an array from new[]" });
        }
        if !whole {
            if arr {
                return ub("NEW-5", "delete[] through a pointer that is not the array's first element");
            }
            // A base subobject: the destructor must be virtual (VIRT-12).
            let t = self.obj_ty(p.obj);
            let base_ok = match (&t, self.class_at(&p)) {
                (Some(QTy { t: Ty::Class(_), .. }), Some(bc)) => self.p.classes[bc].dtor.is_some_and(|d| self.is_virtual_fn(d)),
                _ => false,
            };
            if !base_ok {
                return ub("VIRT-12", "delete through a pointer to a base whose destructor is not virtual, or to a member");
            }
        }
        if arr {
            if let Some(QTy { t: Ty::Array(el, _), .. }) = &self.obj_ty(p.obj) {
                if let Ty::Class(sc) = &nonref(&x.ty).t {
                    if let Ty::Ptr(pt) = &x.ty.t {
                        let _ = pt;
                    }
                    if el.t != Ty::Class(*sc) {
                        return ub("NEW-5", "delete[] through a pointer to a base of the element type");
                    }
                }
            }
        }
        // The object's destructor runs with the types of the unit that made
        // the object.
        let tu = self.objs[p.obj].tu;
        let t = self.objs[p.obj].ty.clone().unwrap_or(QTy::unknown());
        self.in_tu(tu, |m| m.destroy(&Ptr::whole(p.obj), &t))?;
        self.end_obj(p.obj);
        Ok(())
    }

    fn eval_conv(&mut self, k: Conv, a: &Expr, to: &QTy) -> R<Val> {
        match k {
            Conv::LToR => {
                if let Ty::Class(c) = &a.ty.t {
                    if self.copy_runs_code(*c) {
                        return self.copy_to_temp(a, *c);
                    }
                }
                let p = self.lvalue(a)?;
                self.read(&p, a.ty.v)
            }
            Conv::ArrayDecay => {
                let p = if a.cat == Cat::PR { self.materialize(a)? } else { self.lvalue(a)? };
                Ok(Val::Ptr(Ptr { obj: p.obj, path: full_path(&p), off: 0, array: true }))
            }
            Conv::FuncDecay => match &a.k {
                EK::Func(f) => Ok(Val::FnPtr(self.tu, *f)),
                _ => match self.eval(a)? {
                    v @ Val::FnPtr(..) => Ok(v),
                    _ => unsup("function decay"),
                },
            },
            Conv::ToVoid => {
                self.discard(a)?;
                Ok(Val::Void)
            }
            Conv::ToBool => {
                let v = self.eval(a)?;
                Ok(Val::Int(match v {
                    Val::Int(x) => (x != 0) as i128,
                    Val::Float(f) => (f != 0.0) as i128,
                    Val::Null => 0,
                    Val::Ptr(_) | Val::FnPtr(..) => 1,
                    _ => return unsup("conversion to bool"),
                }))
            }
            Conv::NullPtr => {
                self.eval(a)?;
                Ok(Val::Null)
            }
            Conv::PtrConv | Conv::NoOp => {
                if let Ty::Array(..) = to.t {
                    if a.cat != Cat::PR {
                        let p = self.lvalue(a)?;
                        return self.read(&p, false);
                    }
                    let p = self.materialize(&Expr::new(EK::Conv(k, Box::new(a.clone())), to.clone(), Cat::PR, a.line))?;
                    return Ok(self.get_at(&p)?.clone());
                }
                if a.cat != Cat::PR && !matches!(a.ty.t, Ty::Array(..)) {
                    return self.eval(a);
                }
                let v = self.eval(a)?;
                if let (Ty::Ptr(f), Ty::Ptr(t), Val::Ptr(p)) = (&a.ty.t, &to.t, &v) {
                    if let (Ty::Class(_), Ty::Class(_)) = (&f.t, &t.t) {
                        if !self.objs[p.obj].alive && !self.constant {
                            // STG-22: no cast to a derived class outside the
                            // lifetime.
                            return ub("STG-22", "cast of a pointer to an object whose lifetime has ended");
                        }
                        return Ok(Val::Ptr(self.class_cast(p.clone(), &f.t, &t.t)?));
                    }
                    if f.t == Ty::Void && t.t != Ty::Void {
                        if self.constant {
                            // CE-4: a pointer to void to a pointer to an
                            // object type.
                            return limit("CE-4", "conversion from void*");
                        }
                        return Ok(Val::Ptr(self.storage_pointer(p.clone(), t)?));
                    }
                }
                Ok(v)
            }
            Conv::Arith => {
                let v = self.eval(a)?;
                self.convert_val(v, &a.ty.t, &to.t)
            }
            Conv::Bind => {
                let p = self.bind_ref(a)?;
                self.read(&p, false)
            }
        }
    }

    /// Convert a scalar value between types (CONV-2, CONV-4, CONV-5).
    pub fn convert_val(&self, v: Val, from: &Ty, to: &Ty) -> R<Val> {
        let to_int = |t: &Ty| -> Option<Ty> {
            match t {
                Ty::Enum(e) => Some(self.p.enum_underlying(*e)),
                t if t.is_integral() => Some(t.clone()),
                _ => None,
            }
        };
        match v {
            Val::Int(x) => {
                if let Some(ti) = to_int(to) {
                    let r = wrap_int(x, &ti);
                    if let Ty::Enum(e) = to {
                        // ENUM-21: with a fixed underlying type the value
                        // converts to that type first.
                        let fixed = self.p.enums[*e].fixed.is_some() || self.p.enums[*e].scoped;
                        if !fixed && !matches!(from, Ty::Enum(f) if f == e) {
                            // ENUM-6, CAST-14: a value outside the range of
                            // an enumeration without a fixed type.
                            let (lo, hi) = self.p.enum_range(*e);
                            if x < lo || x > hi {
                                return ub("ENUM-6", format!("value {} is outside the range of the enumeration", x));
                            }
                            return Ok(Val::Int(x));
                        }
                    }
                    return Ok(Val::Int(r));
                }
                if to.is_floating() {
                    let f = x as f64;
                    return Ok(Val::Float(if *to == Ty::Float { f as f32 as f64 } else { f }));
                }
                Ok(Val::Int(x))
            }
            Val::Float(f) => {
                if let Some(ti) = to_int(to) {
                    if ti == Ty::Bool {
                        return Ok(Val::Int((f != 0.0) as i128));
                    }
                    let t = f.trunc();
                    if f.is_nan() || t < ti.int_min() as f64 || t > ti.int_max() as f64 {
                        return ub("CONV-5", "floating value out of the range of the integer type");
                    }
                    return Ok(Val::Int(t as i128));
                }
                if *to == Ty::Float {
                    let r = f as f32;
                    if r.is_infinite() && f.is_finite() {
                        if self.constant {
                            // CONV-11 is impl-defined; Clang and GCC reject in
                            // constant evaluation.
                            return limit("CONV-11", "floating conversion overflows the target type");
                        }
                        return unsup("floating conversion out of range (CONV-11, implementation-defined)");
                    }
                    return Ok(Val::Float(r as f64));
                }
                Ok(Val::Float(f))
            }
            v => Ok(v),
        }
    }

    /// Pointer arithmetic: `rule` names the operation for an error.
    pub fn ptr_add(&self, p: &Ptr, n: i128, rule: &'static str) -> R<Ptr> {
        if p.array {
            let base = Ptr { array: false, off: 0, ..p.clone() };
            let len = match self.get_at(&base)? {
                Val::Agg(items) => items.len() as i128,
                _ => return unsup("pointer arithmetic"),
            };
            let no = p.off as i128 + n;
            if no < 0 || no > len {
                // ADD-10: outside the array and its one-past position.
                return ub(rule, "pointer arithmetic outside the array");
            }
            Ok(Ptr { off: no as i64, ..p.clone() })
        } else {
            let no = p.off as i128 + n;
            if !(0..=1).contains(&no) {
                return ub(rule, "pointer arithmetic outside an object (an array of one)");
            }
            Ok(Ptr { off: no as i64, ..p.clone() })
        }
    }

    /// UNI-13: make the union members of a chain such as `s.u.a.tag` or
    /// `u.arr[1]` active, from the outside in. Only a chain of names,
    /// member accesses, and subscripts with no side effects.
    fn activate_chain(&mut self, l: &Expr) -> R<()> {
        fn plain(e: &Expr) -> bool {
            match &e.k {
                EK::Var(_) | EK::Int(_) => true,
                EK::Member(o, _) => plain(o),
                EK::Index(a, i) => plain(a) && plain(i),
                EK::Conv(Conv::ArrayDecay | Conv::LToR | Conv::NoOp | Conv::Arith, x) => plain(x),
                _ => false,
            }
        }
        if !plain(l) {
            return Ok(());
        }
        match &l.k {
            EK::Member(o, _) => {
                self.activate_chain(o)?;
                if matches!(o.k, EK::Member(..) | EK::Index(..)) {
                    let p = self.lvalue(o)?;
                    self.activate(&p)?;
                }
            }
            EK::Index(a, _) => {
                let mut x = &**a;
                while let EK::Conv(Conv::ArrayDecay | Conv::NoOp, y) = &x.k {
                    x = y;
                }
                if matches!(x.k, EK::Member(..)) {
                    self.activate_chain(x)?;
                    let p = self.lvalue(x)?;
                    self.activate(&p)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// UNI-5: an assignment to a member of a union makes that member
    /// active, also when the assignment names a member of the member, as
    /// in `u.a.tag = 7`. The other members end.
    pub fn activate(&mut self, place: &Ptr) -> R<()> {
        let fp = full_path(place);
        let mut t = self.obj_ty(place.obj).map(|q| q.t);
        for (k, &i) in fp.iter().enumerate() {
            let next = match &t {
                Some(Ty::Class(c)) => {
                    let c = *c;
                    let nb = self.nbases(c);
                    let ft = if i < nb { self.p.classes[c].bases[i].class.map(|b| QTy::new(Ty::Class(b))) } else { self.fields(c).into_iter().find(|(_, fi)| fi.index == i - nb).map(|x| x.1.ty) };
                    if self.p.classes[c].key == 2 {
                        let holder = Ptr { obj: place.obj, path: fp[..k].to_vec(), off: 0, array: false };
                        if let (Ok(Val::Agg(items)), Some(ft)) = (self.get_at(&holder), &ft) {
                            // UNI-13: only a member whose default constructor
                            // is trivial becomes active.
                            let trivial = match &ft.t {
                                Ty::Class(m) => self.trivial_default(*m, 0),
                                _ => true,
                            };
                            if matches!(items.get(i), Some(Val::Inactive)) && trivial {
                                let sh = if ft.t.is_ref() { Val::Uninit } else { self.shape(ft)? };
                                if let Val::Agg(items) = self.slot(&holder, true)? {
                                    for (j, it) in items.iter_mut().enumerate() {
                                        *it = if j == i { sh.clone() } else { Val::Inactive };
                                    }
                                }
                            }
                        }
                    }
                    ft.map(|q| q.t)
                }
                Some(Ty::Array(el, _)) => Some(el.t.clone()),
                _ => None,
            };
            t = next;
        }
        Ok(())
    }

    /// Whether the default constructor of a class is trivial: no
    /// constructor of its own, no default member initializer, no virtual
    /// function, and trivial bases and members.
    fn trivial_default(&self, c: usize, depth: u32) -> bool {
        let ci = &self.p.classes[c];
        if depth > 16 || !ci.ctors.is_empty() || ci.has_virtual {
            return false;
        }
        ci.bases.iter().all(|b| b.class.is_some_and(|bc| self.trivial_default(bc, depth + 1)))
            && ci.fields.iter().all(|&f| match &self.p.decls[f].k {
                DK::Field(fi) => {
                    fi.init.is_none()
                        && match &fi.ty.t {
                            Ty::Class(m) => self.trivial_default(*m, depth + 1),
                            _ => true,
                        }
                }
                _ => true,
            })
    }

    /// `++` and `--` on the object at `place`. Returns the old value.
    fn incdec(&mut self, place: &Ptr, a: &Expr, inc: bool, prefix: bool) -> R<Val> {
        let old = self.read(place, a.ty.v)?;
        let delta = if inc { 1 } else { -1 };
        let (rule, ptr_rule) = if prefix { ("UN-21", "ADD-10") } else { ("PFX-18", "PFX-18") };
        let nv = match &old {
            Val::Int(x) => {
                let t = match &a.ty.t {
                    Ty::Enum(e) => self.p.enum_underlying(*e),
                    t => t.clone(),
                };
                if t == Ty::Bool {
                    Val::Int(1)
                } else {
                    let r = x + delta;
                    // UN-21, PFX-18: a signed type that promotion does not
                    // change overflows.
                    if t.is_signed() && !fits(r, &t) && promote(&t) == t {
                        return ub(rule, "increment or decrement overflows");
                    }
                    Val::Int(wrap_int(r, &t))
                }
            }
            Val::Float(f) => {
                let r = f + delta as f64;
                Val::Float(if a.ty.t == Ty::Float { r as f32 as f64 } else { r })
            }
            Val::Ptr(p) => Val::Ptr(self.ptr_add(p, delta, ptr_rule)?),
            Val::Null => return ub(ptr_rule, "increment of a null pointer"),
            _ => return unsup("increment"),
        };
        let nv = self.bitfield_value(a, nv);
        self.write(place, nv, false)?;
        Ok(old)
    }

    /// BF-10: a bit-field keeps the low bits of the value.
    fn bitfield_value(&self, target: &Expr, v: Val) -> Val {
        if let EK::Member(_, fid) = &target.k {
            if let DK::Field(fi) = &self.p.decls[*fid].k {
                if let (Some(w), Val::Int(x)) = (fi.bits, &v) {
                    if w > 0 && w < 64 && fi.ty.t.is_integral() && fi.ty.t != Ty::Bool {
                        let m = 1i128 << w;
                        let mut y = x.rem_euclid(m);
                        if fi.ty.t.is_signed() && y >= m / 2 {
                            y -= m;
                        }
                        return Val::Int(y);
                    }
                }
            }
        }
        v
    }

    /// An assignment: returns the place of the left operand.
    pub fn assign(&mut self, e: &Expr) -> R<Ptr> {
        let EK::Assign(op, l, r) = &e.k else { unreachable!() };
        if let Ty::Class(c) = &l.ty.t {
            // The implicit copy or move assignment of a class.
            let c = *c;
            if op.is_some() {
                return unsup("compound assignment of a class");
            }
            // SEQ-3: the right operand first.
            let (src, mv) = if r.cat == Cat::PR { (self.materialize(r)?, true) } else { (self.lvalue(r)?, r.cat == Cat::X) };
            let src = self.class_cast(src, &nonref(&r.ty).t, &Ty::Class(c))?;
            let place = self.lvalue(l)?;
            self.activate(&place)?;
            self.assign_parts(&place, &src, c, mv)?;
            return Ok(place);
        }
        // SEQ-3: the right operand first, then the left.
        let rv = self.eval(r)?;
        if op.is_none() {
            // UNI-13: the union members on a chain of member accesses
            // and subscripts become active before the place is formed.
            self.activate_chain(l)?;
        }
        let place = self.lvalue(l)?;
        if op.is_none() {
            self.activate(&place)?;
        }
        let nv = match op {
            None => rv,
            Some((bop, ct)) => {
                let old = self.read(&place, l.ty.v)?;
                if let Val::Ptr(_) | Val::Null = old {
                    // `p += n` and `p -= n`.
                    let Val::Int(n) = rv else { return unsup("pointer compound assignment") };
                    let n = if *bop == BOp::Sub { -n } else { n };
                    match old {
                        Val::Ptr(p) => Val::Ptr(self.ptr_add(&p, n, "ADD-10")?),
                        _ if n == 0 => Val::Null,
                        _ => return ub("ADD-10", "arithmetic on a null pointer"),
                    }
                } else {
                    let lv = self.convert_val(old, &l.ty.t, &ct.t)?;
                    let res = self.arith(*bop, lv, rv, &ct.t, l.ty.t.clone())?;
                    self.convert_val(res, &ct.t, &l.ty.t)?
                }
            }
        };
        let nv = self.bitfield_value(l, nv);
        self.write(&place, nv, false)?;
        // UNI-5, CE-6: an assignment to a union member makes it active.
        // (A write to an inactive member reaches here only through a plain
        // member access, which `slot` refuses; the write below handles the
        // switch.)
        Ok(place)
    }

    fn eval_unary(&mut self, op: UOp, a: &Expr, e: &Expr) -> R<Val> {
        match op {
            UOp::Plus => self.eval(a),
            UOp::Neg => match self.eval(a)? {
                Val::Int(x) => {
                    let t = &e.ty.t;
                    if t.is_signed() {
                        if !fits(-x, t) {
                            return ub("UN-4", "negation overflows");
                        }
                        Ok(Val::Int(-x))
                    } else {
                        Ok(Val::Int(wrap_int(-x, t)))
                    }
                }
                Val::Float(f) => Ok(Val::Float(-f)),
                _ => unsup("negation"),
            },
            UOp::Not => Ok(Val::Int(!self.eval_bool(a)? as i128)),
            UOp::BitNot => {
                let x = self.eval_int(a)?;
                Ok(Val::Int(wrap_int(!x, &e.ty.t)))
            }
            UOp::Deref => {
                if matches!(e.ty.t, Ty::Func(_)) {
                    return self.eval(a);
                }
                let p = self.lvalue(e)?;
                self.read(&p, e.ty.v)
            }
            UOp::AddrOf => {
                if let EK::Func(f) = &a.k {
                    return Ok(Val::FnPtr(self.tu, *f));
                }
                if matches!(a.ty.t, Ty::Func(_)) {
                    return self.eval(a);
                }
                // `&a[n]` can form the pointer one past the end.
                let p = if matches!(a.k, EK::Index(..)) { self.index_place(a)? } else { self.lvalue(a)? };
                Ok(Val::Ptr(p))
            }
            UOp::PreInc | UOp::PreDec => {
                let place = self.lvalue(e)?;
                self.read(&place, a.ty.v)
            }
            UOp::PostInc | UOp::PostDec => {
                let place = self.lvalue(a)?;
                self.incdec(&place, a, op == UOp::PostInc, false)
            }
        }
    }

    fn eval_binary(&mut self, op: BOp, a: &Expr, b: &Expr, rt: &QTy) -> R<Val> {
        if op == BOp::LAnd || op == BOp::LOr {
            // LOG-4, LOG-7: the right operand only if needed.
            let l = self.eval_bool(a)?;
            if (op == BOp::LAnd && !l) || (op == BOp::LOr && l) {
                return Ok(Val::Int(l as i128));
            }
            return Ok(Val::Int(self.eval_bool(b)? as i128));
        }
        if !self.constant && !matches!(op, BOp::Shl | BOp::Shr) && has_effects(a) && has_effects(b) {
            // SEQ-8: the operands of a built-in operator are unsequenced.
            self.choice("SEQ-8", format!("the left operand of '{}' first", op.spelling()));
        }
        let l = self.eval(a)?;
        let r = self.eval(b)?;
        let ct = a.ty.t.clone();
        match (&l, &r) {
            (Val::Ptr(_) | Val::Null, _) | (_, Val::Ptr(_) | Val::Null) | (Val::FnPtr(..), _) | (_, Val::FnPtr(..)) => {
                return self.ptr_binary(op, l, r);
            }
            _ => {}
        }
        self.arith(op, l, r, &ct, rt.t.clone())
    }

    /// A key for the address that a pointer holds: the members of a union
    /// share one address (UNI-1), and a first member or element has the
    /// address of the object that contains it.
    pub fn addr_key(&self, p: &Ptr) -> Vec<usize> {
        let fp = full_path(p);
        let mut out = Vec::new();
        let mut t = self.obj_ty(p.obj).map(|q| q.t);
        for &i in &fp {
            let (step, next) = match &t {
                Some(Ty::Class(c)) => {
                    let c = *c;
                    let nb = self.nbases(c);
                    let next = if i < nb {
                        self.p.classes[c].bases[i].class.map(Ty::Class)
                    } else {
                        self.fields(c).into_iter().find(|(_, fi)| fi.index == i - nb).map(|x| x.1.ty.t)
                    };
                    (if self.p.classes[c].key == 2 { 0 } else { i }, next)
                }
                Some(Ty::Array(el, _)) => (i, Some(el.t.clone())),
                _ => (i, None),
            };
            out.push(step);
            t = next;
        }
        if !p.array && p.off != 0 {
            out.push(usize::MAX);
            return out;
        }
        while out.last() == Some(&0) {
            out.pop();
        }
        out
    }

    fn is_past_end(&self, p: &Ptr) -> bool {
        if p.array {
            matches!(self.get_at(&Ptr { array: false, off: 0, ..p.clone() }), Ok(Val::Agg(items)) if p.off as usize == items.len())
        } else {
            p.off == 1
        }
    }

    fn ptr_binary(&mut self, op: BOp, l: Val, r: Val) -> R<Val> {
        match op {
            BOp::Add | BOp::Sub => match (&l, &r) {
                (Val::Ptr(p), Val::Int(n)) => {
                    let n = if op == BOp::Sub { -n } else { *n };
                    Ok(Val::Ptr(self.ptr_add(p, n, "ADD-10")?))
                }
                (Val::Int(n), Val::Ptr(p)) if op == BOp::Add => Ok(Val::Ptr(self.ptr_add(p, *n, "ADD-10")?)),
                (Val::Null, Val::Int(n)) | (Val::Int(n), Val::Null) => {
                    if *n == 0 {
                        Ok(Val::Null)
                    } else {
                        ub("ADD-10", "arithmetic on a null pointer")
                    }
                }
                (Val::Ptr(p), Val::Ptr(q)) => {
                    if p.obj != q.obj || p.path != q.path || p.array != q.array {
                        return ub("ADD-11", "subtraction of pointers into different arrays");
                    }
                    Ok(Val::Int(p.off as i128 - q.off as i128))
                }
                (Val::Null, Val::Null) => Ok(Val::Int(0)),
                _ => unsup("pointer arithmetic"),
            },
            BOp::Eq | BOp::Ne => {
                let eq = match (&l, &r) {
                    (Val::Null, Val::Null) => true,
                    (Val::Null, _) | (_, Val::Null) => false,
                    (Val::FnPtr(a, x), Val::FnPtr(b, y)) => {
                        let ka = self.links.get(&(*a, *x)).copied().unwrap_or((*a, *x));
                        let kb = self.links.get(&(*b, *y)).copied().unwrap_or((*b, *y));
                        ka == kb
                    }
                    (Val::Ptr(p), Val::Ptr(q)) => {
                        let (ps, qs) = (self.objs[p.obj].string, self.objs[q.obj].string);
                        if ps && qs {
                            if self.constant && p.obj != q.obj {
                                return limit("CE-13", "comparison of the addresses of string literals");
                            }
                            if !self.constant {
                                self.choice("PRIM-9", "string literals with the same characters are one object");
                            }
                        }
                        if let (Some(a), Some(b)) = (self.byte_address(p), self.byte_address(q)) {
                            // STG-1: an object made in a byte array has the
                            // address of the bytes it occupies.
                            if self.residents.contains_key(&a.0) && (p.obj != q.obj) {
                                return Ok(Val::Int((if op == BOp::Eq { a == b } else { a != b }) as i128));
                            }
                        }
                        if p.obj != q.obj {
                            if self.is_past_end(p) || self.is_past_end(q) {
                                if self.constant {
                                    return limit("CMP-20", "comparison of a past-the-end pointer with a pointer to another object");
                                }
                                self.choice("CMP-20", "a past-the-end pointer compares unequal to a pointer to another object");
                            }
                            false
                        } else {
                            self.addr_key(p) == self.addr_key(q)
                        }
                    }
                    _ => return unsup("pointer comparison"),
                };
                Ok(Val::Int((if op == BOp::Eq { eq } else { !eq }) as i128))
            }
            BOp::Lt | BOp::Gt | BOp::Le | BOp::Ge => match (&l, &r) {
                (Val::Ptr(p), Val::Ptr(q)) => {
                    let (x, y) = if p.obj != q.obj {
                        if self.constant {
                            return limit("CE-15", "relational comparison of pointers to unrelated objects");
                        }
                        // CMP-8: the result is unspecified.
                        self.choice("CMP-8", "pointers to unrelated objects compare in the order the objects were created");
                        (vec![p.obj as i64], vec![q.obj as i64])
                    } else {
                        // CMP-7: subobjects of one object compare in order.
                        let mut x: Vec<i64> = p.path.iter().map(|&v| v as i64).collect();
                        x.push(if p.array { p.off } else { p.off * i64::MAX / 2 });
                        let mut y: Vec<i64> = q.path.iter().map(|&v| v as i64).collect();
                        y.push(if q.array { q.off } else { q.off * i64::MAX / 2 });
                        if !p.array && !q.array && p.path.len() != q.path.len() && p.off == 0 && q.off == 0 {
                            return unsup("comparison of a subobject pointer with its enclosing object");
                        }
                        (x, y)
                    };
                    let res = match op {
                        BOp::Lt => x < y,
                        BOp::Gt => x > y,
                        BOp::Le => x <= y,
                        _ => x >= y,
                    };
                    Ok(Val::Int(res as i128))
                }
                _ => {
                    if self.constant {
                        return limit("CE-15", "relational comparison involving a null or function pointer");
                    }
                    unsup("relational comparison involving a null or function pointer")
                }
            },
            _ => unsup("pointer operation"),
        }
    }

    /// Arithmetic in the computation type `ct`, result type `rt`.
    pub fn arith(&self, op: BOp, l: Val, r: Val, ct: &Ty, rt: Ty) -> R<Val> {
        match (l, r) {
            (Val::Int(x), Val::Int(y)) => {
                let t = if matches!(op, BOp::Shl | BOp::Shr) {
                    rt.clone()
                } else if let Ty::Enum(e) = ct {
                    self.p.enum_underlying(*e)
                } else {
                    ct.clone()
                };
                let t = if t.is_integral() { t } else { rt.clone() };
                let signed = t.is_signed();
                let check = |v: i128, rule: &'static str| -> R<Val> {
                    if signed {
                        if !fits(v, &t) {
                            return ub(rule, format!("signed overflow: {} does not fit in {}", v, base_name(&t)));
                        }
                        Ok(Val::Int(v))
                    } else {
                        Ok(Val::Int(wrap_int(v, &t)))
                    }
                };
                match op {
                    BOp::Add => check(x + y, "ADD-9"),
                    BOp::Sub => check(x - y, "ADD-9"),
                    BOp::Mul => {
                        let v = x.checked_mul(y).unwrap_or(i128::MAX);
                        check(v, "MUL-10")
                    }
                    BOp::Div | BOp::Rem => {
                        if y == 0 {
                            return ub("MUL-9", "division by zero");
                        }
                        let q = x / y; // rounds toward zero (MUL-6)
                        if signed && !fits(q, &t) {
                            return ub("MUL-10", "quotient does not fit in the type");
                        }
                        if op == BOp::Div {
                            check(q, "MUL-10")
                        } else {
                            Ok(Val::Int(wrap_int(x % y, &t)))
                        }
                    }
                    BOp::Shl | BOp::Shr => {
                        let w = t.int_bits() as i128;
                        if y < 0 || y >= w {
                            return ub("SHF-7", "shift count negative or not less than the width");
                        }
                        if op == BOp::Shl {
                            // SHF-5: modulo 2^N, also for negative values
                            Ok(Val::Int(wrap_int(x.wrapping_shl(y as u32), &t)))
                        } else {
                            Ok(Val::Int(x >> y))
                        }
                    }
                    BOp::BitAnd => Ok(Val::Int(wrap_int(x & y, &t))),
                    BOp::BitOr => Ok(Val::Int(wrap_int(x | y, &t))),
                    BOp::BitXor => Ok(Val::Int(wrap_int(x ^ y, &t))),
                    BOp::Lt => Ok(Val::Int((x < y) as i128)),
                    BOp::Gt => Ok(Val::Int((x > y) as i128)),
                    BOp::Le => Ok(Val::Int((x <= y) as i128)),
                    BOp::Ge => Ok(Val::Int((x >= y) as i128)),
                    BOp::Eq => Ok(Val::Int((x == y) as i128)),
                    BOp::Ne => Ok(Val::Int((x != y) as i128)),
                    _ => unsup("operator"),
                }
            }
            (Val::Float(x), Val::Float(y)) => {
                let is_f32 = *ct == Ty::Float;
                let round = |v: f64| if is_f32 { v as f32 as f64 } else { v };
                let maxv = if is_f32 { f32::MAX as f64 } else { f64::MAX };
                let fin = |v: f64, rule: &'static str| -> R<Val> {
                    let r = round(v);
                    if (r.is_infinite() || v.abs() > maxv) && x.is_finite() && y.is_finite() {
                        return ub(rule, "floating result out of the range of its type");
                    }
                    Ok(Val::Float(r))
                };
                match op {
                    BOp::Add => fin(x + y, "ADD-17"),
                    BOp::Sub => fin(x - y, "ADD-17"),
                    BOp::Mul => fin(x * y, "MUL-12"),
                    BOp::Div => {
                        if y == 0.0 {
                            // CE-28: not in constant evaluation.
                            if self.constant {
                                return limit("CE-28", "floating division by zero in constant evaluation");
                            }
                            // MUL-11, MUL-20: the types of the matrix
                            // targets follow IEEE 754, so the result is a
                            // NaN or an infinity.
                            return Ok(Val::Float(round(x / y)));
                        }
                        fin(x / y, "MUL-12")
                    }
                    BOp::Lt => Ok(Val::Int((x < y) as i128)),
                    BOp::Gt => Ok(Val::Int((x > y) as i128)),
                    BOp::Le => Ok(Val::Int((x <= y) as i128)),
                    BOp::Ge => Ok(Val::Int((x >= y) as i128)),
                    BOp::Eq => Ok(Val::Int((x == y) as i128)),
                    BOp::Ne => Ok(Val::Int((x != y) as i128)),
                    _ => unsup("floating operator"),
                }
            }
            (Val::Int(x), Val::Float(y)) => self.arith(op, Val::Float(x as f64), Val::Float(y), ct, rt),
            (Val::Float(x), Val::Int(y)) => self.arith(op, Val::Float(x), Val::Float(y as f64), ct, rt),
            _ => unsup("operands of an arithmetic operator"),
        }
    }

    // ---- initialization ----

    /// Initialize the object at `place`, of type `ty`, from `e`. A class
    /// prvalue initializes the object directly (VC-4).
    pub fn init_into(&mut self, place: &Ptr, e: &Expr, ty: &QTy) -> R<()> {
        self.step()?;
        self.line = e.line;
        if ty.t.is_ref() {
            let p = self.bind_ref(e)?;
            return self.write(place, Val::Ptr(p), true);
        }
        match &ty.t {
            Ty::Class(c) => {
                let c = *c;
                match &e.k {
                    EK::Construct(cid, args) => return self.construct_into(place, *cid, args, c),
                    EK::Closure(_, inits, _) => {
                        // LAM rules: each member of the closure from its
                        // capture, in order.
                        for (i, (_, fi)) in self.fields(c).into_iter().enumerate() {
                            let Some(x) = inits.get(i) else { break };
                            if fi.ty.t.is_ref() {
                                // LAM-7: an init capture by reference extends
                                // a temporary as a reference variable does.
                                let p = self.bind_ref(x)?;
                                if super::stmt::binds_temporary_p(self.p, x) && !matches!(x.k, EK::Var(_)) {
                                    if let Some(c) = self.extend_temp(p.obj) {
                                        self.extended.push(c);
                                    }
                                }
                                self.write(&place.sub(fi.index), Val::Ptr(p), true)?;
                                continue;
                            }
                            self.init_into(&place.sub(fi.index), x, &fi.ty)?;
                        }
                        return Ok(());
                    }
                    EK::Call(..) if e.cat == Cat::PR => {
                        return match self.call(e, Some(place.clone()))? {
                            Ret::InSlot => Ok(()),
                            Ret::Val(v) => self.write(place, v, true),
                            Ret::Place(_) => unsup("class result"),
                        };
                    }
                    EK::InitList(items) => return self.init_aggregate(place, items, c),
                    EK::Cond(k, a, b) if e.cat == Cat::PR => {
                        return if self.eval_bool(k)? { self.init_into(place, a, ty) } else { self.init_into(place, b, ty) };
                    }
                    EK::Comma(a, b) if e.cat == Cat::PR => {
                        self.discard(a)?;
                        return self.init_into(place, b, ty);
                    }
                    EK::Conv(Conv::NoOp | Conv::PtrConv, a) if e.cat == Cat::PR && a.cat == Cat::PR && matches!(a.ty.t, Ty::Class(x) if x == c) => {
                        return self.init_into(place, a, ty);
                    }
                    EK::ZeroInit => return self.write(place, self.zero(ty), true),
                    EK::Throw(_) => {
                        self.eval(e)?;
                        return Ok(());
                    }
                    _ => {}
                }
                // A copy or move of a class object (CPY rules).
                let mut x = e;
                while let EK::Conv(Conv::LToR | Conv::NoOp, a) = &x.k {
                    x = a;
                }
                if x.cat == Cat::PR {
                    if !std::ptr::eq(x, e) {
                        return self.init_into(place, x, ty);
                    }
                    return match &e.k {
                        EK::Unknown(w) => self.unknown(w),
                        _ => unsup("class prvalue"),
                    };
                }
                let src = self.lvalue(x)?;
                let from = nonref(&x.ty).t;
                let src = self.class_cast(src, &from, &Ty::Class(c))?;
                let mv = e.cat == Cat::X || x.cat == Cat::X;
                self.copy_class_cv(place, &src, c, mv, nonref(&x.ty).c)
            }
            Ty::Array(el, n) => {
                match &e.k {
                    EK::InitList(items) => {
                        let n = n.unwrap_or(items.len() as u64) as usize;
                        let mut done = Vec::new();
                        for i in 0..n {
                            let sub = place.sub(i);
                            // LIST-7: each element in order.
                            let r = if i < items.len() { self.init_into(&sub, &items[i], el) } else { self.value_init(&sub, el) };
                            if let Err(e) = r {
                                // UNW-2: the elements built before end.
                                return Err(self.unwind(done, e));
                            }
                            done.push(Cleanup { place: sub, ty: (**el).clone(), item: 0 });
                        }
                        Ok(())
                    }
                    EK::Str(units) => {
                        let mut v: Vec<Val> = units.iter().map(|&u| Val::Int(u as i128)).collect();
                        v.push(Val::Int(0));
                        if let Some(n) = n {
                            v.resize(*n as usize, Val::Int(0));
                        }
                        self.write(place, Val::Agg(v), true)
                    }
                    EK::Conv(Conv::NoOp | Conv::PtrConv | Conv::LToR, a) => self.init_into(place, a, ty),
                    EK::ZeroInit => self.write(place, self.zero(ty), true),
                    _ if e.cat != Cat::PR => {
                        let p = self.lvalue(e)?;
                        let v = self.read(&p, false)?;
                        self.write(place, v, true)
                    }
                    EK::Unknown(w) => self.unknown(w),
                    _ => unsup("array initialization"),
                }
            }
            _ => {
                // INIT-1: a copy of an unsigned char with no value into an
                // unsigned char is not undefined, and has no value either.
                if ty.t == Ty::UChar && !self.constant {
                    if let EK::Conv(Conv::LToR, a) = &e.k {
                        if a.ty.t == Ty::UChar {
                            let p = self.lvalue(a)?;
                            let v = if matches!(self.get_at(&p), Ok(Val::Uninit)) { Val::Uninit } else { self.read(&p, a.ty.v)? };
                            return self.write(place, v, true);
                        }
                    }
                }
                let v = match &e.k {
                    EK::InitList(items) if items.len() == 1 => return self.init_into(place, &items[0], ty),
                    EK::InitList(items) if items.is_empty() => self.zero(ty),
                    _ => self.eval(e)?,
                };
                let v = match (&ty.t, v) {
                    (Ty::Ptr(_), Val::Int(0)) => Val::Null,
                    (_, v) => v,
                };
                self.write(place, v, true)
            }
        }
    }

    /// Value-initialization (INIT rules): a class with a user-provided
    /// default constructor runs it; anything else is zeroed first.
    pub fn value_init(&mut self, place: &Ptr, ty: &QTy) -> R<()> {
        match &ty.t {
            Ty::Class(c) => {
                let c = *c;
                let user = self.p.default_ctor(c).and_then(|d| self.p.func(d)).is_some_and(|f| !f.defaulted);
                if !user {
                    self.write(place, self.zero(ty), true)?;
                }
                if self.p.classes[c].ctors.is_empty() && self.p.classes[c].bases.is_empty() {
                    // An aggregate: default member initializers apply.
                    return self.init_aggregate(place, &[], c);
                }
                self.default_init(place, ty)
            }
            Ty::Array(el, Some(n)) => {
                // VC-9: each element's initialization is a full expression.
                let mut done = Vec::new();
                for i in 0..*n as usize {
                    if let Err(e) = self.full(|m| m.value_init(&place.sub(i), el)) {
                        // UNW-2: the elements built before end.
                        return Err(self.unwind(done, e));
                    }
                    done.push(Cleanup { place: place.sub(i), ty: (**el).clone(), item: 0 });
                }
                Ok(())
            }
            _ => self.write(place, self.zero(ty), true),
        }
    }

    /// Default-initialization (INIT rules): a class runs its default
    /// constructor; a scalar keeps no value (INIT-1).
    pub fn default_init(&mut self, place: &Ptr, ty: &QTy) -> R<()> {
        match &ty.t {
            Ty::Class(c) => {
                let c = *c;
                self.class_ok(c)?;
                if !self.p.classes[c].ctors.is_empty() {
                    return match self.p.default_ctor(c) {
                        Some(cid) => self.construct_into(place, cid, &[], c),
                        None => unsup("default constructor"),
                    };
                }
                self.implicit_ctor(place, c, None)
            }
            Ty::Array(el, Some(n)) => {
                // VC-9: each element's initialization is a full expression.
                let mut done = Vec::new();
                for i in 0..*n as usize {
                    if let Err(e) = self.full(|m| m.default_init(&place.sub(i), el)) {
                        // UNW-2: the elements built before end.
                        return Err(self.unwind(done, e));
                    }
                    done.push(Cleanup { place: place.sub(i), ty: (**el).clone(), item: 0 });
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Aggregate initialization from a braced list (LIST rules, UNI-4).
    fn init_aggregate(&mut self, place: &Ptr, items: &[Expr], c: usize) -> R<()> {
        self.class_ok(c)?;
        if !self.p.classes[c].ctors.is_empty() && items.is_empty() {
            return self.value_init(place, &QTy::new(Ty::Class(c)));
        }
        let union = self.p.classes[c].key == 2;
        let bases: Vec<usize> = self.p.classes[c].bases.iter().filter_map(|b| b.class).collect();
        let nb = bases.len();
        let mut k = 0;
        for (i, bc) in bases.iter().enumerate() {
            let bt = QTy::new(Ty::Class(*bc));
            if k < items.len() {
                self.init_into(&place.sub(i), &items[k], &bt)?;
                k += 1;
            } else {
                self.value_init(&place.sub(i), &bt)?;
            }
        }
        let fields = self.fields(c);
        // UNI-4: with no value given, the member with a default member
        // initializer is active, else the first.
        let active = if items.len() <= k { fields.iter().position(|(_, x)| x.init.is_some()).unwrap_or(0) } else { 0 };
        for (j, (fd, fi)) in fields.iter().enumerate() {
            let sub = place.sub(nb + fi.index);
            if fi.bits.is_some() && self.p.decls[*fd].name.is_empty() {
                // BF-7: an unnamed bit-field takes no element of the list.
                continue;
            }
            if union && j != active {
                self.write(&sub, Val::Inactive, true)?;
                continue;
            }
            if union && !fi.ty.t.is_ref() {
                // The active member gets its parts before they are set.
                let sh = self.shape(&fi.ty)?;
                self.write(&sub, sh, true)?;
            }
            if k < items.len() {
                let it = items[k].clone();
                k += 1;
                self.in_aggregate += 1;
                let r = self.init_field(&sub, &it, fi);
                self.in_aggregate -= 1;
                r?;
            } else if let Some(init) = &fi.init {
                let init = init.clone();
                self.with_this(place.clone(), |m| m.init_field(&sub, &init, fi))?;
            } else if fi.ty.t.is_ref() {
                return unsup("reference member with no initializer");
            } else {
                self.value_init(&sub, &fi.ty)?;
            }
        }
        Ok(())
    }

    fn with_this<T>(&mut self, this: Ptr, f: impl FnOnce(&mut Self) -> R<T>) -> R<T> {
        let save = self.frame_mut().this.replace(this);
        let r = f(self);
        self.frame_mut().this = save;
        r
    }

    fn init_field(&mut self, sub: &Ptr, init: &Expr, fi: &FieldInfo) -> R<()> {
        if fi.ty.t.is_ref() && self.in_aggregate > 0 {
            // REF-4: a reference member of an aggregate that binds a
            // temporary extends it to the aggregate's lifetime.
            let p = self.bind_ref(init)?;
            if super::stmt::binds_temporary_p(self.p, init) {
                if let Some(c) = self.extend_temp(p.obj) {
                    self.extended.push(c);
                }
            }
            return self.write(sub, Val::Ptr(p), true);
        }
        if fi.bits.is_some() {
            let v = self.eval(init)?;
            let target = Expr::new(EK::Unknown(String::new()), fi.ty.clone(), Cat::L, 0);
            let _ = target;
            let v = match (fi.bits, v) {
                (Some(w), Val::Int(x)) if w > 0 && w < 64 && fi.ty.t.is_integral() && fi.ty.t != Ty::Bool => {
                    let m = 1i128 << w;
                    let mut y = x.rem_euclid(m);
                    if fi.ty.t.is_signed() && y >= m / 2 {
                        y -= m;
                    }
                    Val::Int(y)
                }
                (_, v) => v,
            };
            return self.write(sub, v, true);
        }
        self.init_into(sub, init, &fi.ty)
    }

    // ---- constructors ----

    /// The implicit default constructor, or the defaulted one: bases and
    /// members by their default member initializers or by
    /// default-initialization.
    fn implicit_ctor(&mut self, place: &Ptr, c: usize, _f: Option<&FuncInfo>) -> R<()> {
        let mut done = Vec::new();
        match self.construct_parts(place, c, &[], &mut done) {
            Err(e) => Err(self.unwind(done, e)),
            r => r,
        }
    }

    /// Initialize the bases and members of a class object (CTOR-6, OBJ-1).
    fn construct_parts(&mut self, place: &Ptr, c: usize, mem_inits: &[MemInit], done: &mut Vec<Cleanup>) -> R<()> {
        self.class_ok(c)?;
        let bases: Vec<usize> = self.p.classes[c].bases.iter().filter_map(|b| b.class).collect();
        for (i, bc) in bases.iter().enumerate() {
            let bt = QTy::new(Ty::Class(*bc));
            match mem_inits.iter().find(|m| m.base == Some(*bc)) {
                Some(MemInit { init: Some(init), .. }) => {
                    let init = init.clone();
                    self.full(|m| m.init_into(&place.sub(i), &init, &bt))?;
                }
                Some(_) => return unsup("base initializer the front end does not model"),
                None => self.default_init(&place.sub(i), &bt)?,
            }
            done.push(Cleanup { place: place.sub(i), ty: bt, item: 0 });
        }
        let nb = bases.len();
        let union = self.p.classes[c].key == 2;
        for (fd, fi) in self.fields(c) {
            let sub = place.sub(nb + fi.index);
            let mi = mem_inits.iter().find(|m| m.field == Some(fd)).and_then(|m| m.init.clone());
            if let Some(init) = mi.or(fi.init.clone()) {
                self.full(|m| m.init_field(&sub, &init, &fi))?;
            } else if union {
                self.write(&sub, Val::Inactive, true)?;
            } else if fi.ty.t.is_ref() {
                return unsup("reference member with no initializer");
            } else {
                self.default_init(&sub, &fi.ty)?;
            }
            if !fi.ty.t.is_ref() && !union {
                done.push(Cleanup { place: sub, ty: fi.ty.clone(), item: 0 });
            }
        }
        Ok(())
    }

    /// Run a constructor on the object at `place` (CTOR rules).
    pub fn construct_into(&mut self, place: &Ptr, cid: DeclId, args: &[Expr], c: usize) -> R<()> {
        let Some(f) = self.p.func(cid) else { return unsup("constructor") };
        let f = f.clone();
        let cname = self.p.classes[c].name.clone();
        if self.constant && !f.constexpr && !f.consteval && !f.defaulted {
            // CE-2: only constexpr constructors.
            return limit("CE-2", format!("call of non-constexpr constructor of '{}'", cname));
        }
        self.class_ok(c)?;
        if self.constant {
            // Constant evaluation runs a destructor only if it is
            // constexpr.
            if let Some(d) = self.p.classes[c].dtor {
                let trivial = self.p.func(d).is_some_and(|df| df.defaulted || df.constexpr || matches!(df.body.as_deref(), Some(Stmt::Block(v)) if v.is_empty()));
                if !trivial {
                    return unsup("destructor evaluation");
                }
            }
        }
        if (f.opaque && (f.parse_failed || f.fn_try.is_none())) || f.mem_inits.iter().any(|m| m.init.is_none()) {
            return unsup("constructor with an initializer the front end does not model");
        }
        // A defaulted copy or move constructor copies or moves each part.
        if f.defaulted && !f.params.is_empty() && args.len() == 1 {
            let src = strip_bind(&args[0]);
            let p = if src.cat == Cat::PR { self.materialize(&src)? } else { self.lvalue(&src)? };
            let from = nonref(&src.ty).t;
            let p = self.class_cast(p, &from, &Ty::Class(c))?;
            let mv = self.p.var(f.params[0]).is_some_and(|v| matches!(v.ty.t, Ty::RRef(_)));
            return self.copy_parts(place, &p, c, mv);
        }
        if !f.defaulted && f.body.is_none() {
            // TR-4: a constructor defined in another translation unit runs
            // there, with the arguments that this unit evaluates.
            if let Some(&(t2, cid2)) = self.links.get(&(self.tu, cid)) {
                let f2 = self.ps[t2].func(cid2).unwrap().clone();
                let Some(c2) = f2.class else { return unsup("constructor") };
                let fr = self.bind_args(&f, args, &f)?;
                let frame = rebind(fr, &f, &f2);
                return self.in_tu(t2, |m| m.construct_with(place, cid2, f2.clone(), frame, c2));
            }
        }
        if f.body.is_none() && !f.defaulted {
            if f.defined {
                return unsup(format!("body of the constructor of '{}' is not available yet", cname));
            }
            if self.constant {
                return limit("CE-16", format!("constructor of '{}' is declared but not defined at this point", cname));
            }
            return unsup(format!("constructor of '{}' has no definition in the program", cname));
        }
        let frame = self.bind_args(&f, args, &f)?;
        self.construct_with(place, cid, f, frame, c)
    }

    /// Run constructor `cid` of class `c` on the object at `place`, with
    /// its parameters bound.
    fn construct_with(&mut self, place: &Ptr, cid: DeclId, f: FuncInfo, frame: Frame, c: usize) -> R<()> {
        if (f.opaque && (f.parse_failed || f.fn_try.is_none())) || f.mem_inits.iter().any(|m| m.init.is_none()) {
            return unsup("constructor with an initializer the front end does not model");
        }
        let body = if f.defaulted { None } else { f.body.clone() };
        self.enter_call(Some(cid), Some(place.clone()), false, false, None)?;
        self.install_params(frame);
        self.cdtor.push((place.clone(), c));
        // A mem-initializer does not extend a temporary (REF-5).
        let save_aggr = std::mem::replace(&mut self.in_aggregate, 0);
        let r = (|| -> R<()> {
            // CTOR-7: a delegating constructor runs the target first.
            let delegating = f.mem_inits.iter().find(|m| m.base == Some(c)).cloned();
            let mut done = Vec::new();
            let r = match &delegating {
                Some(d) => {
                    let init = d.init.clone().unwrap();
                    self.full(|m| m.init_into(place, &init, &QTy::new(Ty::Class(c))))
                }
                None => self.construct_parts(place, c, &f.mem_inits, &mut done),
            };
            // A target constructor that throws has ended its own parts.
            let target_done = delegating.is_some() && r.is_ok();
            let r = r.and_then(|_| match &body {
                Some(b) => self.run_body(b).map(|_| ()),
                None => Ok(()),
            });
            match r {
                Err(e @ Stop::Throw(_)) => {
                    // UNW-2: the completed parts end; after a delegation
                    // whose target completed, the whole object ends.
                    let e = if delegating.is_some() && !target_done {
                        e
                    } else if target_done {
                        let t = QTy::new(Ty::Class(c));
                        self.unwinding += 1;
                        let r = self.destroy(place, &t);
                        self.unwinding -= 1;
                        match r {
                            Ok(()) => e,
                            Err(Stop::Throw(_)) => Stop::Terminate("UNW-5", "a destructor throws during the unwinding of another exception".into()),
                            Err(o) => o,
                        }
                    } else {
                        self.unwind(done, e)
                    };
                    match (&e, f.fn_try.clone()) {
                        // TRY-10: the handlers of a function try block run
                        // after the parts end, and rethrow at their end.
                        (Stop::Throw(x), Some(hs)) => {
                            let mut seek = None;
                            self.handle_fn(x.clone(), &hs, &mut seek, true).map(|_| ())
                        }
                        _ => Err(e),
                    }
                }
                r => r,
            }
        })();
        self.cdtor.pop();
        self.in_aggregate = save_aggr;
        let l = self.leave_call();
        r?;
        l?;
        Ok(())
    }

    // ---- calls ----

    /// Evaluate the arguments of a call into parameter objects. The caller
    /// makes the parameters (VC-16). Default arguments come from `defs`,
    /// the function that the call names.
    pub fn bind_args(&mut self, f: &FuncInfo, args: &[Expr], defs: &FuncInfo) -> R<Frame> {
        let mut frame = Frame::new();
        let effects = args.iter().filter(|a| has_effects(a)).count();
        if !self.constant && effects >= 2 {
            // SEQ-1: the order of the arguments is unspecified.
            self.choice("SEQ-1", "arguments evaluated from left to right");
        }
        let mut all: Vec<(DeclId, Expr)> = Vec::new();
        for (i, a) in args.iter().enumerate() {
            if i >= f.params.len() {
                // An argument for the ellipsis: the program cannot read it
                // without the library, so only its evaluation counts.
                if !f.fty.variadic || matches!(nonref(&a.ty).t, Ty::Class(_)) {
                    return unsup("call with variadic arguments");
                }
                self.eval(a)?;
                continue;
            }
            all.push((f.params[i], a.clone()));
        }
        for i in args.len()..f.params.len() {
            let pid = f.params[i];
            let dpid = defs.params.get(i).copied().unwrap_or(pid);
            let d = self.p.var(dpid).and_then(|v| v.init.clone()).or_else(|| defs.default_exprs.get(i).cloned().flatten());
            match d {
                Some(d) => all.push((pid, (*d).clone())),
                None => return unsup("missing argument"),
            }
        }
        for (pid, a) in all {
            let pty = self.p.var(pid).map(|v| v.ty.clone()).unwrap_or(QTy::unknown());
            if pty.is_unknown() {
                return unsup("parameter of unknown type");
            }
            if pty.t.is_ref() {
                let pl = self.bind_ref(&a)?;
                frame.refs.insert(pid, pl);
            } else {
                let place = self.alloc(&pty.unqual())?;
                self.init_into(&place, &a, &pty)?;
                // VC-10: on the matrix targets the caller ends a parameter
                // at the end of its full expression.
                if self.needs_dtor(&pty.t) {
                    self.add_temp(&place, &pty);
                }
                frame.vars.insert(pid, place.obj);
            }
        }
        Ok(frame)
    }

    pub fn install_params(&mut self, frame: Frame) {
        let fr = self.frame_mut();
        fr.vars.extend(frame.vars);
        fr.refs.extend(frame.refs);
    }

    /// The final overrider of a virtual function for the object at `this`
    /// (VIRT-1, VIRT-5), and the place of the subobject of its class. The
    /// search runs from the dynamic type down the bases that lead to
    /// `this`, and the first class that declares the function wins.
    fn final_overrider(&self, fid: DeclId, this: &Ptr) -> Option<(DeclId, Ptr)> {
        let (dc, dpath) = self.dynamic_class(this)?;
        let name = &self.p.decls[fid].name;
        let fty = &self.p.func(fid)?.fty;
        let fp = full_path(this);
        let (mut c, mut path) = (dc, dpath);
        loop {
            let sc = self.p.classes[c].scope;
            if let Some(ids) = self.p.scopes[sc].names.get(name) {
                for &id in ids {
                    if let Some(g) = self.p.func(id) {
                        if id == fid || (same_params(&g.fty, fty) && g.class == Some(c)) {
                            return Some((id, Ptr { obj: this.obj, path: path.clone(), off: 0, array: false }));
                        }
                    }
                }
            }
            if path.len() >= fp.len() {
                return None;
            }
            let i = fp[path.len()];
            c = self.p.classes[c].bases.get(i)?.class?;
            path.push(i);
        }
    }

    /// Whether a member function is virtual: declared `virtual`, or with
    /// the name and parameters of a virtual function of a base (VIRT-1).
    pub fn is_virtual_fn(&self, fid: DeclId) -> bool {
        let Some(f) = self.p.func(fid) else { return false };
        if f.is_virtual {
            return true;
        }
        let Some(c) = f.class else { return false };
        if f.is_static || f.kind == FnKind::Ctor {
            return false;
        }
        let name = &self.p.decls[fid].name;
        let mut stack: Vec<usize> = self.p.classes[c].bases.iter().filter_map(|b| b.class).collect();
        let mut n = 0;
        while let Some(b) = stack.pop() {
            n += 1;
            if n > 64 {
                break;
            }
            let sc = self.p.classes[b].scope;
            if let Some(ids) = self.p.scopes[sc].names.get(name) {
                for &id in ids {
                    if let Some(g) = self.p.func(id) {
                        if g.is_virtual && same_params(&g.fty, &f.fty) {
                            return true;
                        }
                    }
                }
            }
            stack.extend(self.p.classes[b].bases.iter().filter_map(|x| x.class));
        }
        false
    }

    /// Call a function. `slot` is the object a class result initializes.
    pub fn call(&mut self, e: &Expr, slot: Option<Ptr>) -> R<Ret> {
        let EK::Call(callee, args, fid) = &e.k else { unreachable!() };
        if let EK::Unknown(w) = &callee.k {
            if let Some(name) = w.strip_prefix("allocation function ") {
                return self.call_allocation(name, args);
            }
        }
        // SEQ-2: the function, with its object, before the arguments.
        let from_ptr = fid.is_none();
        let mut fid = match fid {
            Some(f) => *f,
            None => {
                let v = match &callee.k {
                    EK::Unary(UOp::Deref, a) => self.eval(a)?,
                    _ => self.eval(callee)?,
                };
                match v {
                    Val::FnPtr(t, f) if t == self.tu => f,
                    Val::FnPtr(..) => return unsup("call through a pointer to a function of another translation unit"),
                    Val::Null => return ub("PFX-8", "call through a null function pointer"),
                    _ => return unsup("indirect call"),
                }
            }
        };
        let Some(f) = self.p.func(fid) else { return unsup("call of a non-function") };
        // ICS-41: the object of a call of a function with an explicit object
        // parameter is the first argument.
        let with_obj: Vec<Expr>;
        let args: &[Expr] = if f.explicit_obj && !matches!(callee.k, EK::Func(_)) {
            with_obj = std::iter::once((**callee).clone()).chain(args.iter().cloned()).collect();
            &with_obj
        } else {
            args
        };
        let static_f = f.clone();
        let static_fid = fid;
        let mut f = static_f.clone();
        let name0 = self.p.decls[fid].name.clone();
        let member = f.class.is_some() && !f.is_static && !f.explicit_obj && f.kind != FnKind::Ctor;
        // SEQ-6: an assignment operator function keeps the order of the
        // built-in assignment, the right operand first.
        let right_first = member && f.kind == FnKind::Operator && is_assignment_op(&name0) && args.len() == 1;
        let mut early_frame = None;
        if right_first {
            early_frame = Some(self.bind_args(&f, args, &f)?);
        }
        let mut this = if member && from_ptr && self.p.closure_conv.values().any(|&op| op == fid) {
            // LAM-10: a call through the pointer of a lambda with no
            // captures runs its call operator on a closure with no state.
            let c = f.class.unwrap();
            let t = QTy::new(Ty::Class(c));
            let p = self.alloc(&t)?;
            self.add_temp(&p, &t);
            Some(p)
        } else if member {
            if matches!(callee.k, EK::Func(_)) {
                return unsup("member call with no object");
            }
            let p = if callee.cat == Cat::PR {
                if let Ty::Ptr(_) = callee.ty.t {
                    match self.eval(callee)? {
                        Val::Ptr(p) => p,
                        // PFX-21: a member call through a null pointer.
                        Val::Null => return ub("PFX-21, UN-7", "member function called through a null pointer"),
                        _ => return unsup("object of a member call"),
                    }
                } else {
                    self.materialize(callee)?
                }
            } else {
                if let EK::Unary(UOp::Deref, x) = &callee.k {
                    if matches!(x.ty.t, Ty::Ptr(_)) && !has_effects(x) {
                        if let Val::Null = self.eval(x)? {
                            // PFX-21: a member call through a null pointer.
                            return ub("PFX-21, UN-7", "member function called through a null pointer");
                        }
                    }
                }
                self.lvalue(callee)?
            };
            // The object converts to the class of the member function.
            let from = match &nonref(&callee.ty).t {
                Ty::Ptr(x) => x.t.clone(),
                t => t.clone(),
            };
            let p = match f.class {
                Some(fc) => self.class_cast(p, &from, &Ty::Class(fc))?,
                None => p,
            };
            Some(p)
        } else {
            None
        };
        if let Some(t) = &this {
            if self.objs[t.obj].heap == Heap::Raw {
                // STG-18, STG-22: the storage holds no object of the class.
                return ub("STG-18, STG-22", "member function called on storage that holds no object of its class");
            }
            if !self.objs[t.obj].alive {
                if self.resident_holder(t.obj).is_some() {
                    // STG-19: the old name does not reach the object that
                    // took the storage.
                    return ub("STG-19, STG-22", "member function called through a name of an object whose storage another object took");
                }
                return ub("STG-22, OBJ-2", "member function called on an object whose lifetime has ended");
            }
            let qualified = callee.qualified || matches!(&callee.k, EK::Unary(UOp::Deref, x) if x.qualified);
            if !qualified && self.is_virtual_fn(fid) {
                if let Some((g, p)) = self.final_overrider(fid, t) {
                    if g != fid {
                        fid = g;
                        f = self.p.func(g).unwrap().clone();
                        this = Some(p);
                    }
                }
            }
        }
        // VIRT-1: the default arguments come from the function that the
        // call names, the parameters from the final overrider. The caller's
        // declaration gives the default arguments (DEFARG rules).
        let disp_f = f.clone();
        let dispatched = fid != static_fid;
        // TR-4: a declaration names the definition, which can be in another
        // translation unit. An inline function is one entity.
        let mut ctu = self.tu;
        if let Some(&(t2, g)) = self.links.get(&(self.tu, fid)) {
            ctu = t2;
            fid = g;
            f = self.ps[t2].func(g).unwrap().clone();
        }
        let cp = self.ps[ctu];
        let name = cp.decls[fid].name.clone();
        if self.constant && !f.constexpr && !f.consteval {
            // CE-2: only constexpr and consteval functions.
            return limit("CE-2", format!("call of non-constexpr function '{}'", name));
        }
        if f.template {
            return unsup("call of a function template");
        }
        if f.opaque && (f.parse_failed || f.fn_try.is_none()) {
            return unsup(format!("function '{}' has a body the front end did not parse", name));
        }
        if f.deleted {
            return unsup("deleted function");
        }
        if f.defaulted && f.body.is_none() {
            if ctu != self.tu {
                return unsup("defaulted function of another translation unit");
            }
            return self.call_defaulted(&f, fid, this, args);
        }
        let Some(body) = f.body.clone() else {
            if self.constant {
                if f.defined {
                    return unsup(format!("body of '{}' is not available yet", name));
                }
                // CE-16: a declared but not yet defined function.
                return limit("CE-16", format!("'{}' is declared but not defined at this point", name));
            }
            if f.defined {
                return unsup(format!("body of '{}' is not available", name));
            }
            return unsup(format!("function '{}' has no definition in the program", name));
        };
        if f.kind == FnKind::Ctor || f.kind == FnKind::Dtor {
            return unsup("explicit call of a constructor or destructor");
        }
        let ret_ref = f.fty.ret.t.is_ref() || f.fty.ret.is_unknown();
        let ret_class = matches!(f.fty.ret.t, Ty::Class(_));
        // A class result needs an object; without a slot from the caller it
        // is a temporary of the type the caller sees.
        let slot = if ret_class {
            match slot {
                Some(s) => Some(s),
                None => {
                    let t = static_f.fty.ret.unqual();
                    let p = self.alloc(&t)?;
                    self.add_temp(&p, &t);
                    Some(p)
                }
            }
        } else {
            None
        };
        let frame = match early_frame {
            Some(fr) => rebind(fr, &static_f, &f),
            None => {
                let fr = self.bind_args(&disp_f, args, &static_f)?;
                rebind(fr, &disp_f, &f)
            }
        };
        let consteval = f.consteval;
        let fn_try = f.fn_try.clone();
        let (r, ret) = self.in_tu(ctu, |m| {
            m.enter_call(Some(fid), this, ret_ref, ret_class, slot.clone())?;
            m.install_params(frame);
            let save = m.in_consteval_fn;
            m.in_consteval_fn = consteval;
            let r = match fn_try {
                Some(hs) => m.run_fn_try(&body, &hs, false),
                None => m.run_body(&body),
            };
            m.in_consteval_fn = save;
            let ret = m.leave_call();
            Ok((r, ret))
        })?;
        if let Err(Stop::Throw(_)) = &r {
            if f.fty.noexcept {
                // THR-9, UNW-5: an exception leaves a non-throwing function.
                return Err(Stop::Terminate("THR-9, UNW-5", format!("an exception leaves '{}', which is non-throwing", name)));
            }
        }
        let flow = r?;
        let ret = ret?;
        if cp.has_attr(fid, "noreturn") || self.p.has_attr(static_fid, "noreturn") || self.p.has_attr(callee_decl(callee).unwrap_or(static_fid), "noreturn") {
            // ATTR-3: a noreturn function that returns.
            return ub("ATTR-3", format!("'{}' is declared noreturn and returns", name));
        }
        match (flow, ret) {
            (Flow::Return, Some(v)) => {
                // VIRT-4: a covariant result converts to the return type of
                // the function that the call names.
                let conv = |m: &mut Self, p: Ptr| -> R<Ptr> {
                    let to = nonref(&static_f.fty.ret);
                    let from = nonref(&disp_f.fty.ret);
                    match (&from.t, &to.t) {
                        (Ty::Ptr(a), Ty::Ptr(b)) => m.class_cast(p, &a.t, &b.t),
                        (a, b) => m.class_cast(p, a, b),
                    }
                };
                Ok(match v {
                    Ret::Val(Val::Ptr(p)) if dispatched => Ret::Val(Val::Ptr(conv(self, p)?)),
                    Ret::Place(p) if dispatched => Ret::Place(conv(self, p)?),
                    v => v,
                })
            }
            (Flow::Return, None) | (Flow::Normal, _) => {
                if f.fty.ret.t == Ty::Void {
                    Ok(Ret::Val(Val::Void))
                } else if f.is_main {
                    Ok(Ret::Val(Val::Int(0)))
                } else {
                    // FN-3: flowing off the end of a value-returning function.
                    ub("FN-3", format!("control reaches the end of '{}' without a return", name))
                }
            }
            (Flow::Goto(l), _) => unsup(format!("goto to label '{}' that the interpreter did not find", l)),
            _ => unsup("break or continue outside a loop"),
        }
    }

    /// A defaulted member function other than a constructor.
    fn call_defaulted(&mut self, f: &FuncInfo, fid: DeclId, this: Option<Ptr>, args: &[Expr]) -> R<Ret> {
        let name = self.p.decls[fid].name.clone();
        if let Some(&op) = self.p.closure_conv.get(&fid) {
            // LAM-10: the pointer to the function of a lambda with no
            // captures.
            return Ok(Ret::Val(Val::FnPtr(self.tu, op)));
        }
        if f.kind == FnKind::Operator && name == "operator=" && args.len() == 1 {
            let Some(this) = this else { return unsup("defaulted assignment") };
            let Some(c) = f.class else { return unsup("defaulted assignment") };
            let src = strip_bind(&args[0]);
            let p = if src.cat == Cat::PR { self.materialize(&src)? } else { self.lvalue(&src)? };
            let p = self.class_cast(p, &nonref(&src.ty).t, &Ty::Class(c))?;
            let mv = self.p.var(f.params[0]).is_some_and(|v| matches!(v.ty.t, Ty::RRef(_)));
            // The defaulted operator assigns each part; it does not call
            // itself.
            let bases: Vec<usize> = self.p.classes[c].bases.iter().filter_map(|b| b.class).collect();
            if self.p.classes[c].key == 2 {
                let v = self.get_at(&p)?.clone();
                self.write(&this, v, false)?;
            } else {
                for (i, bc) in bases.iter().enumerate() {
                    self.assign_parts(&this.sub(i), &p.sub(i), *bc, mv)?;
                }
                let nb = bases.len();
                for (_, fi) in self.fields(c) {
                    let (d, s) = (this.sub(nb + fi.index), p.sub(nb + fi.index));
                    self.assign_value(&d, &s, &fi.ty, mv)?;
                }
            }
            return Ok(if f.fty.ret.t.is_ref() { Ret::Place(this) } else { Ret::Val(Val::Void) });
        }
        unsup(format!("defaulted function '{}'", name))
    }
}

fn is_assignment_op(name: &str) -> bool {
    matches!(name, "operator=" | "operator+=" | "operator-=" | "operator*=" | "operator/=" | "operator%=" | "operator&=" | "operator|=" | "operator^=" | "operator<<=" | "operator>>=")
}

pub fn same_params(a: &FnTy, b: &FnTy) -> bool {
    a.params.len() == b.params.len() && a.params.iter().zip(b.params.iter()).all(|(x, y)| same_unqual(x, y)) && a.cv_c == b.cv_c
}

/// The parameters bound for `from` given to the same positions of `to`.
fn rebind(fr: Frame, from: &FuncInfo, to: &FuncInfo) -> Frame {
    let mut out = Frame::new();
    for (i, p) in from.params.iter().enumerate() {
        let Some(&q) = to.params.get(i) else { continue };
        if let Some(o) = fr.vars.get(p) {
            out.vars.insert(q, *o);
        }
        if let Some(r) = fr.refs.get(p) {
            out.refs.insert(q, r.clone());
        }
    }
    out
}

fn callee_decl(e: &Expr) -> Option<DeclId> {
    match &e.k {
        EK::Func(f) => Some(*f),
        _ => None,
    }
}

impl<'a> Machine<'a> {
    /// A class whose copy runs code: it or a part has a copy or move
    /// constructor that is not defaulted.
    pub fn copy_runs_code(&self, c: usize) -> bool {
        if self.p.has_user_copy(c) {
            return true;
        }
        let ci = &self.p.classes[c];
        if ci.key == 2 {
            return false;
        }
        ci.bases.iter().any(|b| b.class.is_some_and(|bc| self.copy_runs_code(bc))) || self.fields(c).iter().any(|(_, fi)| !fi.ty.t.is_ref() && crate::decl2::elem_class(&fi.ty.t).is_some_and(|fc| self.copy_runs_code(fc)))
    }

    /// The copy or move constructor that copies an object of class `c`:
    /// for a move, a constructor whose parameter is an rvalue reference if
    /// the class has one, else the copy constructor (CPY rules).
    fn copy_ctor(&self, c: usize, mv: bool) -> Option<DeclId> {
        self.copy_ctor_cv(c, mv, false)
    }

    /// The same, for a source that is `const` if `konst`: an rvalue
    /// reference to a class that is not const does not bind it (CPY-9).
    fn copy_ctor_cv(&self, c: usize, mv: bool, konst: bool) -> Option<DeclId> {
        let ctors: Vec<(DeclId, bool)> = self.p.classes[c].ctors.iter().filter_map(|&i| {
            let f = self.p.func(i)?;
            if f.params.is_empty() || f.params.len() - f.defaults.min(f.params.len()) > 1 || f.template {
                return None;
            }
            match &self.p.var(f.params[0])?.ty.t {
                Ty::LRef(x) if x.t == Ty::Class(c) && (x.c || !konst) => Some((i, false)),
                Ty::RRef(x) if x.t == Ty::Class(c) && (x.c || !konst) => Some((i, true)),
                _ => None,
            }
        }).collect();
        if mv {
            if let Some((i, _)) = ctors.iter().find(|x| x.1) {
                return Some(*i);
            }
        }
        ctors.iter().find(|x| !x.1).map(|x| x.0)
    }

    /// A copy of a class glvalue in a temporary, for a use that needs its
    /// value.
    fn copy_to_temp(&mut self, e: &Expr, c: usize) -> R<Val> {
        let src = self.lvalue(e)?;
        let t = QTy::new(Ty::Class(c));
        let place = self.alloc(&t)?;
        self.copy_class(&place, &src, c, e.cat == Cat::X)?;
        self.add_temp(&place, &t);
        Ok(self.get_at(&place)?.clone())
    }

    /// Whether class `c` has an implicit move constructor: no
    /// user-declared copy constructor, copy or move assignment, or
    /// destructor (CPY-5).
    fn has_implicit_move(&self, c: usize) -> bool {
        if self.p.classes[c].dtor.is_some_and(|d| self.p.func(d).is_some_and(|f| !f.defaulted || f.first_defaulted)) {
            return false;
        }
        if self.copy_ctor(c, false).is_some() {
            return false;
        }
        let sc = self.p.classes[c].scope;
        !self.p.scopes[sc].names.get("operator=").is_some_and(|v| !v.is_empty())
    }

    /// Initialize the object at `place` as a copy or move of the object of
    /// class `c` at `src`.
    pub fn copy_class(&mut self, place: &Ptr, src: &Ptr, c: usize, mv: bool) -> R<()> {
        self.copy_class_cv(place, src, c, mv, false)
    }

    pub fn copy_class_cv(&mut self, place: &Ptr, src: &Ptr, c: usize, mv: bool, konst: bool) -> R<()> {
        if let Some(cid) = self.copy_ctor_cv(c, mv, konst) {
            if self.p.func(cid).is_some_and(|f| !f.defaulted) {
                let i = self.synth.len();
                self.synth.push(src.clone());
                let t = QTy::new(Ty::Class(c));
                let arg = Expr::new(EK::Unknown(format!("\u{1}{}", i)), t, if mv { Cat::X } else { Cat::L }, self.line);
                return self.construct_into(place, cid, &[arg], c);
            }
        }
        let mv = mv && (self.copy_ctor(c, true).is_some() || self.has_implicit_move(c));
        self.copy_parts(place, src, c, mv)
    }

    /// The implicit or defaulted copy or move: each base and member in
    /// order (CPY-8).
    pub fn copy_parts(&mut self, place: &Ptr, src: &Ptr, c: usize, mv: bool) -> R<()> {
        if !self.copy_runs_code(c) {
            let v = self.get_at(src)?.clone();
            if self.constant && has_unset(&v) {
                return limit("CE-22", "copy of an object that has a scalar part with no value");
            }
            return self.write(place, v, true);
        }
        self.class_ok(c)?;
        let bases: Vec<usize> = self.p.classes[c].bases.iter().filter_map(|b| b.class).collect();
        for (i, bc) in bases.iter().enumerate() {
            self.copy_class(&place.sub(i), &src.sub(i), *bc, mv)?;
        }
        let nb = bases.len();
        for (_, fi) in self.fields(c) {
            let (d, s) = (place.sub(nb + fi.index), src.sub(nb + fi.index));
            self.copy_value(&d, &s, &fi.ty, mv)?;
        }
        Ok(())
    }

    fn copy_value(&mut self, d: &Ptr, s: &Ptr, t: &QTy, mv: bool) -> R<()> {
        match &t.t {
            Ty::Class(fc) => self.copy_class(d, s, *fc, mv),
            Ty::Array(el, Some(n)) => {
                for i in 0..*n as usize {
                    self.copy_value(&d.sub(i), &s.sub(i), el, mv)?;
                }
                Ok(())
            }
            _ => {
                let v = self.get_at(s)?.clone();
                if self.constant && has_unset(&v) {
                    return limit("CE-22", "copy of an object that has a scalar part with no value");
                }
                self.write(d, v, true)
            }
        }
    }
}

impl<'a> Machine<'a> {
    /// The assignment operator that class `c` declares for a copy or a
    /// move, if any.
    fn assign_op(&self, c: usize, mv: bool) -> Option<DeclId> {
        let sc = self.p.classes[c].scope;
        let ids = self.p.scopes[sc].names.get("operator=")?;
        let ops: Vec<(DeclId, bool)> = ids.iter().filter_map(|&i| {
            let f = self.p.func(i)?;
            if f.params.len() != 1 || f.template {
                return None;
            }
            match &self.p.var(f.params[0])?.ty.t {
                Ty::LRef(x) if x.t == Ty::Class(c) => Some((i, false)),
                Ty::RRef(x) if x.t == Ty::Class(c) => Some((i, true)),
                Ty::Class(x) if *x == c => Some((i, false)),
                _ => None,
            }
        }).collect();
        if mv {
            if let Some((i, _)) = ops.iter().find(|x| x.1) {
                return Some(*i);
            }
        }
        ops.iter().find(|x| !x.1).map(|x| x.0)
    }

    /// Assign the object of class `c` at `src` to the one at `dst`: the
    /// class's own assignment operator, or each base and member in order
    /// (CPY rules). A union copies its active member (UNI-5).
    pub fn assign_parts(&mut self, dst: &Ptr, src: &Ptr, c: usize, mv: bool) -> R<()> {
        if let Some(fid) = self.assign_op(c, mv) {
            if self.p.func(fid).is_some_and(|f| !f.defaulted) {
                let i = self.synth.len();
                self.synth.push(dst.clone());
                self.synth.push(src.clone());
                let t = QTy::new(Ty::Class(c));
                let obj = Expr::new(EK::Unknown(format!("\u{1}{}", i)), t.clone(), Cat::L, self.line);
                let arg = Expr::new(EK::Unknown(format!("\u{1}{}", i + 1)), t.clone(), if mv { Cat::X } else { Cat::L }, self.line);
                let ret = self.p.func(fid).unwrap().fty.ret.clone();
                let (rt, cat) = if ret.t.is_ref() { (nonref(&ret), Cat::L) } else { (ret, Cat::PR) };
                let call = Expr::new(EK::Call(Box::new(obj), vec![arg], Some(fid)), rt, cat, self.line);
                self.full(|m| m.discard(&call))?;
                return Ok(());
            }
        }
        if !self.copy_runs_code(c) && !self.assign_runs_code(c) {
            let v = self.get_at(src)?.clone();
            if self.constant && has_unset(&v) {
                return limit("CE-22", "copy of an object that has a scalar part with no value");
            }
            return self.write(dst, v, false);
        }
        if self.p.classes[c].key == 2 {
            let v = self.get_at(src)?.clone();
            return self.write(dst, v, false);
        }
        // CPY-5: without an implicit move assignment, the copy assignment
        // assigns each part.
        let mv = mv && self.has_implicit_move(c);
        let bases: Vec<usize> = self.p.classes[c].bases.iter().filter_map(|b| b.class).collect();
        for (i, bc) in bases.iter().enumerate() {
            self.assign_parts(&dst.sub(i), &src.sub(i), *bc, mv)?;
        }
        let nb = bases.len();
        for (_, fi) in self.fields(c) {
            let (d, s) = (dst.sub(nb + fi.index), src.sub(nb + fi.index));
            self.assign_value(&d, &s, &fi.ty, mv)?;
        }
        Ok(())
    }

    fn assign_value(&mut self, d: &Ptr, s: &Ptr, t: &QTy, mv: bool) -> R<()> {
        match &t.t {
            Ty::Class(fc) => self.assign_parts(d, s, *fc, mv),
            Ty::Array(el, Some(n)) => {
                for i in 0..*n as usize {
                    self.assign_value(&d.sub(i), &s.sub(i), el, mv)?;
                }
                Ok(())
            }
            _ => {
                let v = self.get_at(s)?.clone();
                if self.constant && has_unset(&v) {
                    return limit("CE-22", "copy of an object that has a scalar part with no value");
                }
                self.write(d, v, false)
            }
        }
    }

    /// A class whose assignment runs code that the program wrote.
    fn assign_runs_code(&self, c: usize) -> bool {
        if self.assign_op(c, false).or(self.assign_op(c, true)).is_some_and(|f| self.p.func(f).is_some_and(|f| !f.defaulted)) {
            return true;
        }
        let ci = &self.p.classes[c];
        ci.bases.iter().any(|b| b.class.is_some_and(|bc| self.assign_runs_code(bc))) || self.fields(c).iter().any(|(_, fi)| crate::decl2::elem_class(&fi.ty.t).is_some_and(|fc| self.assign_runs_code(fc)))
    }
}
