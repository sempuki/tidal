// Expressions: parsing (precedence from the Constraints segments of the
// expression pages, e.g. MUL-1, ADD-1, SHF-1, CMP-1, BIT-1, LOG-6), and
// the type rules of the built-in operators.

use crate::ast::*;
use crate::decl2::InitForm;
use crate::lexer::{ISize, Lit, Prefix, FSuffix, TK};
use crate::parser::*;
use crate::types::*;
use std::rc::Rc;

fn prec(op: &str) -> Option<u8> {
    Some(match op {
        "||" => 1,
        "&&" => 2,
        "|" => 3,
        "^" => 4,
        "&" => 5,
        "==" | "!=" => 6,
        "<" | ">" | "<=" | ">=" => 7,
        "<=>" => 8,
        "<<" | ">>" => 9,
        "+" | "-" => 10,
        "*" | "/" | "%" => 11,
        ".*" | "->*" => 12,
        _ => return None,
    })
}

pub fn is_assign_op(s: &str) -> bool {
    matches!(s, "=" | "*=" | "/=" | "%=" | "+=" | "-=" | "<<=" | ">>=" | "&=" | "^=" | "|=")
}

impl P {
    // ---------------- parsing ----------------

    pub fn parse_expression(&mut self) -> PR<Expr> {
        let mut e = self.parse_assignment()?;
        while self.at(",") {
            let line = self.line();
            self.pos += 1;
            let r = self.parse_assignment()?;
            e = self.build_comma(e, r, line);
        }
        Ok(e)
    }

    pub fn parse_assignment(&mut self) -> PR<Expr> {
        let line = self.line();
        if self.at("throw") {
            self.pos += 1;
            if self.at(")") || self.at(";") || self.at(",") || self.at(":") || self.at("]") || self.at("}") {
                return Ok(Expr::new(EK::Throw(None), QTy::new(Ty::Void), Cat::PR, line));
            }
            let e = self.parse_assignment()?;
            let e = self.rvalue(e);
            return Ok(Expr::new(EK::Throw(Some(Box::new(e))), QTy::new(Ty::Void), Cat::PR, line));
        }
        if self.at("co_yield") {
            return Err(self.err("coroutines (outside the specification)"));
        }
        let lhs = self.parse_conditional()?;
        let t = self.tok().clone();
        if t.kind == TK::Punct && is_assign_op(&t.text) {
            self.pos += 1;
            let rhs = if self.at("{") { self.parse_braced_init()? } else { self.parse_assignment()? };
            return Ok(self.build_assign(&t.text, lhs, rhs, t.line));
        }
        Ok(lhs)
    }

    pub fn parse_conditional(&mut self) -> PR<Expr> {
        let c = self.parse_binary(1)?;
        if self.at("?") {
            let line = self.line();
            self.pos += 1;
            let save = self.no_gt;
            self.no_gt = 0;
            let a = self.parse_expression();
            self.no_gt = save;
            let a = a?;
            self.expect(":")?;
            let b = self.parse_assignment()?;
            return Ok(self.build_cond(c, a, b, line));
        }
        Ok(c)
    }

    pub fn parse_binary_pub(&mut self, min: u8) -> PR<Expr> {
        self.parse_binary(min)
    }

    pub fn parse_primary_pub(&mut self) -> PR<Expr> {
        self.parse_postfix_name()
    }

    /// A primary expression; a qualified name and a template-id
    /// included.
    fn parse_postfix_name(&mut self) -> PR<Expr> {
        self.parse_primary()
    }

    fn parse_binary(&mut self, min: u8) -> PR<Expr> {
        let mut lhs = self.parse_cast()?;
        loop {
            let t = self.tok().clone();
            if t.kind != TK::Punct {
                break;
            }
            if self.no_gt > 0 && (t.text == ">" || t.text == ">>" || t.text == ">=" || t.text == ">>=") {
                break;
            }
            let Some(p) = prec(&t.text) else { break };
            if p < min {
                break;
            }
            self.pos += 1;
            let rhs = self.parse_binary(p + 1)?;
            lhs = match t.text.as_str() {
                ".*" | "->*" => self.build_member_ptr(&t.text, lhs, rhs, t.line),
                op => self.build_binary(BOp::from_str(op).unwrap(), lhs, rhs, t.line),
            };
        }
        Ok(lhs)
    }

    pub fn parse_cast(&mut self) -> PR<Expr> {
        if self.at("(") && self.is_cast_paren() {
            let line = self.line();
            let save = self.pos;
            self.pos += 1;
            let muted = self.d.list.len();
            self.d.muted += 1;
            let t = self.parse_type_id();
            self.d.muted -= 1;
            if let Ok(t) = t {
                if self.at(")") && !self.at_k(1, "{") || self.at(")") && self.at_k(1, "{") {
                    self.pos += 1;
                    if self.at("{") {
                        // `(T){...}` is not C++
                        return Err(self.err("compound literal"));
                    }
                    let e = self.parse_cast()?;
                    return Ok(self.build_cast(CastKind::CStyle, t, e, line));
                }
            }
            self.d.list.truncate(muted);
            self.pos = save;
        }
        self.parse_unary()
    }

    fn is_cast_paren(&self) -> bool {
        // `(` type-id `)`: the first token must start a type.
        let t = self.peek(1);
        if t.kind != TK::Ident {
            return false;
        }
        let s = t.text.as_str();
        if crate::decl::simple_type_kw(s) || matches!(s, "const" | "volatile" | "struct" | "class" | "union" | "enum" | "typename" | "decltype") {
            if s == "decltype" {
                return true;
            }
            return true;
        }
        if is_keyword(s) {
            return false;
        }
        self.qualified_names_type(1)
    }

    fn parse_unary(&mut self) -> PR<Expr> {
        let t = self.tok().clone();
        let line = t.line;
        if t.kind == TK::Punct {
            let op = match t.text.as_str() {
                "+" => Some(UOp::Plus),
                "-" => Some(UOp::Neg),
                "!" => Some(UOp::Not),
                "~" => Some(UOp::BitNot),
                "*" => Some(UOp::Deref),
                "&" => Some(UOp::AddrOf),
                "++" => Some(UOp::PreInc),
                "--" => Some(UOp::PreDec),
                _ => None,
            };
            if let Some(op) = op {
                self.pos += 1;
                if op == UOp::AddrOf && self.is_member_ptr_operand() {
                    return self.parse_member_pointer_constant(line);
                }
                // `~` before a type name in an expression is a destructor
                // name; not modeled.
                let e = self.parse_cast()?;
                return Ok(self.build_unary(op, e, line));
            }
            if t.text == "::" && (self.at_k(1, "new") || self.at_k(1, "delete")) {
                self.pos += 1;
                return self.parse_unary();
            }
        }
        if t.kind == TK::Ident {
            match t.text.as_str() {
                "sizeof" => {
                    self.pos += 1;
                    if self.eat("...") {
                        // PACK-19, PACK-20: the name of a pack in parentheses.
                        let named = self.at("(") && self.peek(1).kind == TK::Ident && self.at_k(2, ")");
                        if self.inst_depth == 0 && self.in_template() {
                            let is_pack = named && self.lookup(&self.peek(1).text.clone()).is_some_and(|ids| self.pack_decls.contains(&ids[0]) || self.pack_params.contains(&ids[0]));
                            if !is_pack {
                                self.error(line, "PACK-20", "the operand of sizeof... is not the name of a pack in parentheses");
                            }
                        }
                        if !named && !self.at("(") {
                            if self.inst_depth == 0 && !self.in_template() {
                                self.error(line, "PACK-20", "the operand of sizeof... is not the name of a pack in parentheses");
                            }
                            let _ = self.parse_unary();
                            return Ok(Expr::new(EK::Unknown("sizeof...".into()), QTy::new(Ty::size_t()), Cat::PR, line));
                        }
                        if !self.in_template() && self.at("(") && self.peek(1).kind == TK::Ident && self.at_k(2, ")") {
                            // The number of elements of a pack.
                            let name = self.peek(1).text.clone();
                            if let Some(ids) = self.lookup(&name) {
                                let n = self.fn_param_packs.get(&ids[0]).map(|v| v.len()).or_else(|| self.pack_binds.get(&ids[0]).map(|v| v.len()));
                                if let Some(n) = n {
                                    self.pos += 3;
                                    return Ok(Expr::new(EK::Int(n as i128), QTy::new(Ty::size_t()), Cat::PR, line));
                                }
                            }
                        }
                        self.skip_balanced();
                        return Ok(Expr::new(EK::Unknown("sizeof...".into()), QTy::new(Ty::size_t()), Cat::PR, line));
                    }
                    if self.at("(") && self.is_cast_paren() {
                        let save = self.pos;
                        self.pos += 1;
                        let ty = self.parse_type_id();
                        if let Ok(ty) = ty {
                            if self.eat(")") {
                                return Ok(self.build_sizeof(ty, line, false));
                            }
                        }
                        self.pos = save;
                    }
                    self.in_unevaluated += 1;
                    let e = self.parse_unary();
                    self.in_unevaluated -= 1;
                    let e = e?;
                    self.check_unevaluated_dtor(&e, false);
                    if matches!(e.k, EK::Overloads(_) | EK::Func(_)) {
                        self.error(line, "UN-14", "sizeof applied to a function");
                        return Ok(Expr::new(EK::Int(1), QTy::new(Ty::size_t()), Cat::PR, line));
                    }
                    let ty = e.ty.clone();
                    return Ok(self.build_sizeof(ty, line, true));
                }
                "alignof" | "_Alignof" => {
                    self.pos += 1;
                    self.expect("(")?;
                    let ty = self.parse_type_id()?;
                    self.expect(")")?;
                    return Ok(self.build_alignof(ty, line));
                }
                "noexcept" => {
                    self.pos += 1;
                    self.expect("(")?;
                    self.in_unevaluated += 1;
                    let e = self.parse_expression();
                    self.in_unevaluated -= 1;
                    let e = e?;
                    self.expect(")")?;
                    self.check_unevaluated_dtor(&e, false);
                    let v = self.noexcept_of(&e);
                    return Ok(match v {
                        Some(b) => Expr::new(EK::Int(b as i128), QTy::bool_(), Cat::PR, line),
                        None => Expr::new(EK::Unknown("noexcept operator".into()), QTy::bool_(), Cat::PR, line),
                    });
                }
                "new" => return self.parse_new(line),
                "delete" => {
                    let global = self.pos >= 1 && self.toks[self.pos - 1].text == "::";
                    self.pos += 1;
                    let arr = if self.at("[") && self.at_k(1, "]") {
                        self.pos += 2;
                        true
                    } else {
                        false
                    };
                    let e = self.parse_cast()?;
                    let e = self.rvalue(e);
                    let pt = e.ty.clone();
                    self.require_pointee(&pt, line);
                    match &pt.t {
                        Ty::Ptr(_) | Ty::Unknown | Ty::Dep(_) | Ty::Class(_) => {}
                        _ => self.error(line, "NEW-8", "operand of delete must be a pointer"),
                    }
                    let _ = arr;
                    if let Ty::Ptr(x) = &pt.t {
                        if x.t == Ty::Void && !self.in_template() {
                            // NEW-8: a void* operand is not valid.
                            self.error(line, "NEW-8", "operand of delete is a pointer to void");
                        }
                        if let Ty::Class(c) = x.t {
                            if !self.in_template() && self.classes[c].complete && self.dtor_unusable(c) {
                                // DTOR-3: no delete with a deleted destructor.
                                self.error(line, "DTOR-3", format!("delete of '{}', whose destructor is deleted", self.classes[c].name));
                            }
                        }
                    }
                    let user = ["operatordelete", "operatordelete[]"].iter().any(|n| self.scopes[self.global].names.get(*n).is_some_and(|v| !v.is_empty()))
                        || matches!(&pt.t, Ty::Ptr(x) if matches!(x.t, Ty::Class(c) if self.lookup_member(c, "operatordelete").is_some() || self.lookup_member(c, "operatordelete[]").is_some()));
                    if global || user || !matches!(pt.t, Ty::Ptr(_)) || self.in_template() {
                        return Ok(Expr::new(EK::Unknown("delete expression".into()), QTy::new(Ty::Void), Cat::PR, line));
                    }
                    return Ok(Expr::new(EK::Delete(arr, Box::new(e)), QTy::new(Ty::Void), Cat::PR, line));
                }
                "co_await" => return Err(self.err("coroutines (outside the specification)")),
                _ => {}
            }
        }
        self.parse_postfix()
    }

    fn is_member_ptr_operand(&self) -> bool {
        // & C::m  or & ::N::C::m
        let mut k = 0;
        if self.at_k(0, "::") {
            k = 1;
        }
        let mut saw = false;
        loop {
            if self.peek(k).kind != TK::Ident {
                return false;
            }
            k += 1;
            if self.at_k(k, "<") {
                k = self.skip_angle_at(k);
            }
            if self.at_k(k, "::") {
                saw = true;
                k += 1;
                continue;
            }
            break;
        }
        if !saw {
            return false;
        }
        // the qualifier must name a class
        let mut kk = 0;
        if self.at_k(0, "::") {
            kk = 1;
        }
        let _ = kk;
        // find the last qualifier and check it is a class
        let save_pos = self.pos;
        let _ = save_pos;
        true
    }

    fn parse_member_pointer_constant(&mut self, line: u32) -> PR<Expr> {
        let save = self.pos;
        let scope = self.parse_nested_name()?;
        let Some(Ok(s)) = scope else {
            self.pos = save;
            let e = self.parse_cast()?;
            return Ok(self.build_unary(UOp::AddrOf, e, line));
        };
        let SK::Class(c) = self.scopes[s].kind else {
            self.pos = save;
            let e = self.parse_cast()?;
            return Ok(self.build_unary(UOp::AddrOf, e, line));
        };
        let name = if self.eat("operator") {
            let mut s = String::from("operator");
            s.push_str(&self.tok().text.clone());
            self.pos += 1;
            if s == "operator(" || s == "operator[" {
                self.pos += 1;
            }
            s
        } else {
            self.ident()?
        };
        if name == self.classes[c].name && !self.in_template() {
            // CTOR-1: a constructor has no address.
            self.error(line, "CTOR-1", "the address of a constructor");
            return Ok(Expr::unknown(line, "pointer to member"));
        }
        let found = self.lookup_member(c, &name);
        let Some(ids) = found else {
            if !self.class_uncertain(c) {
                self.error(line, "NAM-8", format!("no member named '{}'", name));
            }
            return Ok(Expr::unknown(line, "pointer to member"));
        };
        let id = ids[0];
        if ids.iter().filter(|&&i| self.func(i).is_some()).count() > 1 {
            // OVL-18: a target type chooses the function later.
            let fs: Vec<DeclId> = ids.iter().copied().filter(|&i| self.func(i).is_some()).collect();
            let set = Expr::new(EK::Overloads(fs), QTy::unknown(), Cat::L, line);
            return Ok(Expr::new(EK::Unary(UOp::AddrOf, Box::new(set)), QTy::unknown(), Cat::PR, line));
        }
        match self.decls[id].k.clone() {
            DK::Field(f) => {
                if f.ty.t.is_ref() {
                    self.error(line, "MPTR-8", "pointer to a reference member");
                }
                // MPTR-2: the class is the one that declares the member.
                let t = QTy::new(Ty::MemPtr(f.class, Box::new(f.ty.clone())));
                Ok(Expr::new(EK::Unknown("pointer to member constant".into()), t, Cat::PR, line))
            }
            DK::Func(f) if f.explicit_obj => {
                // CLS-17: an ordinary pointer to function.
                let _ = f;
                Ok(Expr::unknown(line, "address of an explicit object member function"))
            }
            DK::Func(f) if !f.is_static => {
                let t = QTy::new(Ty::MemPtr(c, Box::new(QTy::new(Ty::Func(Box::new(f.fty.clone()))))));
                Ok(Expr::new(EK::Unknown("pointer to member function".into()), t, Cat::PR, line))
            }
            DK::Func(f) => {
                let t = QTy::new(Ty::Ptr(Box::new(QTy::new(Ty::Func(Box::new(f.fty.clone()))))));
                if !self.in_template() && self.fn_cons.contains_key(&id) && self.fn_sat(id) == crate::cons::Sat::No {
                    // CNS-12: the address of a function whose constraints are
                    // not satisfied.
                    self.error(line, "CNS-12", format!("the address of '{}', whose constraints are not satisfied", self.decls[id].name));
                    return Ok(Expr::unknown(line, "bad address"));
                }
                if self.in_template() || f.template {
                    return Ok(Expr::new(EK::Unknown("address of static member function".into()), t, Cat::PR, line));
                }
                let fe = Expr::new(EK::Func(id), QTy::new(Ty::Func(Box::new(f.fty.clone()))), Cat::L, line);
                Ok(Expr::new(EK::Unary(UOp::AddrOf, Box::new(fe)), t, Cat::PR, line))
            }
            DK::Var(v) => {
                let t = QTy::new(Ty::Ptr(Box::new(v.ty.clone())));
                let ve = Expr::new(EK::Var(id), v.ty.clone(), Cat::L, line);
                Ok(Expr::new(EK::Unary(UOp::AddrOf, Box::new(ve)), t, Cat::PR, line))
            }
            _ => Ok(Expr::unknown(line, "pointer to member")),
        }
    }

    /// NEW-21 to NEW-25: the deallocation function that a throw from the
    /// initialization would call must be accessible and not deleted, and
    /// with placement arguments it must not be a usual form.
    fn check_new_deallocation(&mut self, t: &QTy, array: bool, placement: Option<&[Expr]>, global: bool, line: u32) {
        let Ty::Class(c) = t.t else { return };
        if global || !self.classes[c].complete {
            return;
        }
        let dname = if array { "operatordelete[]" } else { "operatordelete" };
        let nname = if array { "operatornew[]" } else { "operatornew" };
        let Some(ds) = self.lookup_member(c, dname) else { return };
        let ds: Vec<DeclId> = ds.into_iter().filter(|&d| self.func(d).is_some()).collect();
        if ds.is_empty() {
            return;
        }
        let size_t = Ty::size_t();
        let usual = |p: &P, d: DeclId| -> bool {
            p.func(d).is_some_and(|f| matches!(&f.fty.params.first().map(|q| q.t.clone()), Some(Ty::Ptr(x)) if x.t == Ty::Void) && (f.fty.params.len() == 1 || (f.fty.params.len() == 2 && f.fty.params[1].t == size_t)))
        };
        let chosen = match placement {
            None => {
                let one = ds.iter().copied().find(|&d| usual(self, d) && self.func(d).is_some_and(|f| f.fty.params.len() == 1));
                one.or_else(|| ds.iter().copied().find(|&d| usual(self, d)))
            }
            Some(pa) => {
                // The allocation function with the placement parameters.
                let Some(ns) = self.lookup_member(c, nname) else { return };
                let Some(alloc) = ns.into_iter().find(|&n| self.func(n).is_some_and(|f| f.fty.params.len() == 1 + pa.len())) else { return };
                let after: Vec<QTy> = self.func(alloc).unwrap().fty.params[1..].to_vec();
                let m = ds.iter().copied().find(|&d| self.func(d).is_some_and(|f| f.fty.params.len() == 1 + after.len() && f.fty.params[1..] == after[..]));
                if let Some(d) = m {
                    if usual(self, d) {
                        self.error(line, "NEW-25", "the deallocation function that matches the placement allocation function is a usual form");
                        return;
                    }
                }
                m
            }
        };
        let Some(d) = chosen else { return };
        let deleted = self.func(d).is_some_and(|f| f.deleted);
        if deleted {
            self.error(line, "NEW-25", format!("the deallocation function of '{}' that a new expression would call is deleted", self.classes[c].name));
        } else if !self.accessible(d, false) {
            self.error(line, "NEW-25", format!("the deallocation function of '{}' that a new expression would call is not accessible", self.classes[c].name));
        }
    }

    /// NEW-7, NEW-11, NEW-16, NEW-17, NEW-20: the type, bounds, and
    /// initializer of a `new` expression. `init` is (braced, number of
    /// values) when there is an initializer.
    #[allow(clippy::too_many_arguments)]
    fn check_new_type(&mut self, t: &QTy, is_auto: bool, bounds: &[Option<u64>], first_absent: bool, first_const: Option<i128>, init: Option<(bool, usize)>, placement: Option<&[Expr]>, global: bool, line: u32) {
        self.require_ty(t, line);
        if is_auto {
            if !bounds.is_empty() {
                self.error(line, "NEW-16", "array of 'auto' in new");
            } else if init.is_none_or(|(_, n)| n != 1) {
                self.error(line, "NEW-16", "'new auto' needs exactly one initializer value");
            }
            return;
        }
        if t.is_unknown() {
            return;
        }
        match &t.t {
            Ty::Func(_) => self.error(line, "NEW-7", "new of a function type"),
            Ty::Class(c) if !self.classes[*c].complete => self.error(line, "NEW-7", format!("new of the incomplete class '{}'", self.classes[*c].name)),
            Ty::Class(_) if self.is_abstract(&t.t) => self.error(line, "NEW-7", "new of an abstract class"),
            _ => {}
        }
        if t.c && init.is_none() && bounds.is_empty() && self.is_scalar(t) {
            self.error(line, "NEW-11", "new of a const scalar without an initializer");
        }
        if first_absent && init.is_none() {
            self.error(line, "NEW-17", "new of an array without a size or an initializer");
        }
        if let (Some(n), Some((true, k))) = (first_const, init) {
            if n >= 0 && k as i128 > n {
                self.error(line, "NEW-17", "more initializer values than the array size in new");
            }
        }
        if let Some(pargs) = placement {
            let np = pargs.len();
            // NEW-20: a declared allocation function must accept the
            // placement arguments; NEW-19's functions accept none.
            let name = if bounds.is_empty() { "operatornew" } else { "operatornew[]" };
            let class_fn = match &t.t {
                Ty::Class(c) if !global => self.lookup_member(*c, name).is_some() || self.class_uncertain(*c),
                _ => false,
            };
            let fns = self.scopes[self.global].names.get(name).cloned().unwrap_or_default();
            let fits_count: Vec<DeclId> = fns.iter().copied().filter(|&f| self.func(f).is_none_or(|fi| fi.template || fi.fty.variadic || (fi.fty.params.len() >= 1 + np && fi.fty.params.len() - fi.defaults.min(fi.fty.params.len()) <= 1 + np))).collect();
            // NEW-20, PFX-6: each placement argument converts to its
            // parameter type.
            let fits = fits_count.iter().any(|&f| {
                let Some(fi) = self.func(f).cloned() else { return true };
                if fi.template || fi.fty.variadic {
                    return true;
                }
                pargs.iter().enumerate().all(|(i, a)| !matches!(self.ics_form(a, &fi.fty.params[i + 1], false), Ok(None)))
            });
            if !class_fn && !fits && !self.scopes[self.global].uncertain {
                self.error(line, "NEW-20", format!("no declared '{}' takes {} placement argument{}", if bounds.is_empty() { "operator new" } else { "operator new[]" }, np, if np == 1 { "" } else { "s" }));
            }
        }
    }

