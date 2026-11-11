// Expressions on the abstract machine: places, values, conversions,
// operators, calls, and initialization.

use super::*;
use super::f80::F80;

/// The largest allocation the interpreter's machine gives. A larger
/// request fails (NEW-38).
const ALLOC_LIMIT: u64 = 1 << 40;

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
        if !self.deferred_tl.is_empty() && self.deferred_tl.contains(&key) {
            self.start_deferred_tl(key.0)?;
        }
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
                    let this = match self.frame().this.clone() {
                        Some(t) => t,
                        None => {
                            // LAM-16: with an explicit object parameter, the
                            // closure object is that parameter's object.
                            let p0 = self.p.func(op).filter(|f| f.explicit_obj).and_then(|f| f.params.first().copied());
                            let Some(p0) = p0 else { return unsup("closure without an object") };
                            if let Some(r) = self.frame().refs.get(&p0) {
                                r.clone()
                            } else if let Some(&o) = self.frame().vars.get(&p0) {
                                Ptr::whole(o)
                            } else {
                                return unsup("closure without an object");
                            }
                        }
                    };
                    let place = this.sub(idx);
                    let c = self.p.func(op).and_then(|f| f.class);
                    let is_ref = c.and_then(|c| self.fields(c).into_iter().find(|(_, fi)| fi.index == idx)).is_some_and(|(_, fi)| fi.ty.t.is_ref());
                    if is_ref {
                        return match self.get_at(&place)?.clone() {
                            Val::Ptr(q) if !self.objs[q.obj].alive && !self.constant => {
                                // LAM-19: the body uses a reference capture
                                // after the life of the variable ended.
                                ub("LAM-19", "use of a reference capture whose variable's lifetime has ended")
                            }
                            Val::Ptr(q) => Ok(q),
                            _ => unsup("reference capture"),
                        };
                    }
                    return Ok(place);
                }
            }
        }
        if let Some(e) = self.p.sb_place.get(&id) {
            // SB-1: a structured binding names a part of the hidden variable.
            let e = e.clone();
            return self.lvalue(&e);
        }
        let Some(v) = self.p.var(id) else {
            return unsup("reference to something that is not a variable");
        };
        let v = v.clone();
        if v.dependent {
            return unsup("value of a template parameter, structured binding, or init capture");
        }
        if v.ty.is_unknown() {
            // A variable whose type the front end did not work out, such
            // as a failed class template argument deduction.
            return unsup(format!("variable '{}' of a type the front end does not know", self.p.decls[id].name));
        }
        if !self.constant {
            if v.local && v.storage == Storage::Auto {
                // ODR-2, ODR-3: a lambda or a local class can read a
                // constant of the enclosing function without a capture. The
                // object is in the frame of the enclosing call.
                let usable = !v.param && v.init.is_some() && !v.ty.v && (v.constexpr || (v.ty.c && (v.ty.t.is_integral() || matches!(v.ty.t, Ty::Enum(_)))));
                if usable {
                    for fr in self.frames.iter().rev() {
                        if let Some(r) = fr.refs.get(&id) {
                            return Ok(r.clone());
                        }
                        if let Some(&o) = fr.vars.get(&id) {
                            return Ok(Ptr::whole(o));
                        }
                    }
                }
                return unsup("variable of an enclosing function (local class or lambda)");
            }
            if v.local && v.storage == Storage::Extern {
                // STO-7, LNK-9: a block-scope extern declaration names the
                // variable of the enclosing namespace with that name.
                let name = self.p.decls[id].name.clone();
                let mut sc = self.p.scopes[self.p.decls[id].scope].parent;
                while let Some(x) = sc {
                    if self.p.scopes[x].kind == SK::Namespace {
                        if let Some(g) = self.p.scopes[x].names.get(&name).and_then(|v| v.iter().copied().find(|&g| g != id && self.p.var(g).is_some_and(|gv| !gv.local))) {
                            return self.var_place(g);
                        }
                    }
                    sc = self.p.scopes[x].parent;
                }
            }
            if v.local {
                return unsup("block-scope static variable used before its declaration ran");
            }
            // LSUP-8: the library defines `std::nothrow`, an object of an
            // empty class with no state. As a front-end choice under
            // decision 0035, the run makes it on first use.
            let d = &self.p.decls[id];
            if d.name == "nothrow" && self.p.scopes[d.scope].name == "std" && matches!(&v.ty.t, Ty::Class(c) if self.p.classes[*c].name == "nothrow_t" && self.p.classes[*c].fields.is_empty()) {
                let place = self.alloc(&v.ty)?;
                self.globals.insert(key, place.obj);
                return Ok(place);
            }
            return unsup(format!("variable '{}' has no definition in this translation unit", self.p.decls[id].name));
        }
        self.const_var_place(id, v)
    }

    /// A variable from outside the constant evaluation (CE-3).
    pub(crate) fn const_var_place(&mut self, id: DeclId, v: VarInfo) -> R<Ptr> {
        let name = self.p.decls[id].name.clone();
        let usable_local = v.local && !v.param && v.init.is_some() && !v.ty.v && (v.constexpr || (v.ty.c && (v.ty.t.is_integral() || matches!(v.ty.t, Ty::Enum(_)))));
        if (v.param || (v.local && v.storage == Storage::Auto)) && !usable_local {
            // A variable of a caller that is not part of this evaluation.
            let o = self.new_obj(Val::NotUsable(format!("'{}' is not usable in constant expressions", name)), false);
            self.globals.insert((self.tu, id), o);
            if let Some(&a) = self.p.var_align.get(&id) {
                self.raw_align.insert(o, a);
            }
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
                    if matches!(v.ty.t, Ty::Class(_)) && v.constexpr && v.storage != Storage::Extern {
                        // A constexpr class object without an initializer
                        // runs its default constructor.
                        let r = self.with_fresh_frame(|m| {
                            let place = m.alloc(&v.ty.unqual())?;
                            m.full(|m| m.default_init(&place, &v.ty))?;
                            Ok(m.objs[place.obj].val.clone())
                        });
                        match r {
                            Ok(x) => x,
                            Err(Stop::Limit(r, msg) | Stop::Undefined(r, msg)) => return Err(Stop::Limit(r, msg)),
                            Err(e) => return Err(e),
                        }
                    } else if matches!(v.ty.t, Ty::Class(_)) {
                        return unsup("default-initialized class object (constructor evaluation)");
                    } else if v.class.is_some() || v.storage == Storage::Extern {
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
            // CE-14: the address of a subobject is a constant, so a class
            // or array keeps its shape, and each scalar part is not usable.
            if matches!(v.ty.t, Ty::Class(_) | Ty::Array(..)) && self.leaves(&v.ty, 0) <= 4096 {
                fn mark(v: Val, why: &str) -> Val {
                    match v {
                        Val::Agg(xs) => Val::Agg(xs.into_iter().map(|x| mark(x, why)).collect()),
                        _ => Val::NotUsable(why.to_string()),
                    }
                }
                mark(self.zero(&v.ty), &why)
            } else {
                Val::NotUsable(why)
            }
        };
        let o = self.new_obj(val, false);
        self.objs[o].is_static = true;
        self.objs[o].volatile = v.ty.v;
        self.objs[o].ty = Some(v.ty.clone());
        self.globals.insert((self.tu, id), o);
        if let Some(&a) = self.p.var_align.get(&id) {
            self.raw_align.insert(o, a);
        }
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
            let mark = m.extended.len();
            let pl = m.bind_ref(init)?;
            // REF-4, Q-0193: the temporaries that reference members of an
            // extended aggregate bind are static too.
            let mut cs = m.extended.split_off(mark);
            if super::stmt::binds_temporary_p(m.p, init) {
                if let Some(c) = m.extend_temp(pl.obj) {
                    cs.push(c);
                }
            }
            for c in cs {
                m.objs[c.place.obj].is_static = true;
                if !m.constant {
                    m.static_dtors.push(c);
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
            EK::Func(f) => {
                // A function lvalue that a reference binds: a place that
                // holds the function, which a call and `&` read.
                let o = self.new_obj(Val::FnPtr(self.tu, *f), true);
                self.objs[o].ty = Some(QTy::new(Ty::Ptr(Box::new(e.ty.unqual()))));
                // A function exists for the whole program.
                self.objs[o].is_static = true;
                self.objs[o].konst = true;
                Ok(Ptr::whole(o))
            }
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
                        if self.std_class("bad_cast").is_some() {
                            // LSUP-8: the program declares std::bad_cast.
                            return Err(self.throw_std("bad_cast")?);
                        }
                        let place = self.alloc(&QTy::int())?;
                        self.write(&place, Val::Int(0), true)?;
                        Err(Stop::Throw(Exc { obj: place.obj, ty: QTy::new(Ty::Unknown) }))
                    }
                }
            }
            EK::Reinterpret(a, w) => {
                if self.constant {
                    return limit("CE-4", w.clone());
                }
                // CAST-6: a reference cast acts on the place of the operand.
                let p = self.lvalue(a)?;
                self.retarget(p, &e.ty)
            }
            EK::PtrMem(obj, mp) => self.ptr_mem_place(obj, mp),
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
        // SEQ-2: the array before the index.
        let base = self.eval(a)?;
        let idx = self.eval_int(i)?;
        match base {
            Val::Ptr(p) => self.ptr_add_ty(&p, idx, "PFX-4", &a.ty.t),
            Val::Null => ub("UN-7", "subscript of a null pointer"),
            _ => unsup("subscript"),
        }
    }

    /// The object of a string literal. A run gives each set of characters
    /// one object (PRIM-9: whether two literals share an object is
    /// unspecified). Constant evaluation makes a new object each time.
    pub fn string_obj(&mut self, units: &[u32]) -> Ptr {
        let mut root = None;
        if !self.constant {
            if let Some(&o) = self.strings.get(units) {
                // PRIM-9: a later evaluation gives an object of its own,
                // which a comparison treats as the first object or as
                // another one. A copy of one pointer stays one object.
                let n = self.string_count.entry(o).or_insert(1);
                if *n >= 256 {
                    return Ptr::whole(o);
                }
                *n += 1;
                root = Some(o);
            }
        }
        let o = self.new_obj(Val::Agg(units.iter().map(|&u| Val::Int(u as i128)).chain(std::iter::once(Val::Int(0))).collect()), false);
        self.objs[o].string = true;
        self.objs[o].is_static = true;
        self.objs[o].konst = true;
        if let Some(r) = root {
            self.string_root.insert(o, r);
        } else if !self.constant {
            self.strings.insert(units.to_vec(), o);
        }
        Ptr::whole(o)
    }

    /// PRIM-9: the first object of a string literal object's characters.
    pub fn string_root_of(&self, o: usize) -> usize {
        self.string_root.get(&o).copied().unwrap_or(o)
    }

    fn member_place(&mut self, base: &Expr, fid: DeclId) -> R<Ptr> {
        let fi = match &self.p.decls[fid].k {
            DK::Field(f) => f.clone(),
            _ => return unsup("member"),
        };
        let bp = if base.cat == Cat::PR { self.materialize(base)? } else { self.lvalue(base)? };
        if !bp.array && !bp.bytes && bp.off != 0 {
            return ub("UN-7", "member access through a pointer past the end");
        }
        self.class_ok(fi.class)?;
        let bc = match nonref(&base.ty).t {
            Ty::Class(c) => c,
            _ => fi.class,
        };
        if base.cat != Cat::PR {
            self.check_class_access(&bp, bc)?;
        }
        let mut p = Ptr { obj: bp.obj, path: full_path(&bp), off: 0, array: false, bytes: false };
        if bc != fi.class {
            match self.base_path(bc, fi.class) {
                Some(bp) => {
                    p.path.extend(bp);
                    p = self.canon(p);
                }
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
        let holder = Ptr { obj: p.obj, path: p.path[..n - 1].to_vec(), off: 0, array: false, bytes: false };
        let union = Ptr { obj: p.obj, path: p.path[..n - 2].to_vec(), off: 0, array: false, bytes: false };
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
        // UNI-15, CLS-19: the members of a class can come from a base.
        let Some((prefix, ad)) = self.decl_member_class(ac, 0) else { return };
        if ad == fi.class {
            return;
        }
        let (a, b) = (self.fields(ad), self.fields(fi.class));
        let common = a.iter().zip(b.iter()).take_while(|(x, y)| self.layout_compatible(&x.1.ty, &y.1.ty, 0) && x.1.bits == y.1.bits).count();
        if fi.index < common {
            let last = self.nbases(ad) + fi.index;
            p.path.truncate(n - 2);
            p.path.push(active);
            p.path.extend(prefix);
            p.path.push(last);
            if !same_unqual(&fi.ty, &a[fi.index].1.ty) {
                self.uni_alias.borrow_mut().insert((p.obj, p.path.clone()));
            }
        }
    }

    /// The class that declares the members of `c`, and the path of base
    /// indices to it: `c` itself, or the one base that declares them all
    /// (CLS-19).
    fn decl_member_class(&self, c: usize, depth: u32) -> Option<(Vec<usize>, usize)> {
        let ci = &self.p.classes[c];
        if depth > 16 || ci.key == 2 {
            return None;
        }
        if !self.fields(c).is_empty() {
            return if ci.bases.is_empty() { Some((vec![], c)) } else { None };
        }
        match ci.bases.as_slice() {
            [] => Some((vec![], c)),
            [b] => {
                let bc = b.class?;
                let (mut path, d) = self.decl_member_class(bc, depth + 1)?;
                path.insert(0, 0);
                Some((path, d))
            }
            _ => None,
        }
    }

    /// UNI-16: two layout-compatible types.
    fn layout_compatible(&self, a: &QTy, b: &QTy, depth: u32) -> bool {
        if same_unqual(a, b) {
            return true;
        }
        match (&a.t, &b.t) {
            (Ty::Enum(x), Ty::Enum(y)) => self.p.enums[*x].underlying == self.p.enums[*y].underlying,
            (Ty::Class(x), Ty::Class(y)) if depth < 8 => {
                let (Some((_, x)), Some((_, y))) = (self.decl_member_class(*x, 0), self.decl_member_class(*y, 0)) else { return false };
                let (fa, fb) = (self.fields(x), self.fields(y));
                fa.len() == fb.len() && fa.iter().zip(fb.iter()).all(|(p, q)| p.1.bits == q.1.bits && self.layout_compatible(&p.1.ty, &q.1.ty, depth + 1))
            }
            _ => false,
        }
    }

    /// A pointer or glvalue conversion between a class and its base.
    pub fn class_cast(&mut self, p: Ptr, from: &Ty, to: &Ty) -> R<Ptr> {
        let (Ty::Class(f), Ty::Class(t)) = (from, to) else { return Ok(p) };
        if f == t {
            return Ok(p);
        }
        if let Some(bp) = self.base_path(*f, *t) {
            let mut q = Ptr { obj: p.obj, path: full_path(&p), off: 0, array: false, bytes: false };
            if !p.array && p.off != 0 {
                return unsup("base conversion of a pointer past the end");
            }
            self.class_ok(*f)?;
            q.path.extend(bp);
            return Ok(self.canon(q));
        }
        if let Some(bp) = self.base_path(*t, *f) {
            // CAST-14, DER-17: the object must be a base subobject of a `to`.
            let fp = full_path(&p);
            let foreign = self.objs[p.obj].tu != self.tu;
            let ok = fp.len() >= bp.len() && fp[fp.len() - bp.len()..] == bp[..] && (foreign || self.class_at(&Ptr { obj: p.obj, path: fp[..fp.len() - bp.len()].to_vec(), off: 0, array: false, bytes: false }) == Some(*t));
            if !ok {
                return ub("DER-17", "cast to a derived class of an object that is not a base subobject of that class");
            }
            return Ok(Ptr { obj: p.obj, path: fp[..fp.len() - bp.len()].to_vec(), off: 0, array: false, bytes: false });
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
        let at = |path: Vec<usize>| Ptr { obj: p.obj, path, off: 0, array: false, bytes: false };
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
                    } else if i >= self.nslots(c) {
                        Ty::Class(*self.vbases_of(c).get(i - self.nslots(c))?)
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
            Val::Float(_) | Val::LD(_) => unsup("float where integer expected"),
            _ => unsup("non-integer value"),
        }
    }

    pub fn eval_bool(&mut self, e: &Expr) -> R<bool> {
        match self.eval(e)? {
            Val::Int(v) => Ok(v != 0),
            Val::Float(f) => Ok(f != 0.0),
            Val::LD(f) => Ok(!f.is_zero()),
            Val::Null => Ok(false),
            Val::Ptr(_) | Val::FnPtr(..) | Val::Addr(_) | Val::Member(..) => Ok(true),
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
            EK::Float(f) if e.ty.t == Ty::LDouble => Ok(Val::LD(F80::from_f64(*f))),
            EK::Float(f) => Ok(Val::Float(*f)),
            EK::LFloat(s) => match F80::parse(s) {
                Some(v) => Ok(Val::LD(v)),
                None => unsup("long double literal"),
            },
            EK::NullPtr => Ok(Val::Null),
            EK::ZeroInit => Ok(self.zero(&e.ty)),
            EK::MemberDefault => unsup("member with no designator outside an aggregate"),
            EK::Str(_) => {
                let p = self.lvalue(e)?;
                Ok(Val::Ptr(p))
            }
            EK::MemberPtr(d) => {
                // MPTR-2: the class of the type is the class named; the path
                // leads to the class that declares the member.
                let named = match &e.ty.t {
                    Ty::MemPtr(c, _) => *c,
                    _ => return unsup("pointer to member type"),
                };
                let decl = self.member_class(*d);
                Ok(Val::Member(*d, decl.and_then(|m| self.base_path(named, m))))
            }
            EK::Var(_) | EK::Member(..) | EK::Index(..) | EK::PtrMem(..) => {
                if e.cat == Cat::PR {
                    return unsup("prvalue member");
                }
                if matches!(e.ty.t, Ty::Func(_)) {
                    // A reference to a function: its place holds the
                    // function.
                    let p = self.lvalue(e)?;
                    return match self.get_at(&p)?.clone() {
                        v @ Val::FnPtr(..) => Ok(v),
                        _ => unsup("a reference to a function"),
                    };
                }
                if let Ty::Class(c) = &e.ty.t {
                    if self.copy_runs_code(*c) {
                        return self.copy_to_temp(e, *c);
                    }
                }
                let p = self.lvalue(e)?;
                self.read_gl(&p, &e.ty)
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
                self.read_gl(&p, &l.ty)
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
                    Ret::Place(p) => self.read_gl(&p, &e.ty),
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
                    self.ended_by_call.insert(p.obj);
                } else if !p.array && self.in_union(&p) {
                    // OBJ-1: the destroyed member of a union ends, so no
                    // member is active.
                    *self.slot(&p, true)? = Val::Inactive;
                } else if self.constant && !matches!(t.t, Ty::Class(_) | Ty::Array(..)) {
                    // CE-21 (Q-0447): a scalar subobject whose lifetime the
                    // call ended has no value.
                    if let Ok(v) = self.slot(&p, true) {
                        *v = Val::Uninit;
                    }
                    self.ended_parts.insert((p.obj, crate::interp::full_path(&p)));
                }
                Ok(Val::Void)
            }
            // NEW-37, LSUP-6: during constant evaluation, the global
            // nothrow forms are allocation functions of the
            // implementation (Q-0303), and they do not fail.
            EK::New { ty, count, init, alloc: Some(al), .. } if self.constant && al.nothrow && al.args.len() == 1 && al.func.is_some_and(|f| self.p.func(f).is_some_and(|fi| fi.class.is_none() && fi.fty.params.len() == 2 && matches!(&fi.fty.params[1].t, Ty::LRef(x) if matches!(x.t, Ty::Class(c) if self.p.classes[c].name == "nothrow_t")))) => self.eval_new(ty, count.as_deref(), init.as_deref()),
            EK::New { ty, count, init, alloc: Some(al), .. } => self.eval_new_user(ty, count.as_deref(), init.as_deref(), al),
            EK::New { ty, count, init, place: None, .. } => self.eval_new(ty, count.as_deref(), init.as_deref()),
            EK::New { ty, count, init, place: Some(pl), .. } => self.eval_placement_new(ty, count.as_deref(), init.as_deref(), pl),
            EK::Delete(arr, x, global) => {
                self.eval_delete(*arr, x, *global)?;
                Ok(Val::Void)
            }
            EK::Reinterpret(a, w) => {
                if e.cat != Cat::PR {
                    let p = self.lvalue(e)?;
                    return self.read_gl(&p, &e.ty);
                }
                self.eval_reinterpret(a, w, &e.ty)
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
                // THR-1: the parser marks a local that is moved as an xvalue.
                self.copy_class_cv(&place, &src, *c, x.cat == Cat::X, nonref(&x.ty).c)?;
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
                    // NEW-2: a floating size converts to an integer.
                    Val::Float(f) => f as i128,
                    _ => return unsup("array size"),
                };
                if n < 0 {
                    // NEW-35: an erroneous bound throws.
                    return Err(self.throw_std("bad_array_new_length")?);
                }
                if let Some(es) = self.p.sizeof_ty(&elem.t) {
                    let total = (n as u128).saturating_mul(es as u128);
                    if total > Ty::size_t().int_max() as u128 {
                        // NEW-35: the size does not fit the type of sizeof.
                        return Err(self.throw_std("bad_array_new_length")?);
                    }
                    if total > ALLOC_LIMIT as u128 {
                        // NEW-38: the allocation function fails.
                        return Err(self.throw_std("bad_alloc")?);
                    }
                }
                if let Some(Expr { k: EK::InitList(items), .. }) = init {
                    if items.len() as i128 > n {
                        // NEW-35: more initializers than the bound gives.
                        return Err(self.throw_std("bad_array_new_length")?);
                    }
                }
                let at = QTy::new(Ty::Array(Box::new(elem.clone()), Some(n as u64)));
                let place = self.alloc(&at)?;
                self.objs[place.obj].heap = Heap::Array;
                match init {
                    Some(e) => self.init_into(&place, e, &at)?,
                    None => self.default_init(&place, &at)?,
                }
                Ok(Val::Ptr(Ptr { obj: place.obj, path: vec![], off: 0, array: true, bytes: false }))
            }
        }
    }

    /// A global allocation or deallocation function called by name
    /// (STG-7, NEW-5, NEW-19): storage of bytes with no object yet.
    /// The library's allocation and deallocation at run time, for
    /// `malloc` and `free`.
    fn call_allocation_lib(&mut self, name: &str, args: &[Expr]) -> R<Ret> {
        if self.constant {
            return limit("CE-2", "call of a C library function");
        }
        self.call_allocation(name, args)
    }

    fn call_allocation(&mut self, name: &str, args: &[Expr]) -> R<Ret> {
        // LSUP-11, NEW-37: during constant evaluation, only the members of
        // std::allocator call ::operator new and ::operator delete by name.
        let in_allocator = self.frame().func.and_then(|f| self.p.func(f)).and_then(|f| f.class).is_some_and(|c| {
            let ci = &self.p.classes[c];
            let base = ci.name.split('<').next().unwrap_or("");
            base == "allocator" && self.p.scopes[ci.scope].parent.is_some_and(|ps| self.p.scopes[ps].name == "std" && self.p.scopes[ps].parent == Some(self.p.global))
        });
        if self.constant && !(in_allocator && matches!(name, "operatornew" | "operatordelete")) {
            return limit("CE-2", "call of an allocation function");
        }
        // NEW-19, NEW-26: the size and the alignment arguments, by type.
        let mut extra_size: Option<i128> = None;
        let mut align: Option<i128> = None;
        for a in args.iter().skip(1) {
            let is_align = matches!(nonref(&a.ty).t, Ty::Enum(en) if self.p.enums[en].name == "align_val_t");
            let is_size = nonref(&a.ty).t.is_integral();
            if !is_align && !is_size {
                continue;
            }
            let v = match self.eval(a)? {
                Val::Int(v) => v,
                _ => return unsup("allocation argument"),
            };
            if is_align {
                align = Some(v);
            } else {
                extra_size = Some(v);
            }
        }
        if name.starts_with("operatornew") {
            let n = match self.eval(&args[0])? {
                Val::Int(v) => v,
                _ => return unsup("allocation size"),
            };
            if let Some(a) = align {
                if a <= 0 || a & (a - 1) != 0 {
                    // NEW-27: the alignment is a power of two.
                    return ub("NEW-27", "aligned allocation with an alignment that is not a power of two");
                }
            }
            if n > ALLOC_LIMIT as i128 {
                // NEW-38: the allocation function fails. The form with a
                // `std::nothrow_t` argument returns a null pointer (LSUP-6,
                // NEW-31).
                let nothrow = args.iter().skip(1).any(|a| matches!(nonref(&a.ty).t, Ty::Class(c) if self.p.classes[c].name == "nothrow_t"));
                if nothrow {
                    return Ok(Ret::Val(Val::Null));
                }
                return Err(self.throw_std("bad_alloc")?);
            }
            if !(0..=1 << 20).contains(&n) {
                return unsup("allocation of a size the interpreter does not model");
            }
            let t = QTy::new(Ty::Array(Box::new(QTy::new(Ty::UChar)), Some(n.max(1) as u64)));
            let p = self.alloc(&t)?;
            self.objs[p.obj].heap = Heap::Raw;
            if name.ends_with("[]") {
                self.raw_array.insert(p.obj);
            }
            self.raw_args.insert(p.obj, (n, align));
            self.residents.entry(p.obj).or_default();
            return Ok(Ret::Val(Val::Ptr(Ptr { obj: p.obj, path: vec![], off: 0, array: true, bytes: false })));
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
        if self.objs[host].heap == Heap::Raw && self.raw_array.contains(&host) != name.ends_with("[]") {
            // NEW-40: the deallocation function must be the partner of the
            // allocation function.
            return ub("NEW-40", "deallocation by a function that is not the partner of the allocation function");
        }
        if self.objs[host].heap == Heap::Raw {
            if let Some(&(n, al)) = self.raw_args.get(&host) {
                if al != align {
                    // NEW-40: an aligned form with an aligned form, with the
                    // same alignment.
                    return ub("NEW-40", "deallocation whose alignment argument differs from the allocation's");
                }
                if extra_size.is_some_and(|s| s != n) {
                    return ub("NEW-40", "sized deallocation with a size other than the allocation's");
                }
            }
            if !self.objs[host].alive {
                return ub("NEW-40", "second deallocation of the same storage");
            }
        }
        if let Some(v) = self.residents.get(&host).cloned() {
            for (_, _, o) in v {
                self.objs[o].alive = false;
            }
        }
        self.objs[host].alive = false;
        Ok(Ret::Val(Val::Void))
    }

    /// A closed list of C library functions that a program declares with C
    /// language linkage and the interpreter knows (a front-end choice
    /// under decision 0035; the specification states no library):
    /// `_Exit`, `abort`, `exit`, `malloc`, `free`, `puts`, and `putchar`.
    /// None when the name or the parameters are not one of these.
    fn c_library_call(&mut self, name: &str, f: &FuncInfo, args: &[Expr]) -> R<Option<Ret>> {
        let ps = &f.fty.params;
        let int_param = ps.len() == 1 && ps[0].t == Ty::Int;
        let ptr_param = |q: &QTy, inner: Option<Ty>| matches!(&q.t, Ty::Ptr(x) if inner.as_ref().is_none_or(|t| x.t == *t));
        match name {
            "_Exit" | "exit" if int_param => {
                let Val::Int(st) = self.eval(&args[0])? else { return unsup("exit status") };
                // `exit` ends the static objects; `_Exit` ends no object.
                Err(Stop::Exit(st, name == "exit"))
            }
            "abort" if ps.is_empty() => Err(Stop::Terminate("abort", "the program called abort".into())),
            "malloc" if ps.len() == 1 && ps[0].t.is_integral() && ptr_param(&f.fty.ret, Some(Ty::Void)) => {
                let n = match self.eval(&args[0])? {
                    Val::Int(v) => v,
                    _ => return unsup("allocation size"),
                };
                if n > ALLOC_LIMIT as i128 {
                    return Ok(Some(Ret::Val(Val::Null)));
                }
                let size = Expr::new(EK::Int(n), QTy::new(Ty::size_t()), Cat::PR, self.line);
                self.call_allocation_lib("operatornew", &[size]).map(Some)
            }
            "free" if ps.len() == 1 && ptr_param(&ps[0], Some(Ty::Void)) => self.call_allocation_lib("operatordelete", args).map(Some),
            "puts" if ps.len() == 1 && ptr_param(&ps[0], None) => {
                // The text goes to the standard output, which a run does
                // not record; the result is a nonnegative value.
                self.eval(&args[0])?;
                Ok(Some(Ret::Val(Val::Int(1))))
            }
            "putchar" if int_param => {
                let Val::Int(c) = self.eval(&args[0])? else { return unsup("putchar argument") };
                Ok(Some(Ret::Val(Val::Int(c & 0xff))))
            }
            _ => Ok(None),
        }
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
        let mut carried: Vec<(i64, bytes::Byte)> = Vec::new();
        let mut replaced: Option<usize> = None;
        if let Some(v) = self.residents.get(&p.obj) {
            if let Some(&(_, len, o)) = v.iter().rev().find(|r| r.0 == off && self.objs[r.2].alive) {
                match self.obj_ty(o) {
                    Some(q) if q.unqual().t == t.t => return Ok(Ptr::whole(o)),
                    Some(QTy { t: Ty::Array(el, _), .. }) if el.unqual().t == t.t => return Ok(Ptr { obj: o, path: vec![], off: 0, array: true, bytes: false }),
                    _ if self.implicit.contains(&o) && off == 0 && (self.implicit_lifetime(&t.t) || (matches!(t.t, Ty::Class(_)) && !self.constant)) => {
                        // STG-8: a later use needs another set of objects.
                        // The set that serves every use holds the bytes
                        // that the program wrote (STG-12).
                        for k in 0..len {
                            if let Ok(b @ bytes::Byte::Known(_)) = self.read_byte(p.obj, k) {
                                carried.push((k, b));
                            }
                        }
                        self.objs[o].alive = false;
                        replaced = Some(o);
                    }
                    _ => return Ok(p),
                }
            }
        }
        let built = self.implicit_lifetime(&t.t);
        // STG-28: an array of a type that is not an implicit-lifetime type
        // also exists, and its elements begin their lifetimes when the
        // program constructs them.
        let unbuilt = !built && matches!(t.t, Ty::Class(_));
        if self.objs[p.obj].heap == Heap::Raw && off == 0 && (built || unbuilt) {
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
                    // Only two scalar types that the aliasing rules keep
                    // apart: a class can hold the old object as a member.
                    let old_el = replaced.and_then(|o| self.obj_ty(o)).map(|q| match q.t {
                        Ty::Array(el, _) => el.unqual().t,
                        x => x,
                    });
                    let scalars_apart = old_el.is_some_and(|a| {
                        let b = &t.t;
                        let scalar = |x: &Ty| !matches!(x, Ty::Class(_) | Ty::Array(..) | Ty::Unknown | Ty::Dep(_));
                        let same_int = a.is_integral() && b.is_integral() && a.int_bits() == b.int_bits();
                        scalar(&a) && scalar(b) && a != *b && !same_int && !self.is_byte_type(&a) && !self.is_byte_type(b)
                    });
                    if scalars_apart && replaced.is_some_and(|o| self.implicit_used.contains(&o)) {
                        // STG-18: the replaced object was used, so no one
                        // set of objects serves both uses.
                        self.no_set.insert(r.obj);
                    }
                    if built && !self.constant {
                        self.implicit.insert(r.obj);
                    } else {
                        // LSUP-11 (Q-0421): in constant evaluation, an
                        // element begins its lifetime only when the program
                        // constructs it.
                        for i in 0..n as usize {
                            self.unbuilt.insert((r.obj, i));
                        }
                    }
                    for (k, b) in carried {
                        if (k as u64) < n * size {
                            let _ = self.write_byte(r.obj, k, b);
                        }
                    }
                    if let Some(old) = replaced {
                        self.forward_pointers(old, r.obj);
                    }
                    return Ok(Ptr { obj: r.obj, path: vec![], off: 0, array: true, bytes: false });
                }
            }
        }
        Ok(p)
    }

    /// STG-8: the set of objects that the storage holds is the one that
    /// serves every use, so a pointer from an earlier use points to the
    /// object of the new set at the same byte with the same type.
    fn forward_pointers(&mut self, old: usize, new: usize) {
        for i in 0..self.objs.len() {
            let mut v = std::mem::replace(&mut self.objs[i].val, Val::Void);
            self.forward_val(old, new, &mut v);
            self.objs[i].val = v;
        }
        for fi in 0..self.frames.len() {
            let keys: Vec<DeclId> = self.frames[fi].refs.keys().copied().collect();
            for k in keys {
                if let Some(q) = self.forward_ptr(old, new, &self.frames[fi].refs[&k]) {
                    self.frames[fi].refs.insert(k, q);
                }
            }
        }
    }

    fn forward_val(&self, old: usize, new: usize, v: &mut Val) {
        match v {
            Val::Ptr(p) => {
                if let Some(q) = self.forward_ptr(old, new, p) {
                    *p = q;
                }
            }
            Val::Agg(items) => {
                for x in items {
                    self.forward_val(old, new, x);
                }
            }
            _ => {}
        }
    }

    fn forward_ptr(&self, old: usize, new: usize, p: &Ptr) -> Option<Ptr> {
        if p.obj != old || p.bytes {
            return None;
        }
        let (_, off) = self.byte_pos(p).ok()?;
        let t = self.path_offset(old, &p.path).ok()?.1;
        let et = match (&t.t, p.array) {
            (Ty::Array(el, _), true) => el.unqual(),
            _ => t.unqual(),
        };
        self.find_at(new, vec![], off, &et.t, 0)
    }

    /// The path of a subobject of `o` at byte `off` whose type is `t`.
    fn find_at(&self, o: usize, path: Vec<usize>, off: i64, t: &Ty, depth: u32) -> Option<Ptr> {
        let (base, ty) = self.path_offset(o, &path).ok()?;
        if depth > 16 {
            return None;
        }
        if base == off && ty.unqual().t == *t {
            return Some(Ptr { obj: o, path, off: 0, array: false, bytes: false });
        }
        match &ty.t {
            Ty::Array(el, Some(n)) => {
                let es = self.size_of(&el.t).ok()?.max(1);
                let i = (off - base) / es;
                if i < 0 || i as u64 >= *n {
                    return None;
                }
                let mut p2 = path.clone();
                p2.push(i as usize);
                self.find_at(o, p2, off, t, depth + 1)
            }
            Ty::Class(c) => {
                for (idx, po, _) in self.class_parts(*c).ok()? {
                    if base + po <= off {
                        let mut p2 = path.clone();
                        p2.push(idx);
                        if let Some(r) = self.find_at(o, p2, off, t, depth + 1) {
                            return Some(r);
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// The object that lives in the storage of the object `o`.
    pub fn resident_holder(&self, o: usize) -> Option<usize> {
        self.residents.get(&o)?.iter().rev().find(|r| self.objs[r.2].alive).map(|r| r.2)
    }

    /// The host object and byte offset of a pointer into a byte array
    /// that holds objects, or to such an object.
    pub fn byte_address(&self, p: &Ptr) -> Option<(usize, i64)> {
        if let Some(&(h, off)) = self.resident_of.get(&p.obj) {
            // The offset in the complete object, for a host that is a
            // member byte array.
            let off = self.resident_abs.get(&p.obj).copied().unwrap_or(off);
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
        // A byte array that is a member: the offset in the complete
        // object (STG-29).
        if self.residents.contains_key(&p.obj) && p.array && self.type_at(p).is_some_and(|t| self.is_byte_type(&t.t)) {
            return self.byte_pos(p).ok();
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
        let n = match count {
            None => None,
            Some(n) => match self.eval(n)? {
                Val::Int(v) => Some(v),
                // NEW-2: a floating size converts to an integer.
                Val::Float(f) => Some(f as i128),
                _ => return unsup("array size"),
            },
        };
        self.place_new_at(p, ty, n, init)
    }

    /// Create the object of a `new` expression in the storage at `p`, with
    /// `count` elements for an array (STG-4, STG-5, STG-9).
    fn place_new_at(&mut self, p: Ptr, ty: &QTy, count: Option<i128>, init: Option<&Expr>) -> R<Val> {
        let p = if p.bytes { self.byte_place(&p)? } else { p };
        let elem = ty.unqual();
        let t = match count {
            None => elem.clone(),
            Some(n) => {
                if n < 0 {
                    return Err(self.throw_std("bad_array_new_length")?);
                }
                QTy::new(Ty::Array(Box::new(elem.clone()), Some(n as u64)))
            }
        };
        let at = self.type_at(&p);
        let host = self.type_at(&Ptr { obj: p.obj, path: p.path.clone(), off: 0, array: false, bytes: false });
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
            // A new object ends the objects whose storage it takes (STG-6).
            // The overlap uses the byte offsets in the complete object, so
            // that objects in two rows of an array of byte arrays, or in
            // the byte arrays of two elements, do not overlap (STG-4).
            let (lo, hi) = (p.off, p.off + size as i64);
            let abs = self.byte_pos(&p).map(|x| x.1).unwrap_or(lo);
            if let Some(v) = self.residents.get(&p.obj).cloned() {
                for (o, s2, obj) in v {
                    let oa = self.resident_abs.get(&obj).copied().unwrap_or(o);
                    if oa < abs + size as i64 && abs < oa + s2 {
                        self.objs[obj].alive = false;
                    }
                }
            }
            let _ = hi;
            self.resident_abs.insert(r.obj, abs);
            self.residents.entry(p.obj).or_default().push((lo, size as i64, r.obj));
            self.resident_of.insert(r.obj, (p.obj, lo));
            let target = if count.is_some() { r.clone() } else { r.clone() };
            match init {
                Some(e) => self.init_into(&target, e, &t)?,
                None => self.default_init(&target, &t)?,
            }
            return Ok(Val::Ptr(if count.is_some() { Ptr { obj: r.obj, path: vec![], off: 0, array: true, bytes: false } } else { r }));
        }
        // STG-9: a new object of the same type in the place of an old one
        // replaces it.
        let Some(at) = at else { return unsup("placement new in storage of unknown type") };
        let whole0 = p.path.is_empty() && !p.array;
        if !p.path.is_empty() && !p.array && count.is_none() {
            let parent = Ptr { obj: p.obj, path: p.path[..p.path.len() - 1].to_vec(), off: 0, array: false, bytes: false };
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
        if let Some(&i) = full_path(&p).first() {
            self.unbuilt.remove(&(p.obj, i));
        }
        // UNI-5: an object made in a union member makes it the active
        // member, and the other members end.
        if !p.path.is_empty() && !p.array {
            if !self.constant {
                // UNI-14 (Q-0425): at run time, a new object inside a
                // member that is not active, such as a member of a member
                // union, is valid. The enclosing members become active, from
                // the outside in. Constant evaluation rejects it (UNI-6).
                for k in 1..p.path.len() {
                    let outer = Ptr { obj: p.obj, path: p.path[..k].to_vec(), off: 0, array: false, bytes: false };
                    let parent = Ptr { obj: p.obj, path: p.path[..k - 1].to_vec(), off: 0, array: false, bytes: false };
                    if let Ok(Val::Agg(items)) = self.get_at(&parent) {
                        if matches!(items.get(p.path[k - 1]), Some(Val::Inactive)) {
                            self.activate_member(&outer)?;
                        }
                    }
                }
            }
            self.activate_member(&p)?;
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

    /// A `new` expression whose allocation function a class or the
    /// program declares, or that has placement arguments (NEW-10, NEW-13,
    /// NEW-21 to NEW-24, NEW-31, NEW-32).
    fn eval_new_user(&mut self, ty: &QTy, count: Option<&Expr>, init: Option<&Expr>, al: &NewAlloc) -> R<Val> {
        let elem = ty.unqual();
        let array = count.is_some();
        let n = match count {
            None => None,
            Some(c) => match self.eval(c)? {
                Val::Int(v) => Some(v),
                // NEW-2: a floating size converts to an integer.
                Val::Float(f) => Some(f as i128),
                _ => return unsup("array size"),
            },
        };
        let es = self.size_of(&elem.t)? as i128;
        let size = es.saturating_mul(n.unwrap_or(1));
        // NEW-35: more initializers than the bound gives is an erroneous
        // bound too.
        let too_few = matches!((n, init), (Some(v), Some(Expr { k: EK::InitList(items), .. })) if items.len() as i128 > v);
        if n.is_some_and(|v| v < 0) || size > Ty::size_t().int_max() || too_few {
            // NEW-35: an erroneous bound calls no allocation function.
            // NEW-36: with a non-throwing allocation function the result
            // is null.
            if al.nothrow {
                return Ok(Val::Null);
            }
            return Err(self.throw_std("bad_array_new_length")?);
        }
        // The placement arguments, each held in a temporary so that the
        // deallocation function can receive them again.
        let base = self.synth.len();
        let mut pargs = Vec::new();
        for a in &al.args {
            let v = self.eval(a)?;
            let o = self.new_obj(v, true);
            self.objs[o].ty = Some(a.ty.unqual());
            self.synth.push(Ptr::whole(o));
            let i = self.synth.len() - 1;
            let pl = Expr::new(EK::Unknown(format!("\u{1}{}", i)), a.ty.unqual(), Cat::L, a.line);
            pargs.push(Expr::new(EK::Conv(Conv::LToR, Box::new(pl)), a.ty.unqual(), Cat::PR, a.line));
        }
        let size_e = Expr::new(EK::Int(size), QTy::new(Ty::size_t()), Cat::PR, self.line);
        let aname = if array { "operatornew[]" } else { "operatornew" };
        let r = (|| -> R<Val> {
            // STG-5, LSUP-10: the non-allocating placement form returns
            // its argument; constant evaluation runs it that way.
            let place_form = al.func.is_some_and(|f| self.p.func(f).is_some_and(|fi| fi.fty.params.len() == 2 && matches!(&fi.fty.params[1].t, Ty::Ptr(x) if x.t == Ty::Void))) && pargs.len() == 1;
            let storage = match al.func {
                Some(_) if place_form && self.constant => {
                    let in_construct_at = self.frame().func.is_some_and(|f| {
                        let d = &self.p.decls[f];
                        d.name == "construct_at" && self.p.scopes[d.scope].name == "std"
                    });
                    if !in_construct_at {
                        // NEW-37: only std::construct_at runs a placement
                        // form in constant evaluation (LSUP-10).
                        return limit("NEW-37", "placement new expression outside std::construct_at");
                    }
                    self.eval(&pargs[0])?
                }
                Some(_) if self.constant => {
                    // NEW-37: a class's or the program's allocation function.
                    return limit("NEW-37", "new expression that calls an allocation function the program declares");
                }
                None => match self.call_allocation(aname, std::slice::from_ref(&size_e))? {
                    Ret::Val(v) => {
                        // NEW-28: the storage has the type's alignment.
                        if let (Val::Ptr(p), Some(a)) = (&v, self.p.alignof_ty(&ty.t)) {
                            if a > 16 {
                                self.raw_align.insert(p.obj, a);
                            }
                        }
                        v
                    }
                    _ => return unsup("allocation result"),
                },
                Some(f) => {
                    let fty = self.p.func(f).map(|x| x.fty.clone()).unwrap();
                    let callee = Expr::new(EK::Func(f), QTy::new(Ty::Func(Box::new(fty.clone()))), Cat::L, self.line);
                    let mut args = vec![size_e.clone()];
                    args.extend(pargs.iter().cloned());
                    let call = Expr::new(EK::Call(Box::new(callee), args, Some(f)), fty.ret.clone(), Cat::PR, self.line);
                    match self.call(&call, None)? {
                        Ret::Val(v) => v,
                        _ => return unsup("allocation result"),
                    }
                }
            };
            let p = match storage {
                Val::Null if al.nothrow => {
                    // NEW-31: no initialization, and the result is null.
                    return Ok(Val::Null);
                }
                Val::Null => return ub("NEW-32", "an allocation function that is not non-throwing returns a null pointer"),
                Val::Ptr(p) => p,
                _ => return unsup("allocation function result that points to no object"),
            };
            let before = self.objs.len();
            let made = match self.place_new_at(p.clone(), ty, n, init) {
                Ok(v) => v,
                Err(Stop::Throw(x)) => {
                    // NEW-21: the deallocation function frees the storage
                    // before the exception leaves the expression. UNW-3,
                    // NEW-28: for a type with extended alignment, the
                    // library storage goes to the aligned form.
                    let al_ty = self.p.alignof_ty(&ty.t).filter(|&a| a > 16);
                    let global_dealloc = match &al.dealloc {
                        Dealloc::Library => true,
                        Dealloc::Func(f, _) => self.p.func(*f).is_some_and(|x| x.class.is_none()),
                        Dealloc::None => false,
                    };
                    match (&al.dealloc, al_ty) {
                        (_, Some(a)) if global_dealloc && pargs.is_empty() && al.func.is_none() => match self.aligned_delete_fn(None, array, true, a, false) {
                            Some((d, extra)) => self.call_dealloc(&d, &p, size, &extra, aname)?,
                            None => self.call_dealloc(&al.dealloc, &p, size, &pargs, aname)?,
                        },
                        _ => self.call_dealloc(&al.dealloc, &p, size, &pargs, aname)?,
                    }
                    return Err(Stop::Throw(x));
                }
                Err(e) => return Err(e),
            };
            if let Val::Ptr(q) = &made {
                if place_form && q.obj < before && !self.constant {
                    // STG-9: the non-allocating form built the object in the
                    // place of an object that already exists, such as an
                    // element of an array from new[]. The complete object
                    // keeps the way that it was created (NEW-5).
                    return Ok(made);
                }
                self.objs[q.obj].heap = if array { Heap::Array } else { Heap::Single };
                self.user_storage.insert(q.obj, p);
            }
            Ok(made)
        })();
        self.synth.truncate(base);
        r
    }

    /// Call a deallocation function with the storage `p`, the size, and
    /// the placement arguments (NEW-21 to NEW-24).
    fn call_dealloc(&mut self, d: &Dealloc, p: &Ptr, size: i128, pargs: &[Expr], aname: &str) -> R<()> {
        match d {
            Dealloc::None => Ok(()),
            Dealloc::Library => {
                let dname = if aname.ends_with("[]") { "operatordelete[]" } else { "operatordelete" };
                let o = self.new_obj(Val::Ptr(p.clone()), true);
                self.objs[o].ty = Some(QTy::new(Ty::Ptr(Box::new(QTy::new(Ty::Void)))));
                self.synth.push(Ptr::whole(o));
                let i = self.synth.len() - 1;
                let vp = QTy::new(Ty::Ptr(Box::new(QTy::new(Ty::Void))));
                let pl = Expr::new(EK::Unknown(format!("\u{1}{}", i)), vp.clone(), Cat::L, self.line);
                let a = Expr::new(EK::Conv(Conv::LToR, Box::new(pl)), vp, Cat::PR, self.line);
                self.call_allocation(dname, &[a])?;
                Ok(())
            }
            Dealloc::Func(f, sized) => {
                let f = *f;
                let fty = self.p.func(f).map(|x| x.fty.clone()).unwrap();
                let vp = QTy::new(Ty::Ptr(Box::new(QTy::new(Ty::Void))));
                let o = self.new_obj(Val::Ptr(p.clone()), true);
                self.objs[o].ty = Some(vp.clone());
                self.synth.push(Ptr::whole(o));
                let i = self.synth.len() - 1;
                let pl = Expr::new(EK::Unknown(format!("\u{1}{}", i)), vp.clone(), Cat::L, self.line);
                let mut args = vec![Expr::new(EK::Conv(Conv::LToR, Box::new(pl)), vp, Cat::PR, self.line)];
                if *sized {
                    args.push(Expr::new(EK::Int(size), QTy::new(Ty::size_t()), Cat::PR, self.line));
                }
                args.extend(pargs.iter().cloned());
                let callee = Expr::new(EK::Func(f), QTy::new(Ty::Func(Box::new(fty.clone()))), Cat::L, self.line);
                let call = Expr::new(EK::Call(Box::new(callee), args, Some(f)), fty.ret.clone(), Cat::PR, self.line);
                self.call(&call, None)?;
                Ok(())
            }
        }
    }

    /// A `delete` or `delete[]` expression (NEW-5, VIRT-12).
    fn eval_delete(&mut self, arr: bool, x: &Expr, global: bool) -> R<()> {
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
        let whole = if arr { p.array && p.off == 0 && p.path.is_empty() } else { !p.array && p.off == 0 && p.path.is_empty() };
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
                (Some(QTy { t: Ty::Class(_), .. }), Some(bc)) => self.class_dtor_virtual(bc, 0),
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
        // NEW-13: the deallocation function of the class of the object, or
        // the global one.
        let storage = self.user_storage.remove(&p.obj);
        let class = match &t.t {
            Ty::Class(c) => Some(*c),
            Ty::Array(el, _) => match el.t {
                Ty::Class(c) => Some(c),
                _ => None,
            },
            _ => None,
        };
        let elem_t = match &t.t {
            Ty::Array(el, _) => el.t.clone(),
            x => x.clone(),
        };
        let align = self.p.alignof_ty(&elem_t).filter(|&a| a > 16);
        let (d, extra) = match (tu == self.tu, align.and_then(|a| self.aligned_delete_fn(class, arr, global, a, true))) {
            (true, Some(x)) => x,
            (true, None) => (self.delete_fn(class, arr, global), vec![]),
            _ => (Dealloc::Library, vec![]),
        };
        let size = self.p.sizeof_ty(&t.t).unwrap_or(0) as i128;
        let aname = if arr { "operatornew[]" } else { "operatornew" };
        match (&d, storage) {
            (Dealloc::Func(..), Some(s)) => self.call_dealloc(&d, &s, size, &extra, aname)?,
            (Dealloc::Func(..), None) => {
                let whole = if arr { Ptr { obj: p.obj, path: vec![], off: 0, array: true, bytes: false } } else { Ptr::whole(p.obj) };
                self.call_dealloc(&d, &whole, size, &extra, aname)?
            }
            (_, Some(s)) => {
                let host = self.resident_of.get(&p.obj).map(|x| x.0).unwrap_or(s.obj);
                if self.objs[host].heap == Heap::Raw && self.objs[host].alive {
                    self.call_dealloc(&Dealloc::Library, &s, size, &[], aname)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// NEW-28: the aligned usual deallocation function for a type with
    /// alignment `a`, with the alignment argument; None when the class
    /// or the program declares none (NEW-29).
    fn aligned_delete_fn(&self, class: Option<usize>, arr: bool, global: bool, a: u64, delete_expr: bool) -> Option<(Dealloc, Vec<Expr>)> {
        let dname = if arr { "operatordelete[]" } else { "operatordelete" };
        let size_t = Ty::size_t();
        let (fns, from_class): (Vec<DeclId>, bool) = match class.filter(|_| !global).and_then(|c| self.p.lookup_member(c, dname)) {
            Some(v) if !v.is_empty() => (v, true),
            _ => (self.p.scopes[self.p.global].names.get(dname).cloned().unwrap_or_default(), false),
        };
        let is_align = |q: &QTy| matches!(q.t, Ty::Enum(e) if self.p.enums[e].name == "align_val_t");
        let form = |d: DeclId| -> Option<(bool, QTy)> {
            let f = self.p.func(d)?;
            let ps = &f.fty.params;
            let first = matches!(&ps.first().map(|q| q.t.clone()), Some(Ty::Ptr(x)) if x.t == Ty::Void);
            match ps.len() {
                2 if first && is_align(&ps[1]) => Some((false, ps[1].clone())),
                3 if first && ps[1].t == size_t && is_align(&ps[2]) => Some((true, ps[2].clone())),
                _ => None,
            }
        };
        // NEW-22, UNW-3: after `delete`, a single object of complete type
        // uses the global sized aligned form. A class's forms, and the
        // other cases, prefer the form without the size.
        let sized_first = delete_expr && !arr && !from_class;
        let pick = fns.iter().copied().find(|&d| form(d).is_some_and(|f| f.0 == sized_first)).or_else(|| fns.iter().copied().find(|&d| form(d).is_some()))?;
        let (sized, at) = form(pick)?;
        Some((Dealloc::Func(pick, sized), vec![Expr::new(EK::Int(a as i128), at.unqual(), Cat::PR, self.line)]))
    }

    /// The deallocation function of a `delete` expression: a usual form
    /// that the class or the program declares, with one parameter first.
    fn delete_fn(&self, class: Option<usize>, arr: bool, global: bool) -> Dealloc {
        let dname = if arr { "operatordelete[]" } else { "operatordelete" };
        let size_t = Ty::size_t();
        let fns: Vec<DeclId> = match class.filter(|_| !global).and_then(|c| self.p.lookup_member(c, dname)) {
            Some(v) if !v.is_empty() => v,
            _ => self.p.scopes[self.p.global].names.get(dname).cloned().unwrap_or_default(),
        };
        let usual = |d: DeclId| -> Option<bool> {
            let f = self.p.func(d)?;
            let first = matches!(&f.fty.params.first().map(|q| q.t.clone()), Some(Ty::Ptr(x)) if x.t == Ty::Void);
            match f.fty.params.len() {
                1 if first => Some(false),
                2 if first && f.fty.params[1].t == size_t => Some(true),
                _ => None,
            }
        };
        if let Some(d) = fns.iter().copied().find(|&d| usual(d) == Some(false)) {
            return Dealloc::Func(d, false);
        }
        if let Some(d) = fns.iter().copied().find(|&d| usual(d) == Some(true)) {
            return Dealloc::Func(d, true);
        }
        Dealloc::Library
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
                let v = self.read_gl(&p, &a.ty)?;
                if let (Ty::Enum(e), Val::Int(x)) = (&a.ty.t, &v) {
                    // ENUM-6: a stored value outside the range of an
                    // enumeration without a fixed type.
                    if self.p.enums[*e].fixed.is_none() && !self.p.enums[*e].scoped {
                        let (lo, hi) = self.p.enum_range(*e);
                        if *x < lo || *x > hi {
                            return ub("ENUM-6", format!("load of value {}, which is outside the range of the enumeration", x));
                        }
                    }
                }
                Ok(v)
            }
            Conv::ArrayDecay => {
                let p = if a.cat == Cat::PR { self.materialize(a)? } else { self.lvalue(a)? };
                if a.cat == Cat::PR && matches!(a.k, EK::InitList(_)) {
                    // ILC-5: the array behind a std::initializer_list lives
                    // as long as the list object, as a temporary that a
                    // reference binds does.
                    if let Some(c) = self.extend_temp(p.obj) {
                        self.extended.push(c);
                    }
                }
                Ok(Val::Ptr(Ptr { obj: p.obj, path: full_path(&p), off: 0, array: true, bytes: false }))
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
                    Val::LD(f) => !f.is_zero() as i128,
                    Val::Null => 0,
                    Val::Ptr(_) | Val::FnPtr(..) | Val::Addr(_) | Val::Member(..) => 1,
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
                if let (Ty::MemPtr(fc, _), Ty::MemPtr(tc, _), Val::Member(d, path)) = (&a.ty.t, &to.t, &v) {
                    // CONV-15, MPTR-4: the path follows the conversion.
                    let path = match (path, self.base_path(*tc, *fc), self.base_path(*fc, *tc)) {
                        (Some(pt), Some(mut up), _) => {
                            up.extend(pt.iter().copied());
                            Some(up)
                        }
                        (Some(pt), None, Some(down)) if pt.starts_with(&down) => Some(pt[down.len()..].to_vec()),
                        _ => None,
                    };
                    return Ok(Val::Member(*d, path));
                }
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
                        let raw = self.residents.contains_key(&p.obj) && self.objs[p.obj].heap == Heap::Raw && !p.bytes;
                        if self.constant && !raw {
                            // CE-4: a pointer to void to a pointer to an
                            // object type. STG-28, LSUP-11: the storage that
                            // std::allocator obtains is an array of T.
                            return limit("CE-4", "conversion from void*");
                        }
                        if self.constant {
                            // CE-4 (Q-0349): inside std::allocator<T>, only
                            // the conversion to T*.
                            let elem = self.frame().func.and_then(|f| self.p.func(f)).and_then(|f| f.class).and_then(|c| self.p.inst_of.get(&c)).and_then(|(_, args)| match args.first() {
                                Some(TArg::Type(q)) => Some(q.unqual().t),
                                _ => None,
                            });
                            if elem.is_some_and(|e| e != t.unqual().t) {
                                return limit("CE-4", "conversion of the storage of std::allocator<T> to a pointer to a type other than T");
                            }
                        }
                        if self.residents.contains_key(&p.obj) && !p.bytes {
                            return Ok(Val::Ptr(self.storage_pointer(p.clone(), t)?));
                        }
                        // STG-16: a static_cast through `void*` acts as a
                        // reinterpret_cast.
                        return Ok(Val::Ptr(self.retarget(p.clone(), t)?));
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
                if *to == Ty::LDouble {
                    return Ok(Val::LD(F80::from_i128(x)));
                }
                if to.is_floating() {
                    let f = x as f64;
                    return Ok(Val::Float(if *to == Ty::Float { f as f32 as f64 } else { f }));
                }
                Ok(Val::Int(x))
            }
            Val::Float(f) if *to == Ty::LDouble => Ok(Val::LD(F80::from_f64(f))),
            Val::LD(x) => {
                if let Some(ti) = to_int(to) {
                    if ti == Ty::Bool {
                        return Ok(Val::Int(!x.is_zero() as i128));
                    }
                    match x.to_i128() {
                        Some(t) if t >= ti.int_min() && t <= ti.int_max() => return Ok(Val::Int(t)),
                        _ => return ub("CONV-5", "floating value out of the range of the integer type"),
                    }
                }
                if *to == Ty::LDouble {
                    return Ok(Val::LD(x));
                }
                let f = x.to_f64();
                if to.is_floating() {
                    let r = if *to == Ty::Float { f as f32 as f64 } else { f };
                    // CONV-11: a value above the range gives infinity, as
                    // the matrix table gives for Clang and GCC on both
                    // targets, also in constant evaluation.
                    return Ok(Val::Float(r));
                }
                Ok(Val::LD(x))
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
                    // CONV-11: a value above the range gives infinity (the
                    // matrix table: Clang and GCC on both targets).
                    let r = f as f32;
                    return Ok(Val::Float(r as f64));
                }
                Ok(Val::Float(f))
            }
            v => Ok(v),
        }
    }

    /// The size of the pointee of `pt` when `p` is a pointer to a byte
    /// whose type is a pointer to another object type (STG-29, ADD-14).
    fn typed_step(&self, p: &Ptr, pt: &Ty) -> Option<i64> {
        let Ty::Ptr(el) = pt else { return None };
        let el = el.unqual().t;
        if el == Ty::Void || self.is_byte_type(&el) || matches!(el, Ty::Unknown | Ty::Dep(_)) {
            return None;
        }
        if !p.bytes {
            // An element of a byte array that a cast reached (STG-29).
            if !p.array || p.bytes {
                return None;
            }
            let base = Ptr { array: false, off: 0, ..p.clone() };
            match self.type_at(&base) {
                Some(QTy { t: Ty::Array(b, _), .. }) if self.is_byte_type(&b.unqual().t) => {}
                _ => return None,
            }
        }
        self.size_of(&el).ok().filter(|&s| s > 0)
    }

    /// Pointer arithmetic on a pointer of type `pt`. A pointer to a byte
    /// whose type is a pointer to another object type counts the byte as
    /// an array of one element of that type (ADD-14), and its step is the
    /// size of the type (Q-0430, provisional answer).
    pub fn ptr_add_ty(&self, p: &Ptr, n: i128, rule: &'static str, pt: &Ty) -> R<Ptr> {
        if let Some(size) = self.typed_step(p, pt) {
            if !(-1..=1).contains(&n) {
                return ub("ADD-14", "arithmetic on a pointer to a byte moves past one element of its type");
            }
            return self.ptr_add(p, n * size as i128, rule);
        }
        self.ptr_add(p, n, rule)
    }

    /// Pointer arithmetic: `rule` names the operation for an error.
    pub fn ptr_add(&self, p: &Ptr, n: i128, rule: &'static str) -> R<Ptr> {
        if p.bytes {
            // STG-13: the bytes of the object act as an array.
            let no = p.off as i128 + n;
            if no < 0 || no > self.obj_size(p.obj)? as i128 {
                return ub(rule, "byte pointer arithmetic outside the object");
            }
            return Ok(Ptr { off: no as i64, ..p.clone() });
        }
        if p.array {
            let base = Ptr { array: false, off: 0, ..p.clone() };
            let len = match self.get_at(&base)? {
                Val::Agg(items) => items.len() as i128,
                // STG-10, UNI-14: an array member of a union that is not
                // active has no elements, and its pointers still step
                // over the positions that its type gives.
                _ => match self.type_at(&base) {
                    Some(QTy { t: Ty::Array(_, Some(n)), .. }) => n as i128,
                    _ => return unsup("pointer arithmetic"),
                },
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
                EK::Var(_) | EK::Int(_) | EK::This => true,
                // The implicit object of a member function (`*this`).
                EK::Unary(UOp::Deref, x) => matches!(x.k, EK::This),
                EK::Member(o, _) => plain(o),
                EK::Index(a, i) => plain(a) && plain(i),
                EK::Conv(Conv::ArrayDecay | Conv::LToR | Conv::NoOp | Conv::Arith, x) => plain(x),
                _ => false,
            }
        }
        if !plain(l) {
            return Ok(());
        }
        // A chain from `*this` activates only the steps it names, and not
        // the member that the object itself may be.
        fn from_this(e: &Expr) -> bool {
            match &e.k {
                EK::Unary(UOp::Deref, x) => matches!(x.k, EK::This),
                EK::Member(o, _) => from_this(o),
                EK::Index(a, _) => from_this(a),
                EK::Conv(_, x) => from_this(x),
                _ => false,
            }
        }
        let this_root = from_this(l);
        match &l.k {
            EK::Member(o, _) => {
                self.activate_chain(o)?;
                if matches!(o.k, EK::Member(..) | EK::Index(..)) {
                    let p = self.lvalue(o)?;
                    if this_root {
                        self.activate_last(&p, Self::chain_len(o))?;
                    } else {
                        self.activate(&p)?;
                    }
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
                    if this_root {
                        self.activate_last(&p, Self::chain_len(x))?;
                    } else {
                        self.activate(&p)?;
                    }
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
        self.activate_last(place, usize::MAX)
    }

    /// The number of member accesses and subscripts that name the left
    /// operand of an assignment, from its innermost operand (UNI-13).
    fn chain_len(e: &Expr) -> usize {
        match &e.k {
            EK::Member(o, _) => 1 + Self::chain_len(o),
            EK::Index(a, _) => 1 + Self::chain_len(a),
            EK::Conv(Conv::ArrayDecay | Conv::NoOp, x) => Self::chain_len(x),
            _ => 0,
        }
    }

    /// UNI-5, UNI-13: as `activate`, for the last `n` steps of the path
    /// only. A step before them is not named by the assignment (as in
    /// `this->v` inside a member function of an inactive member).
    pub fn activate_last(&mut self, place: &Ptr, n: usize) -> R<()> {
        let fp = full_path(place);
        let first = fp.len().saturating_sub(n);
        let mut t = self.obj_ty(place.obj).map(|q| q.t);
        for (k, &i) in fp.iter().enumerate() {
            if k < first {
                t = match &t {
                    Some(Ty::Class(c)) => {
                        let c = *c;
                        let nb = self.nbases(c);
                        if i < nb { self.p.classes[c].bases[i].class.map(Ty::Class) } else { self.fields(c).into_iter().find(|(_, fi)| fi.index == i - nb).map(|x| x.1.ty.t) }
                    }
                    Some(Ty::Array(el, _)) => Some(el.t.clone()),
                    _ => None,
                };
                continue;
            }
            let next = match &t {
                Some(Ty::Class(c)) => {
                    let c = *c;
                    let nb = self.nbases(c);
                    let ft = if i < nb { self.p.classes[c].bases[i].class.map(|b| QTy::new(Ty::Class(b))) } else { self.fields(c).into_iter().find(|(_, fi)| fi.index == i - nb).map(|x| x.1.ty) };
                    if self.p.classes[c].key == 2 {
                        let holder = Ptr { obj: place.obj, path: fp[..k].to_vec(), off: 0, array: false, bytes: false };
                        if let (Ok(Val::Agg(items)), Some(ft)) = (self.get_at(&holder), &ft) {
                            // UNI-13: only a member whose default constructor
                            // is trivial becomes active. For an array member,
                            // the default constructor of its element class.
                            let mut et = &ft.t;
                            while let Ty::Array(el, _) = et {
                                et = &el.t;
                            }
                            let trivial = match et {
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

    /// Whether the place is a member of a union.
    fn in_union(&self, p: &Ptr) -> bool {
        let n = p.path.len();
        if n == 0 {
            return false;
        }
        let parent = Ptr { obj: p.obj, path: p.path[..n - 1].to_vec(), off: 0, array: false, bytes: false };
        matches!(self.class_at(&parent), Some(c) if self.p.classes[c].key == 2)
    }

    /// Make the union member at `p` active, whatever its default
    /// constructor (placement `new` constructs it).
    fn activate_member(&mut self, p: &Ptr) -> R<()> {
        if !self.in_union(p) {
            return Ok(());
        }
        let n = p.path.len();
        let i = p.path[n - 1];
        let holder = Ptr { obj: p.obj, path: p.path[..n - 1].to_vec(), off: 0, array: false, bytes: false };
        let Some(ft) = self.type_at(p) else { return Ok(()) };
        let sh = if matches!(ft.t, Ty::Class(_) | Ty::Array(..)) { self.shape(&ft)? } else { Val::Uninit };
        if let Val::Agg(items) = self.slot(&holder, true)? {
            for (j, it) in items.iter_mut().enumerate() {
                if j == i {
                    if matches!(it, Val::Inactive) {
                        *it = sh.clone();
                    }
                } else {
                    *it = Val::Inactive;
                }
            }
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
        let old = self.read_gl(place, &a.ty)?;
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
            Val::LD(f) => Val::LD(f.add(F80::from_i128(delta))),
            Val::Ptr(p) => Val::Ptr(self.ptr_add_ty(p, delta, ptr_rule, &a.ty.t)?),
            Val::Null => return ub(ptr_rule, "increment of a null pointer"),
            _ => return unsup("increment"),
        };
        let nv = self.bitfield_value(a, nv);
        self.write_gl(place, &a.ty, nv)?;
        Ok(old)
    }

    /// BF-10: a bit-field keeps the low bits of the value.
    fn bitfield_value(&self, target: &Expr, v: Val) -> Val {
        if let EK::Var(id) = &target.k {
            // SB-10: a binding that names a bit-field.
            if let Some(e) = self.p.sb_place.get(id) {
                return self.bitfield_value(&e.clone(), v);
            }
        }
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
            // SEQ-2: the right operand first.
            let (src, mv) = if r.cat == Cat::PR { (self.materialize(r)?, true) } else { (self.lvalue(r)?, r.cat == Cat::X) };
            let src = self.class_cast(src, &nonref(&r.ty).t, &Ty::Class(c))?;
            let place = self.lvalue(l)?;
            self.activate_last(&place, Self::chain_len(l))?;
            self.assign_parts(&place, &src, c, mv)?;
            return Ok(place);
        }
        // SEQ-2: the right operand first, then the left. INIT-1: a copy of
        // an `unsigned char` or `std::byte` with no value has no value.
        let rv = match &r.k {
            EK::Conv(Conv::LToR, ra) if op.is_none() && !self.constant && (l.ty.t == Ty::UChar || matches!(&l.ty.t, Ty::Enum(x) if self.p.enums[*x].name == "byte")) && ra.ty.t == l.ty.t => {
                let p = self.lvalue(ra)?;
                self.read_gl_raw(&p, &ra.ty)?
            }
            _ => self.eval(r)?,
        };
        if op.is_none() {
            // UNI-13: the union members on a chain of member accesses
            // and subscripts become active before the place is formed.
            self.activate_chain(l)?;
        }
        let place = self.lvalue(l)?;
        if op.is_none() {
            self.activate_last(&place, Self::chain_len(l))?;
        }
        let nv = match op {
            None => rv,
            Some((bop, ct)) => {
                let old = self.read_gl(&place, &l.ty)?;
                if let Val::Ptr(_) | Val::Null = old {
                    // `p += n` and `p -= n`.
                    let Val::Int(n) = rv else { return unsup("pointer compound assignment") };
                    let n = if *bop == BOp::Sub { -n } else { n };
                    match old {
                        Val::Ptr(p) => Val::Ptr(self.ptr_add_ty(&p, n, "ADD-10", &l.ty.t)?),
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
        self.write_gl(&place, &l.ty, nv)?;
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
                Val::LD(f) => Ok(Val::LD(f.neg())),
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
                self.read_gl(&p, &e.ty)
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
                self.read_gl(&place, &a.ty)
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
        let mut right_first = false;
        if !self.constant && !matches!(op, BOp::Shl | BOp::Shr) && has_effects(a) && has_effects(b) {
            // SEQ-8: the operands of a built-in operator are unsequenced.
            right_first = self.decide(2) == 1;
            self.choice("SEQ-8", format!("the {} operand of '{}' first", if right_first { "right" } else { "left" }, op.spelling()));
        }
        // SEQ-4: the operands are unsequenced (shifts excepted, SHF-8), so
        // the run records their accesses to the same frame's objects.
        let watch = !self.constant && !matches!(op, BOp::Shl | BOp::Shr) && (may_conflict(a) || may_conflict(b));
        let mark = self.acc.len();
        if watch {
            let d = self.frames.len();
            self.seq_watch.push(d);
        }
        let r0 = (|| -> R<(Val, Val, usize)> {
            if right_first {
                let r = self.eval(b)?;
                let mid = self.acc.len();
                let l = self.eval(a)?;
                Ok((l, r, mid))
            } else {
                let l = self.eval(a)?;
                let mid = self.acc.len();
                let r = self.eval(b)?;
                Ok((l, r, mid))
            }
        })();
        if watch {
            self.seq_watch.pop();
        }
        let (l, r, mid) = match r0 {
            Ok(x) => x,
            Err(e) => {
                if self.seq_watch.is_empty() {
                    self.acc.truncate(mark);
                }
                return Err(e);
            }
        };
        if watch {
            let depth = self.frames.len();
            let conflict = {
                let (x, y) = self.acc[mark..].split_at(mid - mark);
                x.iter().filter(|e| e.3 == depth).any(|e| y.iter().filter(|f| f.3 == depth).any(|f| e.0 == f.0 && (e.2 || f.2) && (e.1.starts_with(&f.1) || f.1.starts_with(&e.1))))
            };
            if self.seq_watch.is_empty() {
                self.acc.truncate(mark);
            }
            if conflict {
                return ub("SEQ-4", format!("unsequenced accesses to one memory location in the operands of '{}', at least one a modification", op.spelling()));
            }
        }
        let ct = a.ty.t.clone();
        match (&l, &r) {
            (Val::Ptr(_) | Val::Null, _) | (_, Val::Ptr(_) | Val::Null) | (Val::FnPtr(..), _) | (_, Val::FnPtr(..)) | (Val::Member(..), _) | (_, Val::Member(..)) => {
                let pt = if matches!(a.ty.t, Ty::Ptr(_)) { a.ty.t.clone() } else { b.ty.t.clone() };
                return self.ptr_binary(op, l, r, &pt);
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

    /// CMP-26: `p` points past the end of one row of an array of arrays
    /// and `q` to the first element of the next row, or the reverse.
    fn rows_meet(&self, p: &Ptr, q: &Ptr) -> bool {
        let one = |a: &Ptr, b: &Ptr| -> bool {
            if !a.array || !b.array || a.bytes || b.bytes || a.obj != b.obj || a.path.is_empty() || a.path.len() != b.path.len() || b.off != 0 || !self.is_past_end(a) {
                return false;
            }
            let n = a.path.len();
            a.path[..n - 1] == b.path[..n - 1]
                && b.path[n - 1] == a.path[n - 1] + 1
                && matches!(self.type_at(&Ptr { obj: a.obj, path: a.path[..n - 1].to_vec(), off: 0, array: false, bytes: false }).map(|t| t.t), Some(Ty::Array(..)))
        };
        one(p, q) || one(q, p)
    }

    fn is_past_end(&self, p: &Ptr) -> bool {
        if p.bytes {
            return self.obj_size(p.obj).is_ok_and(|n| p.off == n);
        }
        if p.array {
            matches!(self.get_at(&Ptr { array: false, off: 0, ..p.clone() }), Ok(Val::Agg(items)) if p.off as usize == items.len())
        } else {
            p.off == 1
        }
    }

    /// The class that declares a data member or a member function.
    fn member_class(&self, d: DeclId) -> Option<usize> {
        match &self.p.decls[d].k {
            DK::Field(f) => Some(f.class),
            DK::Func(f) => f.class,
            _ => None,
        }
    }

    /// The place that `obj.*mp` names for a data member (MPTR-5, MPTR-6).
    fn ptr_mem_place(&mut self, obj: &Expr, mp: &Expr) -> R<Ptr> {
        // SEQ-2: the object first.
        let place = if obj.cat == Cat::PR { self.materialize(obj)? } else { self.lvalue(obj)? };
        let (d, path) = match self.eval(mp)? {
            Val::Member(d, path) => (d, path),
            Val::Null => return ub("MPTR-6", "'.*' or '->*' with a null pointer to member"),
            _ => return unsup("pointer to member value"),
        };
        let fi = match &self.p.decls[d].k {
            DK::Field(f) => f.clone(),
            _ => return unsup("pointer to member function used as an object"),
        };
        let pc = match &nonref(&mp.ty).t {
            Ty::MemPtr(c, _) => *c,
            _ => fi.class,
        };
        let p = self.member_object(place, &nonref(&obj.ty).t, pc, fi.class, path)?;
        let mut q = Ptr { obj: p.obj, path: full_path(&p), off: 0, array: false, bytes: false };
        q.path.push(self.nbases(fi.class) + fi.index);
        Ok(q)
    }

    /// The subobject of class `mc` in the object at `p`, whose static
    /// type is `st`. MPTR-6: the object must have the member.
    fn member_object(&mut self, p: Ptr, st: &Ty, pc: usize, mc: usize, path: Option<Vec<usize>>) -> R<Ptr> {
        let Ty::Class(sc) = st else { return unsup("pointer to member access on a non-class object") };
        if let Some(path) = path {
            if self.base_path(*sc, pc).is_some() {
                // CMP-22: the path that the conversions of the pointer took.
                let mut q = self.class_cast(p, st, &Ty::Class(pc))?;
                q.path = full_path(&q);
                q.off = 0;
                q.array = false;
                q.path.extend(path);
                return Ok(self.canon(q));
            }
        }
        if self.base_path(*sc, mc).is_some() {
            return self.class_cast(p, st, &Ty::Class(mc));
        }
        // A member of a derived class, through a pointer to member that a
        // cast made: the object's dynamic class must have it.
        match self.dynamic_class(&p) {
            Some((dc, dpath)) if self.base_path(dc, mc).is_some() => {
                let mut path = dpath;
                path.extend(self.base_path(dc, mc).unwrap());
                Ok(self.canon(Ptr { obj: p.obj, path, off: 0, array: false, bytes: false }))
            }
            Some(_) => ub("MPTR-6", "'.*' or '->*' with an object that does not have the member"),
            None => unsup("dynamic type of an object the interpreter cannot see"),
        }
    }

    fn ptr_binary(&mut self, op: BOp, l: Val, r: Val, pt: &Ty) -> R<Val> {
        if matches!(l, Val::Member(..)) || matches!(r, Val::Member(..)) {
            if !matches!(op, BOp::Eq | BOp::Ne) {
                return unsup("pointer to member operation");
            }
            let eq = match (&l, &r) {
                (Val::Member(a, _), Val::Member(b, _)) if a != b && matches!((&self.p.decls[*a].k, &self.p.decls[*b].k), (DK::Field(x), DK::Field(y)) if x.class == y.class && self.p.classes[x.class].key == 2) => {
                    // MPTR-5: the members of a union have one address.
                    true
                }
                (Val::Member(a, Some(pa)), Val::Member(b, Some(pb))) if a == b && pa.len() == pb.len() && pa != pb => {
                    // CMP-22: the same member of two different base
                    // subobjects.
                    false
                }
                (Val::Member(a, _), Val::Member(b, _)) => {
                    let virt = |m: &Self, d: DeclId| m.p.func(d).is_some() && m.is_virtual_fn(d);
                    if !self.constant && (virt(self, *a) || virt(self, *b)) {
                        // MPTR-7: the result is unspecified.
                        let c = self.decide(2);
                        self.choice("MPTR-7", if c == 0 { "pointers to virtual member functions compare by the function they name" } else { "pointers to virtual member functions compare unequal" });
                        if c == 1 {
                            false
                        } else {
                            a == b
                        }
                    } else {
                        a == b
                    }
                }
                (Val::Null, Val::Null) => true,
                _ => false,
            };
            return Ok(Val::Int((if op == BOp::Eq { eq } else { !eq }) as i128));
        }
        if let Some(v) = self.addr_binary(op, &l, &r)? {
            return Ok(v);
        }
        match op {
            BOp::Add | BOp::Sub => match (&l, &r) {
                (Val::Ptr(p), Val::Int(n)) => {
                    let n = if op == BOp::Sub { -n } else { *n };
                    Ok(Val::Ptr(self.ptr_add_ty(p, n, "ADD-10", pt)?))
                }
                (Val::Int(n), Val::Ptr(p)) if op == BOp::Add => Ok(Val::Ptr(self.ptr_add_ty(p, *n, "ADD-10", pt)?)),
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
                    let mut d = p.off as i128 - q.off as i128;
                    if let Some(size) = self.typed_step(p, pt).filter(|_| self.typed_step(q, pt).is_some()) {
                        // ADD-14 (Q-0430): two typed pointers to bytes
                        // count in units of their type.
                        if d % size as i128 != 0 {
                            return ub("ADD-14", "subtraction of pointers to different bytes as arrays of one");
                        }
                        d /= size as i128;
                    }
                    let pt = Ty::ptrdiff_t();
                    if d < pt.int_min() || d > pt.int_max() {
                        // ADD-18: the difference does not fit in the type
                        // of the result.
                        return ub("ADD-18", "the difference of two pointers does not fit in the type of the result");
                    }
                    Ok(Val::Int(d))
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
                            let root = self.string_root_of(p.obj);
                            if !self.constant && p.obj != q.obj && root == self.string_root_of(q.obj) {
                                // PRIM-9: two evaluations can give one
                                // object or two. One choice per set of
                                // characters.
                                let two = match self.prim9.get(&root) {
                                    Some(&c) => c == 1,
                                    None => {
                                        let c = self.decide(2);
                                        self.prim9.insert(root, c);
                                        c == 1
                                    }
                                };
                                self.choice("PRIM-9", if two { "string literals with the same characters are distinct objects" } else { "string literals with the same characters are one object" });
                                if two {
                                    return Ok(Val::Int((op != BOp::Eq) as i128));
                                }
                                let eq = p.off == q.off && p.path == q.path;
                                return Ok(Val::Int((if op == BOp::Eq { eq } else { !eq }) as i128));
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
                            if self.is_past_end(p) != self.is_past_end(q) {
                                if self.constant {
                                    return limit("CMP-20", "comparison of a past-the-end pointer with a pointer to another object");
                                }
                                let equal = self.decide(2) == 1;
                                self.choice("CMP-20", if equal { "a past-the-end pointer compares equal to a pointer to another object" } else { "a past-the-end pointer compares unequal to a pointer to another object" });
                                if equal {
                                    return Ok(Val::Int((op == BOp::Eq) as i128));
                                }
                            }
                            false
                        } else if !p.bytes && !q.bytes && !self.is_past_end(p) && !self.is_past_end(q) && self.addr_key(p) != self.addr_key(q) && matches!((self.byte_pos(p), self.byte_pos(q)), (Ok(_), Ok(_))) {
                            // CLS-19, CLS-30: subobjects at the same byte,
                            // such as empty bases and the first member.
                            self.byte_pos(p).ok() == self.byte_pos(q).ok()
                        } else if self.addr_key(p) != self.addr_key(q) && self.rows_meet(p, q) {
                            // CMP-26: the end of one row of an array of
                            // arrays and the start of the next are equal.
                            true
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
                        let rev = self.decide(2) == 1;
                        self.choice("CMP-8", if rev { "pointers to unrelated objects compare in the reverse of the order the objects were created" } else { "pointers to unrelated objects compare in the order the objects were created" });
                        if rev {
                            (vec![-(p.obj as i64)], vec![-(q.obj as i64)])
                        } else {
                            (vec![p.obj as i64], vec![q.obj as i64])
                        }
                    } else if self.rows_meet(p, q) {
                        // CMP-26: equal pointers.
                        (vec![0], vec![0])
                    } else {
                        if self.constant && !p.bytes && !q.bytes {
                            // CMP-8 (Q-0387): no order for a base class
                            // subobject against another subobject of the
                            // same object.
                            let k = p.path.iter().zip(q.path.iter()).take_while(|(a, b)| a == b).count();
                            if k < p.path.len() && k < q.path.len() {
                                let at = Ptr { obj: p.obj, path: p.path[..k].to_vec(), off: 0, array: false, bytes: false };
                                if let Some(QTy { t: Ty::Class(c), .. }) = self.type_at(&at) {
                                    let (nb, ns) = (self.nbases(c), self.nslots(c));
                                    let base = |i: usize| i < nb || i >= ns;
                                    if base(p.path[k]) || base(q.path[k]) {
                                        return limit("CE-15", "relational comparison of pointers that involve a base class subobject");
                                    }
                                }
                            }
                        }
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
                        // MUL-8: an unsigned product wraps; the low bits of
                        // the 128-bit product are exact.
                        let v = if signed { x.checked_mul(y).unwrap_or(i128::MAX) } else { (x as u128).wrapping_mul(y as u128) as i128 };
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
            (Val::LD(x), Val::LD(y)) => {
                let fin = |v: F80, rule: &'static str| -> R<Val> {
                    if !v.is_finite() && x.is_finite() && y.is_finite() {
                        return ub(rule, "floating result out of the range of its type");
                    }
                    Ok(Val::LD(v))
                };
                let c = x.partial_cmp(y);
                use std::cmp::Ordering::*;
                match op {
                    BOp::Add => fin(x.add(y), "ADD-17"),
                    BOp::Sub => fin(x.sub(y), "ADD-17"),
                    BOp::Mul => fin(x.mul(y), "MUL-12"),
                    BOp::Div => {
                        if y.is_zero() {
                            if self.constant {
                                return limit("CE-28", "floating division by zero in constant evaluation");
                            }
                            // MUL-11, MUL-20: IEEE 754 on the matrix targets.
                            return Ok(Val::LD(x.div(y)));
                        }
                        fin(x.div(y), "MUL-12")
                    }
                    BOp::Lt => Ok(Val::Int((c == Some(Less)) as i128)),
                    BOp::Gt => Ok(Val::Int((c == Some(Greater)) as i128)),
                    BOp::Le => Ok(Val::Int(matches!(c, Some(Less | Equal)) as i128)),
                    BOp::Ge => Ok(Val::Int(matches!(c, Some(Greater | Equal)) as i128)),
                    BOp::Eq => Ok(Val::Int((c == Some(Equal)) as i128)),
                    BOp::Ne => Ok(Val::Int((c != Some(Equal)) as i128)),
                    _ => unsup("floating operator"),
                }
            }
            (Val::LD(x), Val::Float(y)) => self.arith(op, Val::LD(x), Val::LD(F80::from_f64(y)), ct, rt),
            (Val::Float(x), Val::LD(y)) => self.arith(op, Val::LD(F80::from_f64(x)), Val::LD(y), ct, rt),
            (Val::LD(x), Val::Int(y)) => self.arith(op, Val::LD(x), Val::LD(F80::from_i128(y)), ct, rt),
            (Val::Int(x), Val::LD(y)) => self.arith(op, Val::LD(F80::from_i128(x)), Val::LD(y), ct, rt),
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
                        // capture, in order. UNW-2: when a capture throws,
                        // the members built before it end.
                        let mut done = Vec::new();
                        let ext = self.extended.len();
                        let r = (|| -> R<()> {
                            for (i, (_, fi)) in self.fields(c).into_iter().enumerate() {
                                let Some(x) = inits.get(i) else { break };
                                if fi.ty.t.is_ref() {
                                    // LAM-7: an init capture by reference extends
                                    // a temporary as a reference variable does.
                                    let p = self.bind_ref(x)?;
                                    if super::stmt::binds_temporary_p(self.p, x) && !matches!(x.k, EK::Var(_)) {
                                        if let Some(c) = self.extend_temp(p.obj) {
                                            done.push(c.clone());
                                            self.extended.push(c);
                                        }
                                    }
                                    self.write(&place.sub(fi.index), Val::Ptr(p), true)?;
                                    continue;
                                }
                                self.init_into(&place.sub(fi.index), x, &fi.ty)?;
                                done.push(Cleanup { place: place.sub(fi.index), ty: fi.ty.clone(), item: 0 });
                            }
                            Ok(())
                        })();
                        if let Err(e) = r {
                            self.extended.truncate(ext);
                            return Err(self.unwind(done, e));
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
                    EK::InitList(items) => {
                        // REF-5 (Q-0040): parenthesized aggregate
                        // initialization does not extend a temporary.
                        let save = std::mem::replace(&mut self.paren_aggregate, e.paren);
                        let r = self.init_aggregate(place, items, c);
                        self.paren_aggregate = save;
                        return r;
                    }
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
                    EK::ZeroInit => return self.value_init(place, ty),
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
                if (ty.t == Ty::UChar || matches!(&ty.t, Ty::Enum(x) if self.p.enums[*x].name == "byte")) && !self.constant {
                    if let EK::Conv(Conv::LToR, a) = &e.k {
                        if a.ty.t == ty.t {
                            let p = self.lvalue(a)?;
                            let v = self.read_gl_raw(&p, &a.ty)?;
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
    /// UNW-2: when an initializer throws, the parts built before it end in
    /// reverse order, with the temporaries that reference members extend.
    fn init_aggregate(&mut self, place: &Ptr, items: &[Expr], c: usize) -> R<()> {
        let ext = self.extended.len();
        let mut done = Vec::new();
        match self.init_aggregate_parts(place, items, c, &mut done, ext) {
            Err(e) => {
                self.extended.truncate(ext);
                Err(self.unwind(done, e))
            }
            r => r,
        }
    }

    fn init_aggregate_parts(&mut self, place: &Ptr, items: &[Expr], c: usize, done: &mut Vec<Cleanup>, ext: usize) -> R<()> {
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
            done.push(Cleanup { place: place.sub(i), ty: bt, item: 0 });
        }
        let fields = self.fields(c);
        // UNI-4: with no value given, the member with a default member
        // initializer is active, else the first.
        // LIST-5: a designated list of a union names its active member.
        let active = if items.len() <= k { fields.iter().position(|(_, x)| x.init.is_some()).unwrap_or(0) } else { items[k..].iter().position(|x| !matches!(x.k, EK::MemberDefault)).unwrap_or(0) };
        for (j, (fd, fi)) in fields.iter().enumerate() {
            let sub = place.sub(nb + fi.index);
            if fi.bits.is_some() && self.p.decls[*fd].name.is_empty() {
                // BF-7: an unnamed bit-field takes no element of the list.
                continue;
            }
            if union && j != active {
                self.write(&sub, Val::Inactive, true)?;
                if j < active && k < items.len() {
                    k += 1;
                }
                continue;
            }
            if union && !fi.ty.t.is_ref() {
                // The active member gets its parts before they are set.
                let sh = self.shape(&fi.ty)?;
                self.write(&sub, sh, true)?;
            }
            let item = if k < items.len() {
                k += 1;
                Some(items[k - 1].clone()).filter(|x| !matches!(x.k, EK::MemberDefault))
            } else {
                None
            };
            let before = self.extended.len().max(ext);
            if let Some(it) = item {
                let save = self.in_aggregate;
                self.in_aggregate = if self.paren_aggregate { 0 } else { save + 1 };
                let r = self.init_field(&sub, &it, fi);
                self.in_aggregate = save;
                r?;
            } else if let Some(init) = &fi.init {
                let init = init.clone();
                // CTOR-17: an aggregate initialization that leaves a
                // reference member to its default member initializer runs
                // no constructor, and the temporary lives as long as the
                // aggregate.
                let save = self.in_aggregate;
                if fi.ty.t.is_ref() && !self.paren_aggregate {
                    self.in_aggregate = save + 1;
                }
                let r = self.with_this(place.clone(), |m| m.init_field(&sub, &init, fi));
                self.in_aggregate = save;
                r?;
            } else if fi.ty.t.is_ref() {
                return unsup("reference member with no initializer");
            } else {
                self.value_init(&sub, &fi.ty)?;
            }
            if fi.ty.t.is_ref() {
                if self.extended.len() > before {
                    done.extend(self.extended[before..].iter().cloned());
                }
            } else if !union {
                done.push(Cleanup { place: sub, ty: fi.ty.clone(), item: 0 });
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
        if !self.is_base_place(place) {
            // DER-7, DER-13: the most derived class initializes the
            // virtual bases first, and the other classes skip them.
            let vb = self.vbases_of(c);
            for (j, &v) in vb.iter().enumerate() {
                let bt = QTy::new(Ty::Class(v));
                let sub = place.sub(self.nslots(c) + j);
                match mem_inits.iter().find(|m| m.base == Some(v)) {
                    Some(MemInit { init: Some(init), .. }) => {
                        let init = init.clone();
                        self.full(|m| m.init_into(&sub, &init, &bt))?;
                    }
                    Some(_) => return unsup("base initializer the front end does not model"),
                    None => self.default_init(&sub, &bt)?,
                }
                done.push(Cleanup { place: sub, ty: bt, item: 0 });
            }
        }
        let bases: Vec<usize> = self.p.classes[c].bases.iter().filter_map(|b| b.class).collect();
        // OBJ-2, CTOR-12 (Q-0374): while the bases are initialized, the
        // lifetime of each member of class type has not begun, so a call
        // of its member function stops.
        let nb0 = bases.len();
        if self.p.classes[c].key != 2 {
            for (_, fi) in self.fields(c) {
                if matches!(fi.ty.t, Ty::Class(_)) {
                    self.no_calls.push((place.sub(nb0 + fi.index), "OBJ-2"));
                }
            }
        }
        let r = self.construct_parts_in(place, c, mem_inits, done, &bases);
        let pp = full_path(place);
        self.no_calls.retain(|(q, r)| !(*r == "OBJ-2" && q.obj == place.obj && full_path(q).starts_with(&pp)));
        r
    }

    fn construct_parts_in(&mut self, place: &Ptr, c: usize, mem_inits: &[MemInit], done: &mut Vec<Cleanup>, bases: &[usize]) -> R<()> {
        let bases: Vec<usize> = bases.to_vec();
        for (i, bc) in bases.iter().enumerate() {
            if self.p.classes[c].bases[i].virt {
                continue;
            }
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
        // CTOR-12: the bases are initialized, so a member function can
        // be called on the object.
        if let Some(k) = self.no_calls.iter().rposition(|(q, r)| *r == "CTOR-12" && q == place) {
            self.no_calls.remove(k);
        }
        let nb = bases.len();
        let union = self.p.classes[c].key == 2;
        for (fd, fi) in self.fields(c) {
            let sub = place.sub(nb + fi.index);
            if let Some(k) = self.no_calls.iter().rposition(|(q, r)| *r == "OBJ-2" && *q == sub) {
                self.no_calls.remove(k);
            }
            let mi = mem_inits.iter().find(|m| m.field == Some(fd)).and_then(|m| m.init.clone());
            // UNI-7: a mem-initializer for a member of an anonymous union
            // initializes the unnamed member that holds it.
            if let Ty::Class(u) = fi.ty.t {
                if mi.is_none() && self.p.anon_parent.get(&u).is_some_and(|x| x.1 == fd) {
                    let names: Vec<DeclId> = mem_inits.iter().filter_map(|m| m.field).filter(|&g| matches!(&self.p.decls[g].k, DK::Field(gi) if gi.class == u || self.p.anon_top(gi.class, u).is_some())).collect();
                    if !names.is_empty() {
                        let mut inner = Vec::new();
                        self.construct_parts(&sub, u, mem_inits, &mut inner)?;
                        continue;
                    }
                }
            }
            if let Some(init) = mi.or(fi.init.clone()) {
                if union {
                    // UNI-5: the member that is initialized is active.
                    self.activate_member(&sub)?;
                }
                // PRIM-11: `this` in a default member initializer is the
                // object being built.
                let here = place.clone();
                self.full(|m| m.with_this(here, |m| m.init_field(&sub, &init, &fi)))?;
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
                None => {
                    let n = self.no_calls.len();
                    self.no_calls.push((place.clone(), "CTOR-12"));
                    let r = self.construct_parts(place, c, &f.mem_inits, &mut done);
                    self.no_calls.truncate(n);
                    r
                }
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
                            let n = self.no_calls.len();
                            self.no_calls.push((place.clone(), "TRY-11"));
                            let r = self.handle_fn(x.clone(), &hs, &mut seek, true).map(|_| ());
                            self.no_calls.truncate(n);
                            r
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
        let fixed = std::mem::take(&mut self.fixed_order);
        let explore = !self.constant && effects >= 2 && !fixed;
        let mut all: Vec<(DeclId, Expr)> = Vec::new();
        for (i, a) in args.iter().enumerate() {
            if i >= f.params.len() {
                // An argument for the ellipsis: the program cannot read it
                // without the library, so only its evaluation counts.
                if !f.fty.variadic {
                    return unsup("call with variadic arguments");
                }
                if let Ty::Class(c) = nonref(&a.ty).t {
                    // A class argument with a trivial copy constructor and
                    // destructor: a copy that nothing reads.
                    let p = self.p;
                    if p.trivial(c, crate::special::Sm::CopyCtor) != Some(true) || p.trivial(c, crate::special::Sm::Dtor) != Some(true) {
                        return unsup("call with variadic arguments");
                    }
                    self.discard(a)?;
                    continue;
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
        if explore {
            // SEQ-1: the order of the arguments is unspecified. Each run
            // picks one order, one argument at a time, so that the runs
            // together try every order up to the bound. Option 0 takes the
            // first argument left, and option j the j-th from the end, so
            // that the second run of a call reverses the order.
            let mut rest = std::mem::take(&mut all);
            let (mut forward, mut backward) = (true, true);
            let picked = rest.len() > 1;
            while rest.len() > 1 {
                let j = self.decide(rest.len().min(255) as u8) as usize;
                let k = if j == 0 { 0 } else { rest.len() - j };
                forward &= k == 0;
                backward &= k == rest.len() - 1;
                all.push(rest.remove(k));
            }
            all.extend(rest);
            if picked {
                self.choice("SEQ-1", if forward { "arguments evaluated from left to right" } else if backward { "arguments evaluated from right to left" } else { "arguments evaluated in another order" });
            }
        }
        let over = std::mem::take(&mut self.ptr_param_tys);
        for (pid, a) in all {
            let pty = match &over {
                // LNK-2: the parameter types of a function of another
                // unit, from the type of the pointer.
                Some(v) => f.params.iter().position(|&q| q == pid).and_then(|i| v.get(i).cloned()).unwrap_or(QTy::unknown()),
                None => self.p.var(pid).map(|v| v.ty.clone()).unwrap_or(QTy::unknown()),
            };
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
                // DTOR-7: also with a trivial destructor, so that a second
                // destruction shows.
                if self.needs_dtor(&pty.t) || matches!(pty.t, Ty::Class(_)) {
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
        if !self.vbases_of(dc).is_empty() {
            return self.final_overrider_virtual(fid, this, dc, path);
        }
        loop {
            let sc = self.p.classes[c].scope;
            if let Some(ids) = self.p.scopes[sc].names.get(name) {
                for &id in ids {
                    if let Some(g) = self.p.func(id) {
                        if id == fid || (same_params(&g.fty, fty) && g.class == Some(c)) {
                            return Some((id, Ptr { obj: this.obj, path: path.clone(), off: 0, array: false, bytes: false }));
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

    /// VIRT-5 for a class with virtual bases: the function of the most
    /// derived class among those that declare one that overrides `fid`.
    fn final_overrider_virtual(&self, fid: DeclId, this: &Ptr, dc: usize, dpath: Vec<usize>) -> Option<(DeclId, Ptr)> {
        let name = &self.p.decls[fid].name;
        let f0 = self.p.func(fid)?;
        let (fty, fc) = (&f0.fty, f0.class?);
        // Every class of the object, each once.
        let mut all = vec![dc];
        let mut k = 0;
        while k < all.len() && all.len() < 256 {
            for b in &self.p.classes[all[k]].bases {
                if let Some(bc) = b.class {
                    if !all.contains(&bc) {
                        all.push(bc);
                    }
                }
            }
            k += 1;
        }
        let mut cands: Vec<(usize, DeclId)> = Vec::new();
        for &c in &all {
            if c != fc && self.base_path(c, fc).is_none() {
                continue;
            }
            let sc = self.p.classes[c].scope;
            if let Some(ids) = self.p.scopes[sc].names.get(name) {
                if let Some(&id) = ids.iter().find(|&&id| self.p.func(id).is_some_and(|g| id == fid || (same_params(&g.fty, fty) && g.class == Some(c)))) {
                    cands.push((c, id));
                }
            }
        }
        let (c, id) = *cands.iter().find(|(c, _)| !cands.iter().any(|(d, _)| d != c && self.base_path(*d, *c).is_some()))?;
        let _ = this;
        let mut path = dpath;
        path.extend(self.base_path(dc, c)?);
        Some((id, self.canon(Ptr { obj: this.obj, path, off: 0, array: false, bytes: false })))
    }

    /// Whether the destructor of class `c` is virtual: declared so, or the
    /// destructor of a base is (VIRT-7). An implicit destructor counts.
    fn class_dtor_virtual(&self, c: usize, depth: u32) -> bool {
        if depth > 32 {
            return false;
        }
        if self.p.classes[c].dtor.is_some_and(|d| self.p.func(d).is_some_and(|f| f.is_virtual)) {
            return true;
        }
        self.p.classes[c].bases.iter().filter_map(|b| b.class).any(|b| self.class_dtor_virtual(b, depth + 1))
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
        if let EK::Call(callee, args, None) = &e.k {
            let mut c = &**callee;
            while let EK::Conv(Conv::FuncDecay, x) = &c.k {
                c = x;
            }
            if let EK::PtrMem(obj, mp) = &c.k {
                // MPTR-5: a call through a pointer to member function. The
                // object comes first (SEQ-2), then the pointer.
                let place = if obj.cat == Cat::PR { self.materialize(obj)? } else { self.lvalue(obj)? };
                let (d, path) = match self.eval(mp)? {
                    Val::Member(d, path) => (d, path),
                    Val::Null => return ub("MPTR-6", "call through a null pointer to member function"),
                    _ => return unsup("pointer to member function value"),
                };
                let Some(fc) = self.p.func(d).and_then(|f| f.class) else { return unsup("pointer to member function value") };
                let st = nonref(&obj.ty).t;
                let pc = match &nonref(&mp.ty).t {
                    Ty::MemPtr(c, _) => *c,
                    _ => fc,
                };
                let p = self.member_object(place, &st, pc, fc, path)?;
                self.synth.push(p);
                let i = self.synth.len() - 1;
                let o = Expr::new(EK::Unknown(format!("\u{1}{}", i)), QTy::new(Ty::Class(fc)), Cat::L, obj.line);
                let call = Expr::new(EK::Call(Box::new(o), args.clone(), Some(d)), e.ty.clone(), e.cat, e.line);
                let r = self.call(&call, slot);
                self.synth.truncate(i);
                return r;
            }
        }
        let EK::Call(callee, args, fid) = &e.k else { unreachable!() };
        if let EK::Unknown(w) = &callee.k {
            if let Some(name) = w.strip_prefix("allocation function ") {
                return self.call_allocation(name, args);
            }
            if w == "builtin __builtin_memcpy" {
                if self.constant {
                    return limit("CE-2", "call of __builtin_memcpy");
                }
                let d = self.eval(&args[0])?;
                let s = self.eval(&args[1])?;
                let n = match self.eval(&args[2])? {
                    Val::Int(v) => v as i64,
                    _ => return unsup("byte count"),
                };
                let (Val::Ptr(dp), Val::Ptr(sp)) = (&d, &s) else { return unsup("__builtin_memcpy of these pointers") };
                let (dobj, doff) = self.byte_pos(dp)?;
                let (sobj, soff) = self.byte_pos(sp)?;
                for k in 0..n {
                    let b = self.read_byte(sobj, soff + k)?;
                    self.write_byte(dobj, doff + k, b)?;
                }
                return Ok(Ret::Val(d));
            }
        }
        // SEQ-2: the function, with its object, before the arguments.
        let from_ptr = fid.is_none();
        // LNK-2: a pointer to a function of another unit runs that unit's
        // function.
        let mut ptr_tu: Option<usize> = None;
        let mut fid = match fid {
            Some(f) => *f,
            None => {
                let v = match &callee.k {
                    EK::Unary(UOp::Deref, a) => self.eval(a)?,
                    _ => self.eval(callee)?,
                };
                match v {
                    Val::FnPtr(t, f) if t == self.tu => f,
                    Val::FnPtr(t, f) if self.ps[t].func(f).is_some_and(|g| g.class.is_none()) || self.ps[t].closure_conv.values().any(|&op| op == f) => {
                        // Only scalar types, whose meaning does not depend
                        // on the unit.
                        let g = self.ps[t].func(f).unwrap();
                        let scalar = |q: &QTy| !q.is_unknown() && !matches!(nonref(q).t, Ty::Class(_) | Ty::Enum(_) | Ty::MemPtr(..)) && !matches!(&nonref(q).t, Ty::Ptr(x) if matches!(x.t, Ty::Class(_) | Ty::Enum(_)));
                        if g.fty.variadic || !g.fty.params.iter().all(scalar) || !(g.fty.ret.t == Ty::Void || scalar(&g.fty.ret)) || g.params.len() != g.fty.params.len() {
                            return unsup("call through a pointer to a function of another translation unit");
                        }
                        self.ptr_param_tys = Some(g.fty.params.clone());
                        ptr_tu = Some(t);
                        f
                    }
                    Val::FnPtr(..) => return unsup("call through a pointer to a member function of another translation unit"),
                    Val::Null => return ub("PFX-8", "call through a null function pointer"),
                    _ => return unsup("indirect call"),
                }
            }
        };
        let fp = match ptr_tu {
            Some(t) => self.ps[t],
            None => self.p,
        };
        let Some(f) = fp.func(fid) else { return unsup("call of a non-function") };
        if from_ptr && !self.constant {
            // PFX-19: the pointer's function type must be the function's
            // type, `noexcept` included (Q-0178). A pointer without
            // `noexcept` can call a `noexcept` function (CONV-10).
            let called = match &nonref(&callee.ty).t {
                Ty::Ptr(q) => q.t.clone(),
                t => t.clone(),
            };
            if let Ty::Func(ft) = called {
                let same = |a: &QTy, b: &QTy| a.t == b.t;
                let fty = &f.fty;
                let differs = !same(&ft.ret, &fty.ret) || ft.params.len() != fty.params.len() || ft.params.iter().zip(fty.params.iter()).any(|(a, b)| !same(&a.unqual(), &b.unqual())) || ft.variadic != fty.variadic || (ft.noexcept && !fty.noexcept);
                if differs && f.class.is_none() && !ft.ret.is_unknown() && !fty.ret.is_unknown() {
                    return ub("PFX-19", "call through a pointer whose function type is not the type of the function");
                }
            }
        }
        // ICS-41: the object of a call of a function with an explicit object
        // parameter is the first argument.
        let with_obj: Vec<Expr>;
        // A call through a pointer to the function (CLS-17) passes the
        // object among the arguments.
        let args: &[Expr] = if f.explicit_obj && !matches!(callee.k, EK::Func(_)) && !matches!(nonref(&callee.ty).t, Ty::Ptr(_) | Ty::Func(_)) {
            with_obj = std::iter::once((**callee).clone()).chain(args.iter().cloned()).collect();
            &with_obj
        } else {
            args
        };
        let static_f = f.clone();
        let static_fid = fid;
        let mut f = static_f.clone();
        let name0 = fp.decls[fid].name.clone();
        let member = f.class.is_some() && !f.is_static && !f.explicit_obj && f.kind != FnKind::Ctor;
        // SEQ-6: an assignment operator function keeps the order of the
        // built-in assignment, the right operand first.
        let right_first = member && f.kind == FnKind::Operator && is_assignment_op(&name0) && args.len() == 1;
        // SEQ-6: shifts, subscripts, the comma, the pointer-to-member
        // operators, `&&`, and `||` keep the left-to-right order.
        let fixed = f.kind == FnKind::Operator && matches!(name0.as_str(), "operator<<" | "operator>>" | "operator[]" | "operator," | "operator->*" | "operator&&" | "operator||");
        // OPR-7: the flag is set just before the arguments, so that a call
        // in the object expression does not take it.
        let mut early_frame = None;
        if right_first {
            self.fixed_order = fixed;
            early_frame = Some(self.bind_args(&f, args, &f)?);
        }
        let mut this = if member && from_ptr && fp.closure_conv.values().any(|&op| op == fid) {
            // LAM-10: a call through the pointer of a lambda with no
            // captures runs its call operator on a closure with no state.
            let c = f.class.unwrap();
            let t = QTy::new(Ty::Class(c));
            let p = self.in_tu(ptr_tu.unwrap_or(self.tu), |m| {
                let p = m.alloc(&t)?;
                m.add_temp(&p, &t);
                Ok(p)
            })?;
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
            let elem = full_path(t).first().copied();
            if !self.constant && (t.bytes || self.type_at(t).is_some_and(|q| !matches!(q.t, Ty::Class(_) | Ty::Unknown | Ty::Dep(_)))) {
                // STG-22: the storage holds no object of the class.
                return ub("STG-22, STG-23", "member function called on storage that holds no object of its class");
            }
            if self.objs[t.obj].heap == Heap::Raw || elem.is_some_and(|i| self.unbuilt.contains(&(t.obj, i))) {
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
        if let Some(t2) = ptr_tu {
            ctu = t2;
        } else if let Some(&(t2, g)) = self.links.get(&(self.tu, fid)) {
            ctu = t2;
            fid = g;
            f = self.ps[t2].func(g).unwrap().clone();
        }
        let cp = self.ps[ctu];
        let name = cp.decls[fid].name.clone();
        // DCMP-1, DCMP-4: a defaulted comparison can be used in constant
        // evaluation.
        let dcmp = f.defaulted && matches!(name.as_str(), "operator==" | "operator!=" | "operator<=>" | "operator<" | "operator>" | "operator<=" | "operator>=");
        if self.constant && !f.constexpr && !f.consteval && !dcmp {
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
                return unsup(format!("defaulted function {} of another translation unit", name));
            }
            return self.call_defaulted(&f, fid, this, args);
        }
        let (fid, f) = match (&f.body, cp.c_fns.contains(&fid)) {
            (None, true) => {
                // LSP-4: a function with C linkage declared in a namespace
                // is the function of that name declared elsewhere.
                let other = cp.c_fns.iter().copied().filter(|&g| g != fid && cp.decls[g].name == cp.decls[fid].name).find(|&g| cp.func(g).is_some_and(|x| x.body.is_some()));
                match other {
                    Some(g) => (g, cp.func(g).unwrap().clone()),
                    None => (fid, f),
                }
            }
            (None, false) if f.class.is_none() && !matches!(cp.scopes[cp.decls[fid].scope].kind, SK::Namespace | SK::Class(_)) => {
                // DEFARG-9, NAM rules: a function declared in a block is
                // the function of the innermost enclosing namespace.
                let mut s = cp.scopes[cp.decls[fid].scope].parent;
                while let Some(x) = s {
                    if matches!(cp.scopes[x].kind, SK::Namespace) {
                        break;
                    }
                    s = cp.scopes[x].parent;
                }
                let other = s.and_then(|x| cp.scopes[x].names.get(&cp.decls[fid].name)).and_then(|v| v.iter().copied().find(|&g| cp.func(g).is_some_and(|x| x.body.is_some() && x.fty.params == f.fty.params)));
                match other {
                    Some(g) => (g, cp.func(g).unwrap().clone()),
                    None => (fid, f),
                }
            }
            _ => (fid, f),
        };
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
            if (name.starts_with("operatornew") || name.starts_with("operatordelete")) && f.class.is_none() && cp.decls[fid].scope == cp.global {
                // LSUP-6: a declared form of the library's allocation and
                // deallocation functions, such as the nothrow forms.
                return self.call_allocation(&name, args);
            }
            if cp.c_fns.contains(&fid) && f.class.is_none() && cp.decls[fid].scope == cp.global {
                if let Some(r) = self.c_library_call(&name, &f, args)? {
                    return Ok(r);
                }
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
                self.fixed_order = fixed;
                let fr = self.bind_args(&disp_f, args, &static_f)?;
                rebind(fr, &disp_f, &f)
            }
        };
        let consteval = f.consteval;
        let nothrow = f.fty.noexcept;
        let fn_try = f.fn_try.clone();
        if let Some(t) = &this {
            let tp = full_path(t);
            for (q, rule) in &self.no_calls {
                let qp = full_path(q);
                let hit = q.obj == t.obj && !q.bytes && !t.bytes && if *rule == "CTOR-12" { qp == tp } else { tp.starts_with(&qp) };
                if hit {
                    let msg = match *rule {
                        "CTOR-12" => "call of a member function before the bases of the object are initialized",
                        "OBJ-2" => "call of a member function of a member whose lifetime has not begun",
                        _ => "call of a member function of an object whose parts ended before the handler",
                    };
                    return ub(rule, msg);
                }
            }
        }
        let (r, ret) = self.in_tu(ctu, |m| {
            m.enter_call(Some(fid), this, ret_ref, ret_class, slot.clone())?;
            m.install_params(frame);
            let save = m.in_consteval_fn;
            m.in_consteval_fn = consteval;
            // UNW-7: the boundary of a non-throwing function.
            let n = m.catch_stack.len();
            if nothrow {
                m.catch_stack.push(CatchEntry::Boundary(m.depth));
            }
            let r = match fn_try {
                Some(hs) => m.run_fn_try(&body, &hs, false),
                None => m.run_body(&body),
            };
            m.catch_stack.truncate(n);
            // UNW-1: a return statement built the class result, and then
            // the destructor of a local object threw. The result ends.
            let r = match r {
                Err(e @ Stop::Throw(_)) if matches!(m.frame().ret, Some(Ret::InSlot)) && m.frame().slot.is_some() => {
                    let s = m.frame().slot.clone().unwrap();
                    m.frame_mut().ret = None;
                    let t = f.fty.ret.unqual();
                    m.unwinding += 1;
                    let d = m.destroy(&s, &t);
                    m.unwinding -= 1;
                    match d {
                        Ok(()) => Err(e),
                        Err(Stop::Throw(_)) => Err(Stop::Terminate("UNW-5", "a destructor throws during the unwinding of another exception".into())),
                        Err(o) => Err(o),
                    }
                }
                r => r,
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
        if self.p.has_user_copy(c) || !self.vbases_of(c).is_empty() {
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
        // (id, rvalue reference, const, volatile)
        let ctors: Vec<(DeclId, bool, bool, bool)> = self.p.classes[c].ctors.iter().filter_map(|&i| {
            let f = self.p.func(i)?;
            if f.params.is_empty() || f.params.len() - f.defaults.min(f.params.len()) > 1 || f.template || f.deleted || self.p.inherited_from.contains_key(&i) {
                // OVL-17: an inherited constructor is not a copy.
                return None;
            }
            match &self.p.var(f.params[0])?.ty.t {
                Ty::LRef(x) if x.t == Ty::Class(c) && (x.c || !konst) => Some((i, false, x.c, x.v)),
                Ty::RRef(x) if x.t == Ty::Class(c) && (x.c || !konst) => Some((i, true, x.c, x.v)),
                _ => None,
            }
        }).collect();
        // ICS-30: of two reference bindings, the one to the less
        // cv-qualified type wins.
        let best = |want_rv: bool| -> Option<DeclId> {
            let mut v: Vec<&(DeclId, bool, bool, bool)> = ctors.iter().filter(|x| x.1 == want_rv).collect();
            v.sort_by_key(|x| (x.3 as u8) * 2 + (x.2 as u8));
            v.first().map(|x| x.0)
        };
        if mv {
            if let Some(i) = best(true) {
                return Some(i);
            }
        }
        best(false)
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

    /// CE-22 (Q-0449): in constant evaluation, the trivial copy of a union
    /// whose active member is mutable reads that member, which is not
    /// valid when the source object began outside the evaluation.
    pub fn union_copy_mutable(&self, src: &Ptr, c: usize, v: &Val) -> R<()> {
        if !self.constant || self.p.classes[c].key != 2 || self.objs[src.obj].own {
            return Ok(());
        }
        let Val::Agg(items) = v else { return Ok(()) };
        for (_, fi) in self.fields(c) {
            if fi.mutable && items.get(fi.index).is_some_and(|x| !matches!(x, Val::Inactive)) {
                return limit("CE-22", "copy of a union whose active member is mutable, from an object that began outside the evaluation");
            }
        }
        Ok(())
    }

    /// The implicit or defaulted copy or move: each base and member in
    /// order (CPY-8).
    pub fn copy_parts(&mut self, place: &Ptr, src: &Ptr, c: usize, mv: bool) -> R<()> {
        if !self.copy_runs_code(c) {
            let v = self.get_at(src)?.clone();
            // CE-22: a trivial copy of a union copies its active member and
            // reads none of its parts.
            if self.constant && has_unset(&v) && self.p.classes[c].key != 2 {
                return limit("CE-22", "copy of an object that has a scalar part with no value");
            }
            self.union_copy_mutable(src, c, &v)?;
            return self.write(place, v, true);
        }
        self.class_ok(c)?;
        if !self.is_base_place(place) {
            // DER-7: the complete object copies its virtual bases first.
            let vb = self.vbases_of(c);
            for (j, &v) in vb.iter().enumerate() {
                let Some(bp) = self.base_path(c, v) else { return unsup("virtual base") };
                let mut sp = Ptr { obj: src.obj, path: full_path(src), off: 0, array: false, bytes: false };
                sp.path.extend(bp);
                let sp = self.canon(sp);
                self.copy_class(&place.sub(self.nslots(c) + j), &sp, v, mv)?;
            }
        }
        let bases: Vec<(usize, bool)> = self.p.classes[c].bases.iter().filter_map(|b| b.class.map(|x| (x, b.virt))).collect();
        for (i, (bc, virt)) in bases.iter().enumerate() {
            if !virt {
                self.copy_class(&place.sub(i), &src.sub(i), *bc, mv)?;
            }
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
            if self.constant && has_unset(&v) && self.p.classes[c].key != 2 {
                return limit("CE-22", "copy of an object that has a scalar part with no value");
            }
            self.union_copy_mutable(src, c, &v)?;
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
            // CPY-18: a virtual base can be assigned more than once.
            let (d, s) = (self.base_sub(dst, c, i), self.base_sub(src, c, i));
            self.assign_parts(&d, &s, *bc, mv)?;
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
