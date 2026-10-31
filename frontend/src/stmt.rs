// Statements: spec/statements/*.md.

use crate::ast::*;
use crate::decl2::{Ctx, InitForm};
use crate::lexer::TK;
use crate::parser::*;
use crate::types::*;
use std::collections::HashSet;

impl P {
    /// Parse `{ ... }` using `scope`, which the caller has pushed. This
    /// lets a function body share the parameter scope (NAM-3, FN-1).
    pub fn parse_compound_in(&mut self, _scope: usize) -> PR<Stmt> {
        self.expect("{")?;
        let mut items = Vec::new();
        while !self.at("}") {
            if self.tok().kind == TK::Eof {
                return Err(self.err("unexpected end of file in a block"));
            }
            items.push(self.parse_statement_recover());
        }
        self.expect("}")?;
        Ok(Stmt::Block(items))
    }

    pub fn parse_compound(&mut self) -> PR<Stmt> {
        let s = self.push_scope(SK::Block);
        let r = self.parse_compound_in(s);
        self.pop_scope();
        r
    }

    pub fn parse_statement_recover(&mut self) -> Stmt {
        let start = self.pos;
        match self.parse_statement() {
            Ok(s) => s,
            Err(e) => {
                self.unsupported(e.line, format!("parse: {}", e.msg));
                let s = self.decl_scope();
                self.scopes[s].uncertain = true;
                if self.pos == start {
                    self.pos += 1;
                }
                self.recover();
                if let Some(f) = self.fns.last().and_then(|f| f.func) {
                    if let Some(fi) = self.func_mut(f) {
                        fi.opaque = true;
                    }
                }
                Stmt::Unsupported(e.msg)
            }
        }
    }

    pub fn parse_statement(&mut self) -> PR<Stmt> {
        self.skip_attributes();
        let line = self.line();
        let t = self.tok().clone();
        if t.kind == TK::Punct {
            match t.text.as_str() {
                "{" => return self.parse_compound(),
                ";" => {
                    self.pos += 1;
                    return Ok(Stmt::Null);
                }
                "}" => return Err(self.err("unexpected '}'")),
                _ => {}
            }
        }
        if t.kind == TK::Ident {
            match t.text.as_str() {
                "if" => return self.parse_if(),
                "switch" => return self.parse_switch(),
                "while" => return self.parse_while(),
                "do" => return self.parse_do(),
                "for" => return self.parse_for(),
                "break" => {
                    self.pos += 1;
                    self.expect(";")?;
                    if self.fns.last().map_or(true, |f| f.breakable == 0) {
                        // JMP-7: break inside a loop or switch.
                        self.error(line, "JMP-7", "'break' outside a loop or switch");
                    }
                    return Ok(Stmt::Break);
                }
                "continue" => {
                    self.pos += 1;
                    self.expect(";")?;
                    if self.fns.last().map_or(true, |f| f.loops == 0) {
                        // JMP-7: continue inside a loop.
                        self.error(line, "JMP-7", "'continue' outside a loop");
                    }
                    return Ok(Stmt::Continue);
                }
                "return" => return self.parse_return(),
                "goto" => {
                    self.pos += 1;
                    let l = self.ident()?;
                    self.expect(";")?;
                    let set = self.jump_set();
                    let depth = self.fns.len();
                    self.goto_sets.push((l.clone(), line, set, depth));
                    if let Some(f) = self.fns.last_mut() {
                        f.gotos.push((l.clone(), line));
                    }
                    return Ok(Stmt::Goto(l));
                }
                "case" => return self.parse_case(),
                "default" if self.at_k(1, ":") => {
                    self.pos += 2;
                    self.label_switch_check(line, None);
                    let s = self.parse_label_target()?;
                    return Ok(Stmt::Default(Box::new(s)));
                }
                "try" => {
                    self.pos += 1;
                    self.parse_compound()?;
                    self.parse_handlers()?;
                    return Ok(Stmt::Unsupported("try block".into()));
                }
                "co_return" => return Err(self.err("coroutines (outside the spec)")),
                _ => {}
            }
            // label
            if self.is_ident() && self.at_k(1, ":") && !self.at_k(1, "::") {
                let name = t.text.clone();
                self.pos += 2;
                self.define_label(&name, line);
                let s = self.parse_label_target()?;
                return Ok(Stmt::Label(name, Box::new(s)));
            }
        }
        if self.is_decl_start() {
            let r = self.parse_declaration(Ctx::Block)?;
            return Ok(r.unwrap_or(Stmt::Null));
        }
        let e = self.parse_expression()?;
        self.expect(";")?;
        self.check_unused(&e);
        Ok(Stmt::Expr(e))
    }

    /// The statement after a label; a label can end a block (STMT-5).
    fn parse_label_target(&mut self) -> PR<Stmt> {
        if self.at("}") {
            return Ok(Stmt::Null);
        }
        self.parse_statement()
    }

    fn define_label(&mut self, name: &str, line: u32) {
        let set = self.jump_set();
        let Some(f) = self.fns.last_mut() else {
            return;
        };
        if f.labels.contains_key(name) {
            // JMP-5, NAM-7: a label once per function.
            self.error(line, "JMP-5", format!("label '{}' defined twice", name));
            return;
        }
        f.labels.insert(name.to_string(), line);
        self.label_sets.push((name.to_string(), line, set, self.fns.len()));
    }