    fn parse_new(&mut self, line: u32) -> PR<Expr> {
        let global = self.pos >= 1 && self.toks[self.pos - 1].text == "::";
        self.expect("new")?;
        let mut placement: Option<Vec<Expr>> = None;
        if self.at("(") {
            // placement args or parenthesized type
            let save = self.pos;
            self.pos += 1;
            let is_type = self.starts_type();
            if is_type {
                let nerr = self.d.list.len();
                self.d.muted += 1;
                let t = self.parse_type_id();
                self.d.muted -= 1;
                self.d.list.truncate(nerr);
                if let Ok(t) = t {
                    if self.eat(")") {
                        self.parse_new_init()?;
                        return Ok(Expr::new(EK::Unknown("new expression".into()), QTy::new(Ty::Ptr(Box::new(t))), Cat::PR, line));
                    }
                }
            }
            self.pos = save;
            self.pos += 1;
            placement = Some(self.parse_call_args()?);
        }
        let ds = self.parse_decl_specs(None)?;
        let is_auto = ds.auto.is_some() && ds.ty.is_none();
        let mut base = self.ds_base(&ds).unwrap_or(QTy::unknown());
        if let Some(ctid) = ds.ctad {
            // CTAD-1: a new expression with a deduced class type.
            if !self.in_template() && (self.at("(") || self.at("{")) {
                if let Some(q) = self.ctad_lookahead(ctid, line) {
                    base = add_cv(&q, ds.c > 0, ds.v > 0);
                }
            }
        }
        let mut t = base;
        // new-declarator: ptr ops and array bounds
        loop {
            if self.eat("*") {
                let (c, v) = self.parse_cv();
                t = QTy { t: Ty::Ptr(Box::new(t)), c, v };
            } else {
                break;
            }
        }
        if self.at("&") || self.at("&&") {
            // NEW-7: the type must not be a reference.
            self.pos += 1;
            if !self.in_template() {
                self.error(line, "NEW-7", "new of a reference type");
            }
            t = QTy::unknown();
        }
        let mut bounds: Vec<Option<u64>> = Vec::new();
        let mut first_absent = false;
        let mut first_const: Option<i128> = None;
        let mut first_bound: Option<Expr> = None;
        while self.at("[") {
            self.pos += 1;
            let bl = self.line();
            let mut b = None;
            if bounds.is_empty() && self.at("]") {
                first_absent = true;
            }
            if !self.at("]") {
                let e = self.parse_expression()?;
                if bounds.is_empty() {
                    first_bound = Some(self.rvalue(e.clone()));
                }
                let et = nonref(&e.ty);
                if matches!(et.t, Ty::Ptr(_)) {
                    // NEW-2: the size converts to the type of sizeof.
                    self.error(bl, "NEW-17", "array size in new must not be a pointer");
                }
                if bounds.is_empty() && !self.in_template() && et.t.is_integral() {
                    if let Ok(v) = self.const_int(&e) {
                        first_const = Some(v);
                        if v < 0 {
                            // NEW-17: a constant size must not be negative.
                            self.error(bl, "NEW-17", "negative constant array size in new");
                        }
                    }
                }
                if !bounds.is_empty() {
                    match self.const_int(&e) {
                        Ok(v) if v > 0 => b = Some(v as u64),
                        Ok(_) => self.error(bl, "NEW-17", "array bound after the first must be positive"),
                        Err(crate::consteval::CeErr::NotConst(..)) => self.error(bl, "NEW-11", "array bound after the first is not a constant"),
                        Err(_) => b = Some(u64::MAX),
                    }
                }
            }
            self.expect("]")?;
            bounds.push(b);
        }
        let has_init = self.at("(") || self.at("{");
        if !has_init && !self.in_template() {
            // INIT-9: default initialization of the new object.
            if let Ty::Class(c) = t.t {
                if self.classes[c].complete && !self.is_abstract(&t.t) {
                    self.check_default_init(c, line);
                }
            }
        }
        self.new_paren_args.clear();
        self.new_braced = None;
        let init = self.parse_new_init()?;
        let pargs = std::mem::take(&mut self.new_paren_args);
        let braced_init = self.new_braced.take();
        if let (Ty::Class(c), true, Some((false, _))) = (&t.t, bounds.is_empty(), init) {
            // CTOR-25: a constructor must accept the arguments.
            if !self.class_uncertain(*c) && !self.in_template() && self.ctor_no_match(*c, &pargs) {
                self.error(line, "CTOR-25", format!("no constructor of '{}' accepts the arguments", self.classes[*c].name));
            }
        }
        if !self.in_template() {
            self.check_new_type(&t, is_auto, &bounds, first_absent, first_const, init, placement.as_deref(), global, line);
            self.check_new_deallocation(&t, !bounds.is_empty(), placement.as_deref(), global, line);
        }
        // NEW-2: the result points to the first element, which is an array
        // for a multi-dimensional new.
        let mut rt = t;
        for b in bounds.iter().skip(1).rev() {
            rt = match b {
                Some(u64::MAX) => QTy::unknown(),
                Some(n) => QTy::new(Ty::Array(Box::new(rt), Some(*n))),
                None => QTy::unknown(),
            };
        }
        if rt.t == Ty::Void {
            self.error(line, "NEW-7", "new of void");
        }
        let unknown = Expr::new(EK::Unknown("new expression".into()), QTy::new(Ty::Ptr(Box::new(rt.clone()))), Cat::PR, line);
        // The expression for the interpreter: the plain forms only, with
        // the global allocation function (NEW-13, NEW-19).
        let user_alloc = |p: &P, c: Option<usize>| -> bool {
            let names = ["operatornew", "operatornew[]", "operatordelete", "operatordelete[]"];
            names.iter().any(|n| p.scopes[p.global].names.get(*n).is_some_and(|v| !v.is_empty()) || c.is_some_and(|c| p.lookup_member(c, n).is_some()))
        };
        let elem_class = crate::decl2::elem_class(&rt.t);
        // STG-5: the non-allocating placement form, declared as the
        // library declares it, with one pointer argument.
        let mut place_arg: Option<Expr> = None;
        if let Some(pa) = &placement {
            let forms_only = |p: &P, name: &str| -> bool {
                p.scopes[p.global].names.get(name).is_none_or(|v| v.iter().all(|&d| p.func(d).is_some_and(|f| f.fty.params.len() == 2 && matches!(&f.fty.params[1].t, Ty::Ptr(x) if x.t == Ty::Void))))
            };
            let has_form = |p: &P, name: &str| p.scopes[p.global].names.get(name).is_some_and(|v| v.iter().any(|&d| p.func(d).is_some_and(|f| f.fty.params.len() == 2 && matches!(&f.fty.params[1].t, Ty::Ptr(x) if x.t == Ty::Void))));
            let name = if bounds.is_empty() { "operatornew" } else { "operatornew[]" };
            let class_alloc = elem_class.is_some_and(|c| ["operatornew", "operatornew[]", "operatordelete", "operatordelete[]"].iter().any(|n| self.lookup_member(c, n).is_some()));
            if pa.len() == 1 && matches!(nonref(&pa[0].ty).t, Ty::Ptr(_) | Ty::Array(..)) && has_form(self, name) && forms_only(self, "operatornew") && forms_only(self, "operatornew[]") && !class_alloc && !self.in_template() && !global {
                let vp = QTy::new(Ty::Ptr(Box::new(QTy { t: Ty::Void, c: false, v: false })));
                let mark = self.d.list.len();
                let a = self.convert_init(pa[0].clone(), &vp, InitForm::Copy, line);
                if self.d.list.len() == mark && !matches!(a.k, EK::Unknown(_)) {
                    place_arg = Some(a);
                }
                self.d.list.truncate(mark);
            }
        }
        if (placement.is_some() && place_arg.is_none()) || global || self.in_template() || rt.is_unknown() || (place_arg.is_none() && user_alloc(self, elem_class)) || bounds.iter().skip(1).any(|b| b.is_none()) {
            return Ok(unknown);
        }
        let mark = self.d.list.len();
        let count = if bounds.is_empty() { None } else { first_bound.clone() };
        if !bounds.is_empty() && count.is_none() {
            return Ok(unknown);
        }
        let conv_init = match (init, braced_init) {
            (None, _) => None,
            (Some((true, _)), Some(b)) => {
                let t = if bounds.is_empty() { rt.clone() } else { QTy::new(Ty::Array(Box::new(rt.clone()), Some(first_const.unwrap_or(0).max(0) as u64))) };
                Some(self.convert_init(b, &t, InitForm::DirectList, line))
            }
            (Some((false, n)), _) => {
                if !bounds.is_empty() {
                    if n != 0 {
                        return Ok(unknown);
                    }
                    Some(Expr::new(EK::ZeroInit, rt.clone(), Cat::PR, line))
                } else if n == 0 {
                    let e = Expr::new(EK::InitList(vec![]), QTy::unknown(), Cat::PR, line);
                    Some(self.convert_init(e, &rt, InitForm::DirectList, line))
                } else {
                    let e = Expr::new(EK::InitList(pargs.clone()), QTy::unknown(), Cat::PR, line);
                    Some(self.convert_init(e, &rt, InitForm::Paren(n), line))
                }
            }
            _ => return Ok(unknown),
        };
        if self.d.list.len() != mark || conv_init.as_ref().is_some_and(|e| matches!(e.k, EK::Unknown(_))) {
            self.d.list.truncate(mark);
            return Ok(unknown);
        }
        let k = EK::New { ty: rt.clone(), count: count.map(Box::new), init: conv_init.map(Box::new), place: place_arg.map(Box::new) };
        Ok(Expr::new(k, QTy::new(Ty::Ptr(Box::new(rt))), Cat::PR, line))
    }

    /// DTOR-3: a class prvalue written where its destructor is deleted
    /// or not accessible.
    /// DTOR-3 for an unevaluated operand: every class prvalue in the
    /// operand, except a call that is the operand of `decltype` (also as
    /// the right operand of a comma).
    pub fn check_unevaluated_dtor(&mut self, e: &Expr, decltype: bool) {
        if self.in_template() {
            return;
        }
        let mut top = e;
        let mut spine: Vec<*const Expr> = Vec::new();
        while let EK::Comma(_, r) = &top.k {
            spine.push(top);
            top = r;
        }
        let exempt_call = decltype && matches!(top.k, EK::Call(..));
        if exempt_call {
            spine.push(top);
        } else {
            spine.clear();
        }
        let mut found: Option<(usize, u32)> = None;
        walk_expr(e, &mut |x: &Expr| {
            if found.is_some() || spine.iter().any(|&p| std::ptr::eq(x, p)) || x.cat != Cat::PR {
                return;
            }
            if let Ty::Class(c) = x.ty.t {
                found = Some((c, x.line));
            }
        });
        if let Some((c, line)) = found {
            if self.dtor_unusable(c) {
                self.error(line, "DTOR-3", format!("prvalue of '{}', whose destructor is deleted or not accessible", self.classes[c].name));
            }
        }
    }

    pub fn check_prvalue_dtor(&mut self, e: &Expr) {
        if e.cat != Cat::PR || self.in_template() || self.in_unevaluated > 0 {
            return;
        }
        if let Ty::Class(c) = e.ty.t {
            if self.dtor_unusable(c) {
                self.error(e.line, "DTOR-3", format!("prvalue of '{}', whose destructor is deleted or not accessible", self.classes[c].name));
            }
        }
    }

    fn parse_new_init(&mut self) -> PR<Option<(bool, usize)>> {
        if self.at("(") {
            self.pos += 1;
            let mut n = 0;
            let mut args = Vec::new();
            if !self.at(")") {
                loop {
                    if self.at("{") {
                        n += 1;
                        args.push(self.parse_braced_init()?);
                    } else {
                        // PACK-13: an expansion gives one argument for each
                        // element.
                        let (items, expanded) = self.parse_expansion(|p| p.parse_assignment())?;
                        if !expanded && self.eat("...") {
                            n += 1;
                            args.push(Expr::unknown(self.line(), "pack expansion"));
                        } else {
                            for a in items {
                                n += 1;
                                self.check_prvalue_dtor(&a);
                                args.push(a);
                            }
                        }
                    }
                    if !self.eat(",") {
                        break;
                    }
                }
            }
            self.expect(")")?;
            self.new_paren_args = args;
            return Ok(Some((false, n)));
        } else if self.at("{") {
            let e = self.parse_braced_init()?;
            let n = match &e.k {
                EK::InitList(v) => v.len(),
                _ => 1,
            };
            self.new_braced = Some(e);
            return Ok(Some((true, n)));
        }
        Ok(None)
    }

    fn parse_postfix(&mut self) -> PR<Expr> {
        let mut e = self.parse_primary()?;
        loop {
            let line = self.line();
            if self.at("[") {
                if self.at_k(1, "[") {
                    // ATTR-17: `[[` only introduces an attribute.
                    self.error(line, "ATTR-17", "'[[' that does not start an attribute");
                }
                self.pos += 1;
                if self.at("]") && matches!(nonref(&e.ty).t, Ty::Class(_) | Ty::Unknown | Ty::Dep(_)) {
                    // an overloaded operator[] with no parameters (OPR)
                    self.pos += 1;
                    e = Expr::unknown(line, "overloaded operator[]");
                    continue;
                }
                let save = self.no_gt;
                self.no_gt = 0;
                let idx = if self.at("{") { self.parse_braced_init() } else { self.parse_assignment() };
                let mut extra = false;
                while self.at(",") {
                    self.pos += 1;
                    let _ = self.parse_assignment();
                    extra = true;
                }
                self.no_gt = save;
                let idx = idx?;
                self.expect("]")?;
                if extra && !matches!(nonref(&e.ty).t, Ty::Class(_) | Ty::Unknown | Ty::Dep(_)) {
                    // COMMA-6: the built-in subscript takes one argument.
                    self.error(line, "COMMA-6", "comma in a built-in subscript");
                }
                e = if extra {
                    // Several arguments for a member operator[] (OPR-3) are
                    // not modeled.
                    Expr::unknown(line, "operator[] with several arguments")
                } else {
                    self.build_subscript(e, idx, line)
                };
            } else if self.at("(") {
                let adl = self.adl_pos == Some(self.pos);
                let open = self.pos;
                self.pos += 1;
                let save = self.no_gt;
                self.no_gt = 0;
                let args = self.parse_call_args();
                self.no_gt = save;
                let args = args?;
                if adl && self.in_template() && !args.iter().any(|a| a.ty.is_unknown()) {
                    // DEP-3: a call whose arguments do not depend on a
                    // parameter is bound at the definition.
                    self.note_fact(open, crate::templ::FACT_INDEPENDENT_CALL);
                }
                self.call_adl_independent = adl && self.has_fact(open, crate::templ::FACT_INDEPENDENT_CALL);
                self.call_adl = adl;
                e = self.build_call(e, args, line);
            } else if self.at(".") || self.at("->") {
                let arrow = self.at("->");
                self.pos += 1;
                self.eat("template");
                e = self.parse_member_access(e, arrow, line)?;
                if let EK::BoundMember(_, fs) = &e.k {
                    let non_static = fs.iter().all(|&f| self.func(f).is_some_and(|fi| !fi.is_static) && !self.decls[f].name.starts_with("operator"));
                    if !self.at("(") && non_static && !fs.is_empty() && !self.in_template() {
                        // PFX-14: a non-static member function reached
                        // through member access is called.
                        self.error(line, "PFX-14", "a member function named through member access is not called");
                    }
                }
            } else if self.at("++") || self.at("--") {
                let op = if self.at("++") { UOp::PostInc } else { UOp::PostDec };
                self.pos += 1;
                e = self.build_unary(op, e, line);
            } else {
                break;
            }
        }
        Ok(e)
    }

    pub fn parse_call_args(&mut self) -> PR<Vec<Expr>> {
        let mut args = Vec::new();
        if self.eat(")") {
            return Ok(args);
        }
        loop {
            let (items, expanded) = self.parse_expansion(|p| if p.at("{") { p.parse_braced_init() } else { p.parse_assignment() })?;
            if !expanded {
                self.eat("...");
            }
            args.extend(items);
            if self.eat(",") {
                continue;
            }
            self.expect(")")?;
            return Ok(args);
        }
    }

    /// Whether the qualified name at `start` starts with a template
    /// parameter or a template-id with a dependent argument.
    pub fn qual_starts_with_tparam(&self, start: usize) -> bool {
        let t = &self.toks[start.min(self.toks.len() - 1)];
        if t.kind != TK::Ident {
            return false;
        }
        match self.lookup(&t.text) {
            Some(ids) => ids.iter().any(|&i| self.tparam.contains_key(&i)),
            None => false,
        }
    }

    /// At `<` after a member name: the argument list is followed by `(`.
    fn at_member_call_after_targs(&self) -> bool {
        let k = self.skip_angle_at(0);
        self.at_k(k, "(")
    }

    fn parse_member_access(&mut self, obj: Expr, arrow: bool, line: u32) -> PR<Expr> {
        // INST-1: a member access needs the class complete.
        if arrow {
            self.require_pointee(&obj.ty, line);
        } else {
            self.require_ty(&nonref(&obj.ty), line);
        }
        // destructor call
        if self.at("~") {
            self.pos += 1;
            let mut named: Option<Ty> = None;
            if self.at("decltype") {
                self.pos += 1;
                self.skip_balanced();
            } else {
                if self.is_ident() && !self.at_k(1, "::") && !self.at_k(1, "<") {
                    named = self.lookup(&self.tok().text.clone()).and_then(|ids| match &self.decls[ids[0]].k {
                        DK::Class(b) => Some(Ty::Class(*b)),
                        DK::Type(q) => Some(q.t.clone()),
                        _ => None,
                    });
                }
                self.skip_qualified_name()?;
            }
            if !self.in_template() {
                // DTOR-8: the name after `~` denotes the object's type, and
                // the destructor is called.
                let ot = nonref(&obj.ty);
                let objt = match (&ot.t, arrow) {
                    (Ty::Ptr(p), true) => Some(p.t.clone()),
                    (t, false) => Some(t.clone()),
                    _ => None,
                };
                if let (Some(n), Some(o)) = (&named, &objt) {
                    if *o != Ty::Unknown && *n != Ty::Unknown && n != o {
                        self.error(line, "DTOR-8", "the name after '~' does not denote the type of the object");
                    }
                }
                if !self.at("(") {
                    self.error(line, "DTOR-8", "a destructor must be called with '()'");
                }
            }
            if !self.in_template() && self.at("(") && self.at_k(1, ")") {
                // STG-6: the call ends the object's lifetime.
                let target = if arrow {
                    match &nonref(&obj.ty).t {
                        Ty::Ptr(p) => Some(Expr::new(EK::Unary(UOp::Deref, Box::new(self.rvalue(obj.clone()))), (**p).clone(), Cat::L, line)),
                        _ => None,
                    }
                } else if obj.cat != Cat::PR {
                    Some(obj.clone())
                } else {
                    None
                };
                if let Some(t) = target {
                    if !nonref(&t.ty).is_unknown() {
                        self.pos += 2;
                        return Ok(Expr::new(EK::Destroy(Box::new(t)), QTy::new(Ty::Void), Cat::PR, line));
                    }
                }
            }
            return Ok(Expr::new(EK::Unknown("pseudo destructor".into()), QTy::unknown(), Cat::PR, line));
        }
        let scope = self.parse_nested_name()?;
        let name = if self.eat("operator") {
            let mut d = crate::decl::Declarator {
                name: None,
                kind: crate::decl::NameKind::Plain,
                qual: None,
                qual_unknown: false,
                ops: vec![],
                line,
                conv_ty: None,
                template_args: false,
                pack: false,
            };
            self.pos -= 1;
            self.parse_declarator_name_pub(&mut d)?;
            d.name.unwrap_or_default()
        } else {
            self.ident()?
        };
        let name_pos = self.pos.saturating_sub(1);
        if self.at("<") && self.in_template() && obj.ty.is_unknown() && !(name_pos >= 2 && self.toks[name_pos - 1].text == "template") {
            // DEP-24: without `template` the `<` is the less-than operator.
            self.note_fact(name_pos, crate::templ::FACT_LESS);
        }
        if self.at("<") && self.has_fact(name_pos, crate::templ::FACT_LESS) {
            let ot = nonref(&obj.ty);
            let c = match (&ot.t, arrow) {
                (Ty::Class(c), false) => Some(*c),
                (Ty::Ptr(p), true) => if let Ty::Class(c) = p.t { Some(c) } else { None },
                _ => None,
            };
            if c.and_then(|c| self.lookup_member(c, &name)).is_some_and(|ids| ids.iter().any(|&i| self.func(i).is_some_and(|f| f.template) || matches!(self.decls[i].k, DK::Template(_)))) {
                self.error(line, "DEP-24", format!("the template reads '<' after '{}' as the less-than operator; the member template needs 'template'", name));
                self.skip_template_args()?;
                if self.at("(") {
                    self.skip_balanced();
                }
                return Ok(Expr::unknown(line, "member template without template"));
            }
        }
        if self.at("<") {
            // member template arguments
            let ot = nonref(&obj.ty);
            let class = match (&ot.t, arrow) {
                (Ty::Class(c), false) => Some(*c),
                (Ty::Ptr(p), true) => if let Ty::Class(c) = p.t { Some(c) } else { None },
                _ => None,
            };
            let found = class.and_then(|c| self.lookup_member(c, &name));
            let is_templ = found.as_ref().is_some_and(|ids| ids.iter().any(|&i| self.func(i).is_some_and(|f| f.template) || matches!(self.decls[i].k, DK::Template(_))));
            let fn_templ = found.as_ref().is_some_and(|ids| ids.iter().any(|&i| self.is_func_templ(i)));
            if fn_templ && !self.in_template() && self.at_member_call_after_targs() {
                // DED-2: a member function template with explicit
                // arguments.
                let fids: Vec<DeclId> = found.unwrap().into_iter().filter(|&i| self.func(i).is_some()).collect();
                let raw = self.parse_raw_targs()?;
                let o = if arrow {
                    let o = self.rvalue(obj);
                    let pt = match &o.ty.t {
                        Ty::Ptr(p) => (**p).clone(),
                        _ => QTy::unknown(),
                    };
                    Expr::new(EK::Unary(UOp::Deref, Box::new(o)), pt, Cat::L, line)
                } else {
                    obj
                };
                return Ok(Expr::new(EK::TemplId(Some(Box::new(o)), fids, std::rc::Rc::new(raw)), QTy::unknown(), Cat::L, line));
            }
            if is_templ || ot.is_unknown() {
                self.skip_template_args()?;
                return Ok(Expr::unknown(line, "member template"));
            }
        }
        // object expression
        let obj = if arrow {
            let o = self.rvalue(obj);
            match &o.ty.t {
                Ty::Ptr(p) => {
                    let pt = (**p).clone();
                    Expr::new(EK::Unary(UOp::Deref, Box::new(o)), pt, Cat::L, line)
                }
                Ty::Unknown | Ty::Dep(_) => return Ok(Expr::unknown(line, "member of unknown")),
                Ty::Class(c) => {
                    let c = *c;
                    if !self.in_template() && self.classes[c].complete && !self.class_uncertain(c) && self.lookup_member(c, "operator->").is_none() {
                        // PFX-11: `->` needs a pointer, or a class with
                        // `operator->`.
                        self.error(line, "PFX-11", format!("'->' applied to an object of class '{}', which has no operator->", self.classes[c].name));
                        return Ok(Expr::unknown(line, "bad ->"));
                    }
                    return Ok(Expr::unknown(line, "overloaded operator->"));
                }
                _ => {
                    self.error(line, "PFX-11", "'->' applied to a non-pointer");
                    return Ok(Expr::unknown(line, "bad ->"));
                }
            }
        } else {
            obj
        };
        let ot = nonref(&obj.ty);
        let Ty::Class(c) = ot.t else {
            if !ot.is_unknown() {
                self.error(line, "PFX-11", "member access on a non-class type");
            }
            return Ok(Expr::unknown(line, "member of unknown"));
        };
        if !self.classes[c].complete && !self.in_template() {
            // TYN-7: no member access on an incomplete class.
            self.error(line, "TYN-7", "member access into an incomplete class");
            return Ok(Expr::unknown(line, "incomplete"));
        }
        // DER-3: a qualified name looks in the class it names, which is
        // the object's class or one of its bases.
        let mut obj = obj;
        let mut c = c;
        if let Some(Ok(s)) = scope {
            if let SK::Class(qc) = self.scopes[s].kind {
                if qc == c || self.is_base_of(qc, c) {
                    c = qc;
                    obj.qualified = true;
                }
            }
        }
        let Some(ids) = self.lookup_member(c, &name) else {
            if !self.class_uncertain(c) {
                self.error(line, "NAM-8", format!("no member named '{}'", name));
            }
            return Ok(Expr::unknown(line, "unknown member"));
        };
        if scope.is_none() && !self.in_template() && !self.class_uncertain(c) && self.member_lookup(c, &name).is_some_and(|r| r.1) {
            // DER-4: the member is found in two base subobjects.
            self.error(line, "DER-4", format!("member '{}' is found in more than one base of '{}'", name, self.classes[c].name));
            if self.at("(") {
                self.skip_balanced();
            }
            return Ok(Expr::unknown(line, "ambiguous member"));
        }
        let id = ids[0];
        if matches!(self.decls[id].k, DK::Field(_) | DK::Var(_)) && !self.in_template() && self.scopes[self.decls[id].scope].kind == SK::Class(c) && !self.accessible(id, false) {
            // ACC-2, ACC-4: a private or protected data member of the
            // object's own class, used outside its members and friends.
            let (rule, what) = if self.member_access.get(&id) == Some(&1) { ("ACC-4", "protected") } else { ("ACC-2", "private") };
            self.error(line, rule, format!("'{}' is a {} member of '{}'", name, what, self.classes[c].name));
        }
        match self.decls[id].k.clone() {
            DK::Field(f) => {
                // PRIM-12 / VC-1 / VC-3: member of an lvalue is an lvalue,
                // member of an rvalue is an xvalue.
                let mut t = nonref(&f.ty);
                if !f.ty.t.is_ref() {
                    if !f.mutable {
                        t.c |= ot.c;
                    }
                    t.v |= ot.v;
                }
                let cat = if f.ty.t.is_ref() || obj.cat == Cat::L { Cat::L } else { Cat::X };
                Ok(Expr::new(EK::Member(Box::new(obj), id), t, cat, line))
            }
            DK::Var(v) => {
                let t = nonref(&v.ty);
                Ok(Expr::new(EK::Comma(Box::new(Expr::new(EK::Conv(Conv::ToVoid, Box::new(obj)), QTy::new(Ty::Void), Cat::PR, line)), Box::new(Expr::new(EK::Var(id), t.clone(), Cat::L, line))), t, Cat::L, line))
            }
            DK::Func(_) => {
                let fids: Vec<DeclId> = ids.iter().copied().filter(|&i| self.func(i).is_some()).collect();
                Ok(Expr::new(EK::BoundMember(Box::new(obj), fids), QTy::unknown(), Cat::PR, line))
            }
            DK::Enumerator(e, v) => Ok(Expr::new(EK::Int(v), QTy::new(Ty::Enum(e)), Cat::PR, line)),
            _ => Ok(Expr::unknown(line, "member")),
        }
    }

    pub fn parse_declarator_name_pub(&mut self, d: &mut crate::decl::Declarator) -> PR<()> {
        // reuse the declarator-name parser for `operator...`
        let r = self.parse_declarator(false, false)?;
        *d = r;
        Ok(())
    }

