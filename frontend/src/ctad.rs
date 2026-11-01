// Deduction of class template arguments
// (specification/templates/class-template-deduction.md).
//
// Each guide becomes a hidden function template whose result type is a
// specialization of the class template (CTAD-P1). Deduction and the
// choice among the guides then use the code for calls: `deduce_call`,
// substitution, `ics_select`, and partial ordering. The tie-breakers of
// CTAD-19 to CTAD-22 come after the usual ones.

use crate::ast::*;
use crate::cons::CNorm;
use crate::decl2::InitForm;
use crate::parser::*;
use crate::templ::*;
use crate::types::*;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GKind {
    /// From a constructor; true for a constructor template.
    Ctor(bool),
    /// No constructor (CTAD-6).
    Default,
    Copy,
    Aggregate,
    User,
}

#[derive(Clone, Debug)]
pub struct Guide {
    pub head: Vec<TParam>,
    pub depth: u32,
    pub params: Vec<QTy>,
    pub defaults: usize,
    pub variadic: bool,
    pub fn_pack: Option<usize>,
    pub result: QTy,
    pub explicit: bool,
    pub kind: GKind,
    /// Inherited from a base (CTAD-16).
    pub inherited: bool,
    /// The parameters of the class: no forwarding reference (CTAD-8).
    pub class_params: usize,
    pub cons: Option<Rc<CNorm>>,
    pub line: u32,
    /// The number of declarations when a user guide was declared
    /// (CTAD-11).
    pub seq: usize,
    /// A guide of an alias template: the alias, and the type that the
    /// guide's own result must equal (CTAD-28).
    pub alias: Option<(usize, QTy)>,
    /// The condition of a conditional `explicit` (CTAD-5).
    pub explicit_cond: Option<Source>,
}

impl P {
    /// The class template that a deduction guide at the current token
    /// names, with the offset of its name: `[explicit] Name ( ... ) ->`.
    pub fn at_guide(&self) -> Option<(usize, usize)> {
        let mut k = 0;
        while self.peek(k).kind == crate::lexer::TK::Ident && matches!(self.peek(k).text.as_str(), "explicit" | "inline" | "static" | "constexpr" | "consteval" | "friend" | "virtual" | "extern") {
            if self.peek(k).text == "explicit" && self.at_k(k + 1, "(") {
                k = self.close_paren_at(k + 1) + 1;
                continue;
            }
            k += 1;
        }
        let t = self.peek(k);
        if t.kind != crate::lexer::TK::Ident || is_keyword(&t.text) || !self.at_k(k + 1, "(") {
            return None;
        }
        let ids = self.lookup(&t.text)?;
        let id = ids[0];
        if !matches!(self.decls[id].k, DK::Template(TemplKind::Class(_))) {
            return None;
        }
        let tid = self.tid_of(id)?;
        let close = self.close_paren_at(k + 1);
        if !self.at_k(close + 1, "->") {
            return None;
        }
        Some((tid, k))
    }

    fn close_paren_at(&self, k: usize) -> usize {
        let mut d = 0i32;
        let mut i = k;
        loop {
            let t = self.peek(i);
            if t.kind == crate::lexer::TK::Eof {
                return i;
            }
            if t.kind == crate::lexer::TK::Punct {
                match t.text.as_str() {
                    "(" | "[" | "{" => d += 1,
                    ")" | "]" | "}" => {
                        d -= 1;
                        if d == 0 {
                            return i;
                        }
                    }
                    _ => {}
                }
            }
            i += 1;
        }
    }

