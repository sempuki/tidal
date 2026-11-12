// Calls, casts, initialization, and placeholder deduction.

use crate::ast::*;
use crate::decl::{DOp, DeclSpec, Declarator};
use crate::decl2::InitForm;
use crate::expr::CastKind;
use crate::parser::*;
use crate::types::*;

impl P {
    // ---------------- calls ----------------

    pub fn build_call(&mut self, callee: Expr, args: Vec<Expr>, line: u32) -> Expr {
        let adl = std::mem::take(&mut self.call_adl);
        if let EK::Unknown(w) = &callee.k {
            if let Some(name) = w.strip_prefix("allocation function ") {
                // STG-7, NEW-19: a call of a global allocation or
                // deallocation function by name, when the program replaces
                // none of the usual forms.
                // NEW-19: the functions that the program declares with this
                // name, next to the library's forms.
                let declared: Vec<DeclId> = self.scopes[self.global].names.get(name).cloned().unwrap_or_default().into_iter().filter(|&d| self.func(d).is_some_and(|f| !f.template)).collect();
                if !declared.is_empty() && !self.in_template() && !args.iter().any(|a| a.ty.is_unknown()) {
                    let mark = self.d.list.len();
                    let r = self.ics_select(None, &declared, &args);
                    self.d.list.truncate(mark);
                    if let Ok(Some((f, _))) = r {
                        let fi = self.func(f).unwrap().clone();
                        let mark = self.d.list.len();
                        let conv = self.convert_args(&fi.fty, args.clone(), line, Some(f));
                        if self.d.list.len() == mark && !conv.iter().any(|x| matches!(x.k, EK::Unknown(_))) {
                            if self.in_unevaluated == 0 {
                                self.use_fn(f);
                            }
                            let fe = Expr::new(EK::Func(f), QTy::new(Ty::Func(Box::new(fi.fty.clone()))), Cat::L, line);
                            return Expr::new(EK::Call(Box::new(fe), conv, Some(f)), fi.fty.ret.clone(), Cat::PR, line);
                        }
                        self.d.list.truncate(mark);
                    }
                }
                let lib_taken = |p: &P, n: &str| p.scopes[p.global].names.get(n).is_some_and(|v| v.iter().any(|&d| p.func(d).is_some_and(|f| f.fty.params.len() == 1)));
                // NEW-19, NEW-26: the sized and the aligned forms, by the
                // types of the arguments after the first.
                let is_align = |p: &P, e: &Expr| matches!(nonref(&e.ty).t, Ty::Enum(en) if p.enums[en].name == "align_val_t");
                if !self.in_template() && (2..=3).contains(&args.len()) && !args.iter().any(|a| a.ty.is_unknown()) {
                    let newf = name.starts_with("operatornew");
                    let vp = QTy::new(Ty::Ptr(Box::new(QTy::new(Ty::Void))));
                    let st = QTy::new(Ty::size_t());
                    let mark = self.d.list.len();
                    let mut out = Vec::new();
                    let mut ok = true;
                    let mut seen_align = false;
                    for (i, a) in args.iter().enumerate() {
                        let x = if i == 0 {
                            self.convert_init(a.clone(), if newf { &st } else { &vp }, InitForm::Copy, line)
                        } else if is_align(self, a) && !seen_align {
                            seen_align = true;
                            self.rvalue(a.clone())
                        } else if i == 1 && !newf && nonref(&a.ty).t.is_integral() {
                            self.convert_init(a.clone(), &st, InitForm::Copy, line)
                        } else {
                            ok = false;
                            break;
                        };
                        out.push(x);
                    }
                    if ok && self.d.list.len() == mark && !out.iter().any(|x| matches!(x.k, EK::Unknown(_))) {
                        let rt = if newf { vp } else { QTy::new(Ty::Void) };
                        return Expr::new(EK::Call(Box::new(callee), out, None), rt, Cat::PR, line);
                    }
                    self.d.list.truncate(mark);
                }
                if !lib_taken(self, name) && !self.in_template() && args.len() == 1 {
                    let vp = QTy::new(Ty::Ptr(Box::new(QTy::new(Ty::Void))));
                    let mark = self.d.list.len();
                    let (a, rt) = if name.starts_with("operatornew") {
                        (self.convert_init(args[0].clone(), &QTy::new(Ty::size_t()), InitForm::Copy, line), vp)
                    } else {
                        (self.convert_init(args[0].clone(), &vp, InitForm::Copy, line), QTy::new(Ty::Void))
                    };
                    if self.d.list.len() == mark && !matches!(a.k, EK::Unknown(_)) {
                        return Expr::new(EK::Call(Box::new(callee), vec![a], None), rt, Cat::PR, line);
                    }
                    self.d.list.truncate(mark);
                }
                return Expr::new(EK::Unknown(format!("call of allocation function {}", name)), QTy::unknown(), Cat::PR, line);
            }
        }
        match callee.k.clone() {
            EK::Func(f) => {
                self.call_adl = adl;
                self.call_overloads(None, vec![f], args, line)
            }
            EK::Overloads(fs) => {
                self.call_adl = adl;
                self.call_overloads(None, fs, args, line)
            }
            EK::BoundMember(obj, fs) => self.call_overloads(Some(*obj), fs, args, line),
            EK::TemplId(obj, fs, raw) => {
                self.call_explicit = Some(raw);
                self.call_adl = adl;
                self.call_overloads(obj.map(|o| *o), fs, args, line)
            }
            EK::Unary(UOp::AddrOf, ref x) if !self.in_template() && ((x.paren && matches!(x.k, EK::BoundMember(..))) || (x.paren && matches!(&x.k, EK::Overloads(fs) if fs.iter().any(|&f| self.func(f).is_some_and(|f| f.class.is_some() && !f.is_static && !f.explicit_obj))))) => {
                // OVL-44: the set holds a member with an implicit object
                // parameter, so the operand of `&` must be a qualified name
                // without parentheses.
                self.error(line, "OVL-44", "a call through '&' of an unqualified or parenthesized name of a set with a non-static member");
                Expr::unknown(line, "bad call")
            }
            EK::Unary(UOp::AddrOf, ref x) if matches!(x.k, EK::BoundMember(..)) && !self.in_template() => {
                // OVL-21, Q-0161: `(&f)(a)` with an unqualified name
                // resolves the set as a call by name, with no implied
                // object argument.
                let EK::BoundMember(_, fs) = x.k.clone() else { unreachable!() };
                self.call_no_object = true;
                self.call_overloads(None, fs, args, line)
            }
            EK::Unary(UOp::AddrOf, ref x) if matches!(x.k, EK::Overloads(_)) => {
                // OVL-21: `(&f)(a)` resolves `f` as a call by name, with no
                // implied object argument.
                let EK::Overloads(fs) = x.k.clone() else { unreachable!() };
                self.call_no_object = true;
                self.call_overloads(None, fs, args, line)
            }
            EK::Unknown(ref m) if !matches!(nonref(&callee.ty).t, Ty::Class(_)) => {
                let _ = self.args_rvalues(args);
                if m.starts_with("allocation function ") {
                    // UN-18 reads which allocation function is called.
                    return Expr::new(EK::Unknown(format!("call of {}", m)), QTy::unknown(), Cat::PR, line);
                }
                Expr::unknown(line, "call of unknown callee")
            }
            EK::Call(_, _, Some(fid)) if self.ref_lambda_fns.contains(&fid) => {
                Expr::unknown(line, "LAM-19: call of a closure whose reference captures name variables of a function that has returned")
            }
            _ => {
                let ct = nonref(&callee.ty);
                match &ct.t {
                    Ty::Class(c) => {
                        let c = *c;
                        // PFX-17: a class object needs an operator() or a
                        // conversion function.
                        if !self.in_template() && self.classes[c].complete && !self.class_uncertain(c) && !self.scopes[self.classes[c].scope].uncertain && self.lookup_member(c, "operator()").is_none() && !self.any_conv(c, 0) {
                            self.error(line, "PFX-17", format!("'{}' has no operator() and no conversion function", self.classes[c].name));
                            return Expr::unknown(line, "bad call");
                        }
                        // OVL-5, OVL-6: the call operators and the surrogates.
                        if !self.in_template() && !self.class_uncertain(c) && self.any_conv(c, 0) {
                            if let Some(x) = self.object_call(&callee, c, &args, line) {
                                return x;
                            }
                        }
                        // OPR-7: a call of a class object calls its operator().
                        if !self.in_template() && !self.class_uncertain(c) && !self.scopes[self.classes[c].scope].uncertain && !self.any_conv(c, 0) {
                            if let Some(ids) = self.lookup_member(c, "operator()") {
                                if ids.iter().all(|&i| self.func(i).is_some_and(|f| !f.template) || self.generic_lambdas.contains_key(&i) || self.is_func_templ(i)) {
                                    return self.call_overloads(Some(callee), ids, args, line);
                                }
                            }
                        }
                        Expr::unknown(line, "call operator")
                    }
                    Ty::Unknown | Ty::Dep(_) => Expr::unknown(line, "call of unknown callee"),
                    Ty::Func(_) | Ty::Ptr(_) => {
                        let c = self.rvalue(callee);
                        let Ty::Ptr(p) = &c.ty.t else {
                            return Expr::unknown(line, "call");
                        };
                        let Ty::Func(ft) = &p.t else {
                            self.error(line, "PFX-17", "call of a value that is not a function");
                            return Expr::unknown(line, "bad call");
                        };
                        let ft = (**ft).clone();
                        let args = self.convert_args(&ft, args, line, None);
                        let (t, cat) = ret_cat(&ft.ret);
                        Expr::new(EK::Call(Box::new(c), args, None), t, cat, line)
                    }
                    _ => {
                        self.error(line, "PFX-17", format!("called object of type {} is not a function", show(&ct)));
                        Expr::unknown(line, "bad call")
                    }
                }
            }
        }
    }

    /// Whether a class or one of its bases has a conversion function.
    pub fn any_conv(&self, c: usize, depth: u32) -> bool {
        let ci = &self.classes[c];
        ci.has_conv || depth > 16 || ci.bases.iter().any(|b| b.class.is_none_or(|bc| self.any_conv(bc, depth + 1)))
    }


    fn args_rvalues(&mut self, args: Vec<Expr>) -> Vec<Expr> {
        args
    }

    /// Overload resolution is outside the specification. This picks the single
    /// viable candidate by arity and implicit convertibility, or the
    /// candidate with the best conversion ranks; anything harder gives
    /// an unknown result.
    /// Argument-dependent lookup (NAM-27 to NAM-30): the functions named
    /// `name` in the associated namespaces of the arguments and the
    /// friends that the associated classes declare. The flag is false
    /// when an argument type is not modeled, so the result can be
    /// incomplete.
    pub fn adl_lookup(&self, name: &str, args: &[Expr]) -> (Vec<DeclId>, bool) {
        let mut ns: Vec<usize> = Vec::new();
        let mut cls: Vec<usize> = Vec::new();
        let mut certain = true;
        for a in args {
            match &a.k {
                // NAM-28: a braced list adds nothing.
                EK::InitList(_) if a.ty.is_unknown() => continue,
                // NAM-27: a closure type adds the namespace and the class
                // around the lambda, which is where the call is.
                EK::Unknown(w) if w.starts_with("lambda") => {
                    if let Some(&s) = self.stack.iter().rev().find(|&&s| self.scopes[s].kind == SK::Namespace) {
                        self.add_ns(s, &mut ns);
                    }
                    if let Some(c) = self.stack.iter().rev().find_map(|&s| match self.scopes[s].kind {
                        SK::Class(c) => Some(c),
                        _ => None,
                    }) {
                        if !cls.contains(&c) {
                            cls.push(c);
                        }
                    }
                    continue;
                }
                EK::Overloads(ids) => {
                    // NAM-27: each function of an overload set.
                    for &f in ids {
                        if let Some(fi) = self.func(f) {
                            self.assoc_ty(&Ty::Func(Box::new(fi.fty.clone())), &mut ns, &mut cls, &mut certain, 0);
                        } else {
                            certain = false;
                        }
                    }
                    continue;
                }
                _ => {}
            }
            if let EK::Unary(UOp::AddrOf, inner) = &a.k {
                if let EK::Overloads(ids) = &inner.k {
                    for &f in ids {
                        if let Some(fi) = self.func(f) {
                            if let Some(c) = fi.class.filter(|_| !fi.is_static) {
                                self.assoc_ty(&Ty::MemPtr(c, Box::new(QTy::new(Ty::Func(Box::new(fi.fty.clone()))))), &mut ns, &mut cls, &mut certain, 0);
                            } else {
                                self.assoc_ty(&Ty::Func(Box::new(fi.fty.clone())), &mut ns, &mut cls, &mut certain, 0);
                            }
                        }
                    }
                    continue;
                }
            }
            self.assoc_ty(&a.ty.t, &mut ns, &mut cls, &mut certain, 0);
        }
        let mut out: Vec<DeclId> = Vec::new();
        let is_fn = |p: &P, d: DeclId| match &p.decls[d].k {
            DK::Func(f) => f.class.is_none(),
            DK::Template(TemplKind::Func) => true,
            _ => false,
        };
        for &n in &ns {
            if self.scopes[n].uncertain {
                certain = false;
            }
            // NAM-29: the namespace and its inline namespaces.
            let mut found = Vec::new();
            self.ns_and_inline_names(n, name, &mut found, 0);
            for d in found {
                if is_fn(self, d) && !out.contains(&d) {
                    out.push(d);
                }
            }
        }
        for &c in &cls {
            // A friend class declaration adds no function that the
            // search can find; a friend function the front end did not
            // record leaves the search uncertain.
            if self.class_uncertain(c) || self.friend_fn_classes.contains(&c) && !self.class_friends.contains_key(&c) {
                certain = false;
            }
            if let Some(fr) = self.class_friends.get(&c) {
                for &d in fr {
                    if self.decls[d].name == name && is_fn(self, d) && !out.contains(&d) {
                        out.push(d);
                    }
                }
            }
        }
        (out, certain)
    }

    fn ns_and_inline_names(&self, s: usize, name: &str, out: &mut Vec<DeclId>, depth: u32) {
        if let Some(v) = self.scopes[s].names.get(name) {
            for &d in v {
                if !self.hidden_friends.contains_key(&d) && !out.contains(&d) {
                    out.push(d);
                }
            }
        }
        if depth > 16 {
            return;
        }
        for &n in &self.scopes[s].using_ns {
            if self.inline_ns.contains(&n) && self.scopes[n].parent == Some(s) {
                self.ns_and_inline_names(n, name, out, depth + 1);
            }
        }
    }

    /// Add a namespace, and the enclosing namespace of an inline one
    /// (NAM-29).
    fn add_ns(&self, mut s: usize, ns: &mut Vec<usize>) {
        loop {
            if !ns.contains(&s) {
                ns.push(s);
            }
            if !self.inline_ns.contains(&s) {
                return;
            }
            match self.scopes[s].parent {
                Some(p) => s = p,
                None => return,
            }
        }
    }

    /// The innermost namespace around a scope.
    fn ns_of(&self, mut s: usize) -> usize {
        loop {
            if self.scopes[s].kind == SK::Namespace {
                return s;
            }
            match self.scopes[s].parent {
                Some(p) => s = p,
                None => return self.global,
            }
        }
    }

    /// NAM-27: what one type adds to the associated namespaces and
    /// classes.
    fn assoc_ty(&self, t: &Ty, ns: &mut Vec<usize>, cls: &mut Vec<usize>, certain: &mut bool, depth: u32) {
        if depth > 24 {
            *certain = false;
            return;
        }
        match t {
            Ty::Unknown | Ty::Dep(_) => *certain = false,
            Ty::Ptr(q) | Ty::LRef(q) | Ty::RRef(q) | Ty::Array(q, _) => self.assoc_ty(&q.t, ns, cls, certain, depth + 1),
            Ty::Func(f) => {
                for p in &f.params {
                    self.assoc_ty(&p.t, ns, cls, certain, depth + 1);
                }
                self.assoc_ty(&f.ret.t, ns, cls, certain, depth + 1);
            }
            Ty::MemPtr(c, q) => {
                self.assoc_ty(&Ty::Class(*c), ns, cls, certain, depth + 1);
                self.assoc_ty(&q.t, ns, cls, certain, depth + 1);
            }
            Ty::Class(c) => self.assoc_class(*c, ns, cls, certain, true, depth),
            Ty::Enum(e) => {
                let s = self.enums[*e].scope;
                if let Some(p) = self.scopes[s].parent {
                    if let SK::Class(o) = self.scopes[p].kind {
                        if !cls.contains(&o) {
                            cls.push(o);
                        }
                    }
                    self.add_ns(self.ns_of(p), ns);
                }
            }
            _ => {}
        }
    }

    fn assoc_class(&self, c: usize, ns: &mut Vec<usize>, cls: &mut Vec<usize>, certain: &mut bool, outer: bool, depth: u32) {
        if depth > 24 {
            *certain = false;
            return;
        }
        // A class seen before adds nothing new, as when a class is its own
        // base's template argument (`struct R : Base<R>`).
        if !outer && cls.contains(&c) {
            return;
        }
        let ci = &self.classes[c];
        if ci.template || ci.opaque {
            *certain = false;
        }
        if !cls.contains(&c) {
            cls.push(c);
        }
        if let Some(p) = self.scopes[ci.scope].parent {
            // the class that the class is a member of
            if outer {
                if let SK::Class(o) = self.scopes[p].kind {
                    if !cls.contains(&o) {
                        cls.push(o);
                    }
                }
            }
            self.add_ns(self.ns_of(p), ns);
        }
        for b in &ci.bases {
            match b.class {
                Some(bc) => self.assoc_class(bc, ns, cls, certain, false, depth + 1),
                None => *certain = false,
            }
        }
        // NAM-27: a specialization adds the namespace of the template,
        // what each type argument adds, and the namespace of each
        // template argument that is a template.
        if let Some((t, args)) = self.inst_of.get(&c) {
            let s = self.templs[*t].scope;
            self.add_ns(self.ns_of(s), ns);
            for a in args.clone() {
                self.assoc_targ(&a, ns, cls, certain, depth);
            }
        }
    }

