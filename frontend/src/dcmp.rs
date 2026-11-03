// Defaulted comparison operators (specification/classes/comparisons.md).
//
// The body of a defaulted `==`, `<=>`, or secondary operator is made at
// its first use, from the operators of the bases and members, so that
// overload resolution and the interpreter treat it as an ordinary
// function.

use crate::ast::*;
use crate::parser::*;
use crate::types::*;
use std::rc::Rc;

impl P {
    /// The parts that a defaulted comparison compares, in order: the
    /// bases, then the members, with array members element by element.
    fn cmp_parts(&mut self, c: usize, x: &Expr, y: &Expr, line: u32) -> Option<Vec<(Expr, Expr)>> {
        let mut out = Vec::new();
        let bases = self.classes[c].bases.clone();
        for b in &bases {
            let bc = b.class?;
            let bt = QTy { t: Ty::Class(bc), c: true, v: false };
            out.push((Expr::new(EK::Conv(Conv::NoOp, Box::new(x.clone())), bt.clone(), Cat::L, line), Expr::new(EK::Conv(Conv::NoOp, Box::new(y.clone())), bt, Cat::L, line)));
        }
        let fields = self.classes[c].fields.clone();
        for f in fields {
            let DK::Field(fi) = self.decls[f].k.clone() else { continue };
            if fi.bits.is_some() && self.decls[f].name.is_empty() {
                continue;
            }
            let mut ft = fi.ty.clone();
            if !ft.t.is_ref() {
                ft.c = true;
            }
            let mx = Expr::new(EK::Member(Box::new(x.clone()), f), ft.clone(), Cat::L, line);
            let my = Expr::new(EK::Member(Box::new(y.clone()), f), ft.clone(), Cat::L, line);
            if let Ty::Array(_, Some(n)) = &ft.t {
                if *n > 64 {
                    return None;
                }
                for i in 0..*n {
                    let ix = Expr::new(EK::Int(i as i128), QTy::new(Ty::Long), Cat::PR, line);
                    let ex = self.build_subscript(mx.clone(), ix.clone(), line);
                    let ey = self.build_subscript(my.clone(), ix, line);
                    out.push((ex, ey));
                }
                continue;
            }
            out.push((mx, my));
        }
        Some(out)
    }

    /// The static member `name` of the comparison category class `cc`.
    fn category_value(&mut self, cc: usize, name: &str, line: u32) -> Option<Expr> {
        let ids = self.lookup_member(cc, name)?;
        let id = *ids.iter().find(|&&i| self.var(i).is_some())?;
        let vt = self.var(id)?.ty.clone();
        Some(Expr::new(EK::Var(id), vt, Cat::L, line))
    }

    /// DCMP: make the body of the defaulted comparison `fid`. False when
    /// the front end cannot.
    pub fn synth_defaulted_cmp(&mut self, fid: DeclId) -> bool {
        if let Some(&ok) = self.dcmp_done.get(&fid) {
            return ok;
        }
        self.dcmp_done.insert(fid, false);
        let ok = self.synth_cmp_inner(fid).is_some();
        self.dcmp_done.insert(fid, ok);
        ok
    }

