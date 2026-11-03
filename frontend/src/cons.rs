// Constraints (specification/templates/concepts.md and constraints.md).
//
// A constraint is kept as its normal form (CNS-P1): a tree of
// conjunctions and disjunctions over atomic constraints. An atomic
// constraint keeps the tokens of its expression, the scope stack to
// parse them in, the template parameters that the expression names, and
// the arguments of those parameters (the parameter mapping of CNS-4).
// A concept-id is expanded into the normal form of its concept, with the
// arguments mapped (CNS-2). The check (CNS-P2) substitutes the mapping:
// it parses the tokens again with each parameter bound to its argument,
// as a specialization is made (templ.rs). Errors in that parse are a
// substitution failure, and the atom is not satisfied (CNS-7).

use crate::ast::*;
use crate::decl2::Ctx;
use crate::lexer::{Token, TK};
use crate::parser::*;
use crate::templ::*;
use crate::types::*;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum CNorm {
    And(Rc<CNorm>, Rc<CNorm>),
    Or(Rc<CNorm>, Rc<CNorm>),
    Atom(Rc<Atom>),
}

#[derive(Clone, Debug)]
pub struct Atom {
    /// The place of the expression in the source (CNS-4).
    pub id: (usize, usize),
    pub src: Source,
    /// The parameters that the tokens can name, and their arguments.
    pub params: Rc<Vec<TParam>>,
    pub map: Vec<TArg>,
    /// The parameters that the expression names.
    pub used: Vec<bool>,
    pub line: u32,
    /// A type constraint on a constant parameter with a placeholder type
    /// (CPT-21): the concept, applied to the type of the value `map[0]`
    /// and the arguments `map[1..]`.
    pub type_of: Option<usize>,
    /// For equivalence (CNS-27): the tokens of an expression outside a
    /// concept, with names replaced by what they name; empty for an
    /// expression of a concept, which its place identifies.
    pub key: String,
}

/// The result of a check.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sat {
    Yes,
    No,
    /// The front end cannot tell, or the arguments are dependent.
    Unknown,
    /// The program is not valid (CNS-9, CNS-33, CNS-35); the error was
    /// reported.
    Error,
}

pub fn and(a: Option<Rc<CNorm>>, b: Option<Rc<CNorm>>) -> Option<Rc<CNorm>> {
    match (a, b) {
        (Some(a), Some(b)) => Some(Rc::new(CNorm::And(a, b))),
        (a, None) => a,
        (None, b) => b,
    }
}

fn identity_arg(p: &TParam, depth: u32, i: usize) -> TArg {
    let a = match &p.kind {
        TPK::Type => TArg::Type(dep(Dep::Param(depth, i as u32))),
        TPK::Const(_) | TPK::Auto => TArg::Val(TVal::Param(depth, i as u32)),
        TPK::Templ(_) => TArg::Templ(TName::Param(depth, i as u32)),
    };
    if p.pack {
        TArg::Expand(Box::new(a))
    } else {
        a
    }
}

/// Whether `a` names parameter `i` of depth `depth` itself.
fn is_param_arg(a: &TArg, depth: u32, i: usize) -> bool {
    let inner = match a {
        TArg::Expand(x) => &**x,
        x => x,
    };
    match inner {
        TArg::Type(q) => !q.c && !q.v && matches!(&q.t, Ty::Dep(d) if **d == Dep::Param(depth, i as u32)),
        TArg::Val(TVal::Param(d, j)) => *d == depth && *j == i as u32,
        TArg::Templ(TName::Param(d, j)) => *d == depth && *j == i as u32,
        _ => false,
    }
}

/// The tokens that end a constraint operand in a requires clause.
fn clause_operand_ok(t: &Token) -> bool {
    if t.kind == TK::Eof {
        return true;
    }
    if t.kind != TK::Punct {
        return true;
    }
    !matches!(
        t.text.as_str(),
        "(" | "[" | "." | "->" | "++" | "--" | "::" | "==" | "!=" | "<" | "<=" | ">=" | "<=>" | "+" | "-" | "*" | "/" | "%" | "|" | "^" | "&" | "<<" | ">>" | "?" | ".*" | "->*"
    )
}

impl P {
    // ---- normal forms ----

    /// Parse a requires clause after the keyword (CPT-9). `lambda`: the
    /// clause of a lambda before its parameters.
    pub fn cons_clause(&mut self, lambda: bool) -> PR<Option<Rc<CNorm>>> {
        let mut l = self.cons_clause_and(lambda)?;
        while self.at("||") {
            self.pos += 1;
            let r = self.cons_clause_and(lambda)?;
            l = match (l, r) {
                (Some(a), Some(b)) => Some(Rc::new(CNorm::Or(a, b))),
                _ => {
                    self.cons_unmodeled = true;
                    None
                }
            };
        }
        Ok(l)
    }

    fn cons_clause_and(&mut self, lambda: bool) -> PR<Option<Rc<CNorm>>> {
        let mut l = self.cons_clause_primary(lambda)?;
        while self.at("&&") {
            self.pos += 1;
            let r = self.cons_clause_primary(lambda)?;
            l = and(l, r);
        }
        Ok(l)
    }

    fn cons_clause_primary(&mut self, lambda: bool) -> PR<Option<Rc<CNorm>>> {
        let line = self.line();
        if self.at("(") {
            self.pos += 1;
            let save = self.no_gt;
            self.no_gt = 0;
            let n = self.cons_expr();
            self.no_gt = save;
            let n = n?;
            self.expect(")")?;
            if !lambda && !clause_operand_ok(self.tok()) {
                self.error(line, "CPT-10", "an operand of a requires clause is not a primary expression");
                let _ = self.parse_binary_pub(3);
            }
            return Ok(n);
        }
        let start = self.pos;
        if let Some(n) = self.cons_concept_id()? {
            if !lambda && !clause_operand_ok(self.tok()) {
                self.error(line, "CPT-10", "an operand of a requires clause is not a primary expression");
                self.pos = start;
                let _ = self.parse_binary_pub(3);
                return Ok(Some(self.make_atom(start, self.pos, line)));
            }
            return Ok(n);
        }
        let primary = match self.tok().kind {
            TK::Ident => !matches!(self.tok().text.as_str(), "sizeof" | "alignof" | "noexcept" | "new" | "delete" | "throw" | "co_await" | "static_cast" | "const_cast" | "reinterpret_cast" | "dynamic_cast" | "typeid"),
            TK::Punct => matches!(self.tok().text.as_str(), "::" | "["),
            _ => true,
        };
        if !primary {
            self.error(line, "CPT-10", "an operand of a requires clause is not a primary expression");
            let _ = self.parse_binary_pub(3)?;
            return Ok(Some(self.make_atom(start, self.pos, line)));
        }
        let e = self.parse_primary_pub()?;
        self.atom_type_check(&e, line);
        // A qualified name or a template-id is still a primary expression.
        if !lambda && !clause_operand_ok(self.tok()) {
            self.error(line, "CPT-10", "an operand of a requires clause is not a primary expression");
            self.pos = start;
            let _ = self.parse_binary_pub(3);
        }
        Ok(Some(self.make_atom(start, self.pos, line)))
    }

