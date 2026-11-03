// Templates (specification/templates/).
//
// A template keeps the tokens of its declaration. A specialization is
// made by parsing those tokens again with each template parameter bound
// to its argument (`replay`): a type parameter becomes an alias of its
// argument, a constant parameter a constant, and a template template
// parameter a second name of its argument. The pattern itself is parsed
// once, with the parameters as dependent types (`Ty::Dep`), for the
// checks that need no arguments and as the patterns of deduction
// (DED-P3), selection (TSP-14), and partial ordering (ORD-P2).

use crate::ast::*;
use crate::decl2::{Ctx, InitForm};
use crate::lexer::{Token, TK};
use crate::parser::*;
use crate::types::*;
use std::rc::Rc;

/// The depth of the invented parameters of partial ordering (ORD-1).
pub const INV: u32 = u32::MAX - 1;

/// The kind of a template parameter (TPL-P2).
#[derive(Clone, Debug)]
pub enum TPK {
    Type,
    Const(QTy),
    /// A constant parameter whose type is a placeholder (TPL-11).
    Auto,
    Templ(Rc<Vec<TParam>>),
}

#[derive(Clone, Debug)]
pub struct TParam {
    pub name: String,
    pub kind: TPK,
    pub pack: bool,
    pub default: Option<Rc<Vec<Token>>>,
    pub line: u32,
    /// A `decltype(auto)` placeholder (TPL-11).
    pub dauto: bool,
    /// The declared type of a constant parameter whose placeholder has
    /// declarator operators, as in `auto*` (DED-12).
    pub pattern: Option<QTy>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TKind {
    Class,
    Func,
    Var,
    Alias,
    Concept,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Inst {
    Class(usize),
    Func(DeclId),
    Var(DeclId),
    Type(QTy),
    /// Being made: a use inside its own definition.
    Busy,
    Fail,
}

/// The tokens of a declaration and the context to parse them in.
#[derive(Clone, Debug)]
pub struct Source {
    pub toks: Rc<Vec<Token>>,
    pub stack: Vec<usize>,
    pub ctx: Ctx,
    pub access: u8,
    /// The number of declarations at the definition: lookup in a
    /// namespace sees only these (DEP-5).
    pub limit: usize,
    /// The token vector the tokens come from, and the index of the first
    /// one there, for the facts of the pattern's parse.
    pub vid: usize,
    pub base: usize,
}

#[derive(Clone, Debug)]
pub struct Partial {
    pub params: Vec<TParam>,
    pub pats: Vec<TArg>,
    pub src: Option<Source>,
    pub depth: u32,
    pub line: u32,
    /// The pattern class of a partial specialization of a class template.
    pub class: Option<usize>,
    /// The head has constraints that the front end does not model.
    pub constrained: bool,
    /// The associated constraints (CNS-1).
    pub cons: Option<Rc<crate::cons::CNorm>>,
}

/// A member of a class template defined outside the class (TPL-23,
/// TSP-18): the head, the arguments after the class name, and the
/// partial specialization it belongs to.
#[derive(Clone, Debug)]
pub struct Member {
    pub params: Vec<TParam>,
    pub pats: Vec<TArg>,
    pub src: Source,
    pub depth: u32,
    pub partial: Option<usize>,
}

pub struct TemplInfo {
    pub name: String,
    pub kind: TKind,
    pub params: Vec<TParam>,
    pub depth: u32,
    pub src: Option<Source>,
    /// The scope stack where the template is declared.
    pub stack: Vec<usize>,
    pub decl: DeclId,
    pub scope: usize,
    pub line: u32,
    /// Declared inside the pattern of another template: never
    /// instantiated on its own.
    pub dependent: bool,
    pub explicit: Vec<(Vec<TArg>, Inst)>,
    pub partials: Vec<Partial>,
    pub insts: Vec<(Vec<TArg>, Inst, Option<usize>)>,
    pub members: Vec<Member>,
    /// For a function template: the index of the function parameter
    /// pack, if any.
    pub fn_pack: Option<usize>,
    /// Made at the start of its first declaration.
    pub fresh: bool,
    /// Constraints that the front end does not model.
    pub constrained: bool,
    /// The associated constraints (CNS-1); for a concept, the normal
    /// form of its expression.
    pub cons: Option<Rc<crate::cons::CNorm>>,
    /// The constraints of the head as written, for equivalence (CNS-26).
    pub cons_key: String,
}

/// A template argument as written, before it is matched to a parameter.
#[derive(Clone, Debug)]
pub enum RawArg {
    Type(QTy),
    Expr(Expr),
    Templ(TName),
    Expand(Box<RawArg>),
}

/// What the declaration after a template head declared.
#[derive(Clone, Debug)]
pub enum Last {
    Class(usize),
    Partial(usize, Vec<TArg>),
    Func(DeclId),
    Var(DeclId),
    /// The definition of a static data member template outside its class.
    VarDef(DeclId),
    VarPartial(usize, Vec<TArg>),
    Alias(DeclId),
    /// A member of a class template defined outside the class.
    Member(usize, Vec<TArg>),
}

/// The declaration that a replay makes into a specialization.
#[derive(Clone, Debug)]
pub enum InstReq {
    Class { name: String, tid: usize, args: Vec<TArg>, existing: Option<usize> },
    Func { name: String, existing: Option<DeclId> },
    Var { name: String },
    Alias,
}

#[derive(Clone, Debug)]
pub enum InstOut {
    Class(usize),
    Func(DeclId),
    Var(DeclId),
    Type(QTy),
}

/// The values that deduction gives to the parameters of one template
/// head.
#[derive(Clone, Debug)]
pub struct Binds {
    pub depth: u32,
    pub vals: Vec<Option<TArg>>,
    pub kinds: Vec<TPK>,
    pub patterns: Vec<Option<QTy>>,
    pub packs: Vec<bool>,
    /// The pattern has a part the front end does not model.
    pub unknown: bool,
    /// The parameters that explicit arguments fixed (DED-3).
    pub fixed: Vec<bool>,
    /// The value being matched comes from an array bound (DED-22).
    pub from_bound: bool,
}

impl Binds {
    pub fn new(depth: u32, params: &[TParam]) -> Binds {
        Binds {
            depth,
            vals: vec![None; params.len()],
            kinds: params.iter().map(|p| p.kind.clone()).collect(),
            patterns: params.iter().map(|p| p.pattern.clone()).collect(),
            packs: params.iter().map(|p| p.pack).collect(),
            unknown: false,
            fixed: vec![false; params.len()],
            from_bound: false,
        }
    }
    pub fn bind(&mut self, i: u32, a: TArg) -> bool {
        let i = i as usize;
        if i >= self.vals.len() {
            return false;
        }
        match &self.vals[i] {
            None => {
                self.vals[i] = Some(a);
                true
            }
            Some(x) => *x == a,
        }
    }
}

/// Remove `const` and `volatile` from a type, also from the elements of
/// an array.
pub fn strip_cv(q: &QTy, c: bool, v: bool) -> QTy {
    match &q.t {
        Ty::Array(e, n) => {
            let e2 = strip_cv(e, c, v);
            QTy { c: e2.c, v: e2.v, t: Ty::Array(Box::new(e2), *n) }
        }
        _ => QTy { t: q.t.clone(), c: q.c && !c, v: q.v && !v },
    }
}

pub fn dep(d: Dep) -> QTy {
    QTy::new(Ty::Dep(Rc::new(d)))
}

/// The state that a replay changes and puts back.
pub struct Saved {
    toks: Vec<Token>,
    pos: usize,
    stack: Vec<usize>,
    fns: Vec<FnCtx>,
    templ: u32,
    class_depth: u32,
    deferred: Vec<Deferred>,
    pending_template: Option<crate::decl2::PendingTemplate>,
    access_stack: Vec<u8>,
    in_unevaluated: u32,
    discarded: u32,
    no_gt: u32,
    dmi_depth: u32,
    adl_pos: Option<usize>,
    call_adl: bool,
    decl_attrs: Vec<Attr>,
    pending_attrs: Vec<Attr>,
    tdepth: u32,
    inst: Option<InstReq>,
    inst_out: Option<InstOut>,
    explicit_spec: bool,
    last_templ: Option<Last>,
    auto_in_own_init: Option<DeclId>,
    in_base_clause: u32,
    inst_fresh: bool,
    last_align: u64,
    saw_alignas: bool,
    align_unknown: bool,
    saw_nua: bool,
    call_no_object: bool,
    in_inst_body: bool,
    member_replay: bool,
    lookup_limit: usize,
    tok_origin: (usize, usize),
    abbrev: Option<crate::cons::Abbrev>,
    param_nest: u32,
    auto_param_k: usize,
    cons_head: Option<(Rc<Vec<TParam>>, u32)>,
    head_cons: Option<Rc<crate::cons::CNorm>>,
    tparam_cons: Option<Rc<crate::cons::CNorm>>,
    head_parts: (Option<Rc<crate::cons::CNorm>>, Option<Rc<crate::cons::CNorm>>),
}

impl P {
    // ---- records ----

    /// The tokens from `start` to `end`, with an end-of-file token.
    pub fn copy_toks(&self, start: usize, end: usize) -> Rc<Vec<Token>> {
        let end = end.min(self.toks.len());
        let mut v: Vec<Token> = self.toks[start.min(end)..end].to_vec();
        let line = v.last().map(|t| t.line).unwrap_or(0);
        v.push(Token { kind: TK::Eof, text: String::new(), line, lit: None });
        Rc::new(v)
    }

    /// The template that a declaration names: a template, a template
    /// template parameter bound to a template, or an injected class name
    /// (TARG-11).
    pub fn tid_of(&self, id: DeclId) -> Option<usize> {
        if let Some(&t) = self.decl_tid.get(&id) {
            return Some(t);
        }
        if let Some(&t) = self.injected.get(&id) {
            return Some(t);
        }
        if let DK::Template(TemplKind::Class(c)) = &self.decls[id].k {
            return self.class_tid.get(c).copied();
        }
        None
    }

    /// The name of a template for a template argument.
    pub fn tname_of(&self, id: DeclId) -> Option<TName> {
        if let Some(&(d, i)) = self.tparam.get(&id) {
            if matches!(self.decls[id].k, DK::Template(_)) {
                return Some(TName::Param(d, i));
            }
        }
        self.tid_of(id).map(TName::Templ)
    }

    /// Whether `<` after this declaration's name opens a template
    /// argument list (TARG-1, TARG-11).
    pub fn names_template(&self, id: DeclId) -> bool {
        matches!(self.decls[id].k, DK::Template(_)) || self.injected.contains_key(&id) || self.func(id).is_some_and(|f| f.template)
    }

    pub fn tparam_type(&self, id: DeclId) -> QTy {
        match self.tparam.get(&id) {
            Some(&(d, i)) => dep(Dep::Param(d, i)),
            None => QTy::unknown(),
        }
    }

    /// The display name of a class: an instance shows its arguments.
    pub fn class_label_of(&self, c: usize) -> String {
        self.class_label.get(&c).cloned().unwrap_or_else(|| self.classes[c].name.clone())
    }

    pub fn show_ty(&self, q: &QTy) -> String {
        let cv = format!("{}{}", if q.c { "const " } else { "" }, if q.v { "volatile " } else { "" });
        let t = match &q.t {
            Ty::Class(c) => self.class_label_of(*c),
            Ty::Enum(e) => self.enums[*e].name.clone(),
            Ty::Ptr(x) => format!("{}*", self.show_ty(x)),
            Ty::LRef(x) => format!("{}&", self.show_ty(x)),
            Ty::RRef(x) => format!("{}&&", self.show_ty(x)),
            Ty::Array(x, n) => format!("{}[{}]", self.show_ty(x), n.map(|n| n.to_string()).unwrap_or_default()),
            Ty::Func(f) => format!("{}({}){}", self.show_ty(&f.ret), f.params.iter().map(|p| self.show_ty(p)).collect::<Vec<_>>().join(","), if f.noexcept { " noexcept" } else { "" }),
            Ty::MemPtr(c, x) => format!("{} {}::*", self.show_ty(x), self.class_label_of(*c)),
            Ty::Dep(_) => "<dependent>".into(),
            t => show(&QTy::new(t.clone())),
        };
        format!("{}{}", cv, t)
    }

    pub fn show_targs(&self, args: &[TArg]) -> String {
        let one = |a: &TArg| -> String {
            match a {
                TArg::Type(q) => self.show_ty(q),
                TArg::Val(TVal::Const(_, v)) => match v {
                    CVal::Int(i) => i.to_string(),
                    CVal::Float(b) => f64::from_bits(*b).to_string(),
                    CVal::Null => "nullptr".into(),
                    CVal::Addr(d, k) => format!("&{}#{}", self.decls[*d].name, k),
                    CVal::Obj(d) => format!("{{{}}}", d),
                },
                TArg::Val(_) => "<value>".into(),
                TArg::Templ(TName::Templ(t)) => self.templs[*t].name.clone(),
                TArg::Templ(_) => "<template>".into(),
                TArg::Pack(v) => format!("{{{}}}", self.show_targs(v)),
                TArg::Expand(_) => "...".into(),
            }
        };
        args.iter().map(one).collect::<Vec<_>>().join(", ")
    }

    /// A scope of the class template `t` (its pattern, an explicit or
    /// partial specialization, or an instance) that declares `member`.
    pub fn templ_member_scope(&self, t: usize, member: &str) -> Option<usize> {
        if self.templs[t].kind == TKind::Alias {
            // DEP-15: an alias template for a specialization of a class
            // template looks in that class template.
            let src = self.templs[t].src.as_ref()?;
            let k = src.toks.iter().position(|x| x.text == "=")?;
            let n = src.toks.get(k + 1)?;
            let ids = self.lookup(&n.text)?;
            let b = self.tid_of(ids[0])?;
            if b == t || self.templs[b].kind != TKind::Class {
                return None;
            }
            return self.templ_member_scope(b, member);
        }
        let mut classes: Vec<usize> = Vec::new();
        if let DK::Template(TemplKind::Class(pc)) = self.decls[self.templs[t].decl].k {
            if pc != usize::MAX {
                classes.push(pc);
            }
        }
        for (_, i) in &self.templs[t].explicit {
            if let Inst::Class(c) = i {
                classes.push(*c);
            }
        }
        for p in &self.templs[t].partials {
            if let Some(c) = p.class {
                classes.push(c);
            }
        }
        for (_, i, _) in &self.templs[t].insts {
            if let Inst::Class(c) = i {
                classes.push(*c);
            }
        }
        // DEP-15: an enclosing partial specialization names itself.
        for &s in &self.stack {
            if let SK::Class(c) = self.scopes[s].kind {
                if self.classes[c].name == self.templs[t].name {
                    classes.push(c);
                }
            }
        }
        classes.into_iter().map(|c| self.classes[c].scope).find(|&s| self.scopes[s].names.contains_key(member))
    }

    /// The record of a class template, made when its first declaration
    /// starts, so that its body can name it.
    pub fn class_templ_early(&mut self, c: usize, decl: DeclId) -> usize {
        if let Some(&t) = self.class_tid.get(&c) {
            return t;
        }
        let pt = self.pending_template.as_ref().unwrap();
        let info = TemplInfo {
            name: self.classes[c].name.clone(),
            kind: TKind::Class,
            params: pt.tparams.clone(),
            depth: pt.depth,
            src: None,
            stack: {
                let mut v = self.stack.clone();
                while v.last().is_some_and(|&s| self.scopes[s].kind == SK::Template) {
                    v.pop();
                }
                v
            },
            decl,
            scope: self.decl_scope(),
            line: self.classes[c].line,
            dependent: pt.dependent,
            explicit: vec![],
            partials: vec![],
            insts: vec![],
            members: vec![],
            fn_pack: None,
            fresh: true,
            constrained: self.head_constrained,
            cons: self.head_cons.clone(),
            cons_key: self.head_key(None),
        };
        self.templs.push(info);
        let t = self.templs.len() - 1;
        self.class_tid.insert(c, t);
        self.decl_tid.insert(decl, t);
        t
    }

    // ---- template argument lists ----

    /// A template name alone as an argument: a class, alias, or variable
    /// template, or a template template parameter, before `,` or `>`.
    fn bare_template_name(&mut self) -> Option<TName> {
        let mut k = 0;
        if self.at_k(0, "::") {
            k = 1;
        }
        loop {
            let t = self.peek(k);
            if t.kind != TK::Ident || is_keyword(&t.text) {
                return None;
            }
            if self.at_k(k + 1, "::") {
                k += 2;
                continue;
            }
            if !(self.at_k(k + 1, ",") || self.at_k(k + 1, ">") || self.at_k(k + 1, ">>") || self.at_k(k + 1, "...") || self.peek(k + 1).kind == TK::Eof) {
                return None;
            }
            break;
        }
        let save = self.pos;
        let mark = self.d.list.len();
        self.d.muted += 1;
        let r = (|| -> PR<Option<Vec<DeclId>>> {
            let scope = self.parse_nested_name()?;
            let name = self.ident()?;
            Ok(match scope {
                None => self.lookup(&name),
                Some(Ok(s)) => self.lookup_in_scope(s, &name, true),
                Some(Err(())) => None,
            })
        })();
        self.d.muted -= 1;
        self.d.list.truncate(mark);
        if let Ok(Some(ids)) = r {
            for &id in &ids {
                if self.injected.contains_key(&id) {
                    break;
                }
                if matches!(self.decls[id].k, DK::Template(TemplKind::Class(_)) | DK::Template(TemplKind::Alias) | DK::Template(TemplKind::Var { .. })) {
                    if let Some(n) = self.tname_of(id) {
                        return Some(n);
                    }
                }
            }
        }
        self.pos = save;
        None
    }

    /// Parse `<` arguments `>` as written (TARG-3: the first `>` outside
    /// brackets ends the list).
    pub fn parse_raw_targs(&mut self) -> PR<Vec<RawArg>> {
        self.expect("<")?;
        let mut out = Vec::new();
        if self.at(">") || self.at(">>") || self.at(">=") {
            self.expect(">")?;
            return Ok(out);
        }
        self.no_gt += 1;
        let r = (|| -> PR<()> {
            loop {
                let (mut items, expanded) = self.parse_expansion(|p| p.parse_raw_targ())?;
                if expanded {
                    out.extend(items);
                } else {
                    let a = items.pop().unwrap();
                    let a = if self.eat("...") { RawArg::Expand(Box::new(a)) } else { a };
                    out.push(a);
                }
                if !self.eat(",") {
                    return Ok(());
                }
            }
        })();
        self.no_gt -= 1;
        r?;
        self.expect(">")?;
        Ok(out)
    }

    pub fn parse_raw_targ(&mut self) -> PR<RawArg> {
        if let Some(n) = self.bare_template_name() {
            return Ok(RawArg::Templ(n));
        }
        if self.starts_type() || self.at("typename") {
            // TARG-8: what can be a type is a type.
            let save = self.pos;
            let mark = self.d.list.len();
            match self.parse_type_id() {
                Ok(q) if self.at(",") || self.at(">") || self.at(">>") || self.at("...") || self.at(">=") || self.tok().kind == TK::Eof => {
                    // The declarator took the `...` of a pack expansion.
                    if self.pos > 0 && self.toks[self.pos - 1].text == "..." && self.toks[self.pos - 1].kind == TK::Punct {
                        self.pos -= 1;
                    }
                    return Ok(RawArg::Type(q));
                }
                _ => {
                    self.pos = save;
                    self.d.list.truncate(mark);
                }
            }
        }
        if self.at("{") {
            // TARG-16: a braced list initializes a constant parameter of
            // class type.
            let e = self.parse_braced_init()?;
            return Ok(RawArg::Expr(e));
        }
        let e = self.parse_conditional()?;
        Ok(RawArg::Expr(e))
    }

    /// The arguments of a template-id as patterns, inside a template
    /// definition. Defaults are added when they can be.
    pub fn pattern_targs(&mut self, tid: usize, raw: Vec<RawArg>) -> Option<Vec<TArg>> {
        let params = self.templs[tid].params.clone();
        let mut out: Vec<TArg> = Vec::new();
        let mut pack: Vec<TArg> = Vec::new();
        let mut in_pack = false;
        for (i, r) in raw.into_iter().enumerate() {
            let p = params.get(i).or(params.last().filter(|p| p.pack));
            let pk = p.map(|p| p.pack).unwrap_or(false);
            let a = self.pattern_arg(r, p.map(|p| &p.kind))?;
            if pk {
                in_pack = true;
                pack.push(a);
            } else {
                out.push(a);
            }
        }
        if in_pack {
            // A single expansion of a pack stays as it is.
            if pack.len() == 1 && matches!(pack[0], TArg::Expand(_)) {
                out.push(pack.pop().unwrap());
            } else {
                out.push(TArg::Pack(pack));
            }
        }
        while out.len() < params.len() {
            let i = out.len();
            if params[i].pack {
                out.push(TArg::Pack(vec![]));
                continue;
            }
            let d = self.default_arg(tid, i, &out)?;
            out.push(d);
        }
        Some(out)
    }

    pub fn pattern_arg(&mut self, r: RawArg, kind: Option<&TPK>) -> Option<TArg> {
        Some(match r {
            RawArg::Type(q) => match kind {
                Some(TPK::Templ(_)) => match &q.t {
                    Ty::Class(c) => TArg::Templ(TName::Templ(self.inst_of.get(c)?.0)),
                    _ => return None,
                },
                _ => TArg::Type(q),
            },
            RawArg::Templ(n) => TArg::Templ(n),
            RawArg::Expand(x) => TArg::Expand(Box::new(self.pattern_arg(*x, kind)?)),
            RawArg::Expr(e) => {
                if let EK::Unknown(w) = &e.k {
                    if w.starts_with("template parameter#") {
                        let id: usize = w["template parameter#".len()..].parse().ok()?;
                        if let Some(&(d, i)) = self.tparam.get(&id) {
                            return Some(TArg::Val(TVal::Param(d, i)));
                        }
                    }
                }
                if e.ty.is_unknown() || has_unknown(&e) {
                    return Some(TArg::Val(TVal::Opaque));
                }
                match kind {
                    Some(TPK::Const(t)) if !t.is_unknown() => match self.const_targ(e, t, 0, true) {
                        Ok(v) => TArg::Val(v),
                        Err(_) => TArg::Val(TVal::Opaque),
                    },
                    _ => {
                        let t = nonref(&e.ty).unqual();
                        match self.const_targ(e, &t, 0, true) {
                            Ok(v) => TArg::Val(v),
                            Err(_) => TArg::Val(TVal::Opaque),
                        }
                    }
                }
            }
        })
    }

    /// The default of parameter `i` of a template, with the arguments
    /// before it (TARG-9).
    pub fn default_arg(&mut self, tid: usize, i: usize, prev: &[TArg]) -> Option<TArg> {
        let p = self.templs[tid].params.get(i)?.clone();
        let toks = p.default.clone()?;
        let params = self.templs[tid].params.clone();
        let src = Source { toks, stack: self.templs[tid].stack.clone(), ctx: Ctx::Namespace, access: 0, limit: usize::MAX, vid: usize::MAX, base: 0 };
        let mut args: Vec<TArg> = prev.to_vec();
        args.truncate(i);
        let kind = p.kind.clone();
        let (r, _) = self.replay(&src, &params[..i], &args, None, |p| -> Option<TArg> {
            let raw = p.parse_raw_targ().ok()?;
            if p.tok().kind != TK::Eof {
                return None;
            }
            match kind {
                TPK::Type => match raw {
                    RawArg::Type(q) => Some(TArg::Type(q)),
                    _ => None,
                },
                TPK::Templ(_) => match raw {
                    RawArg::Templ(n) => Some(TArg::Templ(n)),
                    _ => None,
                },
                TPK::Const(t) => match raw {
                    RawArg::Expr(e) => {
                        if e.ty.is_unknown() || t.is_unknown() {
                            return Some(TArg::Val(TVal::Opaque));
                        }
                        p.const_targ(e, &t, 0, false).ok().map(TArg::Val)
                    }
                    _ => None,
                },
                TPK::Auto => match raw {
                    RawArg::Expr(e) => {
                        if e.ty.is_unknown() {
                            return Some(TArg::Val(TVal::Opaque));
                        }
                        let t = auto_param_type(&e);
                        p.const_targ(e, &t, 0, false).ok().map(TArg::Val)
                    }
                    _ => None,
                },
            }
        })?;
        r
    }

    /// Match the written arguments of a template-id to the parameters
    /// (TARG-5 to TARG-21) outside a template definition. For a function
    /// template the list can end early (TARG-20). Ok(None) when the
    /// front end cannot tell; Err after an error.
    pub fn targs_for(&mut self, tid: usize, raw: Vec<RawArg>, line: u32, func: bool) -> Result<Option<Vec<TArg>>, ()> {
        let params = self.templs[tid].params.clone();
        let name = self.templs[tid].name.clone();
        let mut out: Vec<TArg> = Vec::new();
        let mut pack: Vec<TArg> = Vec::new();
        let mut in_pack = false;
        let n = raw.len();
        for (i, r) in raw.into_iter().enumerate() {
            let pi = if i < params.len() { i } else { params.len().saturating_sub(1) };
            let Some(p) = params.get(pi) else {
                self.error(line, "TARG-6", format!("too many template arguments for '{}'", name));
                return Err(());
            };
            if i >= params.len() && !p.pack {
                self.error(line, "TARG-6", format!("too many template arguments for '{}'", name));
                return Err(());
            }
            if matches!(r, RawArg::Expand(_)) {
                return Ok(None);
            }
            let a = match self.convert_targ(tid, p, r, &out, line) {
                Ok(a) => a,
                Err(true) => return Err(()),
                Err(false) => return Ok(None),
            };
            if p.pack {
                in_pack = true;
                pack.push(a);
            } else {
                out.push(a);
            }
        }
        let _ = n;
        if in_pack {
            out.push(TArg::Pack(pack));
        }
        if func {
            return Ok(Some(out));
        }
        while out.len() < params.len() {
            let i = out.len();
            if params[i].pack {
                out.push(TArg::Pack(vec![]));
                continue;
            }
            if params[i].default.is_none() {
                self.error(line, "TARG-6", format!("too few template arguments for '{}'", name));
                return Err(());
            }
            match self.default_arg(tid, i, &out) {
                Some(a) if !a.is_dep() => out.push(a),
                _ => return Ok(None),
            }
        }
        Ok(Some(out))
    }

    /// One argument for one parameter. Err(true) after an error,
    /// Err(false) when the front end cannot tell.
    fn convert_targ(&mut self, tid: usize, p: &TParam, r: RawArg, prev: &[TArg], line: u32) -> Result<TArg, bool> {
        match (&p.kind, r) {
            (TPK::Type, RawArg::Type(q)) => {
                if q.is_unknown() {
                    return Err(false);
                }
                Ok(TArg::Type(q))
            }
            (TPK::Type, RawArg::Expr(e)) => {
                if matches!(e.k, EK::Unknown(_)) && e.ty.is_unknown() {
                    return Err(false);
                }
                self.error(line, "TARG-7", "an expression is given for a type parameter");
                Err(true)
            }
            (TPK::Type, RawArg::Templ(n)) => {
                if let TName::Templ(t) = n {
                    if self.templs[t].kind == TKind::Class || self.templs[t].kind == TKind::Alias {
                        self.error(line, "TARG-10", format!("the template '{}' is used without an argument list", self.templs[t].name));
                        return Err(true);
                    }
                }
                self.error(line, "TARG-7", "a template is given for a type parameter");
                Err(true)
            }
            (TPK::Const(_) | TPK::Auto, RawArg::Type(_)) => {
                self.error(line, "TARG-7", "a type is given for a constant parameter");
                Err(true)
            }
            (TPK::Const(_) | TPK::Auto, RawArg::Templ(_)) => {
                self.error(line, "TARG-7", "a template is given for a constant parameter");
                Err(true)
            }
            (TPK::Const(t), RawArg::Expr(e)) => {
                if e.ty.is_unknown() && !matches!(e.k, EK::InitList(_)) {
                    return Err(false);
                }
                let t = if t.is_unknown() {
                    let params = self.templs[tid].params.clone();
                    let mut b = Binds::new(self.templs[tid].depth, &params);
                    for (i, a) in prev.iter().enumerate() {
                        b.vals[i] = Some(a.clone());
                    }
                    let s = self.subst_ty(t, &b);
                    if s.is_unknown() {
                        return Err(false);
                    }
                    s
                } else {
                    t.clone()
                };
                self.const_targ(e, &t, line, false).map(TArg::Val)
            }
            (TPK::Auto, RawArg::Expr(e)) => {
                if e.ty.is_unknown() {
                    return Err(false);
                }
                let t = if p.dauto { self.decltype_of(&e, e.paren) } else { auto_param_type(&e) };
                if !self.structural(&t) {
                    self.error(line, "TPL-7", "the type that the placeholder gets is not a structural type");
                    return Err(true);
                }
                self.const_targ(e, &t, line, false).map(TArg::Val)
            }
            (TPK::Templ(inner), r) => {
                let n = match r {
                    RawArg::Templ(n) => n,
                    RawArg::Type(q) => match &q.t {
                        Ty::Class(c) if self.inst_of.contains_key(c) => TName::Templ(self.inst_of[c].0),
                        _ if q.is_unknown() => return Err(false),
                        _ => {
                            self.error(line, "TARG-7", "a type is given for a template template parameter");
                            return Err(true);
                        }
                    },
                    _ => {
                        self.error(line, "TARG-7", "an expression is given for a template template parameter");
                        return Err(true);
                    }
                };
                let TName::Templ(t) = n.clone() else { return Ok(TArg::Templ(n)) };
                if !matches!(self.templs[t].kind, TKind::Class | TKind::Alias) {
                    self.error(line, "TARG-7", "a template that is not a class or alias template is given for a template template parameter");
                    return Err(true);
                }
                let inner = inner.clone();
                if !self.templ_head_accepts(&inner, t) {
                    self.error(line, "TARG-18", format!("the template '{}' does not accept the arguments of the parameter's head", self.templs[t].name));
                    return Err(true);
                }
                Ok(TArg::Templ(n))
            }
            (_, RawArg::Expand(_)) => Err(false),
        }
    }

    /// TARG-17: each argument list that the head `inner` accepts is valid
    /// for the template `t`.
    fn templ_head_accepts(&self, inner: &[TParam], t: usize) -> bool {
        let ps = &self.templs[t].params;
        let same_kind = |a: &TParam, q: &TParam| match (&a.kind, &q.kind) {
            (TPK::Type, TPK::Type) => true,
            (TPK::Const(_) | TPK::Auto, TPK::Const(_) | TPK::Auto) => true,
            (TPK::Templ(_), TPK::Templ(_)) => true,
            _ => false,
        };
        let mut i = 0;
        let mut j = 0;
        while i < inner.len() {
            let a = &inner[i];
            if a.pack {
                // A pack in the parameter's head takes any number of
                // parameters of its kind.
                return ps[j.min(ps.len())..].iter().all(|q| same_kind(a, q) || q.default.is_some());
            }
            let Some(q) = ps.get(j) else { return false };
            if !same_kind(a, q) {
                return false;
            }
            if q.pack {
                return inner[i..].iter().all(|x| same_kind(x, q));
            }
            i += 1;
            j += 1;
        }
        // The argument's other parameters need defaults or a pack.
        ps[j.min(ps.len())..].iter().all(|q| q.default.is_some() || q.pack)
    }

    /// TPL-6: a structural type.
    pub fn structural(&self, t: &QTy) -> bool {
        match &t.t {
            Ty::LRef(_) => true,
            Ty::RRef(_) | Ty::Void => false,
            Ty::Class(c) => {
                let ci = &self.classes[*c];
                ci.complete
                    && ci.bases.iter().all(|b| b.access == 0)
                    && ci.fields.iter().all(|&f| match &self.decls[f].k {
                        DK::Field(fi) => !fi.mutable && self.member_access.get(&f).copied().unwrap_or(0) == 0 && self.structural(&fi.ty),
                        _ => true,
                    })
            }
            Ty::Array(e, _) => self.structural(e),
            Ty::Func(_) => false,
            _ => true,
        }
    }

    /// The value of a constant template argument for a parameter of type
    /// `pty` (TARG-12 to TARG-16). `quiet`: no diagnostics.
    pub fn const_targ(&mut self, e: Expr, pty: &QTy, line: u32, quiet: bool) -> Result<TVal, bool> {
        let line = if line == 0 { e.line } else { line };
        if quiet {
            self.d.muted += 1;
        }
        let r = self.const_targ_in(e, pty, line);
        if quiet {
            self.d.muted -= 1;
        }
        r
    }

    fn const_targ_in(&mut self, e: Expr, pty: &QTy, line: u32) -> Result<TVal, bool> {
        let pt = pty.unqual();
        let et = nonref(&e.ty);
        let bad = |p: &mut P, rule: &'static str, msg: &str| -> Result<TVal, bool> {
            p.error(line, rule, msg.to_string());
            Err(true)
        };
        if pt.t.is_integral() || matches!(pt.t, Ty::Enum(_)) {
            let e = match &et.t {
                Ty::Class(_) => match self.implicit(e, &pt, false) {
                    Ok(x) => x,
                    Err(_) => return bad(self, "TARG-13", "the argument does not convert to the parameter's type"),
                },
                t if t.is_floating() => return bad(self, "TARG-13", "a floating argument for an integer parameter"),
                Ty::Enum(x) if self.enums[*x].scoped && pt.t != et.t => return bad(self, "TARG-13", "a scoped enumeration argument needs an explicit conversion"),
                Ty::Enum(_) if matches!(pt.t, Ty::Enum(_)) && pt.t != et.t => return bad(self, "TARG-13", "the argument does not convert to the enumeration type"),
                t if matches!(pt.t, Ty::Enum(_)) && pt.t != *t => return bad(self, "TARG-13", "the argument does not convert to the enumeration type"),
                t if t.is_integral() || matches!(t, Ty::Enum(_)) => e,
                Ty::Unknown | Ty::Dep(_) => return Err(false),
                _ => return bad(self, "TARG-13", "the argument does not convert to the parameter's type"),
            };
            let v = match self.const_int(&e) {
                Ok(v) => v,
                Err(crate::consteval::CeErr::NotConst(..)) => return bad(self, "TARG-13", "the template argument is not a constant expression"),
                Err(_) => return Err(false),
            };
            let target = match &pt.t {
                Ty::Enum(x) => self.enum_underlying(*x),
                t => t.clone(),
            };
            let src_enum = matches!(nonref(&e.ty).t, Ty::Enum(_));
            if !fits(v, &target) && !(src_enum && matches!(pt.t, Ty::Enum(_))) {
                return bad(self, "TARG-13", "the conversion of the template argument narrows");
            }
            return Ok(TVal::Const(pt, CVal::Int(v)));
        }
        if pt.t.is_floating() {
            if !et.t.is_floating() {
                if matches!(et.t, Ty::Unknown | Ty::Dep(_)) {
                    return Err(false);
                }
                return bad(self, "TARG-13", "an integer argument for a floating-point parameter");
            }
            let Some(v) = self.const_float_quiet(&e) else {
                return bad(self, "TARG-13", "the template argument is not a constant expression");
            };
            let v = if pt.t == Ty::Float { v as f32 as f64 } else { v };
            return Ok(TVal::Const(pt, CVal::Float(v.to_bits())));
        }
        if pt.t == Ty::NullPtr {
            if et.t == Ty::NullPtr {
                return Ok(TVal::Const(pt, CVal::Null));
            }
            return bad(self, "TARG-13", "the argument is not a null pointer");
        }
        if let Ty::Ptr(pointee) = &pt.t {
            let mut x = &e;
            while let EK::Conv(Conv::NoOp | Conv::PtrConv | Conv::LToR, inner) = &x.k {
                if matches!(x.k, EK::Conv(Conv::LToR, _)) && !matches!(inner.ty.t, Ty::Array(..)) {
                    break;
                }
                x = inner;
            }
            if et.t == Ty::NullPtr {
                return Ok(TVal::Const(pt, CVal::Null));
            }
            if let EK::Int(_) = x.k {
                return bad(self, "TARG-15", "an integer is not a pointer argument");
            }
            let (target, kind): (Option<DeclId>, u8) = match &x.k {
                EK::Str(_) => return bad(self, "TARG-15", "a string literal is not a valid pointer argument"),
                EK::Unary(UOp::AddrOf, y) => match &y.k {
                    EK::Var(d) => (Some(*d), 0),
                    EK::Func(d) => (Some(*d), 0),
                    EK::Str(_) => return bad(self, "TARG-15", "the address of a string literal is not a valid pointer argument"),
                    EK::Index(a, i) => match (&strip_conv(a).k, &i.k) {
                        (EK::Var(d), EK::Int(k)) if *k >= 0 => (Some(*d), (*k as u8).saturating_add(1)),
                        _ => (None, 0),
                    },
                    _ => (None, 0),
                },
                EK::Conv(Conv::ArrayDecay, y) => match &y.k {
                    EK::Var(d) => (Some(*d), 1),
                    EK::Str(_) => return bad(self, "TARG-15", "a string literal is not a valid pointer argument"),
                    _ => (None, 0),
                },
                EK::Conv(Conv::FuncDecay, y) => match &y.k {
                    EK::Func(d) => (Some(*d), 0),
                    _ => (None, 0),
                },
                EK::Func(d) => (Some(*d), 0),
                EK::Overloads(fs) => {
                    // TARG-12: the overload of the parameter's type.
                    let want = (**pointee).clone();
                    match fs.iter().copied().find(|&f| self.func(f).is_some_and(|fi| QTy::new(Ty::Func(Box::new(fi.fty.clone()))).t == want.t)) {
                        Some(f) => (Some(f), 0),
                        None => return bad(self, "TARG-12", "no overload has the parameter's type"),
                    }
                }
                EK::Var(d) if matches!(self.var(*d).map(|v| &v.ty.t), Some(Ty::Array(..))) => (Some(*d), 1),
                _ => (None, 0),
            };
            let Some(d) = target else {
                if e.ty.is_unknown() {
                    return Err(false);
                }
                // A constant expression of pointer type: the entity it
                // points to.
                return match self.const_addr(&e) {
                    Ok(None) => Ok(TVal::Const(pt, CVal::Null)),
                    Ok(Some((d, k))) => Ok(TVal::Const(pt, CVal::Addr(d, k))),
                    Err(crate::consteval::CeErr::NotConst(..)) => bad(self, "TARG-13", "the template argument is not a constant expression"),
                    _ => Err(false),
                };
            };
            if let Some(v) = self.var(d) {
                if self.decls[d].name == "__func__" {
                    return bad(self, "TARG-15", "the address of '__func__' is not a valid pointer argument");
                }
                if v.local && v.storage == Storage::Auto {
                    return bad(self, "TARG-15", "the address of an object with automatic storage duration is not a constant");
                }
                let vt = v.ty.clone();
                let obj = if kind >= 1 {
                    match &vt.t {
                        Ty::Array(el, _) => (**el).clone(),
                        _ => vt.clone(),
                    }
                } else {
                    vt.clone()
                };
                // The conversion is judged on the argument's own pointer
                // type: an explicit cast is the program's own.
                let obj = match &et.t {
                    Ty::Ptr(x) => (**x).clone(),
                    Ty::Array(el, _) => (**el).clone(),
                    _ => obj,
                };
                if pointee.t == Ty::Void && obj.t != Ty::Void {
                    return bad(self, "TARG-13", "a pointer argument converts to a pointer to void");
                }
                if let (Ty::Class(a), Ty::Class(b)) = (&obj.t, &pointee.t) {
                    if a != b {
                        return bad(self, "TARG-13", "a pointer argument converts to a pointer to a base class");
                    }
                }
                if obj.t != pointee.t || (obj.c && !pointee.c) || (obj.v && !pointee.v) {
                    return bad(self, "TARG-13", "the pointer argument does not convert to the parameter's type");
                }
                return Ok(TVal::Const(pt, CVal::Addr(d, kind)));
            }
            if let Some(f) = self.func(d) {
                let ft = Ty::Func(Box::new(f.fty.clone()));
                let ok = match (&ft, &pointee.t) {
                    (Ty::Func(a), Ty::Func(b)) => a.params == b.params && a.ret == b.ret && (a.noexcept || !b.noexcept),
                    _ => false,
                };
                if !ok {
                    return bad(self, "TARG-13", "the function does not have the parameter's type");
                }
                return Ok(TVal::Const(pt, CVal::Addr(d, 0)));
            }
            return Err(false);
        }
        if let Ty::LRef(referred) = &pt.t {
            if e.cat == Cat::PR {
                return bad(self, "TARG-15", "the argument would bind the reference to a temporary object");
            }
            let mut x = &e;
            while let EK::Conv(Conv::NoOp | Conv::Bind, inner) = &x.k {
                x = inner;
            }
            let mut d = match &x.k {
                EK::Var(d) => *d,
                EK::Func(d) => *d,
                _ => return Err(false),
            };
            if self.var(d).is_some_and(|v| v.ty.t.is_ref()) {
                // A reference names the entity that it is bound to.
                let addr = Expr::new(EK::Unary(UOp::AddrOf, Box::new(x.clone())), QTy::new(Ty::Ptr(Box::new(nonref(&x.ty)))), Cat::PR, line);
                match self.const_addr(&addr) {
                    Ok(Some((t, 0))) => d = t,
                    _ => return Err(false),
                }
            }
            if let Some(v) = self.var(d) {
                if v.local && v.storage == Storage::Auto {
                    return bad(self, "TARG-15", "a reference argument names an object with automatic storage duration");
                }
                if v.ty.t != referred.t || (v.ty.c && !referred.c) {
                    if referred.c && v.ty.t != referred.t {
                        return bad(self, "TARG-15", "the argument would bind the reference to a temporary object");
                    }
                    return bad(self, "TARG-13", "the argument does not have the referred type");
                }
            }
            return Ok(TVal::Const(pt, CVal::Addr(d, 0)));
        }
        if let Ty::Class(c) = &pt.t {
            // TARG-16: copy-initialization of the parameter's object.
            let c = *c;
            let mut ct = pt.clone();
            ct.c = true;
            let form = if matches!(e.k, EK::InitList(_)) { InitForm::CopyList } else { InitForm::Copy };
            let ce = self.convert_init(e, &ct, form, line);
            if ce.ty.is_unknown() || has_unknown(&ce) {
                return Err(false);
            }
            let v = match self.const_eval(&ce) {
                Ok(v) => v,
                Err(crate::consteval::CeErr::NotConst(r, m)) => {
                    if r == "CE-14" || m.contains("string literal") {
                        return bad(self, "TARG-15", "a member of the class argument points to an object that is not valid");
                    }
                    return bad(self, "TARG-13", "the template argument is not a constant expression");
                }
                Err(_) => return Err(false),
            };
            if !self.class_value_ok(&v) {
                return bad(self, "TARG-15", "a member of the class argument points to a temporary or a string literal");
            }
            let key = format!("{}:{:?}", c, v);
            let d = match self.class_targ_objs.iter().find(|x| x.0 == key) {
                Some(x) => x.1,
                None => {
                    let vi = VarInfo { ty: ct.clone(), storage: Storage::Static, constexpr: true, constinit: false, init: Some(Rc::new(ce)), param: false, local: false, dependent: false, defined: true, class: None };
                    let g = self.global;
                    let d = self.add_decl_in(g, "", DK::Var(vi), line);
                    self.class_targ_objs.push((key, d));
                    d
                }
            };
            return Ok(TVal::Const(pt, CVal::Obj(d)));
        }
        Err(false)
    }

    fn class_value_ok(&self, v: &crate::consteval::Val) -> bool {
        let s = format!("{:?}", v);
        !s.contains("Str")
    }

    /// The expression for the value of a constant parameter.
    pub fn tval_expr(&self, ty: &QTy, cv: &CVal, line: u32) -> Expr {
        match cv {
            CVal::Int(v) => Expr::new(EK::Int(*v), ty.unqual(), Cat::PR, line),
            CVal::Float(b) => Expr::new(EK::Float(f64::from_bits(*b)), ty.unqual(), Cat::PR, line),
            CVal::Null => {
                let n = Expr::new(EK::NullPtr, QTy::new(Ty::NullPtr), Cat::PR, line);
                if ty.t == Ty::NullPtr {
                    n
                } else {
                    Expr::new(EK::Conv(Conv::NullPtr, Box::new(n)), ty.unqual(), Cat::PR, line)
                }
            }
            CVal::Addr(d, k) => {
                let d = *d;
                let obj = match &self.decls[d].k {
                    DK::Var(v) => Expr::new(EK::Var(d), nonref(&v.ty), Cat::L, line),
                    DK::Func(f) => Expr::new(EK::Func(d), QTy::new(Ty::Func(Box::new(f.fty.clone()))), Cat::L, line),
                    _ => Expr::unknown(line, "template argument"),
                };
                match &ty.t {
                    Ty::LRef(_) => {
                        // The lvalue that the parameter names (TPL-9).
                        let t = obj.ty.clone();
                        Expr::new(EK::Conv(Conv::Bind, Box::new(obj)), t, Cat::L, line)
                    }
                    Ty::Ptr(_) => {
                        if *k >= 1 {
                            let el = match &obj.ty.t {
                                Ty::Array(e, _) => (**e).clone(),
                                _ => QTy::unknown(),
                            };
                            let at = Expr::new(EK::Conv(Conv::ArrayDecay, Box::new(obj)), QTy::new(Ty::Ptr(Box::new(el.clone()))), Cat::PR, line);
                            let p = if *k == 1 {
                                at
                            } else {
                                let idx = Expr::new(EK::Int((*k - 1) as i128), QTy::new(Ty::ptrdiff_t()), Cat::PR, line);
                                Expr::new(EK::Binary(BOp::Add, Box::new(at), Box::new(idx)), QTy::new(Ty::Ptr(Box::new(el))), Cat::PR, line)
                            };
                            Expr::new(EK::Conv(Conv::NoOp, Box::new(p)), ty.unqual(), Cat::PR, line)
                        } else if matches!(obj.ty.t, Ty::Func(_)) {
                            let ft = obj.ty.clone();
                            Expr::new(EK::Conv(Conv::FuncDecay, Box::new(obj)), QTy::new(Ty::Ptr(Box::new(ft))), Cat::PR, line)
                        } else {
                            let ot = obj.ty.clone();
                            let a = Expr::new(EK::Unary(UOp::AddrOf, Box::new(obj)), QTy::new(Ty::Ptr(Box::new(ot))), Cat::PR, line);
                            Expr::new(EK::Conv(Conv::NoOp, Box::new(a)), ty.unqual(), Cat::PR, line)
                        }
                    }
                    _ => obj,
                }
            }
            CVal::Obj(d) => {
                let t = self.var(*d).map(|v| v.ty.clone()).unwrap_or(QTy::unknown());
                Expr::new(EK::Var(*d), t, Cat::L, line)
            }
        }
    }

    // ---- replay ----

    fn save_state(&mut self) -> Saved {
        Saved {
            toks: std::mem::take(&mut self.toks),
            pos: self.pos,
            stack: std::mem::take(&mut self.stack),
            fns: std::mem::take(&mut self.fns),
            templ: self.templ,
            class_depth: self.class_depth,
            deferred: std::mem::take(&mut self.deferred),
            pending_template: self.pending_template.take(),
            access_stack: std::mem::take(&mut self.access_stack),
            in_unevaluated: self.in_unevaluated,
            discarded: self.discarded,
            no_gt: self.no_gt,
            dmi_depth: self.dmi_depth,
            adl_pos: self.adl_pos.take(),
            call_adl: self.call_adl,
            decl_attrs: std::mem::take(&mut self.decl_attrs),
            pending_attrs: std::mem::take(&mut self.pending_attrs),
            tdepth: self.tdepth,
            inst: self.inst.take(),
            inst_out: self.inst_out.take(),
            explicit_spec: std::mem::take(&mut self.explicit_spec),
            last_templ: self.last_templ.take(),
            auto_in_own_init: self.auto_in_own_init.take(),
            in_base_clause: self.in_base_clause,
            inst_fresh: std::mem::take(&mut self.inst_fresh),
            last_align: self.last_align,
            saw_alignas: self.saw_alignas,
            align_unknown: self.align_unknown,
            saw_nua: self.saw_nua,
            call_no_object: self.call_no_object,
            in_inst_body: self.in_inst_body,
            member_replay: std::mem::take(&mut self.member_replay),
            lookup_limit: self.lookup_limit,
            tok_origin: self.tok_origin,
            abbrev: self.abbrev.take(),
            param_nest: std::mem::take(&mut self.param_nest),
            auto_param_k: std::mem::take(&mut self.auto_param_k),
            cons_head: self.cons_head.take(),
            head_cons: self.head_cons.take(),
            tparam_cons: self.tparam_cons.take(),
            head_parts: std::mem::take(&mut self.head_parts),
        }
    }

    fn restore_state(&mut self, s: Saved) {
        self.toks = s.toks;
        self.pos = s.pos;
        self.stack = s.stack;
        self.fns = s.fns;
        self.templ = s.templ;
        self.class_depth = s.class_depth;
        self.deferred = s.deferred;
        self.pending_template = s.pending_template;
        self.access_stack = s.access_stack;
        self.in_unevaluated = s.in_unevaluated;
        self.discarded = s.discarded;
        self.no_gt = s.no_gt;
        self.dmi_depth = s.dmi_depth;
        self.adl_pos = s.adl_pos;
        self.call_adl = s.call_adl;
        self.decl_attrs = s.decl_attrs;
        self.pending_attrs = s.pending_attrs;
        self.tdepth = s.tdepth;
        self.inst = s.inst;
        self.inst_out = s.inst_out;
        self.explicit_spec = s.explicit_spec;
        self.last_templ = s.last_templ;
        self.auto_in_own_init = s.auto_in_own_init;
        self.in_base_clause = s.in_base_clause;
        self.inst_fresh = s.inst_fresh;
        self.last_align = s.last_align;
        self.saw_alignas = s.saw_alignas;
        self.align_unknown = s.align_unknown;
        self.saw_nua = s.saw_nua;
        self.call_no_object = s.call_no_object;
        self.in_inst_body = s.in_inst_body;
        self.member_replay = s.member_replay;
        self.lookup_limit = s.lookup_limit;
        self.tok_origin = s.tok_origin;
        self.abbrev = s.abbrev;
        self.param_nest = s.param_nest;
        self.auto_param_k = s.auto_param_k;
        self.cons_head = s.cons_head;
        self.head_cons = s.head_cons;
        self.tparam_cons = s.tparam_cons;
        self.head_parts = s.head_parts;
    }

    /// Parse the tokens of `src` again with `params` bound to `args`.
    /// None when the instantiations nest too deep.
    pub fn replay<R>(&mut self, src: &Source, params: &[TParam], args: &[TArg], req: Option<InstReq>, f: impl FnOnce(&mut P) -> R) -> Option<(R, Option<InstOut>)> {
        if self.inst_depth >= 256 || self.inst_count >= 8000 {
            return None;
        }
        self.inst_depth += 1;
        self.inst_count += 1;
        let saved = self.save_state();
        self.toks = (*src.toks).clone();
        self.pos = 0;
        self.stack = src.stack.clone();
        if self.stack.is_empty() {
            self.stack.push(self.global);
        }
        let scope = self.push_scope(SK::Template);
        self.binding_scopes.insert(scope);
        self.bind_params(scope, params, args);
        self.access_stack = vec![src.access];
        self.lookup_limit = src.limit;
        self.tok_origin = (src.vid, src.base);
        self.templ = 0;
        self.class_depth = 0;
        self.tdepth = 0;
        self.inst = req;
        let r = f(self);
        let out = self.inst_out.take();
        self.restore_state(saved);
        self.inst_depth -= 1;
        Some((r, out))
    }

    /// Declare each parameter in `scope` as its argument.
    pub fn bind_params(&mut self, scope: usize, params: &[TParam], args: &[TArg]) {
        for (i, p) in params.iter().enumerate() {
            let Some(a) = args.get(i) else { continue };
            if p.name.is_empty() {
                continue;
            }
            let line = p.line;
            match (&p.kind, a) {
                (_, TArg::Pack(v)) => {
                    let id = match &p.kind {
                        TPK::Type => self.add_decl_in(scope, &p.name, DK::Type(QTy::unknown()), line),
                        _ => {
                            let vi = VarInfo { ty: QTy::unknown(), storage: Storage::Static, constexpr: true, constinit: false, init: None, param: true, local: false, dependent: true, defined: true, class: None };
                            self.add_decl_in(scope, &p.name, DK::Var(vi), line)
                        }
                    };
                    self.pack_binds.insert(id, v.clone());
                }
                (TPK::Type, TArg::Type(q)) => {
                    self.add_decl_in(scope, &p.name, DK::Type(q.clone()), line);
                }
                (TPK::Const(_) | TPK::Auto, TArg::Val(TVal::Const(t, cv))) => {
                    let e = self.tval_expr(t, cv, line);
                    let vt = if t.t.is_ref() { t.clone() } else { let mut x = t.clone(); x.c = true; x };
                    let vi = VarInfo { ty: vt, storage: Storage::Static, constexpr: true, constinit: false, init: Some(Rc::new(e.clone())), param: true, local: false, dependent: false, defined: true, class: None };
                    let id = self.add_decl_in(scope, &p.name, DK::Var(vi), line);
                    self.const_param_val.insert(id, e);
                }
                (TPK::Const(t), TArg::Val(TVal::Param(d, j))) => {
                    let vi = VarInfo { ty: t.clone(), storage: Storage::Auto, constexpr: false, constinit: false, init: None, param: true, local: false, dependent: true, defined: true, class: None };
                    let id = self.add_decl_in(scope, &p.name, DK::Var(vi), line);
                    self.tparam.insert(id, (*d, *j));
                }
                (TPK::Templ(inner), TArg::Templ(n)) => {
                    let id = self.add_decl_in(scope, &p.name, DK::Template(TemplKind::Class(usize::MAX)), line);
                    self.ttp_inner.insert(id, inner.clone());
                    match n {
                        TName::Templ(t) => {
                            self.decl_tid.insert(id, *t);
                        }
                        TName::Param(d, j) => {
                            self.tparam.insert(id, (*d, *j));
                        }
                    }
                }
                _ => {
                    let vi = VarInfo { ty: QTy::unknown(), storage: Storage::Auto, constexpr: false, constinit: false, init: None, param: true, local: false, dependent: true, defined: true, class: None };
                    self.add_decl_in(scope, &p.name, DK::Var(vi), line);
                }
            }
        }
    }

    // ---- class, variable, and alias templates ----

    /// The type that the template-id `tid<args>` names (TSP-13).
    pub fn class_inst(&mut self, tid: usize, args: Vec<TArg>, line: u32) -> QTy {
        if self.templs[tid].kind == TKind::Alias && !self.templs[tid].dependent && !args.iter().any(|a| matches!(a, TArg::Val(TVal::Opaque) | TArg::Expand(_))) {
            // TPL-28: an alias template gives its type, also as a pattern.
            return self.alias_inst(tid, args, line);
        }
        if args.iter().any(|a| a.is_dep()) {
            return dep(Dep::Id(TName::Templ(tid), args));
        }
        if self.templs[tid].dependent {
            return QTy::unknown();
        }
        match self.templs[tid].kind {
            TKind::Alias => return self.alias_inst(tid, args, line),
            TKind::Class => {}
            _ => return QTy::unknown(),
        }
        if let Some(x) = self.templs[tid].explicit.iter().find(|x| x.0 == args) {
            if let Inst::Class(c) = x.1 {
                return QTy::new(Ty::Class(c));
            }
            return QTy::unknown();
        }
        let cached = self.templs[tid].insts.iter().position(|x| x.0 == args);
        let mut existing = None;
        if let Some(k) = cached {
            match self.templs[tid].insts[k].1.clone() {
                Inst::Class(c) => {
                    if self.classes[c].complete || self.inst_busy.contains(&c) {
                        return QTy::new(Ty::Class(c));
                    }
                    // The template is defined now: complete the class.
                    existing = Some(c);
                }
                _ => return QTy::unknown(),
            }
        }
        if cached.is_none() && !self.cns13_ok(tid, &args, line) {
            self.templs[tid].insts.push((args, Inst::Fail, None));
            return QTy::unknown();
        }
        let sel = match self.select_partial(tid, &args, line) {
            Ok(s) => s,
            Err(()) => return QTy::unknown(),
        };
        let name = self.templs[tid].name.clone();
        let (src, params, bargs, pidx) = match sel {
            Some((k, b)) => {
                let pt = self.templs[tid].partials[k].clone();
                (pt.src, pt.params, b, Some(k))
            }
            None => (self.templs[tid].src.clone(), self.templs[tid].params.clone(), args.clone(), None),
        };
        let Some(src) = src else {
            if let Some(c) = existing {
                return QTy::new(Ty::Class(c));
            }
            // Declared and not defined: an incomplete class (TSP-20).
            let c = self.new_inst_class(tid, &args, line);
            self.templs[tid].insts.push((args, Inst::Class(c), pidx));
            return QTy::new(Ty::Class(c));
        };
        if cached.is_none() {
            self.templs[tid].insts.push((args.clone(), Inst::Busy, pidx));
        }
        if self.sfinae_mark.is_some_and(|m| self.d.muted_errors > m) {
            // DED-25: substitution stopped at its first failure, so no
            // later part instantiates a class.
            return QTy::unknown();
        }
        let req = InstReq::Class { name, tid, args: args.clone(), existing };
        let ctx = src.ctx;
        // DED-29: errors in the body of a class are errors of the program,
        // also during substitution.
        let saved_muted = std::mem::take(&mut self.d.muted);
        let saved_mark = self.sfinae_mark.take();
        let r = self.replay(&src, &params, &bargs, Some(req), |p| {
            let _ = p.parse_declaration(ctx);
        });
        self.sfinae_mark = saved_mark;
        self.d.muted = saved_muted;
        let k = self.templs[tid].insts.iter().position(|x| x.0 == args).unwrap();
        self.templs[tid].insts[k].2 = pidx;
        match r {
            Some((_, Some(InstOut::Class(c)))) => {
                self.templs[tid].insts[k].1 = Inst::Class(c);
                self.select_dtor(c, line);
                self.replay_members(tid, c, &args, pidx);
                QTy::new(Ty::Class(c))
            }
            None => {
                self.templs[tid].insts[k].1 = Inst::Fail;
                // INST-14: the nesting of instantiations has a limit.
                if !self.depth_reported {
                    self.depth_reported = true;
                    self.error(line, "INST-14", format!("instantiation of '{}<{}>' nests deeper than the limit of 256", self.templs[tid].name, self.show_targs(&args)));
                }
                QTy::unknown()
            }
            _ => {
                self.templs[tid].insts[k].1 = Inst::Fail;
                self.unsupported(line, format!("instantiation of '{}<{}>'", self.templs[tid].name, self.show_targs(&args)));
                QTy::unknown()
            }
        }
    }

    /// The class that `tid<args>` names, made complete only when a use
    /// needs it (INST-1, INST-2).
    pub fn class_ref(&mut self, tid: usize, args: Vec<TArg>, line: u32) -> QTy {
        if args.iter().any(|a| a.is_dep()) || self.templs[tid].dependent || self.templs[tid].kind != TKind::Class {
            return self.class_inst(tid, args, line);
        }
        if let Some(x) = self.templs[tid].explicit.iter().find(|x| x.0 == args) {
            if let Inst::Class(c) = x.1 {
                return QTy::new(Ty::Class(c));
            }
            return QTy::unknown();
        }
        if let Some(x) = self.templs[tid].insts.iter().find(|x| x.0 == args) {
            return match x.1 {
                Inst::Class(c) => QTy::new(Ty::Class(c)),
                _ => QTy::unknown(),
            };
        }
        if !self.cns13_ok(tid, &args, line) {
            self.templs[tid].insts.push((args, Inst::Fail, None));
            return QTy::unknown();
        }
        let c = self.new_inst_class(tid, &args, line);
        self.templs[tid].insts.push((args, Inst::Class(c), None));
        QTy::new(Ty::Class(c))
    }

    /// INST-1: a use that needs the class complete instantiates it.
    pub fn require_class(&mut self, c: usize, line: u32) {
        if self.classes[c].complete || self.inst_busy.contains(&c) {
            return;
        }
        let Some((t, args)) = self.inst_of.get(&c).cloned() else { return };
        if self.templs[t].explicit.iter().any(|x| x.1 == Inst::Class(c)) {
            return;
        }
        self.class_inst(t, args, line);
    }

    /// INST-1 for a type: the class, or the class of an array element.
    pub fn require_ty(&mut self, q: &QTy, line: u32) {
        match &q.t {
            Ty::Class(c) => self.require_class(*c, line),
            Ty::Array(e, _) => self.require_ty(e, line),
            _ => {}
        }
    }

    /// INST-1 for a use through a pointer or reference: the class it
    /// points or refers to.
    pub fn require_pointee(&mut self, q: &QTy, line: u32) {
        match &q.t {
            Ty::Ptr(x) | Ty::LRef(x) | Ty::RRef(x) => self.require_ty(x, line),
            _ => self.require_ty(q, line),
        }
    }

    /// Whether a class is an instance or is nested in one.
    pub fn in_instance(&self, c: usize) -> bool {
        if self.inst_of.contains_key(&c) {
            return true;
        }
        let mut s = self.scopes[self.classes[c].scope].parent;
        while let Some(x) = s {
            if let SK::Class(o) = self.scopes[x].kind {
                if self.inst_of.contains_key(&o) {
                    return true;
                }
            }
            s = self.scopes[x].parent;
        }
        false
    }

    /// A class for a specialization, not yet defined.
    pub fn new_inst_class(&mut self, tid: usize, args: &[TArg], line: u32) -> usize {
        let name = self.templs[tid].name.clone();
        let parent = self.templs[tid].scope;
        let key = match self.decls[self.templs[tid].decl].k {
            DK::Template(TemplKind::Class(pc)) if pc != usize::MAX => self.classes[pc].key,
            _ => 0,
        };
        let scope = self.new_scope(SK::Class(self.classes.len()), Some(parent), &name);
        self.classes.push(ClassInfo {
            name: name.clone(),
            key,
            scope,
            complete: false,
            fields: vec![],
            bases: vec![],
            has_conv: false,
            conv_targets: vec![],
            ctors: vec![],
            dtor: None,
            has_virtual: false,
            template: false,
            opaque: false,
            line,
            align: 0,
            anon_unions: vec![],
        });
        let c = self.classes.len() - 1;
        self.inst_of.insert(c, (tid, args.to_vec()));
        self.class_label.insert(c, format!("{}<{}>", name, self.show_targs(args)));
        c
    }

    /// The members defined outside the class template, for one instance.
    fn replay_members(&mut self, tid: usize, c: usize, args: &[TArg], pidx: Option<usize>) {
        let members = self.templs[tid].members.clone();
        for m in members {
            self.replay_member(&m, c, args, pidx);
        }
    }

    pub fn replay_member(&mut self, m: &Member, c: usize, args: &[TArg], pidx: Option<usize>) {
        if m.partial != pidx || !self.classes[c].complete {
            return;
        }
        let mut b = Binds::new(m.depth, &m.params);
        if m.pats.len() != args.len() || !m.pats.iter().zip(args.iter()).all(|(p, a)| self.deduce_targ(p, a, &mut b)) {
            return;
        }
        let vals: Vec<TArg> = b.vals.iter().map(|v| v.clone().unwrap_or(TArg::Pack(vec![]))).collect();
        let ctx = m.src.ctx;
        let mark = self.d.list.len();
        let r = self.replay(&m.src, &m.params, &vals, None, |p| {
            p.member_replay = true;
            p.member_replay_decl = None;
            let _ = p.parse_declaration(ctx);
            p.member_replay = false;
            p.member_replay_decl.take()
        });
        // INST-5, INST-8: the definition waits for a use; so do its errors.
        if self.d.list.len() > mark {
            if let Some((Some(d), _)) = r {
                let kept: Vec<crate::diag::Diag> = self.d.list.drain(mark..).collect();
                self.body_errors.entry(d).or_default().extend(kept);
            }
        }
    }

    /// TSP-14 to TSP-16: the partial specialization that matches, with
    /// the values of its parameters. Err after an ambiguity.
    pub fn select_partial(&mut self, tid: usize, args: &[TArg], line: u32) -> Result<Option<(usize, Vec<TArg>)>, ()> {
        let parts = self.templs[tid].partials.clone();
        if parts.iter().any(|p| p.constrained) {
            // Constraints that the front end does not model.
            return Err(());
        }
        let mut found: Vec<(usize, Vec<TArg>)> = Vec::new();
        for (k, pt) in parts.iter().enumerate() {
            if pt.pats.len() != args.len() {
                continue;
            }
            let mut b = Binds::new(pt.depth, &pt.params);
            if pt.pats.iter().zip(args.iter()).all(|(p, a)| self.deduce_targ(p, a, &mut b)) {
                if b.unknown {
                    return Err(());
                }
                let mut vals = Vec::new();
                let mut ok = true;
                for (i, v) in b.vals.iter().enumerate() {
                    match v {
                        Some(v) => vals.push(v.clone()),
                        None if pt.params[i].pack => vals.push(TArg::Pack(vec![])),
                        None => ok = false,
                    }
                }
                if ok {
                    // CNS-17: the constraints must be satisfied.
                    if let Some(c) = pt.cons.clone() {
                        match self.cons_check(&c, pt.depth, &vals) {
                            crate::cons::Sat::Yes => {}
                            crate::cons::Sat::Unknown => return Err(()),
                            _ => continue,
                        }
                    }
                    found.push((k, vals));
                }
            }
        }
        if found.len() <= 1 {
            return Ok(found.pop());
        }
        for i in 0..found.len() {
            let pi = parts[found[i].0].clone();
            let best = (0..found.len()).all(|j| {
                if i == j {
                    return true;
                }
                let pj = parts[found[j].0].clone();
                self.partial_order(&pi, &pj) && !self.partial_order(&pj, &pi)
            });
            if best {
                return Ok(Some(found.swap_remove(i)));
            }
        }
        let name = self.templs[tid].name.clone();
        self.error(line, "TSP-16", format!("more than one partial specialization of '{}' matches, and none is more specialized than the others", name));
        Err(())
    }

    /// ORD-14: `a` is at least as specialized as `b`, with the
    /// constraints when the patterns are equivalent (CNS-P4).
    pub fn partial_order(&mut self, a: &Partial, b: &Partial) -> bool {
        if !self.partial_at_least(&a.params, &a.pats, a.depth, &b.params, &b.pats, b.depth) {
            return false;
        }
        if !self.partial_at_least(&b.params, &b.pats, b.depth, &a.params, &a.pats, a.depth) {
            return true;
        }
        if a.cons.is_none() && b.cons.is_none() {
            return true;
        }
        // The parameters of `b` in terms of the invented ones of `a`.
        let inv: Vec<TArg> = a.pats.iter().map(|x| self.invent_targ(x, a.depth)).collect();
        let mut bb = Binds::new(b.depth, &b.params);
        if !b.pats.iter().zip(inv.iter()).all(|(p, x)| self.deduce_targ(p, x, &mut bb)) {
            return false;
        }
        let bargs: Vec<TArg> = bb.vals.iter().map(|v| v.clone().unwrap_or(TArg::Pack(vec![]))).collect();
        let aargs: Vec<TArg> = a.params.iter().enumerate().map(|(i, p)| {
            let x = match &p.kind {
                TPK::Type => TArg::Type(dep(Dep::Param(INV, i as u32))),
                TPK::Templ(_) => TArg::Templ(TName::Param(INV, i as u32)),
                _ => TArg::Val(TVal::Param(INV, i as u32)),
            };
            if p.pack { TArg::Expand(Box::new(x)) } else { x }
        }).collect();
        self.at_least_constrained(&a.cons, a.depth, &aargs, &b.cons, b.depth, &bargs)
    }

    /// ORD-14: the partial specialization `a` is at least as specialized
    /// as `b`: `b`'s arguments deduce from `a`'s with invented values.
    pub fn partial_at_least(&mut self, ap: &[TParam], apats: &[TArg], ad: u32, bp: &[TParam], bpats: &[TArg], bd: u32) -> bool {
        let _ = ap;
        let inv: Vec<TArg> = apats.iter().map(|x| self.invent_targ(x, ad)).collect();
        if inv.len() != bpats.len() {
            return false;
        }
        let mut b = Binds::new(bd, bp);
        bpats.iter().zip(inv.iter()).all(|(p, a)| self.deduce_targ(p, a, &mut b))
    }

    /// Replace the parameters of depth `d` by invented ones (ORD-1).
    pub fn invent_ty(&mut self, q: &QTy, d: u32) -> QTy {
        let n = Binds { depth: d, vals: vec![], kinds: vec![], patterns: vec![], packs: vec![], unknown: false, fixed: vec![], from_bound: false };
        self.subst_inv(q, &n)
    }

    pub fn invent_targ(&mut self, a: &TArg, d: u32) -> TArg {
        let n = Binds { depth: d, vals: vec![], kinds: vec![], patterns: vec![], packs: vec![], unknown: false, fixed: vec![], from_bound: false };
        self.subst_targ_inv(a, &n)
    }

    fn subst_inv(&mut self, q: &QTy, b: &Binds) -> QTy {
        self.subst_gen(q, b, true)
    }

    fn subst_targ_inv(&mut self, a: &TArg, b: &Binds) -> TArg {
        self.subst_targ_gen(a, b, true)
    }

    /// Put the values of `b` into a pattern type. Parts with no value
    /// stay as they are.
    pub fn subst_ty(&mut self, q: &QTy, b: &Binds) -> QTy {
        self.subst_gen(q, b, false)
    }

    pub fn subst_targ(&mut self, a: &TArg, b: &Binds) -> TArg {
        self.subst_targ_gen(a, b, false)
    }

    fn subst_gen(&mut self, q: &QTy, b: &Binds, inv: bool) -> QTy {
        let t = match &q.t {
            Ty::Dep(d) => match &**d {
                Dep::Param(dd, i) if *dd == b.depth => {
                    if inv {
                        dep(Dep::Param(INV, *i))
                    } else {
                        match b.vals.get(*i as usize) {
                            Some(Some(TArg::Type(t))) => t.clone(),
                            _ => return q.clone(),
                        }
                    }
                }
                Dep::Id(n, args) => {
                    let n2 = match n {
                        TName::Param(dd, i) if *dd == b.depth => {
                            if inv {
                                TName::Param(INV, *i)
                            } else {
                                match b.vals.get(*i as usize) {
                                    Some(Some(TArg::Templ(x))) => x.clone(),
                                    _ => n.clone(),
                                }
                            }
                        }
                        _ => n.clone(),
                    };
                    let a2: Vec<TArg> = args.iter().map(|a| self.subst_targ_gen(a, b, inv)).collect();
                    match &n2 {
                        TName::Templ(t) if !a2.iter().any(|a| a.is_dep()) && !inv => {
                            let t = *t;
                            let r = self.class_inst(t, a2, 0);
                            QTy { t: r.t.clone(), c: q.c || r.c, v: q.v || r.v }
                        }
                        _ => dep(Dep::Id(n2, a2)),
                    }
                }
                Dep::Array(el, bound) => {
                    let el2 = self.subst_gen(el, b, inv);
                    let bound2 = self.subst_val(bound, b, inv);
                    match &bound2 {
                        TVal::Const(_, CVal::Int(n)) if !el2.is_unknown() && *n > 0 => QTy::new(Ty::Array(Box::new(el2), Some(*n as u64))),
                        _ => dep(Dep::Array(el2, bound2)),
                    }
                }
                Dep::MemPtr(c, m) => {
                    let c2 = self.subst_gen(c, b, inv);
                    let m2 = self.subst_gen(m, b, inv);
                    match &c2.t {
                        Ty::Class(cc) if !m2.is_unknown() => QTy::new(Ty::MemPtr(*cc, Box::new(m2))),
                        _ => dep(Dep::MemPtr(c2, m2)),
                    }
                }
                _ => return q.clone(),
            },
            Ty::Ptr(x) => QTy::new(Ty::Ptr(Box::new(self.subst_gen(x, b, inv)))),
            Ty::LRef(x) => {
                let x2 = self.subst_gen(x, b, inv);
                // DCL-4: a reference to a reference collapses.
                match &x2.t {
                    Ty::LRef(_) => x2,
                    Ty::RRef(i) => QTy::new(Ty::LRef(i.clone())),
                    _ => QTy::new(Ty::LRef(Box::new(x2))),
                }
            }
            Ty::RRef(x) => {
                let x2 = self.subst_gen(x, b, inv);
                match &x2.t {
                    Ty::LRef(_) | Ty::RRef(_) => x2,
                    _ => QTy::new(Ty::RRef(Box::new(x2))),
                }
            }
            Ty::Array(x, n) => {
                let x2 = self.subst_gen(x, b, inv);
                QTy { c: x2.c, v: x2.v, t: Ty::Array(Box::new(x2), *n) }
            }
            Ty::MemPtr(c, x) => QTy::new(Ty::MemPtr(*c, Box::new(self.subst_gen(x, b, inv)))),
            Ty::Func(f) => {
                let mut f2 = (**f).clone();
                f2.ret = self.subst_gen(&f.ret, b, inv);
                f2.params = f.params.iter().map(|p| self.subst_gen(p, b, inv)).collect();
                QTy::new(Ty::Func(Box::new(f2)))
            }
            _ => return q.clone(),
        };
        add_cv(&t, q.c, q.v)
    }

    fn subst_val(&mut self, v: &TVal, b: &Binds, inv: bool) -> TVal {
        match v {
            TVal::Param(d, i) if *d == b.depth => {
                if inv {
                    TVal::Param(INV, *i)
                } else {
                    match b.vals.get(*i as usize) {
                        Some(Some(TArg::Val(x))) => x.clone(),
                        _ => v.clone(),
                    }
                }
            }
            _ => v.clone(),
        }
    }

    fn subst_targ_gen(&mut self, a: &TArg, b: &Binds, inv: bool) -> TArg {
        match a {
            TArg::Type(q) => TArg::Type(self.subst_gen(q, b, inv)),
            TArg::Val(v) => TArg::Val(self.subst_val(v, b, inv)),
            TArg::Templ(TName::Param(d, i)) if *d == b.depth => {
                if inv {
                    TArg::Templ(TName::Param(INV, *i))
                } else {
                    match b.vals.get(*i as usize) {
                        Some(Some(TArg::Templ(x))) => TArg::Templ(x.clone()),
                        _ => a.clone(),
                    }
                }
            }
            TArg::Pack(v) => TArg::Pack(v.iter().map(|x| self.subst_targ_gen(x, b, inv)).collect()),
            TArg::Expand(x) => {
                if !inv {
                    if let TArg::Type(q) = &**x {
                        if let Ty::Dep(d) = &q.t {
                            if let Dep::Param(dd, i) = &**d {
                                if *dd == b.depth {
                                    if let Some(Some(TArg::Pack(v))) = b.vals.get(*i as usize) {
                                        return TArg::Pack(v.clone());
                                    }
                                }
                            }
                        }
                    }
                }
                TArg::Expand(Box::new(self.subst_targ_gen(x, b, inv)))
            }
            _ => a.clone(),
        }
    }

    /// The variable that `tid<args>` names (TSP-13, TPL-26).
    pub fn var_inst(&mut self, tid: usize, args: Vec<TArg>, line: u32) -> Option<DeclId> {
        if args.iter().any(|a| a.is_dep()) || self.templs[tid].dependent {
            return None;
        }
        if let Some(x) = self.templs[tid].explicit.iter().find(|x| x.0 == args) {
            return match x.1 {
                Inst::Var(d) => Some(d),
                _ => None,
            };
        }
        if let Some(x) = self.templs[tid].insts.iter().find(|x| x.0 == args) {
            return match x.1 {
                Inst::Var(d) => Some(d),
                _ => None,
            };
        }
        if !self.cns13_ok(tid, &args, line) {
            self.templs[tid].insts.push((args, Inst::Fail, None));
            return None;
        }
        let sel = self.select_partial(tid, &args, line).ok()?;
        let name = self.templs[tid].name.clone();
        let (src, params, bargs, pidx) = match sel {
            Some((k, b)) => {
                let pt = self.templs[tid].partials[k].clone();
                (pt.src, pt.params, b, Some(k))
            }
            None => (self.templs[tid].src.clone(), self.templs[tid].params.clone(), args.clone(), None),
        };
        let src = src?;
        self.templs[tid].insts.push((args.clone(), Inst::Busy, pidx));
        let ctx = src.ctx;
        let mark = self.d.list.len();
        let r = self.replay(&src, &params, &bargs, Some(InstReq::Var { name }), |p| {
            let _ = p.parse_declaration(ctx);
        });
        let k = self.templs[tid].insts.iter().position(|x| x.0 == args).unwrap();
        match r {
            Some((_, Some(InstOut::Var(d)))) => {
                // INST-8: the initializer's errors wait for a use.
                if self.d.list.len() > mark {
                    let kept: Vec<crate::diag::Diag> = self.d.list.drain(mark..).collect();
                    self.body_errors.insert(d, kept);
                }
                self.templs[tid].insts[k].1 = Inst::Var(d);
                self.spec_of.insert(d, (tid, args.clone()));
                Some(d)
            }
            _ => {
                self.templs[tid].insts[k].1 = Inst::Fail;
                None
            }
        }
    }

    /// The type that an alias template gives (TPL-28).
    pub fn alias_inst(&mut self, tid: usize, args: Vec<TArg>, line: u32) -> QTy {
        if let Some(x) = self.templs[tid].insts.iter().find(|x| x.0 == args) {
            return match &x.1 {
                Inst::Type(q) => q.clone(),
                _ => QTy::unknown(),
            };
        }
        let Some(src) = self.templs[tid].src.clone() else { return QTy::unknown() };
        let params = self.templs[tid].params.clone();
        if !self.cns13_ok(tid, &args, line) {
            self.templs[tid].insts.push((args, Inst::Fail, None));
            return QTy::unknown();
        }
        self.templs[tid].insts.push((args.clone(), Inst::Busy, None));
        let ctx = src.ctx;
        let r = self.replay(&src, &params, &args, Some(InstReq::Alias), |p| {
            let _ = p.parse_declaration(ctx);
        });
        let k = self.templs[tid].insts.iter().position(|x| x.0 == args).unwrap();
        match r {
            Some((_, Some(InstOut::Type(q)))) => {
                self.templs[tid].insts[k].1 = Inst::Type(q.clone());
                q
            }
            _ => {
                self.templs[tid].insts[k].1 = Inst::Fail;
                let _ = line;
                QTy::unknown()
            }
        }
    }

    // ---- deduction (DED-P3) ----

    /// Match a pattern argument with an argument (TSP-14, DED-10).
    pub fn deduce_targ(&mut self, p: &TArg, a: &TArg, b: &mut Binds) -> bool {
        match (p, a) {
            (TArg::Type(pq), TArg::Type(aq)) => self.deduce_ty(pq, aq, b),
            (TArg::Val(pv), TArg::Val(av)) => self.deduce_val(pv, av, b),
            (TArg::Templ(pn), TArg::Templ(an)) => self.deduce_tname(pn, an, b),
            (TArg::Pack(ps), TArg::Pack(xs)) => {
                // DED-37: a trailing expansion takes the rest.
                if let Some(TArg::Expand(pat)) = ps.last() {
                    let n = ps.len() - 1;
                    if xs.len() < n {
                        return false;
                    }
                    return ps[..n].iter().zip(xs.iter()).all(|(p, a)| self.deduce_targ(p, a, b)) && self.deduce_expand(pat, &xs[n..], b);
                }
                ps.len() == xs.len() && ps.iter().zip(xs.iter()).all(|(p, a)| self.deduce_targ(p, a, b))
            }
            (TArg::Expand(pat), TArg::Pack(xs)) => self.deduce_expand(pat, xs, b),
            (TArg::Val(TVal::Opaque), _) => true,
            _ => false,
        }
    }

    /// DED-37: a pack expansion takes the elements one by one.
    fn deduce_expand(&mut self, pat: &TArg, xs: &[TArg], b: &mut Binds) -> bool {
        let idx = match pat {
            TArg::Type(q) => match &q.t {
                Ty::Dep(d) => match &**d {
                    Dep::Param(dd, i) if *dd == b.depth => Some(*i),
                    _ => None,
                },
                _ => None,
            },
            TArg::Val(TVal::Param(dd, i)) if *dd == b.depth => Some(*i),
            _ => None,
        };
        match idx {
            Some(i) => b.bind(i, TArg::Pack(xs.to_vec())),
            None => {
                b.unknown = true;
                true
            }
        }
    }

    /// DED-11, DED-22: the value that an array bound gives, with the type
    /// of the constant parameter, or the type of `sizeof` when that type
    /// is a placeholder or a type parameter.
    fn bound_value(&self, bound: &TVal, n: i128, b: &Binds) -> TVal {
        let t = match bound {
            TVal::Param(d, i) if *d == b.depth => match b.kinds.get(*i as usize) {
                Some(TPK::Const(pt)) if !pt.is_unknown() => pt.unqual(),
                Some(TPK::Const(pt)) => match &pt.t {
                    // DED-22: a type parameter that another pair gives.
                    Ty::Dep(dd) => match &**dd {
                        Dep::Param(x, j) if *x == b.depth => match b.vals.get(*j as usize) {
                            Some(Some(TArg::Type(q))) => q.unqual(),
                            _ => QTy::new(Ty::size_t()),
                        },
                        _ => QTy::new(Ty::size_t()),
                    },
                    _ => QTy::new(Ty::size_t()),
                },
                _ => QTy::new(Ty::size_t()),
            },
            _ => QTy::new(Ty::size_t()),
        };
        TVal::Const(t, CVal::Int(n))
    }

    pub fn deduce_tname(&mut self, p: &TName, a: &TName, b: &mut Binds) -> bool {
        match p {
            TName::Param(d, i) if *d == b.depth => b.bind(*i, TArg::Templ(a.clone())),
            _ => p == a,
        }
    }

    pub fn deduce_val(&mut self, p: &TVal, a: &TVal, b: &mut Binds) -> bool {
        match p {
            TVal::Param(d, i) if *d == b.depth => {
                if matches!(a, TVal::Opaque) {
                    b.unknown = true;
                    return true;
                }
                if let (TVal::Const(at, _), Some(Some(pt))) = (a, b.patterns.get(*i as usize)) {
                    if !placeholder_fits(&pt.unqual(), &at.unqual()) {
                        return false;
                    }
                }
                if let (TVal::Const(at, _), Some(TPK::Const(pt))) = (a, b.kinds.get(*i as usize).cloned()) {
                    match &pt.t {
                        // DED-11: a type parameter takes the type of the
                        // value.
                        Ty::Dep(dd) => {
                            // DED-22: a bound gives the type parameter a
                            // value only if no other pair does.
                            if let Dep::Param(x, j) = &**dd {
                                if *x == b.depth && !b.from_bound && !b.bind(*j, TArg::Type(at.unqual())) {
                                    return false;
                                }
                            }
                        }
                        // DED-12: the two parameters have the same type. A
                        // placeholder in the type, as in `auto*`, takes any
                        // type in its place.
                        t if !pt.is_unknown() && *t != at.t && !placeholder_fits(&pt.unqual(), &at.unqual()) => return false,
                        _ => {}
                    }
                }
                b.bind(*i, TArg::Val(a.clone()))
            }
            TVal::Opaque => true,
            TVal::Const(pt, pv) => match a {
                TVal::Const(at, av) => pv == av && (pt.t == at.t || pt.t.is_integral() == at.t.is_integral()),
                TVal::Opaque => {
                    b.unknown = true;
                    true
                }
                _ => false,
            },
            _ => p == a,
        }
    }

    /// Match a pattern type with a type, with no allowed differences
    /// (DED-10). Template parameters of other depths match only
    /// themselves.
    pub fn deduce_ty(&mut self, p: &QTy, a: &QTy, b: &mut Binds) -> bool {
        if let Ty::Dep(d) = &p.t {
            match &**d {
                Dep::Param(dd, i) if *dd == b.depth => {
                    if (p.c && !a.c) || (p.v && !a.v) {
                        return false;
                    }
                    if matches!(a.t, Ty::Unknown) {
                        b.unknown = true;
                        return true;
                    }
                    let val = strip_cv(a, p.c, p.v);
                    return b.bind(*i, TArg::Type(val));
                }
                Dep::Opaque => return true,
                _ => {}
            }
            if p.c != a.c || p.v != a.v {
                return false;
            }
            return match &**d {
                Dep::Param(..) => p.t == a.t,
                Dep::Id(n, pargs) => match &a.t {
                    Ty::Class(c) => match self.inst_of.get(c).cloned() {
                        Some((t, aargs)) => self.deduce_tname(n, &TName::Templ(t), b) && pargs.len() == aargs.len() && pargs.iter().zip(aargs.iter()).all(|(x, y)| self.deduce_targ(x, y, b)),
                        None => false,
                    },
                    Ty::Dep(ad) => match &**ad {
                        Dep::Id(an, aargs) => self.deduce_tname(n, an, b) && pargs.len() == aargs.len() && pargs.iter().zip(aargs.iter()).all(|(x, y)| self.deduce_targ(x, y, b)),
                        _ => false,
                    },
                    Ty::Unknown => {
                        b.unknown = true;
                        true
                    }
                    _ => false,
                },
                Dep::Array(el, bound) => match &a.t {
                    Ty::Array(ael, Some(n)) => {
                        let v = self.bound_value(bound, *n as i128, b);
                        if !self.deduce_ty(el, ael, b) {
                            return false;
                        }
                        b.from_bound = true;
                        let r = self.deduce_val(bound, &v, b);
                        b.from_bound = false;
                        r
                    }
                    Ty::Dep(ad) => match &**ad {
                        Dep::Array(ael, abound) => self.deduce_ty(el, ael, b) && self.deduce_val(bound, abound, b),
                        _ => false,
                    },
                    _ => false,
                },
                Dep::MemPtr(cls, mem) => match &a.t {
                    Ty::MemPtr(c, m) => self.deduce_ty(cls, &QTy::new(Ty::Class(*c)), b) && self.deduce_ty(mem, m, b),
                    Ty::Dep(ad) => match &**ad {
                        Dep::MemPtr(ac, am) => self.deduce_ty(cls, ac, b) && self.deduce_ty(mem, am, b),
                        _ => false,
                    },
                    _ => false,
                },
                Dep::Opaque => true,
            };
        }
        if matches!(p.t, Ty::Unknown) {
            b.unknown = true;
            return true;
        }
        if p.c != a.c || p.v != a.v {
            return false;
        }
        match (&p.t, &a.t) {
            (Ty::Ptr(x), Ty::Ptr(y)) | (Ty::LRef(x), Ty::LRef(y)) | (Ty::RRef(x), Ty::RRef(y)) => self.deduce_ty(x, y, b),
            (Ty::Array(x, n), Ty::Array(y, m)) => n == m && self.deduce_ty(x, y, b),
            (Ty::MemPtr(c, x), Ty::MemPtr(d, y)) => c == d && self.deduce_ty(x, y, b),
            (Ty::Func(f), Ty::Func(g)) if self.fn_pack_param(f, b).is_some() && !g.variadic => {
                // DED-37: the expansion takes the remaining parameters.
                let k = self.fn_pack_param(f, b).unwrap();
                let n = f.params.len() - 1;
                if g.params.len() < n || f.cv_c != g.cv_c || f.cv_v != g.cv_v || f.refq != g.refq || f.noexcept != g.noexcept {
                    return false;
                }
                let rest: Vec<TArg> = g.params[n..].iter().map(|q| TArg::Type(q.clone())).collect();
                f.params[..n].iter().zip(g.params.iter()).all(|(p, a)| self.deduce_ty(p, a, b)) && b.bind(k, TArg::Pack(rest)) && self.deduce_ty(&f.ret, &g.ret, b)
            }
            (Ty::Func(f), Ty::Func(g)) => {
                f.variadic == g.variadic
                    && f.cv_c == g.cv_c
                    && f.cv_v == g.cv_v
                    && f.refq == g.refq
                    && f.noexcept == g.noexcept
                    && self.deduce_fn_params(&f.params, &g.params, b)
                    && self.deduce_ty(&f.ret, &g.ret, b)
            }
            (_, Ty::Unknown) => {
                b.unknown = true;
                true
            }
            _ => p.t == a.t,
        }
    }

    /// The parameter lists of two function types; a trailing pack
    /// expansion takes the rest (DED-37).
    fn deduce_fn_params(&mut self, ps: &[QTy], xs: &[QTy], b: &mut Binds) -> bool {
        if ps.len() != xs.len() {
            return false;
        }
        ps.iter().zip(xs.iter()).all(|(p, a)| self.deduce_ty(p, a, b))
    }

    /// The index of the pack that the last parameter of a function type
    /// expands, if any (DED-37).
    fn fn_pack_param(&self, f: &FnTy, b: &Binds) -> Option<u32> {
        if !f.variadic {
            return None;
        }
        let last = f.params.last()?;
        match &last.t {
            Ty::Dep(d) => match &**d {
                Dep::Param(dd, i) if *dd == b.depth && b.packs.get(*i as usize).copied().unwrap_or(false) => Some(*i),
                _ => None,
            },
            _ => None,
        }
    }
}

/// The type of a placeholder constant parameter from its argument
/// (TPL-11): the argument's type after the adjustments of a variable
/// declared with `auto`.
pub fn auto_param_type(e: &Expr) -> QTy {
    let t = nonref(&e.ty);
    match &t.t {
        Ty::Array(el, _) => QTy::new(Ty::Ptr(el.clone())),
        Ty::Func(_) => QTy::new(Ty::Ptr(Box::new(t.clone()))),
        _ => t.unqual(),
    }
}

/// Whether an expression has a part the front end does not model.
pub fn has_unknown(e: &Expr) -> bool {
    match &e.k {
        EK::Unknown(_) | EK::Reinterpret(..) => true,
        EK::Unary(_, x) | EK::Conv(_, x) | EK::Member(x, _) | EK::DynamicCast(x) => has_unknown(x),
        EK::Binary(_, x, y) | EK::Assign(_, x, y) | EK::Comma(x, y) | EK::Index(x, y) => has_unknown(x) || has_unknown(y),
        EK::Cond(x, y, z) => has_unknown(x) || has_unknown(y) || has_unknown(z),
        EK::Call(f, a, _) => has_unknown(f) || a.iter().any(has_unknown),
        EK::InitList(v) | EK::Construct(_, v) => v.iter().any(has_unknown),
        _ => false,
    }
}

fn strip_conv(e: &Expr) -> &Expr {
    match &e.k {
        EK::Conv(_, x) => strip_conv(x),
        _ => e,
    }
}

/// The result of deduction for one function template.
pub enum Ded {
    Ok(Vec<TArg>),
    Fail,
    Unknown,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Pair {
    Ok,
    Fail,
    Unknown,
}

pub enum SpecRes {
    Ok(DeclId),
    Fail,
    Unknown,
}

impl P {
    /// A function template with a record.
    pub fn is_func_templ(&self, id: DeclId) -> bool {
        self.func(id).is_some_and(|f| f.template && !f.constrained) && self.decl_tid.get(&id).is_some_and(|&t| !self.templs[t].dependent && !self.templs[t].constrained && self.templs[t].kind == TKind::Func)
    }