    /// A deduction guide (CTAD-11 to CTAD-13).
    pub fn parse_guide(&mut self, tid: usize, k: usize, ctx: crate::decl2::Ctx) -> PR<()> {
        let line = self.line();
        let mut explicit = false;
        let mut bad_spec = false;
        let mut cond_explicit = false;
        for _ in 0..k {
            if self.at("explicit") {
                explicit = true;
                if self.at_k(1, "(") {
                    cond_explicit = true;
                }
            } else if self.tok().kind == crate::lexer::TK::Ident && matches!(self.tok().text.as_str(), "inline" | "static" | "constexpr" | "consteval" | "friend" | "virtual" | "extern") {
                bad_spec = true;
            }
            self.pos += 1;
        }
        let _ = cond_explicit;
        let (head, depth) = match self.pending_template.take() {
            Some(pt) => (pt.tparams, pt.depth),
            None => (vec![], self.tdepth),
        };
        self.pos += 1; // the name
        let rstart = self.pos + self.close_paren_at(0) + 2;
        let f = self.parse_params()?;
        // The parameter list reads `-> type` as a trailing return type.
        let result = match f.trailing.clone() {
            Some(t) => t,
            None => {
                self.expect("->")?;
                self.parse_type_id()?
            }
        };
        let qualified = self.toks[rstart.min(self.pos)..self.pos].iter().any(|t| t.text == "::");
        let mut ok = true;
        if self.inst_depth == 0 {
            if bad_spec {
                self.error(line, "CTAD-12", "a deduction guide with a specifier other than 'explicit'");
                ok = false;
            }
            let names_tid = match &result.t {
                Ty::Class(c) => self.inst_of.get(c).is_some_and(|x| x.0 == tid),
                Ty::Dep(d) => matches!(&**d, Dep::Id(TName::Templ(t), _) if *t == tid),
                _ => false,
            };
            if !names_tid && !result.is_unknown() {
                self.error(line, "CTAD-12", format!("the result of a deduction guide is not a specialization of '{}'", self.templs[tid].name));
                ok = false;
            } else if result.is_unknown() && !matches!(result.t, Ty::Dep(_)) {
                ok = false;
            }
            if result.c || result.v {
                self.error(line, "CTAD-12", "the result of a deduction guide has a cv-qualifier");
                ok = false;
            }
            if qualified {
                self.error(line, "CTAD-12", "the result of a deduction guide is a qualified name");
                ok = false;
            }
            if self.at("{") || self.at("=") {
                self.error(line, "CTAD-12", "a deduction guide with a body");
                self.recover();
                return Ok(());
            }
            // The scope of the class template.
            let here = self.decl_scope();
            if here != self.templs[tid].scope {
                self.error(line, "CTAD-12", "a deduction guide outside the scope of its class template");
                ok = false;
            }
            if let crate::decl2::Ctx::Class(_) = ctx {
                // A guide for a member template has the member's access.
                let a = self.access_stack.last().copied().unwrap_or(0);
                let ma = self.member_access.get(&self.templs[tid].decl).copied().unwrap_or(0);
                if a != ma {
                    self.error(line, "CTAD-12", "a deduction guide with another access than its member template");
                    ok = false;
                }
            }
        }
        self.expect(";")?;
        if self.in_template() && self.inst_depth == 0 && self.templ > if head.is_empty() { 0 } else { 1 } {
            // A guide inside a template pattern: not modeled.
            return Ok(());
        }
        let fn_pack = f.params.iter().position(|p| self.pack_params.contains(p));
        let g = Guide {
            head: head.clone(),
            depth,
            params: f.ptys.clone(),
            defaults: f.defaults,
            variadic: f.variadic && fn_pack.is_none(),
            fn_pack,
            result,
            explicit,
            kind: GKind::User,
            inherited: false,
            class_params: 0,
            cons: self.head_cons.clone(),
            line,
            seq: self.decls.len(),
            alias: None,
            explicit_cond: None,
        };
        if self.inst_depth == 0 {
            // CTAD-12: not declared twice; CTAD-13: each parameter
            // deducible.
            let existing = self.user_guides.get(&tid).cloned().unwrap_or_default();
            for e in &existing {
                if e.params == g.params && e.head.len() == g.head.len() && crate::templ::heads_equivalent(&e.head, &g.head) && e.result == g.result {
                    self.error(line, "CTAD-12", "a deduction guide is declared twice");
                    ok = false;
                } else if e.head.is_empty() && g.head.is_empty() && e.params == g.params {
                    self.error(line, "CTAD-12", "two deduction guides that are not templates have the same parameters");
                    ok = false;
                } else if !e.head.is_empty() && e.params == g.params && crate::templ::heads_equivalent(&e.head, &g.head) {
                    self.error(line, "CTAD-12", "two deduction guide templates with the same parameters");
                    ok = false;
                }
            }
            for (i, p) in head.iter().enumerate() {
                if p.default.is_some() || p.pack {
                    continue;
                }
                let mut used = Vec::new();
                let b = Binds::new(depth, &head);
                for t in &g.params {
                    self.guide_deducible(t, &b, &mut used);
                }
                if !used.contains(&(i as u32)) {
                    self.error(line, "CTAD-13", format!("the template parameter '{}' of a deduction guide cannot be deduced", p.name));
                    ok = false;
                }
            }
        }
        if ok {
            self.user_guides.entry(tid).or_default().push(g);
        }
        Ok(())
    }