    fn parse_primary(&mut self) -> PR<Expr> {
        let t = self.tok().clone();
        let line = t.line;
        match t.kind {
            TK::Int => {
                self.pos += 1;
                return Ok(self.int_literal(&t.lit, line));
            }
            TK::Float => {
                self.pos += 1;
                if let Some(Lit::Float { value, suffix }) = t.lit {
                    let ty = match suffix {
                        FSuffix::None => Ty::Double,
                        FSuffix::F => Ty::Float,
                        FSuffix::L => Ty::LDouble,
                        FSuffix::Ext => Ty::ExtFloat,
                    };
                    if value.is_infinite() || (ty == Ty::Float && (value as f32).is_infinite()) {
                        // PRIM-15: out of range, conditionally supported; the
                        // matrix compilers accept with a warning.
                        self.warn(line, "PRIM-13", "floating literal out of range");
                    }
                    return Ok(Expr::new(EK::Float(value), QTy::new(ty), Cat::PR, line));
                }
                return Ok(Expr::unknown(line, "user-defined literal"));
            }
            TK::Char => {
                self.pos += 1;
                if let Some(Lit::Char { prefix, value, multi }) = t.lit {
                    let ty = if multi {
                        self.warn(line, "PRIM-13", "multi-character literal");
                        Ty::Int
                    } else {
                        match prefix {
                            Prefix::None => Ty::Char,
                            Prefix::U8 => Ty::Char8,
                            Prefix::U16 => Ty::Char16,
                            Prefix::U32 => Ty::Char32,
                            Prefix::Wide => Ty::WChar,
                        }
                    };
                    let v = wrap_int(value, &ty);
                    return Ok(Expr::new(EK::Int(v), QTy::new(ty), Cat::PR, line));
                }
                return Ok(Expr::unknown(line, "user-defined literal"));
            }
            TK::Str => return self.string_literal(),
            TK::Punct => {
                if t.text == "(" {
                    self.pos += 1;
                    let save = self.no_gt;
                    self.no_gt = 0;
                    if self.at("...") || self.is_fold() {
                        if !self.in_template() && self.inst_depth > 0 {
                            let r = self.parse_fold(line);
                            self.no_gt = save;
                            return r;
                        }
                        self.no_gt = save;
                        self.pos -= 1;
                        self.skip_balanced();
                        return Ok(Expr::unknown(line, "fold expression"));
                    }
                    let e = self.parse_expression();
                    self.no_gt = save;
                    let mut e = e?;
                    if self.at("...") {
                        self.pos += 1;
                        self.skip_to_close_paren();
                        return Ok(Expr::unknown(line, "fold expression"));
                    }
                    self.expect(")")?;
                    // PRIM-14: parentheses change nothing else.
                    e.paren = true;
                    return Ok(e);
                }
                if t.text == "[" {
                    return self.parse_lambda();
                }
                if t.text == "::" || t.text == "~" {
                    return self.parse_id_expression();
                }
                return Err(self.err(format!("expected an expression, found '{}'", t.text)));
            }
            TK::Ident => {}
            TK::Eof => return Err(self.err("unexpected end of file")),
        }
        let s = t.text.as_str();
        match s {
            "true" | "false" => {
                self.pos += 1;
                return Ok(Expr::new(EK::Int((s == "true") as i128), QTy::bool_(), Cat::PR, line));
            }
            "nullptr" => {
                self.pos += 1;
                return Ok(Expr::new(EK::NullPtr, QTy::new(Ty::NullPtr), Cat::PR, line));
            }
            "this" => {
                self.pos += 1;
                return Ok(self.build_this(line));
            }
            "static_cast" | "const_cast" | "reinterpret_cast" | "dynamic_cast" => {
                self.pos += 1;
                self.expect("<")?;
                self.no_gt += 1;
                let ty = self.parse_type_id();
                self.no_gt -= 1;
                let ty = ty?;
                self.expect(">")?;
                self.expect("(")?;
                let e = self.parse_expression()?;
                self.expect(")")?;
                let k = match s {
                    "static_cast" => CastKind::Static,
                    "const_cast" => CastKind::Const,
                    "reinterpret_cast" => CastKind::Reinterpret,
                    _ => CastKind::Dynamic,
                };
                return Ok(self.build_cast(k, ty, e, line));
            }
            "typeid" => {
                self.pos += 1;
                self.skip_balanced();
                return Ok(Expr::new(EK::Unknown("typeid".into()), QTy::unknown(), Cat::L, line));
            }
            "requires" => {
                self.pos += 1;
                return self.parse_requires_expr(line);
            }
            "__func__" => {
                self.pos += 1;
                if self.fns.is_empty() {
                    // FN-5: __func__ only in a function body.
                    self.error(line, "FN-5", "__func__ outside a function body");
                }
                let name: Vec<u32> = self.fns.last().and_then(|f| f.func).map(|f| self.decls[f].name.chars().map(|c| c as u32).collect()).unwrap_or_default();
                let n = name.len() as u64 + 1;
                let ty = QTy { t: Ty::Array(Box::new(QTy { t: Ty::Char, c: true, v: false }), Some(n)), c: true, v: false };
                return Ok(Expr::new(EK::Str(Rc::new(name)), ty, Cat::L, line));
            }
            "co_await" | "co_yield" | "co_return" => return Err(self.err("coroutines (outside the specification)")),
            "operator" => {
                // an operator function named in an expression (OPR)
                let mut d = crate::decl::Declarator {
                    name: None,
                    kind: crate::decl::NameKind::Plain,
                    qual: None,
                    qual_unknown: false,
                    ops: vec![],
                    line,
                    conv_ty: None,
                    template_args: false,
                    pack: false,
                };
                self.parse_declarator_name(&mut d)?;
                let name = d.name.unwrap_or_default();
                // The global allocation and deallocation functions exist in
                // every program, next to any that the program declares, so a
                // call of one is not checked against the declared ones alone.
                if matches!(name.as_str(), "operatornew" | "operatornew[]" | "operatordelete" | "operatordelete[]") {
                    return Ok(Expr::new(EK::Unknown(format!("allocation function {}", name)), QTy::unknown(), Cat::L, line));
                }
                let found = self.lookup(&name);
                return Ok(match found {
                    Some(ids) => self.name_expr(ids, line),
                    None => Expr::unknown(line, "operator function name"),
                });
            }
            _ => {}
        }
        // functional cast with a simple type keyword
        if crate::decl::simple_type_kw(s) || s == "typename" || s == "decltype" || s == "auto" {
            if s == "decltype" && !self.at_k(2, "auto") {
                // decltype(e)::member or decltype(e)(args)
            }
            let ty = if s == "auto" {
                self.pos += 1;
                QTy::unknown()
            } else {
                let ds = self.parse_decl_specs(None)?;
                self.ds_base(&ds)?
            };
            return self.parse_functional_cast(ty, line);
        }
        if is_keyword(s) {
            return Err(self.err(format!("unexpected keyword '{}' in an expression", s)));
        }
        if s == "__STDCPP_DEFAULT_NEW_ALIGNMENT__" {
            // NEW-27: the default new alignment of the matrix targets.
            self.pos += 1;
            return Ok(Expr::new(EK::Int(16), QTy::new(Ty::size_t()), Cat::PR, line));
        }
        if let Some((_, n)) = self.concept_at() {
            let mut k = n;
            if self.at_k(k, "<") {
                k = self.skip_angle_at(k);
            }
            if self.at_k(k, "auto") || (self.at_k(k, "decltype") && self.at_k(k + 2, "auto")) {
                // CPT-37: no constrained placeholder in a functional cast.
                self.error(line, "CPT-37", "a constrained placeholder in a functional cast");
                self.pos += k;
                return self.parse_primary();
            }
        }
        self.parse_id_expression()
    }

    pub fn is_fold(&self) -> bool {
        // ( e op ... ) or ( ... op e ): look for `...` before the matching )
        let mut k = 0;
        let mut depth = 0;
        loop {
            let t = self.peek(k);
            if t.kind == TK::Eof {
                return false;
            }
            if t.kind == TK::Punct {
                match t.text.as_str() {
                    "(" | "[" | "{" => depth += 1,
                    ")" | "]" | "}" => {
                        if depth == 0 {
                            return false;
                        }
                        depth -= 1;
                    }
                    "..." if depth == 0 => {
                        // a pack expansion in a call is inside nested parens
                        let prev = &self.peek(k.wrapping_sub(1)).text;
                        let next = &self.peek(k + 1).text;
                        return k == 0 || (prec(prev).is_some() || is_assign_op(prev) || prev == ",") || (prec(next).is_some() || is_assign_op(next));
                    }
                    ";" => return false,
                    _ => {}
                }
            }
            k += 1;
        }
    }

    fn skip_to_close_paren(&mut self) {
        let mut depth = 0;
        while self.tok().kind != TK::Eof {
            if self.at("(") {
                depth += 1;
            } else if self.at(")") {
                if depth == 0 {
                    self.pos += 1;
                    return;
                }
                depth -= 1;
            }
            self.pos += 1;
        }
    }

    fn parse_functional_cast(&mut self, ty: QTy, line: u32) -> PR<Expr> {
        if self.at("{") {
            let init = self.parse_braced_init()?;
            return Ok(self.build_functional(ty, init, true, line));
        }
        if self.at("(") {
            self.pos += 1;
            let args = self.parse_call_args()?;
            let init = Expr::new(EK::InitList(args), QTy::unknown(), Cat::PR, line);
            return Ok(self.build_functional(ty, init, false, line));
        }
        if self.at("::") {
            // T::member
            if let Ty::Class(c) = nonref(&ty).t {
                self.pos += 1;
                let name = self.ident()?;
                return Ok(match self.lookup_member(c, &name) {
                    Some(ids) => {
                        let mut e = self.name_expr(ids, line);
                        // A qualified member function name calls with no
                        // virtual dispatch (VIRT-1).
                        if let EK::BoundMember(obj, _) = &mut e.k {
                            obj.qualified = true;
                        }
                        e
                    }
                    None => {
                        self.error(line, "NAM-9", format!("no member named '{}'", name));
                        Expr::unknown(line, "member")
                    }
                });
            }
            self.pos += 1;
            self.skip_qualified_name()?;
            if self.at("(") {
                self.skip_balanced();
            }
            return Ok(Expr::unknown(line, "dependent member"));
        }
        Err(self.err("expected '(' or '{' after a type name in an expression"))
    }

    fn int_literal(&mut self, lit: &Option<Lit>, line: u32) -> Expr {
        let Some(Lit::Int { value, decimal, unsigned, size }) = lit.clone() else {
            return Expr::unknown(line, "user-defined literal");
        };
        // PRIM-1: the candidate list.
        let mut cands: Vec<Ty> = Vec::new();
        let signed_list: Vec<Ty> = match size {
            ISize::None => vec![Ty::Int, Ty::Long, Ty::LLong],
            ISize::L => vec![Ty::Long, Ty::LLong],
            ISize::LL => vec![Ty::LLong],
            ISize::Z => {
                let t = if unsigned { Ty::size_t() } else { Ty::ptrdiff_t() };
                if !fits(value as i128, &t) {
                    self.error(line, "PRIM-2", "integer literal does not fit its type");
                }
                return Expr::new(EK::Int(value as i128), QTy::new(t), Cat::PR, line);
            }
        };
        for t in signed_list {
            if !unsigned {
                cands.push(t.clone());
            }
            if unsigned || !decimal {
                cands.push(t.to_unsigned());
            }
        }
        for t in &cands {
            if fits(value as i128, t) {
                return Expr::new(EK::Int(value as i128), QTy::new(t.clone()), Cat::PR, line);
            }
        }
        // PRIM-2: no type in the list holds the value.
        self.error(line, "PRIM-2", "integer literal too large for its candidate types");
        Expr::new(EK::Int(0), QTy::new(Ty::ULLong), Cat::PR, line)
    }

    fn string_literal(&mut self) -> PR<Expr> {
        let line = self.line();
        let mut prefix = Prefix::None;
        let mut units: Vec<u32> = Vec::new();
        let mut seen_prefix = false;
        while self.tok().kind == TK::Str {
            match self.tok().lit.clone() {
                Some(Lit::Str { prefix: p, units: u }) => {
                    if p != Prefix::None {
                        if seen_prefix && p != prefix {
                            // PRIM-19: adjacent prefixed literals must agree.
                            self.error(line, "PRIM-19", "adjacent string literals with different prefixes");
                        }
                        prefix = p;
                        seen_prefix = true;
                    }
                    units.extend(u);
                }
                _ => {
                    self.pos += 1;
                    return Ok(Expr::unknown(line, "user-defined string literal"));
                }
            }
            self.pos += 1;
        }
        let ct = match prefix {
            Prefix::None => Ty::Char,
            Prefix::U8 => Ty::Char8,
            Prefix::U16 => Ty::Char16,
            Prefix::U32 => Ty::Char32,
            Prefix::Wide => Ty::WChar,
        };
        // PRIM-7: an lvalue array of N const characters, N counting the null.
        let n = units.len() as u64 + 1;
        let ty = QTy { t: Ty::Array(Box::new(QTy { t: ct, c: true, v: false }), Some(n)), c: true, v: false };
        Ok(Expr::new(EK::Str(Rc::new(units)), ty, Cat::L, line))
    }

    pub fn parse_braced_init(&mut self) -> PR<Expr> {
        let line = self.line();
        self.expect("{")?;
        let save = self.no_gt;
        self.no_gt = 0;
        let mut items = Vec::new();
        let r = (|| -> PR<()> {
            while !self.at("}") {
                if self.at(".") && self.peek(1).kind == TK::Ident {
                    // designator (LIST-5): kept as an unknown item with the
                    // member name
                    let dname = self.peek(1).text.clone();
                    self.pos += 2;
                    self.eat("=");
                    let e = if self.at("{") { self.parse_braced_init()? } else { self.parse_assignment()? };
                    let mut u = Expr::unknown(line, "designated initializer");
                    u.ty = e.ty.clone();
                    items.push(Expr::new(EK::Unknown(format!("designator {}", dname)), QTy::unknown(), Cat::PR, line));
                    continue_after(self);
                    if !self.eat(",") {
                        break;
                    }
                    continue;
                }
                let e = if self.at("{") { self.parse_braced_init()? } else { self.parse_assignment()? };
                self.eat("...");
                items.push(e);
                if !self.eat(",") {
                    break;
                }
            }
            self.expect("}")?;
            Ok(())
        })();
        self.no_gt = save;
        r?;
        fn continue_after(_p: &mut P) {}
        Ok(Expr::new(EK::InitList(items), QTy::unknown(), Cat::PR, line))
    }

    fn parse_lambda(&mut self) -> PR<Expr> {
        let line = self.line();
        self.expect("[")?;
        // captures (specification/expressions/lambdas.md grammar)
        let in_fn = !self.fns.is_empty();
        let cap_scope = self.push_scope(SK::Params);
        let r = self.parse_captures(in_fn, line);
        let cap_default = match r {
            Err(e) => {
                self.pop_scope();
                return Err(e);
            }
            Ok(d) => d,
        };
        let (cap_default, cap_refs, cap_items) = cap_default;
        let mut is_mutable = false;
        let mut is_static = false;
        let mut is_consteval = false;
        let mut generic = false;
        if self.at("<") {
            self.skip_template_args()?;
            generic = true;
            if self.at("requires") {
                // CPT-16: a requires clause after the template parameters.
                self.pos += 1;
                self.skip_constraint()?;
            }
        }
        let mark = self.decls.len();
        let ppos = self.pos;
        let (scope, fdecl) = if self.at("(") {
            let f = self.parse_params()?;
            (f.scope, Some(f))
        } else {
            (self.new_scope(SK::Params, Some(self.cur_scope()), ""), None)
        };
        // The parameter list can carry `noexcept` and the trailing return
        // type. A `decltype` there can name a capture (Q-0092), which the
        // front end does not model, so such a lambda stays unknown.
        let has_decltype = self.toks[ppos..self.pos].iter().any(|t| t.text == "decltype");
        if has_decltype {
            generic = true;
        }
        let mut ret = if has_decltype { None } else { fdecl.as_ref().and_then(|f| f.trailing.clone()) };
        let mut lambda_noexcept = fdecl.as_ref().is_some_and(|f| f.noexcept);
        let mut lambda_noexcept_expr = false;
        loop {
            if self.at("mutable") || self.at("constexpr") || self.at("consteval") || self.at("static") {
                if self.at("mutable") {
                    is_mutable = true;
                }
                if self.at("static") {
                    // LAM-13: a static call operator, which a call
                    // chooses over the surrogate (OVL-25).
                    is_static = true;
                }
                if self.at("consteval") {
                    is_consteval = true;
                }
                self.pos += 1;
            } else if self.at("noexcept") {
                self.pos += 1;
                lambda_noexcept = true;
                if self.at("(") {
                    self.skip_balanced();
                    lambda_noexcept_expr = true;
                }
            } else if self.at("->") {
                self.pos += 1;
                ret = Some(self.parse_type_id()?);
            } else if self.at("requires") {
                self.pos += 1;
                self.skip_constraint()?;
            } else if self.at("[") && self.at_k(1, "[") {
                self.skip_attributes();
            } else {
                break;
            }
        }
        // body: a separate function (JMP-7, SW-3)
        let mut ctx = FnCtx::new(ret.clone());
        ctx.deduce = ret.is_none();
        ctx.is_lambda = true;
        ctx.lambda_mutable = is_mutable;
        ctx.lambda_copy_default = cap_default == Some(true);
        ctx.lambda_refs = cap_refs;
        ctx.scope_base = self.stack.len();
        self.fns.push(ctx);
        self.stack.push(scope);
        let save = self.no_gt;
        self.no_gt = 0;
        let r = self.parse_compound_in(scope);
        self.no_gt = save;
        self.stack.pop();
        let depth = self.fns.len();
        self.check_jumps(depth);
        let ctx = self.fns.pop().unwrap();
        self.pop_scope();
        let _ = cap_scope;
        let body = r?;
        for (l, ln) in &ctx.gotos {
            if !ctx.labels.contains_key(l) {
                self.error(*ln, "JMP-14", format!("goto names an undefined label '{}' (a lambda body is a separate function)", l));
            }
        }
        let what = if ctx.lambda_refs.is_empty() || !in_fn { "lambda" } else { "lambda with reference captures" };
        let refs = what != "lambda";
        // The closure type, for the interpreter and for calls. A generic
        // lambda, or one in a template, stays unknown.
        let generic = generic || fdecl.as_ref().is_some_and(|f| f.ptys.iter().any(|t| t.t.contains_unknown()) || f.variadic);
        if !generic && !self.in_template() && !self.fns.iter().any(|f| f.is_lambda && f.deduce && f.deduced.is_none() && false) {
            let rt = match (&ret, &ctx.deduced) {
                (Some(t), _) => Some(t.clone()),
                (None, Some(t)) => Some(t.clone()),
                (None, None) => Some(QTy::new(Ty::Void)),
            };
            if let Some(rt) = rt.filter(|t| !t.t.contains_unknown()) {
                let noexcept = if lambda_noexcept_expr { None } else { Some(lambda_noexcept) };
                if let Some(x) = noexcept.and_then(|ne| self.build_closure(line, mark, cap_default, &cap_items, fdecl, rt, body, is_mutable, is_static, is_consteval, refs, in_fn, ne)) {
                    return Ok(x);
                }
            }
        }
        Ok(Expr::new(EK::Unknown(what.into()), QTy::unknown(), Cat::PR, line))
    }