    /// Whether a pattern names a parameter of `b` that has no value yet
    /// in a deducible position.
    fn mentions_unbound(&self, q: &QTy, b: &Binds) -> bool {
        let open = |i: u32| !b.fixed.get(i as usize).copied().unwrap_or(true);
        match &q.t {
            Ty::Dep(d) => match &**d {
                Dep::Param(dd, i) => *dd == b.depth && open(*i),
                Dep::Id(n, args) => {
                    let nm = matches!(n, TName::Param(dd, i) if *dd == b.depth && open(*i));
                    nm || args.iter().any(|a| self.targ_mentions_unbound(a, b))
                }
                Dep::Array(el, bound) => self.mentions_unbound(el, b) || matches!(bound, TVal::Param(dd, i) if *dd == b.depth && open(*i)),
                Dep::MemPtr(c, m) => self.mentions_unbound(c, b) || self.mentions_unbound(m, b),
                Dep::Opaque => false,
            },
            Ty::Ptr(x) | Ty::LRef(x) | Ty::RRef(x) | Ty::Array(x, _) | Ty::MemPtr(_, x) => self.mentions_unbound(x, b),
            Ty::Func(f) => self.mentions_unbound(&f.ret, b) || f.params.iter().any(|p| self.mentions_unbound(p, b)),
            _ => false,
        }
    }