    fn assoc_targ(&self, a: &TArg, ns: &mut Vec<usize>, cls: &mut Vec<usize>, certain: &mut bool, depth: u32) {
        match a {
            TArg::Type(q) => self.assoc_ty(&q.t, ns, cls, certain, depth + 1),
            TArg::Templ(TName::Templ(t)) => {
                let s = self.templs[*t].scope;
                self.add_ns(self.ns_of(s), ns);
                // NAM-27: a member template adds the class it is a
                // member of.
                if let SK::Class(o) = self.scopes[s].kind {
                    if !cls.contains(&o) {
                        cls.push(o);
                    }
                }
            }
            TArg::Pack(v) => {
                for x in v {
                    self.assoc_targ(x, ns, cls, certain, depth);
                }
            }
            _ => {}
        }
    }

    /// OVL-20: an overload set, or a function template, used where no
    /// target type chooses a function. True after the error.
    pub fn overload_set_no_target(&mut self, e: &Expr, line: u32) -> bool {
        if self.in_template() {
            return false;
        }
        let set = Self::overload_set_of(e).or_else(|| match &e.k {
            // A function template name alone is an overload set too.
            EK::Func(f) if self.is_func_templ(*f) => Some((vec![*f], false)),
            _ => None,
        });
        let Some((fs, _)) = set else { return false };
        let plain = fs.iter().all(|&f| !self.fn_cons.contains_key(&f) && self.func(f).is_some_and(|g| !g.constrained && g.class.is_none_or(|c| !self.class_uncertain(c))));
        if plain && (fs.len() > 1 || fs.iter().any(|&f| self.is_func_templ(f))) {
            self.error(line, "OVL-20", format!("the overload set '{}' is used where no target type chooses a function", self.decls[fs[0]].name));
            return true;
        }
        false
    }

    pub fn call_overloads_pub(&mut self, obj: Option<Expr>, cands: Vec<DeclId>, args: Vec<Expr>, line: u32) -> Expr {
        self.call_overloads(obj, cands, args, line)
    }

    pub fn call_overloads(&mut self, obj: Option<Expr>, cands: Vec<DeclId>, args: Vec<Expr>, line: u32) -> Expr {
        let no_object = std::mem::take(&mut self.call_no_object);
        let mut cands = cands;
        let adl = std::mem::take(&mut self.call_adl);
        let independent = std::mem::take(&mut self.call_adl_independent);
        if obj.is_none() && adl {
            if let Some(&c0) = cands.first() {
                let name = self.decls[c0].name.clone();
                for d in self.adl_lookup(&name, &args).0 {
                    // DEP-5: an independent call sees the declarations
                    // before the template's definition only.
                    if independent && d >= self.lookup_limit {
                        continue;
                    }
                    if !cands.contains(&d) {
                        cands.push(d);
                    }
                }
            }
        }
        // OVL-38 (Q-0140): one function with C language linkage declared
        // in two scopes with different default arguments, and a call that
        // uses the default.
        if !self.in_template() {
            let cs: Vec<DeclId> = cands.iter().copied().filter(|c| self.c_fns.contains(c)).collect();
            'pairs: for (i, &a) in cs.iter().enumerate() {
                for &b in &cs[i + 1..] {
                    let (Some(fa), Some(fb)) = (self.func(a).cloned(), self.func(b).cloned()) else { continue };
                    if fa.fty.params != fb.fty.params || args.len() >= fa.params.len() || fa.params.len() != fb.params.len() {
                        continue;
                    }
                    let k = args.len();
                    let da = self.var(fa.params[k]).and_then(|v| v.init.clone());
                    let db = self.var(fb.params[k]).and_then(|v| v.init.clone());
                    if let (Some(da), Some(db)) = (da, db) {
                        // OVL-38 (G140): the same expression, where two
                        // literals with one value are the same and `5` and
                        // `2 + 3` are not. The form compares the trees,
                        // which keep literals by value and drop parentheses.
                        let differ = crate::ast::sexp(&da) != crate::ast::sexp(&db);
                        if differ {
                            self.error(line, "OVL-38", format!("the call of '{}' uses a default argument that two declarations in different scopes give differently", self.decls[a].name));
                            break 'pairs;
                        }
                    }
                }
            }
        }
        // DED-1: each function template adds the specialization that
        // deduction gives.
        let explicit = self.call_explicit.take();
        if !self.in_template() && (explicit.is_some() || cands.iter().any(|&c| self.func(c).is_some_and(|f| f.template))) {
            let name = cands.first().map(|&c| self.decls[c].name.clone()).unwrap_or_default();
            let obj_x = obj.as_ref().map(|o| match (&o.k, &o.ty.t) {
                (EK::This, Ty::Ptr(p)) => Expr::new(EK::Unary(UOp::Deref, Box::new(o.clone())), (**p).clone(), Cat::L, line),
                _ => o.clone(),
            });
            match self.templ_candidates(obj_x.as_ref(), &cands, explicit.as_ref(), &args, line) {
                Ok(v) => cands = v,
                Err(true) => return Expr::unknown(line, "call needing template deduction or unknown arguments"),
                Err(false) => {
                    if args.iter().any(|a| a.ty.is_unknown() && !matches!(a.k, EK::InitList(_) | EK::Overloads(_) | EK::Func(_))) {
                        return Expr::unknown(line, "call needing template deduction or unknown arguments");
                    }
                    // DED-35: no function after deduction.
                    self.error(line, "DED-35", format!("no viable function for the call of '{}': deduction fails for each template", name));
                    return Expr::unknown(line, "bad call");
                }
            }
        }
        let any_template = cands.iter().any(|&c| self.func(c).is_some_and(|f| f.template));
        let obj_const = obj.as_ref().is_some_and(|o| {
            let t = match &o.k {
                EK::This => match &o.ty.t {
                    Ty::Ptr(p) => p.c,
                    _ => false,
                },
                _ => nonref(&o.ty).c,
            };
            t
        });
        let obj = obj.map(|o| {
            if let EK::This = o.k {
                let t = match &o.ty.t {
                    Ty::Ptr(p) => (**p).clone(),
                    _ => QTy::unknown(),
                };
                Expr::new(EK::Unary(UOp::Deref, Box::new(o)), t, Cat::L, line)
            } else {
                o
            }
        });
        // ICS-P5: choose by the conversion sequences when the front end can
        // form all of them.
        let mut picked: Option<(DeclId, Vec<crate::ics::Ics>)> = None;
        self.ics_no_viable = false;
        if !any_template && !self.in_template() {
            match self.ics_select(obj.as_ref(), &cands, &args) {
                Ok(Some(x)) => picked = Some(x),
                Ok(None) => {
                    // OVL-36: no viable function is better than each of the
                    // others.
                    let name = cands.first().map(|&c| self.decls[c].name.clone()).unwrap_or_default();
                    self.error(line, "OVL-36", format!("ambiguous call of '{}': no viable function is better than each of the others", name));
                    return Expr::unknown(line, "bad call");
                }
                Err(()) => {}
            }
        }
        let fid = if let Some((f, _)) = &picked {
            self.selected(*f);
            *f
        } else {
        let mut viable: Vec<(DeclId, Vec<u8>)> = Vec::new();
        let mut arity_ok_any = false;
        let mut const_blocked = false;
        for &c in &cands {
            let Some(f) = self.func(c).cloned() else { continue };
            if f.template {
                continue;
            }
            let np = f.fty.params.len();
            let min = np - f.defaults.min(np);
            if args.len() < min || (args.len() > np && !f.fty.variadic) {
                continue;
            }
            arity_ok_any = true;
            if obj.is_some() && f.class.is_some() && !f.is_static && obj_const && !f.fty.cv_c && f.kind != FnKind::Ctor {
                const_blocked = true;
                continue;
            }
            let mut ranks = Vec::new();
            let mut ok = true;
            for (i, a) in args.iter().enumerate() {
                if i >= np {
                    ranks.push(4);
                    continue;
                }
                match self.conv_rank(a, &f.fty.params[i]) {
                    Some(r) => ranks.push(r),
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                viable.push((c, ranks));
            }
        }
        if viable.is_empty() {
            if any_template || self.in_template() || args.iter().any(|a| a.ty.is_unknown() || matches!(a.k, EK::InitList(_)) || matches!(&a.k, EK::Unknown(w) if w != "class temporary")) {
                return Expr::unknown(line, "call needing template deduction or unknown arguments");
            }
            if const_blocked {
                // CLS-5: no call of a non-const member function on a const object.
                self.error(line, "CLS-25", "non-const member function called on a const object");
                return Expr::unknown(line, "bad call");
            }
            if !arity_ok_any {
                // PFX-5: one argument for each parameter without a default.
                self.error(line, "PFX-5", "wrong number of arguments in a function call");
            } else {
                self.error(line, "OVL-35", "no viable function: an argument cannot be converted to the parameter type");
            }
            return Expr::unknown(line, "bad call");
        }
        if self.ics_no_viable && !cands.is_empty() && cands.iter().all(|&c| self.fn_sat_cache.get(&c) == Some(&crate::cons::Sat::No)) {
            // CNS-12, OVL-24: the constraints of each candidate are not
            // satisfied.
            self.ics_no_viable = false;
            self.error(line, "CNS-12", format!("no viable function for the call of '{}': its constraints are not satisfied", self.decls[cands[0]].name));
            return Expr::unknown(line, "bad call");
        }
        if std::mem::take(&mut self.ics_no_viable) {
            // ICS-1 to ICS-20: no candidate has a sequence for every
            // argument.
            self.error(line, "OVL-35", "no viable function: an argument has no implicit conversion sequence to its parameter");
            return Expr::unknown(line, "bad call");
        }
        if any_template && !viable.iter().any(|v| self.strict_exact(v.0, &args)) {
            // A template specialization can have better sequences than
            // every plain function; OVL-27 decides only a tie.
            return Expr::unknown(line, "call that may select a template");
        }
        // best viable: no worse in every argument, better in one
        let mut best: Option<usize> = None;
        for i in 0..viable.len() {
            let better_than_all = (0..viable.len()).all(|j| {
                if i == j {
                    return true;
                }
                let a = &viable[i].1;
                let b = &viable[j].1;
                let no_worse = a.iter().zip(b.iter()).all(|(x, y)| x <= y);
                let better = a.iter().zip(b.iter()).any(|(x, y)| x < y);
                no_worse && (better || a == b && false)
            });
            if better_than_all {
                best = Some(i);
                break;
            }
        }
        let Some(bi) = best else {
            if viable.iter().any(|v| v.1.contains(&3)) {
                return Expr::unknown(line, "ambiguous call (needs full overload resolution)");
            }
            if viable.len() > 1 && viable.iter().all(|v| v.1 == viable[0].1) {
                // identical ranks: needs tie-breakers on cv/ref qualification
                return Expr::unknown(line, "ambiguous call");
            }
            return Expr::unknown(line, "ambiguous call");
        };
        viable[bi].0
        };
        if let Some((_, seqs)) = &picked {
            self.check_chosen_seqs(seqs, fid, line);
            if obj.is_none() && self.func(fid).is_some_and(|f| f.class.is_some() && !f.is_static && !f.explicit_obj && f.kind != FnKind::Ctor) && (no_object || !self.has_object_for(fid)) {
                // OVL-3: the call has no object and chooses a member with an
                // implicit object parameter.
                self.error(line, "OVL-3", format!("call of non-static member function '{}' without an object", self.decls[fid].name));
                return Expr::unknown(line, "bad call");
            }
        }
        if let Some(errs) = self.noexcept_errors.remove(&fid) {
            // DED-27, DED-29: the exception specification of the chosen
            // specialization.
            self.d.list.extend(errs);
        }
        if !self.in_template() {
            // INST-9: a call that uses a default argument makes it.
            let params = self.func(fid).map(|f| f.params.clone()).unwrap_or_default();
            for &p in params.iter().skip(args.len()) {
                if let Some(errs) = self.default_errors.remove(&p) {
                    self.d.list.extend(errs);
                }
            }
            // A use that needs the definition of a specialization makes
            // it; a deduced return type needs it also in an unevaluated
            // operand (INST-7, DED-29).
            let needs_ret = self.func(fid).is_some_and(|f| f.fty.ret.is_unknown());
            // INST-7 (Q-0252): a constexpr function named in a braced
            // initializer of an unevaluated operand is instantiated.
            let braced_constexpr = self.braced_depth > 0 && self.func(fid).is_some_and(|f| f.constexpr);
            if self.in_unevaluated == 0 || needs_ret || braced_constexpr {
                self.use_fn(fid);
            }
        }
        let f = self.func(fid).unwrap().clone();
        if let (Some(o), Some(owner)) = (&obj, f.class) {
            // OVL-37, ACC-3: the chosen member must be accessible. Only a
            // member of the object's own class is checked, since a
            // using-declaration can change the access of a base member.
            if !self.in_template() && nonref(&o.ty).t == Ty::Class(owner) && !self.accessible(fid, false) {
                self.error(line, "ACC-3", format!("'{}' is not accessible here", self.decls[fid].name));
            }
        } else if obj.is_none() && f.is_static && f.class.is_some() && !self.in_template() && !self.accessible(fid, false) {
            // ACC-3: a static member function that overload resolution
            // selects must be accessible where the call is, also in a
            // specialization of a template.
            self.error(line, "ACC-3", format!("'{}' is not accessible here", self.decls[fid].name));
        }
        if f.deleted {
            // DEL-1: a program must not refer to a deleted function.
            self.error(line, "DEL-1", format!("call of deleted function '{}'", self.decls[fid].name));
        }
        if f.is_main {
            // TR-3: a program must not call main.
            self.error(line, "TR-3", "call of 'main'");
        }
        if self.in_unevaluated == 0 && !self.in_template() && self.internal_fns.contains(&fid) {
            self.internal_uses.push((fid, line));
        }
        if self.in_unevaluated == 0 && !self.in_template() {
            let (ret, params) = (f.fty.ret.clone(), f.fty.params.clone());
            self.require_ty(&ret, line);
            for p in params {
                self.require_ty(&p, line);
            }
        }
        let f = self.func(fid).unwrap().clone();
        if self.in_unevaluated == 0 && !self.in_template() {
            // PFX-14: no call with an incomplete class as the return type or
            // as a parameter type.
            let incomplete = |p: &P, t: &Ty| matches!(t, Ty::Class(c) if !p.classes[*c].complete && !p.classes[*c].template);
            if incomplete(self, &f.fty.ret.t) || f.fty.params.iter().any(|q| incomplete(self, &q.t)) {
                self.error(line, "PFX-14", format!("call of '{}', whose return or parameter type is an incomplete class", self.decls[fid].name));
            }
        }
        let args = self.convert_args(&f.fty, args, line, Some(fid));
        let args_n = args.len();
        let (t, cat) = ret_cat(&f.fty.ret);
        let callee = match obj {
            Some(o) if f.class.is_some() && !f.is_static => o,
            _ => Expr::new(EK::Func(fid), QTy::new(Ty::Func(Box::new(f.fty.clone()))), Cat::L, line),
        };
        if cat == Cat::PR && !self.in_template() && self.is_abstract(&t.t) {
            // VIRT-8: a call that returns an abstract class.
            self.error(line, "VIRT-8", "call of a function that returns an abstract class");
        }
        if !self.in_template() && f.fty.params.iter().any(|q| self.is_abstract(&q.t)) {
            // VIRT-8: the argument would make an object of an abstract
            // class.
            self.error(line, "VIRT-8", "call of a function with a parameter of an abstract class");
        }
        let call = Expr::new(EK::Call(Box::new(callee), args, Some(fid)), t, cat, line);
        let in_lambda = self.fns.iter().any(|f| f.is_lambda);
        if f.consteval && !self.in_immediate_context() && self.in_unevaluated == 0 && !self.in_template() && !in_lambda && self.in_default_arg == 0 {
            // (A lambda that calls a consteval function with non-constant
            // arguments becomes consteval itself, CEV-1.)
            // CEV-1: each call of a consteval function is a constant expression.
            // CEV-1: a constexpr function made from a template takes on
            // `consteval` from such a call.
            let escalating = self.fns.last().and_then(|c| c.func).filter(|&g| (self.spec_of.contains_key(&g) || self.inst_depth > 0) && self.func(g).is_some_and(|x| x.constexpr));
            match self.const_check(&call) {
                Ok(()) => {}
                Err(crate::consteval::CeErr::NotConst(..)) if escalating.is_some() => {
                    if let Some(fm) = self.func_mut(escalating.unwrap()) {
                        fm.consteval = true;
                    }
                }
                Err(crate::consteval::CeErr::NotConst(r, m)) => self.error_because(line, "CEV-1", r, format!("call of consteval function is not a constant expression ({}: {})", r, m)),
                Err(crate::consteval::CeErr::Unsupported(m)) => self.unsupported(line, format!("consteval call: {}", m)),
            }
        }
        if !self.in_immediate_context() && self.in_unevaluated == 0 && !self.in_template() && !in_lambda && self.in_default_arg == 0 && args_n < f.params.len() {
            // DEFARG-10: each call that uses a default argument checks the
            // calls of immediate functions in it.
            let mut imm = Vec::new();
            for &p in &f.params[args_n..] {
                if let Some(init) = self.var(p).and_then(|v| v.init.clone()) {
                    crate::ast::walk_expr(&init, &mut |x| {
                        if let EK::Call(_, _, Some(g)) = &x.k {
                            imm.push((*g, x.clone()));
                        }
                    });
                }
            }
            for (g, x) in imm {
                if !self.func(g).is_some_and(|gi| gi.consteval) {
                    continue;
                }
                match self.const_check(&x) {
                    Err(crate::consteval::CeErr::NotConst(r, m)) => {
                        self.error_because(line, "DEFARG-10", r, format!("the default argument's call of consteval function '{}' is not a constant expression ({}: {})", self.decls[g].name, r, m));
                    }
                    Err(crate::consteval::CeErr::Unsupported(m)) => self.unsupported(line, format!("consteval call in a default argument: {}", m)),
                    Ok(()) => {}
                }
            }
        }
        call
    }