    /// Variables in scope at this point whose initialization a jump must
    /// not skip (JMP-6, SW-5).
    pub fn jump_set(&self) -> HashSet<DeclId> {
        let mut s = HashSet::new();
        // IF-17: a marker for each enclosing if constexpr / if consteval branch
        for &b in &self.cx_branches {
            s.insert(usize::MAX - b);
        }
        let base = self.fns.last().map_or(self.stack.len(), |f| f.scope_base);
        for &sc in &self.stack[base.min(self.stack.len())..] {
            for ids in self.scopes[sc].names.values() {
                for &id in ids {
                    if self.jump_vars.contains(&id) {
                        s.insert(id);
                    }
                }
            }
        }
        s
    }

    /// Check gotos against labels at the end of a function (JMP-6).
    pub fn check_jumps(&mut self, depth: usize) {
        let labels: Vec<_> = self.label_sets.iter().filter(|l| l.3 == depth).cloned().collect();
        let gotos: Vec<_> = self.goto_sets.iter().filter(|g| g.3 == depth).cloned().collect();
        self.goto_sets.retain(|g| g.3 != depth);
        for (name, gl, gset, _) in &gotos {
            if let Some((_, _ll, lset, _)) = labels.iter().find(|l| &l.0 == name) {
                let missing: Vec<&usize> = lset.iter().filter(|v| !gset.contains(v)).collect();
                if missing.iter().any(|&&v| v > usize::MAX / 2) {
                    self.error(*gl, "IF-17", format!("goto '{}' jumps into a branch of if constexpr or if consteval", name));
                } else if !missing.is_empty() {
                    self.error(*gl, "JMP-6", format!("goto '{}' jumps past the initialization of a variable", name));
                }
            }
        }
        self.label_sets.retain(|l| l.3 != depth);
    }

    fn label_switch_check(&mut self, line: u32, value: Option<(i128, u32)>) {
        let set = self.jump_set();
        let Some(f) = self.fns.last_mut() else {
            self.error(line, "SW-3", "case or default label outside a switch");
            return;
        };
        let Some(sw) = f.switches.last_mut() else {
            // SW-3: a label must be inside a switch, and a lambda or a
            // local class body is a separate function.
            self.error(line, "SW-3", "case or default label outside a switch");
            return;
        };
        let start = sw.start_set.clone();
        match value {
            None => {
                if sw.has_default {
                    self.error(line, "SW-3", "two default labels in one switch");
                }
                if let Some(f) = self.fns.last_mut() {
                    f.switches.last_mut().unwrap().has_default = true;
                }
            }
            Some((v, _)) => {
                if let Some(prev) = sw.cases.get(&v) {
                    let _ = prev;
                    self.error(line, "SW-3", format!("duplicate case value {}", v));
                } else {
                    sw.cases.insert(v, line);
                }
            }
        }
        let missing: Vec<&usize> = set.iter().filter(|v| !start.contains(v)).collect();
        if missing.iter().any(|&&v| v > usize::MAX / 2) {
            self.error(line, "IF-17", "case or default label inside a branch of if constexpr or if consteval");
        } else if !missing.is_empty() {
            // SW-5: a jump to a label must not skip an initialization.
            self.error(line, "SW-5", "jump to case label skips the initialization of a variable");
        }
    }

    fn parse_case(&mut self) -> PR<Stmt> {
        let line = self.line();
        self.expect("case")?;
        let e = self.parse_conditional()?;
        if self.at(",") {
            // COMMA-8: a comma operator in a case label needs parentheses.
            self.error(line, "COMMA-8", "comma operator in a case label");
            while !self.at(":") && self.tok().kind != TK::Eof {
                self.pos += 1;
            }
        }
        if self.at("...") {
            return Err(self.err("case range (not standard)"));
        }
        self.expect(":")?;
        let cond_ty = self.fns.last().and_then(|f| f.switches.last()).and_then(|s| s.cond_ty.clone());
        let mut value = None;
        if let Some(ct) = cond_ty {
            if !ct.is_unknown() && !e.ty.is_unknown() {
                value = self.case_value(&e, &ct, line);
            }
        } else if self.fns.last().is_some_and(|f| !f.switches.is_empty()) {
            // unknown condition type: evaluate anyway for duplicates
            if let Ok(v) = self.const_int_quiet(&e) {
                value = Some(v);
            }
        }
        match value {
            Some(v) => self.label_switch_check(line, Some((v, line))),
            None => {
                let set_check = self.fns.last().is_some_and(|f| !f.switches.is_empty());
                if !set_check {
                    self.label_switch_check(line, None.or(Some((i128::MIN, line))));
                } else {
                    let s = self.jump_set();
                    let start = self.fns.last().unwrap().switches.last().unwrap().start_set.clone();
                    if s.iter().any(|v| !start.contains(v)) {
                        self.error(line, "SW-5", "jump to case label skips the initialization of a variable");
                    }
                }
            }
        }
        let s = self.parse_label_target()?;
        Ok(Stmt::Case(value.unwrap_or(i128::MIN), Box::new(s)))
    }