    fn targ_mentions_unbound(&self, a: &TArg, b: &Binds) -> bool {
        let open = |i: u32| !b.fixed.get(i as usize).copied().unwrap_or(true);
        match a {
            TArg::Type(q) => self.mentions_unbound(q, b),
            TArg::Val(TVal::Param(dd, i)) => *dd == b.depth && open(*i),
            TArg::Templ(TName::Param(dd, i)) => *dd == b.depth && open(*i),
            TArg::Pack(v) => v.iter().any(|x| self.targ_mentions_unbound(x, b)),
            TArg::Expand(x) => self.targ_mentions_unbound(x, b),
            _ => false,
        }
    }

    /// The template parameters of `b` that a pattern names.
    fn params_in(&self, q: &QTy, b: &Binds, out: &mut Vec<u32>) {
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
                        self.targ_params_in(a, b, out);
                    }
                }
                Dep::Array(el, bound) => {
                    self.params_in(el, b, out);
                    if let TVal::Param(dd, i) = bound {
                        if *dd == b.depth {
                            out.push(*i);
                        }
                    }
                }
                Dep::MemPtr(c, m) => {
                    self.params_in(c, b, out);
                    self.params_in(m, b, out);
                }
                _ => {}
            },
            Ty::Ptr(x) | Ty::LRef(x) | Ty::RRef(x) | Ty::Array(x, _) | Ty::MemPtr(_, x) => self.params_in(x, b, out),
            Ty::Func(f) => {
                self.params_in(&f.ret, b, out);
                for p in &f.params {
                    self.params_in(p, b, out);
                }
            }
            _ => {}
        }
    }

    fn targ_params_in(&self, a: &TArg, b: &Binds, out: &mut Vec<u32>) {
        match a {
            TArg::Type(q) => self.params_in(q, b, out),
            TArg::Val(TVal::Param(dd, i)) | TArg::Templ(TName::Param(dd, i)) if *dd == b.depth => out.push(*i),
            TArg::Pack(v) => {
                for x in v {
                    self.targ_params_in(x, b, out);
                }
            }
            TArg::Expand(x) => self.targ_params_in(x, b, out),
            _ => {}
        }
    }

    /// DED-P2: deduction of the template arguments of the function
    /// template `fid` for a call with `args`.
    pub fn deduce_call(&mut self, fid: DeclId, explicit: Option<&Rc<Vec<RawArg>>>, args: &[Expr], line: u32) -> Ded {
        let Some(&tid) = self.decl_tid.get(&fid) else { return Ded::Unknown };
        if self.templs[tid].dependent {
            return Ded::Unknown;
        }
        let params = self.templs[tid].params.clone();
        let depth = self.templs[tid].depth;
        let Some(f) = self.func(fid).cloned() else { return Ded::Unknown };
        let mut b = Binds::new(depth, &params);
        // Step 1: the explicit arguments (DED-2, DED-5).
        let mut explicit_pack_len: Option<usize> = None;
        if let Some(raw) = explicit {
            let me = self.d.muted_errors;
            self.d.muted += 1;
            let r = self.targs_for(tid, (**raw).clone(), line, true);
            self.d.muted -= 1;
            match r {
                Ok(Some(v)) => {
                    if self.d.muted_errors != me {
                        return Ded::Fail;
                    }
                    for (i, a) in v.into_iter().enumerate() {
                        if let TArg::Pack(p) = &a {
                            explicit_pack_len = Some(p.len());
                        }
                        b.vals[i] = Some(a);
                        b.fixed[i] = true;
                    }
                }
                Ok(None) => return Ded::Unknown,
                Err(()) => return Ded::Fail,
            }
        }
        // Steps 2 and 3: each pair (DED-6 to DED-20).
        let ptys = f.fty.params.clone();
        let np = ptys.len();
        let pack_i = self.templs[tid].fn_pack;
        let mut pack_vals: Vec<(u32, Vec<TArg>)> = Vec::new();
        let mut pack_params: Vec<u32> = Vec::new();
        if let Some(k) = pack_i {
            let mut v = Vec::new();
            self.params_in(&ptys[k], &b, &mut v);
            for i in v {
                if params.get(i as usize).is_some_and(|p| p.pack) && !pack_params.contains(&i) {
                    pack_params.push(i);
                }
            }
        }
        let mut deduced_pack = false;
        for (i, a) in args.iter().enumerate() {
            let p = match pack_i {
                Some(k) if k + 1 == np && i >= k => {
                    // DED-20: the trailing pack takes the remaining
                    // arguments, each with the pattern.
                    if pack_params.is_empty() || explicit_pack_len.is_some() {
                        if let Some(n) = explicit_pack_len {
                            if i - k < n {
                                continue;
                            }
                        }
                        return Ded::Unknown;
                    }
                    let mut t = b.clone();
                    for &q in &pack_params {
                        t.vals[q as usize] = None;
                    }
                    match self.deduce_pair(&ptys[k], a, &mut t) {
                        Pair::Ok => {}
                        Pair::Fail => return Ded::Fail,
                        Pair::Unknown => return Ded::Unknown,
                    }
                    for &q in &pack_params {
                        let v = t.vals[q as usize].take();
                        let Some(v) = v else { return Ded::Unknown };
                        match pack_vals.iter_mut().find(|x| x.0 == q) {
                            Some(x) => x.1.push(v),
                            None => pack_vals.push((q, vec![v])),
                        }
                    }
                    for (j, v) in t.vals.into_iter().enumerate() {
                        if !pack_params.contains(&(j as u32)) {
                            b.vals[j] = v;
                        }
                    }
                    b.unknown |= t.unknown;
                    deduced_pack = true;
                    continue;
                }
                Some(k) if i >= k => {
                    // DED-20: a pack before the end is non-deduced; it
                    // takes the explicit elements only.
                    let n = explicit_pack_len.unwrap_or(0);
                    if i < k + n {
                        continue;
                    }
                    let j = i - n + 1;
                    if j >= np {
                        return Ded::Fail;
                    }
                    ptys[j].clone()
                }
                _ => {
                    if i >= np {
                        if f.fty.variadic {
                            break;
                        }
                        return Ded::Fail;
                    }
                    ptys[i].clone()
                }
            };
            match self.deduce_pair(&p, a, &mut b) {
                Pair::Ok => {}
                Pair::Fail => return Ded::Fail,
                Pair::Unknown => return Ded::Unknown,
            }
        }
        if deduced_pack {
            for (q, v) in pack_vals {
                b.vals[q as usize] = Some(TArg::Pack(v));
            }
        }
        // DED-22: a type parameter that only an array bound gives takes
        // the bound's type; the bound converts to the type that another
        // pair gave.
        for i in 0..params.len() {
            if let (TPK::Const(pt), Some(TArg::Val(TVal::Const(vt, cv)))) = (&params[i].kind, b.vals[i].clone()) {
                if let Ty::Dep(dd) = &pt.t {
                    if let Dep::Param(x, j) = &**dd {
                        if *x == depth {
                            match b.vals[*j as usize].clone() {
                                None => b.vals[*j as usize] = Some(TArg::Type(vt.unqual())),
                                Some(TArg::Type(t)) if t.unqual().t != vt.t => b.vals[i] = Some(TArg::Val(TVal::Const(t.unqual(), cv))),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        // Step 4: defaults and empty packs (DED-23).
        for i in 0..params.len() {
            if b.vals[i].is_some() {
                continue;
            }
            if params[i].pack {
                b.vals[i] = Some(TArg::Pack(vec![]));
                continue;
            }
            if params[i].default.is_some() {
                let prev: Vec<TArg> = b.vals[..i].iter().map(|v| v.clone().unwrap_or(TArg::Pack(vec![]))).collect();
                let me = self.d.muted_errors;
                self.d.muted += 1;
                let r = self.default_arg(tid, i, &prev);
                self.d.muted -= 1;
                match r {
                    Some(a) if !a.is_dep() && self.d.muted_errors == me => b.vals[i] = Some(a),
                    Some(_) => return Ded::Unknown,
                    None => return Ded::Fail,
                }
                continue;
            }
            return Ded::Fail;
        }
        if b.unknown {
            return Ded::Unknown;
        }
        // DED-28: a parameter whose type holds no template parameter is
        // checked against its argument before the function type is
        // substituted.
        for (i, a) in args.iter().enumerate() {
            let Some(p) = ptys.get(i) else { break };
            if p.is_unknown() || pack_i.is_some_and(|k| i >= k) {
                continue;
            }
            if let Ok(None) = self.ics_form(a, p, false) {
                return Ded::Fail;
            }
        }
        Ded::Ok(b.vals.into_iter().map(|v| v.unwrap()).collect())
    }

    /// One pair of a parameter type and a call argument.
    pub fn deduce_pair(&mut self, p: &QTy, a: &Expr, b: &mut Binds) -> Pair {
        if !self.mentions_unbound(p, b) {
            // DED-3: no parameter to deduce; DED-13: non-deduced only.
            return Pair::Ok;
        }
        if let EK::InitList(items) = &a.k {
            if a.ty.is_unknown() || !matches!(a.ty.t, Ty::Class(_)) {
                return self.deduce_list(p, items, b, false);
            }
        }
        let set = match &a.k {
            EK::Overloads(fs) => Some(fs.clone()),
            EK::Func(f) => Some(vec![*f]).filter(|_| self.is_func_templ(*f)),
            EK::Unary(UOp::AddrOf, x) => match &x.k {
                EK::Overloads(fs) => Some(fs.clone()),
                _ => None,
            },
            EK::TemplId(..) => return Pair::Unknown,
            _ => None,
        };
        if let Some(fs) = set {
            return self.deduce_ovset(p, &fs, b);
        }
        if a.ty.is_unknown() || has_unknown(a) {
            return Pair::Unknown;
        }
        let at = nonref(&a.ty);
        let (pp, is_ref) = match &p.t {
            Ty::LRef(x) => ((**x).clone(), 1),
            Ty::RRef(x) => ((**x).clone(), 2),
            _ => (p.unqual(), 0),
        };
        let mut aa = at.clone();
        if is_ref == 0 {
            // DED-6: the adjustments of an argument for a value parameter.
            aa = match &at.t {
                Ty::Array(el, _) => QTy::new(Ty::Ptr(el.clone())),
                Ty::Func(_) => QTy::new(Ty::Ptr(Box::new(at.clone()))),
                _ => at.unqual(),
            };
        } else if is_ref == 2 && a.cat == Cat::L {
            // DED-8: a forwarding reference with an lvalue gives `A&`.
            if let Ty::Dep(d) = &pp.t {
                if let Dep::Param(dd, i) = &**d {
                    // CTAD-8: a parameter of the class gives no forwarding
                    // reference in a guide from a constructor.
                    let class_param = self.no_fwd.is_some_and(|(nd, n)| nd == *dd && *i < n);
                    if *dd == b.depth && !pp.c && !pp.v && !class_param {
                        aa = QTy::new(Ty::LRef(Box::new(at.clone())));
                    }
                }
            }
        }
        if self.deduce_adjusted(&pp, &aa, is_ref != 0, b) {
            Pair::Ok
        } else if b.unknown {
            Pair::Unknown
        } else {
            Pair::Fail
        }
    }

    /// The match of a pair after the adjustments, with the differences
    /// of DED-14 and DED-15 when no exact match exists.
    fn deduce_adjusted(&mut self, pp: &QTy, aa: &QTy, is_ref: bool, b: &mut Binds) -> bool {
        let mut t = b.clone();
        if self.deduce_ty(pp, aa, &mut t) {
            *b = t;
            return true;
        }
        // DED-14: more const on the referred type of a reference.
        if is_ref && (!aa.c || pp.c) && (!aa.v || pp.v) && (pp.c || pp.v) {
            let mut t = b.clone();
            if self.deduce_ty(&strip_cv(pp, true, true), &strip_cv(aa, true, true), &mut t) {
                *b = t;
                return true;
            }
        }
        // DED-14: a qualification adjustment or a dropped noexcept.
        if matches!(pp.t, Ty::Ptr(_) | Ty::MemPtr(..)) || matches!(&pp.t, Ty::Dep(d) if matches!(**d, Dep::MemPtr(..))) {
            let mut t = b.clone();
            if self.deduce_qual(pp, aa, &mut t, true) {
                *b = t;
                return true;
            }
        }
        // DED-15: a base class that is a specialization.
        let (pcls, acls, ptr) = match (&pp.t, &aa.t) {
            (Ty::Dep(d), Ty::Class(c)) if matches!(**d, Dep::Id(..)) => (pp.clone(), *c, false),
            (Ty::Ptr(pi), Ty::Ptr(ai)) => match (&pi.t, &ai.t) {
                (Ty::Dep(d), Ty::Class(c)) if matches!(**d, Dep::Id(..)) => ((**pi).clone(), *c, true),
                _ => return false,
            },
            _ => return false,
        };
        let acv = if ptr {
            match &aa.t {
                Ty::Ptr(ai) => (ai.c, ai.v),
                _ => (false, false),
            }
        } else {
            (aa.c, aa.v)
        };
        self.require_class(acls, 0);
        let mut bases: Vec<usize> = Vec::new();
        self.all_bases(acls, &mut bases, 0);
        let mut found: Vec<(usize, Binds)> = Vec::new();
        for bc in bases {
            let mut t = b.clone();
            let bq = QTy { t: Ty::Class(bc), c: acv.0, v: acv.1 };
            let ok = self.deduce_ty(&pcls, &bq, &mut t) || {
                t = b.clone();
                (!bq.c || pcls.c) && (!bq.v || pcls.v) && self.deduce_ty(&strip_cv(&pcls, true, true), &strip_cv(&bq, true, true), &mut t)
            };
            if ok {
                found.push((bc, t));
            }
        }
        if found.is_empty() {
            return false;
        }
        // A base derived from another matching base wins over it.
        let snapshot: Vec<usize> = found.iter().map(|x| x.0).collect();
        found.retain(|(c, _)| !snapshot.iter().any(|&o| o != *c && self.is_base_of_p(*c, o)));
        let first = found[0].1.vals.clone();
        if found.iter().any(|x| x.1.vals != first) {
            return false;
        }
        *b = found.swap_remove(0).1;
        true
    }

    fn all_bases(&self, c: usize, out: &mut Vec<usize>, depth: u32) {
        if depth > 16 {
            return;
        }
        for bi in &self.classes[c].bases {
            if let Some(bc) = bi.class {
                if !out.contains(&bc) {
                    out.push(bc);
                }
                self.all_bases(bc, out, depth + 1);
            }
        }
    }

    /// DED-14: a pointer that reaches the parameter type by adding
    /// qualifiers or dropping `noexcept`.
    fn deduce_qual(&mut self, p: &QTy, a: &QTy, b: &mut Binds, top: bool) -> bool {
        if let (Ty::Dep(d), Ty::MemPtr(c, m)) = (&p.t, &a.t) {
            if let Dep::MemPtr(pc, pm) = &**d {
                if (m.c && !pm.c) || (m.v && !pm.v) {
                    return false;
                }
                let mut t = b.clone();
                if self.deduce_ty(pc, &QTy::new(Ty::Class(*c)), &mut t) && self.deduce_ty(&strip_cv(pm, true, true), &strip_cv(m, true, true), &mut t) {
                    *b = t;
                    return true;
                }
                return false;
            }
        }
        match (&p.t, &a.t) {
            (Ty::Ptr(pi), Ty::Ptr(ai)) => {
                if (ai.c && !pi.c) || (ai.v && !pi.v) {
                    return false;
                }
                if !top && (p.c != a.c || p.v != a.v) && !(p.c || !a.c) {
                    return false;
                }
                let mut t = b.clone();
                if self.deduce_ty(pi, ai, &mut t) {
                    *b = t;
                    return true;
                }
                let pi2 = strip_cv(pi, true, true);
                let ai2 = strip_cv(ai, true, true);
                if self.deduce_qual(&QTy { t: pi2.t.clone(), c: false, v: false }, &QTy { t: ai2.t.clone(), c: false, v: false }, b, false) {
                    return true;
                }
                let mut t = b.clone();
                if self.deduce_ty(&pi2, &ai2, &mut t) {
                    *b = t;
                    return true;
                }
                // A function pointer that drops `noexcept`.
                if let (Ty::Func(_), Ty::Func(af)) = (&pi.t, &ai.t) {
                    if af.noexcept {
                        let mut f2 = (**af).clone();
                        f2.noexcept = false;
                        let a2 = QTy::new(Ty::Func(Box::new(f2)));
                        let mut t = b.clone();
                        if self.deduce_ty(pi, &a2, &mut t) {
                            *b = t;
                            return true;
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// DED-16 to DED-18: a braced list argument.
    fn deduce_list(&mut self, p: &QTy, items: &[Expr], b: &mut Binds, nested: bool) -> Pair {
        let base = nonref(p).unqual();
        if let Ty::Dep(d) = &base.t {
            match &**d {
                Dep::Id(TName::Templ(t), args) if self.templs[*t].name == "initializer_list" && args.len() == 1 => {
                    if let TArg::Type(el) = &args[0] {
                        let el = el.clone();
                        for e in items {
                            match self.deduce_pair(&el, e, b) {
                                Pair::Ok => {}
                                r => return r,
                            }
                        }
                    }
                    return Pair::Ok;
                }
                Dep::Array(el, bound) => {
                    if !p.t.is_ref() && !nested {
                        return Pair::Ok;
                    }
                    let el = el.clone();
                    for e in items {
                        // DED-17: nested lists give the inner bounds.
                        let r = match &e.k {
                            EK::InitList(inner) if matches!(&el.t, Ty::Dep(d) if matches!(**d, Dep::Array(..))) || matches!(el.t, Ty::Array(..)) => self.deduce_list(&el, inner, b, true),
                            _ => self.deduce_pair(&el, e, b),
                        };
                        match r {
                            Pair::Ok => {}
                            r => return r,
                        }
                    }
                    let n = self.bound_value(bound, items.len() as i128, b);
                    if items.is_empty() {
                        return Pair::Fail;
                    }
                    b.from_bound = true;
                    let r = self.deduce_val(bound, &n, b);
                    b.from_bound = false;
                    return if r { Pair::Ok } else { Pair::Fail };
                }
                _ => {}
            }
        }
        if let Ty::Array(el, n) = &base.t {
            if p.t.is_ref() || nested {
                if let Some(n) = n {
                    if items.len() as u64 > *n {
                        return Pair::Fail;
                    }
                }
                let el = (**el).clone();
                for e in items {
                    match self.deduce_pair(&el, e, b) {
                        Pair::Ok => {}
                        r => return r,
                    }
                }
                return Pair::Ok;
            }
        }
        // DED-18: no value from the list.
        Pair::Ok
    }

    /// DED-19: an argument that names an overload set.
    fn deduce_ovset(&mut self, p: &QTy, fs: &[DeclId], b: &mut Binds) -> Pair {
        if fs.iter().any(|&f| self.func(f).is_none_or(|fi| fi.template)) {
            return Pair::Ok;
        }
        let (pp, is_ref) = match &p.t {
            Ty::LRef(x) | Ty::RRef(x) => ((**x).clone(), true),
            _ => (p.unqual(), false),
        };
        let mut good: Vec<Binds> = Vec::new();
        for &f in fs {
            let fi = self.func(f).unwrap();
            if fi.class.is_some() && !fi.is_static {
                continue;
            }
            let ft = QTy::new(Ty::Func(Box::new(fi.fty.clone())));
            let a = if is_ref { ft } else { QTy::new(Ty::Ptr(Box::new(ft))) };
            let mut t = b.clone();
            if self.deduce_ty(&pp, &a, &mut t) {
                good.push(t);
            }
        }
        if good.len() == 1 {
            *b = good.pop().unwrap();
        }
        Pair::Ok
    }

    /// A specialization of a function template: its declaration, made by
    /// substitution (DED-24). Errors in the declaration remove it.
    pub fn func_spec(&mut self, tid: usize, args: &[TArg], line: u32) -> SpecRes {
        if let Some(x) = self.templs[tid].explicit.iter().find(|x| x.0 == args) {
            if let Inst::Func(d) = x.1 {
                return SpecRes::Ok(d);
            }
        }
        if let Some(x) = self.templs[tid].insts.iter().find(|x| x.0 == args) {
            return match x.1 {
                Inst::Func(d) => SpecRes::Ok(d),
                Inst::Fail => SpecRes::Fail,
                _ => SpecRes::Unknown,
            };
        }
        // CNS-11: the constraints are checked before the function type is
        // formed.
        match self.templ_sat(tid, args) {
            crate::cons::Sat::Yes => {}
            crate::cons::Sat::Unknown => return SpecRes::Unknown,
            crate::cons::Sat::No | crate::cons::Sat::Error => {
                self.templs[tid].insts.push((args.to_vec(), Inst::Fail, None));
                return SpecRes::Fail;
            }
        }
        let Some(src) = self.templs[tid].src.clone() else { return SpecRes::Unknown };
        let params = self.templs[tid].params.clone();
        let name = self.decls[self.templs[tid].decl].name.clone();
        self.templs[tid].insts.push((args.to_vec(), Inst::Busy, None));
        let (me, mu) = (self.d.muted_errors, self.d.muted_unsup);
        self.d.muted += 1;
        let ctx = src.ctx;
        let saved_mark = self.sfinae_mark.replace(me);
        let r = self.replay(&src, &params, args, Some(InstReq::Func { name, existing: None }), |p| {
            let r = p.parse_declaration(ctx);
            r.is_ok()
        });
        self.sfinae_mark = saved_mark;
        self.d.muted -= 1;
        let k = self.templs[tid].insts.iter().position(|x| x.0 == args).unwrap();
        let errs = self.d.muted_errors != me;
        let unsup = self.d.muted_unsup != mu;
        let res = match r {
            Some((true, Some(InstOut::Func(id)))) if !errs && !unsup => {
                self.templs[tid].insts[k].1 = Inst::Func(id);
                self.spec_of.insert(id, (tid, args.to_vec()));
                let label = format!("{}<{}>", self.templs[tid].name, self.show_targs(args));
                self.fn_label.insert(id, label);
                SpecRes::Ok(id)
            }
            Some((_, out)) if errs && !unsup => {
                let _ = out;
                self.templs[tid].insts[k].1 = Inst::Fail;
                SpecRes::Fail
            }
            _ => {
                self.templs[tid].insts.remove(k);
                SpecRes::Unknown
            }
        };
        let _ = line;
        res
    }

    /// The definition of a function specialization, made when a use needs
    /// it. Errors in it are errors of the program (DED-29).
    pub fn func_body(&mut self, id: DeclId) {
        let Some((tid, args)) = self.spec_of.get(&id).cloned() else { return };
        if self.func(id).is_none_or(|f| f.body.is_some() || f.defaulted || f.deleted) || self.body_busy.contains(&id) {
            return;
        }
        if self.templs[tid].explicit.iter().any(|x| x.1 == Inst::Func(id)) {
            return;
        }
        let Some(src) = self.templs[tid].src.clone() else { return };
        if !src.toks.iter().any(|t| t.kind == TK::Punct && (t.text == "{" || t.text == "=")) {
            return;
        }
        let params = self.templs[tid].params.clone();
        let name = self.decls[self.templs[tid].decl].name.clone();
        self.body_busy.insert(id);
        let ctx = src.ctx;
        let saved_muted = std::mem::take(&mut self.d.muted);
        let r = self.replay(&src, &params, &args, Some(InstReq::Func { name, existing: Some(id) }), |p| {
            let _ = p.parse_declaration(ctx);
            if !p.deferred.is_empty() {
                let f = std::mem::replace(&mut p.force_bodies, true);
                p.run_deferred();
                p.force_bodies = f;
            }
        });
        self.d.muted = saved_muted;
        self.body_busy.remove(&id);
        if let Some((_, Some(InstOut::Func(nid)))) = r {
            if nid != id {
                let nf = self.func(nid).cloned();
                if let (Some(nf), Some(f)) = (nf, self.func_mut(id)) {
                    f.body = nf.body;
                    f.params = nf.params;
                    f.mem_inits = nf.mem_inits;
                    f.fn_try = nf.fn_try;
                    f.defined = nf.defined;
                    f.opaque = nf.opaque;
                    f.parse_failed = nf.parse_failed;
                    if f.fty.ret.is_unknown() && !nf.fty.ret.is_unknown() {
                        f.fty.ret = nf.fty.ret.clone();
                    }
                    f.default_exprs = nf.default_exprs;
                    f.consteval |= nf.consteval;
                }
            }
        }
    }

    /// The candidates of a call after deduction: each function template
    /// adds its specialization (DED-1, OVL-P2). Err(true) when the front
    /// end cannot tell; Err(false) when every template failed and no
    /// other candidate is left.
    pub fn templ_candidates(&mut self, cands: &[DeclId], explicit: Option<&Rc<Vec<RawArg>>>, args: &[Expr], line: u32) -> Result<Vec<DeclId>, bool> {
        let mut out = Vec::new();
        for &c in cands {
            if self.is_func_templ(c) {
                match self.deduce_call(c, explicit, args, line) {
                    Ded::Ok(targs) => {
                        let tid = self.decl_tid[&c];
                        match self.func_spec(tid, &targs, line) {
                            SpecRes::Ok(s) => {
                                if !out.contains(&s) {
                                    out.push(s);
                                }
                            }
                            SpecRes::Fail => {}
                            SpecRes::Unknown => return Err(true),
                        }
                    }
                    Ded::Fail => {}
                    Ded::Unknown => return Err(true),
                }
            } else if self.func(c).is_some_and(|f| f.template) {
                return Err(true);
            } else if explicit.is_none() {
                out.push(c);
            }
        }
        if out.is_empty() {
            return Err(false);
        }
        Ok(out)
    }

    /// OVL-28, ORD-P2: the specialization `s1` comes from a template more
    /// specialized than that of `s2`, for a call with `nargs` arguments.
    pub fn spec_more_specialized(&mut self, s1: DeclId, s2: DeclId, nargs: usize) -> bool {
        let (Some((t1, _)), Some((t2, _))) = (self.spec_of.get(&s1).cloned(), self.spec_of.get(&s2).cloned()) else { return false };
        if t1 == t2 {
            return false;
        }
        let (a, b) = self.func_order(t1, t2, Some(nargs));
        a && !b
    }

    /// ORD-1: (`t1` is at least as specialized as `t2`, and the reverse),
    /// after the tie-breakers of ORD-10 to ORD-12. `nargs`: the number of
    /// call arguments (ORD-5); None compares the whole function types
    /// (ORD-9).
    pub fn func_order(&mut self, t1: usize, t2: usize, nargs: Option<usize>) -> (bool, bool) {
        let f1 = self.func(self.templs[t1].decl).cloned();
        let f2 = self.func(self.templs[t2].decl).cloned();
        let (Some(f1), Some(f2)) = (f1, f2) else { return (false, false) };
        let d1 = self.templs[t1].depth;
        let d2 = self.templs[t2].depth;
        let p1 = self.templs[t1].params.clone();
        let p2 = self.templs[t2].params.clone();
        let k1 = self.templs[t1].fn_pack;
        let k2 = self.templs[t2].fn_pack;
        // The types that take part, with a flag for a pack expansion.
        let types = |f: &FuncInfo, k: Option<usize>, n: Option<usize>| -> Vec<(QTy, bool)> {
            let mut v = Vec::new();
            match n {
                Some(n) => {
                    for i in 0..n {
                        match k {
                            Some(k) if i >= k && k + 1 == f.fty.params.len() => v.push((f.fty.params[k].clone(), true)),
                            _ => {
                                if i < f.fty.params.len() {
                                    v.push((f.fty.params[i].clone(), false));
                                }
                            }
                        }
                    }
                }
                None => {
                    v.push((f.fty.ret.clone(), false));
                    for (i, p) in f.fty.params.iter().enumerate() {
                        v.push((p.clone(), Some(i) == k));
                    }
                }
            }
            v
        };
        // ORD-6: a member with an implicit object parameter against a
        // function without one gets a first parameter of reference to its
        // class.
        let member = |f: &FuncInfo| f.class.is_some() && !f.is_static && !f.explicit_obj && f.kind != FnKind::Ctor;
        let obj_param = |f: &FuncInfo, other: &FuncInfo| -> QTy {
            let c = f.class.unwrap();
            let ct = QTy { t: Ty::Class(c), c: f.fty.cv_c, v: f.fty.cv_v };
            let rref = f.fty.refq == 2 || (f.fty.refq == 0 && other.fty.params.first().is_some_and(|p| matches!(p.t, Ty::RRef(_))));
            QTy::new(if rref { Ty::RRef(Box::new(ct)) } else { Ty::LRef(Box::new(ct)) })
        };
        let shift = |f: &FuncInfo, other: &FuncInfo, k: Option<usize>| -> (FuncInfo, Option<usize>) {
            let mut g = f.clone();
            if member(f) && !member(other) && nargs.is_some() {
                g.fty.params.insert(0, obj_param(f, other));
                return (g, k.map(|k| k + 1));
            }
            (g, k)
        };
        let (g1, k1) = shift(&f1, &f2, k1);
        let (g2, k2) = shift(&f2, &f1, k2);
        let ty1 = types(&g1, k1, nargs);
        let ty2 = types(&g2, k2, nargs);
        let (f1, f2) = (g1, g2);
        let n = ty1.len().min(ty2.len());
        // Steps 1 to 4: deduction in both directions, pair by pair.
        let dir = |p: &mut P, from: &[(QTy, bool)], fd: u32, to: &[(QTy, bool)], tparams: &[TParam], td: u32| -> Vec<bool> {
            let mut b = Binds::new(td, tparams);
            let mut res = Vec::new();
            for i in 0..n {
                let (a, apack) = &from[i];
                let (pt, ppack) = &to[i];
                if *apack && !*ppack {
                    // ORD-4: an invented pack gives no single value.
                    res.push(false);
                    continue;
                }
                let a = p.invent_ty(a, fd);
                let a = strip_cv(&nonref(&a), true, true);
                let pt2 = strip_cv(&nonref(pt), true, true);
                let mut t = b.clone();
                let ok = p.deduce_ty(&pt2, &a, &mut t) && !t.unknown;
                // A pack pattern deduces one element per position.
                if ok && !*ppack {
                    b = t;
                }
                res.push(ok);
            }
            res
        };
        let mut r12 = dir(self, &ty1, d1, &ty2, &p2, d2);
        let mut r21 = dir(self, &ty2, d2, &ty1, &p1, d1);
        // Step 5: the reference tie-breakers inside each pair.
        for i in 0..n {
            if !(r12[i] && r21[i]) {
                continue;
            }
            let (a, b) = (&ty1[i].0, &ty2[i].0);
            if !(a.t.is_ref() && b.t.is_ref()) {
                continue;
            }
            let (ar, br) = (matches!(a.t, Ty::RRef(_)), matches!(b.t, Ty::RRef(_)));
            if ar && !br {
                r12[i] = false;
            } else if br && !ar {
                r21[i] = false;
            }
            let (ai, bi) = (nonref(a), nonref(b));
            let a_more = (ai.c || !bi.c) && (ai.v || !bi.v) && (ai.c != bi.c || ai.v != bi.v);
            let b_more = (bi.c || !ai.c) && (bi.v || !ai.v) && (ai.c != bi.c || ai.v != bi.v);
            if a_more {
                r21[i] = false;
            } else if b_more {
                r12[i] = false;
            }
        }
        let a12 = r12.iter().all(|&x| x);
        let a21 = r21.iter().all(|&x| x);
        if a12 && a21 {
            // ORD-12: no trailing pack beats a trailing pack.
            let pk1 = k1.is_some_and(|k| k + 1 == f1.fty.params.len());
            let pk2 = k2.is_some_and(|k| k + 1 == f2.fty.params.len());
            if pk1 && !pk2 && f2.fty.params.len() <= k1.unwrap() {
                return (false, true);
            }
            if pk2 && !pk1 && f1.fty.params.len() <= k2.unwrap() {
                return (true, false);
            }
            // ORD-13: the more constrained template.
            if let Some(x) = self.templ_cons_order(t1, t2) {
                return x;
            }
        }
        (a12, a21)
    }
}

/// TPL-20: two template heads are equivalent.
pub fn heads_equivalent(a: &[TParam], b: &[TParam]) -> bool {
    a.len() == b.len()
        && a.iter().zip(b.iter()).all(|(x, y)| {
            x.pack == y.pack
                && match (&x.kind, &y.kind) {
                    (TPK::Type, TPK::Type) => true,
                    (TPK::Auto, TPK::Auto) => true,
                    (TPK::Const(s), TPK::Const(t)) => s.t == t.t,
                    (TPK::Templ(s), TPK::Templ(t)) => heads_equivalent(s, t),
                    _ => false,
                }
        })
}

impl P {
    /// Start a discarded statement in a specialization (IF-13): it is
    /// parsed as a template pattern, with no diagnostics.
    pub fn begin_discarded(&mut self) -> (u32, u32) {
        self.templ += 1;
        self.d.muted += 1;
        (self.d.muted_errors, self.d.muted_unsup)
    }

    pub fn end_discarded(&mut self, s: (u32, u32)) {
        self.templ -= 1;
        self.d.muted -= 1;
        self.d.muted_errors = s.0;
        self.d.muted_unsup = s.1;
    }
}

impl P {
    /// Parse a pattern. In a specialization, when `...` follows and the
    /// pattern names packs, parse it again once for each element
    /// (a pack expansion).
    pub fn parse_expansion<T>(&mut self, parse: impl Fn(&mut P) -> PR<T>) -> PR<(Vec<T>, bool)> {
        let start = self.pos;
        let mark = self.pack_refs.len();
        let dmark = self.d.list.len();
        let (me, mu) = (self.d.muted_errors, self.d.muted_unsup);
        let first = parse(self)?;
        if self.at("...") && !self.in_template() && self.pack_refs.len() > mark {
            let n = self.pack_refs[mark];
            if self.pack_refs[mark..].iter().any(|&m| m != n) {
                // PACK-6: the packs of one expansion have one length.
                let line = self.line();
                self.error(line, "PACK-6", "the packs of one expansion have different numbers of elements");
            }
            self.pack_refs.truncate(mark);
            self.d.list.truncate(dmark);
            self.d.muted_errors = me;
            self.d.muted_unsup = mu;
            let end = self.pos;
            let saved = self.pack_index;
            let mut out = Vec::new();
            for k in 0..n {
                self.pos = start;
                self.pack_index = Some(k);
                let x = parse(self);
                self.pack_index = saved;
                out.push(x?);
            }
            self.pos = end + 1;
            return Ok((out, true));
        }
        Ok((vec![first], false))
    }

    /// The copies of a pattern for each element of the packs it names, or
    /// None when it names no pack.
    fn pack_copies(&mut self, parse: impl Fn(&mut P) -> PR<Expr>) -> PR<(Expr, Option<Vec<Expr>>)> {
        let start = self.pos;
        let mark = self.pack_refs.len();
        let dmark = self.d.list.len();
        let first = parse(self)?;
        if self.pack_refs.len() == mark {
            return Ok((first, None));
        }
        let n = self.pack_refs[mark];
        if self.pack_refs[mark..].iter().any(|&m| m != n) {
            // PACK-6: the packs of one expansion have one length.
            let line = self.line();
            self.error(line, "PACK-6", "the packs of one expansion have different numbers of elements");
        }
        self.pack_refs.truncate(mark);
        self.d.list.truncate(dmark);
        let end = self.pos;
        let saved = self.pack_index;
        let mut out = Vec::new();
        for k in 0..n {
            self.pos = start;
            self.pack_index = Some(k);
            let x = parse(self);
            self.pack_index = saved;
            out.push(x?);
        }
        self.pos = end;
        Ok((first, Some(out)))
    }

    /// A fold expression in a specialization, after `(` (PRIM-27 to
    /// PRIM-29).
    pub fn parse_fold(&mut self, line: u32) -> PR<Expr> {
        const OPS: &[&str] = &["+", "-", "*", "/", "%", "^", "&", "|", "<<", ">>", "+=", "-=", "*=", "/=", "%=", "^=", "&=", "|=", "<<=", ">>=", "=", "==", "!=", "<", ">", "<=", ">=", "&&", "||", ","];
        let take_op = |p: &mut P| -> Option<String> {
            let t = p.tok().text.clone();
            if p.tok().kind == TK::Punct && OPS.contains(&t.as_str()) {
                p.pos += 1;
                Some(t)
            } else {
                None
            }
        };
        let bad = |p: &mut P, msg: &str| -> PR<Expr> {
            p.error(line, "PRIM-29", msg.to_string());
            let mut depth = 0;
            while p.tok().kind != TK::Eof {
                if p.at("(") {
                    depth += 1;
                } else if p.at(")") {
                    if depth == 0 {
                        p.pos += 1;
                        break;
                    }
                    depth -= 1;
                }
                p.pos += 1;
            }
            Ok(Expr::unknown(line, "fold expression"))
        };
        if self.eat("...") {
            let Some(op) = take_op(self) else { return bad(self, "a fold expression needs a fold operator") };
            let (_, items) = self.pack_copies(|p| p.parse_cast())?;
            self.expect(")")?;
            let Some(items) = items else { return bad(self, "the operand of a fold expression names no pack") };
            return Ok(self.fold_combine(&op, None, items, true, line));
        }
        let (first, items1) = self.pack_copies(|p| p.parse_cast())?;
        let Some(op) = take_op(self) else { return bad(self, "a fold expression needs a fold operator") };
        if !self.eat("...") {
            return bad(self, "a binary operator in the operand of a fold expression needs parentheses");
        }
        if self.eat(")") {
            let Some(items) = items1 else { return bad(self, "the operand of a fold expression names no pack") };
            return Ok(self.fold_combine(&op, None, items, false, line));
        }
        let Some(op2) = take_op(self) else { return bad(self, "a fold expression needs a fold operator") };
        if op2 != op {
            return bad(self, "the two operators of a fold expression differ");
        }
        let (second, items2) = self.pack_copies(|p| p.parse_cast())?;
        self.expect(")")?;
        match (items1, items2) {
            (Some(p), None) => Ok(self.fold_combine(&op, Some(second), p, false, line)),
            (None, Some(p)) => Ok(self.fold_combine(&op, Some(first), p, true, line)),
            _ => {
                self.error(line, "PRIM-29", "exactly one operand of a fold expression names a pack");
                Ok(Expr::unknown(line, "fold expression"))
            }
        }
    }

    /// PRIM-27: the written-out expression of a fold.
    fn fold_combine(&mut self, op: &str, init: Option<Expr>, items: Vec<Expr>, left: bool, line: u32) -> Expr {
        let bin = |p: &mut P, l: Expr, r: Expr| -> Expr {
            if op == "," {
                return p.build_comma(l, r, line);
            }
            if crate::expr::is_assign_op(op) {
                return p.build_assign(op, l, r, line);
            }
            match BOp::from_str(op) {
                Some(b) => p.build_binary(b, l, r, line),
                None => Expr::unknown(line, "fold expression"),
            }
        };
        let mut v: Vec<Expr> = items;
        if v.is_empty() && init.is_none() {
            return match op {
                "&&" => Expr::new(EK::Int(1), QTy::bool_(), Cat::PR, line),
                "||" => Expr::new(EK::Int(0), QTy::bool_(), Cat::PR, line),
                "," => Expr::new(EK::Conv(Conv::ToVoid, Box::new(Expr::new(EK::Int(0), QTy::int(), Cat::PR, line))), QTy::new(Ty::Void), Cat::PR, line),
                _ => {
                    self.error(line, "PRIM-29", "a fold over an empty pack needs '&&', '||', ',' or an extra operand");
                    Expr::unknown(line, "fold expression")
                }
            };
        }
        if left {
            let mut acc = match init {
                Some(i) => i,
                None => v.remove(0),
            };
            for x in v {
                acc = bin(self, acc, x);
            }
            acc
        } else {
            let mut acc = match init {
                Some(i) => i,
                None => v.pop().unwrap(),
            };
            while let Some(x) = v.pop() {
                acc = bin(self, x, acc);
            }
            acc
        }
    }
}

impl P {
    /// The body of a member function of an instance, made now.
    pub fn run_lazy_body(&mut self, fid: DeclId) {
        let Some((toks, df)) = self.lazy_bodies.remove(&fid) else { return };
        let limit = self.lazy_limits.get(&fid).copied().unwrap_or(usize::MAX);
        let origin = self.lazy_origins.get(&fid).copied().unwrap_or((usize::MAX, 0));
        let saved = self.save_state();
        self.lookup_limit = limit;
        self.tok_origin = origin;
        self.toks = (*toks).clone();
        self.deferred = vec![df];
        self.force_bodies = true;
        self.inst_depth += 1;
        self.run_deferred();
        self.inst_depth -= 1;
        self.force_bodies = false;
        self.restore_state(saved);
    }

    /// A use of a function that needs its definition: the body of a
    /// member of an instance, the definition of a specialization, and
    /// the errors that were kept for a use (INST-7).
    pub fn use_fn(&mut self, fid: DeclId) {
        if self.in_template() {
            return;
        }
        // INST-31: a function that constant evaluation can call, or whose
        // return type is deduced, is made now; any other at the end of
        // the unit, where it sees every declaration.
        let now = self.func(fid).is_some_and(|f| f.constexpr || f.consteval || f.fty.ret.is_unknown() || f.ret_auto.is_some());
        if !now && !self.finishing && (self.lazy_bodies.contains_key(&fid) || self.spec_of.contains_key(&fid)) {
            if !self.used_later.contains(&fid) {
                self.used_later.push(fid);
            }
            if let Some(errs) = self.body_errors.remove(&fid) {
                self.d.list.extend(errs);
            }
            return;
        }
        if self.lazy_bodies.contains_key(&fid) {
            self.run_lazy_body(fid);
        }
        if self.spec_of.contains_key(&fid) {
            self.func_body(fid);
        }
        if let Some(errs) = self.body_errors.remove(&fid) {
            self.d.list.extend(errs);
        }
    }

    /// At the end of the unit: the definitions that no check needed, for
    /// the interpreter. They make no diagnostics, since no use needs
    /// them (INST-5).
    pub fn finish_lazy(&mut self) {
        // INST-20: an explicit instantiation definition whose template is
        // defined later in the unit.
        for (t, args, line, var) in std::mem::take(&mut self.later_explicit) {
            let defined = self.templs[t].src.as_ref().is_some_and(|s| s.toks.iter().any(|x| x.kind == TK::Punct && (x.text == "=" || x.text == "{")));
            if !defined {
                self.error(line, "INST-20", format!("an explicit instantiation of '{}', whose template is not defined in the unit", self.templs[t].name));
                continue;
            }
            if var {
                if let Some(v) = self.var_inst(t, args, line) {
                    if let Some(errs) = self.body_errors.remove(&v) {
                        self.d.list.extend(errs);
                    }
                }
            } else if let SpecRes::Ok(s) = self.func_spec(t, &args, line) {
                self.used_later.push(s);
            }
        }
        // The definitions that uses need, with their diagnostics.
        self.finishing = true;
        let mut k = 0;
        while k < self.used_later.len() {
            let f = self.used_later[k];
            k += 1;
            self.use_fn(f);
        }
        self.finishing = false;
        let mark = self.d.list.len();
        let mut rounds = 0;
        loop {
            rounds += 1;
            let mut todo: Vec<DeclId> = self.lazy_bodies.keys().copied().collect();
            let specs: Vec<DeclId> = self.spec_of.keys().copied().filter(|&d| self.func(d).is_some_and(|f| f.body.is_none() && !f.defaulted && !f.deleted)).collect();
            todo.extend(specs.iter().copied().filter(|d| !self.finished_specs.contains(d)));
            if todo.is_empty() || rounds > 8 {
                break;
            }
            todo.sort();
            for f in todo {
                self.finished_specs.insert(f);
                if self.lazy_bodies.contains_key(&f) {
                    self.run_lazy_body(f);
                } else {
                    self.func_body(f);
                }
            }
        }
        self.d.list.truncate(mark);
        // INST-8: a static data member that no use needed has no
        // definition, and its initialization does not run.
        let unused: Vec<DeclId> = self.lazy_static_defs.drain().collect();
        for v in unused {
            self.static_member_defs.remove(&v);
            if let DK::Var(vi) = &mut self.decls[v].k {
                vi.init = None;
                vi.defined = false;
            }
        }
    }
}

impl P {
    /// The function templates among `fids` replaced by their
    /// specializations for these arguments (DED-1). None when the front
    /// end cannot tell.
    pub fn expand_fn_templates(&mut self, fids: &[DeclId], args: &[Expr], line: u32) -> Option<Vec<DeclId>> {
        let mut out = Vec::new();
        for &f in fids {
            if self.is_func_templ(f) {
                match self.deduce_call(f, None, args, line) {
                    Ded::Ok(a) => {
                        let t = self.decl_tid[&f];
                        match self.func_spec(t, &a, line) {
                            SpecRes::Ok(s) => out.push(s),
                            SpecRes::Fail => {}
                            SpecRes::Unknown => return None,
                        }
                    }
                    Ded::Fail => {}
                    Ded::Unknown => return None,
                }
            } else if self.func(f).is_some_and(|x| x.template) {
                return None;
            } else {
                out.push(f);
            }
        }
        Some(out)
    }

    /// DED-31: the specialization of a conversion function template for
    /// the type that an initialization needs. `p` is the needed type,
    /// without its reference when `to_ref` is Some.
    pub fn conv_spec(&mut self, fid: DeclId, p: &QTy, to_ref: Option<bool>) -> SpecRes {
        let Some(&tid) = self.decl_tid.get(&fid) else { return SpecRes::Unknown };
        let Some(f) = self.func(fid).cloned() else { return SpecRes::Unknown };
        let params = self.templs[tid].params.clone();
        let mut b = Binds::new(self.templs[tid].depth, &params);
        let mut r = nonref(&f.fty.ret);
        if to_ref.is_none() && !f.fty.ret.t.is_ref() {
            r = match &r.t {
                Ty::Array(el, _) => QTy::new(Ty::Ptr(el.clone())),
                Ty::Func(_) => QTy::new(Ty::Ptr(Box::new(r.clone()))),
                _ => r,
            };
        }
        // The exact match first, with the qualifiers of a referred type
        // (outside the text of DED-31, which removes them first).
        let mut t0 = b.clone();
        let exact_ref = f.fty.ret.t.is_ref() && self.deduce_ty(&r, &nonref(p), &mut t0);
        let r = strip_cv(&r, true, true);
        let a = strip_cv(&nonref(p), true, true);
        let mut t = b.clone();
        if exact_ref {
            b = t0;
        } else if self.deduce_ty(&r, &a, &mut t) {
            b = t;
        } else {
            let mut t = b.clone();
            if !self.deduce_qual(&r, &a, &mut t, true) {
                return if t.unknown { SpecRes::Unknown } else { SpecRes::Fail };
            }
            b = t;
        }
        if b.unknown {
            return SpecRes::Unknown;
        }
        let mut args = Vec::new();
        for (i, v) in b.vals.into_iter().enumerate() {
            match v {
                Some(v) => args.push(v),
                None if params[i].pack => args.push(TArg::Pack(vec![])),
                None => {
                    let prev = args.clone();
                    match self.default_arg(tid, i, &prev) {
                        Some(a) if !a.is_dep() => args.push(a),
                        _ => return SpecRes::Fail,
                    }
                }
            }
        }
        self.func_spec(tid, &args, 0)
    }
}

impl P {
    /// DED-30: the specializations of the function templates in `fs`
    /// whose function types match the target type. Plain functions stay.
    /// None when the front end cannot tell.
    pub fn target_specs(&mut self, fs: &[DeclId], explicit: Option<&Rc<Vec<RawArg>>>, to: &QTy, line: u32) -> Option<Vec<DeclId>> {
        let ft = match &to.t {
            Ty::Ptr(p) | Ty::LRef(p) | Ty::RRef(p) => match &p.t {
                Ty::Func(f) => (**f).clone(),
                _ => return None,
            },
            Ty::MemPtr(_, p) => match &p.t {
                Ty::Func(f) => (**f).clone(),
                _ => return None,
            },
            _ => return None,
        };
        let mut out = Vec::new();
        for &f in fs {
            if !self.is_func_templ(f) {
                if self.func(f).is_some_and(|x| x.template) {
                    return None;
                }
                if explicit.is_none() {
                    out.push(f);
                }
                continue;
            }
            let t = self.decl_tid[&f];
            let params = self.templs[t].params.clone();
            let mut b = Binds::new(self.templs[t].depth, &params);
            if let Some(raw) = explicit {
                let me = self.d.muted_errors;
                self.d.muted += 1;
                let r = self.targs_for(t, (**raw).clone(), line, true);
                self.d.muted -= 1;
                match r {
                    Ok(Some(v)) if self.d.muted_errors == me => {
                        for (i, a) in v.into_iter().enumerate() {
                            b.vals[i] = Some(a);
                            b.fixed[i] = true;
                        }
                    }
                    Ok(None) => return None,
                    _ => continue,
                }
            }
            let Some(fi) = self.func(f).cloned() else { continue };
            let mut pf = fi.fty.clone();
            let mut af = ft.clone();
            // A noexcept specialization also matches a target without
            // noexcept (OVL-18).
            if !af.noexcept {
                pf.noexcept = false;
            }
            if pf.params.len() != af.params.len() {
                continue;
            }
            let mut ok = true;
            for (p, a) in pf.params.iter().zip(af.params.iter_mut()) {
                // DED-30: a forwarding reference with an lvalue reference
                // gives `A&`.
                if let (Ty::RRef(pi), Ty::LRef(_)) = (&p.t, &a.t) {
                    if matches!(&pi.t, Ty::Dep(d) if matches!(**d, Dep::Param(dd, _) if dd == b.depth)) && !pi.c && !pi.v {
                        let mut t2 = b.clone();
                        if self.deduce_ty(pi, a, &mut t2) {
                            b = t2;
                            continue;
                        }
                    }
                }
                let mut t2 = b.clone();
                if !self.deduce_ty(p, a, &mut t2) {
                    ok = false;
                    break;
                }
                b = t2;
            }
            if ok && !(fi.ret_auto.is_some() && fi.fty.ret.is_unknown()) {
                let mut t2 = b.clone();
                ok = self.deduce_ty(&pf.ret, &af.ret, &mut t2);
                b = t2;
            }
            if !ok {
                continue;
            }
            if b.unknown {
                return None;
            }
            let mut args = Vec::new();
            let mut complete = true;
            for (i, v) in b.vals.iter().enumerate() {
                match v {
                    Some(v) => args.push(v.clone()),
                    None if params[i].pack => args.push(TArg::Pack(vec![])),
                    None => match self.default_arg(t, i, &args) {
                        Some(a) if !a.is_dep() => args.push(a),
                        _ => complete = false,
                    },
                }
            }
            if !complete {
                continue;
            }
            match self.func_spec(t, &args, line) {
                SpecRes::Ok(s) => out.push(s),
                SpecRes::Fail => {}
                SpecRes::Unknown => return None,
            }
        }
        Some(out)
    }
}

/// A qualified name with a dependent qualifier that the pattern read as a
/// value (DEP-19).
pub const FACT_VALUE_QUAL: u8 = 1;
/// A `<` after a member name of a dependent object that the pattern read
/// as the less-than operator (DEP-24).
pub const FACT_LESS: u8 = 2;
/// An unqualified call that does not depend on a parameter (DEP-3,
/// DEP-5).
pub const FACT_INDEPENDENT_CALL: u8 = 4;
/// The keyword `template` before a name with no argument list after it
/// (DEP-26).
pub const FACT_TEMPLATE_NO_LIST: u8 = 8;

impl P {
    /// The vector and index of the token at `pos` in the current tokens.
    pub fn origin_of(&self, pos: usize) -> (usize, usize) {
        if self.tok_origin.0 == usize::MAX {
            return (usize::MAX, 0);
        }
        (self.tok_origin.0, self.tok_origin.1 + pos)
    }

    /// Record a fact of the pattern's parse at the token `pos`.
    pub fn note_fact(&mut self, pos: usize, fact: u8) {
        if self.in_template() {
            let k = self.origin_of(pos);
            if k.0 != usize::MAX {
                *self.pattern_facts.entry(k).or_default() |= fact;
            }
        }
    }

    /// In a specialization: whether the pattern's parse recorded `fact`
    /// at the token `pos`.
    pub fn has_fact(&self, pos: usize, fact: u8) -> bool {
        if self.inst_depth == 0 || self.in_template() {
            return false;
        }
        let k = self.origin_of(pos);
        k.0 != usize::MAX && self.pattern_facts.get(&k).is_some_and(|f| f & fact != 0)
    }
}

/// DED-12: whether the type `a` fits the type `p` of a constant parameter
/// whose declared type holds a placeholder, which shows as an unknown
/// type. The rest of the two types are the same.
pub fn placeholder_fits(p: &QTy, a: &QTy) -> bool {
    fn go(p: &QTy, a: &QTy, top: bool) -> bool {
        if !top && (p.c != a.c || p.v != a.v) {
            return false;
        }
        match (&p.t, &a.t) {
            (Ty::Unknown, _) => true,
            (Ty::Ptr(x), Ty::Ptr(y)) | (Ty::LRef(x), Ty::LRef(y)) | (Ty::RRef(x), Ty::RRef(y)) => go(x, y, false),
            (x, y) => x == y,
        }
    }
    go(p, a, true)
}