    /// Each argument matches its parameter of `fid` with no conversion at
    /// all: the same type, and for a reference the same cv-qualifiers and
    /// the kind of reference that the value category binds. No template
    /// specialization can then have a better sequence.
    fn strict_exact(&self, fid: DeclId, args: &[Expr]) -> bool {
        let Some(f) = self.func(fid) else { return false };
        if args.len() > f.fty.params.len() {
            return false;
        }
        args.iter().zip(f.fty.params.iter()).all(|(a, p)| {
            let at = nonref(&a.ty);
            match &p.t {
                Ty::LRef(i) => a.cat == Cat::L && i.t == at.t && i.c == at.c && i.v == at.v,
                Ty::RRef(i) => a.cat != Cat::L && i.t == at.t && i.c == at.c && i.v == at.v,
                t => *t == at.t && !matches!(t, Ty::Unknown | Ty::Dep(_)),
            }
        })
    }

    /// OVL-37, ICS-8: the faults in the sequences of the chosen function.
    pub fn check_chosen_seqs(&mut self, seqs: &[crate::ics::Ics], fid: DeclId, line: u32) {
        use crate::ics::{IK, UKey};
        for q in seqs {
            match (&q.kind, &q.user) {
                (IK::Ambig, Some(_)) => {
                    // ICS-19: a braced list that no constructor initializes
                    // unambiguously.
                    self.error_because(line, "ICS-8", "CTOR-25", format!("the call of '{}' needs a braced list that no constructor initializes unambiguously", self.decls[fid].name));
                }
                (IK::Ambig, None) => {
                    // ICS-7: two or more conversion functions or constructors.
                    self.error_because(line, "ICS-8", "CVF-12", format!("the call of '{}' needs an ambiguous conversion", self.decls[fid].name));
                }
                (IK::User, Some(UKey::Fn(u))) if self.func(*u).is_some_and(|f| f.deleted) => {
                    self.error_because(line, "ICS-8", "DEL-1", format!("the call of '{}' needs the deleted function '{}'", self.decls[fid].name, self.decls[*u].name));
                }
                _ => {}
            }
        }
    }

    /// OVL-2: `this` can be used here and its class is the class of
    /// member `fid` or derived from it. A default member initializer
    /// counts. Otherwise a call by name has no object (OVL-3).
    fn has_object_for(&self, fid: DeclId) -> bool {
        let Some(fc) = self.func(fid).and_then(|f| f.class) else { return true };
        let related = |c: usize| c == fc || self.is_base_of(fc, c) || self.class_uncertain(c);
        match self.fns.iter().rev().find(|f| !f.is_lambda || f.class.is_some()) {
            Some(f) => {
                let explicit_obj = f.func.and_then(|id| self.func(id)).is_some_and(|fi| fi.explicit_obj);
                f.class.is_some_and(|c| !f.is_static && !explicit_obj && related(c))
            }
            None => self.enclosing_class().is_some_and(related),
        }
    }

    /// Conversion rank of an argument to a parameter: 0 exact, 1
    /// promotion, 2 conversion, 3 user-defined or unknown. None: not
    /// convertible.
    fn conv_rank(&mut self, a: &Expr, p: &QTy) -> Option<u8> {
        if a.ty.is_unknown() || p.is_unknown() || is_raw_list(a) || matches!(a.k, EK::Overloads(_)) {
            return Some(3);
        }
        let at = nonref(&a.ty);
        if let Ty::LRef(inner) | Ty::RRef(inner) = &p.t {
            let is_l = matches!(p.t, Ty::LRef(_));
            if at.t == inner.t {
                if is_l && !inner.c && a.cat != Cat::L {
                    return None;
                }
                if !is_l && a.cat == Cat::L {
                    return None;
                }
                if at.c && !inner.c {
                    return None;
                }
                return Some(0);
            }
            if let (Ty::Class(x), Ty::Class(y)) = (&at.t, &inner.t) {
                if self.is_base_of(*y, *x) {
                    return Some(2);
                }
            }
            if is_l && !inner.c {
                if matches!(at.t, Ty::Class(_)) {
                    return Some(3);
                }
                return None;
            }
            return self.conv_rank(a, &inner.unqual()).map(|r| r.max(2));
        }
        if let Ty::Class(_) = p.t {
            if at.t == p.t {
                return Some(0);
            }
            return Some(3);
        }
        if let Ty::Class(c) = at.t {
            return if self.class_converts(c) == Some(false) { None } else { Some(3) };
        }
        let nerr = self.d.list.len();
        self.d.muted += 1;
        let r = self.implicit(a.clone(), &p.unqual(), false);
        self.d.muted -= 1;
        self.d.list.truncate(nerr);
        match r {
            Err(_) => None,
            Ok(_) => {
                let mut from = at.t.clone();
                if let Ty::Array(e, _) = &from {
                    from = Ty::Ptr(e.clone());
                }
                if from == p.t {
                    Some(0)
                } else if let (Ty::Ptr(x), Ty::Ptr(y)) = (&from, &p.t) {
                    if x.t == y.t {
                        Some(0) // qualification adjustment
                    } else {
                        Some(2)
                    }
                } else if self.promoted(&from) == p.t && from != p.t && from.is_integral() || (from == Ty::Float && p.t == Ty::Double) {
                    Some(1)
                } else {
                    Some(2)
                }
            }
        }
    }

    pub fn convert_args(&mut self, ft: &FnTy, args: Vec<Expr>, line: u32, fid: Option<DeclId>) -> Vec<Expr> {
        let np = ft.params.len();
        // In a pattern, an argument of unknown type can be an expansion
        // of a pack whose length is not known yet.
        let open_pack = self.in_template() && args.iter().any(|a| a.ty.is_unknown());
        if fid.is_none() && !open_pack {
            if args.len() < np || (args.len() > np && !ft.variadic) {
                self.error(line, "PFX-5", "wrong number of arguments in a call through a pointer");
            }
        }
        let mut out = Vec::new();
        for (i, a) in args.into_iter().enumerate() {
            if i < np {
                let p = ft.params[i].clone();
                let al = a.line;
                let c = if is_raw_list(&a) {
                    self.convert_init(a, &p, InitForm::CopyList, al)
                } else {
                    if let Ty::Class(pc) = p.t {
                        // PFX-6: a class parameter is copy-initialized.
                        if nonref(&a.ty).t == Ty::Class(pc) {
                            let mut x = a.clone();
                            x.ty = nonref(&x.ty);
                            self.check_copy_from(pc, &x, true, false, al);
                        }
                    }
                    match self.implicit(a, &p, false) {
                        Ok(x) => x,
                        Err(m) => {
                            self.error(al, "PFX-16", format!("argument {}: {}", i + 1, m));
                            Expr::unknown(al, "bad argument")
                        }
                    }
                };
                out.push(c);
            } else {
                // ellipsis: promotions. An overload set has no target type
                // there (OVL-20, ICS-47).
                if self.overload_set_no_target(&a, line) {
                    out.push(Expr::unknown(line, "bad argument"));
                    continue;
                }
                let a = self.rvalue(a);
                out.push(a);
            }
        }
        out
    }

    // ---------------- casts ----------------