    /// A constraint expression: `&&` and `||` over operands (CNS-2).
    pub fn cons_expr(&mut self) -> PR<Option<Rc<CNorm>>> {
        let mut l = self.cons_expr_and()?;
        while self.at("||") {
            self.pos += 1;
            let r = self.cons_expr_and()?;
            l = match (l, r) {
                (Some(a), Some(b)) => Some(Rc::new(CNorm::Or(a, b))),
                _ => {
                    self.cons_unmodeled = true;
                    None
                }
            };
        }
        Ok(l)
    }

    fn cons_expr_and(&mut self) -> PR<Option<Rc<CNorm>>> {
        let mut l = self.cons_expr_unit()?;
        while self.at("&&") {
            self.pos += 1;
            let r = self.cons_expr_unit()?;
            l = and(l, r);
        }
        Ok(l)
    }

    /// Whether the token after the group that opens at offset `k` ends
    /// an operand of `&&` or `||`.
    fn ends_operand_after(&self, k: usize) -> bool {
        let t = self.peek(k);
        t.kind == TK::Eof || (t.kind == TK::Punct && matches!(t.text.as_str(), "&&" | "||" | ")" | ";" | "," | "}" | "{" | "=" | ":")) || (t.kind == TK::Ident && !is_keyword_op(&t.text))
    }

    fn cons_expr_unit(&mut self) -> PR<Option<Rc<CNorm>>> {
        let line = self.line();
        let fold = self.at("(") && {
            self.pos += 1;
            let f = self.at("...") || self.is_fold();
            self.pos -= 1;
            f
        };
        if self.at("(") && !fold {
            // `( constraint )`, unless the group is part of a larger
            // operand.
            let close = self.matching_close(0);
            if self.ends_operand_after(close + 1) {
                self.pos += 1;
                let save = self.no_gt;
                self.no_gt = 0;
                let n = self.cons_expr();
                self.no_gt = save;
                let n = n?;
                self.expect(")")?;
                return Ok(n);
            }
        }
        let start = self.pos;
        let mark = self.d.list.len();
        let me = self.d.muted_errors;
        if let Some(n) = self.cons_concept_id()? {
            if self.ends_operand_after(0) {
                return Ok(n);
            }
            self.pos = start;
            self.d.list.truncate(mark);
            self.d.muted_errors = me;
        }
        let e = self.parse_binary_pub(3)?;
        self.atom_type_check(&e, line);
        Ok(Some(self.make_atom(start, self.pos, line)))
    }

    /// CNS-9: an atomic constraint whose type does not depend on template
    /// parameters must be bool.
    fn atom_type_check(&mut self, e: &Expr, line: u32) {
        let t = nonref(&e.ty);
        if !t.is_unknown() && t.t != Ty::Bool && self.inst_depth == 0 && !self.has_dep_operand(e) {
            self.error(line, "CNS-9", format!("an atomic constraint has type {}, not bool", self.show_ty(&t)));
        }
    }

    fn has_dep_operand(&self, e: &Expr) -> bool {
        // `sizeof` of a dependent type still has the type size_t.
        matches!(&e.k, EK::Unknown(w) if !w.starts_with("sizeof") && !w.starts_with("alignof"))
    }

    /// The offset of the token that closes the group at offset `k`.
    pub fn matching_close(&self, k: usize) -> usize {
        let mut depth = 0i32;
        let mut i = k;
        loop {
            let t = self.peek(i);
            if t.kind == TK::Eof {
                return i;
            }
            if t.kind == TK::Punct {
                match t.text.as_str() {
                    "(" | "[" | "{" => depth += 1,
                    ")" | "]" | "}" => {
                        depth -= 1;
                        if depth == 0 {
                            return i;
                        }
                    }
                    _ => {}
                }
            }
            i += 1;
        }
    }

    /// The concept that the name at the current token names, with the
    /// number of tokens of the name.
    pub fn concept_at(&self) -> Option<(usize, usize)> {
        let mut k = 0;
        if self.at("::") {
            k = 1;
        }
        let mut scope: Option<usize> = if k == 1 { Some(self.global) } else { None };
        loop {
            let t = self.peek(k);
            if t.kind != TK::Ident || is_keyword(&t.text) {
                return None;
            }
            let ids = match scope {
                None => self.lookup(&t.text)?,
                Some(s) => self.scopes[s].names.get(&t.text)?.clone(),
            };
            let id = *ids.first()?;
            if self.at_k(k + 1, "::") {
                match &self.decls[id].k {
                    DK::Namespace(s) => {
                        scope = Some(*s);
                        k += 2;
                        continue;
                    }
                    _ => return None,
                }
            }
            if let DK::Template(TemplKind::Concept) = self.decls[id].k {
                let tid = self.decl_tid.get(&id).copied()?;
                return Some((tid, k + 1));
            }
            return None;
        }
    }

    /// A concept-id at the current token: its normal form (CNS-2).
    fn cons_concept_id(&mut self) -> PR<Option<Option<Rc<CNorm>>>> {
        let Some((tid, n)) = self.concept_at() else { return Ok(None) };
        if !self.at_k(n, "<") {
            return Ok(None);
        }
        let start = self.pos;
        let line = self.line();
        self.pos += n;
        let raw = self.parse_raw_targs()?;
        let args = self.pattern_targs(tid, raw);
        let opaque = |a: &TArg| -> bool {
            fn go(a: &TArg) -> bool {
                match a {
                    TArg::Val(TVal::Opaque) => true,
                    TArg::Type(q) => q.t.has_plain_unknown(),
                    TArg::Pack(v) => v.iter().any(go),
                    TArg::Expand(x) => go(x),
                    _ => false,
                }
            }
            go(a)
        };
        match args {
            Some(args) if !args.iter().any(opaque) => Ok(Some(self.concept_expand(tid, &args))),
            // An argument that the front end cannot map: the concept-id is
            // checked as a whole.
            _ => Ok(Some(Some(self.make_atom(start, self.pos, line)))),
        }
    }

    /// The normal form of the concept `tid` for `args` (CNS-2).
    pub fn concept_expand(&mut self, tid: usize, args: &[TArg]) -> Option<Rc<CNorm>> {
        let Some(n) = self.templs[tid].cons.clone() else {
            self.cons_unmodeled = true;
            return None;
        };
        let depth = self.templs[tid].depth;
        Some(self.cons_subst(&n, depth, args))
    }

    /// Substitute `args` for the parameters of depth `depth` in each
    /// mapping of `n`.
    pub fn cons_subst(&mut self, n: &Rc<CNorm>, depth: u32, args: &[TArg]) -> Rc<CNorm> {
        match &**n {
            CNorm::And(a, b) => Rc::new(CNorm::And(self.cons_subst(a, depth, args), self.cons_subst(b, depth, args))),
            CNorm::Or(a, b) => Rc::new(CNorm::Or(self.cons_subst(a, depth, args), self.cons_subst(b, depth, args))),
            CNorm::Atom(a) => {
                let map: Vec<TArg> = a.map.iter().map(|m| self.map_subst(m, depth, args)).collect();
                let mut a2 = (**a).clone();
                a2.map = map;
                Rc::new(CNorm::Atom(Rc::new(a2)))
            }
        }
    }

    pub fn map_subst(&mut self, m: &TArg, depth: u32, args: &[TArg]) -> TArg {
        for (i, a) in args.iter().enumerate() {
            if is_param_arg(m, depth, i) {
                if let (TArg::Expand(_), TArg::Pack(_) | TArg::Expand(_)) = (m, a) {
                    return a.clone();
                }
                if !matches!(m, TArg::Expand(_)) {
                    return a.clone();
                }
            }
        }
        let b = Binds { depth, vals: args.iter().cloned().map(Some).collect(), kinds: vec![], patterns: vec![], packs: vec![], unknown: false, fixed: vec![], from_bound: false };
        self.subst_targ(m, &b)
    }

