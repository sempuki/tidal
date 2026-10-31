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
            return Err(self.err("coroutines (outside the spec)"));
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

    fn parse_cast(&mut self) -> PR<Expr> {
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
                    let v = self.noexcept_of(&e);
                    return Ok(match v {
                        Some(b) => Expr::new(EK::Int(b as i128), QTy::bool_(), Cat::PR, line),
                        None => Expr::new(EK::Unknown("noexcept operator".into()), QTy::bool_(), Cat::PR, line),
                    });
                }
                "new" => return self.parse_new(line),
                "delete" => {
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
                    match &pt.t {
                        Ty::Ptr(_) | Ty::Unknown | Ty::Class(_) => {}
                        _ => self.error(line, "NEW-8", "operand of delete must be a pointer"),
                    }
                    let _ = arr;
                    if let Ty::Ptr(x) = &pt.t {
                        if let Ty::Class(c) = x.t {
                            if !self.in_template() && self.classes[c].complete && self.dtor_unusable(c) {
                                // DTOR-3: no delete with a deleted destructor.
                                self.error(line, "DTOR-3", format!("delete of '{}', whose destructor is deleted", self.classes[c].name));
                            }
                        }
                    }
                    return Ok(Expr::new(EK::Unknown("delete expression".into()), QTy::new(Ty::Void), Cat::PR, line));
                }
                "co_await" => return Err(self.err("coroutines (outside the spec)")),
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
                Ok(Expr::new(EK::Unknown("address of static member function".into()), t, Cat::PR, line))
            }
            DK::Var(v) => {
                let t = QTy::new(Ty::Ptr(Box::new(v.ty.clone())));
                let ve = Expr::new(EK::Var(id), v.ty.clone(), Cat::L, line);
                Ok(Expr::new(EK::Unary(UOp::AddrOf, Box::new(ve)), t, Cat::PR, line))
            }
            _ => Ok(Expr::unknown(line, "pointer to member")),
        }
    }

    fn parse_new(&mut self, line: u32) -> PR<Expr> {
        self.expect("new")?;
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
            let _ = self.parse_call_args()?;
        }
        let ds = self.parse_decl_specs(None)?;
        let base = self.ds_base(&ds).unwrap_or(QTy::unknown());
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
        let mut bounds: Vec<Option<u64>> = Vec::new();
        while self.at("[") {
            self.pos += 1;
            let bl = self.line();
            let mut b = None;
            if !self.at("]") {
                let e = self.parse_expression()?;
                let et = nonref(&e.ty);
                if matches!(et.t, Ty::Ptr(_)) {
                    // NEW-2: the size converts to the type of sizeof.
                    self.error(bl, "NEW-17", "array size in new must not be a pointer");
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
                self.check_default_init(c, line);
            }
        }
        self.parse_new_init()?;
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
        Ok(Expr::new(EK::Unknown("new expression".into()), QTy::new(Ty::Ptr(Box::new(rt))), Cat::PR, line))
    }

    fn parse_new_init(&mut self) -> PR<()> {
        if self.at("(") {
            self.pos += 1;
            if !self.at(")") {
                loop {
                    if self.at("{") {
                        self.parse_braced_init()?;
                    } else {
                        self.parse_assignment()?;
                    }
                    if !self.eat(",") {
                        break;
                    }
                }
            }
            self.expect(")")?;
        } else if self.at("{") {
            self.parse_braced_init()?;
        }
        Ok(())
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
                if self.at("]") && matches!(nonref(&e.ty).t, Ty::Class(_) | Ty::Unknown) {
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
                if extra && !matches!(nonref(&e.ty).t, Ty::Class(_) | Ty::Unknown) {
                    // COMMA-6: the built-in subscript takes one argument.
                    self.error(line, "COMMA-6", "comma in a built-in subscript");
                }
                e = self.build_subscript(e, idx, line);
            } else if self.at("(") {
                self.pos += 1;
                let save = self.no_gt;
                self.no_gt = 0;
                let args = self.parse_call_args();
                self.no_gt = save;
                let args = args?;
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
            let a = if self.at("{") { self.parse_braced_init()? } else { self.parse_assignment()? };
            self.eat("...");
            args.push(a);
            if self.eat(",") {
                continue;
            }
            self.expect(")")?;
            return Ok(args);
        }
    }

    fn parse_member_access(&mut self, obj: Expr, arrow: bool, line: u32) -> PR<Expr> {
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
        if self.at("<") {
            // member template arguments
            let ot = nonref(&obj.ty);
            let class = match (&ot.t, arrow) {
                (Ty::Class(c), false) => Some(*c),
                (Ty::Ptr(p), true) => if let Ty::Class(c) = p.t { Some(c) } else { None },
                _ => None,
            };
            let is_templ = class.and_then(|c| self.lookup_member(c, &name)).is_some_and(|ids| ids.iter().any(|&i| self.func(i).is_some_and(|f| f.template) || matches!(self.decls[i].k, DK::Template(_))));
            if is_templ || ot.is_unknown() {
                self.skip_template_args()?;
                return Ok(Expr::unknown(line, "member template"));
            }
        }
        let _ = scope;
        // object expression
        let obj = if arrow {
            let o = self.rvalue(obj);
            match &o.ty.t {
                Ty::Ptr(p) => {
                    let pt = (**p).clone();
                    Expr::new(EK::Unary(UOp::Deref, Box::new(o)), pt, Cat::L, line)
                }
                Ty::Unknown => return Ok(Expr::unknown(line, "member of unknown")),
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
        let Some(ids) = self.lookup_member(c, &name) else {
            if !self.class_uncertain(c) {
                self.error(line, "NAM-8", format!("no member named '{}'", name));
            }
            return Ok(Expr::unknown(line, "unknown member"));
        };
        let id = ids[0];
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
                if self.at("(") {
                    self.skip_balanced();
                }
                self.skip_balanced();
                return Ok(Expr::new(EK::Unknown("requires expression".into()), QTy::bool_(), Cat::PR, line));
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
            "co_await" | "co_yield" | "co_return" => return Err(self.err("coroutines (outside the spec)")),
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
                    return Ok(Expr::unknown(line, "allocation function"));
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
        self.parse_id_expression()
    }

    fn is_fold(&self) -> bool {
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
                    Some(ids) => self.name_expr(ids, line),
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
                    // designator (LIST-5): kept as an unknown item
                    self.pos += 2;
                    self.eat("=");
                    let e = if self.at("{") { self.parse_braced_init()? } else { self.parse_assignment()? };
                    let mut u = Expr::unknown(line, "designated initializer");
                    u.ty = e.ty.clone();
                    items.push(Expr::new(EK::Unknown("designator".into()), QTy::unknown(), Cat::PR, line));
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
        // captures (spec/expressions/lambdas.md grammar)
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
        let (cap_default, cap_refs) = cap_default;
        let mut is_mutable = false;
        if self.at("<") {
            self.skip_template_args()?;
        }
        let scope = if self.at("(") {
            let f = self.parse_params()?;
            f.scope
        } else {
            self.new_scope(SK::Params, Some(self.cur_scope()), "")
        };
        let mut ret = None;
        loop {
            if self.at("mutable") || self.at("constexpr") || self.at("consteval") || self.at("static") {
                if self.at("mutable") {
                    is_mutable = true;
                }
                self.pos += 1;
            } else if self.at("noexcept") {
                self.pos += 1;
                if self.at("(") {
                    self.skip_balanced();
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
        r?;
        for (l, ln) in &ctx.gotos {
            if !ctx.labels.contains_key(l) {
                self.error(*ln, "JMP-14", format!("goto names an undefined label '{}' (a lambda body is a separate function)", l));
            }
        }
        Ok(Expr::new(EK::Unknown("lambda".into()), QTy::unknown(), Cat::PR, line))
    }

    fn parse_captures(&mut self, in_fn: bool, line: u32) -> PR<(Option<bool>, Vec<String>)> {
        let mut default: Option<bool> = None; // Some(true) for [=], Some(false) for [&]
        let mut refs: Vec<String> = Vec::new();
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
                if self.at("*") {
                    self.pos += 1;
                }
                self.pos += 1;
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
                    let vi = VarInfo { ty, storage: Storage::Auto, constexpr: false, constinit: false, init: None, param: false, local: true, dependent: true, defined: true, class: None };
                    let s = self.cur_scope();
                    self.add_decl_in(s, &name, DK::Var(vi), cl);
                } else {
                    if default == Some(!by_ref) {
                        // LAM-6: an explicit capture must not repeat the default mode.
                        self.error(cl, "LAM-6", format!("explicit capture of '{}' uses the default capture mode", name));
                    }
                    match self.lookup(&name) {
                        Some(ids) => {
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
        Ok((default, refs))
    }

    fn parse_id_expression(&mut self) -> PR<Expr> {
        let line = self.line();
        let start = self.pos;
        let scope = self.parse_nested_name()?;
        if let Some(Err(())) = scope {
            // qualifier names something unknown or dependent
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
                return Ok(Expr::unknown(line, "allocation function"));
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
        if let Some(Ok(sc)) = scope {
            if let SK::Class(c) = self.scopes[sc].kind {
                if self.classes[c].name == name && start > 0 && self.toks.get(start - 1).is_some_and(|t| t.text == "&") && !self.in_template() {
                    // CTOR-1: a constructor has no address.
                    self.error(line, "CTOR-1", "the address of a constructor");
                }
            }
        }
        let found = match scope {
            None => self.lookup(&name),
            Some(Ok(s)) => self.lookup_in_scope(s, &name, true),
            Some(Err(())) => None,
        };
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
                let found = self.adl_candidates_pub(&name, &args);
                if !found.is_empty() {
                    let callee = Expr::new(EK::Overloads(found), QTy::unknown(), Cat::L, line);
                    return Ok(self.build_call(callee, args, line));
                }
                let _ = save;
                let in_class_body = self.class_depth > 0 && self.fns.is_empty();
                if !self.lookup_uncertain() && !in_class_body && args.iter().all(|a| !a.ty.is_unknown() && !matches!(nonref(&a.ty).t, Ty::Class(_) | Ty::Enum(_) | Ty::Ptr(_))) && !self.in_template() {
                    // NAM-1: no declaration, and argument-dependent lookup
                    // has nothing to search for fundamental types (NAM-5).
                    self.error(line, "NAM-8", format!("call of undeclared function '{}'", name));
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
        // template-id
        let tid = ids.iter().copied().find(|&i| matches!(self.decls[i].k, DK::Template(_)));
        if let Some(t) = tid {
            if self.at("<") {
                return self.parse_template_id_expr(t, line);
            }
        }
        if ids.iter().any(|&i| self.func(i).is_some_and(|f| f.template)) && self.at("<") {
            self.skip_template_args()?;
            return Ok(Expr::unknown(line, "function template specialization"));
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
        Ok(self.name_expr(ids, line))
    }

    fn parse_template_id_expr(&mut self, t: DeclId, line: u32) -> PR<Expr> {
        let k = self.decls[t].k.clone();
        match k {
            DK::Template(TemplKind::Var { params, init, specs }) => {
                // Parse the type arguments.
                self.expect("<")?;
                let mut args: Vec<QTy> = Vec::new();
                self.no_gt += 1;
                let r = (|| -> PR<()> {
                    if self.at(">") {
                        return Ok(());
                    }
                    loop {
                        if self.starts_type() {
                            args.push(self.parse_type_id()?);
                        } else {
                            let _ = self.parse_conditional()?;
                            args.push(QTy::unknown());
                        }
                        if !self.eat(",") {
                            return Ok(());
                        }
                    }
                })();
                self.no_gt -= 1;
                r?;
                self.expect(">")?;
                let _ = params;
                if args.iter().any(|a| a.is_unknown()) {
                    return Ok(Expr::new(EK::Unknown("variable template with unknown arguments".into()), QTy::unknown(), Cat::PR, line));
                }
                // most specialized matching partial specialization; with
                // patterns of plain parameters, the first match is enough
                for (sp, pats, e) in &specs {
                    if pats.len() != args.len() {
                        continue;
                    }
                    let mut bind: Vec<Option<QTy>> = vec![None; sp.len()];
                    let mut ok = true;
                    for (p, a) in pats.iter().zip(args.iter()) {
                        match p {
                            QTyPat::Param(i) => match &bind[*i] {
                                None => bind[*i] = Some(a.clone()),
                                Some(b) => {
                                    if b != a {
                                        ok = false;
                                    }
                                }
                            },
                            QTyPat::Ty(q) => {
                                if q != a {
                                    ok = false;
                                }
                            }
                        }
                    }
                    if ok {
                        let mut ex = (**e).clone();
                        ex.line = line;
                        return Ok(ex);
                    }
                }
                match init {
                    Some(e) => {
                        let mut ex = (*e).clone();
                        ex.line = line;
                        if ex.ty.is_unknown() {
                            return Ok(Expr::unknown(line, "variable template"));
                        }
                        Ok(ex)
                    }
                    None => Ok(Expr::unknown(line, "variable template")),
                }
            }
            DK::Template(TemplKind::Concept) => {
                self.skip_template_args()?;
                Ok(Expr::new(EK::Unknown("concept".into()), QTy::bool_(), Cat::PR, line))
            }
            DK::Template(TemplKind::Class(_)) | DK::Template(TemplKind::Alias) => {
                self.skip_template_args()?;
                self.parse_functional_cast(QTy::unknown(), line)
            }
            _ => {
                self.skip_template_args()?;
                Ok(Expr::unknown(line, "template"))
            }
        }
    }

    /// Build the expression for a name that lookup found (PRIM-12).
    pub fn name_expr(&mut self, ids: Vec<DeclId>, line: u32) -> Expr {
        let id = ids[0];
        if ids.len() == 1 && self.func(id).is_some_and(|f| f.deleted) && !self.in_template() {
            // DEL-1: no reference to a deleted function.
            self.error(line, "DEL-1", format!("use of the deleted function '{}'", self.decls[id].name));
        }
        if ids.len() == 1 && !self.in_template() {
            if let Some(f) = self.func(id) {
                // RET-3: no use before the return type is known.
                if f.ret_auto.is_some() && f.fty.ret.is_unknown() && !f.template {
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
        }
        match self.decls[id].k.clone() {
            DK::Var(v) => {
                if v.dependent {
                    if v.param {
                        return Expr::new(EK::Unknown("template parameter".into()), nonref(&v.ty), Cat::PR, line);
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
                        return Expr::new(EK::BoundMember(Box::new(this), fids), QTy::unknown(), Cat::PR, line);
                    }
                }
                if fids.len() == 1 {
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

    fn class_has_friend_op(&self, c: usize, name: &str) -> bool {
        let _ = (c, name);
        // Friend functions are declared in the enclosing namespace scope.
        false
    }

    /// Implicit conversion of a prvalue or glvalue to a target type.
    /// Returns Err with a reason if no implicit conversion exists.
    pub fn implicit(&mut self, e: Expr, to: &QTy, direct: bool) -> Result<Expr, String> {
        let line = e.line;
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
            (Ty::Array(a, Some(_)), Ty::Array(b, None)) => a == b,
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
    pub fn bind_ref(&mut self, e: Expr, to: &QTy, line: u32, rule: &'static str) -> Expr {
        let (is_l, inner) = match &to.t {
            Ty::LRef(i) => (true, (**i).clone()),
            Ty::RRef(i) => (false, (**i).clone()),
            _ => unreachable!(),
        };
        if e.ty.is_unknown() || inner.is_unknown() {
            return Expr::new(EK::Conv(Conv::Bind, Box::new(e)), inner, Cat::L, line);
        }
        if matches!(e.k, EK::InitList(_)) {
            return Expr::new(EK::Unknown("reference bound to a braced list".into()), inner, Cat::L, line);
        }
        if matches!(e.k, EK::Overloads(_) | EK::Func(_)) && matches!(inner.t, Ty::Func(_)) {
            return Expr::new(EK::Conv(Conv::Bind, Box::new(e)), inner, Cat::L, line);
        }
        let et = e.ty.clone();
        let related = et.t == inner.t
            || matches!((&et.t, &inner.t), (Ty::Class(a), Ty::Class(b)) if self.is_base_of(*b, *a))
            || matches!((&et.t, &inner.t), (Ty::Array(a, Some(_)), Ty::Array(b, None)) if a == b);
        let cv_ok = (!et.c || inner.c) && (!et.v || inner.v);
        if is_l && !(inner.c && !inner.v) {
            // REF-1: a non-const lvalue reference binds only to an lvalue
            // of a related type with no more qualifiers.
            if e.cat != Cat::L {
                if matches!(inner.t, Ty::Func(_)) {
                    return Expr::new(EK::Conv(Conv::Bind, Box::new(e)), inner, Cat::L, line);
                }
                self.error(line, "REF-8", format!("non-const lvalue reference cannot bind to an rvalue ({})", rule));
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
                Expr::new(EK::Unknown("reference bound to a temporary".into()), inner, Cat::L, line)
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
        let ty = r.ty.clone();
        let cat = r.cat;
        let l = Expr::new(EK::Conv(Conv::ToVoid, Box::new(l)), QTy::new(Ty::Void), Cat::PR, line);
        Expr::new(EK::Comma(Box::new(l), Box::new(r)), ty, cat, line)
    }

    pub fn class_operand(&mut self, op: &str, l: &Expr, r: Option<&Expr>, rule: &'static str, line: u32) -> Option<Expr> {
        // Returns Some(result) if a class operand is involved.
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
                return Some(Expr::unknown(line, "overloaded operator for an enumeration"));
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
            BOp::Spaceship => {
                if ls != rs || (ls && lt != rt) {
                    self.error(line, "CMP-23", "three-way comparison of a scoped enumeration with another type");
                }
                Expr::new(EK::Unknown("<=> (comparison category types are library types)".into()), QTy::unknown(), Cat::PR, line)
            }
            BOp::LAnd | BOp::LOr => unreachable!(),
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
        if matches!(e.k, EK::Overloads(_)) && op == UOp::AddrOf {
            return Expr::unknown(line, "address of overload set");
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
            if let Some(x) = self.class_operand(sp, &e, None, rule, line) {
                return x;
            }
        } else if let Ty::Class(cc) = nonref(&e.ty).t {
            if self.operator_visible("&", &[cc]) || self.class_uncertain(cc) {
                return Expr::unknown(line, "overloaded operator&");
            }
            if e.cat == Cat::PR && !self.in_template() {
                self.error(line, "UN-9", "address of a prvalue");
            }
            let t = QTy::new(Ty::Ptr(Box::new(e.ty.clone())));
            return Expr::new(EK::Unary(op, Box::new(e)), t, Cat::PR, line);
        }
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
            Ty::Unknown => Expr::unknown(line, "pointer to member access"),
            _ => {
                self.error(line, "MPTR-10", "right operand of a pointer-to-member operator is not a pointer to member");
                Expr::unknown(line, "bad operand")
            }
        }
    }

    pub fn build_subscript(&mut self, a: Expr, i: Expr, line: u32) -> Expr {
        let at = nonref(&a.ty);
        if let Ty::Class(_) = at.t {
            return Expr::unknown(line, "overloaded operator[]");
        }
        if matches!(i.k, EK::InitList(_)) {
            return Expr::unknown(line, "braced subscript");
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

    pub fn build_functional(&mut self, ty: QTy, init: Expr, braced: bool, line: u32) -> Expr {
        let EK::InitList(items) = &init.k else { unreachable!() };
        if ty.is_unknown() {
            return Expr::unknown(line, "functional cast to unknown type");
        }
        if let Ty::Class(c) = ty.t {
            if !braced && !items.is_empty() && !self.in_template() && self.classes[c].complete && !self.classes[c].ctors.is_empty() {
                let items = items.clone();
                let x = self.ctor_call(c, &items, &ty, line);
                if matches!(x.k, EK::Construct(..)) {
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
                EK::Unknown(_) => {
                    if *res == Some(true) {
                        *res = None;
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
        res
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