    pub fn build_cast(&mut self, kind: CastKind, ty: QTy, e: Expr, line: u32) -> Expr {
        if ty.is_unknown() {
            return Expr::unknown(line, "cast to unknown type");
        }
        let (target, cat) = match &ty.t {
            Ty::LRef(i) => ((**i).clone(), Cat::L),
            Ty::RRef(i) => ((**i).clone(), if matches!(i.t, Ty::Func(_)) { Cat::L } else { Cat::X }),
            _ => (ty.unqual(), Cat::PR),
        };
        let ptr_to_fn = match &ty.t {
            Ty::Ptr(x) | Ty::LRef(x) | Ty::RRef(x) => matches!(x.t, Ty::Func(_)),
            _ => false,
        };
        if kind == CastKind::Static && ptr_to_fn && !self.in_template() && matches!(e.k, EK::Overloads(_) | EK::TemplId(..)) {
            // OVL-19: the target type of a cast selects from the set.
            let mark = self.d.list.len();
            let r = self.convert_init(e.clone(), &ty, crate::decl2::InitForm::Copy, line);
            if self.d.list.len() == mark && !matches!(r.k, EK::Unknown(_)) && !r.ty.is_unknown() {
                let mut r = r;
                r.cat = cat;
                return r;
            }
            if self.d.list.len() > mark && self.d.list[mark..].iter().all(|d| d.line != line && d.sev == crate::diag::Severity::Error) {
                // DED-29: an error in the body of a specialization that the
                // target type made the front end instantiate.
                return Expr::unknown(line, "bad cast");
            }
            self.d.list.truncate(mark);
        }
        if target.t == Ty::Void && cat == Cat::PR && self.overload_set_no_target(&e, line) {
            // OVL-20: a cast to void gives no target type.
            return Expr::unknown(line, "bad cast");
        }
        if e.ty.is_unknown() || matches!(e.k, EK::Overloads(_) | EK::BoundMember(..)) {
            return Expr::new(EK::Unknown("cast of unknown operand".into()), target, cat, line);
        }
        if kind == CastKind::Dynamic {
            let mark = self.d.list.len();
            if !self.in_template() {
                self.check_dynamic_cast(&e, &ty, line);
            }
            if self.d.list.len() == mark && !self.in_template() && !matches!(e.k, EK::Unknown(_)) {
                // VIRT rules: the cast checks the object at run time.
                let x = if cat == Cat::PR { self.rvalue(e) } else { e };
                return Expr::new(EK::DynamicCast(Box::new(x)), target, cat, line);
            }
            return Expr::new(EK::Unknown("dynamic_cast".into()), target, cat, line);
        }
        if kind == CastKind::Reinterpret {
            if cat == Cat::PR {
                let et = nonref(&e.ty);
                if let (Ty::Ptr(_), t) = (&et.t, &target.t) {
                    if t.is_integral() && t.int_bits() < if ilp32() { 32 } else { 64 } {
                        self.error(line, "CAST-7", "reinterpret_cast of a pointer to a smaller integer");
                    }
                }
                if let (Ty::Ptr(a), Ty::Ptr(b)) = (&et.t, &target.t) {
                    if (a.c && !b.c) || (a.v && !b.v) {
                        self.error(line, "CAST-7", "reinterpret_cast removes a cv-qualifier");
                    }
                }
                if let (Ty::MemPtr(_, a), Ty::MemPtr(_, b)) = (&et.t, &target.t) {
                    if matches!(a.t, Ty::Func(_)) != matches!(b.t, Ty::Func(_)) && !self.in_template() {
                        // MPTR-11: no conversion between a pointer to data
                        // member and a pointer to member function.
                        self.error(line, "MPTR-11", "reinterpret_cast between a pointer to data member and a pointer to member function");
                        return Expr::unknown(line, "bad cast");
                    }
                }
            }
            // CE-4 / CAST-8: not in a constant expression. The interpreter
            // runs the cast (CAST-6, STG-16).
            if self.in_template() {
                return Expr::new(EK::Unknown("CE-4: reinterpret_cast".into()), target, cat, line);
            }
            let x = if cat == Cat::PR { self.rvalue(e) } else { e };
            return Expr::new(EK::Reinterpret(Box::new(x), "reinterpret_cast".into()), target, cat, line);
        }
        if cat != Cat::PR {
            // cast to a reference: binds to the operand
            let et = nonref(&e.ty);
            if kind == CastKind::Static && cat == Cat::L && e.cat != Cat::L && !target.c {
                self.error(line, "CAST-2", "static_cast of an rvalue to a non-const lvalue reference");
            }
            if (kind == CastKind::Static) && ((et.c && !target.c) || (et.v && !target.v)) {
                self.error(line, "CAST-2", "static_cast removes a cv-qualifier");
            }
            if let (Ty::Class(a), Ty::Class(b)) = (&et.t, &target.t) {
                if kind == CastKind::Static && a != b && !self.in_template() && self.has_virtual_base(*b, *a, 0) {
                    // DER-11: no static_cast from a virtual base.
                    self.error(line, "DER-11", format!("static_cast from the virtual base '{}' to the derived class '{}'", self.classes[*a].name, self.classes[*b].name));
                    return Expr::unknown(line, "bad cast");
                }
            }
            if et.t == target.t || matches!((&et.t, &target.t), (Ty::Class(a), Ty::Class(b)) if self.is_base_of(*b, *a) || self.is_base_of(*a, *b)) {
                return Expr::new(EK::Conv(Conv::Bind, Box::new(e)), target, cat, line);
            }
            if kind == CastKind::Static && matches!(et.t, Ty::Class(_)) && !self.in_template() {
                // CAST-1, ICS-45: the operand of a cast is direct-initialized,
                // and the reference binds the result of a conversion
                // function.
                let rt = QTy::new(if cat == Cat::L { Ty::LRef(Box::new(target.clone())) } else { Ty::RRef(Box::new(target.clone())) });
                let saved = std::mem::replace(&mut self.direct_conv, true);
                let mark = self.d.list.len();
                let x = self.user_conv(&e, &rt, line);
                self.direct_conv = saved;
                match x {
                    Some(mut x) if self.d.list.len() == mark && !matches!(x.k, EK::Unknown(_)) => {
                        x.ty = target;
                        x.cat = cat;
                        return x;
                    }
                    _ => self.d.list.truncate(mark),
                }
                if let Ty::Class(sc) = et.t {
                    if self.class_ok(sc) && !self.class_uncertain(sc) && self.classes[sc].complete && !target.is_unknown() && !matches!(target.t, Ty::Dep(_)) {
                        // ICS-45, OVL-15: no conversion function gives a
                        // result that the reference binds, and no
                        // temporary can be copy-initialized from the
                        // operand.
                        let saved = std::mem::replace(&mut self.direct_conv, true);
                        let none = matches!(self.ics_user_from_class(&e, sc, &target, Some(cat != Cat::L)), Ok(None));
                        self.direct_conv = saved;
                        let temp_none = match target.t {
                            Ty::Class(tc) => self.class_ok(tc) && !self.class_uncertain(tc) && self.classes[tc].complete && matches!(self.ics_user_to_class(&e, tc), Ok(None)),
                            _ => cat == Cat::L && !target.c || matches!(self.ics_user_from_class(&e, sc, &target.unqual(), None), Ok(None)),
                        };
                        if none && temp_none {
                            self.error(line, "CAST-19", format!("static_cast: no conversion of '{}' to the reference type", self.classes[sc].name));
                            return Expr::unknown(line, "bad cast");
                        }
                    }
                }
            }
            return Expr::new(EK::Unknown("reference cast".into()), target, cat, line);
        }
        if target.t == Ty::Void {
            if self.overload_set_no_target(&e, line) {
                return Expr::unknown(line, "bad cast");
            }
            return self.conv(e, Conv::ToVoid, target);
        }
        if let Ty::Class(c) = target.t {
            if kind == CastKind::Static && !self.in_template() && !self.class_uncertain(c) && self.classes[c].complete && !e.ty.is_unknown() {
                // CAST-1: the result is direct-initialized from the
                // operand, as `T t(e)` is.
                let mark = self.d.list.len();
                if let Ty::Class(sc) = nonref(&e.ty).t {
                    if sc != c && !self.is_base_of(c, sc) {
                        // OVL-14, CVF-1: a conversion function gives the class.
                        if let Ok(x) = self.implicit(e.clone(), &target, true) {
                            if self.d.list.len() == mark && x.cat == Cat::PR && matches!(x.k, EK::Call(..)) {
                                let mut x = x;
                                x.ty = target;
                                return x;
                            }
                        }
                        self.d.list.truncate(mark);
                    }
                }
                if let Ty::Class(sc) = nonref(&e.ty).t {
                    if sc != c && !self.is_base_of(c, sc) && self.class_ok(sc) && !self.class_uncertain(sc) {
                        // ICS-45: an explicit conversion function whose
                        // result is `C` or a reference to `C` serves the
                        // parameter of the copy or move constructor.
                        let saved = std::mem::replace(&mut self.direct_conv, true);
                        let r = self.ics_user_from_class(&e, sc, &target.unqual(), None);
                        self.direct_conv = saved;
                        if let Ok(Some(crate::ics::Ics { kind: crate::ics::IK::User, user: Some(crate::ics::UKey::Fn(fid)), .. })) = r {
                            let rt = self.func(fid).map(|f| nonref(&f.fty.ret).t.clone());
                            if rt == Some(Ty::Class(c)) {
                                if self.in_unevaluated == 0 {
                                    self.use_fn(fid);
                                }
                                let (t, rc) = ret_cat(&self.func(fid).unwrap().fty.ret.clone());
                                let call = Expr::new(EK::Call(Box::new(e.clone()), vec![], Some(fid)), t, rc, line);
                                let init = Expr::new(EK::InitList(vec![call]), QTy::unknown(), Cat::PR, line);
                                let x = self.convert_init(init, &target, crate::decl2::InitForm::Paren(1), line);
                                if self.d.list.len() == mark && !matches!(x.k, EK::Unknown(_)) {
                                    let mut x = Expr::new(EK::Conv(Conv::NoOp, Box::new(x)), target.clone(), Cat::PR, line);
                                    x.ty = target;
                                    return x;
                                }
                                self.d.list.truncate(mark);
                            }
                        }
                    }
                }
                let init = Expr::new(EK::InitList(vec![e.clone()]), QTy::unknown(), Cat::PR, line);
                let x = self.convert_init(init, &target, crate::decl2::InitForm::Paren(1), line);
                if self.d.list.len() == mark && !matches!(x.k, EK::Unknown(_)) && !x.ty.is_unknown() {
                    let mut x = x;
                    if x.cat != Cat::PR || matches!(x.k, EK::InitList(_)) {
                        x = Expr::new(EK::Conv(Conv::NoOp, Box::new(x)), target.clone(), Cat::PR, line);
                    }
                    x.ty = target;
                    return x;
                }
                self.d.list.truncate(mark);
            }
            return Expr::new(EK::Unknown("cast to class".into()), target, Cat::PR, line);
        }
        if let Ty::Class(c) = nonref(&e.ty).t {
            if self.class_converts(c) == Some(false) && kind != CastKind::Const {
                self.error(line, "CAST-19", "no conversion from a class without conversion functions");
            }
            if matches!(kind, CastKind::Static | CastKind::Functional) {
                // CAST-1, CVF-3: the cast is a direct-initialization, so
                // explicit conversion functions count.
                let saved = std::mem::replace(&mut self.direct_conv, true);
                let x = self.user_conv(&e, &target, line);
                self.direct_conv = saved;
                if let Some(x) = x {
                    return x;
                }
                let et = nonref(&e.ty);
                if (et.c || et.v) && !self.in_template() && self.class_ok(c) && !self.class_uncertain(c) && !target.is_unknown() {
                    // CVF-9: a conversion function that is not const does
                    // not convert a const object.
                    let saved = std::mem::replace(&mut self.direct_conv, true);
                    let none = matches!(self.ics_user_from_class(&e, c, &target.unqual(), None), Ok(None));
                    let mut plain = e.clone();
                    plain.ty = QTy::new(et.t.clone());
                    let would = none && matches!(self.ics_user_from_class(&plain, c, &target.unqual(), None), Ok(Some(crate::ics::Ics { kind: crate::ics::IK::User, .. })));
                    self.direct_conv = saved;
                    if would {
                        self.error(line, "CVF-9", format!("the conversion function of '{}' is not const or volatile, and the object is", self.classes[c].name));
                        return Expr::unknown(line, "bad cast");
                    }
                }
                if kind == CastKind::Static && !self.in_template() && self.class_ok(c) && !self.class_uncertain(c) && self.classes[c].complete && !target.is_unknown() && !matches!(target.t, Ty::Dep(_)) {
                    // CAST-19, ICS-45: no conversion function converts the
                    // operand to the target.
                    let saved = std::mem::replace(&mut self.direct_conv, true);
                    let none = matches!(self.ics_user_from_class(&e, c, &target.unqual(), None), Ok(None));
                    self.direct_conv = saved;
                    if none {
                        self.error(line, "CAST-19", format!("static_cast: no conversion function of '{}' converts to the target type", self.classes[c].name));
                        return Expr::unknown(line, "bad cast");
                    }
                }
            }
            return Expr::new(EK::Unknown("conversion function".into()), target, Cat::PR, line);
        }
        let null = self.is_null_ptr_constant(&e);
        let e = self.rvalue(e);
        let from = e.ty.t.clone();
        let to = target.t.clone();
        if kind == CastKind::Const {
            let ok = match (&from, &to) {
                (Ty::Ptr(a), Ty::Ptr(b)) => self.same_ignoring_cv(a, b) && !matches!(a.t, Ty::Func(_)),
                (Ty::MemPtr(..), Ty::MemPtr(..)) => true,
                _ => false,
            };
            if !ok {
                self.error(line, "CAST-4", "const_cast can only change cv-qualifiers of a pointer or reference");
                return Expr::unknown(line, "bad cast");
            }
            return Expr::new(EK::Conv(Conv::NoOp, Box::new(e)), target, Cat::PR, line);
        }
        let arith_like = |p: &P, t: &Ty| t.is_arithmetic() || matches!(t, Ty::Enum(_)) || p.is_unscoped_enum(t);
        if arith_like(self, &from) && arith_like(self, &to) {
            return Expr::new(EK::Conv(Conv::Arith, Box::new(e)), target, Cat::PR, line);
        }
        if to == Ty::Bool && matches!(from, Ty::Ptr(_) | Ty::MemPtr(..) | Ty::NullPtr) {
            return self.conv(e, Conv::ToBool, target);
        }
        match (&from, &to) {
            (_, Ty::Ptr(_)) | (_, Ty::MemPtr(..)) | (_, Ty::NullPtr) if null || from == Ty::NullPtr => {
                return self.conv(e, Conv::NullPtr, target);
            }
            (Ty::Ptr(a), Ty::Ptr(b)) => {
                let cv_ok = (!a.c || b.c) && (!a.v || b.v);
                let related = a.t == b.t
                    || a.t == Ty::Void
                    || b.t == Ty::Void
                    || matches!((&a.t, &b.t), (Ty::Class(x), Ty::Class(y)) if self.is_base_of(*x, *y) || self.is_base_of(*y, *x));
                let func_obj = matches!(a.t, Ty::Func(_)) != matches!(b.t, Ty::Func(_));
                if kind == CastKind::Static {
                    if !cv_ok {
                        self.error(line, "CAST-2", "static_cast removes a cv-qualifier");
                        return Expr::unknown(line, "bad cast");
                    }
                    if !related || func_obj {
                        self.error_because(line, "CAST-2", "CAST-19", "static_cast between unrelated pointer types");
                        return Expr::unknown(line, "bad cast");
                    }
                    if let (Ty::Class(x), Ty::Class(y)) = (&a.t, &b.t) {
                        if x != y && self.is_base_of(*x, *y) && !self.in_template() && !self.base_accessible(*y, *x) {
                            // CAST-2: a downcast needs an accessible base.
                            self.error(line, "CAST-2", "static_cast to a derived class through a base that is not accessible");
                            return Expr::unknown(line, "bad cast");
                        }
                        if x != y && !self.in_template() && self.has_virtual_base(*y, *x, 0) {
                            // DER-11: no static_cast from a virtual base.
                            self.error(line, "DER-11", format!("static_cast from the virtual base '{}' to the derived class '{}'", self.classes[*x].name, self.classes[*y].name));
                            return Expr::unknown(line, "bad cast");
                        }
                    }
                }
                if a.t == Ty::Void && b.t != Ty::Void {
                    // A pointer to void to a pointer to an object type; CE-4
                    // in constant evaluation (the interpreter checks).
                    if self.in_template() || matches!(b.t, Ty::Func(_)) {
                        return Expr::new(EK::Unknown("CE-4: conversion from void*".into()), target, Cat::PR, line);
                    }
                    return self.conv(e, Conv::PtrConv, target);
                }
                if related && cv_ok && !func_obj {
                    return self.conv(e, Conv::PtrConv, target);
                }
                return if self.in_template() { Expr::new(EK::Unknown("CE-4: reinterpreting pointer cast".into()), target, Cat::PR, line) } else { Expr::new(EK::Reinterpret(Box::new(e), "reinterpreting pointer cast".into()), target, Cat::PR, line) };
            }
            (f, Ty::Ptr(_)) if f.is_integral() || matches!(f, Ty::Enum(_)) => {
                if kind == CastKind::Static {
                    self.error_because(line, "CAST-2", "CAST-19", "static_cast of an integer to a pointer");
                    return Expr::unknown(line, "bad cast");
                }
                return if self.in_template() { Expr::new(EK::Unknown("CE-4: integer to pointer".into()), target, Cat::PR, line) } else { Expr::new(EK::Reinterpret(Box::new(e), "integer to pointer".into()), target, Cat::PR, line) };
            }
            (Ty::Ptr(_), t) if t.is_integral() => {
                if kind == CastKind::Static {
                    self.error(line, "CAST-2", "static_cast of a pointer to an integer");
                    return Expr::unknown(line, "bad cast");
                }
                return if self.in_template() { Expr::new(EK::Unknown("CE-4: pointer to integer".into()), target, Cat::PR, line) } else { Expr::new(EK::Reinterpret(Box::new(e), "pointer to integer".into()), target, Cat::PR, line) };
            }
            (Ty::MemPtr(..), Ty::MemPtr(..)) => {
                // MPTR-4: the value keeps its member.
                if self.in_template() || matches!(e.k, EK::Unknown(_)) {
                    return Expr::new(EK::Unknown("member pointer cast".into()), target, Cat::PR, line);
                }
                if let (Ty::MemPtr(fc, _), Ty::MemPtr(tc, _), CastKind::Static) = (&nonref(&e.ty).t, &target.t, kind) {
                    // MPTR-11: a static_cast from a member of a derived class
                    // to one of its base goes through a base that is not
                    // virtual, ambiguous, or inaccessible.
                    let (d, b) = (*fc, *tc);
                    if d != b && self.is_base_of(b, d) && !self.class_uncertain(d) && (self.has_virtual_base(d, b, 0) || self.base_count(b, d, 0) > 1 || !self.base_accessible(d, b)) {
                        self.error(line, "MPTR-11", format!("the pointer to member of '{}' converts to one of '{}' through a virtual, ambiguous, or inaccessible base", self.classes[d].name, self.classes[b].name));
                        return Expr::unknown(line, "bad cast");
                    }
                }
                return Expr::new(EK::Conv(Conv::PtrConv, Box::new(e)), target, Cat::PR, line);
            }
            _ => {}
        }
        match self.implicit(e, &target, true) {
            Ok(x) => x,
            Err(m) => {
                self.error(line, "CAST-19", format!("invalid cast: {}", m));
                Expr::unknown(line, "bad cast")
            }
        }
    }

    fn same_ignoring_cv(&self, a: &QTy, b: &QTy) -> bool {
        match (&a.t, &b.t) {
            (Ty::Ptr(x), Ty::Ptr(y)) => self.same_ignoring_cv(x, y),
            (x, y) => x == y,
        }
    }

    // ---------------- initialization ----------------

    /// Convert an initializer to the type of the declared object
    /// (INIT-4, LIST-1, LIST-2, LIST-6, REF rules).
    /// An aggregate with bases that are not virtual (LIST rules).
    pub fn aggregate_with_bases(&self, c: usize) -> bool {
        let ci = &self.classes[c];
        ci.complete && !ci.opaque && !ci.template && ci.ctors.is_empty() && !ci.has_virtual && !self.class_uncertain(c) && ci.bases.iter().all(|b| !b.virt && b.access == 0 && b.class.is_some())
    }

    /// The number of scalar elements that a list with brace elision gives
    /// a type: 1 for a scalar, the sum over the bases and members of an
    /// aggregate class, and the bound times the element for an array.
    /// None for a class that is not a simple aggregate.
    fn agg_width(&self, t: &QTy, depth: u32) -> Option<usize> {
        if depth > 8 {
            return None;
        }
        match &t.t {
            Ty::Class(c) => {
                let c = *c;
                if !self.aggregate_with_bases(c) {
                    return None;
                }
                if self.classes[c].key == 2 {
                    // LIST-6, UNI-7: brace elision into a union initializes
                    // its first member.
                    let first = self.classes[c].fields.iter().find_map(|&f| match &self.decls[f].k {
                        DK::Field(fi) if !(fi.bits.is_some() && self.decls[f].name.is_empty()) => Some(fi.ty.clone()),
                        _ => None,
                    })?;
                    return self.agg_width(&first, depth + 1);
                }
                let mut n = 0;
                for b in &self.classes[c].bases {
                    n += self.agg_width(&QTy::new(Ty::Class(b.class?)), depth + 1)?;
                }
                for &f in &self.classes[c].fields {
                    if let DK::Field(fi) = &self.decls[f].k {
                        if fi.bits.is_some() && self.decls[f].name.is_empty() {
                            continue;
                        }
                        n += self.agg_width(&fi.ty, depth + 1)?;
                    }
                }
                Some(n)
            }
            Ty::Array(el, Some(b)) => Some(self.agg_width(el, depth + 1)? * *b as usize),
            Ty::Array(..) | Ty::Unknown | Ty::Dep(_) => None,
            t if t.is_ref() => None,
            _ => Some(1),
        }
    }

    /// Aggregate initialization from a list whose elements each match a
    /// base or member, with no brace elision: the bases first, then the
    /// members (LIST rules). None when the list needs more, or when a
    /// conversion raises a diagnostic, which the older paths then report.
    fn quiet_aggregate(&mut self, c: usize, v: &[Expr], ty: &QTy, is_paren: bool, line: u32) -> Option<Expr> {
        if self.in_template() {
            return None;
        }
        let mut targets: Vec<QTy> = self.classes[c].bases.iter().map(|b| QTy::new(Ty::Class(b.class.unwrap()))).collect();
        let is_union = self.classes[c].key == 2;
        for &f in &self.classes[c].fields {
            if let DK::Field(fi) = &self.decls[f].k {
                if fi.bits.is_some() && self.decls[f].name.is_empty() {
                    continue;
                }
                targets.push(fi.ty.clone());
                if is_union {
                    break;
                }
            }
        }
        if v.iter().any(|x| matches!(x.k, EK::Unknown(_)) || (x.ty.is_unknown() && !matches!(x.k, EK::InitList(_)))) {
            return None;
        }
        // LIST-6: brace elision. An element that does not start a nested
        // list, and does not initialize the base or member whole, takes
        // its place with the elements after it, as many as the base or
        // member needs.
        let elided: Vec<Expr>;
        let v: &[Expr] = if !is_paren && v.len() > 0 {
            let mut out = Vec::new();
            let mut k = 0;
            for t in &targets {
                if k >= v.len() {
                    break;
                }
                let x = &v[k];
                let whole = matches!(x.k, EK::InitList(_) | EK::Str(_)) || match (&t.t, &nonref(&x.ty).t) {
                    (Ty::Class(tc), Ty::Class(xc)) if xc == tc || self.is_base_of(*tc, *xc) => true,
                    (Ty::Class(_), Ty::Class(xc)) if self.classes[*xc].has_conv || self.scopes[self.classes[*xc].scope].names.keys().any(|n| n.starts_with("operator ")) => {
                        // LIST-4: braces are not omitted for a value that
                        // initializes the member directly, such as an
                        // object whose conversion function gives the
                        // member's type.
                        let mark = self.d.list.len();
                        let r = self.ics_value_pub(x, t);
                        self.d.list.truncate(mark);
                        match r {
                            Ok(Some(s)) => s.kind != crate::ics::IK::Ambig,
                            Ok(None) => false,
                            Err(_) => return None,
                        }
                    }
                    (Ty::Class(_) | Ty::Array(..), _) => false,
                    _ => true,
                };
                match (whole, self.agg_width(t, 0)) {
                    (false, Some(0)) => {
                        // LIST-13: an empty aggregate takes no element by
                        // brace elision; it needs braces of its own.
                        self.error(x.line, "LIST-13", "an empty aggregate base or member needs its own braces when later values follow");
                        return Some(Expr::unknown(line, "bad aggregate init"));
                    }
                    (false, Some(w)) => {
                        let n = w.min(v.len() - k);
                        out.push(Expr::new(EK::InitList(v[k..k + n].to_vec()), QTy::unknown(), Cat::PR, x.line));
                        k += n;
                    }
                    (false, None) => return None,
                    (true, _) => {
                        out.push(x.clone());
                        k += 1;
                    }
                }
            }
            if k < v.len() {
                // LIST-12: every base and member has its values, and
                // values are left. A value counted here for a member by
                // brace elision takes at most as many values as the
                // member has scalars, so the list has too many values.
                self.error(v[k].line, "LIST-12", "too many initializers for the aggregate");
                return Some(Expr::unknown(line, "bad aggregate init"));
            }
            elided = out;
            &elided
        } else {
            v
        };
        if v.len() > targets.len() {
            return None;
        }
        let mark = self.d.list.len();
        let mut out = Vec::new();
        for (x, t) in v.iter().zip(targets.iter()) {
            let xl = x.line;
            let xt = nonref(&x.ty).t;
            let e = if matches!(x.k, EK::InitList(_)) {
                self.convert_init(x.clone(), t, InitForm::CopyList, xl)
            } else if let Ty::Class(tc) = t.t {
                // LIST-3: a member of a class with a user-declared
                // constructor is not an aggregate, so no brace elision
                // applies, and the element copy-initializes the member
                // (CTOR-10).
                let not_aggregate = !self.classes[tc].ctors.is_empty() && self.classes[tc].complete && !self.class_uncertain(tc);
                // LIST-4: a value of another class reaches here only when
                // its conversion initializes the member whole.
                if !(xt == Ty::Class(tc) || matches!(xt, Ty::Class(_)) || not_aggregate) {
                    self.d.list.truncate(mark);
                    return None;
                }
                self.convert_init(x.clone(), t, InitForm::Copy, xl)
            } else if matches!(t.t, Ty::Array(..)) {
                if !matches!(x.k, EK::Str(_)) {
                    self.d.list.truncate(mark);
                    return None;
                }
                self.convert_init(x.clone(), t, InitForm::Copy, xl)
            } else {
                let f = if is_paren { InitForm::Copy } else { InitForm::CopyList };
                let x = if is_paren { x.clone() } else { Expr::new(EK::InitList(vec![x.clone()]), QTy::unknown(), Cat::PR, xl) };
                self.convert_init(x, t, f, xl)
            };
            if matches!(e.k, EK::Unknown(_)) {
                self.d.list.truncate(mark);
                return None;
            }
            out.push(e);
        }
        if self.d.list.len() != mark {
            self.d.list.truncate(mark);
            return None;
        }
        // LIST-16: a member with no value in the list and no default
        // member initializer is copy-initialized from an empty list, which
        // an explicit default constructor cannot do.
        let nb = self.classes[c].bases.len();
        if !is_paren && !is_union {
            let fields = self.classes[c].fields.clone();
            for (k, &f) in fields.iter().enumerate() {
                if nb + k < v.len() {
                    continue;
                }
                if let DK::Field(fi) = &self.decls[f].k {
                    if fi.init.is_some() {
                        continue;
                    }
                    if let Ty::Class(mc) = fi.ty.t {
                        if self.classes[mc].complete && !self.class_uncertain(mc) && self.default_ctor(mc).and_then(|d| self.func(d)).is_some_and(|df| df.explicit && !df.deleted) {
                            self.error(line, "LIST-16", format!("the member '{}' has no value in the list, and the default constructor of '{}' is explicit", self.decls[f].name, self.classes[mc].name));
                            return Some(Expr::unknown(line, "bad aggregate init"));
                        }
                    }
                }
            }
        }
        let mut x = Expr::new(EK::InitList(out), ty.clone(), Cat::PR, line);
        x.paren = is_paren;
        Some(x)
    }