    /// An atomic constraint over the tokens from `start` to `end` (CNS-3).
    pub fn make_atom(&mut self, start: usize, end: usize, line: u32) -> Rc<CNorm> {
        let (params, depth) = self.cons_head.clone().unwrap_or((Rc::new(vec![]), u32::MAX));
        let toks = self.copy_toks(start, end);
        let used: Vec<bool> = params.iter().map(|p| !p.name.is_empty() && toks.iter().any(|t| t.kind == TK::Ident && t.text == p.name)).collect();
        let map: Vec<TArg> = params.iter().enumerate().map(|(i, p)| identity_arg(p, depth, i)).collect();
        let mut id = self.origin_of(start);
        if id.0 == usize::MAX {
            self.atom_counter += 1;
            id = (usize::MAX - 2, self.atom_counter);
        }
        let src = Source {
            toks,
            stack: self.stack.clone(),
            ctx: Ctx::Namespace,
            access: self.access_stack.last().copied().unwrap_or(0),
            limit: if self.inst_depth > 0 { self.lookup_limit } else { self.decls.len() },
            vid: id.0,
            base: id.1,
        };
        let key = if self.in_concept_def { String::new() } else { self.clause_key(start, end).join(" ") };
        Rc::new(CNorm::Atom(Rc::new(Atom { id, src, params, map, used, line, type_of: None, key })))
    }

    /// The constraint of a type constraint `C<A...>` on the parameter
    /// `i` of the head `head` (CPT-17, CPT-19, CPT-21). `span`: the
    /// name, the `<` of the arguments if any, and the end.
    #[allow(clippy::too_many_arguments)]
    pub fn type_constraint_norm(&mut self, tid: usize, extra: Vec<RawArg>, param: &TParam, depth: u32, i: usize, head: &[TParam], span: (usize, usize, Option<usize>, usize), line: u32) -> Option<Rc<CNorm>> {
        let me = identity_arg(param, depth, i);
        // CPT-18: a type concept, and an argument list that fits.
        let cps = self.templs[tid].params.clone();
        let first_type = cps.first().is_some_and(|p| matches!(p.kind, TPK::Type));
        if !first_type {
            self.error(line, "CPT-18", format!("the concept '{}' is not a type concept", self.templs[tid].name));
            return None;
        }
        let mut extra_args = Vec::new();
        for (k, r) in extra.into_iter().enumerate() {
            let kind = cps.get(k + 1).or(cps.last().filter(|p| p.pack)).map(|p| p.kind.clone());
            match self.pattern_arg(r, kind.as_ref()) {
                Some(a) => extra_args.push(a),
                None => {
                    self.cons_unmodeled = true;
                    return None;
                }
            }
        }
        let fixed = cps.iter().filter(|p| !p.pack && p.default.is_none()).count();
        let has_pack = cps.iter().any(|p| p.pack);
        if 1 + extra_args.len() < fixed || (!has_pack && 1 + extra_args.len() > cps.len()) {
            self.error(line, "CPT-18", format!("the type constraint gives the concept '{}' the wrong number of arguments", self.templs[tid].name));
            return None;
        }
        let pack_args = |mut args: Vec<TArg>| -> Vec<TArg> {
            // A pack parameter of the concept takes the rest.
            if has_pack {
                let k = cps.len() - 1;
                let rest: Vec<TArg> = if args.len() >= k { args.drain(k..).collect() } else { vec![] };
                if rest.len() == 1 && matches!(rest[0], TArg::Expand(_)) {
                    args.extend(rest);
                } else {
                    args.push(TArg::Pack(rest));
                }
            }
            args
        };
        match &param.kind {
            TPK::Auto | TPK::Const(_) => {
                let mut map = vec![TArg::Val(TVal::Param(depth, i as u32))];
                map.extend(extra_args);
                let used = vec![true; map.len()];
                let src = Source { toks: Rc::new(vec![]), stack: self.stack.clone(), ctx: Ctx::Namespace, access: 0, limit: usize::MAX, vid: usize::MAX, base: 0 };
                Some(Rc::new(CNorm::Atom(Rc::new(Atom { id: (usize::MAX - 1, tid), src, params: Rc::new(vec![]), map, used, line, type_of: Some(tid), key: String::new() }))))
            }
            _ if param.pack => {
                // CPT-19: `(C<T, A...> && ...)`, one atomic constraint.
                let mk = |k: TK, s: &str| Token { kind: k, text: s.to_string(), line, lit: None };
                let (ns, ne, lt, end) = span;
                let mut toks: Vec<Token> = vec![mk(TK::Punct, "(")];
                toks.extend(self.toks[ns..ne].iter().cloned());
                toks.push(mk(TK::Punct, "<"));
                toks.push(mk(TK::Ident, &param.name));
                if let Some(lt) = lt {
                    if end > lt + 2 {
                        toks.push(mk(TK::Punct, ","));
                        toks.extend(self.toks[lt + 1..end - 1].iter().cloned());
                    }
                }
                toks.push(mk(TK::Punct, ">"));
                toks.push(mk(TK::Punct, "&&"));
                toks.push(mk(TK::Punct, "..."));
                toks.push(mk(TK::Punct, ")"));
                toks.push(mk(TK::Eof, ""));
                let params = Rc::new(head.to_vec());
                let used: Vec<bool> = head.iter().map(|p| p.name == param.name || (lt.is_some() && toks.iter().any(|t| t.kind == TK::Ident && t.text == p.name))).collect();
                let map: Vec<TArg> = head.iter().enumerate().map(|(k, p)| identity_arg(p, depth, k)).collect();
                let mut id = self.origin_of(ns);
                if id.0 == usize::MAX {
                    self.atom_counter += 1;
                    id = (usize::MAX - 2, self.atom_counter);
                }
                let src = Source { toks: Rc::new(toks), stack: self.stack.clone(), ctx: Ctx::Namespace, access: 0, limit: usize::MAX, vid: usize::MAX, base: 0 };
                let key = format!("pack {} {}", tid, self.clause_key(ns, end).join(" "));
                Some(Rc::new(CNorm::Atom(Rc::new(Atom { id, src, params, map, used, line, type_of: None, key }))))
            }
            _ => {
                let mut args = vec![me];
                args.extend(extra_args);
                let args = pack_args(args);
                self.concept_expand(tid, &args)
            }
        }
    }

    // ---- checks ----

    /// CNS-P2: check the normal form `n`, whose mappings name the
    /// parameters of depth `depth`, for `args`.
    pub fn cons_check(&mut self, n: &Rc<CNorm>, depth: u32, args: &[TArg]) -> Sat {
        match &**n {
            CNorm::And(a, b) => match self.cons_check(a, depth, args) {
                Sat::Yes => self.cons_check(b, depth, args),
                x => x,
            },
            CNorm::Or(a, b) => match self.cons_check(a, depth, args) {
                Sat::No => self.cons_check(b, depth, args),
                x => x,
            },
            CNorm::Atom(a) => {
                let map: Vec<TArg> = a.map.iter().map(|m| self.map_subst(m, depth, args)).collect();
                self.atom_sat(a, &map)
            }
        }
    }