    /// SW-3: a case value converts to the promoted condition type without
    /// narrowing.
    fn case_value(&mut self, e: &Expr, ct: &QTy, line: u32) -> Option<i128> {
        let et = nonref(&e.ty).t;
        if self.is_scoped_enum(&ct.t) && et != ct.t {
            self.error(line, "SW-3", "case value of a scoped enumeration switch must have that enumeration type");
            return None;
        }
        if !(et.is_integral() || matches!(et, Ty::Enum(_))) {
            self.error(line, "CE-11", "case value must have integer or enumeration type");
            return None;
        }
        if self.is_scoped_enum(&et) && et != ct.t {
            self.error(line, "SW-3", "scoped enumeration case value in a switch of another type");
            return None;
        }
        match self.const_int(e) {
            Ok(v) => {
                let target = if let Ty::Enum(en) = &ct.t { self.enum_underlying(*en) } else { ct.t.clone() };
                let target = promote(&target);
                let src = if let Ty::Enum(en) = &et { self.enum_underlying(*en) } else { et.clone() };
                if !fits(v, &target) && !(src.int_min() >= target.int_min() && src.int_max() <= target.int_max()) {
                    // CE-11: a converted constant must not narrow.
                    self.error(line, "SW-3", format!("case value {} narrows to the condition type", v));
                    return None;
                }
                Some(wrap_int(v, &target))
            }
            Err(crate::consteval::CeErr::NotConst(r, m)) => {
                if !self.in_template() {
                    self.error(line, "SW-3", format!("case value is not a constant expression ({}: {})", r, m));
                }
                None
            }
            Err(crate::consteval::CeErr::Unsupported(m)) => {
                self.unsupported(line, format!("case value: {}", m));
                None
            }
        }
    }

    /// Is the current statement a declaration (STMT-6)?
    pub fn is_decl_start(&self) -> bool {
        let t = self.tok();
        if t.kind == TK::Punct {
            if t.text == "::" {
                return self.qualified_names_type(0) && self.decl_after_type(0);
            }
            if t.text == "[" && self.at_k(1, "[") {
                return true;
            }
            return false;
        }
        if t.kind != TK::Ident {
            return false;
        }
        let s = t.text.as_str();
        if matches!(
            s,
            "static" | "extern" | "const" | "volatile" | "struct" | "class" | "union" | "enum" | "typedef" | "using" | "auto" | "constexpr" | "constinit"
                | "consteval" | "inline" | "thread_local" | "register" | "static_assert" | "template" | "namespace" | "typename" | "alignas" | "mutable"
                | "virtual" | "explicit" | "friend" | "asm"
        ) {
            return true;
        }
        if s == "decltype" {
            return true;
        }
        if crate::decl::simple_type_kw(s) {
            // `int(3)` at the start of a statement is still a declaration
            // only if it has the declarator shape.
            if self.at_k(1, "(") || self.at_k(1, "{") {
                return self.decl_after_type(0);
            }
            return true;
        }
        if is_keyword(s) {
            return false;
        }
        if self.qualified_names_type(0) {
            return self.decl_after_type(0);
        }
        false
    }

    /// After a type name starting at offset k, does a declarator follow
    /// (STMT-6, DCL-13)?
    fn decl_after_type(&self, mut k: usize) -> bool {
        // skip the type name
        if self.at_k(k, "::") {
            k += 1;
        }
        loop {
            let t = self.peek(k);
            if t.kind != TK::Ident {
                return false;
            }
            k += 1;
            if self.at_k(k, "<") {
                k = self.skip_angle_at(k);
            }
            if self.at_k(k, "::") && self.peek(k + 1).kind == TK::Ident {
                k += 1;
                continue;
            }
            break;
        }
        // simple type keywords can repeat: `unsigned int`
        while self.peek(k).kind == TK::Ident && (crate::decl::simple_type_kw(&self.peek(k).text) || matches!(self.peek(k).text.as_str(), "const" | "volatile")) {
            k += 1;
        }
        let n = self.peek(k);
        if n.kind == TK::Ident {
            return true;
        }
        if n.kind != TK::Punct {
            return false;
        }
        match n.text.as_str() {
            "*" | "&" | "&&" => {
                // `T * x` declares unless T is used as a value; T names a
                // type here, so it is a declaration.
                true
            }
            "(" => {
                // `T(x);` `T(x) = ...;` `T(*p);` are declarations; `T(1)`,
                // `T()`, and `T(x).f()` are expressions.
                let k2 = k + 1;
                let a = self.peek(k2);
                if a.kind == TK::Punct && matches!(a.text.as_str(), "*" | "&" | "&&") {
                    return true;
                }
                if self.is_member_ptr_at(k2) {
                    return true;
                }
                if a.kind == TK::Ident && !is_keyword(&a.text) {
                    // find the matching ')'
                    let mut j = k2 + 1;
                    while self.at_k(j, "[") {
                        while !self.at_k(j, "]") && self.peek(j).kind != TK::Eof {
                            j += 1;
                        }
                        j += 1;
                    }
                    if self.at_k(j, ")") {
                        let after = self.peek(j + 1);
                        return after.kind == TK::Punct && matches!(after.text.as_str(), ";" | "=" | "," | "[" | "{") || (after.kind == TK::Punct && after.text == "(" && self.looks_like_params_at(j + 1));
                    }
                    return false;
                }
                false
            }
            "[" => self.at_k(k + 1, "["),
            "..." => true,
            _ => false,
        }
    }

    fn is_member_ptr_at(&self, mut k: usize) -> bool {
        loop {
            if self.peek(k).kind != TK::Ident {
                return false;
            }
            k += 1;
            if !self.at_k(k, "::") {
                return false;
            }
            k += 1;
            if self.at_k(k, "*") {
                return true;
            }
        }
    }

    fn looks_like_params_at(&self, k: usize) -> bool {
        let n = self.peek(k + 1);
        n.kind == TK::Punct && n.text == ")" || n.kind == TK::Ident && (crate::decl::simple_type_kw(&n.text) || self.qualified_names_type(k + 1))
    }