    /// A reference bound to the temporary that a converted list
    /// initializes (REF-14), or unknown if the list did not convert.
    fn bind_temp_or_unknown(&mut self, x: Expr, inner: QTy, line: u32) -> Expr {
        if matches!(x.k, EK::Unknown(_)) || x.ty.is_unknown() {
            return Expr::new(EK::Unknown("reference bound to a temporary".into()), inner, Cat::L, line);
        }
        let mut x = x;
        x.cat = Cat::PR;
        Expr::new(EK::Conv(Conv::Bind, Box::new(x)), inner, Cat::L, line)
    }

    pub fn convert_init(&mut self, e: Expr, ty: &QTy, form: InitForm, line: u32) -> Expr {
        if ty.is_unknown() {
            return e;
        }
        self.check_class_copy_use(&e, ty, line);
        let items: Option<Vec<Expr>> = match &e.k {
            EK::InitList(v) if e.ty.is_unknown() => Some(v.clone()),
            _ => None,
        };
        let is_list = matches!(form, InitForm::CopyList | InitForm::DirectList) || (form == InitForm::None && items.is_some());
        let is_paren = matches!(form, InitForm::Paren(_));
        if let (Some(v), false) = (&items, self.in_template()) {
            let desig = |x: &Expr| matches!(&x.k, EK::Unknown(s) if s.starts_with("designator"));
            let n = v.iter().filter(|x| desig(x)).count();
            if n > 0 {
                let aggregate = match &ty.t {
                    Ty::Class(c) => self.classes[*c].ctors.is_empty() || self.class_uncertain(*c) || !self.classes[*c].complete,
                    _ => false,
                };
                if n < v.len() {
                    // LIST-14: designators on all elements or on none.
                    self.error(line, "LIST-14", "a braced list with designators on some elements only");
                    return Expr::unknown(line, "bad designated list");
                }
                if !aggregate && !ty.t.is_ref() {
                    // LIST-14, ILC-7: designators initialize an aggregate.
                    self.error(line, if matches!(ty.t, Ty::Class(_)) { "ILC-7" } else { "LIST-14" }, "designated initializers for a type that is not an aggregate");
                    return Expr::unknown(line, "bad designated list");
                }
            }
        }
        if let (Ty::Class(lc), Some(_), true) = (&ty.t, &items, is_list) {
            if self.init_list_elem(*lc).is_some() && self.classes[*lc].opaque && !self.in_template() && !self.init_list_shape_ok() && self.inst_depth < 64 {
                // LSUP-1: the program declares std::initializer_list with
                // another shape, so a braced list cannot make its object.
                self.error(line, "LSUP-1", "a braced list needs std::initializer_list, which is not declared as a class template of the needed shape");
                return Expr::unknown(line, "initializer list");
            }
        }
        if let (Ty::Class(lc), Some(v), true) = (&ty.t, &items, is_list) {
            if self.init_list_elem(*lc).is_some() && !self.classes[*lc].opaque && !self.in_template() {
                // ILC-1: a braced list makes the list object.
                return match self.make_init_list(*lc, v, line) {
                    Some(x) => x,
                    None => Expr::new(EK::Unknown("initializer list".into()), ty.unqual(), Cat::PR, line),
                };
            }
        }
        if form == InitForm::Copy && items.is_none() && !ty.t.is_ref() && !matches!(ty.t, Ty::Class(_)) && !self.in_template() {
            if let Ty::Class(sc) = nonref(&e.ty).t {
                if self.explicit_convs_only(sc, None) {
                    // CVF-11: copy-initialization does not use an explicit
                    // conversion function.
                    self.error(line, "CVF-11", format!("the conversion functions of '{}' are explicit, and copy-initialization does not use them", self.classes[sc].name));
                    return Expr::unknown(line, "bad conversion");
                }
            }
        }
        // references
        if ty.t.is_ref() {
            if let (Some(v), true, Ty::LRef(inner) | Ty::RRef(inner)) = (&items, is_list, &ty.t) {
                let inner = (**inner).clone();
                let lref_nonconst = matches!(ty.t, Ty::LRef(_)) && !(inner.c && !inner.v);
                if let Some(x) = self.list_binds_directly(v, &inner) {
                    // REF-6: the list's one expression initializes the
                    // reference, as `=` does.
                    if let (Ty::Array(_, None), Ty::Array(..)) = (&inner.t, &nonref(&x.ty).t) {
                        // An array of unknown bound binds an array of the
                        // same element type.
                        let t2 = QTy { t: nonref(&x.ty).t, c: inner.c, v: inner.v };
                        let rt = if matches!(ty.t, Ty::LRef(_)) { Ty::LRef(Box::new(t2)) } else { Ty::RRef(Box::new(t2)) };
                        return self.bind_ref(x, &QTy::new(rt), line, "INIT-7");
                    }
                    return self.bind_ref(x, ty, line, "INIT-7");
                }
                if matches!(inner.t, Ty::Func(_)) && !self.in_template() {
                    // REF-11: a function type has no temporary.
                    self.error(line, "REF-11", "the braced element does not have the referenced function type, and a function type has no temporary");
                    return Expr::unknown(line, "bad reference");
                }
                // REF-14: the list initializes a temporary of the
                // referenced type.
                if lref_nonconst && !inner.is_unknown() {
                    self.error(line, "REF-9", "non-const lvalue reference cannot bind to a temporary from a braced list");
                    return Expr::unknown(line, "bad reference init");
                }
                if let Ty::Array(el, None) = &inner.t {
                    if v.is_empty() {
                        // REF-11: the array has no elements.
                        self.error(line, "REF-11", "a braced list of no elements for a reference to an array of unknown bound");
                        return Expr::unknown(line, "bad reference init");
                    }
                    let n = match &v[0].k {
                        EK::Str(u) if v.len() == 1 => u.len() as u64 + 1,
                        _ => v.len() as u64,
                    };
                    let t = QTy { t: Ty::Array(el.clone(), Some(n)), c: false, v: false };
                    let x = self.convert_init(e, &t, InitForm::CopyList, line);
                    return self.bind_temp_or_unknown(x, QTy { t: t.t.clone(), c: inner.c, v: inner.v }, line);
                }
                // A class element for a reference to a scalar converts into
                // the temporary too (Q-0157).
                let class_to_scalar = v.len() == 1 && matches!(nonref(&v[0].ty).t, Ty::Class(_)) && !matches!(inner.t, Ty::Class(_) | Ty::Unknown | Ty::Dep(_));
                if matches!(inner.t, Ty::Array(..) | Ty::Class(_)) || v.len() != 1 || class_to_scalar {
                    let x = self.convert_init(e, &inner.unqual(), InitForm::CopyList, line);
                    return self.bind_temp_or_unknown(x, inner, line);
                }
            }
            let one = match (&items, is_list || is_paren) {
                (Some(v), true) => {
                    if v.len() != 1 {
                        if !(is_list && v.is_empty() && false) {
                            self.error(line, "REF-11", "reference initializer must hold exactly one expression");
                        }
                        return Expr::unknown(line, "bad reference init");
                    }
                    let x = v[0].clone();
                    if is_list {
                        if let Ty::LRef(i) | Ty::RRef(i) = &ty.t {
                            if self.is_narrowing(&x, &i.t) && !matches!(x.ty.t, Ty::Class(_)) && nonref(&x.ty).t != i.t {
                                self.error(line, "LIST-2", "narrowing conversion in a braced reference initializer");
                            }
                        }
                    }
                    x
                }
                _ => e,
            };
            if form == InitForm::Copy && !self.in_template() && matches!(nonref(&one.ty).t, Ty::Class(_)) {
                // OVL-15, OVL-35: copy-initialization of a reference from a
                // class takes only the conversion functions that are not
                // explicit, directly or through a temporary.
                if let Ok(None) = self.ics_form(&one, ty, false) {
                    self.error(line, "OVL-35", format!("no conversion function initializes a reference of type {} from {}", show(ty), show(&nonref(&one.ty))));
                    return Expr::unknown(line, "bad reference init");
                }
            }
            // ICS-45: a direct initialization can use an explicit
            // conversion function.
            let direct = matches!(form, InitForm::Paren(_) | InitForm::DirectList);
            let dc = direct || self.direct_conv;
            let saved = std::mem::replace(&mut self.direct_conv, dc);
            let x = self.bind_ref(one, ty, line, "INIT-7");
            self.direct_conv = saved;
            return x;
        }
        match &ty.t {
            Ty::Array(el, n) => {
                let el = (**el).clone();
                // string literal for a character array (LIST-6)
                let str_src = match (&items, &e.k) {
                    (Some(v), _) if v.len() == 1 && matches!(v[0].k, EK::Str(_)) => Some(v[0].clone()),
                    (None, EK::Str(_)) => Some(e.clone()),
                    _ => None,
                };
                if let Some(s) = str_src {
                    if el.t.is_integral() && el.t != Ty::Bool && !matches!(el.t, Ty::Short | Ty::UShort | Ty::Int | Ty::UInt | Ty::Long | Ty::ULong | Ty::LLong | Ty::ULLong) {
                        let st = match &s.ty.t {
                            Ty::Array(c, _) => c.t.clone(),
                            _ => Ty::Unknown,
                        };
                        let compatible = st == el.t
                            || (st == Ty::Char8 && matches!(el.t, Ty::Char | Ty::UChar))
                            || (st == Ty::Char && matches!(el.t, Ty::SChar | Ty::UChar));
                        if !compatible {
                            // LIST-6: the literal's character type must match.
                            self.error(line, "LIST-15", "string literal of the wrong character type for the array");
                        }
                        if let (Some(n), EK::Str(u)) = (n, &s.k) {
                            if (u.len() as u64) + 1 > *n && (u.len() as u64) > *n {
                                self.error(line, "LIST-15", "string literal too long for the array");
                            } else if (u.len() as u64) + 1 > *n {
                                self.error(line, "LIST-15", "string literal and its null character do not fit in the array");
                            }
                        }
                        return Expr::new(EK::Conv(Conv::NoOp, Box::new(s)), ty.clone(), Cat::PR, line);
                    }
                }
                let Some(mut v) = items.clone() else {
                    if matches!(e.ty.t, Ty::Array(..)) {
                        // INIT-5: an array must not be initialized from an array.
                        self.error(line, "INIT-8", "array initialized from another array");
                    } else if !e.ty.is_unknown() {
                        self.error(line, "INIT-8", "array initialized from a value that is not a list");
                    }
                    return Expr::unknown(line, "bad array init");
                };
                if let Some(n) = n {
                    // LIST-7: brace elision applies to an element that is
                    // an array or an aggregate, for a value that is not a
                    // list and not an object of the element's class.
                    let elided = |p: &P, x: &Expr| -> bool {
                        if matches!(x.k, EK::InitList(_)) {
                            return false;
                        }
                        match &el.t {
                            Ty::Array(..) => true,
                            Ty::Class(c) => {
                                let xt = nonref(&x.ty).t;
                                let same = matches!(xt, Ty::Class(d) if d == *c || p.is_base_of(*c, d));
                                let aggregate = p.classes[*c].ctors.is_empty() && !p.class_uncertain(*c);
                                !same && (aggregate || x.ty.is_unknown())
                            }
                            _ => false,
                        }
                    };
                    let mut nested_elision = v.iter().any(|x| elided(self, x));
                    if nested_elision && !is_paren && !self.in_template() && !v.iter().any(|x| matches!(x.k, EK::Str(_)) || matches!(nonref(&x.ty).t, Ty::Array(..))) {
                        // LIST-6: each element that is not a list takes as
                        // many values as its type needs, in order.
                        if let Some(w) = self.agg_width(&el, 0).filter(|&w| w > 0) {
                            let mut out = Vec::new();
                            let mut k = 0;
                            while k < v.len() {
                                if elided(self, &v[k]) {
                                    let m = w.min(v.len() - k);
                                    out.push(Expr::new(EK::InitList(v[k..k + m].to_vec()), QTy::unknown(), Cat::PR, v[k].line));
                                    k += m;
                                } else {
                                    out.push(v[k].clone());
                                    k += 1;
                                }
                            }
                            v = out;
                            nested_elision = false;
                        }
                    }
                    if !nested_elision && v.len() as u64 > *n {
                        // DCL-5: not more elements than the bound.
                        self.error(line, "DCL-19", "too many initializers for the array");
                        return Expr::unknown(line, "bad array init");
                    }
                    if nested_elision {
                        return Expr::new(EK::Unknown("brace elision".into()), ty.clone(), Cat::PR, line);
                    }
                }
                let sub_form = if is_paren { InitForm::Copy } else { InitForm::CopyList };
                let mut out = Vec::new();
                for x in v {
                    let xl = x.line;
                    if !is_paren && !self.in_template() && !matches!(x.k, EK::InitList(_)) && !matches!(el.t, Ty::Class(_) | Ty::Array(..) | Ty::Unknown | Ty::Dep(_)) && !matches!(nonref(&x.ty).t, Ty::Class(_)) && self.is_narrowing(&x, &el.t) {
                        // LIST-2: no narrowing conversion of an element in
                        // braces (also CTOR-21 for a member initializer).
                        self.error(xl, "LIST-2", format!("narrowing conversion from {} to {} in braces", show(&nonref(&x.ty)), show(&el)));
                        return Expr::unknown(line, "bad array init");
                    }
                    // LIST rules: an element that is not a list
                    // copy-initializes an element of class type.
                    let f = if matches!(x.k, EK::InitList(_)) { InitForm::CopyList } else if matches!(el.t, Ty::Class(c) if !self.classes[c].ctors.is_empty() && !matches!(nonref(&x.ty).t, Ty::Class(_))) { InitForm::Copy } else { sub_form };
                    out.push(self.convert_init(x, &el, f, xl));
                }
                Expr::new(EK::InitList(out), ty.clone(), Cat::PR, line)
            }
            Ty::Class(c) => {
                let c = *c;
                if !self.classes[c].complete && !self.in_template() {
                    // INST-1: the initialization needs the class complete.
                    self.require_class(c, line);
                }
                if !self.classes[c].complete && !self.in_template() {
                    self.error(line, "TYN-7", "object of incomplete class type");
                    return Expr::unknown(line, "incomplete");
                }
                // A copy or move from a glvalue of the same class.
                let src = match (&items, form) {
                    (Some(v), InitForm::Paren(1) | InitForm::DirectList | InitForm::CopyList) if v.len() == 1 => Some(v[0].clone()),
                    (None, InitForm::Copy | InitForm::None) => Some(e.clone()),
                    _ => None,
                };
                if let Some(x) = src {
                    if nonref(&x.ty).t == Ty::Class(c) {
                        let mut x = x;
                        x.ty = nonref(&x.ty);
                        self.check_copy_from(c, &x, matches!(form, InitForm::Copy | InitForm::CopyList | InitForm::None), false, line);
                    }
                }
                let simple_aggr = self.is_simple_aggregate(c);
                match (&items, simple_aggr) {
                    (Some(v), true) if is_list || is_paren => {
                        let fields = self.classes[c].fields.clone();
                        let is_union = self.classes[c].key == 2;
                        let nfields = if is_union { 1.min(fields.len()) } else { fields.len() };
                        if v.iter().any(|x| matches!(x.k, EK::Unknown(ref s) if s.starts_with("designator"))) {
                            // LIST-14: designators in declaration order.
                            let mut last = None;
                            for x in v {
                                if let EK::Unknown(s) = &x.k {
                                    let name = s.split_whitespace().nth(1).unwrap_or("");
                                    // UNI-7: a member of an anonymous union
                                    // names the union's one element.
                                    let in_anon = |p: &P, f: DeclId| match &p.decls[f].k {
                                        DK::Field(fi) if p.decls[f].name.is_empty() => matches!(fi.ty.t, Ty::Class(u) if p.classes[u].fields.iter().any(|&g| p.decls[g].name == name)),
                                        _ => false,
                                    };
                                    if let Some(i) = fields.iter().position(|&f| self.decls[f].name == name).or_else(|| fields.iter().position(|&f| in_anon(self, f))) {
                                        if last == Some(i) && self.decls[fields[i]].name.is_empty() {
                                            self.error(line, "LIST-14", format!("designator '{}' names a second member of an anonymous union, which is one element", name));
                                            break;
                                        }
                                        if last.is_some_and(|l| i <= l) {
                                            self.error(line, "LIST-14", format!("designator '{}' is out of declaration order", name));
                                            break;
                                        }
                                        last = Some(i);
                                    }
                                }
                            }
                            if let Some(x) = self.designated_list(c, v, ty, line) {
                                return x;
                            }
                            return Expr::new(EK::Unknown("designated initializers".into()), ty.clone(), Cat::PR, line);
                        }
                        if v.len() == 1 && (nonref(&v[0].ty).t == Ty::Class(c) || matches!(nonref(&v[0].ty).t, Ty::Class(d) if self.is_base_of(c, d))) {
                            // ICS-18: one element of the class or a derived
                            // class initializes by copy.
                            return v[0].clone();
                        }
                        let any_nested_aggr = fields.iter().any(|&f| matches!(&self.decls[f].k, DK::Field(fi) if matches!(fi.ty.t, Ty::Array(..) | Ty::Class(_))));
                        if is_paren && v.len() > nfields && any_nested_aggr && !self.in_template() {
                            // LIST-9 (Q-0043): no brace elision in
                            // parentheses, so each value is one member.
                            self.error(line, "LIST-9", "parenthesized aggregate initialization has more values than members; the values do not split across a nested aggregate");
                            return Expr::unknown(line, "bad aggregate init");
                        }
                        if v.len() > nfields && !any_nested_aggr {
                            // LIST-3: not more values than members.
                            self.error(line, "LIST-12", "too many initializers for the aggregate");
                            return Expr::unknown(line, "bad aggregate init");
                        }
                        if any_nested_aggr {
                            return self.quiet_aggregate(c, v, ty, is_paren, line).unwrap_or_else(|| Expr::new(EK::Unknown("aggregate with nested aggregates".into()), ty.clone(), Cat::PR, line));
                        }
                        let mut out = Vec::new();
                        for (i, x) in v.iter().enumerate() {
                            let fty = match &self.decls[fields[i]].k {
                                DK::Field(fi) => fi.ty.clone(),
                                _ => QTy::unknown(),
                            };
                            let xl = x.line;
                            let f = if is_paren { InitForm::Copy } else { InitForm::CopyList };
                            let f = if matches!(x.k, EK::InitList(_)) { InitForm::CopyList } else { f };
                            // LIST-2: an element in braces does not narrow.
                            let wrap = !is_paren && !matches!(x.k, EK::InitList(_)) && !fty.t.is_ref() && !matches!(fty.t, Ty::Class(_) | Ty::Array(..)) && !fty.is_unknown();
                            let xe = if wrap { Expr::new(EK::InitList(vec![x.clone()]), QTy::unknown(), Cat::PR, xl) } else { x.clone() };
                            out.push(self.convert_init(xe, &fty, f, xl));
                        }
                        // missing members: default member initializers or value-init
                        for &f in fields.iter().skip(v.len()).take(nfields.saturating_sub(v.len())) {
                            if let DK::Field(fi) = &self.decls[f].k {
                                if fi.ty.t.is_ref() && fi.init.is_none() {
                                    // LIST-3: a reference member needs a value.
                                    self.error(line, "LIST-12", "reference member without an initializer in the list");
                                }
                            }
                        }
                        let mut x = Expr::new(EK::InitList(out), ty.clone(), Cat::PR, line);
                        x.paren = is_paren;
                        x
                    }
                    (None, _) => {
                        let et = nonref(&e.ty);
                        if et.t == ty.t || et.is_unknown() {
                            return e;
                        }
                        if let Ty::Class(sc) = et.t {
                            if self.is_base_of(c, sc) {
                                return e;
                            }
                        }
                        if matches!(et.t, Ty::Class(_)) && form == InitForm::Copy && !self.in_template() {
                            // OVL-14: copy-initialization from another class
                            // takes the converting constructors and the
                            // conversion functions that are not explicit.
                            match self.ics_value_pub(&e, &ty.unqual()) {
                                Ok(Some(s)) if s.kind == crate::ics::IK::Ambig => {
                                    self.error(line, "OVL-36", format!("ambiguous initialization of '{}': no constructor or conversion function is better than each of the others", self.classes[c].name));
                                    return Expr::unknown(line, "bad init");
                                }
                                Ok(None) => {
                                    self.error(line, "OVL-35", format!("no constructor or conversion function initializes '{}' from {}", self.classes[c].name, show(&et)));
                                    return Expr::unknown(line, "bad init");
                                }
                                _ => {}
                            }
                        }
                        if matches!(et.t, Ty::Class(_)) {
                            if let Ok(Some(ics)) = self.ics_value_pub(&e, &ty.unqual()) {
                                // ICS-4: a conversion function to the class
                                // or a class derived from it. A function that
                                // returns the class by value gives the object.
                                if let Some(crate::ics::UKey::Fn(fid)) = ics.user {
                                    let f = self.func(fid).cloned();
                                    if let Some(f) = f.filter(|f| f.kind == FnKind::Conversion && !f.deleted && f.fty.ret.t == Ty::Class(c) && !f.fty.ret.t.is_ref()) {
                                        if self.in_unevaluated == 0 {
                                            self.use_fn(fid);
                                        }
                                        let _ = f;
                                        let call = Expr::new(EK::Call(Box::new(e.clone()), vec![], Some(fid)), ty.unqual(), Cat::PR, line);
                                        return call;
                                    }
                                    // OVL-14: a reference to the class, or a
                                    // derived class: the object copies the
                                    // result, or the base part of it.
                                    if let Some(f) = self.func(fid).cloned().filter(|f| f.kind == FnKind::Conversion && !f.deleted && matches!(nonref(&f.fty.ret).t, Ty::Class(r) if r == c || self.is_base_of(c, r))) {
                                        if self.in_unevaluated == 0 {
                                            self.use_fn(fid);
                                        }
                                        let (t, cat) = ret_cat(&f.fty.ret);
                                        let call = Expr::new(EK::Call(Box::new(e.clone()), vec![], Some(fid)), t.clone(), cat, line);
                                        if cat == Cat::PR {
                                            return Expr::new(EK::Conv(Conv::Bind, Box::new(call)), t, Cat::L, line);
                                        }
                                        return call;
                                    }
                                    // OVL-14: a converting constructor won, also
                                    // a specialization of a constructor
                                    // template (DED-1).
                                    if self.classes[c].ctors.contains(&fid) || self.func(fid).is_some_and(|f| f.kind == FnKind::Ctor && f.class == Some(c) && !f.template) {
                                        if let Some(f) = self.func(fid).cloned().filter(|f| !f.deleted && !f.fty.variadic && !f.fty.params.is_empty()) {
                                            let mark = self.d.list.len();
                                            let conv = self.convert_args(&f.fty, vec![e.clone()], line, Some(fid));
                                            if self.d.list.len() == mark && !conv.iter().any(|x| matches!(x.k, EK::Unknown(_))) {
                                                if self.in_unevaluated == 0 {
                                                    self.use_fn(fid);
                                                }
                                                return Expr::new(EK::Construct(fid, conv), ty.unqual(), Cat::PR, line);
                                            }
                                            self.d.list.truncate(mark);
                                        }
                                    }
                                }
                                return Expr::new(EK::Unknown("conversion function".into()), ty.clone(), Cat::PR, line);
                            }
                        }
                        if simple_aggr && !self.class_uncertain(c) && form == InitForm::Copy {
                            self.error(line, "INIT-7", "no conversion to an aggregate class");
                            return Expr::unknown(line, "bad init");
                        }
                        if form == InitForm::Copy && !self.classes[c].ctors.is_empty() && !matches!(et.t, Ty::Class(_)) {
                            let x = self.ctor_call_mode(c, std::slice::from_ref(&e), ty, false, true, false, line);
                            self.check_explicit_copy(&x, line);
                            return x;
                        }
                        Expr::new(EK::Unknown("constructor call".into()), ty.clone(), Cat::PR, line)
                    }
                    (Some(v), false) if v.is_empty() && (is_list || is_paren) && self.classes[c].ctors.is_empty() && !self.aggregate_with_bases(c) && !self.class_uncertain(c) && !self.in_template() && !self.classes[c].bases.iter().any(|b| b.virt) => {
                        // INIT-4: empty parentheses or braces value-initialize
                        // a class without user-declared constructors.
                        Expr::new(EK::ZeroInit, ty.unqual(), Cat::PR, line)
                    }
                    (Some(v), false) if (is_list || is_paren) && self.classes[c].ctors.is_empty() && self.aggregate_with_bases(c) && !(v.len() == 1 && nonref(&v[0].ty).t == Ty::Class(c)) => {
                        self.quiet_aggregate(c, v, ty, is_paren, line).unwrap_or_else(|| Expr::new(EK::Unknown("constructor call".into()), ty.clone(), Cat::PR, line))
                    }
                    (Some(v), false) if (is_list || is_paren) && !self.classes[c].ctors.is_empty() => {
                        let has_list_ctor = self.classes[c].ctors.iter().any(|&cid| self.func(cid).and_then(|f| f.params.first().copied()).and_then(|p| self.var(p)).is_some_and(|v| matches!(nonref(&v.ty).t, Ty::Class(lc) if self.init_list_elem(lc).is_some())));
                        if v.len() == 1 && (nonref(&v[0].ty).t == Ty::Class(c) || matches!(nonref(&v[0].ty).t, Ty::Class(d) if self.is_base_of(c, d) && !self.class_uncertain(d))) {
                            // ILC-3: the second phase takes the copy when no
                            // initializer-list constructor converts the
                            // element to its element type.
                            let list_takes = has_list_ctor && {
                                let ctors = self.classes[c].ctors.clone();
                                let mut takes = false;
                                for cid in ctors {
                                    let el = self.func(cid).and_then(|f| f.params.first().copied()).and_then(|p| self.var(p)).and_then(|vr| match nonref(&vr.ty).t {
                                        Ty::Class(lc) => self.init_list_elem(lc),
                                        _ => None,
                                    });
                                    let Some(el) = el else { continue };
                                    if el.is_unknown() || matches!(el.t, Ty::Dep(_)) || self.func(cid).is_some_and(|f| f.template) || !matches!(self.ics_form(&v[0], &el, false), Ok(None)) {
                                        takes = true;
                                    }
                                }
                                takes
                            };
                            if is_list && list_takes {
                                // ILC-3: a braced list goes to an
                                // initializer-list constructor first, also
                                // with one element of the class.
                                let x = self.ctor_call_mode(c, v, ty, is_list, false, form == InitForm::CopyList, line);
                                if form == InitForm::CopyList {
                                    self.check_explicit_copy(&x, line);
                                }
                                return x;
                            }
                            // CPY-15, CNS-11 (Q-0426): a constructor template
                            // never gives a constructor that takes the class
                            // by value, and its constraints are still checked
                            // for the copy, unless a constructor that is not a
                            // template takes the argument with no conversion.
                            if !self.in_template() && nonref(&v[0].ty).t == Ty::Class(c) {
                                let ctors = self.classes[c].ctors.clone();
                                let templs: Vec<DeclId> = ctors.iter().copied().filter(|&k| self.is_func_templ(k)).collect();
                                if !templs.is_empty() && !self.exact_non_template(&ctors, v) {
                                    let _ = self.expand_fn_templates(&templs, v, line);
                                }
                            }
                            // A copy or move: the interpreter chooses the copy
                            // or move constructor from the source's category.
                            return v[0].clone();
                        }
                        let x = self.ctor_call_mode(c, v, ty, is_list, false, form == InitForm::CopyList, line);
                        if form == InitForm::CopyList {
                            self.check_explicit_copy(&x, line);
                        }
                        x
                    }
                    (Some(v), false) if (is_list || is_paren) && v.len() == 1 && self.classes[c].ctors.is_empty() && !self.class_uncertain(c) && (nonref(&v[0].ty).t == Ty::Class(c) || matches!(nonref(&v[0].ty).t, Ty::Class(d) if self.is_base_of(c, d) && !self.class_uncertain(d))) => {
                        // CPY-8: a class with no user-declared constructor
                        // copies or moves with its implicit constructor,
                        // which the interpreter runs member by member.
                        v[0].clone()
                    }
                    _ => Expr::new(EK::Unknown("constructor call".into()), ty.clone(), Cat::PR, line),
                }
            }
            _ => {
                // scalar
                let target = ty.unqual();
                match items {
                    Some(v) if is_list || form == InitForm::None => {
                        if v.is_empty() {
                            // INIT-2: empty braces give zero.
                            return Expr::new(EK::ZeroInit, target, Cat::PR, line);
                        }
                        if v.len() > 1 {
                            // LIST-1: at most one expression.
                            self.error(line, "LIST-11", "braced initializer for a scalar with more than one value");
                            return Expr::unknown(line, "bad scalar init");
                        }
                        let x = v[0].clone();
                        if matches!(x.k, EK::InitList(_)) {
                            self.error(line, "LIST-11", "nested braces in a scalar initializer");
                            return Expr::unknown(line, "bad scalar init");
                        }
                        // ENUM-8: an enumeration from an integer in braces.
                        if let Ty::Enum(en) = &target.t {
                            let xt = nonref(&x.ty).t;
                            if xt != target.t && !xt.contains_unknown() && (xt.is_integral() || self.is_unscoped_enum(&xt)) {
                                let Some(fixed) = self.enums[*en].fixed.clone() else {
                                    self.error(line, "ENUM-19", "enumeration without a fixed underlying type initialized from an integer");
                                    return Expr::unknown(line, "bad enum init");
                                };
                                if form != InitForm::DirectList {
                                    self.error(line, "ENUM-19", "copy-list-initialization of an enumeration from an integer");
                                    return Expr::unknown(line, "bad enum init");
                                }
                                if self.is_narrowing(&x, &fixed) {
                                    self.error(line, "LIST-2", "narrowing conversion to the underlying type");
                                }
                                let x = self.rvalue(x);
                                return Expr::new(EK::Conv(Conv::Arith, Box::new(x)), target, Cat::PR, line);
                            }
                        }
                        if self.is_narrowing(&x, &target.t) {
                            // LIST-2: no narrowing conversion in braces.
                            self.error(line, "LIST-2", format!("narrowing conversion from {} to {} in braces", show(&nonref(&x.ty)), show(&target)));
                        }
                        let direct = form == InitForm::DirectList;
                        match self.implicit(x, &target, direct) {
                            Ok(c) => c,
                            Err(m) => {
                                self.error(line, "INIT-7", m);
                                Expr::unknown(line, "bad init")
                            }
                        }
                    }
                    Some(v) => {
                        // parentheses
                        if v.len() != 1 {
                            self.error(line, "INIT-7", "parenthesized initializer for a scalar must hold exactly one expression");
                            return Expr::unknown(line, "bad scalar init");
                        }
                        let x = v[0].clone();
                        if matches!(x.k, EK::InitList(_)) {
                            self.error(line, "INIT-7", "braced list in parentheses for a scalar");
                            return Expr::unknown(line, "bad scalar init");
                        }
                        match self.implicit(x, &target, true) {
                            Ok(c) => c,
                            Err(m) => {
                                self.error(line, "INIT-7", m);
                                Expr::unknown(line, "bad init")
                            }
                        }
                    }
                    None => {
                        if matches!(nonref(&e.ty).t, Ty::Class(_)) && !self.in_template() {
                            if let Ok(Some(s)) = self.ics_value_pub(&e, &target) {
                                if s.kind == crate::ics::IK::Ambig {
                                    // ICS-7, ICS-8: no conversion function is
                                    // best.
                                    self.error_because(line, "ICS-8", "CVF-12", format!("ambiguous conversion from {} to {}", show(&nonref(&e.ty)), show(&target)));
                                    return Expr::unknown(line, "bad init");
                                }
                            }
                        }
                        match self.implicit(e, &target, false) {
                            Ok(c) => c,
                            Err(m) => {
                                self.error(line, "INIT-7", m);
                                Expr::unknown(line, "bad init")
                            }
                        }
                    }
                }
            }
        }
    }