    /// CNS-7: one atomic constraint with its mapping substituted.
    fn atom_sat(&mut self, a: &Rc<Atom>, map: &[TArg]) -> Sat {
        if map.iter().zip(a.used.iter()).any(|(m, &u)| u && m.is_dep()) {
            return Sat::Unknown;
        }
        if let Some(tid) = a.type_of {
            let t = match map.first() {
                Some(TArg::Val(TVal::Const(t, _))) => t.unqual(),
                _ => return Sat::Unknown,
            };
            let mut args = vec![TArg::Type(t)];
            args.extend(map[1..].iter().cloned());
            let cps = self.templs[tid].params.clone();
            if cps.last().is_some_and(|p| p.pack) {
                let k = cps.len() - 1;
                if args.len() >= k {
                    let rest: Vec<TArg> = args.drain(k..).collect();
                    args.push(TArg::Pack(rest));
                }
            }
            let Some(n) = self.concept_expand(tid, &args) else { return Sat::Unknown };
            return self.cons_check(&n, u32::MAX, &[]);
        }
        let key_map: Vec<TArg> = map.iter().zip(a.used.iter()).map(|(m, &u)| if u { m.clone() } else { TArg::Pack(vec![]) }).collect();
        if let Some(v) = self.sat_cache.get(&a.id) {
            if let Some(x) = v.iter().find(|x| x.0 == key_map && x.1 == a.src.stack) {
                return x.2;
            }
        }
        if self.sat_busy.iter().any(|x| x.0 == a.id && x.1 == key_map) {
            // CNS-35: the check needs its own result.
            self.hard_error(a.line, "CNS-35", "the check of an atomic constraint needs its own result".into());
            return Sat::Error;
        }
        // INST-1: a class argument is used as a complete type, so its
        // specialization is instantiated (for its hidden friends, too).
        for m in map {
            if let TArg::Type(q) = m {
                if let Ty::Class(c) = nonref(q).t {
                    self.require_class(c, a.line);
                }
            }
        }
        self.sat_busy.push((a.id, key_map.clone()));
        let (me, mu) = (self.d.muted_errors, self.d.muted_unsup);
        self.d.muted += 1;
        let saved_mark = self.sfinae_mark.replace(me);
        let params = a.params.clone();
        let r = self.replay(&a.src, &params, map, None, |p| {
            let e = p.parse_assignment();
            match e {
                Ok(e) if p.tok().kind == TK::Eof => Some(e),
                _ => None,
            }
        });
        self.sfinae_mark = saved_mark;
        self.d.muted -= 1;
        let errs = self.d.muted_errors != me;
        let unsup = self.d.muted_unsup != mu;
        self.d.muted_errors = me;
        self.d.muted_unsup = mu;
        self.sat_busy.pop();
        let res = match r {
            None => Sat::Unknown,
            Some((None, _)) => {
                if errs && !unsup {
                    Sat::No
                } else {
                    Sat::Unknown
                }
            }
            Some((Some(e), _)) => {
                if unsup {
                    Sat::Unknown
                } else if errs {
                    Sat::No
                } else {
                    self.atom_value(&e, a.line)
                }
            }
        };
        if self.sat_busy.iter().all(|x| x.0 != a.id) {
            self.sat_cache.entry(a.id).or_default().push((key_map, a.src.stack.clone(), res));
        }
        res
    }

    /// CNS-7, CNS-9: the value of an atomic constraint after
    /// substitution.
    fn atom_value(&mut self, e: &Expr, line: u32) -> Sat {
        let t = nonref(&e.ty);
        if t.is_unknown() {
            return Sat::Unknown;
        }
        if t.t != Ty::Bool {
            let msg = format!("an atomic constraint has type {}, not bool", self.show_ty(&t));
            self.hard_error(line, "CNS-9", msg);
            return Sat::Error;
        }
        match self.const_eval_bool(e) {
            Ok(true) => Sat::Yes,
            Ok(false) => Sat::No,
            Err(crate::consteval::CeErr::NotConst(r, m)) => {
                self.hard_error(line, "CNS-9", format!("an atomic constraint is not a constant expression ({}: {})", r, m));
                Sat::Error
            }
            Err(crate::consteval::CeErr::Unsupported(_)) => Sat::Unknown,
        }
    }

    /// The associated constraints of the template `tid` for `args`.
    pub fn templ_sat(&mut self, tid: usize, args: &[TArg]) -> Sat {
        if self.templs[tid].constrained {
            return Sat::Unknown;
        }
        let Some(n) = self.templs[tid].cons.clone() else { return Sat::Yes };
        let depth = self.templs[tid].depth;
        self.cons_check(&n, depth, args)
    }

    /// AUTO-16, CPT-22, CPT-23: the type that a constrained placeholder
    /// deduces satisfies its type constraint. `t` is the declared type
    /// after deduction; the placeholder's part is found by removing what
    /// the declaration writes around it.
    pub fn placeholder_check(&mut self, ds: &crate::decl::DeclSpec, d: &crate::decl::Declarator, t: &QTy, rule: &'static str, line: u32) {
        let Some((tid, raw, _)) = ds.cons_auto.clone() else { return };
        if t.is_unknown() {
            return;
        }
        let mut pt = t.clone();
        if ds.auto != Some(true) {
            for op in d.ops.iter().rev() {
                match op {
                    crate::decl::DOp::LRef => pt = nonref(&pt),
                    crate::decl::DOp::RRef => {
                        if matches!(pt.t, Ty::LRef(_)) {
                            // A forwarding reference keeps the lvalue
                            // reference that it deduces.
                            break;
                        }
                        pt = nonref(&pt);
                    }
                    crate::decl::DOp::Ptr(..) => match &pt.t {
                        Ty::Ptr(x) => pt = (**x).clone(),
                        _ => return,
                    },
                    crate::decl::DOp::Func(_) => {}
                    _ => return,
                }
            }
            if !matches!(pt.t, Ty::LRef(_) | Ty::RRef(_)) {
                if ds.c > 0 {
                    pt.c = false;
                }
                if ds.v > 0 {
                    pt.v = false;
                }
            }
        }
        let cps = self.templs[tid].params.clone();
        let mut args = vec![TArg::Type(pt.clone())];
        for (k, r) in raw.into_iter().enumerate() {
            let kind = cps.get(k + 1).or(cps.last().filter(|p| p.pack)).map(|p| p.kind.clone());
            match self.pattern_arg(r, kind.as_ref()) {
                Some(a) if !a.is_dep() => args.push(a),
                _ => return,
            }
        }
        if cps.last().is_some_and(|p| p.pack) {
            let k = cps.len() - 1;
            let rest: Vec<TArg> = if args.len() >= k { args.drain(k..).collect() } else { vec![] };
            args.push(TArg::Pack(rest));
        }
        if self.concept_value(tid, &args) == Sat::No {
            let msg = format!("the deduced type {} does not satisfy the concept '{}'", self.show_ty(&pt), self.templs[tid].name);
            self.error(line, rule, msg);
        }
    }

    /// An error of the program, also during a substitution (CNS-9,
    /// CNS-35).
    pub fn hard_error(&mut self, line: u32, rule: &'static str, msg: String) {
        let m = std::mem::take(&mut self.d.muted);
        self.error(line, rule, msg);
        self.d.muted = m;
    }