    /// The closure class of a lambda (LAM rules): one member for each
    /// capture, in the order of the capture list and then of first use,
    /// and a function call operator with the lambda's body.
    #[allow(clippy::too_many_arguments)]
    fn build_closure(&mut self, line: u32, mark: usize, cap_default: Option<bool>, items: &[(u8, Option<DeclId>, bool, Option<Expr>)], fdecl: Option<crate::decl::FnDecl>, ret: QTy, body: Stmt, is_mutable: bool, is_static: bool, is_consteval: bool, refs: bool, in_fn: bool, noexcept: bool) -> Option<Expr> {
        // The variables and `this` that the body uses.
        let mut used: Vec<DeclId> = Vec::new();
        let mut this_ty: Option<QTy> = None;
        self.stmt_uses(&body, &mut used, &mut this_ty);
        let capturable = |p: &P, id: DeclId| -> bool {
            id < mark && p.var(id).is_some_and(|v| (v.local || v.param) && v.storage == Storage::Auto && v.class.is_none())
        };
        // (initializer, member type, captured variable, by reference)
        let mut members: Vec<(Expr, QTy, Option<DeclId>, bool)> = Vec::new();
        let mut this_member: Option<(usize, bool)> = None;
        let mut explicit: Vec<DeclId> = Vec::new();
        for (kind, id, by_ref, init) in items {
            match kind {
                0 => {
                    let id = (*id)?;
                    let v = self.var(id)?.clone();
                    if v.dependent || v.ty.is_unknown() {
                        return None;
                    }
                    explicit.push(id);
                    let t = nonref(&v.ty);
                    let e = Expr::new(EK::Var(id), t.clone(), Cat::L, line);
                    let mt = if *by_ref { QTy::new(Ty::LRef(Box::new(t.clone()))) } else { t.unqual() };
                    members.push((e, mt, Some(id), *by_ref));
                }
                1 => {
                    let id = (*id)?;
                    let init = init.clone()?;
                    let v = self.var(id)?.clone();
                    if v.ty.is_unknown() || init.ty.is_unknown() {
                        return None;
                    }
                    explicit.push(id);
                    members.push((init, v.ty.clone(), Some(id), *by_ref));
                }
                2 | 3 => {
                    let t = this_ty.clone().or_else(|| self.implicit_this(line, false).map(|x| x.ty))?;
                    let Ty::Ptr(ct) = &t.t else { return None };
                    let this_e = Expr::new(EK::This, t.clone(), Cat::PR, line);
                    if *kind == 3 {
                        let obj = Expr::new(EK::Unary(UOp::Deref, Box::new(this_e)), (**ct).clone(), Cat::L, line);
                        this_member = Some((members.len(), true));
                        members.push((obj, ct.unqual(), None, false));
                    } else {
                        this_member = Some((members.len(), false));
                        members.push((this_e, t.unqual(), None, false));
                    }
                }
                _ => return None,
            }
        }
        // Implicit captures (LAM rules): the variables that the body uses,
        // in the capture default's mode.
        for id in used {
            if !capturable(self, id) || explicit.contains(&id) || members.iter().any(|m| m.2 == Some(id)) {
                continue;
            }
            let by_ref = match cap_default {
                Some(copy) => !copy,
                None => {
                    // A constant read for its value needs no capture.
                    let v = self.var(id)?;
                    if v.ty.c && (v.ty.t.is_integral() || matches!(v.ty.t, Ty::Enum(_))) {
                        continue;
                    }
                    return None;
                }
            };
            let v = self.var(id)?.clone();
            if v.dependent || v.ty.is_unknown() {
                return None;
            }
            let t = nonref(&v.ty);
            let e = Expr::new(EK::Var(id), t.clone(), Cat::L, line);
            let mt = if by_ref { QTy::new(Ty::LRef(Box::new(t.clone()))) } else { t.unqual() };
            members.push((e, mt, Some(id), by_ref));
        }
        if this_member.is_none() {
            if let Some(t) = &this_ty {
                if cap_default.is_none() || !in_fn {
                    return None;
                }
                this_member = Some((members.len(), false));
                members.push((Expr::new(EK::This, t.clone(), Cat::PR, line), t.unqual(), None, false));
            }
        }
        // The class.
        let parent = self.cur_scope();
        let c = self.classes.len();
        let cscope = self.new_scope(SK::Class(c), Some(parent), "(lambda)");
        self.classes.push(ClassInfo {
            name: "(lambda)".into(),
            key: 1,
            scope: cscope,
            complete: true,
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
        let mut caps = Vec::new();
        let mut inits = Vec::new();
        for (i, (e, t, var, by_ref)) in members.into_iter().enumerate() {
            let fi = FieldInfo { ty: t, class: c, index: i, bits: None, mutable: false, init: None, align: 0, nua: false };
            let fd = self.add_decl_in(cscope, "", DK::Field(fi), line);
            self.classes[c].fields.push(fd);
            if let Some(v) = var {
                caps.push((v, i, by_ref));
            }
            inits.push(e);
        }
        let (params, ptys, defaults, variadic) = match &fdecl {
            Some(f) => (f.params.clone(), f.ptys.clone(), f.defaults, f.variadic),
            None => (vec![], vec![], 0, false),
        };
        let fty = FnTy { ret, params: ptys, variadic, noexcept, cv_c: !is_mutable && !is_static, cv_v: false, refq: 0 };
        let ptr_fty = FnTy { cv_c: false, ..fty.clone() };
        let captureless = caps.is_empty() && this_member.is_none() && self.classes[c].fields.is_empty();
        let fi = FuncInfo {
            fty,
            params: params.clone(),
            defaults,
            body: Some(Rc::new(body)),
            defined: true,
            // LAM rules: a lambda's call operator is constexpr when it can be.
            constexpr: true,
            consteval: is_consteval,
            deleted: false,
            defaulted: false,
            class: Some(c),
            kind: FnKind::Operator,
            is_static,
            is_virtual: false,
            template: false,
            explicit: false,
            opaque: false,
            is_main: false,
            explicit_obj: false,
            default_exprs: params.iter().map(|&p| self.var(p).and_then(|v| v.init.clone())).collect(),
            ret_auto: None,
            mem_inits: Vec::new(),
            first_attrs: Vec::new(),
            first_defaulted: false,
            constrained: false,
            fn_try: None,
            parse_failed: false,
            noexcept_false: false,
        };
        let conv_template = fi.clone();
        let op = self.add_decl_in(cscope, "operator()", DK::Func(fi), line);
        if captureless {
            // LAM-10: a lambda with no captures converts to a pointer to a
            // function with the parameters and result of its call operator.
            let target = QTy::new(Ty::Ptr(Box::new(QTy::new(Ty::Func(Box::new(ptr_fty))))));
            let cf = FuncInfo {
                fty: FnTy { ret: target.clone(), params: vec![], variadic: false, noexcept: true, cv_c: true, cv_v: false, refq: 0 },
                params: vec![],
                defaults: 0,
                body: None,
                defaulted: true,
                kind: FnKind::Conversion,
                consteval: false,
                default_exprs: vec![],
                ..conv_template
            };
            let conv = self.add_decl_in(cscope, "operator (lambda function pointer)", DK::Func(cf), line);
            self.classes[c].has_conv = true;
            self.classes[c].conv_targets.push(target);
            self.closure_conv.insert(conv, op);
        }
        self.closure_caps.insert(op, caps);
        if let Some(t) = this_member {
            self.closure_this.insert(op, t);
        }
        Some(Expr::new(EK::Closure(c, inits, refs && in_fn), QTy::new(Ty::Class(c)), Cat::PR, line))
    }

    /// The variables that a statement names, and the type of `this` if it
    /// uses `this`.
    fn stmt_uses(&self, s: &Stmt, out: &mut Vec<DeclId>, this_ty: &mut Option<QTy>) {
        let ex = |e: &Expr, out: &mut Vec<DeclId>, this_ty: &mut Option<QTy>| {
            walk_expr(e, &mut |x| match &x.k {
                EK::Var(id) => {
                    if !out.contains(id) {
                        out.push(*id);
                    }
                }
                EK::This => {
                    if this_ty.is_none() {
                        *this_ty = Some(x.ty.clone());
                    }
                }
                _ => {}
            });
        };
        match s {
            Stmt::Expr(e) | Stmt::Assume(e) => ex(e, out, this_ty),
            Stmt::Return(Some(e)) => ex(e, out, this_ty),
            Stmt::Decl(ids) => {
                for &id in ids {
                    if let Some(i) = self.var(id).and_then(|v| v.init.clone()) {
                        ex(&i, out, this_ty);
                    }
                }
            }
            Stmt::Block(v) => {
                for x in v {
                    self.stmt_uses(x, out, this_ty);
                }
            }
            Stmt::If { init, cond, then, els, .. } => {
                if let Some(i) = init {
                    self.stmt_uses(i, out, this_ty);
                }
                match cond {
                    Some(CondK::Expr(e)) => ex(e, out, this_ty),
                    Some(CondK::Decl(id, e)) => {
                        self.stmt_uses(&Stmt::Decl(vec![*id]), out, this_ty);
                        ex(e, out, this_ty);
                    }
                    None => {}
                }
                self.stmt_uses(then, out, this_ty);
                if let Some(e) = els {
                    self.stmt_uses(e, out, this_ty);
                }
            }
            Stmt::While { cond, body } | Stmt::Switch { cond, body, .. } => {
                if let Stmt::Switch { init: Some(i), .. } = s {
                    self.stmt_uses(i, out, this_ty);
                }
                match cond {
                    CondK::Expr(e) => ex(e, out, this_ty),
                    CondK::Decl(id, e) => {
                        self.stmt_uses(&Stmt::Decl(vec![*id]), out, this_ty);
                        ex(e, out, this_ty);
                    }
                }
                self.stmt_uses(body, out, this_ty);
            }
            Stmt::Do { body, cond } => {
                self.stmt_uses(body, out, this_ty);
                ex(cond, out, this_ty);
            }
            Stmt::For { init, cond, inc, body } => {
                if let Some(i) = init {
                    self.stmt_uses(i, out, this_ty);
                }
                if let Some(CondK::Expr(e)) | Some(CondK::Decl(_, e)) = cond {
                    ex(e, out, this_ty);
                }
                if let Some(i) = inc {
                    ex(i, out, this_ty);
                }
                self.stmt_uses(body, out, this_ty);
            }
            Stmt::Case(_, x) | Stmt::Default(x) | Stmt::Label(_, x) => self.stmt_uses(x, out, this_ty),
            Stmt::Try(b, hs) => {
                self.stmt_uses(b, out, this_ty);
                for h in hs {
                    self.stmt_uses(&h.body, out, this_ty);
                }
            }
            _ => {}
        }
    }

    #[allow(clippy::type_complexity)]
    fn parse_captures(&mut self, in_fn: bool, line: u32) -> PR<(Option<bool>, Vec<String>, Vec<(u8, Option<DeclId>, bool, Option<Expr>)>)> {
        let mut default: Option<bool> = None; // Some(true) for [=], Some(false) for [&]
        let mut refs: Vec<String> = Vec::new();
        // (0 variable, 1 init capture, 2 this, 3 *this; the variable; by
        // reference; the initializer)
        let mut items: Vec<(u8, Option<DeclId>, bool, Option<Expr>)> = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        let mut first = true;
        while !self.at("]") {
            let cl = self.line();
            if first && (self.at("=") || self.at("&")) && (self.at_k(1, ",") || self.at_k(1, "]")) {
                default = Some(self.at("="));
                self.pos += 1;
                if !in_fn && self.class_depth == 0 && self.dmi_depth == 0 {
                    // LAM-6: a capture default outside a function (Q-0037).
                    self.error(line, "LAM-6", "capture default in a lambda outside a function");
                }
            } else if self.at("this") || (self.at("*") && self.at_k(1, "this")) {
                let star = self.at("*");
                if self.at("*") {
                    self.pos += 1;
                }
                self.pos += 1;
                items.push((if star { 3 } else { 2 }, None, false, None));
                if seen.iter().any(|s| s == "this") {
                    self.error(cl, "LAM-6", "'this' captured twice");
                }
                seen.push("this".into());
            } else {
                let by_ref = self.eat("&");
                let pack = self.eat("...");
                let name = self.ident()?;
                self.eat("...");
                let _ = pack;
                if seen.contains(&name) {
                    self.error(cl, "LAM-6", format!("'{}' captured twice", name));
                }
                seen.push(name.clone());
                if by_ref {
                    refs.push(name.clone());
                }
                let has_init = self.at("=") || self.at("{") || self.at("(");
                if has_init {
                    // LAM-7: an init capture declares a new variable.
                    let init = if self.eat("=") {
                        if self.at("{") { self.parse_braced_init()? } else { self.parse_assignment()? }
                    } else if self.at("{") {
                        self.parse_braced_init()?
                    } else {
                        self.pos += 1;
                        let e = self.parse_assignment()?;
                        self.expect(")")?;
                        e
                    };
                    let it = match &init.k {
                        EK::InitList(v) if v.len() == 1 => v[0].clone(),
                        _ => init,
                    };
                    let et = nonref(&it.ty);
                    let ty = if by_ref {
                        QTy::new(Ty::LRef(Box::new(et)))
                    } else {
                        match &et.t {
                            Ty::Array(e, _) => QTy::new(Ty::Ptr(e.clone())),
                            _ => et.unqual(),
                        }
                    };
                    let conv = if by_ref {
                        it.clone()
                    } else {
                        let mark = self.d.list.len();
                        let x = self.convert_init(it.clone(), &ty, crate::decl2::InitForm::Copy, cl);
                        self.d.list.truncate(mark);
                        x
                    };
                    let vi = VarInfo { ty, storage: Storage::Auto, constexpr: false, constinit: false, init: None, param: false, local: true, dependent: true, defined: true, class: None };
                    let s = self.cur_scope();
                    let id = self.add_decl_in(s, &name, DK::Var(vi), cl);
                    items.push((1, Some(id), by_ref, if matches!(conv.k, EK::Unknown(_)) { None } else { Some(conv) }));
                } else {
                    if default == Some(!by_ref) {
                        // LAM-6: an explicit capture must not repeat the default mode.
                        self.error(cl, "LAM-6", format!("explicit capture of '{}' uses the default capture mode", name));
                    }
                    match self.lookup(&name) {
                        Some(ids) => {
                            items.push((0, Some(ids[0]), by_ref, None));
                            if let Some(v) = self.var(ids[0]) {
                                if !(v.local && v.storage == Storage::Auto) && !v.dependent {
                                    // LAM-6: only automatic variables are captured.
                                    self.error(cl, "LAM-6", format!("'{}' does not have automatic storage and cannot be captured", name));
                                }
                            } else if matches!(self.decls[ids[0]].k, DK::Field(_)) {
                                self.error(cl, "LAM-17", format!("data member '{}' captured by name", name));
                            }
                        }
                        None => {
                            if !self.lookup_uncertain() {
                                self.error(cl, "NAM-8", format!("captured name '{}' is not declared", name));
                            }
                        }
                    }
                }
            }
            first = false;
            if !self.eat(",") {
                break;
            }
        }
        self.expect("]")?;
        Ok((default, refs, items))
    }

    fn parse_id_expression(&mut self) -> PR<Expr> {
        let line = self.line();
        let start = self.pos;
        let scope = self.parse_nested_name()?;
        if let Some(Err(())) = scope {
            // qualifier names something unknown or dependent
            if self.in_template() && self.qual_starts_with_tparam(start) {
                // DEP-19: the pattern reads the name as a value.
                self.note_fact(start, crate::templ::FACT_VALUE_QUAL);
            }
            if self.eat("template") {}
            if self.at("~") {
                self.pos += 1;
            }
            if self.eat("operator") {
                self.pos += 1;
            } else {
                self.ident()?;
            }
            if self.at("<") && self.in_template() {
                self.skip_template_args()?;
            }
            return Ok(Expr::unknown(line, "dependent name"));
        }
        if self.at("operator") {
            // operator function name
            self.pos = start;
            let d = self.parse_declarator(false, false);
            let name = match d {
                Ok(d) => d.name.unwrap_or_default(),
                Err(e) => return Err(e),
            };
            let class_scope = matches!(scope, Some(Ok(s)) if matches!(self.scopes[s].kind, SK::Class(_)));
            if !class_scope && matches!(name.as_str(), "operatornew" | "operatornew[]" | "operatordelete" | "operatordelete[]") {
                return Ok(Expr::new(EK::Unknown(format!("allocation function {}", name)), QTy::unknown(), Cat::L, line));
            }
            let found = match scope {
                Some(Ok(s)) => self.lookup_in_scope(s, &name, true),
                _ => self.lookup(&name),
            };
            return Ok(match found {
                Some(ids) => self.name_expr(ids, line),
                None => Expr::unknown(line, "operator function"),
            });
        }
        if self.at("~") {
            self.pos += 1;
            self.ident()?;
            if !self.at("(") && !self.in_template() {
                // DTOR-8: a destructor is called, and has no address.
                self.error(line, "DTOR-8", "a destructor must be called with '()'");
            }
            return Ok(Expr::unknown(line, "destructor name"));
        }
        let name = self.ident()?;
        if scope.is_none() && self.in_template() && self.inst_depth == 0 {
            if let Some(ids) = self.lookup(&name) {
                if ids.len() == 1 {
                    self.note_pack_use(ids[0], self.pos - 1, line);
                }
            }
        }
        if let Some(Ok(sc)) = scope {
            if let SK::Class(c) = self.scopes[sc].kind {
                if self.classes[c].name == name && start > 0 && self.toks.get(start - 1).is_some_and(|t| t.text == "&") && !self.in_template() {
                    // CTOR-1: a constructor has no address.
                    self.error(line, "CTOR-1", "the address of a constructor");
                }
            }
        }
        let (found, amb) = match scope {
            None => self.lookup_full(&name),
            Some(Ok(s)) => self.lookup_qualified(s, &name),
            Some(Err(())) => (None, false),
        };
        if scope.is_some() && self.has_fact(start, crate::templ::FACT_VALUE_QUAL) {
            if let Some(ids) = &found {
                if ids.iter().all(|&i| matches!(self.decls[i].k, DK::Type(_) | DK::Class(_) | DK::Enum(_) | DK::Template(TemplKind::Class(_)) | DK::Template(TemplKind::Alias))) {
                    // DEP-19: the template read the name as a value, and
                    // in this specialization it names a type.
                    self.error(line, "DEP-19", format!("'{}' names a type, but the template reads it as a value without 'typename'", name));
                    if self.at("(") || self.at("{") {
                        self.skip_balanced();
                    }
                    return Ok(Expr::unknown(line, "type read as a value"));
                }
            }
        }
        if amb && !self.in_template() {
            self.report_ambiguous(scope.and_then(|r| r.ok()), &name, line);
            if self.at("(") {
                self.skip_balanced();
            }
            return Ok(Expr::unknown(line, "ambiguous name"));
        }
        // NAM-31, NAM-32: argument-dependent lookup follows ordinary
        // lookup that finds functions declared at namespace scope.
        if scope.is_none() && self.at("(") {
            if let Some(ids) = &found {
                let ns_funcs = ids.iter().all(|&i| match &self.decls[i].k {
                    DK::Func(f) => f.class.is_none() && self.scopes[self.decls[i].scope].kind == SK::Namespace,
                    DK::Template(TemplKind::Func) => self.scopes[self.decls[i].scope].kind == SK::Namespace,
                    _ => false,
                });
                if ns_funcs {
                    self.adl_pos = Some(self.pos);
                }
            }
        }
        // TARG-23: after an unqualified name that finds only functions or
        // nothing, `<` opens a template argument list, and a call can find
        // a function template by argument-dependent lookup.
        let only_plain_fns = found.as_ref().is_some_and(|ids| ids.iter().all(|&i| self.func(i).is_some_and(|f| !f.template)));
        if scope.is_none() && self.at("<") && !self.in_template() && (found.is_none() || only_plain_fns) {
            let save = self.pos;
            let mark = self.d.list.len();
            self.d.muted += 1;
            let raw = self.parse_raw_targs();
            self.d.muted -= 1;
            if let Ok(raw) = raw {
                if self.at("(") {
                    self.d.list.truncate(mark);
                    self.pos += 1;
                    let args = self.parse_call_args()?;
                    let (adl, _) = self.adl_lookup(&name, &args);
                    let mut cands: Vec<DeclId> = found.clone().unwrap_or_default();
                    for d in adl {
                        if !cands.contains(&d) {
                            cands.push(d);
                        }
                    }
                    if cands.iter().any(|&c| self.func(c).is_some_and(|f| f.template)) {
                        let callee = Expr::new(EK::TemplId(None, cands, std::rc::Rc::new(raw)), QTy::unknown(), Cat::L, line);
                        return Ok(self.build_call(callee, args, line));
                    }
                    self.error(line, "NAM-8", format!("no function template named '{}' is found for the call", name));
                    return Ok(Expr::unknown(line, "undeclared"));
                }
            }
            self.pos = save;
            self.d.list.truncate(mark);
        }
        let Some(ids) = found else {
            if self.at("<") && self.in_template() {
                self.skip_template_args()?;
                return Ok(Expr::unknown(line, "dependent template"));
            }
            // An unqualified call can find a function by argument-dependent
            // lookup (NAM-5).
            if scope.is_none() && self.at("(") {
                let save = self.pos;
                self.pos += 1;
                let args = self.parse_call_args()?;
                if self.in_template() && !args.iter().any(|a| a.ty.is_unknown()) {
                    // DEP-3: a call whose arguments do not depend on a
                    // parameter is bound at the definition.
                    self.note_fact(save, crate::templ::FACT_INDEPENDENT_CALL);
                }
                let (mut found, certain) = self.adl_lookup(&name, &args);
                if self.has_fact(save, crate::templ::FACT_INDEPENDENT_CALL) {
                    // DEP-5: the declarations before the definition only.
                    let lim = self.lookup_limit;
                    found.retain(|&d| d < lim);
                }
                if !found.is_empty() {
                    let callee = Expr::new(EK::Overloads(found), QTy::unknown(), Cat::L, line);
                    return Ok(self.build_call(callee, args, line));
                }
                let _ = save;
                let in_class_body = self.class_depth > 0 && self.fns.is_empty();
                if self.in_template() && certain && !in_class_body && self.class_depth == 0 && self.fns.len() == 1 && self.fns[0].class.is_none()
                    && args.iter().all(|a| !a.ty.is_unknown() && !matches!(nonref(&a.ty).t, Ty::Class(_) | Ty::Enum(_)) && !matches!(a.k, EK::InitList(_)))
                    && !self.stack.iter().any(|&s| self.scopes[s].uncertain)
                {
                    // DEP-6, DEP-10: an independent call with arguments of
                    // fundamental types finds only what the definition
                    // sees.
                    self.error_because(line, "DEP-6", "NAM-8", format!("call of undeclared function '{}' in a template: no declaration before the definition", name));
                }
                if !self.lookup_uncertain() && !in_class_body && certain && !self.in_template() {
                    // NAM-8: no declaration, and argument-dependent lookup
                    // finds no function (NAM-28, NAM-30).
                    self.error_because(line, "NAM-8", "NAM-30", format!("call of undeclared function '{}': argument-dependent lookup finds no function", name));
                }
                return Ok(Expr::unknown(line, "unqualified call to an undeclared name"));
            }
            if let Some(Ok(s)) = scope {
                if let SK::Class(c) = self.scopes[s].kind {
                    if self.class_uncertain(c) {
                        return Ok(Expr::unknown(line, "member of uncertain class"));
                    }
                }
                if self.scopes[s].uncertain {
                    return Ok(Expr::unknown(line, "uncertain namespace"));
                }
                self.error(line, "NAM-9", format!("no member named '{}' in the named scope", name));
                return Ok(Expr::unknown(line, "undeclared"));
            }
            if self.lookup_uncertain() {
                return Ok(Expr::unknown(line, "name in uncertain scope"));
            }
            // NAM-1: a name must be declared before use.
            self.error(line, "NAM-8", format!("use of undeclared name '{}'", name));
            return Ok(Expr::unknown(line, "undeclared"));
        };
        if !self.in_template() {
            if scope.is_none() && self.private_base_name(&name) {
                // ACC-18: a member of a private base of another class.
                self.error(line, "ACC-18", format!("'{}' is a member of a private base of a base class and is not accessible here", name));
            }
            if let Some(Ok(s)) = scope {
                if let SK::Class(c) = self.scopes[s].kind {
                    let after_amp = start > 0 && self.toks.get(start - 1).is_some_and(|t| t.text == "&");
                    if self.classes[c].name == name && !self.classes[c].template && !after_amp {
                        // NAM-34: `C::C` names the constructors.
                        self.error(line, "NAM-34", format!("'{0}::{0}' names the constructors of '{0}', which cannot be called or used as a type", name));
                        if self.at("(") || self.at("{") {
                            self.skip_balanced();
                        }
                        return Ok(Expr::unknown(line, "constructor name"));
                    }
                }
            }
        }
        // template-id
        let tid = ids.iter().copied().find(|&i| matches!(self.decls[i].k, DK::Template(_)) || self.injected.contains_key(&i));
        if let Some(t) = tid {
            if self.at("<") {
                return self.parse_template_id_expr(t, line);
            }
            if matches!(self.decls[t].k, DK::Template(TemplKind::Class(_) | TemplKind::Alias)) && (self.at("(") || self.at("{")) && !self.in_template() && !self.tparam.contains_key(&t) {
                // CTAD-1: a functional cast with a deduced class type.
                if let Some(ctid) = self.tid_of(t) {
                    if let Some(q) = self.ctad_lookahead(ctid, line) {
                        return self.parse_functional_cast(q, line);
                    }
                    if self.at("(") || self.at("{") {
                        self.skip_balanced();
                    }
                    return Ok(Expr::unknown(line, "deduction of class template arguments"));
                }
            }
            if let DK::Template(TemplKind::Var { .. }) = self.decls[t].k {
                if !self.in_template() && self.tid_of(t).is_some() {
                    // TARG-10: a variable template needs an argument list.
                    self.error(line, "TARG-10", format!("the variable template '{}' is used without an argument list", name));
                    return Ok(Expr::unknown(line, "variable template"));
                }
            }
        }
        if ids.iter().any(|&i| self.func(i).is_some_and(|f| f.template)) && self.at("<") {
            if self.in_template() {
                self.skip_template_args()?;
                return Ok(Expr::unknown(line, "function template specialization"));
            }
            // DED-2: explicit template arguments for the call.
            let raw = self.parse_raw_targs()?;
            let fids: Vec<DeclId> = ids.iter().copied().filter(|&i| self.func(i).is_some()).collect();
            let e = self.name_expr(fids.clone(), line);
            let obj = match e.k {
                EK::BoundMember(o, _) => Some(o),
                _ => None,
            };
            return Ok(Expr::new(EK::TemplId(obj, fids, std::rc::Rc::new(raw)), QTy::unknown(), Cat::L, line));
        }
        // TYN-4: a function or variable hides a class of the same name.
        let ids: Vec<DeclId> = if ids.iter().any(|&i| matches!(self.decls[i].k, DK::Var(_) | DK::Func(_) | DK::Enumerator(..))) {
            ids.into_iter().filter(|&i| !matches!(self.decls[i].k, DK::Class(_) | DK::Enum(_))).collect()
        } else {
            ids
        };
        // a type name in an expression: functional cast
        let first = ids[0];
        let ty = match &self.decls[first].k {
            DK::Type(q) => Some(q.clone()),
            DK::Class(c) => Some(if self.classes[*c].template { QTy::unknown() } else { QTy::new(Ty::Class(*c)) }),
            DK::Enum(e) => Some(QTy::new(Ty::Enum(*e))),
            DK::TypeParam => Some(QTy::unknown()),
            _ => None,
        };
        if let Some(ty) = ty {
            // An injected class name in a member function (constructor
            // call) or a type used as a functional cast.
            return self.parse_functional_cast(ty, line);
        }
        let mut e = self.name_expr(ids, line);
        // A qualified name of a member function, as in `A::f()`, calls that
        // function with no virtual dispatch (VIRT-1).
        if let (Some(Ok(_)), EK::BoundMember(obj, _)) = (&scope, &mut e.k) {
            obj.qualified = true;
        }
        Ok(e)
    }

    /// An ambiguous lookup: NAM-14 for a qualified name in a namespace,
    /// NAM-18 for a qualified name in a class, DER-4 for a member found in
    /// two bases, NAM-24 or NAM-26 for using directives.
    pub fn report_ambiguous(&mut self, scope: Option<usize>, name: &str, line: u32) {
        match scope.map(|s| self.scopes[s].kind.clone()) {
            Some(SK::Namespace) => self.error(line, "NAM-14", format!("'{}' is ambiguous: the using directives of the namespace find different entities", name)),
            Some(SK::Class(_)) => self.error_because(line, "NAM-18", "DER-4", format!("'{}' is found in more than one base", name)),
            _ => {
                let in_class = self.stack.iter().rev().find_map(|&s| match self.scopes[s].kind {
                    SK::Class(c) => Some(c),
                    _ => None,
                });
                let in_bases = in_class.is_some_and(|c| self.member_lookup(c, name).is_some_and(|r| r.1));
                if in_bases {
                    self.error(line, "DER-4", format!("'{}' is found in more than one base", name));
                } else {
                    self.error_because(line, "NAM-26", "NAM-24", format!("'{}' is ambiguous: using directives make different entities visible", name));
                }
            }
        }
    }

    fn parse_template_id_expr(&mut self, t: DeclId, line: u32) -> PR<Expr> {
        let k = self.decls[t].k.clone();
        match k {
            DK::Template(TemplKind::Var { .. }) => {
                let Some(tid) = self.tid_of(t) else {
                    self.skip_template_args()?;
                    return Ok(Expr::unknown(line, "variable template"));
                };
                let raw = self.parse_raw_targs()?;
                if self.in_template() {
                    return Ok(Expr::new(EK::Unknown("variable template with unknown arguments".into()), QTy::unknown(), Cat::PR, line));
                }
                match self.targs_for(tid, raw, line, false) {
                    Ok(Some(args)) => match self.var_inst(tid, args, line) {
                        Some(d) => Ok(self.name_expr(vec![d], line)),
                        None => Ok(Expr::unknown(line, "variable template")),
                    },
                    _ => Ok(Expr::new(EK::Unknown("variable template with unknown arguments".into()), QTy::unknown(), Cat::PR, line)),
                }
            }
            DK::Template(TemplKind::Concept) => {
                // CPT-1: a concept-id is a constant of type bool.
                let Some(tid) = self.tid_of(t) else {
                    self.skip_template_args()?;
                    return Ok(Expr::new(EK::Unknown("concept".into()), QTy::bool_(), Cat::PR, line));
                };
                let raw = self.parse_raw_targs()?;
                let args = if self.in_template() {
                    match self.pattern_targs(tid, raw) {
                        Some(a) if !a.iter().any(|x| x.is_dep()) => Some(a),
                        _ => None,
                    }
                } else {
                    match self.concept_targs(tid, raw, line) {
                        Ok(Some(a)) => Some(a),
                        Ok(None) => None,
                        Err(()) => return Ok(Expr::unknown(line, "bad concept-id")),
                    }
                };
                let Some(args) = args else {
                    return Ok(Expr::new(EK::Unknown("concept".into()), QTy::bool_(), Cat::PR, line));
                };
                match self.concept_value(tid, &args) {
                    crate::cons::Sat::Yes => Ok(Expr::new(EK::Int(1), QTy::bool_(), Cat::PR, line)),
                    crate::cons::Sat::No | crate::cons::Sat::Error => Ok(Expr::new(EK::Int(0), QTy::bool_(), Cat::PR, line)),
                    crate::cons::Sat::Unknown => Ok(Expr::new(EK::Unknown("concept".into()), QTy::bool_(), Cat::PR, line)),
                }
            }
            DK::Template(TemplKind::Class(_)) | DK::Template(TemplKind::Alias) | DK::Class(_) => {
                let q = self.parse_template_id_type(t, line)?;
                if self.at("::") {
                    if let Ty::Class(_) = q.t {
                        return self.parse_functional_cast(q, line);
                    }
                }
                self.parse_functional_cast(q, line)
            }
            _ => {
                self.skip_template_args()?;
                Ok(Expr::unknown(line, "template"))
            }
        }
    }

    /// Build the expression for a name that lookup found (PRIM-12).
    pub fn name_expr(&mut self, ids: Vec<DeclId>, line: u32) -> Expr {
        let mut ids = ids;
        if ids.len() == 1 {
            // A pack in a specialization: the element of the copy being
            // parsed.
            if let Some(list) = self.fn_param_packs.get(&ids[0]).cloned() {
                self.pack_refs.push(list.len());
                match list.get(self.pack_index.unwrap_or(0)) {
                    Some(&d) => ids = vec![d],
                    None => return Expr::unknown(line, "empty pack"),
                }
            } else if let Some(v) = self.pack_binds.get(&ids[0]).cloned() {
                if !matches!(self.decls[ids[0]].k, DK::Type(_)) {
                    self.pack_refs.push(v.len());
                    return match v.get(self.pack_index.unwrap_or(0)) {
                        Some(TArg::Val(TVal::Const(t, cv))) => self.tval_expr(t, cv, line),
                        _ => Expr::unknown(line, "empty pack"),
                    };
                }
            }
        }
        let id = ids[0];
        if ids.len() == 1 && self.func(id).is_some_and(|f| f.deleted) && !self.in_template() {
            // DEL-1: no reference to a deleted function.
            self.error(line, "DEL-1", format!("use of the deleted function '{}'", self.decls[id].name));
        }
        if ids.len() == 1 && !self.in_template() {
            if let Some(f) = self.func(id) {
                // RET-3: no use before the return type is known.
                if f.ret_auto.is_some() && f.fty.ret.is_unknown() && !f.template && f.body.is_none() {
                    let own = self.fns.iter().find(|c| c.func == Some(id));
                    let known = match own {
                        Some(c) => c.deduced.is_some(),
                        None => false,
                    };
                    if !known {
                        self.error(line, "RET-3", format!("'{}' is used before its return type is deduced", self.decls[id].name));
                    }
                }
            }
        }
        if ids.len() == 1 && self.has_attr(id, "deprecated") {
            // ATTR-4: warn at each use of a deprecated entity.
            self.warn(line, "ATTR-4", format!("'{}' is deprecated", self.decls[id].name));
        }
        if Some(id) == self.auto_in_own_init {
            // AUTO-8: an auto variable must not occur in its own initializer.
            self.error(line, "AUTO-8", "variable declared with 'auto' used in its own initializer");
        }
        if matches!(self.decls[id].k, DK::Var(_)) {
            self.used_vars.insert(id);
            *self.use_count.entry(id).or_default() += 1;
            if self.in_unevaluated == 0 && !self.in_template() {
                // INST-8: a use of the variable makes its definition.
                self.lazy_static_defs.remove(&id);
                self.used_statics.insert(id);
                if let Some(errs) = self.body_errors.remove(&id) {
                    self.d.list.extend(errs);
                }
            }
        }
        match self.decls[id].k.clone() {
            DK::Var(v) => {
                if let Some(e) = self.const_param_val.get(&id) {
                    // TPL-9: the name of a constant parameter is its value.
                    let mut e = e.clone();
                    e.line = line;
                    return e;
                }
                if v.dependent {
                    if v.param {
                        // TPL-9: a reference or class parameter is an lvalue.
                        let cat = if v.ty.t.is_ref() || matches!(v.ty.t, Ty::Class(_)) { Cat::L } else { Cat::PR };
                        return Expr::new(EK::Unknown(format!("template parameter#{}", id)), nonref(&v.ty), cat, line);
                    }
                    // SB-12: a structured binding name is an lvalue.
                    return Expr::new(EK::Var(id), nonref(&v.ty), Cat::L, line);
                }
                // LAM-18: a variable captured by copy in a lambda that is not
                // mutable is const inside it.
                let mut t = nonref(&v.ty);
                if let Some(f) = self.fns.last() {
                    if f.is_lambda && !f.lambda_mutable && f.lambda_copy_default && v.local && v.storage == Storage::Auto && !v.ty.t.is_ref() && !f.lambda_refs.contains(&self.decls[id].name) {
                        let declared_outside = !self.stack[f.scope_base.saturating_sub(1).min(self.stack.len())..].contains(&self.decls[id].scope);
                        if declared_outside {
                            t.c = true;
                        }
                    }
                }
                let cat = if let Ty::RRef(i) = &v.ty.t { if matches!(i.t, Ty::Func(_)) { Cat::L } else { Cat::L } } else { Cat::L };
                Expr::new(EK::Var(id), t, cat, line)
            }
            DK::Enumerator(e, val) => {
                // Inside the enumerator list the enumeration is incomplete;
                // an enumerator then has the underlying type (fixed) or an
                // integer type that holds its value (ENUM-10, ENUM-13).
                let t = if self.enums[e].complete {
                    QTy::new(Ty::Enum(e))
                } else if let Some(t) = self.enumerator_ty.get(&id) {
                    QTy::new(t.clone())
                } else if let Some(f) = &self.enums[e].fixed {
                    QTy::new(f.clone())
                } else {
                    QTy::new([Ty::Int, Ty::UInt, Ty::Long, Ty::ULong, Ty::LLong, Ty::ULLong].into_iter().find(|t| fits(val, t)).unwrap_or(Ty::LLong))
                };
                Expr::new(EK::Int(val), t, Cat::PR, line)
            }
            DK::Func(_) => {
                let fids: Vec<DeclId> = ids.into_iter().filter(|&i| self.func(i).is_some()).collect();
                // implicit member access for non-static member functions
                let nonstatic = fids.iter().any(|&i| self.func(i).is_some_and(|f| f.class.is_some() && !f.is_static && f.kind != FnKind::Ctor));
                if nonstatic {
                    if let Some(this) = self.implicit_this(line, true) {
                        // OVL-2: `*this` is the implied object argument only
                        // if its class is the members' class or derived
                        // from it.
                        let tc = match &this.ty.t {
                            Ty::Ptr(p) => match p.t {
                                Ty::Class(c) => Some(c),
                                _ => None,
                            },
                            _ => None,
                        };
                        let related = tc.is_none_or(|tc| fids.iter().any(|&i| self.func(i).and_then(|f| f.class).is_none_or(|fc| fc == tc || self.is_base_of(fc, tc))));
                        if related {
                            return Expr::new(EK::BoundMember(Box::new(this), fids), QTy::unknown(), Cat::PR, line);
                        }
                    }
                }
                if fids.len() == 1 {
                    if self.in_unevaluated == 0 && !self.in_template() {
                        // INST-7: a use of the function.
                        self.use_fn(fids[0]);
                    }
                    let f = self.func(fids[0]).unwrap();
                    let t = QTy::new(Ty::Func(Box::new(f.fty.clone())));
                    return Expr::new(EK::Func(fids[0]), t, Cat::L, line);
                }
                Expr::new(EK::Overloads(fids), QTy::unknown(), Cat::L, line)
            }
            DK::Field(f) => {
                // PRIM-12: a member name in a member function means this->m.
                match self.implicit_this(line, false) {
                    Some(this) => {
                        let obj_t = match &this.ty.t {
                            Ty::Ptr(p) => (**p).clone(),
                            _ => QTy::unknown(),
                        };
                        let obj = Expr::new(EK::Unary(UOp::Deref, Box::new(this)), obj_t.clone(), Cat::L, line);
                        let mut t = nonref(&f.ty);
                        if !f.ty.t.is_ref() && !f.mutable {
                            t.c |= obj_t.c;
                        }
                        Expr::new(EK::Member(Box::new(obj), id), t, Cat::L, line)
                    }
                    None => {
                        if self.in_unevaluated == 0 && !self.in_template() && self.fns.iter().all(|f| !f.is_lambda) {
                            // PRIM-12: elsewhere only in an unevaluated operand.
                            self.error(line, "PRIM-25", "non-static member used without an object");
                        }
                        Expr::new(EK::Member(Box::new(Expr::unknown(line, "member without object")), id), nonref(&f.ty), Cat::L, line)
                    }
                }
            }
            DK::Namespace(_) => {
                self.error(line, "NAM-8", "namespace name used as a value");
                Expr::unknown(line, "namespace")
            }
            _ => Expr::unknown(line, "name"),
        }
    }

    /// `this` for implicit member access, or None if there is no object.
    fn implicit_this(&mut self, line: u32, for_call: bool) -> Option<Expr> {
        let f = self.fns.iter().rev().find(|f| !f.is_lambda || f.class.is_some())?;
        let c = f.class?;
        if f.is_static {
            if !for_call && self.in_unevaluated == 0 {
                return None;
            }
            return None;
        }
        let _ = line;
        let mut ct = QTy::new(Ty::Class(c));
        ct.c = f.this_const;
        ct.v = f.this_volatile;
        Some(Expr::new(EK::This, QTy::new(Ty::Ptr(Box::new(ct))), Cat::PR, line))
    }

    fn build_this(&mut self, line: u32) -> Expr {
        let f = self.fns.iter().rev().find(|f| !f.is_lambda);
        match f.and_then(|f| f.class.map(|c| (c, f.is_static, f.this_const, f.this_volatile))) {
            Some((_, false, _, _)) if f.and_then(|f| f.func).and_then(|id| self.func(id)).is_some_and(|fi| fi.explicit_obj) => {
                // PRIM-20: no `this` with an explicit object parameter.
                self.error(line, "PRIM-20", "'this' in a member function with an explicit object parameter");
                Expr::unknown(line, "this")
            }
            Some((c, false, cst, vol)) => {
                let mut ct = QTy::new(Ty::Class(c));
                ct.c = cst;
                ct.v = vol;
                Expr::new(EK::This, QTy::new(Ty::Ptr(Box::new(ct))), Cat::PR, line)
            }
            Some((_, true, _, _)) => {
                // PRIM-20: no `this` in a static member function.
                self.error(line, "PRIM-20", "'this' in a static member function");
                Expr::unknown(line, "this")
            }
            None => {
                if self.class_depth > 0 && self.fns.is_empty() {
                    // default member initializer: allowed (PRIM-11)
                    return Expr::unknown(line, "this in a default member initializer");
                }
                if self.fns.iter().any(|f| f.is_lambda) && self.enclosing_class().is_some() {
                    return Expr::unknown(line, "this in a lambda");
                }
                if self.enclosing_class().is_some() {
                    return Expr::unknown(line, "this");
                }
                // PRIM-20: no `this` outside a class.
                self.error(line, "PRIM-20", "'this' outside a member function");
                Expr::unknown(line, "this")
            }
        }
    }

    // ---------------- conversions ----------------

    /// Turn a glvalue into a prvalue: array and function decay, then
    /// lvalue-to-rvalue (CONV-13).
    pub fn rvalue(&mut self, e: Expr) -> Expr {
        let line = e.line;
        match &e.ty.t {
            Ty::Array(el, _) => {
                if !e.is_glvalue() && !matches!(e.k, EK::Str(_)) {
                    let t = QTy::new(Ty::Ptr(el.clone()));
                    return Expr::new(EK::Conv(Conv::ArrayDecay, Box::new(e)), t, Cat::PR, line);
                }
                let t = QTy::new(Ty::Ptr(el.clone()));
                Expr::new(EK::Conv(Conv::ArrayDecay, Box::new(e)), t, Cat::PR, line)
            }
            Ty::Func(_) => {
                let t = QTy::new(Ty::Ptr(Box::new(e.ty.clone())));
                Expr::new(EK::Conv(Conv::FuncDecay, Box::new(e)), t, Cat::PR, line)
            }
            Ty::Class(_) => e,
            _ => {
                if e.is_glvalue() {
                    // VC-2: a prvalue of non-class type has no cv-qualifiers.
                    let t = e.ty.unqual();
                    Expr::new(EK::Conv(Conv::LToR, Box::new(e)), t, Cat::PR, line)
                } else if e.ty.c || e.ty.v {
                    let mut e = e;
                    e.ty = e.ty.unqual();
                    e
                } else {
                    e
                }
            }
        }
    }

    pub fn conv(&self, e: Expr, k: Conv, to: QTy) -> Expr {
        let line = e.line;
        if e.ty == to && k == Conv::Arith {
            return e;
        }
        Expr::new(EK::Conv(k, Box::new(e)), to, Cat::PR, line)
    }

    /// The type an operand has after integer promotion (CONV-1).
    pub fn promoted(&self, t: &Ty) -> Ty {
        match t {
            Ty::Enum(e) => {
                let u = self.enum_underlying(*e);
                // CONV-1: an unscoped enumeration promotes; with a fixed
                // underlying type, through that type.
                promote(&u)
            }
            t => promote(t),
        }
    }

    /// BF-6: a bit-field promotes by its width.
    pub fn bitfield_promotion(&self, e: &Expr) -> Option<Ty> {
        let inner = match &e.k {
            EK::Conv(Conv::LToR, a) => a,
            _ => e,
        };
        if let EK::Member(_, f) = &inner.k {
            if let DK::Field(fi) = &self.decls[*f].k {
                if let Some(w) = fi.bits {
                    let t = if let Ty::Enum(en) = &fi.ty.t { self.enum_underlying(*en) } else { fi.ty.t.clone() };
                    if !t.is_integral() {
                        return None;
                    }
                    let signed = t.is_signed();
                    let max_bits = if signed { w.saturating_sub(1) } else { w };
                    if max_bits < 31 || (signed && w <= 32) {
                        return Some(Ty::Int);
                    }
                    if !signed && w <= 32 {
                        return Some(Ty::UInt);
                    }
                    return Some(promote(&t));
                }
            }
        }
        None
    }

    pub fn promote_expr(&mut self, e: Expr) -> Expr {
        let e = self.rvalue(e);
        if let Some(bp) = self.bitfield_promotion(&e) {
            if bp != e.ty.t {
                return self.conv(e, Conv::Arith, QTy::new(bp));
            }
            return e;
        }
        let p = self.promoted(&e.ty.t);
        if p != e.ty.t {
            return self.conv(e, Conv::Arith, QTy::new(p));
        }
        e
    }

    pub fn is_null_ptr_constant(&self, e: &Expr) -> bool {
        // CONV-8: nullptr, or an integer literal with the value zero.
        match &e.k {
            EK::NullPtr => true,
            EK::Int(0) => matches!(e.ty.t, Ty::Int | Ty::UInt | Ty::Long | Ty::ULong | Ty::LLong | Ty::ULLong) && e.cat == Cat::PR && !matches!(e.ty.t, Ty::Bool),
            EK::Conv(Conv::LToR, _) => false,
            _ => e.ty.t == Ty::NullPtr,
        }
    }

    /// Integer-ness for operators: integral types and unscoped
    /// enumerations.
    pub fn is_int_like(&self, t: &Ty) -> bool {
        t.is_integral() || self.is_unscoped_enum(t)
    }
    pub fn is_arith_like(&self, t: &Ty) -> bool {
        t.is_arithmetic() || self.is_unscoped_enum(t)
    }

    /// Does a class have a usable conversion to a non-class type? Returns
    /// None when the front end cannot tell.
    pub fn class_converts(&self, c: usize) -> Option<bool> {
        if self.class_uncertain(c) {
            return None;
        }
        if self.classes[c].has_conv {
            return Some(true);
        }
        for b in &self.classes[c].bases {
            if let Some(bc) = b.class {
                if self.class_converts(bc) != Some(false) {
                    return Some(true);
                }
            }
        }
        Some(false)
    }

    /// Is any operator function with this name visible, as a member of
    /// the class or by ordinary lookup?
    pub fn operator_visible(&self, op: &str, classes: &[usize]) -> bool {
        // DCMP: `!=` can use `==`, and the relational operators can use `<=>`.
        let mut names = vec![format!("operator{}", op)];
        if op == "!=" {
            names.push("operator==".into());
        }
        if matches!(op, "<" | ">" | "<=" | ">=" | "==" | "!=") {
            // a defaulted <=> also declares ==
            names.push("operator<=>".into());
        }
        names.iter().any(|n| self.operator_visible1(n, classes))
    }

    fn operator_visible1(&self, name: &str, classes: &[usize]) -> bool {
        let name = name.to_string();
        for &c in classes {
            if self.lookup_member(c, &name).is_some() {
                return true;
            }
            // functions in the class's namespace (found by ADL)
            let mut s = self.scopes[self.classes[c].scope].parent;
            while let Some(x) = s {
                if self.scopes[x].names.get(&name).is_some_and(|v| !v.is_empty()) {
                    return true;
                }
                if self.scopes[x].kind == SK::Namespace {
                    break;
                }
                s = self.scopes[x].parent;
            }
            // friends declared in the class
            if self.class_has_friend_op(c, &name) {
                return true;
            }
        }
        self.lookup(&name).is_some()
    }

    pub fn class_has_friend_op(&self, c: usize, name: &str) -> bool {
        let _ = (c, name);
        // Friend functions are declared in the enclosing namespace scope.
        false
    }

    /// Implicit conversion of a prvalue or glvalue to a target type.
    /// Returns Err with a reason if no implicit conversion exists.
    pub fn implicit(&mut self, e: Expr, to: &QTy, direct: bool) -> Result<Expr, String> {
        let line = e.line;
        if !self.in_template() {
            // INST-1: a conversion between classes needs them complete; a
            // pointer conversion only when the classes differ.
            let cls = |q: &QTy| -> Option<(usize, bool)> {
                match &nonref(q).t {
                    Ty::Class(c) => Some((*c, false)),
                    Ty::Ptr(x) => match x.t {
                        Ty::Class(c) => Some((c, true)),
                        _ => None,
                    },
                    _ => None,
                }
            };
            match (cls(&e.ty), cls(to)) {
                (Some((a, pa)), Some((b, pb))) if a != b || pa != pb => {
                    self.require_class(a, line);
                    self.require_class(b, line);
                }
                (Some((a, false)), None) => self.require_class(a, line),
                (None, Some((b, false))) => self.require_class(b, line),
                _ => {}
            }
        }
        if let Some(x) = self.overload_target(&e, to, line) {
            return Ok(x);
        }
        if to.is_unknown() || e.ty.is_unknown() {
            let mut x = e;
            if !to.is_unknown() {
                x = Expr::new(EK::Conv(Conv::NoOp, Box::new(x)), to.clone(), Cat::PR, line);
            }
            return Ok(x);
        }
        if matches!(e.k, EK::Overloads(_) | EK::BoundMember(..)) {
            return Ok(Expr::new(EK::Unknown("overload set conversion".into()), to.clone(), Cat::PR, line));
        }
        let target = to.unqual();
        match &target.t {
            Ty::LRef(_) | Ty::RRef(_) => return Ok(self.bind_ref(e, to, line, "INIT-7")),
            Ty::Class(tc) => {
                let et = nonref(&e.ty);
                if let Ty::Class(sc) = et.t {
                    if sc == *tc || self.is_base_of(*tc, sc) {
                        return Ok(e);
                    }
                }
                // a constructor call; not modeled unless the class is a
                // plain aggregate without constructors
                let ci = &self.classes[*tc];
                if ci.complete && ci.ctors.is_empty() && !self.class_uncertain(*tc) && !matches!(et.t, Ty::Class(_)) {
                    return Err(format!("no conversion to class '{}'", ci.name));
                }
                let tc = *tc;
                if !direct && ci.complete && !self.class_uncertain(tc) && !self.in_template() && !matches!(et.t, Ty::Class(_)) {
                    let x = self.ctor_call(tc, std::slice::from_ref(&e), to, line);
                    if let EK::Construct(cid, _) = &x.k {
                        if self.func(*cid).is_some_and(|f| f.explicit) {
                            // CTOR-9: copy-initialization does not use an
                            // explicit constructor.
                            self.error(line, "CTOR-9", "copy-initialization uses an explicit constructor");
                        }
                        return Ok(x);
                    }
                    // CTOR-22: one user-defined conversion at most. A
                    // constructor whose parameter is a class needs a second
                    // one from a value that is not a class.
                    let one_arg: Vec<DeclId> = self.classes[tc].ctors.iter().copied().filter(|&i| self.func(i).is_some_and(|f| !f.params.is_empty() && f.params.len() - 1 <= f.defaults)).collect();
                    let all_class = !one_arg.is_empty()
                        && self.classes[tc].ctors.iter().all(|&i| self.func(i).is_some_and(|f| !f.template))
                        && one_arg.iter().all(|&i| {
                            let f = self.func(i).unwrap();
                            let pt = self.var(f.params[0]).map(|v| nonref(&v.ty).t).unwrap_or(Ty::Unknown);
                            matches!(pt, Ty::Class(pc) if !self.class_uncertain(pc))
                        });
                    if all_class && (et.t.is_arithmetic() || matches!(et.t, Ty::Enum(_) | Ty::Ptr(_) | Ty::NullPtr)) {
                        self.error(line, "CTOR-22", format!("conversion to '{}' needs two user-defined conversions", self.classes[tc].name));
                        return Ok(Expr::unknown(line, "bad conversion"));
                    }
                }
                return Ok(Expr::new(EK::Unknown("converting constructor".into()), to.clone(), Cat::PR, line));
            }
            _ => {}
        }
        if let Ty::Class(sc) = nonref(&e.ty).t {
            if let Some(x) = self.user_conv(&e, &target, line) {
                return Ok(x);
            }
            return match self.class_converts(sc) {
                Some(false) => Err(format!("class '{}' has no conversion function", self.classes[sc].name)),
                _ => Ok(Expr::new(EK::Unknown("conversion function".into()), to.clone(), Cat::PR, line)),
            };
        }
        let null = self.is_null_ptr_constant(&e);
        let e = self.rvalue(e);
        let from = e.ty.t.clone();
        let tt = target.t.clone();
        if from == tt {
            return Ok(e);
        }
        match (&from, &tt) {
            (_, Ty::Void) => Ok(self.conv(e, Conv::ToVoid, target)),
            (Ty::Void, _) => Err("void value".into()),
            // bool target
            (Ty::NullPtr, Ty::Bool) => {
                if direct {
                    Ok(self.conv(e, Conv::ToBool, target))
                } else {
                    // CONV-12
                    Err("nullptr converts to bool only in direct initialization".into())
                }
            }
            (f, Ty::Bool) if f.is_arithmetic() || matches!(f, Ty::Ptr(_) | Ty::MemPtr(..)) || self.is_unscoped_enum(f) => Ok(self.conv(e, Conv::ToBool, target)),
            (Ty::Enum(fe), _) if self.enums[*fe].scoped => Err("scoped enumeration does not convert implicitly".into()),
            (f, Ty::Enum(_)) if !matches!(f, Ty::Enum(_)) || f != &tt => Err("no implicit conversion to an enumeration".into()),
            (f, t) if (f.is_arithmetic() || self.is_unscoped_enum(f)) && t.is_arithmetic() => Ok(self.conv(e, Conv::Arith, target)),
            (_, Ty::Ptr(tp)) => {
                if null {
                    return Ok(self.conv(e, Conv::NullPtr, target));
                }
                match &from {
                    Ty::Ptr(fp) => {
                        if self.ptr_convertible(fp, tp) {
                            if let (Ty::Class(d), Ty::Class(b)) = (&fp.t, &tp.t) {
                                if d != b && !self.in_template() && !self.class_uncertain(*d) {
                                    // CONV-17: the base must be unambiguous
                                    // and accessible.
                                    if self.base_count(*b, *d, 0) > 1 {
                                        self.error(line, "CONV-17", "conversion to an ambiguous base");
                                        return Ok(Expr::unknown(line, "bad conversion"));
                                    }
                                    if !self.base_accessible(*d, *b) {
                                        self.error(line, "CONV-17", "conversion to a base that is not accessible");
                                        return Ok(Expr::unknown(line, "bad conversion"));
                                    }
                                }
                            }
                            Ok(self.conv(e, Conv::PtrConv, target))
                        } else {
                            Err(format!("no implicit conversion from {} to {}", show(&e.ty), show(to)))
                        }
                    }
                    _ => Err(format!("no implicit conversion from {} to a pointer (CONV-8)", show(&e.ty))),
                }
            }
            (_, Ty::NullPtr) => {
                if null {
                    Ok(self.conv(e, Conv::NullPtr, target))
                } else {
                    Err("no conversion to std::nullptr_t".into())
                }
            }
            (_, Ty::MemPtr(..)) => {
                if let (Ty::MemPtr(_, a), Ty::MemPtr(_, b)) = (&from, &tt) {
                    if let (Ty::Func(fa), Ty::Func(fb)) = (&a.t, &b.t) {
                        if !fa.noexcept && fb.noexcept {
                            // CONV-10: no implicit conversion that adds
                            // noexcept.
                            self.error(line, "CONV-10", "a pointer to a member function without noexcept does not convert to one with noexcept");
                            return Ok(Expr::unknown(line, "bad conversion"));
                        }
                    }
                }
                if let (Ty::MemPtr(fc, a), Ty::MemPtr(tc, b)) = (&from, &tt) {
                    if fc != tc && a.t == b.t && !self.in_template() && !self.class_uncertain(*fc) && !self.class_uncertain(*tc) {
                        // CONV-15, CONV-17: a pointer to a member of a base
                        // converts to one of a derived class, through a base
                        // that is not virtual, ambiguous, or inaccessible.
                        let bad = if self.is_base_of(*fc, *tc) {
                            self.has_virtual_base(*tc, *fc, 0) || self.base_count(*fc, *tc, 0) > 1 || !self.base_accessible(*tc, *fc)
                        } else {
                            true
                        };
                        if bad {
                            self.error(line, "CONV-17", "invalid conversion between pointers to members");
                            return Ok(Expr::unknown(line, "bad conversion"));
                        }
                    }
                }
                if null || matches!(from, Ty::MemPtr(..)) {
                    Ok(self.conv(e, Conv::NullPtr, target))
                } else {
                    Err("no conversion to a pointer to member".into())
                }
            }
            (Ty::Ptr(_) | Ty::NullPtr | Ty::MemPtr(..), t) if t.is_arithmetic() => Err("a pointer does not convert to an integer".into()),
            _ => Err(format!("no implicit conversion from {} to {}", show(&e.ty), show(to))),
        }
    }

    /// CONV-9, CONV-10, CONV-14, CV-7: pointer conversions.
    pub fn ptr_convertible(&self, from: &QTy, to: &QTy) -> bool {
        if from.t.is_unknown_any() || to.t.is_unknown_any() {
            return true;
        }
        // cv: the target must have at least the source's qualifiers
        if (from.c && !to.c) || (from.v && !to.v) {
            return false;
        }
        if to.t == Ty::Void {
            return !matches!(from.t, Ty::Func(_));
        }
        if from.t == to.t {
            return true;
        }
        match (&from.t, &to.t) {
            (Ty::Class(a), Ty::Class(b)) => self.is_base_of(*b, *a),
            (Ty::Ptr(a), Ty::Ptr(b)) => {
                // multi-level: adding const below needs const at each level above
                if !(to.c || (a.c == b.c && a.v == b.v && self.similar_exact(a, b))) {
                    return a.t == b.t && a.c == b.c && a.v == b.v;
                }
                self.qual_convertible(a, b)
            }
            (Ty::Func(a), Ty::Func(b)) => {
                // CONV-10: noexcept to non-noexcept
                let mut a2 = (**a).clone();
                a2.noexcept = b.noexcept;
                a2 == **b && (a.noexcept || !b.noexcept)
            }
            // A pointer to an array of known bound converts to a pointer
            // to an array of unknown bound, also with cv-qualifiers added
            // to the elements (outside knowledge; ICS-31 implies it).
            (Ty::Array(_, Some(_)), Ty::Array(_, None)) | (Ty::Array(..), Ty::Array(..)) => self.array_qual_ok(&from.t, &to.t),
            _ => false,
        }
    }

    /// An array type converts to `to` by adding cv-qualifiers to its
    /// elements, also through nested arrays, or by losing its bound
    /// (CV-6, CV-7).
    pub fn array_qual_ok(&self, from: &Ty, to: &Ty) -> bool {
        match (from, to) {
            (Ty::Array(a, n), Ty::Array(b, m)) => (n == m || m.is_none()) && (!a.c || b.c) && (!a.v || b.v) && (a.t == b.t || self.array_qual_ok(&a.t, &b.t)),
            _ => false,
        }
    }

    fn similar_exact(&self, a: &QTy, b: &QTy) -> bool {
        a.t == b.t
    }

    fn qual_convertible(&self, from: &QTy, to: &QTy) -> bool {
        if (from.c && !to.c) || (from.v && !to.v) {
            return false;
        }
        match (&from.t, &to.t) {
            (Ty::Ptr(a), Ty::Ptr(b)) => {
                let added = (b.c && !a.c) || (b.v && !a.v);
                if added && !(to.c) {
                    return false;
                }
                self.qual_convertible(a, b)
            }
            (a, b) => a == b,
        }
    }

    /// The number of base subobjects of class `base` in `derived`. A
    /// virtual base counts once.
    pub fn base_count(&self, base: usize, derived: usize, depth: u32) -> usize {
        fn walk(p: &P, base: usize, c: usize, depth: u32, nonvirt: &mut usize, virt: &mut bool) {
            if depth > 16 {
                return;
            }
            for b in &p.classes[c].bases {
                let Some(bc) = b.class else { continue };
                if bc == base {
                    if b.virt {
                        *virt = true;
                    } else {
                        *nonvirt += 1;
                    }
                } else if !b.virt {
                    walk(p, base, bc, depth + 1, nonvirt, virt);
                } else {
                    // through a virtual base: count that base's subobjects
                    // once
                    let mut n = 0;
                    let mut v = false;
                    walk(p, base, bc, depth + 1, &mut n, &mut v);
                    if n > 0 || v {
                        *virt = true;
                    }
                }
            }
        }
        let (mut n, mut v) = (0, false);
        walk(self, base, derived, depth, &mut n, &mut v);
        n + usize::from(v)
    }

    pub fn is_base_of(&self, base: usize, derived: usize) -> bool {
        for b in &self.classes[derived].bases {
            if let Some(bc) = b.class {
                if bc == base || self.is_base_of(base, bc) {
                    return true;
                }
            }
        }
        false
    }

    /// Bind a reference (REF-1, REF-2, REF-3).
    /// The expression names a bit-field.
    pub fn is_bitfield(&self, e: &Expr) -> bool {
        match &e.k {
            EK::Member(_, f) => matches!(&self.decls[*f].k, DK::Field(fi) if fi.bits.is_some()),
            _ => false,
        }
    }

    pub fn bind_ref(&mut self, e: Expr, to: &QTy, line: u32, rule: &'static str) -> Expr {
        let (is_l, inner) = match &to.t {
            Ty::LRef(i) => (true, (**i).clone()),
            Ty::RRef(i) => (false, (**i).clone()),
            _ => unreachable!(),
        };
        if e.ty.is_unknown() || inner.is_unknown() {
            return Expr::new(EK::Conv(Conv::Bind, Box::new(e)), inner, Cat::L, line);
        }
        if matches!(e.k, EK::InitList(_)) && e.ty.is_unknown() {
            return Expr::new(EK::Unknown("reference bound to a braced list".into()), inner, Cat::L, line);
        }
        if matches!(e.k, EK::Overloads(_) | EK::Func(_)) && matches!(inner.t, Ty::Func(_)) {
            return Expr::new(EK::Conv(Conv::Bind, Box::new(e)), inner, Cat::L, line);
        }
        let et = e.ty.clone();
        let related = et.t == inner.t
            || matches!((&et.t, &inner.t), (Ty::Class(a), Ty::Class(b)) if self.is_base_of(*b, *a))
            || (matches!((&et.t, &inner.t), (Ty::Array(..), Ty::Array(..))) && self.array_qual_ok(&et.t, &inner.t));
        let cv_ok = (!et.c || inner.c) && (!et.v || inner.v);
        if matches!(et.t, Ty::Class(_)) && matches!(inner.t, Ty::Func(_)) {
            return Expr::new(EK::Unknown("conversion to a function reference".into()), inner, Cat::L, line);
        }
        if !related && matches!(et.t, Ty::Class(_)) {
            // ICS-11: a conversion function whose result the reference
            // binds.
            if let Some(x) = self.user_conv(&e, to, line) {
                return x;
            }
        }
        if is_l && !(inner.c && !inner.v) {
            // REF-1: a non-const lvalue reference binds only to an lvalue
            // of a related type with no more qualifiers.
            if e.cat != Cat::L {
                if matches!(inner.t, Ty::Func(_)) {
                    return Expr::new(EK::Conv(Conv::Bind, Box::new(e)), inner, Cat::L, line);
                }
                self.error(line, "REF-8", format!("non-const lvalue reference cannot bind to an rvalue ({})", rule));
            } else if related && self.is_bitfield(&e) {
                self.error(line, "REF-8", "non-const lvalue reference cannot bind to a bit-field");
            } else if !related {
                if let Ty::Class(_) = et.t {
                    // conversion function to a reference: not modeled
                    return Expr::new(EK::Unknown("conversion to reference".into()), inner, Cat::L, line);
                }
                self.error(line, "REF-8", format!("non-const lvalue reference to {} cannot bind to {}", show(&inner), show(&et)));
            } else if !cv_ok {
                self.error(line, "REF-8", "reference binding drops a cv-qualifier");
            }
            return Expr::new(EK::Conv(Conv::Bind, Box::new(e)), inner, Cat::L, line);
        }
        if !is_l && related && e.cat == Cat::L && !matches!(inner.t, Ty::Func(_)) {
            // REF-3: an rvalue reference must not bind to an lvalue of the
            // same type.
            self.error(line, "REF-10", "rvalue reference cannot bind to an lvalue");
            return Expr::new(EK::Conv(Conv::Bind, Box::new(e)), inner, Cat::L, line);
        }
        if related && !cv_ok {
            self.error(line, "REF-10", "reference binding drops a cv-qualifier");
            return Expr::new(EK::Conv(Conv::Bind, Box::new(e)), inner, Cat::L, line);
        }
        if related && (e.cat != Cat::PR || matches!(et.t, Ty::Class(_))) {
            return Expr::new(EK::Conv(Conv::Bind, Box::new(e)), inner, Cat::L, line);
        }
        // REF-2: bind to a temporary holding the converted value.
        match self.implicit(e, &inner.unqual(), false) {
            Ok(c) => {
                if c.ty.is_unknown() {
                    return c;
                }
                if matches!(c.k, EK::Unknown(_)) || c.cat != Cat::PR {
                    return Expr::new(EK::Unknown("reference bound to a temporary".into()), inner, Cat::L, line);
                }
                // The reference binds a temporary that the converted
                // prvalue initializes.
                Expr::new(EK::Conv(Conv::Bind, Box::new(c)), inner, Cat::L, line)
            }
            Err(msg) => {
                self.error(line, rule, format!("cannot bind reference: {}", msg));
                Expr::unknown(line, "bad reference")
            }
        }
    }

    /// Contextual conversion to bool (glossary). Reports `rule` on failure.
    pub fn contextual_bool(&mut self, e: Expr, rule: &'static str) -> Expr {
        let line = e.line;
        let t = nonref(&e.ty);
        if t.is_unknown() {
            return self.conv(e, Conv::ToBool, QTy::bool_());
        }
        if self.is_scoped_enum(&t.t) {
            // ENUM-11: a scoped enumeration does not convert to bool.
            self.error(line, "ENUM-11", format!("scoped enumeration value used as a condition ({})", rule));
            return Expr::unknown(line, "bad condition");
        }
        match &t.t {
            Ty::Void => {
                self.error(line, rule, "void value used as a condition");
                Expr::unknown(line, "bad condition")
            }
            Ty::Class(c) => {
                if self.class_converts(*c) == Some(false) {
                    self.error(line, rule, format!("class '{}' does not convert to bool", self.classes[*c].name));
                }
                Expr::new(EK::Unknown("class to bool".into()), QTy::bool_(), Cat::PR, line)
            }
            Ty::Bool => self.rvalue(e),
            Ty::NullPtr => {
                let e = self.rvalue(e);
                self.conv(e, Conv::ToBool, QTy::bool_())
            }
            _ => match self.implicit(e, &QTy::bool_(), true) {
                Ok(x) => x,
                Err(m) => {
                    self.error(line, rule, m);
                    Expr::unknown(line, "bad condition")
                }
            },
        }
    }

    /// Is the conversion of `e` to `to` narrowing (LIST-2)?
    pub fn is_narrowing(&mut self, e: &Expr, to: &Ty) -> bool {
        let from = nonref(&e.ty).t;
        if from.contains_unknown() || to.contains_unknown() {
            return false;
        }
        let from_int = from.is_integral() || self.is_unscoped_enum(&from);
        let cval = || -> Option<crate::consteval::Val> { None };
        let _ = cval;
        if from.is_floating() && to.is_integral() {
            return true;
        }
        if from_int && to.is_floating() {
            if let Ok(v) = self.const_int_quiet(e) {
                // exact in the target?
                let f = v as f64;
                let exact = if *to == Ty::Float { (f as f32) as f64 == f && (f as f32) as i128 == v } else { f as i128 == v };
                return !exact;
            }
            return true;
        }
        if from.is_floating() && to.is_floating() {
            if to.float_rank() < from.float_rank() {
                if let Some(v) = self.const_float_quiet(e) {
                    let in_range = if *to == Ty::Float { v.is_nan() || v.is_infinite() || v.abs() <= f32::MAX as f64 } else { true };
                    return !in_range;
                }
                return true;
            }
            return false;
        }
        if from_int && to.is_integral() {
            let ft = if let Ty::Enum(en) = &from { self.enum_underlying(*en) } else { from.clone() };
            if *to == Ty::Bool && ft != Ty::Bool {
                if let Ok(v) = self.const_int_quiet(e) {
                    return !(v == 0 || v == 1);
                }
                return true;
            }
            let holds_all = ft.int_min() >= to.int_min() && ft.int_max() <= to.int_max();
            if holds_all {
                return false;
            }
            if let Ok(v) = self.const_int_quiet(e) {
                return !fits(v, to);
            }
            return true;
        }
        if matches!(from, Ty::Ptr(_) | Ty::MemPtr(..)) && *to == Ty::Bool {
            return true;
        }
        false
    }

    // ---------------- builders ----------------

    pub fn build_comma(&mut self, l: Expr, r: Expr, line: u32) -> Expr {
        let classes: Vec<usize> = [&l, &r].iter().filter_map(|e| if let Ty::Class(c) = nonref(&e.ty).t { Some(c) } else { None }).collect();
        if !classes.is_empty() && !self.in_template() && self.operator_visible(",", &classes) {
            // An operator function for the comma.
            match self.op_resolve(",", &[l.clone(), r.clone()], false, line) {
                crate::ovl::OpRes::Done(x) => return x,
                // OVL-12: no viable function, so the built-in meaning.
                crate::ovl::OpRes::Builtin(_) => {}
                crate::ovl::OpRes::NotHandled => {
                    if let Some(x) = self.operator_select(",", &l, Some(&r), line) {
                        return x;
                    }
                    return Expr::unknown(line, "overloaded comma");
                }
            }
        }
        let ty = r.ty.clone();
        let cat = r.cat;
        let l = Expr::new(EK::Conv(Conv::ToVoid, Box::new(l)), QTy::new(Ty::Void), Cat::PR, line);
        Expr::new(EK::Comma(Box::new(l), Box::new(r)), ty, cat, line)
    }

    /// Overload resolution for an operator with a class or enumeration
    /// operand. Some(result) when an operator function decides the
    /// expression. None when the built-in operator applies; after a
    /// built-in candidate won, `op_converted` holds the operands
    /// (OVL-11).
    pub fn class_operand(&mut self, op: &str, l: &Expr, r: Option<&Expr>, rule: &'static str, line: u32) -> Option<Expr> {
        self.class_operand_x(op, l, r, false, rule, line)
    }

    pub fn class_operand_x(&mut self, op: &str, l: &Expr, r: Option<&Expr>, postfix: bool, rule: &'static str, line: u32) -> Option<Expr> {
        self.op_converted = None;
        let lt = nonref(&l.ty).t;
        let rt = r.map(|r| nonref(&r.ty).t);
        let mut classes = Vec::new();
        if let Ty::Class(c) = lt {
            classes.push(c);
        }
        if let Some(Ty::Class(c)) = rt {
            classes.push(c);
        }
        if classes.is_empty() {
            // An operator function can take an enumeration (OPR rules).
            let has_enum = matches!(lt, Ty::Enum(_)) || matches!(rt, Some(Ty::Enum(_)));
            if has_enum && self.operator_visible(op, &[]) {
                let ops: Vec<Expr> = std::iter::once(l.clone()).chain(r.cloned()).collect();
                return match self.op_resolve(op, &ops, postfix, line) {
                    crate::ovl::OpRes::Done(x) => Some(x),
                    crate::ovl::OpRes::Builtin(v) => {
                        self.op_converted = Some(v);
                        None
                    }
                    crate::ovl::OpRes::NotHandled => Some(Expr::unknown(line, "overloaded operator for an enumeration")),
                };
            }
            return None;
        }
        if self.in_template() {
            return Some(Expr::unknown(line, "class operand in template"));
        }
        let all_known = classes.iter().all(|&c| self.class_converts(c) == Some(false) && self.classes[c].complete);
        if all_known && !self.operator_visible(op, &classes) {
            self.error(line, rule, format!("no operator '{}' for class operand", op));
            return Some(Expr::unknown(line, "bad class operand"));
        }
        if op != "," && op != "=" {
            let ops: Vec<Expr> = std::iter::once(l.clone()).chain(r.cloned()).collect();
            match self.op_resolve(op, &ops, postfix, line) {
                crate::ovl::OpRes::Done(x) => return Some(x),
                crate::ovl::OpRes::Builtin(v) => {
                    self.op_converted = Some(v);
                    return None;
                }
                crate::ovl::OpRes::NotHandled => {}
            }
        }
        if let Some(x) = self.operator_select(op, l, r, line) {
            return Some(x);
        }
        Some(Expr::unknown(line, "overloaded operator"))
    }

    pub fn build_binary(&mut self, op: BOp, l: Expr, r: Expr, line: u32) -> Expr {
        let sp = op.spelling();
        let rule = match op {
            BOp::Mul | BOp::Div | BOp::Rem => "MUL-3",
            BOp::Add | BOp::Sub => "ADD-3",
            BOp::Shl | BOp::Shr => "SHF-3",
            BOp::BitAnd | BOp::BitOr | BOp::BitXor => "BIT-3",
            BOp::LAnd | BOp::LOr => "LOG-3",
            _ => "CMP-9",
        };
        if matches!(l.k, EK::InitList(_)) || matches!(r.k, EK::InitList(_)) {
            return Expr::unknown(line, "braced list operand");
        }
        if let Some(x) = self.class_operand(sp, &l, Some(&r), rule, line) {
            return x;
        }
        let (l, r) = match self.op_converted.take() {
            Some(v) if v.len() == 2 => (v[0].clone(), v[1].clone()),
            _ => (l, r),
        };
        // Recommended practice: parentheses warnings.
        let is_cmp = |e: &Expr| matches!(e.k, EK::Binary(BOp::Lt | BOp::Gt | BOp::Le | BOp::Ge | BOp::Eq | BOp::Ne, ..)) && !e.paren;
        if matches!(op, BOp::BitAnd | BOp::BitOr | BOp::BitXor) && (is_cmp(&l) || is_cmp(&r)) {
            self.warn(line, "BIT-6", "comparison as an operand of a bitwise operator without parentheses");
        }
        if op == BOp::LOr && [&l, &r].iter().any(|e| matches!(e.k, EK::Binary(BOp::LAnd, ..)) && !e.paren) {
            self.warn(line, "LOG-8", "'&&' within '||' without parentheses");
        }
        if matches!(op, BOp::Eq | BOp::Ne | BOp::Lt | BOp::Gt | BOp::Le | BOp::Ge) && matches!(l.k, EK::Unary(UOp::Not, _)) && !l.paren {
            self.warn(line, "LOG-9", "'!' applies only to the left operand of a comparison");
        }
        if op == BOp::LAnd || op == BOp::LOr {
            let l = self.contextual_bool(l, "LOG-3");
            let r = self.contextual_bool(r, "LOG-3");
            return Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), QTy::bool_(), Cat::PR, line);
        }
        let lnull = self.is_null_ptr_constant(&l);
        let rnull = self.is_null_ptr_constant(&r);
        let both_arrays = matches!(nonref(&l.ty).t, Ty::Array(..)) && matches!(nonref(&r.ty).t, Ty::Array(..));
        let l = self.rvalue(l);
        let r = self.rvalue(r);
        let lt = l.ty.t.clone();
        let rt = r.ty.t.clone();
        if lt.contains_unknown() || rt.contains_unknown() {
            let ty = match op {
                BOp::Lt | BOp::Gt | BOp::Le | BOp::Ge | BOp::Eq | BOp::Ne => QTy::bool_(),
                _ => QTy::unknown(),
            };
            return Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), ty, Cat::PR, line);
        }
        let ls = self.is_scoped_enum(&lt);
        let rs = self.is_scoped_enum(&rt);
        match op {
            BOp::Mul | BOp::Div | BOp::Rem | BOp::BitAnd | BOp::BitOr | BOp::BitXor => {
                let need_int = matches!(op, BOp::Rem | BOp::BitAnd | BOp::BitOr | BOp::BitXor);
                let ok = |p: &P, t: &Ty| if need_int { p.is_int_like(t) } else { p.is_arith_like(t) };
                if ls || rs {
                    // ENUM-3: no arithmetic on a scoped enumeration.
                    self.error(line, rule, format!("scoped enumeration operand of '{}' (ENUM-3)", sp));
                    return Expr::unknown(line, "bad operand");
                }
                if !ok(self, &lt) || !ok(self, &rt) {
                    let r2 = if op == BOp::Rem && (lt.is_floating() || rt.is_floating()) { "MUL-4" } else { rule };
                    self.error(line, r2, format!("invalid operands to '{}': {} and {}", sp, show(&l.ty), show(&r.ty)));
                    return Expr::unknown(line, "bad operand");
                }
                self.enum_mix_warning(&lt, &rt, line, op);
                let lb = self.bitfield_promotion(&l).unwrap_or(self.arith_base(&lt));
                let rb = self.bitfield_promotion(&r).unwrap_or(self.arith_base(&rt));
                let ct = usual_arith(&lb, &rb);
                let l = self.conv(l, Conv::Arith, QTy::new(ct.clone()));
                let r = self.conv(r, Conv::Arith, QTy::new(ct.clone()));
                Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), QTy::new(ct), Cat::PR, line)
            }
            BOp::Add | BOp::Sub => {
                if ls || rs {
                    self.error(line, "ADD-3", format!("scoped enumeration operand of '{}' (ENUM-3)", sp));
                    return Expr::unknown(line, "bad operand");
                }
                if self.is_arith_like(&lt) && self.is_arith_like(&rt) {
                    self.enum_mix_warning(&lt, &rt, line, op);
                    let lb = self.bitfield_promotion(&l).unwrap_or(self.arith_base(&lt));
                    let rb = self.bitfield_promotion(&r).unwrap_or(self.arith_base(&rt));
                    let ct = usual_arith(&lb, &rb);
                    let l = self.conv(l, Conv::Arith, QTy::new(ct.clone()));
                    let r = self.conv(r, Conv::Arith, QTy::new(ct.clone()));
                    return Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), QTy::new(ct), Cat::PR, line);
                }
                let ptr_ok = |p: &mut P, t: &Ty| -> bool {
                    if let Ty::Ptr(pt) = t {
                        if pt.t == Ty::Void || matches!(pt.t, Ty::Func(_)) || (!p.is_complete(&pt.t) && !p.in_template()) {
                            return false;
                        }
                        return true;
                    }
                    false
                };
                if matches!(lt, Ty::Ptr(_)) && self.is_int_like(&rt) {
                    if !ptr_ok(self, &lt) {
                        self.error(line, "ADD-3", "arithmetic on a pointer to void, a function, or an incomplete type");
                        return Expr::unknown(line, "bad operand");
                    }
                    let ty = l.ty.clone();
                    return Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), ty, Cat::PR, line);
                }
                if op == BOp::Add && matches!(rt, Ty::Ptr(_)) && self.is_int_like(&lt) {
                    if !ptr_ok(self, &rt) {
                        self.error(line, "ADD-3", "arithmetic on a pointer to void, a function, or an incomplete type");
                        return Expr::unknown(line, "bad operand");
                    }
                    let ty = r.ty.clone();
                    // normalize to pointer + integer
                    return Expr::new(EK::Binary(op, Box::new(r), Box::new(l)), ty, Cat::PR, line);
                }
                if op == BOp::Sub {
                    if let (Ty::Ptr(a), Ty::Ptr(b)) = (&lt, &rt) {
                        if a.t == b.t && ptr_ok(self, &lt) {
                            return Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), QTy::new(Ty::ptrdiff_t()), Cat::PR, line);
                        }
                    }
                }
                self.error(line, "ADD-3", format!("invalid operands to '{}': {} and {}", sp, show(&l.ty), show(&r.ty)));
                Expr::unknown(line, "bad operand")
            }
            BOp::Shl | BOp::Shr => {
                if ls || rs || !self.is_int_like(&lt) || !self.is_int_like(&rt) {
                    self.error(line, "SHF-3", format!("invalid operands to '{}': {} and {}", sp, show(&l.ty), show(&r.ty)));
                    return Expr::unknown(line, "bad operand");
                }
                // SHF-4: each operand is promoted on its own.
                let l = self.promote_expr(l);
                let r = self.promote_expr(r);
                let ty = l.ty.clone();
                // SHF-9: warn about a constant count out of range.
                if let Ok(n) = self.const_int_quiet(&r) {
                    if n < 0 || n >= ty.t.int_bits() as i128 {
                        self.warn(line, "SHF-9", "shift count is negative or too large");
                    }
                }
                Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), ty, Cat::PR, line)
            }
            BOp::Lt | BOp::Gt | BOp::Le | BOp::Ge | BOp::Eq | BOp::Ne => self.build_compare(op, l, r, lnull, rnull, line),
            BOp::Spaceship => self.build_three_way(l, r, lnull, rnull, both_arrays, line),
            BOp::LAnd | BOp::LOr => unreachable!(),
        }
    }

    /// The built-in `<=>` (CMP-12, CMP-13, CMP-17, CMP-18, CMP-24,
    /// CMP-25). The result is the static member of the comparison
    /// category type that the comparison of the values chooses.
    fn build_three_way(&mut self, l: Expr, r: Expr, lnull: bool, rnull: bool, both_arrays: bool, line: u32) -> Expr {
        let (lt, rt) = (l.ty.t.clone(), r.ty.t.clone());
        let unk = |ty: QTy| Expr::new(EK::Unknown("<=> (comparison category types are library types)".into()), ty, Cat::PR, line);
        if both_arrays {
            self.error(line, "CMP-24", "three-way comparison of two arrays");
            return Expr::unknown(line, "bad operand");
        }
        let (ls, rs) = (self.is_scoped_enum(&lt), self.is_scoped_enum(&rt));
        if ls != rs || (ls && lt != rt) {
            self.error(line, "CMP-23", "three-way comparison of a scoped enumeration with another type");
            return Expr::unknown(line, "bad operand");
        }
        let arith = |p: &P, t: &Ty| t.is_arithmetic() || matches!(t, Ty::Enum(_)) && !p.is_scoped_enum(t);
        if (lt == Ty::Bool) != (rt == Ty::Bool) && arith(self, &lt) && arith(self, &rt) {
            // CMP-13: a bool and an operand of another type.
            self.error(line, "CMP-13", "three-way comparison of a bool with an operand of another type");
            return Expr::unknown(line, "bad operand");
        }
        let (le, re) = (self.is_unscoped_enum(&lt), self.is_unscoped_enum(&rt));
        if (le && re && lt != rt) || (le && rt.is_floating()) || (re && lt.is_floating()) {
            // CMP-18: two unscoped enumerations, or one and a floating value.
            self.error(line, "CMP-18", "three-way comparison of different enumerations, or of an enumeration and a floating value");
            return Expr::unknown(line, "bad operand");
        }
        let fnptr = |t: &Ty| matches!(t, Ty::Ptr(x) if matches!(x.t, Ty::Func(_)));
        let np = |t: &Ty| *t == Ty::NullPtr;
        if (fnptr(&lt) && (fnptr(&rt) || np(&rt) || rnull)) || (fnptr(&rt) && (np(&lt) || lnull)) || (matches!(lt, Ty::MemPtr(..)) || matches!(rt, Ty::MemPtr(..))) || (np(&lt) && np(&rt)) {
            // CMP-25: no three-way comparison of these.
            self.error(line, "CMP-25", "three-way comparison of function pointers, pointers to members, or null pointer values");
            return Expr::unknown(line, "bad operand");
        }
        let lptr = matches!(lt, Ty::Ptr(_));
        let rptr = matches!(rt, Ty::Ptr(_));
        if (lptr && rnull && !np(&rt)) || (rptr && lnull && !np(&lt)) {
            // CMP-13: a pointer and the literal 0.
            self.error(line, "CMP-13", "three-way comparison of a pointer with the literal 0");
            return Expr::unknown(line, "bad operand");
        }
        let floating = lt.is_floating() || rt.is_floating();
        let name = if floating { "partial_ordering" } else { "strong_ordering" };
        let Some(cc) = self.std_class(name) else {
            // CMP-25: the comparison category type must be declared.
            self.error(line, "CMP-25", format!("three-way comparison needs 'std::{}', which is not declared", name));
            return Expr::unknown(line, "bad operand");
        };
        let ty = QTy::new(Ty::Class(cc));
        if arith(self, &lt) && arith(self, &rt) {
            // CMP-13: no narrowing conversion of an integer operand.
            let lb = self.arith_base(&lt);
            let rb = self.arith_base(&rt);
            let ct = usual_arith(&lb, &rb);
            if ct.is_integral() {
                for (e, b) in [(&l, &lb), (&r, &rb)] {
                    if !b.is_integral() || *b == ct {
                        continue;
                    }
                    let fits_type = (b.is_signed() == ct.is_signed() && b.int_bits() <= ct.int_bits()) || (!b.is_signed() && ct.is_signed() && b.int_bits() < ct.int_bits());
                    if fits_type {
                        continue;
                    }
                    let fits_value = match self.const_int_quiet(e) {
                        Ok(v) => {
                            let bits = ct.int_bits();
                            if ct.is_signed() { v >= -(1i128 << (bits - 1)) && v < (1i128 << (bits - 1)) } else { v >= 0 && v < (1i128 << bits) }
                        }
                        Err(_) => false,
                    };
                    if !fits_value {
                        self.error(line, "CMP-13", "three-way comparison needs a narrowing conversion of an operand");
                        return Expr::unknown(line, "bad operand");
                    }
                }
            }
        } else if !(lptr || rptr || lnull || rnull) {
            self.error(line, "CMP-12", format!("invalid operands to '<=>': {} and {}", show(&l.ty), show(&r.ty)));
            return Expr::unknown(line, "bad operand");
        }
        if crate::interp::has_effects(&l) || crate::interp::has_effects(&r) || self.in_template() {
            return unk(ty);
        }
        // The value: less, equal (equivalent), greater, or unordered.
        let member = |p: &mut P, n: &str| -> Option<Expr> {
            let ids = p.lookup_member(cc, n)?;
            let id = *ids.iter().find(|&&i| p.var(i).is_some())?;
            let vt = p.var(id)?.ty.clone();
            Some(Expr::new(EK::Var(id), vt, Cat::L, line))
        };
        let names: &[&str] = if floating { &["less", "equivalent", "greater", "unordered"] } else { &["less", "equal", "greater"] };
        let mut ms = Vec::new();
        for n in names {
            match member(self, n) {
                Some(e) => ms.push(e),
                None => {
                    // LSUP-2: each ordering class has its static members.
                    self.error(line, "LSUP-2", format!("'std::{}' has no static data member '{}'", name, n));
                    return Expr::unknown(line, "bad operand");
                }
            }
        }
        let (me, mu, mark) = (self.d.muted_errors, self.d.muted_unsup, self.d.list.len());
        self.d.muted += 1;
        let lt_e = self.build_binary(BOp::Lt, l.clone(), r.clone(), line);
        let eq_e = self.build_binary(BOp::Eq, l.clone(), r.clone(), line);
        let gt_e = self.build_binary(BOp::Gt, l, r, line);
        self.d.muted -= 1;
        let bad = self.d.muted_errors != me || self.d.muted_unsup != mu;
        self.d.muted_errors = me;
        self.d.muted_unsup = mu;
        self.d.list.truncate(mark);
        if bad {
            return unk(ty);
        }
        let cond = |c: Expr, a: Expr, b: Expr| Expr::new(EK::Cond(Box::new(c), Box::new(a), Box::new(b)), ty.clone(), Cat::PR, line);
        if floating {
            let inner = cond(eq_e, ms[1].clone(), ms[3].clone());
            let mid = cond(gt_e, ms[2].clone(), inner);
            cond(lt_e, ms[0].clone(), mid)
        } else {
            let inner = cond(eq_e, ms[1].clone(), ms[2].clone());
            cond(lt_e, ms[0].clone(), inner)
        }
    }

    /// The class `std::name`, when the program declares it.    /// The class `std::name`, when the program declares it.
    pub fn std_class(&self, name: &str) -> Option<usize> {
        let ns = self.lookup_in_scope(self.global, "std", false)?;
        let DK::Namespace(s) = self.decls[*ns.first()?].k else { return None };
        let ids = self.lookup_in_scope(s, name, false)?;
        match self.decls[*ids.first()?].k {
            DK::Class(c) if self.classes[c].complete => Some(c),
            _ => None,
        }
    }

    fn arith_base(&self, t: &Ty) -> Ty {
        match t {
            Ty::Enum(e) => self.promoted(&Ty::Enum(*e)),
            t => t.clone(),
        }
    }

    fn enum_mix_warning(&mut self, lt: &Ty, rt: &Ty, line: u32, op: BOp) {
        let rule = match op {
            BOp::Mul | BOp::Div | BOp::Rem => "MUL-16",
            BOp::Add | BOp::Sub => "ADD-16",
            BOp::BitAnd | BOp::BitOr | BOp::BitXor => "BIT-8",
            _ => "CMP-21",
        };
        let le = matches!(lt, Ty::Enum(_));
        let re = matches!(rt, Ty::Enum(_));
        if (le && re && lt != rt) || (le && rt.is_floating()) || (re && lt.is_floating()) {
            if matches!(op, BOp::BitAnd | BOp::BitOr | BOp::BitXor) && !(le && re) {
                return;
            }
            self.warn(line, rule, "arithmetic between different enumerations or an enumeration and a floating value");
        }
    }

    fn build_compare(&mut self, op: BOp, l: Expr, r: Expr, lnull: bool, rnull: bool, line: u32) -> Expr {
        let lt = l.ty.t.clone();
        let rt = r.ty.t.clone();
        let eq = matches!(op, BOp::Eq | BOp::Ne);
        let ls = self.is_scoped_enum(&lt);
        let rs = self.is_scoped_enum(&rt);
        let mk = |l: Expr, r: Expr| Expr::new(EK::Binary(op, Box::new(l), Box::new(r)), QTy::bool_(), Cat::PR, line);
        if ls || rs {
            if lt == rt {
                return mk(l, r);
            }
            // ENUM-3, CMP-10: a scoped value compares only with its own type.
            self.error(line, "CMP-23", "comparison of a scoped enumeration with another type (ENUM-3)");
            return Expr::unknown(line, "bad comparison");
        }
        if self.is_arith_like(&lt) && self.is_arith_like(&rt) {
            self.enum_mix_warning(&lt, &rt, line, op);
            // CMP-4: signed/unsigned comparison warning
            let ct = usual_arith(&self.arith_base(&lt), &self.arith_base(&rt));
            let lp = self.arith_base(&lt);
            let rp = self.arith_base(&rt);
            if ct.is_integral() && !ct.is_signed() && (promote(&lp).is_signed() != promote(&rp).is_signed()) {
                let signed_side_const = if promote(&lp).is_signed() { self.const_int_quiet(&l).is_ok_and(|v| v >= 0) } else { self.const_int_quiet(&r).is_ok_and(|v| v >= 0) };
                if !signed_side_const {
                    self.warn(line, "CMP-4", "comparison of integers of different signedness");
                }
            }
            if let EK::Binary(BOp::Lt | BOp::Gt | BOp::Le | BOp::Ge, ..) = &l.k {
                if !l.paren && !eq {
                    self.warn(line, "CMP-14", "chained comparison");
                }
            }
            let l = self.conv(l, Conv::Arith, QTy::new(ct.clone()));
            let r = self.conv(r, Conv::Arith, QTy::new(ct));
            return mk(l, r);
        }
        let lp = matches!(lt, Ty::Ptr(_));
        let rp = matches!(rt, Ty::Ptr(_));
        if lp && rp {
            let (Ty::Ptr(a), Ty::Ptr(b)) = (&lt, &rt) else { unreachable!() };
            if self.ptr_convertible(a, b) || self.ptr_convertible(b, a) || self.composite_ptr(a, b) {
                if matches!(a.t, Ty::Func(_)) != matches!(b.t, Ty::Func(_)) {
                    self.error(line, "CMP-9", "comparison of a function pointer and an object pointer");
                    return Expr::unknown(line, "bad comparison");
                }
                return mk(l, r);
            }
            self.error(line, "CMP-9", format!("comparison of pointers to unrelated types {} and {}", show(&l.ty), show(&r.ty)));
            return Expr::unknown(line, "bad comparison");
        }
        let to_null = |p: &P, e: Expr, t: &QTy| -> Expr { p.conv(e, Conv::NullPtr, t.clone()) };
        if (lp && rnull) || (rp && lnull) || (lp && rt == Ty::NullPtr) || (rp && lt == Ty::NullPtr) {
            if !eq {
                // CMP-9: a pointer and a null constant for a relational operator.
                self.error(line, "CMP-9", "relational comparison of a pointer with a null pointer constant");
                return Expr::unknown(line, "bad comparison");
            }
            let (l, r) = if lp { let t = l.ty.clone(); (l, to_null(self, r, &t)) } else { let t = r.ty.clone(); (to_null(self, l, &t), r) };
            return mk(l, r);
        }
        if lt == Ty::NullPtr && rt == Ty::NullPtr || (lt == Ty::NullPtr && rnull) || (rt == Ty::NullPtr && lnull) {
            if !eq {
                self.error(line, "CMP-9", "relational comparison of two nullptr values");
                return Expr::unknown(line, "bad comparison");
            }
            let nt = QTy::new(Ty::NullPtr);
            let l = to_null(self, l, &nt);
            let r = to_null(self, r, &nt);
            return mk(l, r);
        }
        let lm = matches!(lt, Ty::MemPtr(..));
        let rm = matches!(rt, Ty::MemPtr(..));
        if lm || rm {
            if !eq {
                self.error(line, "CMP-9", "relational comparison of pointers to members");
                return Expr::unknown(line, "bad comparison");
            }
            return mk(l, r);
        }
        if (lp && self.is_int_like(&rt)) || (rp && self.is_int_like(&lt)) {
            self.error(line, "CMP-9", "comparison between a pointer and an integer");
            return Expr::unknown(line, "bad comparison");
        }
        self.error(line, "CMP-9", format!("invalid operands to '{}': {} and {}", op.spelling(), show(&l.ty), show(&r.ty)));
        Expr::unknown(line, "bad comparison")
    }

    fn composite_ptr(&self, a: &QTy, b: &QTy) -> bool {
        // CMP-19: types that differ only in cv-qualifiers at any level
        match (&a.t, &b.t) {
            (Ty::Ptr(x), Ty::Ptr(y)) => self.composite_ptr(x, y),
            (x, y) => x == y,
        }
    }

    /// A template-id of one function template whose explicit arguments
    /// give every template argument names one specialization (DED-2,
    /// OVL-20).
    pub fn resolve_templ_id(&mut self, e: Expr, line: u32) -> Expr {
        let EK::TemplId(None, fs, raw) = &e.k else { return e };
        if self.in_template() {
            return e;
        }
        let ts: Vec<DeclId> = fs.iter().copied().filter(|&f| self.is_func_templ(f)).collect();
        if ts.len() != 1 || fs.iter().any(|&f| self.func(f).is_some_and(|x| !x.template)) {
            return e;
        }
        let tid = self.decl_tid[&ts[0]];
        let n = self.templs[tid].params.len();
        let Ok(Some(mut args)) = self.targs_for(tid, (**raw).clone(), line, true) else { return e };
        // The defaults and empty packs give the rest.
        while args.len() < n {
            let i = args.len();
            if self.templs[tid].params[i].pack {
                args.push(TArg::Pack(vec![]));
                continue;
            }
            match self.default_arg(tid, i, &args) {
                Some(a) if !a.is_dep() => args.push(a),
                _ => return e,
            }
        }
        match self.func_spec(tid, &args, line) {
            crate::templ::SpecRes::Ok(s) => {
                let ft = self.func(s).map(|f| QTy::new(Ty::Func(Box::new(f.fty.clone())))).unwrap_or(QTy::unknown());
                self.use_fn(s);
                Expr::new(EK::Func(s), ft, Cat::L, line)
            }
            _ => e,
        }
    }

    pub fn build_unary(&mut self, op: UOp, e: Expr, line: u32) -> Expr {
        let sp = match op {
            UOp::Plus => "+",
            UOp::Neg => "-",
            UOp::Not => "!",
            UOp::BitNot => "~",
            UOp::Deref => "*",
            UOp::AddrOf => "&",
            UOp::PreInc | UOp::PostInc => "++",
            UOp::PreDec | UOp::PostDec => "--",
        };
        if matches!(e.k, EK::BoundMember(..)) && op == UOp::AddrOf {
            // `&f` with an unqualified name of a set that holds a
            // non-static member; a call through it is OVL-44.
            return Expr::new(EK::Unary(op, Box::new(e)), QTy::unknown(), Cat::PR, line);
        }
        if matches!(e.k, EK::Overloads(_)) && op == UOp::AddrOf {
            // OVL-18: a target type chooses the function later.
            return Expr::new(EK::Unary(op, Box::new(e)), QTy::unknown(), Cat::PR, line);
        }
        let e = if op == UOp::AddrOf { self.resolve_templ_id(e, line) } else { e };
        if op == UOp::AddrOf && !self.in_template() {
            // CNS-12: the address of a function whose constraints are not
            // satisfied.
            let single = match &e.k {
                EK::Func(f) => Some(*f),
                EK::Overloads(v) if v.len() == 1 => Some(v[0]),
                _ => None,
            };
            if let Some(f) = single {
                if self.fn_cons.contains_key(&f) && self.fn_sat(f) == crate::cons::Sat::No {
                    self.error(line, "CNS-12", format!("the address of '{}', whose constraints are not satisfied", self.decls[f].name));
                    return Expr::unknown(line, "bad address");
                }
            }
        }
        if let EK::Func(f) = &e.k {
            if op == UOp::AddrOf && self.func(*f).is_some_and(|fi| fi.is_main) {
                // TR-3: a program must not take the address of main.
                self.error(line, "TR-3", "address of 'main' taken");
            }
            if op == UOp::AddrOf && self.func(*f).is_some_and(|fi| fi.consteval) && !self.in_immediate_context() {
                self.error(line, "CEV-2", "address of a consteval function taken outside an immediate function context");
            }
        }
        let rule = match op {
            UOp::Plus | UOp::Neg => "UN-2",
            UOp::Not => "LOG-3",
            UOp::BitNot => "BIT-3",
            UOp::Deref => "UN-6",
            UOp::AddrOf => "UN-9",
            _ => "UN-11",
        };
        if op != UOp::AddrOf {
            if let Some(x) = self.class_operand_x(sp, &e, None, matches!(op, UOp::PostInc | UOp::PostDec), rule, line) {
                return x;
            }
        } else if let Ty::Class(cc) = nonref(&e.ty).t {
            if self.operator_visible("&", &[cc]) || self.class_uncertain(cc) {
                match self.op_resolve("&", std::slice::from_ref(&e), false, line) {
                    crate::ovl::OpRes::Done(x) => return x,
                    // OVL-12: no viable function, so the built-in meaning.
                    crate::ovl::OpRes::Builtin(_) => {}
                    crate::ovl::OpRes::NotHandled => return Expr::unknown(line, "overloaded operator&"),
                }
            }
            if e.cat == Cat::PR && !self.in_template() {
                self.error(line, "UN-9", "address of a prvalue");
            }
            let t = QTy::new(Ty::Ptr(Box::new(e.ty.clone())));
            return Expr::new(EK::Unary(op, Box::new(e)), t, Cat::PR, line);
        }
        let e = match self.op_converted.take() {
            Some(mut v) if v.len() == 1 => v.remove(0),
            _ => e,
        };
        if e.ty.is_unknown() && !matches!(op, UOp::AddrOf) {
            let cat = if matches!(op, UOp::Deref | UOp::PreInc | UOp::PreDec) { Cat::L } else { Cat::PR };
            let ty = if op == UOp::Not { QTy::bool_() } else { QTy::unknown() };
            return Expr::new(EK::Unary(op, Box::new(e)), ty, cat, line);
        }
        match op {
            UOp::Plus | UOp::Neg => {
                let t = nonref(&e.ty).t;
                if self.is_scoped_enum(&t) {
                    self.error(line, "UN-2", "unary operator on a scoped enumeration");
                    return Expr::unknown(line, "bad operand");
                }
                if op == UOp::Plus && matches!(t, Ty::Ptr(_) | Ty::Array(..) | Ty::Func(_)) {
                    let e = self.rvalue(e);
                    let ty = e.ty.clone();
                    return Expr::new(EK::Unary(op, Box::new(e)), ty, Cat::PR, line);
                }
                if !self.is_arith_like(&t) {
                    self.error(line, "UN-2", format!("invalid operand to unary '{}': {}", sp, show(&e.ty)));
                    return Expr::unknown(line, "bad operand");
                }
                let e = if t.is_floating() { self.rvalue(e) } else { self.promote_expr(e) };
                let ty = e.ty.clone();
                Expr::new(EK::Unary(op, Box::new(e)), ty, Cat::PR, line)
            }
            UOp::Not => {
                let e = self.contextual_bool(e, "LOG-3");
                // LOG-9: `!a == b`
                Expr::new(EK::Unary(op, Box::new(e)), QTy::bool_(), Cat::PR, line)
            }
            UOp::BitNot => {
                let t = nonref(&e.ty).t;
                if !self.is_int_like(&t) {
                    self.error(line, "BIT-3", format!("invalid operand to '~': {}", show(&e.ty)));
                    return Expr::unknown(line, "bad operand");
                }
                if t == Ty::Bool {
                    self.warn(line, "BIT-7", "'~' on a bool operand");
                }
                let e = self.promote_expr(e);
                let ty = e.ty.clone();
                Expr::new(EK::Unary(op, Box::new(e)), ty, Cat::PR, line)
            }
            UOp::Deref => {
                let e = self.rvalue(e);
                match &e.ty.t {
                    Ty::Ptr(p) => {
                        if p.t == Ty::Void {
                            // UN-6: not a pointer to void.
                            self.error(line, "UN-6", "indirection through a pointer to void");
                            return Expr::unknown(line, "bad operand");
                        }
                        let t = (**p).clone();
                        Expr::new(EK::Unary(op, Box::new(e)), t, Cat::L, line)
                    }
                    _ => {
                        self.error(line, "UN-6", format!("indirection requires a pointer, found {}", show(&e.ty)));
                        Expr::unknown(line, "bad operand")
                    }
                }
            }
            UOp::AddrOf => {
                if e.ty.is_unknown() {
                    return Expr::new(EK::Unary(op, Box::new(e)), QTy::unknown(), Cat::PR, line);
                }
                if let EK::Member(_, f) = &e.k {
                    if let DK::Field(fi) = &self.decls[*f].k {
                        if fi.bits.is_some() {
                            self.error(line, "BF-4", "address of a bit-field");
                        }
                    }
                }
                if e.cat != Cat::L && !matches!(e.k, EK::Func(_)) {
                    // UN-9: no address of a prvalue or an xvalue.
                    self.error(line, "UN-9", "address of an rvalue");
                    return Expr::unknown(line, "bad operand");
                }
                let t = QTy::new(Ty::Ptr(Box::new(e.ty.clone())));
                Expr::new(EK::Unary(op, Box::new(e)), t, Cat::PR, line)
            }
            UOp::PreInc | UOp::PreDec | UOp::PostInc | UOp::PostDec => {
                let t = e.ty.clone();
                if t.t == Ty::Bool {
                    self.error(line, "UN-11", format!("'{}' on a bool", sp));
                    return Expr::unknown(line, "bad operand");
                }
                if matches!(t.t, Ty::Enum(_)) {
                    // ENUM-3, UN-11: no ++ or -- on an enumeration.
                    self.error(line, "UN-11", format!("'{}' on an enumeration", sp));
                    return Expr::unknown(line, "bad operand");
                }
                if e.cat != Cat::L {
                    self.error(line, "UN-11", format!("'{}' needs an lvalue", sp));
                    return Expr::unknown(line, "bad operand");
                }
                if t.c {
                    // CV-4: no increment of a const lvalue.
                    self.error(line, "CV-4", format!("'{}' on a const object", sp));
                    return Expr::unknown(line, "bad operand");
                }
                match &t.t {
                    x if x.is_arithmetic() => {}
                    Ty::Ptr(p) => {
                        if p.t == Ty::Void || matches!(p.t, Ty::Func(_)) || !self.is_complete(&p.t) {
                            self.error(line, "UN-11", format!("'{}' on a pointer to an incomplete or function type", sp));
                            return Expr::unknown(line, "bad operand");
                        }
                    }
                    _ => {
                        self.error(line, "UN-11", format!("invalid operand to '{}': {}", sp, show(&t)));
                        return Expr::unknown(line, "bad operand");
                    }
                }
                let cat = if matches!(op, UOp::PreInc | UOp::PreDec) { Cat::L } else { Cat::PR };
                let ty = if cat == Cat::PR { t.unqual() } else { t };
                Expr::new(EK::Unary(op, Box::new(e)), ty, cat, line)
            }
        }
    }

    pub fn build_assign(&mut self, op: &str, l: Expr, r: Expr, line: u32) -> Expr {
        let is_class = |e: &Expr| matches!(nonref(&e.ty).t, Ty::Class(_));
        let is_enum = |e: &Expr| matches!(nonref(&e.ty).t, Ty::Enum(_));
        let (l, r) = if op != "=" && !self.in_template() && (is_class(&l) || is_class(&r) || ((is_enum(&l) || is_enum(&r)) && self.operator_visible(op, &[]))) {
            // OPR-6, OVL-7: a compound assignment with a class or
            // enumeration operand.
            match self.op_resolve(op, &[l.clone(), r.clone()], false, line) {
                crate::ovl::OpRes::Done(x) => return x,
                crate::ovl::OpRes::Builtin(mut v) if v.len() == 2 => {
                    let r = v.pop().unwrap();
                    (v.pop().unwrap(), r)
                }
                _ => (l, r),
            }
        } else {
            (l, r)
        };
        let lt = l.ty.clone();
        if let Ty::Class(c) = lt.t {
            // CPY-22: the implicit copy assignment operator of a derived
            // class hides the assignment operators of its bases.
            let rt = nonref(&r.ty);
            if op == "=" && !self.in_template() && !self.class_uncertain(c) && self.classes[c].ctors.is_empty() && !self.scopes[self.classes[c].scope].names.contains_key("operator=") && (rt.t.is_arithmetic() || matches!(rt.t, Ty::Enum(_) | Ty::Ptr(_) | Ty::NullPtr)) {
                let base_assign = self.classes[c].bases.iter().any(|b| b.class.is_some_and(|bc| self.lookup_member(bc, "operator=").is_some()));
                if base_assign {
                    self.error(line, "CPY-22", format!("the implicit copy assignment operator of '{}' hides the base assignment operators", self.classes[c].name));
                }
            }
            // CPY-21 to CPY-23, UNI-11: the assignment operator used.
            if op == "=" && nonref(&r.ty).t == Ty::Class(c) && !self.in_template() {
                let mut x = r.clone();
                x.ty = nonref(&x.ty);
                if x.cat == Cat::PR {
                    x.cat = Cat::X;
                }
                self.check_copy_from(c, &x, false, true, line);
            }
            if op == "=" {
                if let Some(x) = self.operator_select("=", &l, Some(&r), line) {
                    return x;
                }
                // The implicit copy or move assignment (CPY rules), when the
                // class declares no assignment operator.
                let rt = nonref(&r.ty);
                let same = rt.t == Ty::Class(c) || matches!(rt.t, Ty::Class(d) if self.is_base_of(c, d));
                if same && !self.in_template() && !self.class_uncertain(c) && !self.scopes[self.classes[c].scope].names.contains_key("operator=") && l.cat == Cat::L && !matches!(r.k, EK::Unknown(_)) {
                    return Expr::new(EK::Assign(None, Box::new(l), Box::new(r)), nonref(&lt), Cat::L, line);
                }
            }
            return Expr::unknown(line, "class assignment");
        }
        if lt.is_unknown() {
            return Expr::new(EK::Assign(None, Box::new(l), Box::new(r)), QTy::unknown(), Cat::L, line);
        }
        if let EK::Member(_, f) = &l.k {
            let _ = f;
        }
        // ASN-2: the left operand must be a modifiable lvalue.
        if l.cat != Cat::L {
            self.error(line, "ASN-2", "assignment to an rvalue");
            return Expr::unknown(line, "bad assignment");
        }
        if matches!(lt.t, Ty::Array(..)) {
            self.error(line, "ASN-2", "assignment to an array");
            return Expr::unknown(line, "bad assignment");
        }
        if matches!(lt.t, Ty::Func(_)) || matches!(l.k, EK::Func(_) | EK::Overloads(_)) {
            self.error(line, "ASN-2", "assignment to a function");
            return Expr::unknown(line, "bad assignment");
        }
        if lt.c {
            // CV-4: no assignment to a const lvalue.
            self.error(line, "CV-4", "assignment to a const object");
            return Expr::unknown(line, "bad assignment");
        }
        if op == "=" {
            let rhs = if let EK::InitList(items) = &r.k {
                // ASN-6: a braced list for a scalar has zero or one value.
                if items.len() > 1 {
                    self.error(line, "ASN-14", "braced list with more than one value assigned to a scalar");
                    return Expr::unknown(line, "bad assignment");
                }
                if items.is_empty() {
                    Expr::new(EK::ZeroInit, lt.unqual(), Cat::PR, line)
                } else {
                    let it = items[0].clone();
                    if self.is_narrowing(&it, &lt.t) {
                        self.error(line, "LIST-2", "narrowing conversion in a braced list");
                    }
                    match self.implicit(it, &lt.unqual(), false) {
                        Ok(x) => x,
                        Err(m) => {
                            self.error(line, "ASN-12", m);
                            return Expr::unknown(line, "bad assignment");
                        }
                    }
                }
            } else {
                match self.implicit(r, &lt.unqual(), false) {
                    Ok(x) => x,
                    Err(m) => {
                        self.error(line, "ASN-12", m);
                        return Expr::unknown(line, "bad assignment");
                    }
                }
            };
            if let EK::Var(v) = l.k {
                // ATTR-6: a plain assignment does not read the variable.
                *self.assign_count.entry(v).or_default() += 1;
            }
            return Expr::new(EK::Assign(None, Box::new(l), Box::new(rhs)), lt, Cat::L, line);
        }
        // compound assignment: ASN-5
        if matches!(r.k, EK::InitList(_)) {
            self.error(line, "ASN-14", "braced list in a compound assignment to a scalar");
            return Expr::unknown(line, "bad assignment");
        }
        let bop = BOp::from_str(&op[..op.len() - 1]).unwrap();
        // Check validity with a copy of the left operand as an rvalue.
        let lv = self.rvalue(l.clone());
        let rr = r.clone();
        let nerr = self.d.list.len();
        let probe = self.build_binary(bop, lv, rr, line);
        if self.d.list.len() > nerr && self.d.has_errors() {
            return Expr::unknown(line, "bad compound assignment");
        }
        let ct = match &probe.ty.t {
            Ty::Ptr(_) => probe.ty.clone(),
            _ => probe.ty.clone(),
        };
        let r = if ct.t.is_arithmetic() {
            let r = self.rvalue(r);
            if bop == BOp::Shl || bop == BOp::Shr {
                self.promote_expr(r)
            } else {
                self.conv(r, Conv::Arith, ct.clone())
            }
        } else {
            self.rvalue(r)
        };
        if self.is_scoped_enum(&lt.t) || (matches!(lt.t, Ty::Enum(_)) && !matches!(ct.t, Ty::Enum(_))) {
            self.error(line, "ASN-13", "compound assignment to an enumeration");
        }
        Expr::new(EK::Assign(Some((bop, ct)), Box::new(l), Box::new(r)), lt, Cat::L, line)
    }

    pub fn build_cond(&mut self, c: Expr, a: Expr, b: Expr, line: u32) -> Expr {
        let c = self.contextual_bool(c, "COND-1");
        let at = a.ty.clone();
        let bt = b.ty.clone();
        let mk = |p: &mut P, a: Expr, b: Expr, t: QTy, cat: Cat| -> Expr {
            let _ = p;
            Expr::new(EK::Cond(Box::new(c.clone()), Box::new(a), Box::new(b)), t, cat, line)
        };
        if at.is_unknown() || bt.is_unknown() {
            return mk(self, a, b, QTy::unknown(), Cat::PR);
        }
        // COND-4: throw operands
        let a_throw = matches!(a.k, EK::Throw(_));
        let b_throw = matches!(b.k, EK::Throw(_));
        if a_throw && b_throw {
            return mk(self, a, b, QTy::new(Ty::Void), Cat::PR);
        }
        if a_throw {
            let t = b.ty.clone();
            let cat = b.cat;
            return mk(self, a, b, t, cat);
        }
        if b_throw {
            let t = a.ty.clone();
            let cat = a.cat;
            return mk(self, a, b, t, cat);
        }
        if at.t == Ty::Void || bt.t == Ty::Void {
            if at.t == bt.t {
                return mk(self, a, b, QTy::new(Ty::Void), Cat::PR);
            }
            self.error(line, "COND-13", "conditional with one void operand");
            return Expr::unknown(line, "bad conditional");
        }
        if matches!(at.t, Ty::Class(_)) || matches!(bt.t, Ty::Class(_)) {
            if at.t == bt.t && a.cat == b.cat && at.c == bt.c && at.v == bt.v {
                let t = at.clone();
                let cat = a.cat;
                return mk(self, a, b, t, cat);
            }
            return Expr::unknown(line, "class conditional");
        }
        // COND-3: same-type glvalues keep the category.
        if at.t == bt.t && a.cat == b.cat && a.cat != Cat::PR {
            let t = QTy { t: at.t.clone(), c: at.c || bt.c, v: at.v || bt.v };
            let ok = (at.c == t.c && at.v == t.v) || (bt.c == t.c && bt.v == t.v);
            if ok {
                let cat = a.cat;
                return mk(self, a, b, t, cat);
            }
        }
        let anull = self.is_null_ptr_constant(&a);
        let bnull = self.is_null_ptr_constant(&b);
        let a = self.rvalue(a);
        let b = self.rvalue(b);
        let at = a.ty.t.clone();
        let bt = b.ty.t.clone();
        if at == bt {
            let t = a.ty.clone();
            return mk(self, a, b, t, Cat::PR);
        }
        if self.is_arith_like(&at) && self.is_arith_like(&bt) || (matches!(at, Ty::Enum(_)) && matches!(bt, Ty::Enum(_))) {
            if self.is_scoped_enum(&at) || self.is_scoped_enum(&bt) {
                self.error(line, "COND-5", "conditional operands of a scoped enumeration and another type");
                return Expr::unknown(line, "bad conditional");
            }
            let ct = usual_arith(&self.arith_base(&at), &self.arith_base(&bt));
            let a = self.conv(a, Conv::Arith, QTy::new(ct.clone()));
            let b = self.conv(b, Conv::Arith, QTy::new(ct.clone()));
            return mk(self, a, b, QTy::new(ct), Cat::PR);
        }
        if matches!(at, Ty::Ptr(_)) && (bnull || bt == Ty::NullPtr) {
            let t = a.ty.clone();
            let b = self.conv(b, Conv::NullPtr, t.clone());
            return mk(self, a, b, t, Cat::PR);
        }
        if matches!(bt, Ty::Ptr(_)) && (anull || at == Ty::NullPtr) {
            let t = b.ty.clone();
            let a = self.conv(a, Conv::NullPtr, t.clone());
            return mk(self, a, b, t, Cat::PR);
        }
        if at == Ty::NullPtr && bnull || bt == Ty::NullPtr && anull {
            return mk(self, a, b, QTy::new(Ty::NullPtr), Cat::PR);
        }
        if let (Ty::Ptr(x), Ty::Ptr(y)) = (&at, &bt) {
            if self.ptr_convertible(x, y) {
                let t = b.ty.clone();
                return mk(self, a, b, t, Cat::PR);
            }
            if self.ptr_convertible(y, x) {
                let t = a.ty.clone();
                return mk(self, a, b, t, Cat::PR);
            }
            if self.composite_ptr(x, y) {
                return mk(self, a, b, QTy::unknown(), Cat::PR);
            }
        }
        if matches!(at, Ty::MemPtr(..)) || matches!(bt, Ty::MemPtr(..)) {
            return mk(self, a, b, QTy::unknown(), Cat::PR);
        }
        self.error(line, "COND-5", format!("conditional operands have no common type: {} and {}", show(&a.ty), show(&b.ty)));
        Expr::unknown(line, "bad conditional")
    }

    fn build_member_ptr(&mut self, op: &str, l: Expr, r: Expr, line: u32) -> Expr {
        let rt = self.rvalue(r);
        let lt = nonref(&l.ty);
        match &rt.ty.t {
            Ty::MemPtr(_, inner) => {
                let mut t = (**inner).clone();
                if let Ty::Func(_) = t.t {
                    return Expr::unknown(line, "pointer to member function call");
                }
                let obj_cat = if op == "->*" { Cat::L } else { l.cat };
                if op == ".*" {
                    t.c |= lt.c;
                }
                let cat = if obj_cat == Cat::L { Cat::L } else { Cat::X };
                Expr::new(EK::Unknown("pointer to member access".into()), t, cat, line)
            }
            Ty::Unknown | Ty::Dep(_) => Expr::unknown(line, "pointer to member access"),
            _ => {
                self.error(line, "MPTR-10", "right operand of a pointer-to-member operator is not a pointer to member");
                Expr::unknown(line, "bad operand")
            }
        }
    }

    pub fn build_subscript(&mut self, a: Expr, i: Expr, line: u32) -> Expr {
        let (a, i) = if (matches!(nonref(&a.ty).t, Ty::Class(_)) || matches!(nonref(&i.ty).t, Ty::Class(_))) {
            match self.op_resolve("[]", &[a.clone(), i.clone()], false, line) {
                crate::ovl::OpRes::Done(x) => return x,
                crate::ovl::OpRes::Builtin(mut v) if v.len() == 2 => {
                    let i = v.pop().unwrap();
                    (v.pop().unwrap(), i)
                }
                _ => (a, i),
            }
        } else {
            (a, i)
        };
        let at = nonref(&a.ty);
        if let Ty::Class(_) = at.t {
            return Expr::unknown(line, "overloaded operator[]");
        }
        if matches!(i.k, EK::InitList(_)) {
            return Expr::unknown(line, "braced subscript");
        }
        if let Ty::Class(_) = nonref(&i.ty).t {
            return Expr::unknown(line, "subscript with a class index");
        }
        let it = nonref(&i.ty);
        if at.is_unknown() || it.is_unknown() {
            return Expr::new(EK::Index(Box::new(a), Box::new(i)), QTy::unknown(), Cat::L, line);
        }
        // either operand can be the pointer
        let (base, idx) = if matches!(at.t, Ty::Ptr(_) | Ty::Array(..)) { (a, i) } else { (i, a) };
        let base_cat = base.cat;
        let is_array = matches!(nonref(&base.ty).t, Ty::Array(..));
        let base = self.rvalue(base);
        let idx = self.rvalue(idx);
        let Ty::Ptr(p) = &base.ty.t else {
            self.error(line, "PFX-2", "subscript of a value that is not an array or pointer");
            return Expr::unknown(line, "bad subscript");
        };
        if !self.is_int_like(&idx.ty.t) {
            self.error(line, "PFX-2", "subscript index must be an integer");
            return Expr::unknown(line, "bad subscript");
        }
        let t = (**p).clone();
        // VC-3: a subscript of an array prvalue or xvalue is an xvalue.
        let cat = if is_array && base_cat != Cat::L { Cat::X } else { Cat::L };
        Expr::new(EK::Index(Box::new(base), Box::new(idx)), t, cat, line)
    }

    pub fn build_sizeof(&mut self, ty: QTy, line: u32, is_expr: bool) -> Expr {
        self.require_ty(&nonref(&ty), line);
        let st = QTy::new(Ty::size_t());
        if ty.is_unknown() {
            return Expr::new(EK::Unknown("sizeof unknown type".into()), st, Cat::PR, line);
        }
        let t = nonref(&ty).t;
        match &t {
            Ty::Func(_) => {
                self.error(line, "UN-14", "sizeof applied to a function type");
                return Expr::new(EK::Int(1), st, Cat::PR, line);
            }
            Ty::Void => {
                self.error(line, "UN-14", "sizeof applied to void");
                return Expr::new(EK::Int(1), st, Cat::PR, line);
            }
            _ => {}
        }
        if !self.is_complete(&t) {
            if !self.in_template() {
                self.error(line, "UN-14", "sizeof applied to an incomplete type");
            }
            return Expr::new(EK::Unknown("sizeof incomplete".into()), st, Cat::PR, line);
        }
        let _ = is_expr;
        match self.sizeof_ty(&t) {
            Some(n) => Expr::new(EK::Int(n as i128), st, Cat::PR, line),
            None => Expr::new(EK::Unknown("sizeof of a class with a layout the front end does not compute".into()), st, Cat::PR, line),
        }
    }

    fn build_alignof(&mut self, ty: QTy, line: u32) -> Expr {
        self.require_ty(&nonref(&ty), line);
        let st = QTy::new(Ty::size_t());
        if ty.is_unknown() {
            return Expr::new(EK::Unknown("alignof unknown".into()), st, Cat::PR, line);
        }
        let t = nonref(&ty).t;
        let t = if let Ty::Array(e, _) = &t { e.t.clone() } else { t };
        if matches!(t, Ty::Func(_) | Ty::Void) || (!self.is_complete(&t) && !self.in_template()) {
            // UN-17: alignof needs a complete object type.
            self.error(line, "UN-17", "alignof applied to a function, void, or incomplete type");
            return Expr::new(EK::Int(1), st, Cat::PR, line);
        }
        match self.alignof_ty(&t) {
            Some(n) => Expr::new(EK::Int(n as i128), st, Cat::PR, line),
            None => Expr::new(EK::Unknown("alignof class".into()), st, Cat::PR, line),
        }
    }

    /// Whether an argument of class type matches no constructor of `c`:
    /// the argument's class is not `c` or derived from it and has no
    /// conversion function, and each constructor takes a type that is
    /// not a class.
    fn no_ctor_takes_class(&self, c: usize, a: &Expr) -> bool {
        let Ty::Class(s) = nonref(&a.ty).t else { return false };
        if s == c || self.class_uncertain(c) || self.class_uncertain(s) || !self.classes[c].complete || self.classes[c].ctors.is_empty() {
            return false;
        }
        if self.is_base_of_p(c, s) || self.class_converts(s) != Some(false) {
            return false;
        }
        self.classes[c].ctors.iter().all(|&cid| {
            let Some(f) = self.func(cid) else { return false };
            if f.template {
                return false;
            }
            f.fty.params.iter().take(1).all(|p| match &nonref(p).t {
                Ty::Class(x) => *x == c,
                Ty::Unknown | Ty::Dep(_) => false,
                _ => true,
            })
        })
    }

    pub fn build_functional(&mut self, ty: QTy, init: Expr, braced: bool, line: u32) -> Expr {
        self.require_ty(&ty, line);
        let EK::InitList(items) = &init.k else { unreachable!() };
        if ty.is_unknown() {
            return Expr::unknown(line, "functional cast to unknown type");
        }
        if let Ty::Class(c) = ty.t {
            if !braced && items.len() == 1 && items[0].cat == Cat::PR && nonref(&items[0].ty).t == Ty::Class(c) && !matches!(items[0].k, EK::Unknown(_)) {
                // VC-4: a prvalue of the class initializes the result
                // directly, with no constructor call.
                let mut x = items[0].clone();
                x.ty = ty.unqual();
                return x;
            }
            if !braced && !self.in_template() && self.classes[c].complete && !self.classes[c].ctors.is_empty() {
                let items = items.clone();
                let x = self.ctor_call(c, &items, &ty, line);
                if matches!(x.k, EK::Construct(..)) {
                    return x;
                }
            }
            if !braced && items.len() == 1 && !self.in_template() && self.no_ctor_takes_class(c, &items[0]) {
                // PFX-16: no constructor of the class takes the argument.
                self.error(line, "PFX-16", format!("no constructor of '{}' takes an argument of class type {}", self.classes[c].name, show(&nonref(&items[0].ty))));
                return Expr::unknown(line, "bad functional cast");
            }
            if braced && items.is_empty() && !self.in_template() && self.is_simple_aggregate(c) && self.classes[c].fields.is_empty() && !self.class_uncertain(c) {
                // An empty aggregate from empty braces.
                let mut e = self.convert_init(init.clone(), &ty.unqual(), InitForm::DirectList, line);
                e.cat = Cat::PR;
                e.ty = ty.unqual();
                return Expr::new(EK::Conv(Conv::NoOp, Box::new(e)), ty.unqual(), Cat::PR, line);
            }
            // A class temporary from the initialization of a class object,
            // for the interpreter. A form that raises a diagnostic stays
            // unknown, as before.
            if !self.in_template() && !self.class_uncertain(c) && self.classes[c].complete {
                let mark = self.d.list.len();
                let form = if braced || items.is_empty() { InitForm::DirectList } else { InitForm::Paren(items.len()) };
                let x = self.convert_init(init.clone(), &ty.unqual(), form, line);
                let ok = self.d.list.len() == mark && !matches!(x.k, EK::Unknown(_)) && !x.ty.is_unknown();
                self.d.list.truncate(mark);
                if ok {
                    let mut x = x;
                    if x.cat != Cat::PR || matches!(x.k, EK::InitList(_)) {
                        // A copy of a glvalue, or an aggregate's list: the
                        // prvalue of the cast initializes the temporary.
                        x = Expr::new(EK::Conv(Conv::NoOp, Box::new(x)), ty.unqual(), Cat::PR, line);
                    }
                    x.ty = ty.unqual();
                    return x;
                }
            }
            let mut e = Expr::unknown(line, "class temporary");
            e.ty = ty.unqual();
            return e;
        }
        if braced {
            let form = InitForm::DirectList;
            let mut ty = ty;
            if let Ty::Array(el, None) = &ty.t {
                // CAST-18: the bound is the number of values.
                ty = QTy { t: Ty::Array(el.clone(), Some(items.len() as u64)), c: ty.c, v: ty.v };
            }
            let e = self.convert_init(init.clone(), &ty, form, line);
            let mut e = e;
            e.cat = Cat::PR;
            e.ty = ty.unqual();
            return e;
        }
        match items.len() {
            0 => Expr::new(EK::ZeroInit, ty.unqual(), Cat::PR, line),
            1 => {
                let it = items[0].clone();
                self.build_cast(CastKind::Functional, ty, it, line)
            }
            _ => {
                self.error(line, "CAST-10", "functional cast to a scalar with more than one argument");
                Expr::unknown(line, "bad cast")
            }
        }
    }

    pub fn noexcept_of(&self, e: &Expr) -> Option<bool> {
        // UN-18: false if the operand contains a call to a function that is
        // not noexcept, or a throw.
        let mut res = Some(true);
        fn walk(p: &P, e: &Expr, res: &mut Option<bool>) {
            match &e.k {
                EK::Call(c, args, f) => {
                    match f {
                        Some(fid) => {
                            if let Some(fi) = p.func(*fid) {
                                if !fi.fty.noexcept {
                                    *res = Some(false);
                                }
                            }
                        }
                        None => match &c.ty.t {
                            Ty::Ptr(pt) => {
                                if let Ty::Func(f) = &pt.t {
                                    if !f.noexcept {
                                        *res = Some(false);
                                    }
                                }
                            }
                            _ => {
                                if *res == Some(true) {
                                    *res = None;
                                }
                            }
                        },
                    }
                    for a in args {
                        walk(p, a, res);
                    }
                }
                EK::Throw(_) => *res = Some(false),
                // NEW-19: the allocation functions are potentially throwing.
                EK::New { .. } => *res = Some(false),
                // VIRT-10: a failed cast of a reference throws.
                EK::DynamicCast(a) => {
                    // A cast to the class or a base cannot fail.
                    let up = match (nonref(&a.ty).t, &e.ty.t) {
                        (Ty::Class(f), Ty::Class(t)) => f == *t || p.is_base_of(*t, f),
                        _ => false,
                    };
                    if e.cat != Cat::PR && !up {
                        *res = Some(false);
                    }
                    walk(p, a, res);
                }
                EK::Delete(..) => {
                    if *res == Some(true) {
                        *res = None;
                    }
                }
                EK::Unknown(_) => {
                    if *res == Some(true) {
                        *res = None;
                    }
                }
                EK::Construct(cid, args) => {
                    // A constructor that is not noexcept. A defaulted one
                    // follows its members (EXS-4), which the front end does
                    // not judge here.
                    match p.func(*cid) {
                        Some(fi) if !fi.defaulted && !fi.fty.noexcept => *res = Some(false),
                        _ => {
                            if *res == Some(true) {
                                *res = None;
                            }
                        }
                    }
                    for a in args {
                        walk(p, a, res);
                    }
                }
                EK::Unary(_, a) | EK::Conv(_, a) | EK::Member(a, _) => walk(p, a, res),
                EK::Binary(_, a, b) | EK::Assign(_, a, b) | EK::Comma(a, b) | EK::Index(a, b) => {
                    walk(p, a, res);
                    walk(p, b, res);
                }
                EK::Cond(a, b, c) => {
                    walk(p, a, res);
                    walk(p, b, res);
                    walk(p, c, res);
                }
                EK::InitList(v) => {
                    for x in v {
                        walk(p, x, res);
                    }
                }
                _ => {}
            }
        }
        walk(self, e, &mut res);
        // A class temporary also runs its destructor, which the front end
        // does not judge here.
        let mut temp = false;
        walk_expr(e, &mut |x| {
            if let Ty::Class(c) = x.ty.t {
                // A default member initializer can also throw (EXS-4).
                let dmi = self.classes[c].fields.iter().any(|&f| matches!(&self.decls[f].k, DK::Field(fi) if fi.init.is_some() || matches!(fi.ty.t, Ty::Class(_))));
                if x.cat == Cat::PR && !matches!(x.k, EK::Unknown(_) | EK::Call(..)) && (self.dtor_runs_code(c, 0) || dmi) {
                    temp = true;
                }
                if x.cat == Cat::PR && matches!(x.k, EK::Call(..)) && self.dtor_runs_code(c, 0) {
                    temp = true;
                }
            }
        });
        if temp && res == Some(true) {
            return None;
        }
        res
    }

    /// Whether ending an object of class `c` runs a destructor that the
    /// program wrote.
    pub fn dtor_runs_code(&self, c: usize, depth: u32) -> bool {
        if depth > 16 {
            return true;
        }
        let ci = &self.classes[c];
        if ci.dtor.is_some_and(|d| self.func(d).is_none_or(|f| !f.defaulted)) {
            return true;
        }
        ci.bases.iter().any(|b| b.class.is_none_or(|bc| self.dtor_runs_code(bc, depth + 1)))
            || ci.fields.iter().any(|&f| matches!(&self.decls[f].k, DK::Field(fi) if crate::decl2::elem_class(&fi.ty.t).is_some_and(|fc| self.dtor_runs_code(fc, depth + 1))))
    }

    pub fn in_immediate_context(&self) -> bool {
        self.fns.iter().rev().any(|f| f.consteval_ctx)
    }
}

pub trait UnknownAny {
    fn is_unknown_any(&self) -> bool;
}
impl UnknownAny for Ty {
    fn is_unknown_any(&self) -> bool {
        self.contains_unknown()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CastKind {
    Static,
    Const,
    Reinterpret,
    Dynamic,
    CStyle,
    Functional,
}