    /// REF-6: the one expression of a braced list that binds a reference
    /// to `inner` directly, if the list has one.
    pub fn list_binds_directly(&self, v: &[Expr], inner: &QTy) -> Option<Expr> {
        if v.len() != 1 || matches!(v[0].k, EK::InitList(_)) || matches!(&v[0].k, EK::Unknown(s) if s.starts_with("designator")) {
            return None;
        }
        let xt = nonref(&v[0].ty);
        if matches!(v[0].k, EK::Str(_)) && matches!(inner.t, Ty::Array(..)) {
            // ICS-38 and ICS-24 treat one string literal as a list that
            // initializes the array. A reference to the literal's own type,
            // or to an array of unknown bound of its element type, binds
            // directly to the literal.
            if let (Ty::Array(e1, n1), Ty::Array(e2, n2)) = (&inner.t, &xt.t) {
                if e1 == e2 && (n1.is_none() || n1 == n2) && inner.c == e1.c && inner.v == e1.v {
                    return Some(v[0].clone());
                }
            }
            return None;
        }
        let direct = match (&inner.t, &xt.t) {
            // REF-11: a function of another type (such as a noexcept
            // function) is not the referenced type, and a function type
            // has no temporary.
            (Ty::Func(a), Ty::Func(b)) => a == b,
            (Ty::Func(_), _) => true,
            (_, Ty::Unknown | Ty::Dep(_)) => true,
            (a, b) if a == b => true,
            (Ty::Class(a), Ty::Class(b)) => self.is_base_of(*a, *b),
            // The same type apart from cv-qualifiers, also on the elements
            // of an array.
            (Ty::Array(e1, None), Ty::Array(e2, _)) => self.strip_array_cv(&e1.t) == self.strip_array_cv(&e2.t),
            (a @ Ty::Array(..), b @ Ty::Array(..)) => self.strip_array_cv(a) == self.strip_array_cv(b),
            _ => false,
        };
        direct.then(|| v[0].clone())
    }