    /// CNS-13: the arguments of a template-id satisfy the template's
    /// constraints. False after an error.
    pub fn cns13_ok(&mut self, tid: usize, args: &[TArg], line: u32) -> bool {
        if self.templs[tid].cons.is_none() || args.iter().any(|a| a.is_dep()) {
            return true;
        }
        match self.templ_sat(tid, args) {
            Sat::No => {
                let label = format!("{}<{}>", self.templs[tid].name, self.show_targs(args));
                self.error(line, "CNS-13", format!("the arguments of '{}' do not satisfy the constraints of the template", label));
                false
            }
            Sat::Error => false,
            _ => true,
        }
    }

    /// The trailing requires clause of a function that is not a
    /// template (OVL-24, CNS-12).
    pub fn fn_sat(&mut self, fid: DeclId) -> Sat {
        if let Some(&s) = self.fn_sat_cache.get(&fid) {
            return s;
        }
        let Some(n) = self.fn_cons.get(&fid).cloned() else { return Sat::Yes };
        let s = self.cons_check(&n, u32::MAX, &[]);
        if s != Sat::Unknown {
            self.fn_sat_cache.insert(fid, s);
        }
        s
    }

    /// The value of a concept-id (CPT-1).
    pub fn concept_value(&mut self, tid: usize, args: &[TArg]) -> Sat {
        let Some(n) = self.concept_expand(tid, args) else { return Sat::Unknown };
        self.cons_check(&n, u32::MAX, &[])
    }

    /// The arguments of a concept-id outside a template (CPT-36): they
    /// must fit the concept's parameters.
    pub fn concept_targs(&mut self, tid: usize, raw: Vec<RawArg>, line: u32) -> Result<Option<Vec<TArg>>, ()> {
        let n = self.d.list.len();
        let r = self.targs_for(tid, raw, line, false);
        // Name the rule of the concept-id on the errors of the match.
        for d in self.d.list[n..].iter_mut() {
            if d.sev == crate::diag::Severity::Error && matches!(d.rule, Some("TARG-6" | "TARG-7")) {
                d.cause = d.rule;
                d.rule = Some("CPT-36");
            }
        }
        r
    }