    /// The parameters of depth `b.depth` in deducible positions of `q`.
    fn guide_deducible(&self, q: &QTy, b: &Binds, out: &mut Vec<u32>) {
        match &q.t {
            Ty::Dep(d) => match &**d {
                Dep::Param(dd, i) if *dd == b.depth => out.push(*i),
                Dep::Id(n, args) => {
                    if let TName::Param(dd, i) = n {
                        if *dd == b.depth {
                            out.push(*i);
                        }
                    }
                    for a in args {
                        match a {
                            TArg::Type(t) => self.guide_deducible(t, b, out),
                            TArg::Val(TVal::Param(dd, i)) if *dd == b.depth => out.push(*i),
                            TArg::Templ(TName::Param(dd, i)) if *dd == b.depth => out.push(*i),
                            TArg::Expand(x) => {
                                if let TArg::Type(t) = &**x {
                                    self.guide_deducible(t, b, out);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Dep::Array(el, v) => {
                    self.guide_deducible(el, b, out);
                    if let TVal::Param(dd, i) = v {
                        if *dd == b.depth {
                            out.push(*i);
                        }
                    }
                }
                Dep::MemPtr(c, m) => {
                    self.guide_deducible(c, b, out);
                    self.guide_deducible(m, b, out);
                }
                _ => {}
            },
            Ty::Ptr(x) | Ty::LRef(x) | Ty::RRef(x) | Ty::Array(x, _) | Ty::MemPtr(_, x) => self.guide_deducible(x, b, out),
            Ty::Func(f) => {
                self.guide_deducible(&f.ret, b, out);
                for p in &f.params {
                    self.guide_deducible(p, b, out);
                }
            }
            _ => {}
        }
    }

    /// The identity arguments of a head.
    fn head_args(head: &[TParam], depth: u32) -> Vec<TArg> {
        head.iter()
            .enumerate()
            .map(|(i, p)| {
                let a = match &p.kind {
                    TPK::Type => TArg::Type(dep(Dep::Param(depth, i as u32))),
                    TPK::Templ(_) => TArg::Templ(TName::Param(depth, i as u32)),
                    _ => TArg::Val(TVal::Param(depth, i as u32)),
                };
                if p.pack {
                    TArg::Expand(Box::new(a))
                } else {
                    a
                }
            })
            .collect()
    }

    /// The guides of the class template `tid` that come from the class
    /// (CTAD-5 to CTAD-7), and its user guides (CTAD-11).
    pub fn class_guides(&mut self, tid: usize) -> Option<Vec<Guide>> {
        let params = self.templs[tid].params.clone();
        let depth = self.templs[tid].depth;
        let ident = Self::head_args(&params, depth);
        let result = dep(Dep::Id(TName::Templ(tid), ident.clone()));
        let mut out = Vec::new();
        let pc = match self.decls[self.templs[tid].decl].k {
            DK::Template(TemplKind::Class(pc)) if pc != usize::MAX => Some(pc),
            _ => None,
        };
        let defined = self.templs[tid].src.is_some() && pc.is_some_and(|c| self.classes[c].complete);
        let cons = self.templs[tid].cons.clone();
        let mk = |head: Vec<TParam>, ps: Vec<QTy>, kind: GKind, cons: Option<Rc<CNorm>>| Guide {
            head,
            depth,
            params: ps,
            defaults: 0,
            variadic: false,
            fn_pack: None,
            result: result.clone(),
            explicit: false,
            kind,
            inherited: false,
            class_params: params.len(),
            cons,
            line: 0,
            seq: 0,
            alias: None,
            explicit_cond: None,
        };
        if defined {
            let pc = pc.unwrap();
            let ctors = self.classes[pc].ctors.clone();
            for c in &ctors {
                let Some(f) = self.func(*c).cloned() else { continue };
                let (head, fparams, fn_pack, gcons) = if f.template {
                    let Some(&ct) = self.decl_tid.get(c) else { return None };
                    let cp = self.templs[ct].params.clone();
                    let cd = self.templs[ct].depth;
                    let mut head = params.clone();
                    let n = head.len();
                    head.extend(cp.iter().cloned());
                    let all = Self::head_args(&head, depth);
                    let mine: Vec<TArg> = all[n..].to_vec();
                    let b = Binds { depth: cd, vals: mine.iter().map(|a| Some(match a { TArg::Expand(x) => (**x).clone(), x => x.clone() })).collect(), kinds: vec![], packs: vec![], unknown: false, fixed: vec![], from_bound: false };
                    let ps: Vec<QTy> = f.fty.params.iter().map(|q| self.subst_ty(q, &b)).collect();
                    // CTAD-29: the constraints of the constructor template,
                    // with its parameters after those of the class.
                    let cc = self.templs[ct].cons.clone().map(|x| self.cons_subst(&x, cd, &mine));
                    (head, ps, self.templs[ct].fn_pack, crate::cons::and(cons.clone(), cc))
                } else {
                    let fp = f.params.iter().position(|p| self.pack_params.contains(p));
                    let fc = self.fn_cons.get(c).cloned();
                    (params.clone(), f.fty.params.clone(), fp, crate::cons::and(cons.clone(), fc))
                };
                if fparams.iter().any(|q| q.t.has_plain_unknown()) {
                    return None;
                }
                // The injected class name in the pattern stands for the
                // class template with its own parameters.
                let fparams: Vec<QTy> = fparams.iter().map(|q| replace_class(q, pc, &result)).collect();
                let mut g = mk(head, fparams, GKind::Ctor(f.template), gcons);
                g.defaults = f.defaults;
                g.variadic = f.fty.variadic && fn_pack.is_none();
                g.fn_pack = fn_pack;
                g.explicit = f.explicit;
                g.explicit_cond = self.explicit_cond.get(c).cloned();
                g.line = self.decls[*c].line;
                out.push(g);
            }
            if ctors.is_empty() {
                out.push(mk(params.clone(), vec![], GKind::Default, cons.clone()));
            }
        } else {
            // CTAD-6: declared and not defined.
            out.push(mk(params.clone(), vec![], GKind::Default, cons.clone()));
        }
        // CTAD-7: the copy candidate.
        out.push(mk(params.clone(), vec![result.clone()], GKind::Copy, cons.clone()));
        // CTAD-11: the user guides declared before the use.
        let limit = if self.inst_depth > 0 { usize::MAX } else { self.decls.len() };
        if let Some(us) = self.user_guides.get(&tid) {
            for g in us {
                if g.seq <= limit {
                    out.push(g.clone());
                }
            }
        }
        Some(out)
    }

    /// CTAD-9, CTAD-10, CTAD-27: the aggregate candidate for a list.
    fn aggregate_guide(&mut self, tid: usize, items: &[Expr], braced: bool) -> Option<Guide> {
        let pc = match self.decls[self.templs[tid].decl].k {
            DK::Template(TemplKind::Class(pc)) if pc != usize::MAX => pc,
            _ => return None,
        };
        let ci = self.classes[pc].clone();
        if !ci.complete || !ci.ctors.is_empty() || ci.has_virtual || ci.bases.iter().any(|b| b.virt || b.access != 0) || ci.key == 2 {
            return None;
        }
        if self.user_guides.get(&tid).is_some_and(|v| !v.is_empty()) || items.is_empty() {
            return None;
        }
        let mut targets: Vec<(QTy, bool)> = self.class_base_types.get(&pc).cloned().unwrap_or_default();
        for &f in &ci.fields {
            if let DK::Field(fi) = &self.decls[f].k {
                if self.member_access.get(&f).copied().unwrap_or(0) != 0 {
                    return None;
                }
                if fi.bits.is_some() && self.decls[f].name.is_empty() {
                    continue;
                }
                targets.push((fi.ty.clone(), false));
            }
        }
        let params = self.templs[tid].params.clone();
        let depth = self.templs[tid].depth;
        let result = dep(Dep::Id(TName::Templ(tid), Self::head_args(&params, depth)));
        if items.iter().any(|x| matches!(&x.k, EK::Unknown(s) if s.starts_with("designator"))) {
            return None;
        }
        let mut ptys: Vec<QTy> = Vec::new();
        let mut fn_pack = None;
        let mut i = 0;
        let nt = targets.len();
        for (k, (t, pack)) in targets.into_iter().enumerate() {
            if i >= items.len() {
                break;
            }
            if pack {
                // CTAD-27: a pack of bases takes the rest of the list only
                // at the end.
                if k + 1 == nt {
                    fn_pack = Some(ptys.len());
                    ptys.push(t);
                    i = items.len();
                }
                continue;
            }
            let it = &items[i];
            let is_list = matches!(it.k, EK::InitList(_));
            let is_str = matches!(it.k, EK::Str(_));
            let dep_bound = matches!(&t.t, Ty::Dep(d) if matches!(**d, Dep::Array(..)));
            match &t.t {
                _ if dep_bound => {
                    if is_list {
                        ptys.push(QTy::new(Ty::RRef(Box::new(t.clone()))));
                    } else if is_str {
                        ptys.push(QTy::new(Ty::LRef(Box::new(add_cv(&t, true, false)))));
                    } else {
                        // CTAD-10: no brace elision for a dependent bound.
                        return None;
                    }
                    i += 1;
                }
                Ty::Array(el, Some(n)) => {
                    if is_list {
                        ptys.push(QTy::new(Ty::RRef(Box::new(t.clone()))));
                        i += 1;
                    } else if is_str && !el.is_unknown() {
                        ptys.push(QTy::new(Ty::LRef(Box::new(add_cv(&t, true, false)))));
                        i += 1;
                    } else {
                        // The braces are elided: one element of the list for
                        // each element of the array.
                        for _ in 0..*n {
                            if i >= items.len() {
                                break;
                            }
                            ptys.push((**el).clone());
                            i += 1;
                        }
                    }
                }
                Ty::Class(c) if !is_list && !t.is_unknown() && nonref(&it.ty).unqual().t != Ty::Class(*c) && self.aggregate_with_bases(*c) => {
                    // A member whose type is not dependent: its braces are
                    // elided, and the list fills its members.
                    let fs: Vec<QTy> = self.classes[*c].fields.iter().filter_map(|&f| match &self.decls[f].k {
                        DK::Field(fi) => Some(fi.ty.clone()),
                        _ => None,
                    }).collect();
                    for ft in fs {
                        if i >= items.len() {
                            break;
                        }
                        ptys.push(ft);
                        i += 1;
                    }
                }
                _ => {
                    ptys.push(t.clone());
                    i += 1;
                }
            }
        }
        if i < items.len() {
            // CTAD-10: each element has a member to initialize.
            return None;
        }
        let _ = braced;
        Some(Guide { head: params.clone(), depth, params: ptys, defaults: 0, variadic: false, fn_pack, result, explicit: false, kind: GKind::Aggregate, inherited: false, class_params: params.len(), cons: self.templs[tid].cons.clone(), line: 0, seq: 0, alias: None, explicit_cond: None })
    }

    /// CTAD-14, CTAD-16: the guide `g` of a class template B, adapted to
    /// a template whose head is `head` (depth `da`) and that stands for
    /// `pat`, a type B<...> over that head. `result` is the new result.
    #[allow(clippy::too_many_arguments)]
    fn adapt_guide(&mut self, g: &Guide, pat: &QTy, head: &[TParam], da: u32, result: QTy, alias: Option<usize>, inherited: bool, cons: Option<Rc<CNorm>>) -> Option<Guide> {
        let gargs = match &g.result.t {
            Ty::Dep(d) => match &**d {
                Dep::Id(_, a) => a.clone(),
                _ => return None,
            },
            _ => return None,
        };
        let pargs: Vec<TArg> = match &pat.t {
            Ty::Dep(d) => match &**d {
                Dep::Id(_, a) => a.iter().map(|x| self.invent_targ(x, da)).collect(),
                _ => return None,
            },
            Ty::Class(c) => self.inst_of.get(c)?.1.clone(),
            _ => return None,
        };
        // Match the guide's result with the type of the alias.
        let mut b = Binds::new(g.depth, &g.head);
        if gargs.len() != pargs.len() {
            return None;
        }
        for (x, y) in gargs.iter().zip(pargs.iter()) {
            if !self.deduce_targ(x, y, &mut b) || b.unknown {
                return None;
            }
        }
        // The new head: the alias parameters, then the guide's parameters
        // that the match leaves.
        let mut nh: Vec<TParam> = head.to_vec();
        let na = nh.len();
        let mut vals: Vec<Option<TArg>> = Vec::new();
        for (j, v) in b.vals.iter().enumerate() {
            match v {
                Some(x) => vals.push(Some(x.clone())),
                None => {
                    let k = nh.len() as u32;
                    nh.push(g.head[j].clone());
                    let a = match &g.head[j].kind {
                        TPK::Type => TArg::Type(dep(Dep::Param(da, k))),
                        TPK::Templ(_) => TArg::Templ(TName::Param(da, k)),
                        _ => TArg::Val(TVal::Param(da, k)),
                    };
                    vals.push(Some(a));
                }
            }
        }
        let gb = Binds { depth: g.depth, vals, kinds: vec![], packs: vec![], unknown: false, fixed: vec![], from_bound: false };
        // The invented parameters of the match become the alias's.
        let back = Binds { depth: INV, vals: Self::head_args(head, da).into_iter().map(|a| Some(match a { TArg::Expand(x) => *x, x => x })).collect(), kinds: vec![], packs: vec![], unknown: false, fixed: vec![], from_bound: false };
        let mut ps = Vec::new();
        for q in &g.params {
            let q1 = self.subst_ty(q, &gb);
            ps.push(self.subst_ty(&q1, &back));
        }
        let check = {
            let r1 = self.subst_ty(&g.result, &gb);
            self.subst_ty(&r1, &back)
        };
        let _ = na;
        Some(Guide {
            head: nh,
            depth: da,
            params: ps,
            defaults: g.defaults,
            variadic: g.variadic,
            fn_pack: g.fn_pack,
            result,
            explicit: g.explicit,
            kind: g.kind,
            inherited,
            class_params: if g.kind == GKind::User { 0 } else { head.len() },
            cons,
            line: g.line,
            seq: g.seq,
            alias: alias.map(|a| (a, check)),
            explicit_cond: None,
        })
    }

    /// The guides of an alias template (CTAD-14) or the guides that a
    /// class template inherits (CTAD-16), and the guides of the template
    /// itself.
    fn all_guides(&mut self, tid: usize, items: &[Expr], list: bool, line: u32) -> Option<Option<Vec<Guide>>> {
        let params = self.templs[tid].params.clone();
        let depth = self.templs[tid].depth;
        if self.templs[tid].kind == TKind::Alias {
            let ident = Self::head_args(&params, depth);
            let pat = self.class_inst(tid, ident, line);
            let target = match &pat.t {
                Ty::Dep(d) => match &**d {
                    Dep::Id(TName::Templ(b), _) if self.templs[*b].kind == TKind::Class => Some(*b),
                    _ => None,
                },
                Ty::Class(c) => self.inst_of.get(c).map(|x| x.0),
                _ => None,
            };
            let Some(b) = target else {
                // CTAD-15: only an alias for a class template specialization.
                self.error(line, "CTAD-15", format!("the alias template '{}' does not name a specialization of a class template", self.templs[tid].name));
                return Some(None);
            };
            let mut gs = self.class_guides(b)?;
            if list || !items.is_empty() {
                if let Some(g) = self.aggregate_guide(b, items, list) {
                    gs.push(g);
                }
            }
            let result = dep(Dep::Id(TName::Templ(tid), Self::head_args(&params, depth)));
            let cons = self.templs[tid].cons.clone();
            let mut out = Vec::new();
            for g in gs {
                if let Some(a) = self.adapt_guide(&g, &pat, &params, depth, result.clone(), Some(tid), false, cons.clone()) {
                    out.push(a);
                }
            }
            return Some(Some(out));
        }
        let mut gs = self.class_guides(tid)?;
        if list || !items.is_empty() {
            if let Some(g) = self.aggregate_guide(tid, items, list) {
                gs.push(g);
            }
        }
        // CTAD-16: the guides of a base whose constructors the class
        // inherits.
        let pc = match self.decls[self.templs[tid].decl].k {
            DK::Template(TemplKind::Class(pc)) if pc != usize::MAX => Some(pc),
            _ => None,
        };
        if let Some(pc) = pc {
            let names = self.inherits_ctors.get(&pc).cloned().unwrap_or_default();
            let bases = self.class_base_types.get(&pc).cloned().unwrap_or_default();
            let result = dep(Dep::Id(TName::Templ(tid), Self::head_args(&params, depth)));
            for n in names {
                for (bt, _) in &bases {
                    let b = match &bt.t {
                        Ty::Dep(d) => match &**d {
                            Dep::Id(TName::Templ(b), _) => *b,
                            _ => continue,
                        },
                        _ => continue,
                    };
                    if self.templs[b].name != n {
                        continue;
                    }
                    let bgs = self.class_guides(b)?;
                    for g in bgs {
                        if let Some(mut a) = self.adapt_guide(&g, bt, &params, depth, result.clone(), None, true, self.templs[tid].cons.clone()) {
                            a.alias = None;
                            gs.push(a);
                        }
                    }
                }
            }
        }
        Some(Some(gs))
    }

    /// A hidden function template for a guide.
    fn guide_templ(&mut self, g: &Guide, name: &str) -> (DeclId, usize) {
        let fty = FnTy { ret: g.result.clone(), params: g.params.clone(), variadic: g.variadic, noexcept: false, cv_c: false, cv_v: false, refq: 0 };
        let fi = FuncInfo {
            fty,
            params: vec![],
            defaults: g.defaults,
            body: None,
            defined: false,
            constexpr: false,
            consteval: false,
            deleted: false,
            defaulted: false,
            class: None,
            kind: FnKind::Normal,
            is_static: false,
            is_virtual: false,
            template: !g.head.is_empty(),
            explicit: g.explicit,
            opaque: false,
            is_main: false,
            explicit_obj: false,
            default_exprs: vec![None; g.params.len()],
            ret_auto: None,
            mem_inits: vec![],
            first_attrs: vec![],
            first_defaulted: false,
            constrained: false,
            fn_try: None,
            parse_failed: false,
            noexcept_false: false,
        };
        let g0 = self.global;
        let id = self.add_decl_in(g0, name, DK::Func(fi), g.line);
        self.hide_decl(id);
        let info = TemplInfo {
            name: name.to_string(),
            kind: TKind::Func,
            params: g.head.clone(),
            depth: g.depth,
            src: None,
            stack: vec![g0],
            decl: id,
            scope: g0,
            line: g.line,
            dependent: false,
            explicit: vec![],
            partials: vec![],
            insts: vec![],
            members: vec![],
            fn_pack: g.fn_pack,
            fresh: false,
            constrained: false,
            cons: g.cons.clone(),
            cons_key: String::new(),
        };
        self.templs.push(info);
        let t = self.templs.len() - 1;
        if !g.head.is_empty() {
            self.decl_tid.insert(id, t);
        }
        (id, t)
    }

    /// CTAD-P1: the type that the class template `tid` deduces from an
    /// initializer. None when the front end cannot tell; Some(unknown)
    /// after an error.
    pub fn ctad(&mut self, tid: usize, init: Option<&Expr>, form: &InitForm, line: u32) -> Option<QTy> {
        let (args, list): (Vec<Expr>, bool) = match (form, init) {
            (InitForm::None, _) | (_, None) => (vec![], false),
            (InitForm::Copy, Some(e)) => (vec![e.clone()], false),
            (InitForm::Paren(_), Some(e)) => match &e.k {
                EK::InitList(v) => (v.clone(), false),
                _ => (vec![e.clone()], false),
            },
            (InitForm::DirectList | InitForm::CopyList, Some(e)) => match &e.k {
                EK::InitList(v) => (v.clone(), true),
                _ => (vec![e.clone()], true),
            },
        };
        if args.iter().any(|a| a.ty.is_unknown() && !matches!(a.k, EK::InitList(_))) {
            return None;
        }
        let mut guides = match self.all_guides(tid, &args, list || matches!(form, InitForm::Paren(_)), line)? {
            Some(g) => g,
            None => return Some(QTy::unknown()),
        };
        if !(list || matches!(form, InitForm::Paren(_))) {
            guides.retain(|g| g.kind != GKind::Aggregate);
        }
        let copy_init = matches!(form, InitForm::Copy);
        let name = self.templs[tid].name.clone();
        let mut cands: Vec<DeclId> = Vec::new();
        let mut kinds: Vec<(DeclId, Guide, Option<usize>)> = Vec::new();
        for g in &guides {
            if copy_init && g.explicit && g.explicit_cond.is_none() {
                // CTAD-17: copy initialization leaves out the explicit
                // guides.
                continue;
            }
            let (gid, gt) = self.guide_templ(g, &format!("<guide of {}>", name));
            let spec = if g.head.is_empty() {
                gid
            } else {
                self.no_fwd = Some((g.depth, g.class_params as u32));
                let d = self.deduce_call(gid, None, &args, line);
                self.no_fwd = None;
                let targs = match d {
                    Ded::Ok(a) => a,
                    Ded::Fail => continue,
                    Ded::Unknown => return None,
                };
                match self.templ_sat(gt, &targs) {
                    crate::cons::Sat::Yes => {}
                    crate::cons::Sat::Unknown => return None,
                    _ => continue,
                }
                let b = Binds { depth: g.depth, vals: targs.iter().cloned().map(Some).collect(), kinds: vec![], packs: vec![], unknown: false, fixed: vec![], from_bound: false };
                let me = self.d.muted_errors;
                self.d.muted += 1;
                let mut ps: Vec<QTy> = Vec::new();
                for (i, q) in g.params.iter().enumerate() {
                    if g.fn_pack == Some(i) {
                        // The pack: one parameter for each element.
                        let mut vs = Vec::new();
                        if let Ty::Dep(dd) = &q.t {
                            if let Dep::Param(d0, j) = &**dd {
                                if *d0 == g.depth {
                                    if let Some(TArg::Pack(v)) = targs.get(*j as usize) {
                                        vs = v.clone();
                                    }
                                }
                            }
                        }
                        for v in vs {
                            if let TArg::Type(t) = v {
                                ps.push(t);
                            }
                        }
                        continue;
                    }
                    ps.push(self.subst_ty(q, &b));
                }
                let mut ret = self.subst_ty(&g.result, &b);
                if let Some((al, check)) = &g.alias {
                    // CTAD-28: the result is a specialization of the alias,
                    // and the arguments satisfy its constraints.
                    let na = self.templs[*al].params.len();
                    let aargs: Vec<TArg> = targs[..na.min(targs.len())].to_vec();
                    let want = self.subst_ty(check, &b);
                    ret = self.class_inst(*al, aargs, line);
                    if ret.is_unknown() || want.is_unknown() {
                        self.d.muted -= 1;
                        let failed = self.d.muted_errors != me;
                        self.d.muted_errors = me;
                        if failed {
                            continue;
                        }
                        return None;
                    }
                    if ret.unqual() != want.unqual() {
                        self.d.muted -= 1;
                        continue;
                    }
                }
                self.d.muted -= 1;
                if self.d.muted_errors != me {
                    self.d.muted_errors = me;
                    continue;
                }
                if ps.iter().any(|q| q.is_unknown()) || ret.is_unknown() {
                    return None;
                }
                let mut f = self.func(gid).cloned().unwrap();
                if let Some(src) = &g.explicit_cond {
                    // CTAD-5: a conditional explicit for the deduced
                    // arguments.
                    let cp = g.class_params.min(targs.len());
                    let head: Vec<TParam> = g.head[..cp].to_vec();
                    let r = self.replay(src, &head, &targs[..cp], None, |p| p.parse_assignment().ok());
                    if let Some((Some(e), _)) = r {
                        match self.const_eval_bool(&e) {
                            Ok(v) => f.explicit = v,
                            Err(_) => return None,
                        }
                    }
                    if copy_init && f.explicit {
                        continue;
                    }
                }
                f.template = false;
                f.fty.params = ps.clone();
                f.fty.ret = ret;
                f.default_exprs = vec![None; ps.len()];
                f.defaults = g.defaults.min(ps.len());
                let g0 = self.global;
                let sid = self.add_decl_in(g0, &format!("<guide of {}>", name), DK::Func(f), line);
                self.hide_decl(sid);
                self.spec_of.insert(sid, (gt, targs));
                sid
            };
            cands.push(spec);
            kinds.push((spec, g.clone(), if g.head.is_empty() { None } else { Some(gt) }));
        }
        if cands.is_empty() {
            self.error(line, "CTAD-24", format!("no deduction guide of '{}' is viable for the initializer", name));
            return Some(QTy::unknown());
        }
        // CTAD-19 to CTAD-22, after the usual tie-breakers.
        let mut ties = Vec::new();
        for (a, ga, ta) in &kinds {
            for (b, gb, tb) in &kinds {
                if a == b {
                    continue;
                }
                // OVL-27 and OVL-28 decide first.
                if ta.is_none() != tb.is_none() {
                    continue;
                }
                if let (Some(x), Some(y)) = (ta, tb) {
                    let (p, q) = self.func_order(*x, *y, Some(args.len()));
                    if p != q {
                        continue;
                    }
                }
                let fa = self.func(*a).unwrap().fty.params.clone();
                let fb = self.func(*b).unwrap().fty.params.clone();
                let same = fa == fb;
                let wins = if ga.inherited != gb.inherited && same {
                    !ga.inherited
                } else if (ga.kind == GKind::User) != (gb.kind == GKind::User) {
                    ga.kind == GKind::User
                } else if (ga.kind == GKind::Copy) != (gb.kind == GKind::Copy) {
                    ga.kind == GKind::Copy
                } else if ga.kind == GKind::Ctor(false) && gb.kind == GKind::Ctor(true) {
                    true
                } else {
                    false
                };
                if wins {
                    ties.push((*a, *b));
                }
            }
        }
        self.extra_ties = ties;
        self.ics_no_viable = false;
        let r = self.ics_select(None, &cands, &args);
        self.extra_ties.clear();
        match r {
            Ok(Some((f, _))) => {
                let fi = self.func(f).cloned().unwrap();
                if fi.explicit && matches!(form, InitForm::CopyList) {
                    // CTAD-23: no explicit guide in copy-list
                    // initialization.
                    self.error(line, "CTAD-23", format!("copy-list initialization chooses an explicit deduction guide of '{}'", name));
                    return Some(QTy::unknown());
                }
                Some(fi.fty.ret)
            }
            Ok(None) => {
                self.error(line, "CTAD-24", format!("the deduction guides of '{}' are ambiguous for the initializer", name));
                Some(QTy::unknown())
            }
            Err(()) => {
                if std::mem::take(&mut self.ics_no_viable) {
                    self.error(line, "CTAD-24", format!("no deduction guide of '{}' is viable for the initializer", name));
                    return Some(QTy::unknown());
                }
                None
            }
        }
    }
}

impl P {
    /// The initializer at the current token, `( ... )` or `{ ... }`, read
    /// ahead without its diagnostics, and the type that CTAD deduces from
    /// it. The position stays.
    pub fn ctad_lookahead(&mut self, tid: usize, line: u32) -> Option<QTy> {
        let save = self.pos;
        let mark = self.d.list.len();
        let (me, mu) = (self.d.muted_errors, self.d.muted_unsup);
        self.d.muted += 1;
        let r = (|| -> PR<(Expr, InitForm)> {
            if self.at("{") {
                let e = self.parse_braced_init()?;
                Ok((e, InitForm::DirectList))
            } else {
                self.expect("(")?;
                let mut args = Vec::new();
                if !self.at(")") {
                    loop {
                        let a = if self.at("{") { self.parse_braced_init()? } else { self.parse_assignment()? };
                        args.push(a);
                        if !self.eat(",") {
                            break;
                        }
                    }
                }
                self.expect(")")?;
                let n = args.len();
                Ok((Expr::new(EK::InitList(args), QTy::unknown(), Cat::PR, line), InitForm::Paren(n)))
            }
        })();
        self.d.muted -= 1;
        let bad = self.d.muted_errors != me || self.d.muted_unsup != mu;
        self.d.muted_errors = me;
        self.d.muted_unsup = mu;
        self.d.list.truncate(mark);
        self.pos = save;
        let (init, form) = r.ok()?;
        if bad {
            return None;
        }
        let q = self.ctad(tid, Some(&init), &form, line)?;
        if q.is_unknown() {
            return None;
        }
        Some(q)
    }
}

/// `q` with the class `c` replaced by `with`.
fn replace_class(q: &QTy, c: usize, with: &QTy) -> QTy {
    let t = match &q.t {
        Ty::Class(x) if *x == c => return add_cv(with, q.c, q.v),
        Ty::Ptr(x) => Ty::Ptr(Box::new(replace_class(x, c, with))),
        Ty::LRef(x) => Ty::LRef(Box::new(replace_class(x, c, with))),
        Ty::RRef(x) => Ty::RRef(Box::new(replace_class(x, c, with))),
        Ty::Array(x, n) => Ty::Array(Box::new(replace_class(x, c, with)), *n),
        _ => return q.clone(),
    };
    QTy { t, c: q.c, v: q.v }
}