    /// An array type with the cv-qualifiers of its elements removed.
    pub fn strip_array_cv(&self, t: &Ty) -> Ty {
        match t {
            Ty::Array(e, n) => Ty::Array(Box::new(QTy::new(self.strip_array_cv(&e.t))), *n),
            t => t.clone(),
        }
    }

    /// VIRT-18: the conditions on a dynamic_cast.
    fn check_dynamic_cast(&mut self, e: &Expr, ty: &QTy, line: u32) {
        let et = nonref(&e.ty);
        // (target class or void, operand class) with their qualifiers
        let (tq, oq) = match &ty.t {
            Ty::Ptr(t) => {
                let Ty::Ptr(o) = &et.t else {
                    self.error(line, "VIRT-18", "dynamic_cast to a pointer needs a pointer operand");
                    return;
                };
                ((**t).clone(), (**o).clone())
            }
            Ty::LRef(t) | Ty::RRef(t) => {
                if e.cat == Cat::PR && matches!(ty.t, Ty::LRef(_)) {
                    self.error(line, "VIRT-18", "dynamic_cast to a reference needs an lvalue or xvalue operand");
                    return;
                }
                ((**t).clone(), et.clone())
            }
            _ => {
                self.error(line, "VIRT-18", "dynamic_cast target must be a pointer or reference to a class, or void*");
                return;
            }
        };
        let tc = match tq.t {
            Ty::Class(c) => Some(c),
            Ty::Void if matches!(ty.t, Ty::Ptr(_)) => None,
            _ => {
                self.error(line, "VIRT-18", "dynamic_cast target must be a pointer or reference to a class, or void*");
                return;
            }
        };
        if let Some(c) = tc {
            if !self.classes[c].complete {
                self.error(line, "VIRT-18", "dynamic_cast to an incomplete class");
                return;
            }
        }
        if (oq.c && !tq.c) || (oq.v && !tq.v) {
            self.error(line, "VIRT-18", "dynamic_cast removes a cv-qualifier");
            return;
        }
        let Ty::Class(oc) = oq.t else { return };
        if self.class_uncertain(oc) || tc.is_some_and(|c| self.class_uncertain(c)) {
            return;
        }
        match tc {
            Some(c) if c == oc => {}
            Some(c) if self.is_base_of(c, oc) => {
                let (paths, open) = self.base_paths(c, oc);
                if paths > 1 {
                    self.error(line, "VIRT-18", "dynamic_cast to an ambiguous base");
                } else if !open {
                    self.error(line, "VIRT-18", "dynamic_cast to an inaccessible base");
                }
            }
            _ => {
                if !self.polymorphic(oc) {
                    self.error(line, "VIRT-18", "dynamic_cast operand of a class that is not polymorphic");
                }
            }
        }
    }

    /// The number of non-virtual paths from `derived` to `base`, and
    /// whether one of them goes only through public bases.
    pub fn base_paths(&self, base: usize, derived: usize) -> (u32, bool) {
        let mut n = 0;
        let mut open = false;
        let mut virt_seen = false;
        for b in &self.classes[derived].bases {
            let Some(bc) = b.class else { continue };
            let (k, o) = if bc == base { (1, true) } else { self.base_paths(base, bc) };
            if k > 0 {
                if b.virt && bc == base {
                    if virt_seen {
                        continue;
                    }
                    virt_seen = true;
                }
                n += k;
                open |= o && b.access == 0;
            }
        }
        (n, open)
    }

    fn polymorphic(&self, c: usize) -> bool {
        self.classes[c].has_virtual || self.classes[c].bases.iter().any(|b| b.class.is_some_and(|bc| self.polymorphic(bc)))
    }

    /// CTOR-9: copy-initialization does not use an explicit constructor.
    fn check_explicit_copy(&mut self, x: &Expr, line: u32) {
        if let EK::Construct(cid, _) = &x.k {
            if self.func(*cid).is_some_and(|f| f.explicit) {
                self.error(line, "CTOR-9", "copy-initialization uses an explicit constructor");
            }
        }
    }

    /// The default constructor of a class, when exactly one constructor
    /// takes no arguments and none is a template.
    pub fn default_ctor(&self, c: usize) -> Option<DeclId> {
        let ctors = &self.classes[c].ctors;
        // A constructor template whose parameters all need arguments is no
        // candidate for a call with none.
        let needs_args = |i: DeclId| -> bool {
            let Some(f) = self.func(i) else { return false };
            let pack = self.decl_tid.get(&i).is_some_and(|&t| self.templs[t].fn_pack.is_some()) as usize;
            // A pack makes the type variadic; a C ellipsis alone needs no
            // argument.
            (pack == 1 || !f.fty.variadic) && f.params.len() > f.defaults + pack
        };
        if ctors.iter().any(|&i| self.func(i).is_none_or(|f| f.template && !needs_args(i))) {
            return None;
        }
        let ctors: Vec<DeclId> = ctors.iter().copied().filter(|&i| self.func(i).is_some_and(|f| !f.template)).collect();
        let ctors = &ctors;
        let v: Vec<DeclId> = ctors.iter().copied().filter(|&i| self.func(i).is_some_and(|f| f.params.len() <= f.defaults)).collect();
        if v.len() == 1 {
            Some(v[0])
        } else {
            None
        }
    }

    /// A constructor call with the given arguments. The front end picks a
    /// constructor only when every candidate of the right arity has scalar
    /// parameters and exactly one accepts the arguments; otherwise the
    /// call stays unknown.
    pub fn ctor_call(&mut self, c: usize, args: &[Expr], ty: &QTy, line: u32) -> Expr {
        self.ctor_call_mode(c, args, ty, false, false, false, line)
    }

    /// A constructor call. `list` and `copy` give the form of the
    /// initialization (OVL-14).
    #[allow(clippy::too_many_arguments)]
    pub fn ctor_call_mode(&mut self, c: usize, args: &[Expr], ty: &QTy, list: bool, copy: bool, copy_list: bool, line: u32) -> Expr {
        let unknown = Expr::new(EK::Unknown("constructor call".into()), ty.clone(), Cat::PR, line);
        if !self.class_uncertain(c) && !self.in_template() && self.ctor_no_match(c, args) {
            self.error(line, "CTOR-25", format!("no constructor of '{}' accepts the arguments", self.classes[c].name));
            return unknown;
        }
        let mut chosen_for_build = crate::ovl::CtorRes::Unknown;
        self.ctor_two_user = false;
        let rr = self.ctor_resolve(c, args, list, copy, copy_list);
        match rr {
            crate::ovl::CtorRes::Ambiguous => {
                // CTOR-25, OVL-36.
                self.error(line, "CTOR-25", format!("ambiguous constructor call: no constructor of '{}' is better than each of the others", self.classes[c].name));
                return unknown;
            }
            crate::ovl::CtorRes::NoViable => {
                if std::mem::take(&mut self.ctor_two_user) {
                    // CTOR-22, ICS-9: one converting constructor at most.
                    self.error(line, "CTOR-22", format!("the conversion to '{}' needs two user-defined conversions", self.classes[c].name));
                    return unknown;
                }
                // CTOR-25, OVL-35.
                self.error(line, "CTOR-25", format!("no constructor of '{}' accepts the arguments", self.classes[c].name));
                return unknown;
            }
            crate::ovl::CtorRes::Chosen(cid, seqs) => {
                // OVL-37: the checks on the chosen constructor.
                let mark = self.d.list.len();
                self.check_chosen_seqs(&seqs, cid, line);
                let f = self.func(cid).unwrap().clone();
                let is_list_ctor = list && f.params.first().and_then(|&p| self.var(p)).is_some_and(|v| matches!(nonref(&v.ty).t, Ty::Class(lc) if self.init_list_elem(lc).is_some()));
                let clean = self.d.list.len() == mark;
                if f.deleted {
                    self.error(line, "DEL-1", format!("call of deleted constructor of '{}'", self.classes[c].name));
                    return unknown;
                }
                if !self.in_template() && !self.accessible(cid, false) {
                    // ACC-2, DER-20: the chosen constructor is not
                    // accessible here.
                    let rule = if self.member_access.get(&cid) == Some(&1) { "ACC-4" } else { "ACC-2" };
                    self.error(line, rule, format!("the chosen constructor of '{}' is not accessible here", self.classes[c].name));
                    return unknown;
                }
                if copy_list && f.explicit {
                    // ILC-8: the best match is explicit.
                    self.error(line, "ILC-8", format!("copy-list-initialization of '{}' chooses an explicit constructor", self.classes[c].name));
                    return unknown;
                }
                if list {
                    // ILC-4, LIST-2: no narrowing conversion of an element
                    // to its parameter or element type.
                    let elem = f.params.first().and_then(|&p| self.var(p)).and_then(|v| match nonref(&v.ty).t {
                        Ty::Class(lc) if f.params.len() - f.defaults.min(f.params.len()) <= 1 => self.init_list_elem(lc),
                        _ => None,
                    });
                    for (i, a) in args.iter().enumerate() {
                        let pt = match &elem {
                            Some(e) => e.clone(),
                            None => match f.params.get(i).and_then(|&p| self.var(p)) {
                                Some(v) => v.ty.clone(),
                                None => continue,
                            },
                        };
                        let pt = nonref(&pt);
                        if !matches!(a.k, EK::InitList(_)) && !matches!(pt.t, Ty::Class(_) | Ty::Unknown | Ty::Dep(_)) && !matches!(nonref(&a.ty).t, Ty::Class(_)) && self.is_narrowing(a, &pt.t) {
                            self.error(line, "ILC-4", format!("narrowing conversion of an element from {} to {}", show(&nonref(&a.ty)), show(&pt)));
                            return unknown;
                        }
                    }
                }
                if clean && is_list_ctor {
                    // ILC-3: the initializer-list constructor wins. LSUP-1:
                    // the list object comes from the constructor of two
                    // parameters, with a pointer to an array of the
                    // elements and their number.
                    if let Some(x) = self.init_list_call(cid, &f, args, ty, line) {
                        if self.in_unevaluated == 0 {
                            self.use_fn(cid);
                        }
                        return x;
                    }
                    return Expr::new(EK::Unknown("initializer-list constructor call".into()), ty.clone(), Cat::PR, line);
                }
                if clean {
                    chosen_for_build = crate::ovl::CtorRes::Chosen(cid, Vec::new());
                }
            }
            crate::ovl::CtorRes::Unknown => {}
        }
        if let crate::ovl::CtorRes::Chosen(cid, _) = &chosen_for_build {
            // The call for the interpreter, with each argument converted to
            // its parameter. A conversion that raises a diagnostic leaves
            // the call to the older code below.
            let cid = *cid;
            let f = self.func(cid).unwrap().clone();
            if args.len() <= f.fty.params.len() && !f.fty.variadic && !args.iter().any(|a| matches!(a.k, EK::Unknown(_)) || a.ty.is_unknown() && !matches!(a.k, EK::InitList(_))) {
                let mark = self.d.list.len();
                let conv = self.convert_args(&f.fty, args.to_vec(), line, Some(cid));
                if self.d.list.len() == mark && !conv.iter().any(|x| matches!(x.k, EK::Unknown(_))) {
                    if self.in_unevaluated == 0 {
                        self.use_fn(cid);
                    }
                    let x = Expr::new(EK::Construct(cid, conv), ty.unqual(), Cat::PR, line);
                    return self.check_consteval_ctor(x, cid, line);
                }
                self.d.list.truncate(mark);
            }
        }
        if args.len() == 1 && !list && !self.class_uncertain(c) && !self.in_template() && matches!(chosen_for_build, crate::ovl::CtorRes::Unknown) {
            if let Ty::Class(sc) = nonref(&args[0].ty).t {
                if sc != c && !self.is_base_of(c, sc) && self.class_converts(sc) != Some(false) {
                    // OVL-14, CVF-1: a conversion function of the argument
                    // gives the class, for the copy or move constructor.
                    let mark = self.d.list.len();
                    if let Ok(x) = self.implicit(args[0].clone(), &ty.unqual(), !copy) {
                        if self.d.list.len() == mark && x.cat == Cat::PR && matches!(x.k, EK::Call(..)) {
                            let mut x = x;
                            x.ty = ty.unqual();
                            return x;
                        }
                    }
                    self.d.list.truncate(mark);
                }
            }
        }
        if self.class_uncertain(c) || self.in_template() || args.iter().any(|a| a.ty.is_unknown() || matches!(a.k, EK::InitList(_))) {
            return unknown;
        }
        let mut viable = Vec::new();
        for cid in self.classes[c].ctors.clone() {
            if args.len() == 1 && self.inherited_copy_shape(cid, c) {
                continue;
            }
            let Some(f) = self.func(cid).cloned() else { return unknown };
            if f.template {
                return unknown;
            }
            let np = f.params.len();
            if args.len() > np || args.len() + f.defaults < np {
                continue;
            }
            let mut conv = Vec::new();
            for (i, a) in args.iter().enumerate() {
                let pt = self.var(f.params[i]).map(|v| v.ty.clone()).unwrap_or(QTy::unknown());
                if pt.is_unknown() || pt.t.is_ref() || matches!(pt.t, Ty::Class(_) | Ty::Array(..)) || matches!(nonref(&a.ty).t, Ty::Class(_)) {
                    return unknown;
                }
                match self.implicit(a.clone(), &pt.unqual(), false) {
                    Ok(x) => conv.push(x),
                    Err(_) => break,
                }
            }
            if conv.len() == args.len() {
                if f.deleted {
                    return unknown;
                }
                viable.push((cid, conv));
            }
        }
        if viable.len() > 1 {
            let first = self.func(viable[0].0).map(|f| f.fty.params.clone());
            if viable.iter().all(|(v, _)| self.inherited_from.contains_key(v) && self.func(*v).map(|f| f.fty.params.clone()) == first) {
                // DER-15: two bases give the chosen signature.
                self.error(line, "DER-15", format!("the constructor of '{}' that the call selects is inherited from two bases", self.classes[c].name));
            }
        }
        if viable.len() != 1 {
            return unknown;
        }
        let (cid, conv) = viable.pop().unwrap();
        if !self.in_template() && !self.accessible(cid, false) {
            // ACC-2, DER-20: an inherited constructor keeps its access.
            let rule = if self.member_access.get(&cid) == Some(&1) { "ACC-4" } else { "ACC-2" };
            self.error(line, rule, format!("the chosen constructor of '{}' is not accessible here", self.classes[c].name));
            return unknown;
        }
        let x = Expr::new(EK::Construct(cid, conv), ty.unqual(), Cat::PR, line);
        self.check_consteval_ctor(x, cid, line)
    }

    /// CEV-1, CE-8: the call of a `consteval` constructor outside an
    /// immediate context is a constant expression.
    fn check_consteval_ctor(&mut self, x: Expr, cid: DeclId, line: u32) -> Expr {
        let in_lambda = self.fns.iter().any(|f| f.is_lambda);
        if self.func(cid).is_some_and(|f| f.consteval) && !self.in_immediate_context() && self.in_unevaluated == 0 && !self.in_template() && !in_lambda && self.inst_depth == 0 {
            match self.const_check(&x) {
                Ok(()) => {}
                Err(crate::consteval::CeErr::NotConst(r, m)) => self.error_because(line, "CEV-1", r, format!("call of consteval constructor is not a constant expression ({}: {})", r, m)),
                Err(crate::consteval::CeErr::Unsupported(_)) => {}
            }
        }
        x
    }

    /// LSUP-1, ILC-3: the call of initializer-list constructor `cid` with
    /// the elements `items`: the array of the elements, the list object
    /// that refers to it, and the call. None when a step is not modeled.
    fn init_list_call(&mut self, cid: DeclId, f: &FuncInfo, items: &[Expr], ty: &QTy, line: u32) -> Option<Expr> {
        if f.params.len() > 1 + f.defaults || f.fty.variadic || self.in_template() {
            return None;
        }
        let pt = nonref(&f.fty.params[0]);
        let Ty::Class(lc) = pt.t else { return None };
        let list = self.make_init_list(lc, items, line)?;
        Some(Expr::new(EK::Construct(cid, vec![list]), ty.unqual(), Cat::PR, line))
    }

    /// LSUP-1, ILC-1: the object of initializer-list class `lc` for the
    /// elements `items`: an array of the converted elements, and the
    /// constructor of two parameters with a pointer to it and the number.
    pub fn make_init_list(&mut self, lc: usize, items: &[Expr], line: u32) -> Option<Expr> {
        if self.in_template() {
            return None;
        }
        self.require_class(lc, line);
        let el = self.init_list_elem(lc)?;
        if !self.init_list_shape_ok() {
            // LSUP-1: the object of the braced list needs the shape.
            self.error(line, "LSUP-1", "a braced list needs std::initializer_list, which is not declared as a class template of the needed shape");
            return None;
        }
        let mk = self.classes[lc].ctors.iter().copied().find(|&k| self.func(k).is_some_and(|g| g.fty.params.len() == 2 && !g.template && matches!(g.fty.params[0].t, Ty::Ptr(_)) && g.fty.params[1].t.is_integral()))?;
        let size_ty = self.func(mk)?.fty.params[1].clone();
        let mark = self.d.list.len();
        let mut conv = Vec::new();
        for a in items {
            if matches!(a.k, EK::Unknown(_)) {
                return None;
            }
            let form = if matches!(a.k, EK::InitList(_)) { InitForm::CopyList } else { InitForm::Copy };
            conv.push(self.convert_init(a.clone(), &el.unqual(), form, line));
        }
        if self.d.list.len() != mark || conv.iter().any(|x| matches!(x.k, EK::Unknown(_))) {
            self.d.list.truncate(mark);
            return None;
        }
        let n = conv.len();
        let cel = QTy { t: el.t.clone(), c: true, v: el.v };
        let first = if n == 0 {
            Expr::new(EK::NullPtr, QTy::new(Ty::Ptr(Box::new(cel))), Cat::PR, line)
        } else {
            let arr = Expr::new(EK::InitList(conv), QTy::new(Ty::Array(Box::new(cel.clone()), Some(n as u64))), Cat::PR, line);
            Expr::new(EK::Conv(Conv::ArrayDecay, Box::new(arr)), QTy::new(Ty::Ptr(Box::new(cel))), Cat::PR, line)
        };
        let count = Expr::new(EK::Int(n as i128), size_ty.unqual(), Cat::PR, line);
        Some(Expr::new(EK::Construct(mk, vec![first, count]), QTy::new(Ty::Class(lc)), Cat::PR, line))
    }