    /// The tokens of a requires clause for equivalence (CNS-27): each
    /// name of a template parameter or of a bound argument becomes what it
    /// names.
    pub fn clause_key(&self, start: usize, end: usize) -> Vec<String> {
        let mut out = Vec::new();
        for t in &self.toks[start..end.min(self.toks.len())] {
            if t.kind == TK::Ident && !is_keyword(&t.text) {
                if let Some(ids) = self.lookup(&t.text) {
                    let id = ids[0];
                    if let Some(&(d, i)) = self.tparam.get(&id) {
                        out.push(format!("${}.{}", d, i));
                        continue;
                    }
                    if self.binding_scopes.contains(&self.decls[id].scope) {
                        match &self.decls[id].k {
                            DK::Type(q) => {
                                out.push(format!("$t:{}", self.show_ty(q)));
                                continue;
                            }
                            DK::Var(_) => {
                                if let Some(e) = self.const_param_val.get(&id) {
                                    out.push(format!("$v:{:?}", self.const_int(e).ok()));
                                    continue;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            out.push(t.text.clone());
        }
        out
    }

    /// The key of the head being declared: its type constraints with
    /// those of the placeholder parameters `extra`, and its requires
    /// clause.
    pub fn head_key(&self, extra: Option<Rc<CNorm>>) -> String {
        let tc = and(self.head_parts.0.clone(), extra);
        format!("{}|{}", self.cons_key(&tc), self.cons_key(&self.head_parts.1))
    }

    /// A rendering of a normal form for equivalence (CNS-26, CNS-27).
    pub fn cons_key(&self, n: &Option<Rc<CNorm>>) -> String {
        fn go(n: &CNorm, out: &mut String) {
            match n {
                CNorm::And(a, b) => {
                    out.push('(');
                    go(a, out);
                    out.push_str(" && ");
                    go(b, out);
                    out.push(')');
                }
                CNorm::Or(a, b) => {
                    out.push('(');
                    go(a, out);
                    out.push_str(" || ");
                    go(b, out);
                    out.push(')');
                }
                CNorm::Atom(a) => {
                    if a.key.is_empty() {
                        out.push_str(&format!("{:?}{:?}", a.id, a.map));
                    } else {
                        out.push_str(&format!("[{}]{:?}", a.key, a.map));
                    }
                }
            }
        }
        let mut out = String::new();
        if let Some(n) = n {
            go(n, &mut out);
        }
        out
    }

    // ---- subsumption (CNS-22, CNS-23) ----

    /// The key of an atomic constraint for identity (CNS-4): its place,
    /// its scope stack, and the arguments of the parameters it uses.
    fn atom_key(&mut self, a: &Atom, depth: u32, args: &[TArg]) -> ((usize, usize), Vec<usize>, Vec<TArg>) {
        let map: Vec<TArg> = a.map.iter().zip(a.used.iter()).map(|(m, &u)| if u { self.map_subst(m, depth, args) } else { TArg::Pack(vec![]) }).collect();
        (a.id, a.src.stack.clone(), map)
    }

    /// Disjunctive normal form: a list of conjunctions of atom keys.
    #[allow(clippy::type_complexity)]
    fn dnf(&mut self, n: &Rc<CNorm>, depth: u32, args: &[TArg]) -> Vec<Vec<((usize, usize), Vec<usize>, Vec<TArg>)>> {
        match &**n {
            CNorm::Atom(a) => vec![vec![self.atom_key(a, depth, args)]],
            CNorm::Or(a, b) => {
                let mut v = self.dnf(a, depth, args);
                v.extend(self.dnf(b, depth, args));
                v
            }
            CNorm::And(a, b) => {
                let x = self.dnf(a, depth, args);
                let y = self.dnf(b, depth, args);
                let mut out = Vec::new();
                for p in &x {
                    for q in &y {
                        let mut c = p.clone();
                        c.extend(q.iter().cloned());
                        out.push(c);
                    }
                    if out.len() > 4096 {
                        break;
                    }
                }
                out
            }
        }
    }

    /// Conjunctive normal form: a list of disjunctions of atom keys.
    #[allow(clippy::type_complexity)]
    fn cnf(&mut self, n: &Rc<CNorm>, depth: u32, args: &[TArg]) -> Vec<Vec<((usize, usize), Vec<usize>, Vec<TArg>)>> {
        match &**n {
            CNorm::Atom(a) => vec![vec![self.atom_key(a, depth, args)]],
            CNorm::And(a, b) => {
                let mut v = self.cnf(a, depth, args);
                v.extend(self.cnf(b, depth, args));
                v
            }
            CNorm::Or(a, b) => {
                let x = self.cnf(a, depth, args);
                let y = self.cnf(b, depth, args);
                let mut out = Vec::new();
                for p in &x {
                    for q in &y {
                        let mut c = p.clone();
                        c.extend(q.iter().cloned());
                        out.push(c);
                    }
                    if out.len() > 4096 {
                        break;
                    }
                }
                out
            }
        }
    }

    /// CNS-22: `p` (mappings over depth `pd`, with `pargs`) subsumes `q`.
    pub fn subsumes(&mut self, p: &Rc<CNorm>, pd: u32, pargs: &[TArg], q: &Rc<CNorm>, qd: u32, qargs: &[TArg]) -> bool {
        let d = self.dnf(p, pd, pargs);
        let c = self.cnf(q, qd, qargs);
        d.iter().all(|conj| c.iter().all(|disj| conj.iter().any(|x| disj.contains(x))))
    }

    /// CNS-23: the constraints `a` are at least as constrained as `b`.
    pub fn at_least_constrained(&mut self, a: &Option<Rc<CNorm>>, ad: u32, aargs: &[TArg], b: &Option<Rc<CNorm>>, bd: u32, bargs: &[TArg]) -> bool {
        match (a, b) {
            (_, None) => true,
            (None, Some(_)) => false,
            (Some(x), Some(y)) => self.subsumes(x, ad, aargs, y, bd, bargs),
        }
    }

    /// CNS-23: whether `a` is more constrained than `b`.
    pub fn more_constrained(&mut self, a: &Option<Rc<CNorm>>, ad: u32, aargs: &[TArg], b: &Option<Rc<CNorm>>, bd: u32, bargs: &[TArg]) -> bool {
        self.at_least_constrained(a, ad, aargs, b, bd, bargs) && !self.at_least_constrained(b, bd, bargs, a, ad, aargs)
    }

    /// ORD-13, CNS-34: the constraints of two function templates after
    /// partial ordering: (t1 at least as constrained as t2, the reverse).
    pub fn templ_cons_order(&mut self, t1: usize, t2: usize) -> Option<(bool, bool)> {
        let (c1, c2) = (self.templs[t1].cons.clone(), self.templs[t2].cons.clone());
        if c1.is_none() && c2.is_none() {
            return None;
        }
        if self.templs[t1].constrained || self.templs[t2].constrained {
            return None;
        }
        let (p1, p2) = (self.templs[t1].params.clone(), self.templs[t2].params.clone());
        if p1.len() != p2.len() || p1.iter().zip(p2.iter()).any(|(a, b)| a.pack != b.pack || std::mem::discriminant(&a.kind) != std::mem::discriminant(&b.kind)) {
            return None;
        }
        let (d1, d2) = (self.templs[t1].depth, self.templs[t2].depth);
        // Map the parameters of t2 to those of t1, position by position.
        let to1: Vec<TArg> = p1.iter().enumerate().map(|(i, p)| identity_arg(p, d1, i)).collect();
        let f1 = self.func(self.templs[t1].decl).cloned();
        let f2 = self.func(self.templs[t2].decl).cloned();
        if let (Some(f1), Some(f2)) = (f1, f2) {
            // CNS-34: the function parameter types must agree.
            let b = Binds { depth: d2, vals: to1.iter().map(|a| Some(match a { TArg::Expand(x) => (**x).clone(), x => x.clone() })).collect(), kinds: vec![], patterns: vec![], packs: vec![], unknown: false, fixed: vec![], from_bound: false };
            if f1.fty.params.len() != f2.fty.params.len() {
                return None;
            }
            for (x, y) in f1.fty.params.iter().zip(f2.fty.params.iter()) {
                let y2 = self.subst_ty(y, &b);
                if *x != y2 {
                    return None;
                }
            }
        }
        let ident1: Vec<TArg> = to1.clone();
        let a = self.at_least_constrained(&c1, d1, &ident1, &c2, d2, &to1);
        let b = self.at_least_constrained(&c2, d2, &to1, &c1, d1, &ident1);
        Some((a, b))
    }
}

fn is_keyword_op(s: &str) -> bool {
    matches!(s, "and" | "or" | "not" | "bitand" | "bitor" | "xor" | "compl")
}

// ---- requires expressions (CPT-24 to CPT-35) ----

impl P {
    /// A requires expression after the keyword. In a template definition
    /// its value depends on the arguments. Outside a template, an
    /// invalid requirement is an error of the program (CPT-27). In a
    /// specialization, it makes the value false (CPT-26).
    pub fn parse_requires_expr(&mut self, line: u32) -> PR<Expr> {
        let pattern = self.in_template();
        let sfinae = !pattern && self.inst_depth > 0;
        let depth = self.stack.len();
        if self.at("(") {
            let pl = self.line();
            if self.at_k(1, "void") && self.at_k(2, ")") {
                // CPT-28: `(void)` means no parameters.
                self.pos += 3;
            } else {
                // CPT-29: no placeholder type, default argument, or ellipsis.
                let close = {
                    let mut k = 0;
                    let mut d = 0i32;
                    loop {
                        let t = self.peek(k);
                        if t.kind == TK::Eof {
                            break k;
                        }
                        if t.kind == TK::Punct && t.text == "(" {
                            d += 1;
                        }
                        if t.kind == TK::Punct && t.text == ")" {
                            d -= 1;
                            if d == 0 {
                                break k;
                            }
                        }
                        k += 1;
                    }
                };
                let mut bad = None;
                let mut d = 0i32;
                for k in 1..close {
                    let t = self.peek(k).clone();
                    if t.kind == TK::Punct && matches!(t.text.as_str(), "(" | "[" | "{") {
                        d += 1;
                    } else if t.kind == TK::Punct && matches!(t.text.as_str(), ")" | "]" | "}") {
                        d -= 1;
                    } else if d == 0 && t.kind == TK::Ident && t.text == "auto" {
                        bad = Some("a parameter of a requires expression has a placeholder type");
                    } else if d == 0 && t.kind == TK::Punct && t.text == "=" {
                        bad = Some("a parameter of a requires expression has a default argument");
                    } else if d == 0 && t.kind == TK::Punct && t.text == "..." && k + 1 == close {
                        bad = Some("the parameters of a requires expression end with an ellipsis");
                    }
                }
                let mut names_tparam = None;
                let f = self.parse_params()?;
                if let Some(m) = bad {
                    if !sfinae {
                        self.error(pl, "CPT-29", m);
                    }
                }
                for &pid in &f.params {
                    let n = self.decls[pid].name.clone();
                    if n.is_empty() {
                        continue;
                    }
                    let sc = self.decls[pid].scope;
                    let saved = std::mem::take(&mut self.stack);
                    self.stack = saved.clone();
                    let found = self.lookup(&n);
                    self.stack = saved;
                    if let Some(ids) = found {
                        if ids.iter().any(|&i| self.decls[i].scope != sc && (self.tparam.contains_key(&i) || matches!(self.decls[i].k, DK::TypeParam))) {
                            names_tparam = Some(n);
                        }
                    }
                }
                if let Some(n) = names_tparam {
                    if !sfinae {
                        self.error(pl, "CPT-29", format!("the parameter '{}' of a requires expression has the name of a template parameter", n));
                    }
                }
                self.stack.push(f.scope);
            }
        }
        self.expect("{")?;
        if self.at("}") && !sfinae {
            // CPT-34: at least one requirement.
            self.error(line, "CPT-34", "a requires expression with no requirement");
        }
        self.in_unevaluated += 1;
        let mut val = Some(true);
        let mut done = false;
        let r = (|| -> PR<()> {
            while !self.at("}") && self.tok().kind != TK::Eof {
                if done {
                    self.skip_requirement();
                    continue;
                }
                let (me, mu) = (self.d.muted_errors, self.d.muted_unsup);
                if sfinae {
                    self.d.muted += 1;
                }
                let saved_mark = if sfinae { self.sfinae_mark.replace(me) } else { self.sfinae_mark };
                let r = self.parse_requirement(pattern, sfinae);
                self.sfinae_mark = saved_mark;
                if sfinae {
                    self.d.muted -= 1;
                }
                let errs = self.d.muted_errors != me;
                let unsup = self.d.muted_unsup != mu;
                if sfinae {
                    self.d.muted_errors = me;
                    self.d.muted_unsup = mu;
                }
                match r {
                    Err(e) => {
                        if !sfinae && !pattern {
                            return Err(e);
                        }
                        self.skip_requirement();
                        if errs {
                            val = Some(false);
                            done = true;
                        } else {
                            val = None;
                        }
                        continue;
                    }
                    Ok(x) => {
                        if errs {
                            val = Some(false);
                            done = true;
                        } else if unsup {
                            if val == Some(true) {
                                val = None;
                            }
                        } else {
                            match x {
                                Some(true) => {}
                                Some(false) => {
                                    val = Some(false);
                                    done = true;
                                }
                                None => {
                                    if val == Some(true) {
                                        val = None;
                                    }
                                }
                            }
                        }
                    }
                }
                if !self.eat(";") {
                    if done {
                        self.skip_requirement();
                    } else {
                        self.expect(";")?;
                    }
                }
            }
            Ok(())
        })();
        self.in_unevaluated -= 1;
        self.stack.truncate(depth);
        r?;
        self.expect("}")?;
        if pattern {
            return Ok(Expr::new(EK::Unknown("requires expression".into()), QTy::bool_(), Cat::PR, line));
        }
        Ok(match val {
            Some(b) => Expr::new(EK::Int(b as i128), QTy::bool_(), Cat::PR, line),
            None => Expr::new(EK::Unknown("requires expression".into()), QTy::bool_(), Cat::PR, line),
        })
    }

    /// Skip one requirement and its `;`.
    fn skip_requirement(&mut self) {
        while !self.at(";") && !self.at("}") && self.tok().kind != TK::Eof {
            if self.at("{") || self.at("(") || self.at("[") {
                self.skip_balanced();
            } else {
                self.pos += 1;
            }
        }
        self.eat(";");
    }

    /// One requirement, up to its `;`: Some(holds), or None when the
    /// value is unknown or depends on template parameters.
    fn parse_requirement(&mut self, pattern: bool, sfinae: bool) -> PR<Option<bool>> {
        let line = self.line();
        if self.at("{") {
            // CPT-33: a compound requirement.
            self.pos += 1;
            let save = self.no_gt;
            self.no_gt = 0;
            let e = self.parse_expression();
            self.no_gt = save;
            let e = e?;
            self.expect("}")?;
            let mut res = if pattern || e.ty.is_unknown() && matches!(e.k, EK::Unknown(_)) { None } else { Some(true) };
            if self.eat("noexcept") && res == Some(true) {
                res = match self.noexcept_of(&e) {
                    Some(true) => Some(true),
                    Some(false) => return Ok(Some(false)),
                    None => None,
                };
            }
            if self.eat("->") {
                let Some((tid, n)) = self.concept_at() else {
                    if !sfinae {
                        self.error(line, "CPT-34", "a type after '->' in a compound requirement; a type constraint is needed");
                    }
                    let _ = self.parse_type_id();
                    return Ok(None);
                };
                self.pos += n;
                let raw = if self.at("<") { self.parse_raw_targs()? } else { vec![] };
                if res != Some(true) {
                    return Ok(res);
                }
                let t = self.decltype_of(&e, true);
                let cps = self.templs[tid].params.clone();
                let mut args = vec![TArg::Type(t)];
                for (k, r) in raw.into_iter().enumerate() {
                    let kind = cps.get(k + 1).or(cps.last().filter(|p| p.pack)).map(|p| p.kind.clone());
                    match self.pattern_arg(r, kind.as_ref()) {
                        Some(a) if !a.is_dep() => args.push(a),
                        _ => return Ok(None),
                    }
                }
                if cps.last().is_some_and(|p| p.pack) {
                    let k = cps.len() - 1;
                    let rest: Vec<TArg> = if args.len() >= k { args.drain(k..).collect() } else { vec![] };
                    args.push(TArg::Pack(rest));
                }
                return Ok(match self.concept_value(tid, &args) {
                    Sat::Yes => Some(true),
                    Sat::No | Sat::Error => Some(false),
                    Sat::Unknown => None,
                });
            }
            return Ok(res);
        }
        if self.at("typename") {
            // CPT-32: a type requirement.
            self.pos += 1;
            let t = self.tok().clone();
            if t.kind == TK::Ident && (crate::decl::simple_type_kw(&t.text) || t.text == "decltype") {
                if !sfinae {
                    self.error(line, "CPT-34", "a type requirement names a keyword type");
                }
                let _ = self.parse_type_id();
                return Ok(None);
            }
            // A name with a qualifier, or a template-id.
            let mut k = 0;
            let mut qualified = false;
            loop {
                let t = self.peek(k);
                if t.kind == TK::Eof || (t.kind == TK::Punct && (t.text == ";" || t.text == "}")) {
                    break;
                }
                if t.kind == TK::Punct && t.text == "::" {
                    qualified = true;
                }
                k += 1;
            }
            let q = if qualified { self.parse_typename_type()? } else { self.parse_type_id()? };
            if pattern {
                return Ok(None);
            }
            return Ok(if q.is_unknown() { None } else { Some(true) });
        }
        if self.at("requires") {
            // CPT-35: a nested requirement.
            self.pos += 1;
            let saved_head = self.cons_head.take();
            let n = self.cons_expr();
            self.cons_head = saved_head;
            let n = n?;
            if pattern {
                return Ok(None);
            }
            let Some(n) = n else { return Ok(None) };
            return Ok(match self.cons_check(&n, u32::MAX, &[]) {
                Sat::Yes => Some(true),
                Sat::No | Sat::Error => Some(false),
                Sat::Unknown => None,
            });
        }
        // CPT-31: a simple requirement.
        let e = self.parse_expression()?;
        if self.at("noexcept") || self.at("->") {
            // CPT-34: these belong to a compound requirement.
            if !sfinae {
                self.error(line, "CPT-34", "'noexcept' or '->' after a simple requirement");
            }
            while !self.at(";") && !self.at("}") && self.tok().kind != TK::Eof {
                self.pos += 1;
            }
            return Ok(None);
        }
        if pattern || matches!(e.k, EK::Unknown(_)) {
            return Ok(None);
        }
        Ok(Some(true))
    }
}

// ---- abbreviated function templates (DED-34, CPT-20) ----

/// The invented parameters of an abbreviated function template.
#[derive(Clone, Debug)]
pub struct Abbrev {
    pub depth: u32,
    /// The number of parameters of the written head.
    pub base: usize,
    pub params: Vec<TParam>,
    /// The type constraints of the placeholders (CNS-1, part 3).
    pub cons: Option<Rc<CNorm>>,
    /// The parameter list of the declarator is complete.
    pub closed: bool,
}

impl P {
    /// The type of the placeholder parameter `k`: its argument in a
    /// specialization, or a new invented parameter of the template being
    /// declared.
    pub fn auto_param_base(&mut self, k: usize, ds: &crate::decl::DeclSpec, line: u32) -> PR<Option<QTy>> {
        let name = format!("auto:{}", k);
        if let Some(ids) = self.lookup(&name) {
            let id = ids[0];
            if self.binding_scopes.contains(&self.decls[id].scope) {
                if let DK::Type(q) = &self.decls[id].k {
                    let q = q.clone();
                    if let Some(v) = self.pack_binds.get(&id).cloned() {
                        self.last_pack_ref = Some(id);
                        self.pack_refs.push(v.len());
                        let i = self.pack_index.unwrap_or(0);
                        return Ok(Some(match v.get(i) {
                            Some(TArg::Type(t)) => t.clone(),
                            _ => QTy::unknown(),
                        }));
                    }
                    return Ok(Some(q));
                }
            }
        }
        let Some(ab) = self.abbrev.as_ref() else { return Ok(None) };
        if ab.closed {
            return Ok(None);
        }
        if ds.auto == Some(true) && self.inst_depth == 0 {
            // CPT-37: no decltype(auto) parameter.
            self.error(line, "CPT-37", "a function parameter of type decltype(auto)");
        }
        // A pack: `...` in the declarator.
        let mut pack = false;
        let mut kk = 0;
        let mut d = 0i32;
        loop {
            let t = self.peek(kk);
            if t.kind == TK::Eof {
                break;
            }
            if t.kind == TK::Punct {
                match t.text.as_str() {
                    "(" | "[" | "{" => d += 1,
                    ")" | "]" | "}" => {
                        if d == 0 {
                            break;
                        }
                        d -= 1;
                    }
                    "," | "=" if d == 0 => break,
                    "..." if d == 0 => pack = true,
                    _ => {}
                }
            }
            kk += 1;
        }
        let ab = self.abbrev.as_ref().unwrap();
        let depth = ab.depth;
        let idx = ab.base + ab.params.len();
        let param = TParam { name: name.clone(), kind: TPK::Type, pack, default: None, line, dauto: false, pattern: None };
        self.abbrev.as_mut().unwrap().params.push(param.clone());
        if let Some(pt) = self.pending_template.as_mut() {
            pt.tparams.push(param.clone());
        }
        if let Some((tid, raw, span)) = ds.cons_auto.clone() {
            let head: Vec<TParam> = self.pending_template.as_ref().map(|p| p.tparams.clone()).unwrap_or_else(|| vec![param.clone()]);
            let n = self.type_constraint_norm(tid, raw, &param, depth, idx, &head, span, line);
            let ab = self.abbrev.as_mut().unwrap();
            ab.cons = and(ab.cons.take(), n);
        }
        Ok(Some(dep(Dep::Param(depth, idx as u32))))
    }

    /// Whether the declaration at the current token declares a function
    /// with a placeholder parameter (DED-34).
    pub fn looks_abbreviated(&self) -> bool {
        let first = self.tok();
        if first.kind == TK::Ident && matches!(first.text.as_str(), "template" | "using" | "namespace" | "static_assert" | "typedef" | "concept" | "export") {
            return false;
        }
        let mut k = 0;
        loop {
            let t = self.peek(k);
            if t.kind == TK::Eof {
                return false;
            }
            if t.kind == TK::Punct {
                match t.text.as_str() {
                    ";" | "{" | "=" => return false,
                    "[" => {
                        // An attribute or an array bound.
                        let c = self.close_at(k);
                        k = c + 1;
                        continue;
                    }
                    "(" => {
                        let prev = if k == 0 { None } else { Some(self.peek(k - 1).clone()) };
                        let fnlike = prev.is_some_and(|p| (p.kind == TK::Ident && !matches!(p.text.as_str(), "decltype" | "sizeof" | "alignof" | "alignas" | "noexcept" | "requires" | "explicit" | "__attribute__")) || (p.kind == TK::Punct && (p.text == ">" || p.text == ")")));
                        let c = self.close_at(k);
                        if !fnlike {
                            k = c + 1;
                            continue;
                        }
                        let mut d = 0;
                        for j in k + 1..c {
                            let t = self.peek(j);
                            if t.kind == TK::Punct && matches!(t.text.as_str(), "(" | "[" | "{") {
                                d += 1;
                            } else if t.kind == TK::Punct && matches!(t.text.as_str(), ")" | "]" | "}") {
                                d -= 1;
                            } else if d == 0 && t.kind == TK::Ident && t.text == "auto" && !(self.peek(j + 1).kind == TK::Punct && self.peek(j + 1).text == "[") {
                                return true;
                            }
                        }
                        return false;
                    }
                    _ => {}
                }
            }
            k += 1;
        }
    }

    /// The offset of the token that closes the group at offset `k`.
    fn close_at(&self, k: usize) -> usize {
        let mut depth = 0i32;
        let mut i = k;
        loop {
            let t = self.peek(i);
            if t.kind == TK::Eof {
                return i;
            }
            if t.kind == TK::Punct {
                match t.text.as_str() {
                    "(" | "[" | "{" => depth += 1,
                    ")" | "]" | "}" => {
                        depth -= 1;
                        if depth <= 0 {
                            return i;
                        }
                    }
                    _ => {}
                }
            }
            i += 1;
        }
    }
}

// ---- PACK-8 ----

impl P {
    /// Record a use of a pack in a template definition.
    pub fn note_pack_use(&mut self, id: DeclId, pos: usize, line: u32) {
        if self.in_template() && self.inst_depth == 0 && self.pack_decls.contains(&id) {
            self.pack_uses.push((pos, line));
        }
    }

    /// PACK-8: the uses of packs since `mark` are each followed by an
    /// expansion `...` before the end of the statement or declaration.
    /// Inside a lambda the check waits for the enclosing statement.
    pub fn check_pack_uses(&mut self, mark: usize) {
        if self.pack_uses.len() <= mark {
            return;
        }
        if self.fns.last().is_some_and(|f| f.is_lambda) || !self.in_template() {
            if !self.in_template() {
                self.pack_uses.truncate(mark);
            }
            return;
        }
        let uses: Vec<(usize, u32)> = self.pack_uses.drain(mark..).collect();
        let end = self.pos.min(self.toks.len());
        for (p, line) in uses {
            let expanded = p < end && self.toks[p + 1..end].iter().any(|t| t.kind == TK::Punct && t.text == "...");
            if !expanded {
                self.error(line, "PACK-8", "a pack is used outside the pattern of an expansion");
                return;
            }
        }
    }
}

impl P {
    /// CNS-19, CNS-20: the destructor of a class with several prospective
    /// destructors, or with constraints on its destructor.
    pub fn select_dtor(&mut self, c: usize, line: u32) {
        let ds = self.prospective_dtors.get(&c).cloned().unwrap_or_default();
        if ds.is_empty() || !ds.iter().any(|d| self.fn_cons.contains_key(d)) {
            return;
        }
        let mut sat = Vec::new();
        for &d in &ds {
            match self.fn_sat(d) {
                Sat::Yes => sat.push(d),
                Sat::Unknown => return,
                _ => {}
            }
        }
        let name = self.classes[c].name.clone();
        if sat.is_empty() {
            self.error(line, "CNS-20", format!("no prospective destructor of '{}' has satisfied constraints", name));
            return;
        }
        let best = sat.iter().copied().find(|&a| {
            sat.iter().all(|&b| {
                if a == b {
                    return true;
                }
                let (ca, cb) = (self.fn_cons.get(&a).cloned(), self.fn_cons.get(&b).cloned());
                self.more_constrained(&ca, u32::MAX, &[], &cb, u32::MAX, &[])
            })
        });
        match best {
            Some(d) => self.classes[c].dtor = Some(d),
            None => self.error(line, "CNS-20", format!("no prospective destructor of '{}' is more constrained than the others", name)),
        }
    }
}