    fn synth_cmp_inner(&mut self, fid: DeclId) -> Option<()> {
        let f = self.func(fid)?.clone();
        if !f.defaulted || f.body.is_some() || self.in_template() {
            return None;
        }
        let name = self.decls[fid].name.clone();
        let line = self.decls[fid].line;
        let (c, x, y) = if let Some(c) = f.class {
            let ct = QTy { t: Ty::Class(c), c: true, v: false };
            let this = Expr::new(EK::This, QTy::new(Ty::Ptr(Box::new(ct.clone()))), Cat::PR, line);
            let x = Expr::new(EK::Unary(UOp::Deref, Box::new(this)), ct.clone(), Cat::L, line);
            let p0 = *f.params.first()?;
            let pt = nonref(&self.var(p0)?.ty);
            let y = Expr::new(EK::Var(p0), pt, Cat::L, line);
            (c, x, y)
        } else {
            if f.params.len() != 2 {
                return None;
            }
            let t0 = nonref(&self.var(f.params[0])?.ty);
            let t1 = nonref(&self.var(f.params[1])?.ty);
            let Ty::Class(c) = t0.t else { return None };
            (c, Expr::new(EK::Var(f.params[0]), t0, Cat::L, line), Expr::new(EK::Var(f.params[1]), t1, Cat::L, line))
        };
        if !self.classes[c].complete || self.class_uncertain(c) {
            return None;
        }
        let rule = if name == "operator==" { "DCMP-9" } else { "DCMP-11" };
        if name == "operator==" {
            // DCMP-2: a union, an anonymous union member, or a reference
            // member deletes `==`.
            let ci = &self.classes[c];
            let refm = ci.fields.iter().any(|&fd| matches!(&self.decls[fd].k, DK::Field(fi) if fi.ty.t.is_ref()));
            if ci.key == 2 || !ci.anon_unions.is_empty() || refm {
                self.dcmp_deleted.insert(fid, rule);
                return None;
            }
        }
        if name == "operator<=>" && f.ret_auto.is_none() && !f.fty.ret.is_unknown() {
            // DCMP-5: a written return type must be a comparison category.
            let cats = ["strong_ordering", "weak_ordering", "partial_ordering"].map(|n| self.std_class(n));
            if !matches!(f.fty.ret.t, Ty::Class(k) if cats.contains(&Some(k))) {
                self.dcmp_deleted.insert(fid, rule);
                return None;
            }
        }
        let (me, mu, mark) = (self.d.muted_errors, self.d.muted_unsup, self.d.list.len());
        self.d.muted += 1;
        let r = self.cmp_body(&name, c, &f, x, y, line);
        self.d.muted -= 1;
        let bad = self.d.muted_errors != me || self.d.muted_unsup != mu;
        let only_errors = self.d.muted_unsup == mu && self.d.muted_errors != me;
        self.d.muted_errors = me;
        self.d.muted_unsup = mu;
        self.d.list.truncate(mark);
        // With a written return type, `<=>` can use `==` and `<` of a part
        // instead, which the front end does not build.
        if only_errors && (name != "operator<=>" || f.ret_auto.is_some() || f.fty.ret.is_unknown()) {
            // DCMP-2, DCMP-5: a part has no usable operator.
            self.dcmp_deleted.insert(fid, rule);
        }
        let (e, ret) = r?;
        if bad {
            return None;
        }
        let mut unknown = false;
        walk_expr(&e, &mut |x| {
            if matches!(x.k, EK::Unknown(_)) {
                unknown = true;
            }
        });
        if unknown {
            return None;
        }
        let fm = self.func_mut(fid)?;
        fm.body = Some(Rc::new(Stmt::Block(vec![Stmt::Return(Some(e))])));
        if let Some(r) = ret {
            fm.fty.ret = r;
            fm.ret_auto = None;
        }
        fm.defined = true;
        Some(())
    }