    pub fn is_simple_aggregate(&self, c: usize) -> bool {
        let ci = &self.classes[c];
        ci.complete && !ci.opaque && !ci.template && ci.ctors.is_empty() && ci.bases.is_empty() && !ci.has_virtual
    }

    /// Placeholder type deduction (AUTO-1 to AUTO-11).
    /// LSUP-1: `std::initializer_list` is a class template with one type
    /// parameter, two data members, and a constructor of two parameters.
    pub fn init_list_shape_ok(&self) -> bool {
        let Some(ns) = self.lookup_in_scope(self.global, "std", false) else { return false };
        let DK::Namespace(s) = self.decls[ns[0]].k else { return false };
        let Some(ids) = self.lookup_in_scope(s, "initializer_list", false) else { return false };
        let Some(tid) = ids.iter().find_map(|&i| self.tid_of(i)) else { return false };
        if self.templs[tid].kind != crate::templ::TKind::Class || self.templs[tid].params.len() != 1 {
            return false;
        }
        let pc = match self.decls[self.templs[tid].decl].k {
            DK::Template(TemplKind::Class(pc)) if pc != usize::MAX => pc,
            _ => return false,
        };
        let ci = &self.classes[pc];
        // The pointer, then the count (Q-0431).
        let ftys: Vec<&QTy> = ci.fields.iter().filter_map(|&f| match &self.decls[f].k {
            DK::Field(fi) => Some(&fi.ty),
            _ => None,
        }).collect();
        if ftys.len() != 2 || !matches!(ftys[0].t, Ty::Ptr(_)) || !ftys[1].t.is_integral() {
            return false;
        }
        ci.ctors.iter().any(|&c| self.func(c).is_some_and(|f| f.fty.params.len() == 2))
    }

    pub fn deduce_auto(&mut self, ds: &DeclSpec, d: &Declarator, _ty: &QTy, e: &Expr, form: &InitForm, line: u32) -> Option<QTy> {
        let decltype_auto = ds.auto == Some(true);
        let mut init = e.clone();
        if d.ops.iter().any(|o| matches!(o, DOp::Array(_))) && !self.in_template() {
            // AUTO-8: no array of a placeholder type.
            self.error(line, "AUTO-8", "an array of a placeholder type");
            return None;
        }
        match form {
            InitForm::CopyList => {
                if decltype_auto {
                    self.error(line, "AUTO-11", "decltype(auto) with '= { ... }'");
                    return None;
                }
                // AUTO-7: std::initializer_list, a library type.
                if let EK::InitList(v) = &e.k {
                    let elem = |x: &Expr| -> Option<Ty> {
                        let t = nonref(&x.ty);
                        if t.is_unknown() || matches!(x.k, EK::InitList(_)) {
                            return None;
                        }
                        Some(match t.t {
                            Ty::Array(e, _) => Ty::Ptr(e),
                            Ty::Func(f) => Ty::Ptr(Box::new(QTy::new(Ty::Func(f)))),
                            t => t,
                        })
                    };
                    let types: Vec<Option<Ty>> = v.iter().map(elem).collect();
                    if v.is_empty() {
                        self.error(line, "AUTO-21", "auto deduced from an empty braced list");
                    } else if !self.in_template() && types.iter().all(|t| t.is_some()) && types.windows(2).any(|w| w[0] != w[1]) {
                        // AUTO-21: the elements deduce one type.
                        self.error(line, "AUTO-21", "the elements of the braced list have different types");
                    } else if !self.in_template() && !self.init_list_shape_ok() {
                        // LSUP-1: the program declares std::initializer_list
                        // with its shape first.
                        self.error(line, "LSUP-1", "a braced list needs std::initializer_list, which is not declared as a class template of the needed shape");
                    } else if !self.in_template() && d.ops.is_empty() && types.iter().all(|t| t.is_some()) {
                        // AUTO-7: the type is std::initializer_list<E>, with
                        // E the one type that the elements deduce.
                        let el = QTy::new(types[0].clone().unwrap());
                        let lc = self.init_list_class(el, line);
                        return Some(QTy::new(Ty::Class(lc)));
                    }
                }
                return Some(QTy::unknown());
            }
            InitForm::DirectList | InitForm::Paren(_) => {
                let EK::InitList(v) = &e.k else { return None };
                if v.len() != 1 {
                    self.error(line, if decltype_auto { "AUTO-11" } else { "AUTO-20" }, "placeholder initializer must contain exactly one expression");
                    return None;
                }
                if matches!(v[0].k, EK::InitList(_)) {
                    self.error(line, "AUTO-20", "braced list in parentheses for a placeholder");
                    return None;
                }
                init = v[0].clone();
            }
            _ => {}
        }
        if matches!(init.k, EK::TemplId(None, ..)) {
            // DED-2: a template-id that names one specialization.
            init = self.resolve_templ_id(init, line);
        }
        if self.overload_set_no_target(&init, line) {
            // OVL-20, AUTO-8: a placeholder gives no target type to an
            // overload set or a function template name.
            return None;
        }
        if let EK::Unary(UOp::AddrOf, x) = &init.k {
            if let EK::TemplId(None, fs, raw) = &x.k {
                // OVL-20 (Q-0405): two specializations that explicit
                // template arguments give are not ordered.
                let ts: Vec<DeclId> = fs.iter().copied().filter(|&f| self.is_func_templ(f)).collect();
                // OVL-20: with no target type, a parameter that the
                // explicit arguments leave has nothing to deduce from.
                if !ts.is_empty() && !self.in_template() && ts.len() == fs.len() {
                    let all_short = ts.iter().all(|t| {
                        let Some(&tid) = self.decl_tid.get(t) else { return false };
                        let ps = &self.templs[tid].params;
                        raw.len() < ps.len() && !raw.iter().any(|r| matches!(r, crate::templ::RawArg::Expand(_))) && ps[raw.len()..].iter().any(|p| !p.pack && p.default.is_none())
                    });
                    if all_short {
                        self.error(line, "OVL-20", format!("the address of '{}' leaves a template parameter that no target type deduces", self.decls[fs[0]].name));
                        return None;
                    }
                }
                if ts.len() > 1 && !self.in_template() && ts.len() == fs.len() {
                    let raw = raw.clone();
                    if let Some(v) = self.templ_id_specs(&ts, &raw, line) {
                        if v.len() > 1 {
                            self.error(line, "OVL-20", format!("the address of '{}' with explicit template arguments names more than one specialization and has no target type", self.decls[fs[0]].name));
                            return None;
                        }
                    }
                }
            }
        }
        if matches!(init.k, EK::Overloads(_)) {
            self.error(line, "AUTO-8", "auto initialized from an overloaded function name");
            return None;
        }
        if let Some((fs, true)) = Self::overload_set_of(&init) {
            // OVL-20 step 2: constraints remove functions (Q-0142).
            if !self.in_template() && fs.iter().any(|f| self.fn_cons.contains_key(f)) {
                if let Some(left) = self.constraint_reduced(&fs) {
                    if let [f] = left[..] {
                        if let Some(fi) = self.func(f).cloned() {
                            if !fi.template && !fi.explicit_obj {
                                let ft = QTy::new(Ty::Func(Box::new(fi.fty.clone())));
                                let t = match fi.class {
                                    Some(c) if !fi.is_static => Ty::MemPtr(c, Box::new(ft)),
                                    _ => Ty::Ptr(Box::new(ft)),
                                };
                                return Some(QTy::new(t));
                            }
                        }
                    }
                }
            }
            if !self.in_template() && fs.iter().all(|&f| self.func(f).is_some_and(|f| !f.constrained && f.class.is_none_or(|c| !self.class_uncertain(c)))) {
                // OVL-20: with no target type, the set must have one
                // function that is not a template.
                self.error(line, "OVL-20", format!("the address of the overload set '{}' has no target type", self.decls[fs[0]].name));
                return None;
            }
            return Some(QTy::unknown());
        }
        if init.ty.is_unknown() {
            return Some(QTy::unknown());
        }
        if init.ty.t == Ty::Void {
            self.error(line, "AUTO-8", "auto initialized from a void expression");
            return None;
        }
        if decltype_auto {
            if !d.ops.is_empty() {
                self.error(line, "AUTO-11", "decltype(auto) combined with a declarator operator");
                return None;
            }
            if matches!(init.k, EK::Func(_)) && !init.paren {
                self.error(line, "AUTO-11", "decltype(auto) initialized from a function name");
                return None;
            }
            let t = self.decltype_of(&init, init.paren);
            if let Ty::Array(..) = t.t {
                if matches!(init.k, EK::Var(_)) {
                    // AUTO-15
                    self.error(line, "AUTO-15", "decltype(auto) deduces an array type");
                    return None;
                }
            }
            return Some(t);
        }
        let cvq = (ds.c > 0, ds.v > 0);
        let et = nonref(&init.ty);
        let ops = &d.ops;
        let rebuild = |p: &mut P, base: QTy| -> QTy {
            let mut b = base;
            b.c |= cvq.0;
            b.v |= cvq.1;
            p.apply_ops(b, ops, line)
        };
        if ops.is_empty() {
            // AUTO-1, AUTO-2: drop reference and top-level cv, decay.
            let t = match &et.t {
                Ty::Array(el, _) => QTy::new(Ty::Ptr(el.clone())),
                Ty::Func(_) => QTy::new(Ty::Ptr(Box::new(et.clone()))),
                _ => et.unqual(),
            };
            return Some(rebuild(self, t));
        }
        match &ops[..] {
            [DOp::LRef] => {
                // AUTO-3
                let mut t = et.clone();
                t.c |= cvq.0;
                if init.cat != Cat::L && !t.c {
                    self.error(line, "AUTO-18", "auto& bound to an rvalue");
                    return None;
                }
                Some(QTy::new(Ty::LRef(Box::new(t))))
            }
            [DOp::RRef] => {
                // AUTO-4: forwarding reference
                if cvq.0 || cvq.1 {
                    let mut t = et.clone();
                    t.c |= cvq.0;
                    if init.cat == Cat::L {
                        self.error(line, "REF-10", "const auto&& bound to an lvalue");
                        return None;
                    }
                    return Some(QTy::new(Ty::RRef(Box::new(t))));
                }
                if init.cat == Cat::L {
                    Some(QTy::new(Ty::LRef(Box::new(et.clone()))))
                } else {
                    Some(QTy::new(Ty::RRef(Box::new(et.clone()))))
                }
            }
            [DOp::Ptr(c, v)] => {
                // AUTO-5: auto* needs a pointer
                let pt = match &et.t {
                    Ty::Ptr(p) => (**p).clone(),
                    Ty::Array(el, _) => (**el).clone(),
                    Ty::Func(_) => et.clone(),
                    Ty::NullPtr | _ => {
                        self.error(line, "AUTO-19", "auto* initialized from a value that is not a pointer");
                        return None;
                    }
                };
                let mut pt = pt;
                pt.c |= cvq.0;
                pt.v |= cvq.1;
                Some(QTy { t: Ty::Ptr(Box::new(pt)), c: *c, v: *v })
            }
            _ => {
                if ops.iter().any(|o| matches!(o, DOp::Array(_))) {
                    self.error(line, "AUTO-8", "array of auto");
                    return None;
                }
                Some(QTy::unknown())
            }
        }
    }
}

/// A braced-init-list as written, before it initializes anything.
pub fn is_raw_list(e: &Expr) -> bool {
    matches!(e.k, EK::InitList(_)) && e.ty.is_unknown()
}

pub fn ret_cat(r: &QTy) -> (QTy, Cat) {
    match &r.t {
        Ty::LRef(i) => ((**i).clone(), Cat::L),
        Ty::RRef(i) => ((**i).clone(), if matches!(i.t, Ty::Func(_)) { Cat::L } else { Cat::X }),
        _ => (if matches!(r.t, Ty::Class(_)) { r.clone() } else { r.unqual() }, Cat::PR),
    }
}

impl P {
    /// LIST-5: a designated list of a class with no bases, as a list with
    /// one element per member. A member that the list does not name gets
    /// `MemberDefault`. For a union, the list ends at the named member.
    fn designated_list(&mut self, c: usize, v: &[Expr], ty: &QTy, line: u32) -> Option<Expr> {
        if !self.classes[c].bases.is_empty() || self.in_template() {
            return None;
        }
        let fields: Vec<DeclId> = self.classes[c].fields.iter().copied().filter(|&f| !(matches!(&self.decls[f].k, DK::Field(fi) if fi.bits.is_some()) && self.decls[f].name.is_empty())).collect();
        let mut vals: Vec<Option<Expr>> = vec![None; fields.len()];
        let mut ready = vec![false; fields.len()];
        for x in v {
            let EK::Unknown(s) = &x.k else { return None };
            let mut w = s.split_whitespace().skip(1);
            let name = w.next()?;
            let id: usize = w.next()?.parse().ok()?;
            let i = match fields.iter().position(|&f| self.decls[f].name == name) {
                Some(i) => i,
                None => {
                    // UNI-7: a member of an anonymous union initializes the
                    // union with that member.
                    let (i, u, j) = fields.iter().enumerate().find_map(|(i, &f)| match &self.decls[f].k {
                        DK::Field(fi) if self.decls[f].name.is_empty() => match fi.ty.t {
                            Ty::Class(u) if self.classes[u].key == 2 => self.classes[u].fields.iter().position(|&g| self.decls[g].name == name).map(|j| (i, u, j)),
                            _ => None,
                        },
                        _ => None,
                    })?;
                    if vals[i].is_some() {
                        return None;
                    }
                    let g = self.classes[u].fields[j];
                    let DK::Field(gi) = &self.decls[g].k else { return None };
                    let gty = gi.ty.clone();
                    let xv = self.designated.get(id)?.clone();
                    let xl = xv.line;
                    let mark = self.d.list.len();
                    let y = self.convert_init(xv, &gty, InitForm::CopyList, xl);
                    if self.d.list.len() != mark {
                        return None;
                    }
                    let mut items: Vec<Expr> = Vec::new();
                    for k in 0..j {
                        let DK::Field(hi) = &self.decls[self.classes[u].fields[k]].k else { return None };
                        items.push(Expr::new(EK::MemberDefault, hi.ty.clone(), Cat::PR, line));
                    }
                    items.push(y);
                    vals[i] = Some(Expr::new(EK::InitList(items), QTy::new(Ty::Class(u)), Cat::PR, line));
                    ready[i] = true;
                    continue;
                }
            };
            if vals[i].is_some() {
                return None;
            }
            if self.classes[c].key == 2 && vals.iter().any(|v| v.is_some()) {
                // UNI-8: a designated list of a union names one member.
                self.error(line, "UNI-8", "a designated list of a union names two members");
                return Some(Expr::unknown(line, "bad union init"));
            }
            vals[i] = Some(self.designated.get(id)?.clone());
        }
        let union = self.classes[c].key == 2;
        if !union {
            // LIST-16: a member that no designator names and that has no
            // default member initializer cannot use an explicit default
            // constructor.
            for (i, &f) in fields.iter().enumerate() {
                if vals[i].is_some() {
                    continue;
                }
                if let DK::Field(fi) = &self.decls[f].k {
                    if let (None, Ty::Class(mc)) = (&fi.init, &fi.ty.t) {
                        let mc = *mc;
                        if self.classes[mc].complete && !self.class_uncertain(mc) && self.default_ctor(mc).and_then(|d| self.func(d)).is_some_and(|df| df.explicit && !df.deleted) {
                            self.error(line, "LIST-16", format!("the member '{}' has no value in the list, and the default constructor of '{}' is explicit", self.decls[f].name, self.classes[mc].name));
                            return Some(Expr::unknown(line, "bad aggregate init"));
                        }
                    }
                }
            }
        }
        let last = if union { vals.iter().position(|x| x.is_some())? + 1 } else { fields.len() };
        let mut out = Vec::new();
        for (i, val) in vals.into_iter().enumerate().take(last) {
            let DK::Field(fi) = &self.decls[fields[i]].k else { return None };
            let fty = fi.ty.clone();
            let is_ref = fty.t.is_ref();
            match val {
                Some(x) if ready[i] => out.push(x),
                Some(x) => {
                    let xl = x.line;
                    let mark = self.d.list.len();
                    let y = self.convert_init(x, &fty, InitForm::CopyList, xl);
                    if self.d.list.len() != mark {
                        return None;
                    }
                    out.push(y);
                }
                None if is_ref && fi.init.is_none() => {
                    self.error(line, "LIST-12", "reference member without an initializer in the list");
                    return None;
                }
                None => out.push(Expr::new(EK::MemberDefault, fty, Cat::PR, line)),
            }
        }
        Some(Expr::new(EK::InitList(out), ty.clone(), Cat::PR, line))
    }
}

