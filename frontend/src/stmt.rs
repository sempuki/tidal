// Statements: specification/statements/*.md.

use crate::ast::*;
use crate::decl2::{Ctx, InitForm};
use crate::lexer::TK;
use crate::parser::*;
use crate::types::*;
use std::collections::HashSet;
use std::rc::Rc;

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

    /// Check the attributes of a statement (ATTR-2, ATTR-13). Returns the
    /// statement when the attributes make it an attribute statement such
    /// as `[[fallthrough]];` or `[[assume(e)]];`.
    fn statement_attrs(&mut self, attrs: Vec<Attr>) -> PR<Option<Stmt>> {
        let null = self.at(";");
        let mut stmt = None;
        if attrs.iter().any(|a| a.name == "likely") && attrs.iter().any(|a| a.name == "unlikely") {
            // ATTR-2: not both likely and unlikely.
            self.error(attrs[0].line, "ATTR-2", "'likely' and 'unlikely' on the same statement");
        }
        for a in &attrs {
            match a.name.as_str() {
                "fallthrough" if !null => self.error(a.line, "ATTR-2", "'fallthrough' must be a statement on its own"),
                "fallthrough" => {
                    // ATTR-2: a statement in a switch. Where it may stand
                    // is checked when the switch ends.
                    let in_switch = self.fns.last().is_some_and(|f| !f.switches.is_empty());
                    if !in_switch {
                        self.error(a.line, "ATTR-2", "'fallthrough' outside a switch statement");
                        stmt = Some(Stmt::Null);
                    } else {
                        stmt = Some(Stmt::Fallthrough(a.line));
                    }
                }
                "assume" if !null => self.error(a.line, "ATTR-13", "an assumption must be a statement on its own"),
                "assume" => {
                    stmt = Some(match &a.assume {
                        Some(e) => Stmt::Assume(e.clone()),
                        None => Stmt::Null,
                    });
                }
                _ => {}
            }
        }
        if stmt.is_some() {
            self.pos += 1; // ;
            return Ok(stmt);
        }
        self.pending_attrs = attrs;
        Ok(None)
    }

    pub fn parse_statement_recover(&mut self) -> Stmt {
        let start = self.pos;
        match self.parse_statement() {
            Ok(s) => s,
            Err(e) => {
                self.parse_failure(&e);
                let s = self.decl_scope();
                self.scopes[s].uncertain = true;
                if self.pos == start {
                    self.pos += 1;
                }
                self.recover();
                if let Some(f) = self.fns.last().and_then(|f| f.func) {
                    if let Some(fi) = self.func_mut(f) {
                        fi.opaque = true;
                        fi.parse_failed = true;
                    }
                }
                Stmt::Unsupported(e.msg)
            }
        }
    }

    pub fn parse_statement(&mut self) -> PR<Stmt> {
        let mark = self.pack_uses.len();
        let r = self.parse_statement_inner();
        self.check_pack_uses(mark);
        r
    }

    fn parse_statement_inner(&mut self) -> PR<Stmt> {
        self.pending_attrs.clear();
        self.skip_attributes();
        let attrs = std::mem::take(&mut self.pending_attrs);
        if !attrs.is_empty() {
            if let Some(s) = self.statement_attrs(attrs)? {
                return Ok(s);
            }
        }
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
                    self.enter_try_region();
                    let mark = self.decls.len();
                    if let Some(f) = self.fns.last_mut() {
                        f.try_marks.push(mark);
                    }
                    let body = self.parse_compound();
                    if let Some(f) = self.fns.last_mut() {
                        f.try_marks.pop();
                    }
                    self.try_regions.pop();
                    let body = body?;
                    let hs = self.parse_handlers()?;
                    return Ok(Stmt::Try(Box::new(body), hs));
                }
                "co_return" => return Err(self.err("coroutines (outside the specification)")),
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
        self.check_unsequenced(&e);
        self.check_volatile_assign(&e, false);
        // DTOR-3: a class prvalue here is a temporary, which is destroyed.
        if e.cat == Cat::PR && !self.in_template() {
            if let Ty::Class(c) = e.ty.t {
                if self.dtor_unusable(c) {
                    self.error(line, "DTOR-3", format!("temporary of '{}', whose destructor is deleted or not accessible", self.classes[c].name));
                }
            }
        }
        Ok(Stmt::Expr(e))
    }

    /// The statement after a label; a label can end a block (STMT-5).
    fn parse_label_target(&mut self) -> PR<Stmt> {
        if self.at("}") {
            return Ok(Stmt::Null);
        }
        self.parse_statement()
    }

    pub fn enter_try_region(&mut self) {
        self.try_region_next += 1;
        self.try_regions.push(self.try_region_next);
    }

    fn define_label(&mut self, name: &str, line: u32) {
        let set = self.jump_set();
        let Some(f) = self.fns.last_mut() else {
            return;
        };
        if f.labels.contains_key(name) {
            // JMP-5, NAM-7: a label once per function.
            self.error(line, "JMP-14", format!("label '{}' defined twice", name));
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
        // TRY-7: a marker for each enclosing try block or handler
        for &r in &self.try_regions {
            s.insert(usize::MAX / 4 + r);
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
                } else if missing.iter().any(|&&v| v > usize::MAX / 4) {
                    self.error(*gl, "TRY-7", format!("goto '{}' jumps into a try block or a handler", name));
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
        } else if missing.iter().any(|&&v| v > usize::MAX / 4) {
            self.error(line, "TRY-7", "case or default label inside a try block or a handler that the switch encloses");
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
                    if s.iter().any(|v| !start.contains(v) && *v > usize::MAX / 4 && *v <= usize::MAX / 2) {
                        self.error(line, "TRY-7", "case label inside a try block or a handler that the switch encloses");
                    } else if s.iter().any(|v| !start.contains(v)) {
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
            self.error(line, "CE-25", "case value must have integer or enumeration type");
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
                    self.error_because(line, "SW-3", r, format!("case value is not a constant expression ({}: {})", r, m));
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
                EK::Call(..) | EK::Construct(..) | EK::Assign(..) | EK::Throw(_) | EK::Unknown(_) | EK::Reinterpret(..) | EK::Delete(..) | EK::New { .. } | EK::Destroy(_) => true,
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
        // ATTR-5: the result of a nodiscard call is discarded.
        if let EK::Call(_, _, Some(f)) = &e.k {
            let by_value_class = matches!(&e.ty.t, Ty::Class(c) if e.cat == Cat::PR && self.class_attrs.get(c).is_some_and(|v| v.iter().any(|n| n == "nodiscard")));
            if self.has_attr(*f, "nodiscard") || by_value_class {
                self.warn(e.line, "ATTR-5", "result of a 'nodiscard' call discarded");
            }
        }
        // ATTR-5: a temporary that a nodiscard constructor creates.
        if let EK::Construct(cid, _) = &e.k {
            let class_nd = matches!(&e.ty.t, Ty::Class(c) if self.class_attrs.get(c).is_some_and(|v| v.iter().any(|n| n == "nodiscard")));
            if self.has_attr(*cid, "nodiscard") || class_nd {
                self.warn(e.line, "ATTR-5", "temporary from a 'nodiscard' constructor discarded");
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
        if let Some(x) = &e {
            self.check_prvalue_dtor(x);
        }
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
        if let (Some(x), Some(fid), false) = (&e, f.func, f.is_lambda) {
            if matches!(&x.k, EK::Unknown(w) if w == "lambda with reference captures") || matches!(&x.k, EK::Closure(_, _, true)) {
                // LAM-19: the closure outlives the variables it refers to.
                self.ref_lambda_fns.insert(fid);
            }
        }
        if f.deduce {
            // return type deduction (specification/declarations/functions/return-deduction.md)
            if let Some(x) = &e {
                if matches!(x.k, EK::InitList(_)) {
                    self.error(line, "RET-7", "return type deduced from a braced list");
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
                                self.error(line, "RET-7", "return statements deduce different types");
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
                    self.bind_ref(x, &t, line, "RET-7")
                } else {
                    self.implicit(x, &t, false).unwrap_or_else(|_| Expr::unknown(line, "return"))
                };
                return Ok(Stmt::Return(Some(x)));
            } else {
                if self.discarded == 0 {
                    let bad = self.fns.last().unwrap().deduced.as_ref().is_some_and(|t| t.t != Ty::Void && !t.is_unknown());
                    if bad {
                        self.error(line, "RET-7", "return without a value after a return with a value");
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
                    self.error(line, "JMP-13", "return with a value in a void function");
                }
            }
            return Ok(Stmt::Return(e));
        }
        let Some(x) = e else {
            if !self.fns.last().and_then(|f| f.func).is_some_and(|fid| self.func(fid).is_some_and(|fi| fi.is_main)) {
                // JMP-3: a function that returns a value needs an expression.
                self.error(line, "JMP-13", "return without a value in a function that returns a value");
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
        self.check_unsequenced(&x);
        self.check_volatile_assign(&x, true);
        // CPY-13: a cast to an rvalue that prevents the local variable from
        // being the return value.
        if self.wflag(&["-Wall", "-Wpessimizing-move"]) && !self.in_template() {
            if let EK::Call(_, args, Some(fid)) = &x.k {
                if self.decls[*fid].name == "move" && args.len() == 1 && matches!(ret.t, Ty::Class(_)) {
                    let mut a = &args[0];
                    while let EK::Conv(_, i) = &a.k {
                        a = i;
                    }
                    if let EK::Var(id) = &a.k {
                        if self.var(*id).is_some_and(|v| v.local && !v.param && v.storage == Storage::Auto && v.ty.t == ret.t) {
                            self.warn(line, "CPY-13", "a cast to an rvalue in a return prevents copy elision");
                        }
                    }
                }
            }
        }
        let form = if matches!(x.k, EK::InitList(_)) { InitForm::CopyList } else { InitForm::Copy };
        let c = if ret.t.is_ref() {
            self.bind_ref(x, &ret, line, "JMP-13")
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
                        Err(crate::consteval::CeErr::NotConst(r, m)) => self.error_because(line, "IF-16", r, format!("if constexpr condition is not a constant expression ({}: {})", r, m)),
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
            // IF-13: in a specialization, a discarded statement is not
            // instantiated.
            let saved = if disc_then && self.inst_depth > 0 { Some(self.begin_discarded()) } else { None };
            let then = self.parse_substatement(Some((cs, "IF-10")));
            if let Some(sv) = saved {
                self.end_discarded(sv);
            }
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
                let saved = if disc_else && self.inst_depth > 0 { Some(self.begin_discarded()) } else { None };
                let e = self.parse_substatement(Some((cs, "IF-10")));
                if let Some(sv) = saved {
                    self.end_discarded(sv);
                }
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
                let mut target: Option<Ty> = None;
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
                            target = Some(uniq[0].clone());
                        }
                    }
                    _ => {}
                }
                // SW-2, CVF-4: the conversion function that overload
                // resolution chooses, then the promotion.
                let mut out = Expr::unknown(line, "class switch condition");
                if let Some(t) = target {
                    if !self.in_template() {
                        let mark = self.d.list.len();
                        let x = self.user_conv(&ce, &QTy::new(t.clone()), line);
                        match x {
                            Some(x) if self.d.list.len() == mark && !matches!(x.k, EK::Unknown(_)) => {
                                let x = self.rvalue(x);
                                out = if self.is_scoped_enum(&t) { x } else { let p = self.promoted(&t); self.conv(x, Conv::Arith, QTy::new(p)) };
                            }
                            _ => self.d.list.truncate(mark),
                        }
                    }
                }
                out
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
            let body = self.parse_substatement(Some((cs, "SW-6")));
            let fc = self.fns.last_mut().unwrap();
            fc.switches.pop();
            fc.breakable -= 1;
            let body = body?;
            self.check_fallthrough(&body, false);
            if self.wflag(&["-Wimplicit-fallthrough", "-Wextra"]) && !self.in_template() {
                self.check_implicit_fallthrough(&body);
            }
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
            let body = self.loop_body(Some((cs, "LOOP-8")))?;
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
            let body = self.loop_body(Some((cs, "LOOP-8")))?;
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
        let mut sb_ref: Option<bool> = None;
        if self.at("[") || ((self.at("&") || self.at("&&")) && self.at_k(1, "[")) {
            if self.eat("&") {
                sb_ref = Some(true);
            }
            if self.eat("&&") {
                sb_ref = Some(false);
            }
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
            self.error(line, "RFOR-7", "range-for over a value that is not a range");
        }
        let s = self.cur_scope();
        // RFOR-1, RFOR-2: the loop as a block with a reference to the
        // range, an iterator and its end, for the interpreter. A step that
        // raises a diagnostic leaves the statement unsupported, as before.
        let lowered = if (decl.is_some() || ds.auto == Some(false)) && !self.in_template() { self.lower_range_for(&range, s, line) } else { None };
        let mut loop_var = None;
        if let Some(d) = decl {
            let base = match (&ds.ty, ds.auto) {
                (Some(t), _) => t.clone(),
                _ => QTy::unknown(),
            };
            let mut ty = self.apply_ops(base, &d.ops, line);
            if ds.auto.is_some() {
                ty = match (&elem, &lowered) {
                    (_, Some(lw)) => {
                        let fk = crate::decl2::InitForm::Copy;
                        let x = lw.3.clone();
                        let mark = self.d.list.len();
                        let t = self.deduce_auto(&ds, &d, &ty, &x, &fk, line).unwrap_or(QTy::unknown());
                        if self.d.list.len() != mark {
                            self.d.list.truncate(mark);
                        }
                        t
                    }
                    (Some(e), None) => {
                        let lv = Expr::new(EK::Unknown("element".into()), e.clone(), Cat::L, line);
                        let fk = crate::decl2::InitForm::Copy;
                        self.deduce_auto(&ds, &d, &ty, &lv, &fk, line).unwrap_or(QTy::unknown())
                    }
                    (None, None) => QTy::unknown(),
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
            let vty = vi.ty.clone();
            let id = self.add_decl(&name, DK::Var(vi), line);
            // The loop variable from `*__begin`.
            if let Some(lw) = &lowered {
                if !vty.is_unknown() {
                    let mark = self.d.list.len();
                    let init = self.convert_init(lw.3.clone(), &vty, crate::decl2::InitForm::Copy, line);
                    if self.d.list.len() == mark && !matches!(init.k, EK::Unknown(_)) {
                        if let DK::Var(v) = &mut self.decls[id].k {
                            v.init = Some(Rc::new(init));
                        }
                        loop_var = Some(id);
                    } else {
                        self.d.list.truncate(mark);
                    }
                }
            }
        } else if let Some(lw) = &lowered {
            // SB-1, RFOR-3: the hidden variable from `*__begin`, and the
            // names that refer into it.
            let named: Vec<(String, u32)> = names.iter().map(|n| (n.clone(), line)).collect();
            let mark = self.d.list.len();
            let ids = self.bind_names(&ds, sb_ref, lw.3.clone(), &named, Ctx::Block, line, line)?;
            if self.d.list.len() == mark && ids.len() == names.len() + 1 {
                loop_var = Some(ids[0]);
            }
        } else {
            for n in names {
                let vi = VarInfo { ty: QTy::unknown(), storage: Storage::Auto, constexpr: false, constinit: false, init: None, param: false, local: true, dependent: false, defined: true, class: None };
                self.add_decl(&n, DK::Var(vi), line);
            }
        }
        let body = self.loop_body(Some((cs, "RFOR-6")))?;
        if let (Some((range_var, begin_var, end_var, _, cond, inc)), Some(lv)) = (lowered, loop_var) {
            let decls = Stmt::Decl(vec![begin_var, end_var]);
            let body = Stmt::Block(vec![Stmt::Decl(vec![lv]), body]);
            let lp = Stmt::For { init: Some(Box::new(decls)), cond: Some(CondK::Expr(cond)), inc: Some(inc), body: Box::new(body) };
            let mut items = Vec::new();
            if let Some(i) = init {
                items.push(*i);
            }
            items.push(Stmt::Decl(vec![range_var]));
            items.push(lp);
            return Ok(Stmt::Block(items));
        }
        Ok(Stmt::Unsupported("range-based for".into()))
    }

    /// RFOR-1, RFOR-2: the variables `__range`, `__begin`, and `__end`, and
    /// the expressions `*__begin`, `__begin != __end`, and `++__begin`.
    /// None if a step is not modeled or raises a diagnostic.
    fn lower_range_for(&mut self, range: &Expr, s: usize, line: u32) -> Option<(DeclId, DeclId, DeclId, Expr, Expr, Expr)> {
        if range.ty.is_unknown() || matches!(range.k, EK::InitList(_) | EK::Unknown(_)) {
            return None;
        }
        let mark = self.d.list.len();
        let r = (|| -> Option<(DeclId, DeclId, DeclId, Expr, Expr, Expr)> {
            let rt = nonref(&range.ty);
            // auto&& __range = range;
            let rref = if range.cat == Cat::L { QTy::new(Ty::LRef(Box::new(rt.clone()))) } else { QTy::new(Ty::RRef(Box::new(rt.clone()))) };
            let rinit = self.convert_init(range.clone(), &rref, crate::decl2::InitForm::Copy, line);
            let local = |ty: QTy, init: Expr| VarInfo { ty, storage: Storage::Auto, constexpr: false, constinit: false, init: Some(Rc::new(init)), param: false, local: true, dependent: false, defined: true, class: None };
            let range_var = self.add_decl_in(s, " range", DK::Var(local(rref.clone(), rinit)), line);
            let rv = Expr::new(EK::Var(range_var), rt.clone(), Cat::L, line);
            let (b, e) = match &rt.t {
                Ty::Array(_, Some(n)) => {
                    let b = self.rvalue(rv.clone());
                    let e = self.build_binary(BOp::Add, b.clone(), Expr::new(EK::Int(*n as i128), QTy::new(Ty::size_t()), Cat::PR, line), line);
                    (b, e)
                }
                Ty::Class(c) => {
                    let c = *c;
                    let mb = self.lookup_member(c, "begin");
                    let me = self.lookup_member(c, "end");
                    if mb.is_some() || me.is_some() {
                        let fb: Vec<DeclId> = mb?.into_iter().filter(|&f| self.func(f).is_some()).collect();
                        let fe: Vec<DeclId> = me?.into_iter().filter(|&f| self.func(f).is_some()).collect();
                        let b = self.call_overloads_pub(Some(rv.clone()), fb, vec![], line);
                        let e = self.call_overloads_pub(Some(rv.clone()), fe, vec![], line);
                        (b, e)
                    } else {
                        let (fb, _) = self.adl_lookup("begin", std::slice::from_ref(&rv));
                        let (fe, _) = self.adl_lookup("end", std::slice::from_ref(&rv));
                        if fb.is_empty() || fe.is_empty() {
                            return None;
                        }
                        let b = self.call_overloads_pub(None, fb, vec![rv.clone()], line);
                        let e = self.call_overloads_pub(None, fe, vec![rv.clone()], line);
                        (b, e)
                    }
                }
                _ => return None,
            };
            if matches!(b.k, EK::Unknown(_)) || matches!(e.k, EK::Unknown(_)) || b.ty.is_unknown() || e.ty.is_unknown() {
                return None;
            }
            let bt = nonref(&b.ty).unqual();
            let et = nonref(&e.ty).unqual();
            let b = self.rvalue(b);
            let e = self.rvalue(e);
            let begin_var = self.add_decl_in(s, " begin", DK::Var(local(bt.clone(), b)), line);
            let end_var = self.add_decl_in(s, " end", DK::Var(local(et.clone(), e)), line);
            let bv = Expr::new(EK::Var(begin_var), bt, Cat::L, line);
            let ev = Expr::new(EK::Var(end_var), et, Cat::L, line);
            let cond = self.build_binary(BOp::Ne, bv.clone(), ev, line);
            let cond = self.contextual_bool(cond, "RFOR-1");
            let inc = self.build_unary(UOp::PreInc, bv.clone(), line);
            let deref = self.build_unary(UOp::Deref, bv, line);
            for x in [&cond, &inc, &deref] {
                if matches!(x.k, EK::Unknown(_)) || x.ty.is_unknown() {
                    return None;
                }
            }
            Some((range_var, begin_var, end_var, deref, cond, inc))
        })();
        if self.d.list.len() != mark || r.is_none() {
            self.d.list.truncate(mark);
            return None;
        }
        r
    }
}

/// Whether a statement begins with a `case` or `default` label.
fn starts_with_case(s: &Stmt) -> bool {
    match s {
        Stmt::Case(..) | Stmt::Default(_) => true,
        Stmt::Label(_, x) => starts_with_case(x),
        Stmt::Block(v) => v.first().is_some_and(starts_with_case),
        _ => false,
    }
}

impl P {
    /// ATTR-2: the next statement that can run after a `fallthrough` must
    /// have a `case` or `default` label. `to_case` says whether the
    /// statement after `s` has one.
    fn check_fallthrough(&mut self, s: &Stmt, to_case: bool) {
        match s {
            Stmt::Fallthrough(line) => {
                if !to_case {
                    self.error(*line, "ATTR-2", "'fallthrough' is not followed by a case or default label");
                }
            }
            Stmt::Block(v) => {
                for (i, x) in v.iter().enumerate() {
                    let next = match v.get(i + 1) {
                        Some(n) => starts_with_case(n),
                        None => to_case,
                    };
                    self.check_fallthrough(x, next);
                }
            }
            Stmt::If { then, els, .. } => {
                self.check_fallthrough(then, to_case);
                if let Some(e) = els {
                    self.check_fallthrough(e, to_case);
                }
            }
            // The end of a loop body goes back to the loop.
            Stmt::While { body, .. } | Stmt::Do { body, .. } | Stmt::For { body, .. } => self.check_fallthrough(body, false),
            Stmt::Case(_, x) | Stmt::Default(x) | Stmt::Label(_, x) => self.check_fallthrough(x, to_case),
            // A nested switch was checked when it ended.
            _ => {}
        }
    }
}

/// Whether control can leave a statement at its end into the next one.
/// A `[[fallthrough]];` at the end counts as an end that does not fall
/// through unannotated (ATTR-7).
fn falls_through(p: &P, s: &Stmt) -> bool {
    fn has_break(s: &Stmt) -> bool {
        match s {
            Stmt::Break => true,
            Stmt::Block(v) => v.iter().any(has_break),
            Stmt::If { then, els, .. } => has_break(then) || els.as_deref().is_some_and(has_break),
            Stmt::Case(_, x) | Stmt::Default(x) | Stmt::Label(_, x) => has_break(x),
            _ => false,
        }
    }
    match s {
        Stmt::Break | Stmt::Continue | Stmt::Return(_) | Stmt::Goto(_) | Stmt::Fallthrough(_) => false,
        Stmt::Unsupported(_) => false,
        Stmt::Block(v) => v.last().is_none_or(|l| falls_through(p, l)),
        Stmt::Expr(e) => match &e.k {
            EK::Throw(_) => false,
            EK::Call(_, _, Some(f)) => !p.has_attr(*f, "noreturn"),
            _ => true,
        },
        Stmt::If { then, els: Some(els), .. } => falls_through(p, then) || falls_through(p, els),
        Stmt::For { cond: None, body, .. } => has_break(body),
        Stmt::Case(_, x) | Stmt::Default(x) | Stmt::Label(_, x) => falls_through(p, x),
        Stmt::Try(b, hs) => falls_through(p, b) || hs.iter().any(|h| falls_through(p, &h.body)),
        _ => true,
    }
}

/// The statement under the labels of a labeled statement.
fn unlabeled(s: &Stmt) -> &Stmt {
    match s {
        Stmt::Case(_, x) | Stmt::Default(x) | Stmt::Label(_, x) => unlabeled(x),
        s => s,
    }
}

impl P {
    /// ATTR-7: warn where control goes from the statements of one `case`
    /// into the next label without `[[fallthrough]];`.
    fn check_implicit_fallthrough(&mut self, body: &Stmt) {
        let Stmt::Block(v) = body else { return };
        for j in 1..v.len() {
            if !starts_with_case(&v[j]) {
                continue;
            }
            // the statements since the previous label
            let mut k = j;
            let mut any = false;
            while k > 0 {
                k -= 1;
                if !matches!(unlabeled(&v[k]), Stmt::Null) {
                    any = true;
                }
                if starts_with_case(&v[k]) {
                    break;
                }
            }
            if !any || !starts_with_case(&v[k]) {
                continue;
            }
            if falls_through(self, &v[j - 1]) {
                let line = self.stmt_line(&v[j]);
                self.warn(line, "ATTR-7", "control falls through to the next case label");
            }
        }
    }

    fn stmt_line(&self, s: &Stmt) -> u32 {
        match s {
            Stmt::Expr(e) | Stmt::Return(Some(e)) => e.line,
            Stmt::Case(_, x) | Stmt::Default(x) | Stmt::Label(_, x) => self.stmt_line(x),
            Stmt::Block(v) => v.first().map(|x| self.stmt_line(x)).unwrap_or(0),
            _ => 0,
        }
    }
}

/// The variables that an expression modifies and reads, without looking
/// into calls and into operators that order their operands.
fn mods_reads(e: &Expr, mods: &mut Vec<DeclId>, reads: &mut Vec<DeclId>) {
    let strip = |x: &Expr| -> Option<DeclId> {
        let mut x = x;
        while let EK::Conv(_, a) = &x.k {
            x = a;
        }
        if let EK::Var(id) = &x.k {
            Some(*id)
        } else {
            None
        }
    };
    match &e.k {
        EK::Var(id) => reads.push(*id),
        EK::Unary(UOp::PreInc | UOp::PreDec | UOp::PostInc | UOp::PostDec, a) => {
            if let Some(id) = strip(a) {
                mods.push(id);
            }
            mods_reads(a, mods, reads);
        }
        EK::Unary(_, a) | EK::Conv(_, a) | EK::Member(a, _) => mods_reads(a, mods, reads),
        EK::Binary(op, a, b) if !matches!(op, BOp::LAnd | BOp::LOr) => {
            mods_reads(a, mods, reads);
            mods_reads(b, mods, reads);
        }
        EK::Assign(_, a, b) => {
            if let Some(id) = strip(a) {
                mods.push(id);
            }
            mods_reads(a, mods, reads);
            mods_reads(b, mods, reads);
        }
        _ => {}
    }
}

impl P {
    /// VOL-5: the value of an assignment to a volatile object, used in a
    /// larger expression, is deprecated. `used` is false for an expression
    /// whose value is discarded.
    pub fn check_volatile_assign(&mut self, e: &Expr, used: bool) {
        if self.in_template() || !self.wflag(&["-Wall", "-Wdeprecated", "-Wvolatile", "-Wdeprecated-volatile"]) {
            return;
        }
        let mut unused: Vec<*const Expr> = Vec::new();
        fn discarded(e: &Expr, used: bool, out: &mut Vec<*const Expr>) {
            match &e.k {
                EK::Comma(a, b) => {
                    discarded(a, false, out);
                    discarded(b, used, out);
                }
                EK::Conv(Conv::ToVoid, a) => discarded(a, false, out),
                _ if !used => out.push(e as *const Expr),
                _ => {}
            }
        }
        discarded(e, used, &mut unused);
        let mut lines = Vec::new();
        walk_expr(e, &mut |x: &Expr| {
            if let EK::Assign(None, l, _) = &x.k {
                if l.ty.v && !matches!(l.ty.t, Ty::Class(_)) && !unused.contains(&(x as *const Expr)) {
                    lines.push(x.line);
                }
            }
        });
        for l in lines {
            self.warn(l, "VOL-5", "the value of an assignment to a volatile object is used, which is deprecated");
        }
    }

    /// SEQ-7: warn when the unsequenced operands of a built-in operator
    /// both modify one variable, or one modifies it and the other reads it.
    pub fn check_unsequenced(&mut self, e: &Expr) {
        if !self.wflag(&["-Wall", "-Wsequence-point", "-Wunsequenced"]) || self.in_template() {
            return;
        }
        let mut found = None;
        walk_expr(e, &mut |x: &Expr| {
            if found.is_some() {
                return;
            }
            if let EK::Binary(op, a, b) = &x.k {
                if matches!(op, BOp::LAnd | BOp::LOr | BOp::Shl | BOp::Shr) {
                    return;
                }
                let (mut ma, mut ra, mut mb, mut rb) = (vec![], vec![], vec![], vec![]);
                mods_reads(a, &mut ma, &mut ra);
                mods_reads(b, &mut mb, &mut rb);
                if ma.iter().any(|v| mb.contains(v) || rb.contains(v)) || mb.iter().any(|v| ra.contains(v)) {
                    found = Some(x.line);
                }
            }
        });
        if let Some(line) = found {
            self.warn(line, "SEQ-7", "unsequenced modification and access to a variable");
        }
    }
}