    /// STMT-4: an expression statement with no effect.
    fn check_unused(&mut self, e: &Expr) {
        fn has_effect(p: &P, e: &Expr) -> bool {
            match &e.k {
                EK::Call(..) | EK::Assign(..) | EK::Throw(_) | EK::Unknown(_) => true,
                EK::Unary(op, a) => matches!(op, UOp::PreInc | UOp::PreDec | UOp::PostInc | UOp::PostDec) || has_effect(p, a) || (*op == UOp::Deref && a.ty.v),
                EK::Conv(Conv::ToVoid, _) => true,
                EK::Conv(Conv::LToR, a) => a.ty.v || has_effect(p, a),
                EK::Conv(_, a) => has_effect(p, a),
                EK::Binary(_, a, b) => has_effect(p, a) || has_effect(p, b),
                EK::Comma(_, b) => has_effect(p, b),
                EK::Cond(_, b, c) => has_effect(p, b) || has_effect(p, c),
                EK::Var(id) => p.var(*id).is_some_and(|v| v.ty.v),
                EK::Member(a, _) => has_effect(p, a) || e.ty.v,
                EK::Index(a, b) => has_effect(p, a) || has_effect(p, b) || e.ty.v,
                _ => false,
            }
        }
        if e.ty.is_unknown() {
            return;
        }
        if !has_effect(self, e) {
            self.warn(e.line, "STMT-4", "expression result unused");
        }
        // COMMA-5: a left operand of a comma with no effect.
        if let EK::Comma(l, _) = &e.k {
            if let EK::Conv(Conv::ToVoid, inner) = &l.k {
                if !has_effect(self, inner) && !inner.ty.is_unknown() && !inner.paren {
                    self.warn(e.line, "COMMA-5", "left operand of comma has no effect");
                }
            }
        }
    }

    fn parse_return(&mut self) -> PR<Stmt> {
        let line = self.line();
        self.expect("return")?;
        let e = if self.at(";") {
            None
        } else if self.at("{") {
            Some(self.parse_braced_init()?)
        } else {
            Some(self.parse_expression()?)
        };
        self.expect(";")?;
        let Some(f) = self.fns.last().cloned() else {
            return Ok(Stmt::Return(e));
        };
        if f.ctor_dtor {
            if e.is_some() {
                // JMP-12: no expression in a constructor or destructor.
                self.error(line, "JMP-12", "return with a value in a constructor or destructor");
            }
            return Ok(Stmt::Return(None));
        }
        if f.deduce {
            // return type deduction (spec/declarations/functions/return-deduction.md)
            if let Some(x) = &e {
                if matches!(x.k, EK::InitList(_)) {
                    self.error(line, "RET-1", "return type deduced from a braced list");
                    return Ok(Stmt::Return(None));
                }
                let mut x0 = x.clone();
                if let EK::Var(id) = &x0.k {
                    if let Some(v) = self.var(*id) {
                        if v.local && v.storage == Storage::Auto && (!v.ty.t.is_ref() || matches!(v.ty.t, Ty::RRef(_))) {
                            // JMP-10, RET-6: a returned local name is an xvalue.
                            x0.cat = Cat::X;
                        }
                    }
                }
                let ra = self.fns.last().unwrap().ret_auto.clone();
                let is_void = x0.ty.t == Ty::Void;
                let t = if is_void { QTy::new(Ty::Void) } else { match ra {
                    Some((ds, d)) => {
                        let nerr = self.d.list.len();
                        let r = self.deduce_auto(&ds, &d, &QTy::unknown(), &x0, &InitForm::Copy, line);
                        // errors from deduction are about the return
                        let _ = nerr;
                        r.unwrap_or(QTy::unknown())
                    }
                    None => {
                        let t = nonref(&x.ty);
                        match &t.t {
                            Ty::Array(el, _) => QTy::new(Ty::Ptr(el.clone())),
                            Ty::Func(_) => QTy::new(Ty::Ptr(Box::new(t.clone()))),
                            _ => t.unqual(),
                        }
                    }
                } };
                let in_discarded = self.discarded > 0 || self.in_template();
                if !in_discarded {
                    let fl = self.fns.last_mut().unwrap();
                    match &fl.deduced {
                        None => fl.deduced = Some(t.clone()),
                        Some(prev) => {
                            if *prev != t && !prev.is_unknown() && !t.is_unknown() {
                                self.error(line, "RET-1", "return statements deduce different types");
                            }
                        }
                    }
                }
                let x = e.unwrap();
                let x = if t.is_unknown() {
                    x
                } else if t.t.is_ref() {
                    let mut x = x;
                    x.cat = x0.cat;
                    self.bind_ref(x, &t, line, "RET-1")
                } else {
                    self.implicit(x, &t, false).unwrap_or_else(|_| Expr::unknown(line, "return"))
                };
                return Ok(Stmt::Return(Some(x)));
            } else {
                if self.discarded == 0 {
                    let bad = self.fns.last().unwrap().deduced.as_ref().is_some_and(|t| t.t != Ty::Void && !t.is_unknown());
                    if bad {
                        self.error(line, "RET-1", "return without a value after a return with a value");
                    }
                    let fl = self.fns.last_mut().unwrap();
                    if fl.deduced.is_none() {
                        fl.deduced = Some(QTy::new(Ty::Void));
                    }
                }
                return Ok(Stmt::Return(None));
            }
        }
        let ret = f.ret.clone().unwrap_or(QTy::unknown());
        if ret.is_unknown() {
            return Ok(Stmt::Return(e));
        }
        if ret.t == Ty::Void {
            if let Some(x) = &e {
                if x.ty.t != Ty::Void && !x.ty.is_unknown() {
                    // JMP-3: a void function returns no value.
                    self.error(line, "JMP-3", "return with a value in a void function");
                }
            }
            return Ok(Stmt::Return(e));
        }
        let Some(x) = e else {
            if !self.fns.last().and_then(|f| f.func).is_some_and(|fid| self.func(fid).is_some_and(|fi| fi.is_main)) {
                // JMP-3: a function that returns a value needs an expression.
                self.error(line, "JMP-3", "return without a value in a function that returns a value");
            }
            return Ok(Stmt::Return(None));
        };
        // JMP-10: a local variable or parameter named in a return is an xvalue.
        let mut x = x;
        let strip = |e: &Expr| -> Option<DeclId> {
            if let EK::Var(id) = &e.k {
                Some(*id)
            } else {
                None
            }
        };
        if let Some(id) = strip(&x) {
            if let Some(v) = self.var(id) {
                let implicit_move = v.local && v.storage == Storage::Auto && (!v.ty.t.is_ref() || matches!(v.ty.t, Ty::RRef(_)));
                if implicit_move {
                    x.cat = Cat::X;
                }
            }
        }
        let form = if matches!(x.k, EK::InitList(_)) { InitForm::CopyList } else { InitForm::Copy };
        let c = if ret.t.is_ref() {
            self.bind_ref(x, &ret, line, "JMP-3")
        } else {
            self.convert_init(x, &ret, form, line)
        };
        Ok(Stmt::Return(Some(c)))
    }