    fn cmp_body(&mut self, name: &str, c: usize, f: &FuncInfo, x: Expr, y: Expr, line: u32) -> Option<(Expr, Option<QTy>)> {
        let int0 = Expr::new(EK::Int(0), QTy::new(Ty::Int), Cat::PR, line);
        match name {
            "operator==" => {
                let parts = self.cmp_parts(c, &x, &y, line)?;
                let mut e = Expr::new(EK::Int(1), QTy::bool_(), Cat::PR, line);
                for (a, b) in parts.into_iter().rev() {
                    let eq = self.build_binary(BOp::Eq, a, b, line);
                    let eq = self.contextual_bool(eq, "DCMP-4");
                    if matches!(e.k, EK::Int(1)) {
                        e = eq;
                    } else {
                        e = Expr::new(EK::Binary(BOp::LAnd, Box::new(eq), Box::new(e)), QTy::bool_(), Cat::PR, line);
                    }
                }
                Some((e, None))
            }
            "operator<=>" => {
                let parts = self.cmp_parts(c, &x, &y, line)?;
                let mut cs = Vec::new();
                for (a, b) in parts {
                    let e = self.build_binary(BOp::Spaceship, a, b, line);
                    if e.ty.is_unknown() || matches!(e.k, EK::Unknown(_)) {
                        return None;
                    }
                    cs.push(e);
                }
                // The return type: written, or the common comparison
                // category of the parts.
                let ret = if f.ret_auto.is_some() || f.fty.ret.is_unknown() {
                    let strong = self.std_class("strong_ordering");
                    let weak = self.std_class("weak_ordering");
                    let partial = self.std_class("partial_ordering");
                    let mut rank = 0; // 0 strong, 1 weak, 2 partial
                    for e in &cs {
                        let Ty::Class(k) = nonref(&e.ty).t else { return None };
                        if Some(k) == partial {
                            rank = rank.max(2);
                        } else if Some(k) == weak {
                            rank = rank.max(1);
                        } else if Some(k) != strong {
                            return None;
                        }
                    }
                    let k = match rank {
                        0 => strong?,
                        1 => weak?,
                        _ => partial?,
                    };
                    QTy::new(Ty::Class(k))
                } else {
                    f.fty.ret.clone()
                };
                let Ty::Class(rc) = ret.t else { return None };
                let fin = match self.category_value(rc, "equal", line) {
                    Some(e) => e,
                    None => self.category_value(rc, "equivalent", line)?,
                };
                let mut e = fin;
                for ck in cs.into_iter().rev() {
                    let ne = self.build_binary(BOp::Ne, ck.clone(), int0.clone(), line);
                    let ne = self.contextual_bool(ne, "DCMP-4");
                    let conv = if nonref(&ck.ty).t == ret.t { ck } else { self.convert_init(ck, &ret, crate::decl2::InitForm::Copy, line) };
                    e = Expr::new(EK::Cond(Box::new(ne), Box::new(conv), Box::new(e)), ret.clone(), Cat::PR, line);
                }
                Some((e, Some(ret)))
            }
            "operator!=" => {
                let eq = self.build_binary(BOp::Eq, x, y, line);
                let eq = self.contextual_bool(eq, "DCMP-4");
                Some((Expr::new(EK::Unary(UOp::Not, Box::new(eq)), QTy::bool_(), Cat::PR, line), None))
            }
            "operator<" | "operator>" | "operator<=" | "operator>=" => {
                let op = BOp::from_str(&name["operator".len()..])?;
                let c3 = self.build_binary(BOp::Spaceship, x, y, line);
                let e = self.build_binary(op, c3, int0, line);
                let e = self.contextual_bool(e, "DCMP-4");
                Some((e, None))
            }
            _ => None,
        }
    }
}

impl P {
    /// DCMP: a defaulted `<=>` declares a defaulted `==` with the same
    /// parameters, unless the class declares `==` itself. True when the
    /// class now has such an `==`, or needs none.
    pub fn implicit_eq(&mut self, c: usize) -> bool {
        if self.dcmp_eq.contains(&c) {
            return true;
        }
        self.dcmp_eq.insert(c);
        let sc = self.classes[c].scope;
        let members = self.scopes[sc].names.get("operator<=>").cloned().unwrap_or_default();
        let friends: Vec<DeclId> = self.class_friends.get(&c).cloned().unwrap_or_default().into_iter().filter(|&d| self.decls[d].name == "operator<=>").collect();
        let has_eq = self.scopes[sc].names.get("operator==").is_some_and(|v| !v.is_empty()) || self.class_friends.get(&c).is_some_and(|v| v.iter().any(|&d| self.decls[d].name == "operator=="));
        if has_eq {
            return true;
        }
        for &d in members.iter().chain(friends.iter()) {
            let Some(f) = self.func(d).cloned() else { continue };
            if !f.defaulted {
                continue;
            }
            let mut g = f.clone();
            g.fty.ret = QTy::bool_();
            g.ret_auto = None;
            g.body = None;
            g.first_attrs = vec![];
            let line = self.decls[d].line;
            if f.class.is_some() {
                let id = self.add_decl_in(sc, "operator==", DK::Func(g), line);
                self.member_access.insert(id, 0);
            } else {
                let s = self.decls[d].scope;
                let id = self.add_decl_in(s, "operator==", DK::Func(g), line);
                self.hide_decl(id);
                self.class_friends.entry(c).or_default().push(id);
                self.hidden_friends.insert(id, vec![c]);
            }
            return true;
        }
        true
    }
}