    /// A substatement (branch or loop body) is its own scope (IF-8,
    /// LOOP-4). Names in `conflict` must not be declared again at its top
    /// level (IF-10, LOOP-4, SW-4).
    fn parse_substatement(&mut self, conflict: Option<(usize, &'static str)>) -> PR<Stmt> {
        let s = self.push_scope(SK::Block);
        if let Some(c) = conflict {
            self.conflicts.insert(s, c);
        }
        let r = if self.at("{") {
            self.parse_compound_in(s)
        } else {
            self.parse_statement()
        };
        self.pop_scope();
        r
    }

    pub fn check_condition_conflict(&mut self, name: &str, line: u32) {
        let cur = self.decl_scope();
        if let Some(&(cs, rule)) = self.conflicts.get(&cur) {
            if self.scopes[cs].names.get(name).is_some_and(|v| !v.is_empty()) {
                self.error(line, rule, format!("'{}' redeclares a name from the condition or init-statement", name));
            }
        }
    }

    /// Does a `;` occur at the top level before the closing parenthesis?
    fn has_init_statement(&self) -> bool {
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
                    ";" if depth == 0 => return true,
                    _ => {}
                }
            }
            k += 1;
        }
    }

    fn parse_init_statement(&mut self, rule: &'static str) -> PR<Stmt> {
        let line = self.line();
        if self.at(";") {
            self.pos += 1;
            return Ok(Stmt::Null);
        }
        if self.at("static_assert") {
            // IF-21, SA-2: not an init-statement.
            self.error(line, rule, "static_assert as an init-statement");
            self.parse_static_assert()?;
            return Ok(Stmt::Null);
        }
        if self.at("using") {
            let r = self.parse_declaration(Ctx::Init)?;
            return Ok(r.unwrap_or(Stmt::Null));
        }
        if self.is_decl_start() {
            if self.at("namespace") || self.at("template") {
                self.error(line, rule, "declaration of this kind as an init-statement");
            }
            let r = self.parse_declaration(Ctx::Init)?;
            return Ok(r.unwrap_or(Stmt::Null));
        }
        let e = self.parse_expression()?;
        self.expect(";")?;
        Ok(Stmt::Expr(e))
    }

    /// A condition: expression or declaration (IF-5, IF-6).
    fn parse_condition(&mut self, rule: &'static str) -> PR<CondK> {
        let line = self.line();
        if self.is_decl_start() {
            let ds = self.parse_decl_specs(None)?;
            // IF-6: specifiers
            if !ds.storage.is_empty() || ds.inline || ds.typedef || ds.register || ds.constinit {
                self.error(line, "IF-6", "storage class or other specifier in a condition declaration");
            }
            if ds.defined_class.is_some() || ds.defined_enum.is_some() {
                self.error(line, "IF-6", "class or enumeration defined in a condition");
            }
            if self.at("[") || (self.at("&") || self.at("&&")) && self.at_k(1, "[") {
                // structured binding in a condition
                self.error(line, "IF-6", "structured binding in a condition");
                while !self.at(")") && self.tok().kind != TK::Eof {
                    if self.at("(") || self.at("[") || self.at("{") {
                        self.skip_balanced();
                    } else {
                        self.pos += 1;
                    }
                }
                return Ok(CondK::Expr(Expr::unknown(line, "bad condition")));
            }
            let d = self.parse_declarator(false, false)?;
            if d.func().is_some() {
                self.error(line, "IF-6", "function declared in a condition");
            }
            if d.ops.iter().any(|o| matches!(o, crate::decl::DOp::Array(_))) {
                self.error(line, "IF-6", "array declared in a condition");
            }
            if !(self.at("=") || self.at("{")) {
                // IF-6: a condition declaration needs an initializer.
                self.error(line, "IF-6", "condition declaration without an initializer");
                if self.at("(") {
                    self.skip_balanced();
                }
            }
            let mut auto_ty = None;
            let (id, _) = self.declare(&ds, &d, Ctx::Block, true, &mut auto_ty)?;
            if self.at(",") {
                self.error(line, "IF-6", "condition declares more than one variable");
                while !self.at(")") && self.tok().kind != TK::Eof {
                    self.pos += 1;
                }
            }
            let Some(id) = id else {
                return Ok(CondK::Expr(Expr::unknown(line, "condition")));
            };
            let (ty, ok) = match self.var(id) {
                Some(v) => (v.ty.clone(), true),
                None => (QTy::unknown(), false),
            };
            if !ok {
                return Ok(CondK::Expr(Expr::unknown(line, "condition")));
            }
            let ve = Expr::new(EK::Var(id), nonref(&ty), Cat::L, line);
            let test = if rule == "SW-2" { self.rvalue(ve) } else { self.contextual_bool(ve, rule) };
            return Ok(CondK::Decl(id, test));
        }
        let e = self.parse_expression()?;
        if rule == "SW-2" {
            return Ok(CondK::Expr(e));
        }
        let e = self.contextual_bool(e, rule);
        Ok(CondK::Expr(e))
    }

    fn parse_if(&mut self) -> PR<Stmt> {
        let line = self.line();
        self.expect("if")?;
        // if consteval (IF-18, IF-19, IF-20)
        let neg = self.at("!") && self.at_k(1, "consteval");
        if self.at("consteval") || neg {
            if neg {
                self.pos += 1;
            }
            self.pos += 1;
            if self.at("(") {
                // IF-20: no condition after consteval.
                self.error(line, "IF-20", "'if consteval' takes no condition");
                self.skip_balanced();
            }
            if !self.at("{") {
                self.error(line, "IF-20", "the branches of 'if consteval' must be compound statements");
            }
            // The branch run in constant evaluation is an immediate
            // function context (IF-19).
            self.cx_counter += 1;
            self.cx_branches.push(self.cx_counter);
            let then = self.with_immediate(!neg, |p| p.parse_substatement(None));
            self.cx_branches.pop();
            let then = then?;
            let mut els = None;
            if self.eat("else") {
                if !self.at("{") {
                    self.error(line, "IF-20", "the else branch of 'if consteval' must be a compound statement");
                }
                self.cx_counter += 1;
                self.cx_branches.push(self.cx_counter);
                let e = self.with_immediate(neg, |p| p.parse_substatement(None));
                self.cx_branches.pop();
                els = Some(Box::new(e?));
            }
            return Ok(Stmt::If { init: None, cond: None, then: Box::new(then), els, kind: IfKind::Consteval(neg) });
        }
        if self.at("!") {
            self.error(line, "IF-20", "'!' after 'if' must be followed by 'consteval'");
            self.pos += 1;
        }
        let is_constexpr = self.eat("constexpr");
        self.expect("(")?;
        let cs = self.push_scope(SK::Block);
        let r = (|| -> PR<Stmt> {
            let init = if self.has_init_statement() { Some(Box::new(self.parse_init_statement("IF-21")?)) } else { None };
            let cond = self.parse_condition("IF-4")?;
            self.expect(")")?;
            let mut selected = None;
            if is_constexpr {
                // IF-16: the condition is a constant expression.
                let ce = match &cond {
                    CondK::Expr(e) => e.clone(),
                    CondK::Decl(_, e) => e.clone(),
                };
                if !self.in_template() {
                    match self.const_eval_bool(&ce) {
                        Ok(b) => selected = Some(b),
                        Err(crate::consteval::CeErr::NotConst(r, m)) => self.error(line, "IF-16", format!("if constexpr condition is not a constant expression ({}: {})", r, m)),
                        Err(crate::consteval::CeErr::Unsupported(m)) => self.unsupported(line, format!("if constexpr: {}", m)),
                    }
                }
            }
            let disc_then = selected == Some(false);
            if disc_then {
                self.discarded += 1;
            }
            if is_constexpr {
                self.cx_counter += 1;
                self.cx_branches.push(self.cx_counter);
            }
            let then = self.parse_substatement(Some((cs, "IF-10")));
            if is_constexpr {
                self.cx_branches.pop();
            }
            if disc_then {
                self.discarded -= 1;
            }
            let then = then?;
            let mut els = None;
            if self.eat("else") {
                let disc_else = selected == Some(true);
                if disc_else {
                    self.discarded += 1;
                }
                if is_constexpr {
                    self.cx_counter += 1;
                    self.cx_branches.push(self.cx_counter);
                }
                let e = self.parse_substatement(Some((cs, "IF-10")));
                if is_constexpr {
                    self.cx_branches.pop();
                }
                if disc_else {
                    self.discarded -= 1;
                }
                els = Some(Box::new(e?));
            }
            Ok(Stmt::If { init, cond: Some(cond), then: Box::new(then), els, kind: if is_constexpr { IfKind::Constexpr } else { IfKind::Normal } })
        })();
        self.pop_scope();
        r
    }

    fn with_immediate<T>(&mut self, imm: bool, f: impl FnOnce(&mut P) -> T) -> T {
        if let Some(fc) = self.fns.last_mut() {
            let save = fc.consteval_ctx;
            fc.consteval_ctx |= imm;
            let r = f(self);
            if let Some(fc) = self.fns.last_mut() {
                fc.consteval_ctx = save;
            }
            r
        } else {
            f(self)
        }
    }

    fn parse_switch(&mut self) -> PR<Stmt> {
        let line = self.line();
        self.expect("switch")?;
        self.expect("(")?;
        let cs = self.push_scope(SK::Block);
        let r = (|| -> PR<Stmt> {
            let init = if self.has_init_statement() { Some(Box::new(self.parse_init_statement("IF-21")?)) } else { None };
            let cond = self.parse_condition("SW-2")?;
            self.expect(")")?;
            // SW-2: integer, enumeration, or a class with one conversion.
            let (ce, decl) = match cond {
                CondK::Expr(e) => (e, None),
                CondK::Decl(id, e) => (e, Some(id)),
            };
            let ct = nonref(&ce.ty);
            let mut cond_ty = None;
            let ce = if ct.is_unknown() {
                ce
            } else if ct.t.is_integral() || matches!(ct.t, Ty::Enum(_)) {
                let p = if self.is_scoped_enum(&ct.t) { ct.t.clone() } else { self.promoted(&ct.t) };
                cond_ty = Some(if self.is_scoped_enum(&ct.t) { QTy::new(ct.t.clone()) } else { QTy::new(p.clone()) });
                let e = self.rvalue(ce);
                if self.is_scoped_enum(&ct.t) {
                    e
                } else {
                    self.conv(e, Conv::Arith, QTy::new(p))
                }
            } else if let Ty::Class(c) = ct.t {
                let targets: Vec<QTy> = self.classes[c].conv_targets.iter().filter(|t| t.t.is_integral() || matches!(t.t, Ty::Enum(_))).cloned().collect();
                let has_explicit = self.class_has_only_explicit_conv(c);
                match self.class_converts(c) {
                    Some(false) => self.error(line, "SW-2", "switch condition of class type without a conversion to an integer"),
                    Some(true) if !self.class_uncertain(c) && self.classes[c].bases.is_empty() => {
                        let mut uniq: Vec<Ty> = Vec::new();
                        for t in &targets {
                            if !uniq.contains(&t.t) {
                                uniq.push(t.t.clone());
                            }
                        }
                        if has_explicit && uniq.len() <= 1 {
                            self.error(line, "SW-2", "switch condition needs an explicit conversion");
                        } else if uniq.len() > 1 {
                            self.error(line, "SW-2", "switch condition class converts to several integer types");
                        } else if uniq.len() == 1 {
                            cond_ty = Some(QTy::new(if self.is_scoped_enum(&uniq[0]) { uniq[0].clone() } else { self.promoted(&uniq[0]) }));
                        }
                    }
                    _ => {}
                }
                Expr::unknown(line, "class switch condition")
            } else {
                self.error(line, "SW-2", format!("switch condition must have integer or enumeration type, not {}", show(&ct)));
                Expr::unknown(line, "bad switch condition")
            };
            let start_set = self.jump_set();
            let fc = self.fns.last_mut();
            let Some(fc) = fc else {
                return Err(PErr { line, msg: "switch outside a function".into() });
            };
            fc.switches.push(SwitchCtx { cond_ty, start_set, ..Default::default() });
            fc.breakable += 1;
            let body = self.parse_substatement(Some((cs, "SW-4")));
            let fc = self.fns.last_mut().unwrap();
            fc.switches.pop();
            fc.breakable -= 1;
            let body = body?;
            let cond = match decl {
                Some(id) => CondK::Decl(id, ce),
                None => CondK::Expr(ce),
            };
            Ok(Stmt::Switch { init, cond, body: Box::new(body) })
        })();
        self.pop_scope();
        r
    }

    fn class_has_only_explicit_conv(&self, c: usize) -> bool {
        let sc = self.classes[c].scope;
        let mut any = false;
        let mut all_explicit = true;
        for ids in self.scopes[sc].names.values() {
            for &id in ids {
                if let Some(f) = self.func(id) {
                    if f.kind == FnKind::Conversion {
                        any = true;
                        if !f.explicit {
                            all_explicit = false;
                        }
                    }
                }
            }
        }
        any && all_explicit
    }

    fn loop_body(&mut self, conflict: Option<(usize, &'static str)>) -> PR<Stmt> {
        if let Some(f) = self.fns.last_mut() {
            f.loops += 1;
            f.breakable += 1;
        }
        let r = self.parse_substatement(conflict);
        if let Some(f) = self.fns.last_mut() {
            f.loops -= 1;
            f.breakable -= 1;
        }
        r
    }

    fn parse_while(&mut self) -> PR<Stmt> {
        self.expect("while")?;
        self.expect("(")?;
        let cs = self.push_scope(SK::Block);
        let r = (|| -> PR<Stmt> {
            let cond = self.parse_condition("LOOP-5")?;
            self.expect(")")?;
            let body = self.loop_body(Some((cs, "LOOP-4")))?;
            Ok(Stmt::While { cond, body: Box::new(body) })
        })();
        self.pop_scope();
        r
    }

    fn parse_do(&mut self) -> PR<Stmt> {
        let line = self.line();
        self.expect("do")?;
        let body = self.loop_body(None)?;
        self.expect("while")?;
        self.expect("(")?;
        if self.is_decl_start() {
            // LOOP-6: the condition of do is an expression.
            self.error(line, "LOOP-6", "the condition of 'do' must be an expression");
            while !self.at(")") && self.tok().kind != TK::Eof {
                if self.at("(") || self.at("{") {
                    self.skip_balanced();
                } else {
                    self.pos += 1;
                }
            }
            self.expect(")")?;
            self.expect(";")?;
            return Ok(Stmt::Unsupported("bad do".into()));
        }
        let e = self.parse_expression()?;
        let cond = self.contextual_bool(e, "LOOP-5");
        self.expect(")")?;
        self.expect(";")?;
        Ok(Stmt::Do { body: Box::new(body), cond })
    }

    fn is_range_for(&self) -> bool {
        // a top-level ':' (not '::') before the closing parenthesis and
        // before any top-level ';'
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
                    ";" if depth == 0 => return false,
                    ":" if depth == 0 => return true,
                    "?" if depth == 0 => return false,
                    _ => {}
                }
            }
            k += 1;
        }
    }

    fn parse_for(&mut self) -> PR<Stmt> {
        let line = self.line();
        self.expect("for")?;
        self.expect("(")?;
        let cs = self.push_scope(SK::Block);
        let r = (|| -> PR<Stmt> {
            // range-for, possibly after an init-statement
            let mut init = None;
            if !self.is_range_for() && self.has_init_statement() {
                let save = self.pos;
                // `for (init; decl : range)`: init-statement then range
                let st = self.parse_init_statement("LOOP-7")?;
                if self.is_range_for() {
                    init = Some(Box::new(st));
                    return self.parse_range_for(cs, init, line);
                }
                let _ = save;
                init = Some(Box::new(st));
            } else if self.is_range_for() {
                return self.parse_range_for(cs, None, line);
            }
            // condition
            let cond = if self.at(";") {
                None
            } else {
                // LOOP-7: the condition must not redeclare an init name; a
                // separate scope over the same declarations
                Some(self.parse_condition_for(cs)?)
            };
            self.expect(";")?;
            let inc = if self.at(")") { None } else { Some(self.parse_expression()?) };
            self.expect(")")?;
            let body = self.loop_body(Some((cs, "LOOP-4")))?;
            Ok(Stmt::For { init, cond, inc, body: Box::new(body) })
        })();
        self.pop_scope();
        r
    }

    fn parse_condition_for(&mut self, cs: usize) -> PR<CondK> {
        if self.is_decl_start() {
            // check the declared name against the init-statement names
            let save = self.pos;
            let ds = self.parse_decl_specs(None);
            if ds.is_ok() {
                if let Ok(d) = self.parse_declarator(false, false) {
                    if let Some(n) = d.name {
                        if self.scopes[cs].names.get(&n).is_some_and(|v| !v.is_empty()) {
                            self.error(d.line, "LOOP-7", format!("for condition redeclares '{}' from the init-statement", n));
                            // drop the old entry so that declare() does not report again
                            self.scopes[cs].names.remove(&n);
                        }
                    }
                }
            }
            self.pos = save;
        }
        self.parse_condition("LOOP-5")
    }

    fn parse_range_for(&mut self, cs: usize, init: Option<Box<Stmt>>, line: u32) -> PR<Stmt> {
        let _ = init;
        // for_range_decl : for_range_init
        let ds = self.parse_decl_specs(None)?;
        if !ds.storage.is_empty() || ds.register {
            // RFOR-5: no static, thread_local, or extern loop variable (Q-0026).
            self.error(line, "RFOR-5", "storage class on a range-for declaration");
        }
        let mut names = Vec::new();
        let mut decl = None;
        if self.at("[") || ((self.at("&") || self.at("&&")) && self.at_k(1, "[")) {
            self.eat("&");
            self.eat("&&");
            self.expect("[")?;
            loop {
                names.push(self.ident()?);
                if !self.eat(",") {
                    break;
                }
            }
            self.expect("]")?;
        } else {
            decl = Some(self.parse_declarator(false, false)?);
        }
        self.expect(":")?;
        let range = if self.at("{") { self.parse_braced_init()? } else { self.parse_expression()? };
        self.expect(")")?;
        // the loop variable's type
        let rt = nonref(&range.ty);
        let elem = match &rt.t {
            Ty::Array(e, _) => {
                let mut e = (**e).clone();
                e.c |= rt.c;
                Some(e)
            }
            _ => None,
        };
        if !rt.is_unknown() && self.is_scalar(&rt) {
            self.error(line, "RFOR-2", "range-for over a value that is not a range");
        }
        let s = self.cur_scope();
        if let Some(d) = decl {
            let base = match (&ds.ty, ds.auto) {
                (Some(t), _) => t.clone(),
                _ => QTy::unknown(),
            };
            let mut ty = self.apply_ops(base, &d.ops, line);
            if ds.auto.is_some() {
                ty = match &elem {
                    Some(e) => {
                        let lv = Expr::new(EK::Unknown("element".into()), e.clone(), Cat::L, line);
                        let fk = crate::decl2::InitForm::Copy;
                        self.deduce_auto(&ds, &d, &ty, &lv, &fk, line).unwrap_or(QTy::unknown())
                    }
                    None => QTy::unknown(),
                };
            }
            let name = d.name.clone().unwrap_or_default();
            let vi = VarInfo {
                ty,
                storage: if ds.storage.contains(&Storage::Static) { Storage::Static } else { Storage::Auto },
                constexpr: false,
                constinit: false,
                init: None,
                param: false,
                local: true,
                dependent: false,
                defined: true,
                class: None,
            };
            if self.scopes[s].names.get(&name).is_some_and(|v| !v.is_empty()) {
                self.error(line, "NAM-3", format!("'{}' declared twice", name));
            }
            self.add_decl(&name, DK::Var(vi), line);
        } else {
            for n in names {
                let vi = VarInfo { ty: QTy::unknown(), storage: Storage::Auto, constexpr: false, constinit: false, init: None, param: false, local: true, dependent: false, defined: true, class: None };
                self.add_decl(&n, DK::Var(vi), line);
            }
        }
        let body = self.loop_body(Some((cs, "RFOR-6")))?;
        let _ = body;
        Ok(Stmt::Unsupported("range-based for".into()))
    }
}
