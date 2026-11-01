// Declarations at namespace, class, and block scope: simple
// declarations, functions, classes, enumerations, namespaces, using,
// templates, static assertions.

use crate::ast::*;
use crate::decl::*;
use crate::lexer::{Lit, TK};
use crate::parser::*;
use crate::types::*;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ctx {
    Namespace,
    Class(usize),
    Block,
    /// init-statement of if / switch / for
    Init,
}

pub struct PendingTemplate {
    pub params: Vec<String>,
    pub specialization: bool,
}

impl P {
    pub fn parse_translation_unit(&mut self) {
        while self.tok().kind != TK::Eof {
            if self.at("}") {
                self.error_norule(self.line(), "unexpected '}'");
                self.pos += 1;
                continue;
            }
            self.parse_declaration_recover(Ctx::Namespace);
        }
        self.check_main();
        self.check_internal_uses();
    }

    pub fn parse_declaration_recover(&mut self, ctx: Ctx) -> Option<Stmt> {
        let start = self.pos;
        self.pending_attrs.clear();
        match self.parse_declaration(ctx) {
            Ok(s) => s,
            Err(e) => {
                self.unsupported(e.line, format!("parse: {}", e.msg));
                // Names this declaration would have declared are unknown
                // now, so failed lookups in this scope are not errors.
                let s = self.decl_scope();
                self.scopes[s].uncertain = true;
                if self.pos == start {
                    self.pos += 1;
                }
                self.recover();
                Some(Stmt::Unsupported(e.msg))
            }
        }
    }

    /// Parse one declaration. Returns a statement for block-scope
    /// declarations.
    pub fn parse_declaration(&mut self, ctx: Ctx) -> PR<Option<Stmt>> {
        let saved = std::mem::take(&mut self.decl_attrs);
        let r = self.parse_declaration_inner(ctx);
        self.decl_attrs = saved;
        r
    }

    fn parse_declaration_inner(&mut self, ctx: Ctx) -> PR<Option<Stmt>> {
        let line = self.line();
        self.last_align = 0;
        self.saw_alignas = false;
        self.saw_nua = false;
        self.skip_attributes();
        // The leading attributes apply to the declared entities.
        let attrs = std::mem::take(&mut self.pending_attrs);
        for a in &attrs {
            match a.name.as_str() {
                // ATTR-2: fallthrough is a statement on its own.
                "fallthrough" => self.error(a.line, "ATTR-2", "'fallthrough' on a declaration"),
                // ATTR-13: an assumption is a statement on its own.
                "assume" => self.error(a.line, "ATTR-13", "assumption on a declaration"),
                _ => {}
            }
        }
        self.decl_attrs = attrs;
        if self.eat(";") {
            return Ok(Some(Stmt::Null));
        }
        let t = self.tok().text.clone();
        match t.as_str() {
            "namespace" => {
                if ctx != Ctx::Namespace {
                    // NS-1: a namespace definition is at namespace scope.
                    if !(self.peek(2).text == "=") {
                        self.error(line, "NS-10", "namespace definition not at namespace scope");
                    }
                }
                self.parse_namespace(false)?;
                return Ok(None);
            }
            "inline" if self.at_k(1, "namespace") => {
                self.pos += 1;
                self.parse_namespace(true)?;
                return Ok(None);
            }
            "using" => {
                self.parse_using(ctx)?;
                return Ok(Some(Stmt::Null));
            }
            "template" => {
                self.parse_template(ctx)?;
                return Ok(Some(Stmt::Null));
            }
            "static_assert" => {
                self.parse_static_assert()?;
                return Ok(Some(Stmt::Null));
            }
            "extern" if self.peek(1).kind == TK::Str => {
                self.parse_linkage_spec(ctx)?;
                return Ok(None);
            }
            "export" | "module" | "import" => {
                return Err(self.err("modules (outside the spec)"));
            }
            "asm" => {
                return Err(self.err("asm declaration"));
            }
            "concept" => {
                return Err(self.err("concept outside a template"));
            }
            _ => {}
        }
        if let Ctx::Class(c) = ctx {
            if matches!(t.as_str(), "public" | "private" | "protected") && self.at_k(1, ":") {
                if let Some(a) = self.access_stack.last_mut() {
                    *a = match t.as_str() {
                        "public" => 0,
                        "protected" => 1,
                        _ => 2,
                    };
                }
                self.pos += 2;
                return Ok(None);
            }
            let _ = c;
        }
        let ids = self.parse_simple_declaration(ctx)?;
        Ok(Some(Stmt::Decl(ids)))
    }

    fn parse_namespace(&mut self, is_inline: bool) -> PR<()> {
        let line = self.line();
        self.expect("namespace")?;
        self.skip_attributes();
        let mut names: Vec<(String, bool)> = Vec::new();
        if self.is_ident() {
            // alias?
            if self.at_k(1, "=") {
                let name = self.ident()?;
                self.expect("=")?;
                let scope = self.parse_nested_name()?;
                let target = self.ident()?;
                let found = match scope {
                    None => self.lookup(&target),
                    Some(Ok(s)) => self.lookup_in_scope(s, &target, true),
                    Some(Err(())) => None,
                };
                self.expect(";")?;
                match found.and_then(|ids| match self.decls[ids[0]].k {
                    DK::Namespace(s) => Some(s),
                    _ => None,
                }) {
                    Some(s) => {
                        self.add_decl(&name, DK::Namespace(s), line);
                    }
                    None => {
                        // NS-5: the alias must name a namespace.
                        self.error(line, "NS-14", format!("'{}' does not name a namespace", target));
                    }
                }
                return Ok(());
            }
            loop {
                let inl = self.eat("inline");
                names.push((self.ident()?, inl));
                if !self.eat("::") {
                    break;
                }
            }
        }
        if !names.is_empty() && names[0].1 {
            self.error(line, "NS-11", "the first name of a nested namespace definition must not be inline");
        }
        if is_inline && names.len() > 1 {
            self.error(line, "NS-11", "'inline' before a nested namespace definition");
        }
        self.skip_attributes();
        self.expect("{")?;
        let saved = self.stack.clone();
        if names.is_empty() {
            let cur = self.cur_scope();
            // NS-4: unnamed namespace members are visible in the enclosing scope.
            let existing = self.scopes[cur].names.get("<unnamed>").and_then(|v| v.first().copied());
            let s = match existing.map(|d| self.decls[d].k.clone()) {
                Some(DK::Namespace(s)) => s,
                _ => {
                    let s = self.new_scope(SK::Namespace, Some(cur), "");
                    self.add_decl("<unnamed>", DK::Namespace(s), line);
                    self.scopes[cur].using_ns.push(s);
                    s
                }
            };
            self.stack.push(s);
        }
        let n = names.len();
        for (i, (name, inl)) in names.iter().enumerate() {
            let inl = *inl || (is_inline && i + 1 == n);
            let cur = self.cur_scope();
            let existing = self.scopes[cur].names.get(name).cloned().unwrap_or_default();
            let mut target = None;
            for id in &existing {
                if let DK::Namespace(s) = self.decls[*id].k {
                    target = Some(s);
                } else {
                    // NS-1: the namespace name must not name another entity.
                    self.error(line, "NS-10", format!("'{}' already declared as something else", name));
                }
            }
            let s = match target {
                Some(s) => {
                    if inl && !self.scopes[cur].using_ns.contains(&s) {
                        // NS-3: inline only on the first definition.
                        self.error(line, "NS-12", "namespace first defined without 'inline' is redefined as inline");
                    }
                    s
                }
                None => {
                    let s = self.new_scope(SK::Namespace, Some(cur), name);
                    self.add_decl(name, DK::Namespace(s), line);
                    if inl {
                        self.scopes[cur].using_ns.push(s);
                        self.inline_ns.insert(s);
                    }
                    s
                }
            };
            self.stack.push(s);
        }
        while !self.at("}") && self.tok().kind != TK::Eof {
            self.parse_declaration_recover(Ctx::Namespace);
        }
        self.expect("}")?;
        self.stack = saved;
        Ok(())
    }

    fn parse_linkage_spec(&mut self, ctx: Ctx) -> PR<()> {
        let line = self.line();
        self.expect("extern")?;
        let lang = match self.lit_token() {
            Some(Lit::Str { units, .. }) => units.iter().map(|&u| char::from_u32(u).unwrap_or('?')).collect::<String>(),
            _ => String::new(),
        };
        self.pos += 1;
        if lang != "C" && lang != "C++" {
            // LSP: the string names the language. Other strings are
            // implementation-defined; the front end rejects them.
            self.error(line, "LSP-5", format!("unknown language linkage \"{}\"", lang));
        }
        if ctx != Ctx::Namespace {
            self.error(line, "LSP-2", "linkage specification not at namespace scope");
        }
        let c = lang == "C";
        self.lang_c += c as u32;
        let r = (|| {
            if self.eat("{") {
                while !self.at("}") && self.tok().kind != TK::Eof {
                    self.parse_declaration_recover(Ctx::Namespace);
                }
                self.expect("}")?;
            } else {
                self.parse_declaration(Ctx::Namespace)?;
            }
            Ok(())
        })();
        self.lang_c -= c as u32;
        r
    }

    fn parse_using(&mut self, ctx: Ctx) -> PR<()> {
        let line = self.line();
        self.expect("using")?;
        if self.eat("namespace") {
            if let Ctx::Class(_) = ctx {
                self.error(line, "NS-9", "using directive in a class");
            }
            if ctx == Ctx::Init {
                self.error(line, "IF-21", "using directive as an init-statement");
            }
            let scope = self.parse_nested_name()?;
            let name = self.ident()?;
            self.expect(";")?;
            let found = match scope {
                None => self.lookup(&name),
                Some(Ok(s)) => self.lookup_in_scope(s, &name, true),
                Some(Err(())) => None,
            };
            let cur = self.decl_scope();
            match found.and_then(|ids| match self.decls[ids[0]].k {
                DK::Namespace(s) => Some(s),
                _ => None,
            }) {
                Some(s) => self.scopes[cur].using_ns.push(s),
                None => {
                    // NS-8: the directive must name a namespace.
                    self.error(line, "NS-17", format!("'{}' is not a namespace", name));
                    self.scopes[cur].uncertain = true;
                }
            }
            return Ok(());
        }
        if self.eat("enum") {
            if ctx == Ctx::Init {
                self.error(line, "IF-21", "using-enum declaration as an init-statement");
            }
            let q = self.parse_type_name().or_else(|_| {
                Err(PErr { line, msg: "using enum: unknown name".into() })
            });
            self.expect(";")?;
            let q = match q {
                Ok(q) => q,
                Err(_) => {
                    self.error(line, "ENUM-20", "using enum must name an enumeration");
                    return Ok(());
                }
            };
            match q.t {
                Ty::Enum(e) => {
                    if !self.enums[e].complete {
                        // ENUM-9: the enumeration must be complete.
                        self.error(line, "ENUM-20", "using enum names an incomplete enumeration");
                        return Ok(());
                    }
                    let es = self.enums[e].scope;
                    let names: Vec<(String, Vec<DeclId>)> = self.scopes[es].names.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                    let cur = self.decl_scope();
                    for (n, ids) in names {
                        if let Some(prev) = self.scopes[cur].names.get(&n) {
                            if prev.iter().any(|p| !ids.contains(p)) {
                                self.error(line, "ENUM-20", format!("enumerator '{}' already declared in this scope", n));
                                continue;
                            }
                        }
                        self.scopes[cur].names.entry(n).or_default().extend(ids);
                    }
                }
                Ty::Unknown => {}
                _ => self.error(line, "ENUM-20", "using enum must name an enumeration"),
            }
            return Ok(());
        }
        // alias declaration: using name [attrs] = type-id ;
        if self.is_ident() && (self.at_k(1, "=") || (self.at_k(1, "[") && self.at_k(2, "["))) {
            let name = self.ident()?;
            self.skip_attributes();
            self.expect("=")?;
            let q = self.parse_type_id()?;
            self.expect(";")?;
            if let Some(pt) = self.pending_template.take() {
                let _ = pt;
                self.add_decl(&name, DK::Template(TemplKind::Alias), line);
                return Ok(());
            }
            self.declare_typedef(&name, q, line, ctx);
            return Ok(());
        }
        // using-declaration
        if ctx == Ctx::Init {
            self.error(line, "IF-21", "using-declaration as an init-statement");
        }
        loop {
            self.eat("typename");
            let scope = self.parse_nested_name()?;
            if scope.is_none() {
                // NS-7: a using declaration needs a qualified name.
                self.error(line, "NS-16", "using declaration without a qualified name");
            }
            let mut name = if self.eat("operator") {
                let mut d = Declarator { name: None, kind: NameKind::Plain, qual: None, qual_unknown: false, ops: vec![], line, conv_ty: None, template_args: false, pack: false };
                let _ = &mut d;
                self.recover_until_semicolon();
                return Ok(());
            } else {
                self.ident()?
            };
            if self.eat("~") {
                name = self.ident()?;
            }
            self.eat("...");
            match scope {
                Some(Ok(s)) => {
                    let is_class_scope = matches!(self.scopes[s].kind, SK::Class(_));
                    if is_class_scope && ctx == Ctx::Namespace {
                        self.error(line, "NS-9", "using declaration at namespace scope names a class member");
                    }
                    if let SK::Class(c) = self.scopes[s].kind {
                        if self.classes[c].name == name {
                            // inheriting constructors: not modeled
                            if let Ctx::Class(cc) = ctx {
                                self.classes[cc].opaque = true;
                            }
                        }
                    }
                    match self.lookup_in_scope(s, &name, true) {
                        Some(ids) => {
                            if ids.iter().any(|&i| matches!(self.decls[i].k, DK::Namespace(_))) {
                                self.error(line, "NS-16", "using declaration names a namespace");
                            }
                            let cur = self.decl_scope();
                            let prev = self.scopes[cur].names.get(&name).cloned().unwrap_or_default();
                            // NS-7: must not conflict with another declaration.
                            if !prev.is_empty()
                                && prev.iter().any(|p| !ids.contains(p))
                                && !(prev.iter().all(|&p| self.func(p).is_some()) && ids.iter().all(|&i| self.func(i).is_some()))
                            {
                                self.error(line, "NS-16", format!("using declaration of '{}' conflicts with a declaration in this scope", name));
                            }
                            let e = self.scopes[cur].names.entry(name.clone()).or_default();
                            for i in ids {
                                if !e.contains(&i) {
                                    e.push(i);
                                }
                            }
                        }
                        None => {
                            if let SK::Class(c) = self.scopes[s].kind {
                                if self.class_uncertain(c) {
                                    let cur = self.decl_scope();
                                    self.scopes[cur].uncertain = true;
                                } else {
                                    self.error(line, "NAM-8", format!("no member named '{}'", name));
                                }
                            } else {
                                self.error(line, "NAM-8", format!("no member named '{}' in the namespace", name));
                            }
                        }
                    }
                }
                _ => {
                    let cur = self.decl_scope();
                    self.scopes[cur].uncertain = true;
                }
            }
            if !self.eat(",") {
                break;
            }
        }
        self.expect(";")?;
        Ok(())
    }

    pub fn recover_until_semicolon(&mut self) {
        while !self.at(";") && self.tok().kind != TK::Eof {
            if self.at("(") || self.at("{") || self.at("[") {
                self.skip_balanced();
            } else {
                self.pos += 1;
            }
        }
        self.eat(";");
    }

    pub fn declare_typedef(&mut self, name: &str, q: QTy, line: u32, _ctx: Ctx) {
        let cur = self.decl_scope();
        if let Some(prev) = self.scopes[cur].names.get(name).cloned() {
            for p in prev {
                match &self.decls[p].k {
                    DK::Type(old) => {
                        let in_class = matches!(self.scopes[cur].kind, SK::Class(_));
                        if in_class {
                            // TYN-2: a member alias must not be declared again.
                            self.error(line, "TYN-2", format!("member type alias '{}' declared again", name));
                        } else if *old != q && !old.is_unknown() && !q.is_unknown() {
                            self.error(line, "TYN-2", format!("type alias '{}' redeclared with a different type", name));
                        }
                        return;
                    }
                    DK::Class(c) if Ty::Class(*c) == q.t => return,
                    DK::Enum(e) if Ty::Enum(*e) == q.t => return,
                    _ => {
                        self.error(line, "TYN-2", format!("'{}' already declared as something else", name));
                        return;
                    }
                }
            }
        }
        // TYN-3: a typedef names an unnamed class.
        if let Ty::Class(c) = q.t {
            if self.classes[c].name.is_empty() {
                self.classes[c].name = name.to_string();
            }
        }
        if let Ty::Enum(e) = q.t {
            if self.enums[e].name.is_empty() {
                self.enums[e].name = name.to_string();
            }
        }
        self.add_decl_in(cur, name, DK::Type(q), line);
    }

    fn parse_template(&mut self, ctx: Ctx) -> PR<()> {
        let line = self.line();
        self.expect("template")?;
        if !self.at("<") {
            // explicit instantiation
            self.templ += 1;
            let r = self.parse_declaration(ctx);
            self.templ -= 1;
            r?;
            return Ok(());
        }
        self.expect("<")?;
        let scope = self.push_scope(SK::Template);
        let _ = scope;
        let mut params = Vec::new();
        let specialization = self.at(">");
        let r = (|| -> PR<()> {
            if self.eat(">") {
                return Ok(());
            }
            loop {
                let pl = self.line();
                if self.at("class") || self.at("typename") {
                    self.pos += 1;
                    self.eat("...");
                    let name = if self.is_ident() { self.ident()? } else { String::new() };
                    if self.eat("=") {
                        self.parse_type_id()?;
                    }
                    params.push(name.clone());
                    self.add_decl(&name, DK::TypeParam, pl);
                } else if self.at("template") {
                    self.pos += 1;
                    self.skip_balanced();
                    self.pos += 1; // class/typename
                    self.eat("...");
                    let name = if self.is_ident() { self.ident()? } else { String::new() };
                    params.push(name.clone());
                    self.add_decl(&name, DK::Template(TemplKind::Class(usize::MAX)), pl);
                } else if self.is_ident() && self.lookup(&self.tok().text.clone()).is_some_and(|ids| matches!(self.decls[ids[0]].k, DK::Template(TemplKind::Concept))) {
                    // constrained parameter
                    self.pos += 1;
                    if self.at("<") {
                        self.skip_balanced();
                    }
                    self.eat("...");
                    let name = if self.is_ident() { self.ident()? } else { String::new() };
                    params.push(name.clone());
                    self.add_decl(&name, DK::TypeParam, pl);
                } else {
                    let ds = self.parse_decl_specs(None)?;
                    let base = self.ds_base(&ds).unwrap_or(QTy::unknown());
                    let d = self.parse_declarator(true, true)?;
                    let ty = self.apply_ops(base, &d.ops, pl);
                    let name = d.name.clone().unwrap_or_default();
                    if self.eat("=") {
                        self.templ += 1;
                        let e = self.parse_conditional();
                        self.templ -= 1;
                        e?;
                    }
                    params.push(name.clone());
                    let vi = VarInfo { ty, storage: Storage::Auto, constexpr: false, constinit: false, init: None, param: true, local: false, dependent: true, defined: true, class: None };
                    self.add_decl(&name, DK::Var(vi), pl);
                }
                if self.eat(",") {
                    continue;
                }
                self.expect(">")?;
                return Ok(());
            }
        })();
        if let Err(e) = r {
            self.pop_scope();
            return Err(e);
        }
        if self.at("requires") {
            self.pos += 1;
            self.skip_constraint()?;
        }
        if self.at("concept") {
            self.pos += 1;
            let name = self.ident()?;
            self.recover_until_semicolon();
            self.pop_scope();
            let cur = self.decl_scope();
            self.add_decl_in(cur, &name, DK::Template(TemplKind::Concept), line);
            return Ok(());
        }
        if let Ctx::Block = ctx {
            // Templates cannot be declared in a block.
            self.pop_scope();
            self.error(line, "FN-9", "template declaration in a block");
            self.recover();
            return Ok(());
        }
        self.pending_template = Some(PendingTemplate { params, specialization });
        self.templ += 1;
        let r = self.parse_declaration(ctx);
        self.templ -= 1;
        self.pending_template = None;
        self.pop_scope();
        r?;
        Ok(())
    }

    pub fn skip_constraint(&mut self) -> PR<()> {
        loop {
            if self.at("(") {
                self.skip_balanced();
            } else if self.at("requires") {
                self.pos += 1;
                if self.at("(") {
                    self.skip_balanced();
                }
                if self.at("{") {
                    self.skip_balanced();
                }
            } else if self.at("!") {
                self.pos += 1;
                continue;
            } else if self.at("true") || self.at("false") {
                self.pos += 1;
            } else {
                self.skip_qualified_name()?;
            }
            if self.at("&&") || self.at("||") {
                self.pos += 1;
                continue;
            }
            return Ok(());
        }
    }

    pub fn parse_static_assert(&mut self) -> PR<()> {
        let line = self.line();
        self.expect("static_assert")?;
        self.expect("(")?;
        let e = self.parse_assignment()?;
        if self.eat(",") {
            // SA-4: the message is one or more string literals.
            let ml = self.line();
            if self.tok().kind == TK::Str {
                while self.tok().kind == TK::Str {
                    self.pos += 1;
                }
            } else {
                self.error(ml, "SA-8", "static_assert message must be a string literal");
                while !self.at(")") && self.tok().kind != TK::Eof {
                    if self.at("(") {
                        self.skip_balanced();
                    } else {
                        self.pos += 1;
                    }
                }
            }
        }
        self.expect(")")?;
        self.expect(";")?;
        if self.in_template() {
            // SA-5: checked only on instantiation, which this front end
            // does not do.
            self.skipped_template_check = true;
            return Ok(());
        }
        let b = self.contextual_bool(e, "SA-7");
        match self.const_eval_bool(&b) {
            Ok(true) => {}
            Ok(false) => self.error(line, "SA-1", "static assertion failed"),
            Err(crate::consteval::CeErr::NotConst(rule, msg)) => {
                self.error(line, rule, format!("static_assert condition is not a constant expression: {}", msg))
            }
            Err(crate::consteval::CeErr::Unsupported(msg)) => self.unsupported(line, format!("static_assert: {}", msg)),
        }
        Ok(())
    }

    // ---- simple declarations ----

    pub fn parse_simple_declaration(&mut self, ctx: Ctx) -> PR<Vec<DeclId>> {
        let line = self.line();
        let class = if let Ctx::Class(c) = ctx { Some(c) } else { None };
        let ds = self.parse_decl_specs(class)?;
        if ds.friend {
            if let Ctx::Class(c) = ctx {
                self.friend_classes.insert(c);
            }
        }
        self.check_decl_specs(&ds, ctx, line);
        if self.at(";") {
            self.pos += 1;
            if ds.tag_only {
                if !ds.storage.is_empty() && ds.defined_class.is_some() {
                    // STO-6: no storage class on a declaration that only defines a class.
                    let is_anon_union = ds.defined_class.is_some_and(|c| self.classes[c].name.is_empty());
                    if !is_anon_union {
                        self.error(line, "STO-6", "storage class on a declaration that only defines a class");
                    }
                }
                if ds.constexpr && (ds.defined_class.is_some() || ds.defined_enum.is_some()) {
                    self.error(line, "CX-5", "'constexpr' on a declaration that only defines a type");
                }
                if let Some(c) = ds.defined_class {
                    if self.classes[c].name.is_empty() && self.classes[c].key == 2 {
                        self.anonymous_union(c, &ds, ctx, line);
                    } else if self.classes[c].name.is_empty() && matches!(ctx, Ctx::Class(_)) {
                        // CLS-9: an unnamed member of class type must be an
                        // anonymous union.
                        self.error(line, "CLS-9", "unnamed member of class type that is not an anonymous union");
                    }
                }
                return Ok(vec![]);
            }
            if ds.typedef {
                self.error(line, "TYN-2", "typedef without a name");
                return Ok(vec![]);
            }
            return Err(PErr { line, msg: "declaration does not declare anything".into() });
        }
        if ds.ty.is_none() && ds.auto.is_none() && !self.is_ctor_start(class) && !self.at("~") && !self.at("operator") && !self.at("(") {
            if self.is_ident() && !self.at_k(1, "::") {
                // An unknown type name.
                let name = self.tok().text.clone();
                if self.lookup(&name).is_none() && (self.peek(1).kind == TK::Ident || self.at_k(1, "*") || self.at_k(1, "&")) {
                    if self.lookup_uncertain() {
                        return Err(PErr { line, msg: format!("unknown type name '{}' in uncertain scope", name) });
                    }
                    self.error(line, "NAM-8", format!("unknown type name '{}'", name));
                    self.recover();
                    return Ok(vec![]);
                }
            }
            if !ds.storage.is_empty() || ds.constexpr || ds.inline || ds.c > 0 {
                return Err(PErr { line, msg: "declaration with no type".into() });
            }
            return Err(PErr { line, msg: format!("expected a declaration, found '{}'", self.tok_desc()) });
        }
        if ds.auto.is_some() && (self.at("[") || ((self.at("&") || self.at("&&")) && self.at_k(1, "["))) {
            return self.parse_structured_binding(&ds, ctx, line);
        }
        if (ds.ty.is_some() && self.at("[") && !self.at_k(1, "[")) || (ds.auto.is_some() && self.at("*") && self.at_k(1, "[")) {
            // SB-7: a structured binding uses plain `auto`.
            self.error(line, "SB-15", "structured binding declared with a type other than auto");
            self.eat("*");
            return self.parse_structured_binding(&ds, ctx, line);
        }
        let mut ids = Vec::new();
        let mut first = true;
        let mut auto_ty: Option<QTy> = None;
        loop {
            // an unnamed bit-field has no declarator name
            let unnamed_bf = matches!(ctx, Ctx::Class(_)) && self.at(":");
            let d = self.parse_declarator(unnamed_bf, false)?;
            if let Some(n) = &d.name {
                if n.is_empty() {
                    return Err(self.err("missing declarator name"));
                }
            }
            let (id, was_fn_def) = self.declare(&ds, &d, ctx, first, &mut auto_ty)?;
            if let Some(id) = id {
                ids.push(id);
            }
            if was_fn_def {
                return Ok(ids);
            }
            first = false;
            if self.eat(",") {
                continue;
            }
            break;
        }
        self.expect(";")?;
        Ok(ids)
    }

    /// Structured bindings (spec/declarations/structured-bindings.md).
    fn parse_structured_binding(&mut self, ds: &DeclSpec, ctx: Ctx, line: u32) -> PR<Vec<DeclId>> {
        let is_ref = if self.eat("&") {
            Some(true)
        } else if self.eat("&&") {
            Some(false)
        } else {
            None
        };
        self.expect("[")?;
        let mut names: Vec<(String, u32)> = Vec::new();
        loop {
            let nl = self.line();
            self.eat("...");
            let n = self.ident()?;
            self.skip_attributes();
            names.push((n, nl));
            if !self.eat(",") {
                break;
            }
        }
        self.expect("]")?;
        // SB-7, SB-11: specifiers
        if ds.auto == Some(true) {
            self.error(line, "SB-15", "structured binding with decltype(auto)");
        }
        if ds.constexpr {
            self.error(line, "SB-15", "constexpr structured binding");
        }
        if ds.inline || ds.constinit || ds.storage.contains(&Storage::Extern) || ds.typedef {
            self.error(line, "SB-11", "invalid specifier on a structured binding declaration");
        }
        if matches!(ctx, Ctx::Class(_)) {
            self.error(line, "SB-15", "structured binding as a class member");
        }
        if self.pending_template.is_some() {
            self.error(line, "SB-15", "structured binding template");
        }
        // SB-8: distinct names, not already declared here
        for (i, (n, nl)) in names.iter().enumerate() {
            if names[..i].iter().any(|(m, _)| m == n) {
                self.error(*nl, "SB-8", format!("'{}' named twice in a structured binding", n));
            }
            let cur = self.decl_scope();
            if self.scopes[cur].names.get(n).is_some_and(|v| !v.is_empty()) {
                self.error(*nl, "SB-8", format!("'{}' already declared in this scope", n));
            }
        }
        // initializer: the names are not in scope in it (SB-8)
        let il = self.line();
        let init = if self.eat("=") {
            if self.at("{") {
                self.error(il, "SB-15", "structured binding initialized from '= { ... }'");
                self.parse_braced_init()?
            } else {
                self.parse_assignment()?
            }
        } else if self.at("{") {
            let e = self.parse_braced_init()?;
            match &e.k {
                EK::InitList(v) if v.len() == 1 => v[0].clone(),
                _ => {
                    self.error(il, "SB-15", "structured binding braces must hold one expression");
                    Expr::unknown(il, "sb")
                }
            }
        } else if self.eat("(") {
            let e = self.parse_assignment()?;
            self.expect(")")?;
            e
        } else {
            self.error(il, "SB-15", "structured binding without an initializer");
            Expr::unknown(il, "sb")
        };
        self.expect(";")?;
        // element types
        let et = nonref(&init.ty);
        let cvc = ds.c > 0 || et.c;
        let cvv = ds.v > 0 || et.v;
        if is_ref == Some(true) && init.cat != Cat::L && !(ds.c > 0) && !init.ty.is_unknown() {
            self.error(il, "AUTO-18", "auto& structured binding bound to an rvalue");
        }
        let mut elems: Option<Vec<QTy>> = None;
        match &et.t {
            Ty::Array(e, Some(n)) => {
                let mut e = (**e).clone();
                e.c |= cvc;
                e.v |= cvv;
                elems = Some(vec![e; *n as usize]);
            }
            Ty::Class(c) => {
                let c = *c;
                let tuple = self.lookup_in_scope(self.global, "std", false).is_some();
                if !tuple && self.is_simple_aggregate(c) {
                    let mut v = Vec::new();
                    for &f in &self.classes[c].fields {
                        if let DK::Field(fi) = &self.decls[f].k {
                            let mut t = fi.ty.clone();
                            if !t.t.is_ref() {
                                t.c |= cvc && !fi.mutable;
                                t.v |= cvv;
                            }
                            v.push(t);
                        }
                    }
                    elems = Some(v);
                }
            }
            Ty::Unknown => {}
            _ => {
                if !self.in_template() {
                    self.error(il, "SB-2", "structured binding of a type that is not an array or a class");
                }
            }
        }
        if let Some(es) = &elems {
            if es.len() != names.len() {
                // SB-2: one name per element.
                self.error(line, "SB-2", format!("structured binding declares {} names but the initializer has {} elements", names.len(), es.len()));
            }
        }
        let storage = if ds.storage.contains(&Storage::Static) {
            Storage::Static
        } else if ds.storage.contains(&Storage::ThreadLocal) {
            Storage::ThreadLocal
        } else {
            Storage::Auto
        };
        let mut ids = Vec::new();
        for (i, (n, nl)) in names.iter().enumerate() {
            let ty = elems.as_ref().and_then(|e| e.get(i).cloned()).unwrap_or(QTy::unknown());
            let vi = VarInfo { ty, storage, constexpr: false, constinit: false, init: None, param: false, local: ctx != Ctx::Namespace, dependent: true, defined: true, class: None };
            let id = self.add_decl(n, DK::Var(vi), *nl);
            ids.push(id);
        }
        // ATTR-6: a structured binding declaration that is never read.
        let maybe_unused = self.decl_attrs.iter().any(|a| a.name == "maybe_unused") || self.pending_attrs.iter().any(|a| a.name == "maybe_unused");
        if ctx == Ctx::Block && storage == Storage::Auto && !ids.is_empty() && !maybe_unused && !self.in_template() {
            self.unused_cands.push((ids[0], line));
            self.binding_groups.insert(ids[0], ids.clone());
        }
        Ok(ids)
    }

    fn check_decl_specs(&mut self, ds: &DeclSpec, ctx: Ctx, line: u32) {
        if ds.register {
            // STO-5: `register` is not a storage class.
            self.error(line, "STO-5", "'register' storage class");
        }
        let st = &ds.storage;
        let tl = st.contains(&Storage::ThreadLocal);
        let others = st.iter().filter(|s| **s != Storage::ThreadLocal).count();
        if others > 1 || (tl && st.iter().filter(|s| **s == Storage::ThreadLocal).count() > 1) {
            self.error(line, "STO-6", "two storage classes");
        }
        if ds.typedef && (!st.is_empty() || ds.mutable) {
            self.error(line, "STO-6", "storage class with typedef");
        }
        if ds.typedef && (ds.constexpr || ds.inline) {
            self.error(line, "TYN-2", "typedef with constexpr or inline");
        }
        if ds.consteval && ds.constexpr {
            self.error(line, "CEV-3", "both 'consteval' and 'constexpr'");
        }
        if ds.constinit && ds.constexpr {
            self.error(line, "CINIT-2", "both 'constinit' and 'constexpr'");
        }
        if ds.mutable && !matches!(ctx, Ctx::Class(_)) {
            self.error(line, "STO-6", "'mutable' outside a class");
        }
        if ds.auto.is_some() && ds.ty.is_some() {
            self.error(line, "AUTO-8", "'auto' combined with another type");
        }
        if ds.auto == Some(true) && (ds.c > 0 || ds.v > 0) {
            // AUTO-11: decltype(auto) with cv.
            self.error(line, "AUTO-11", "decltype(auto) combined with a cv-qualifier");
        }
    }

    fn anonymous_union(&mut self, c: usize, ds: &DeclSpec, ctx: Ctx, line: u32) {
        // Members of an anonymous union are members of the enclosing scope.
        if ctx == Ctx::Namespace && !ds.storage.contains(&Storage::Static) {
            // STO-11: an anonymous union at namespace scope must be static.
            self.error(line, "STO-11", "anonymous union at namespace scope must be declared static");
        }
        let fields = self.classes[c].fields.clone();
        let cur = self.decl_scope();
        for f in fields {
            let name = self.decls[f].name.clone();
            if name.is_empty() {
                continue;
            }
            if let Some(prev) = self.scopes[cur].names.get(&name) {
                if !prev.is_empty() {
                    self.error(line, "NAM-3", format!("anonymous union member '{}' conflicts with a declaration", name));
                    continue;
                }
            }
            let ty = match &self.decls[f].k {
                DK::Field(fi) => fi.ty.clone(),
                _ => continue,
            };
            if let Ctx::Class(cc) = ctx {
                if !self.classes[cc].anon_unions.contains(&c) {
                    self.classes[cc].anon_unions.push(c);
                }
                // a variant member of the enclosing class
                let idx = self.classes[cc].fields.len();
                let fi = FieldInfo { ty, class: cc, index: idx, bits: None, mutable: false, init: None, align: 0, nua: false };
                let id = self.add_decl_in(cur, &name, DK::Field(fi), line);
                self.classes[cc].fields.push(id);
                self.classes[cc].opaque = true;
            } else {
                let vi = VarInfo {
                    ty,
                    storage: if ctx == Ctx::Namespace { Storage::Static } else { Storage::Auto },
                    constexpr: false,
                    constinit: false,
                    init: None,
                    param: false,
                    local: ctx != Ctx::Namespace,
                    dependent: false,
                    defined: true,
                    class: None,
                };
                self.add_decl_in(cur, &name, DK::Var(vi), line);
            }
        }
    }

    /// Declare one declarator. Returns the declaration and whether a
    /// function body was parsed (which ends the declaration).
    pub fn declare(&mut self, ds: &DeclSpec, d: &Declarator, ctx: Ctx, first: bool, auto_ty: &mut Option<QTy>) -> PR<(Option<DeclId>, bool)> {
        let line = d.line;
        let name = d.name.clone().unwrap_or_default();
        let base = match (&ds.ty, ds.auto) {
            (Some(t), _) => t.clone(),
            (None, Some(_)) => {
                let mut q = QTy::unknown();
                q.c = ds.c > 0;
                q.v = ds.v > 0;
                q
            }
            (None, None) => {
                // constructor, destructor, conversion function
                if matches!(d.kind, NameKind::Ctor | NameKind::Dtor | NameKind::Conversion) || d.qual_unknown {
                    QTy::new(Ty::Void)
                } else {
                    return Err(PErr { line, msg: format!("declaration of '{}' has no type", name) });
                }
            }
        };
        let _ = first;
        let mut ty = self.apply_ops(base.clone(), &d.ops, line);
        if d.kind == NameKind::Conversion {
            if let Ty::Func(f) = &mut ty.t {
                f.ret = d.conv_ty.clone().unwrap_or(QTy::unknown());
            }
        }
        if (ds.typedef || ds.storage.contains(&Storage::Static)) && matches!(ctx, Ctx::Class(_)) && self.at(":") {
            // BF-2: a bit-field is not static or a typedef.
            self.error(line, "BF-2", "bit-field declared static or typedef");
            self.pos += 1;
            let _ = self.parse_conditional()?;
            return Ok((None, false));
        }
        if ds.typedef {
            if self.at("=") || self.at("{") || (self.at("(") && !self.looks_like_params()) {
                self.error(line, "TYN-2", "typedef with an initializer");
            }
            if d.qual.is_some() {
                return Err(PErr { line, msg: "qualified typedef name".into() });
            }
            self.declare_typedef(&name, ty, line, ctx);
            return Ok((None, false));
        }
        if let Ty::Func(ft) = &ty.t {
            if !matches!(d.ops.last(), Some(DOp::Func(_))) {
                // A function declared through a typedef of function type
                // (TYN-2: it cannot define the function).
                if self.at("{") {
                    self.error(line, "TYN-2", "function defined through a typedef of function type");
                    self.skip_balanced();
                    return Ok((None, true));
                }
                let scope = self.new_scope(SK::Params, Some(self.cur_scope()), "");
                let fd = FnDecl {
                    params: vec![],
                    ptys: ft.params.clone(),
                    variadic: ft.variadic,
                    cv_c: ft.cv_c,
                    cv_v: ft.cv_v,
                    refq: ft.refq,
                    noexcept: ft.noexcept,
                    trailing: None,
                    scope,
                    defaults: 0,
                    has_explicit_this: false,
                    trailing_auto: None,
                };
                let mut d2 = d.clone();
                d2.ops.push(DOp::Func(Box::new(fd)));
                return self.declare_function(ds, &d2, ty, ctx);
            }
            return self.declare_function(ds, d, ty, ctx);
        }
        if ds.has_alignas && self.pending_template.is_none() {
            // ALN-2: the alignment must not be weaker than the natural one.
            if let Some(nat) = self.alignof_ty(&ty.t) {
                if ds.align != 0 && ds.align < nat && !self.align_unknown {
                    self.error(line, "ALN-2", "alignas weaker than the natural alignment");
                }
            }
        }
        if d.qual_unknown || d.template_args {
            // A definition of a member of an unknown class, or a
            // specialization: skip the initializer.
            if let Some(pt) = self.pending_template.take() {
                if d.template_args {
                    return self.declare_var_template_spec(&name, pt, line);
                }
            }
            self.skip_initializer();
            return Ok((None, false));
        }
        if let Ctx::Class(c) = ctx {
            if !ds.storage.contains(&Storage::Static) && d.qual.is_none() {
                return self.declare_field(ds, d, ty, c);
            }
        }
        if let Some(pt) = self.pending_template.take() {
            return self.declare_var_template(ds, &name, ty, pt, line);
        }
        self.declare_variable(ds, d, ty, ctx, auto_ty)
    }

    fn skip_initializer(&mut self) {
        while !self.at(";") && !self.at(",") && self.tok().kind != TK::Eof {
            if self.at("(") || self.at("{") || self.at("[") {
                self.skip_balanced();
            } else if self.at("}") {
                return;
            } else {
                self.pos += 1;
            }
        }
    }

    fn declare_var_template(&mut self, ds: &DeclSpec, name: &str, ty: QTy, pt: PendingTemplate, line: u32) -> PR<(Option<DeclId>, bool)> {
        let _ = (ds, ty);
        let mut init = None;
        if self.eat("=") {
            let e = self.parse_assignment()?;
            init = Some(Rc::new(e));
        } else {
            self.skip_initializer();
        }
        let cur = self.decl_scope();
        let id = self.add_decl_in(cur, name, DK::Template(TemplKind::Var { params: pt.params, init, specs: vec![] }), line);
        Ok((Some(id), false))
    }

    fn declare_var_template_spec(&mut self, name: &str, pt: PendingTemplate, line: u32) -> PR<(Option<DeclId>, bool)> {
        // The declarator stopped at `<`: parse the pattern arguments.
        let found = self.lookup(name);
        let target = found.and_then(|ids| ids.into_iter().find(|&i| matches!(self.decls[i].k, DK::Template(TemplKind::Var { .. }))));
        let mut pats = Vec::new();
        let ok = (|| -> PR<()> {
            self.expect("<")?;
            loop {
                if self.is_ident() {
                    let n = self.tok().text.clone();
                    if let Some(i) = pt.params.iter().position(|p| *p == n) {
                        if self.at_k(1, ",") || self.at_k(1, ">") || self.at_k(1, ">>") {
                            self.pos += 1;
                            pats.push(QTyPat::Param(i));
                            if self.eat(",") {
                                continue;
                            }
                            self.expect(">")?;
                            return Ok(());
                        }
                    }
                }
                if self.starts_type() {
                    let q = self.parse_type_id()?;
                    if q.is_unknown() {
                        return Err(self.err("unsupported pattern"));
                    }
                    pats.push(QTyPat::Ty(q));
                } else {
                    return Err(self.err("non-type pattern"));
                }
                if self.eat(",") {
                    continue;
                }
                self.expect(">")?;
                return Ok(());
            }
        })();
        if ok.is_err() || target.is_none() || !self.at("=") {
            self.skip_initializer();
            if let Some(t) = target {
                if let DK::Template(TemplKind::Var { init, .. }) = &mut self.decls[t].k {
                    *init = None; // can no longer evaluate it
                }
            }
            return Ok((None, false));
        }
        self.expect("=")?;
        let e = self.parse_assignment()?;
        let t = target.unwrap();
        if let DK::Template(TemplKind::Var { specs, .. }) = &mut self.decls[t].k {
            specs.push((pt.params, pats, Rc::new(e)));
        }
        let _ = line;
        Ok((None, false))
    }

    fn declare_field(&mut self, ds: &DeclSpec, d: &Declarator, ty: QTy, c: usize) -> PR<(Option<DeclId>, bool)> {
        let line = d.line;
        let name = d.name.clone().unwrap_or_default();
        self.check_object_attrs(true, self.at(":"));
        if !self.in_template() && self.is_abstract(&ty.t) {
            // VIRT-8: no member of an abstract class type.
            self.error(line, "VIRT-8", format!("member '{}' of an abstract class type", name));
        }
        if ds.auto.is_some() {
            self.error(line, "AUTO-17", "non-static data member declared with a placeholder type");
        }
        if !ds.storage.is_empty() {
            // CLS-9: no thread_local or extern data member.
            self.error(line, "CLS-9", "storage class on a non-static data member");
        }
        if ds.constexpr {
            self.error(line, "CX-5", "non-static data member declared constexpr");
        }
        if ds.mutable && (ty.c || ty.t.is_ref()) {
            self.error(line, "STO-6", "mutable const or reference member");
        }
        if name == self.classes[c].name && !name.is_empty() && !self.classes[c].ctors.is_empty() {
            // CLS-9 names static members, types, and functions; a
            // non-static data member can have the class name in a class
            // without user-declared constructors.
            self.error(line, "CLS-9", "data member with the name of a class that has constructors");
        }
        let mut bits = None;
        if self.at(":") && ds.has_alignas {
            self.error(line, "ALN-2", "alignas on a bit-field");
        }
        if self.eat(":") {
            let bl = self.line();
            let e = self.parse_conditional()?;
            match self.const_int(&e) {
                Ok(v) => {
                    if v < 0 {
                        self.error(bl, "BF-2", "negative bit-field width");
                    } else if v == 0 && !name.is_empty() {
                        self.error(bl, "BF-2", "named bit-field with zero width");
                    } else if let Some(sz) = self.sizeof_ty(&ty.t) {
                        if (v as u64) > sz * 8 && ty.t == Ty::Bool {
                            // a bool bit-field wider than bool is allowed in C++
                        }
                    }
                    bits = Some(v.max(0) as u32);
                }
                Err(_) => bits = Some(1),
            }
            if !(ty.t.is_integral() || matches!(ty.t, Ty::Enum(_) | Ty::Unknown)) {
                self.error(line, "BF-2", "bit-field of non-integral type");
            }
        }
        // CLS-1: a non-static data member must have a complete type.
        if !self.in_template() {
            match &ty.t {
                Ty::Class(cc) if !self.classes[*cc].complete => {
                    self.error(line, "CLS-1", "data member of incomplete class type");
                }
                Ty::Void => self.error(line, "CLS-1", "data member of type void"),
                Ty::Array(_, None) => {
                    // flexible array member: not standard
                    self.error(line, "CLS-1", "data member of array type without a bound");
                }
                _ => {}
            }
        }
        let sc = self.classes[c].scope;
        if let Some(prev) = self.scopes[sc].names.get(&name).cloned() {
            if prev.iter().any(|&p| !matches!(self.decls[p].k, DK::Class(cc) if cc == c)) && !name.is_empty() {
                self.error(line, "CLS-9", format!("member '{}' declared twice", name));
            }
        }
        let idx = self.classes[c].fields.len();
        let fi = FieldInfo { ty: ty.clone(), class: c, index: idx, bits, mutable: ds.mutable, init: None, align: ds.align, nua: self.saw_nua };
        let id = self.add_decl_in(sc, &name, DK::Field(fi), line);
        self.classes[c].fields.push(id);
        // default member initializer: deferred (NAM-6, CLS-2)
        if self.at("=") || self.at("{") {
            if bits.is_some() && false {}
            let start = self.pos;
            if self.eat("=") {
                if self.at("{") {
                    self.skip_balanced();
                } else {
                    self.skip_initializer();
                }
            } else {
                self.skip_balanced();
            }
            let end = self.pos;
            self.deferred.push(Deferred { func: None, field: Some(id), param: None, start, end, stack: self.stack.clone(), templ: self.templ, class: c });
        } else if self.at("(") {
            // CLS-9: no default member initializer in parentheses.
            self.error(line, "CLS-9", "default member initializer in parentheses");
            self.skip_balanced();
        }
        Ok((Some(id), false))
    }

    /// DCMP-3: the form of a defaulted comparison.
    #[allow(clippy::too_many_arguments)]
    fn check_defaulted_comparison(&mut self, name: &str, member: Option<usize>, friend_of: Option<usize>, redeclared: bool, template: bool, ds: &DeclSpec, fd: &FnDecl, placeholder: bool, ret: &QTy, line: u32) {
        let fail = |p: &mut P, why: &str| p.error(line, "DCMP-3", format!("defaulted '{}' {}", name, why));
        if template {
            return fail(self, "is a template");
        }
        let Some(c) = member.or(friend_of) else {
            return fail(self, "is a non-member that is not a friend");
        };
        if self.class_uncertain(c) {
            return;
        }
        if redeclared {
            return fail(self, "is a friend first declared outside the class");
        }
        let ptys: Vec<QTy> = fd.params.iter().map(|&p| self.var(p).map(|v| v.ty.clone()).unwrap_or(QTy::unknown())).collect();
        let cref = |t: &QTy| matches!(&t.t, Ty::LRef(i) if i.t == Ty::Class(c) && i.c && !i.v);
        let cval = |t: &QTy| t.t == Ty::Class(c);
        if member.is_some() {
            if !fd.cv_c || fd.cv_v || fd.refq == 2 {
                return fail(self, "must be a const member function without volatile or '&&'");
            }
            if ptys.len() != 1 || !cref(&ptys[0]) {
                return fail(self, "must have one parameter of type 'const C&'");
            }
        } else if ptys.len() != 2 || !((cref(&ptys[0]) && cref(&ptys[1])) || (cval(&ptys[0]) && cval(&ptys[1]))) {
            return fail(self, "must have two parameters, both 'const C&' or both 'C'");
        }
        if name == "operator<=>" {
            if ds.auto == Some(true) {
                return fail(self, "has a deduced return type not written as 'auto'");
            }
        } else if placeholder || (!ret.is_unknown() && ret.t != Ty::Bool) {
            return fail(self, "must return bool");
        }
    }

    /// OPR-2: the limits on an operator function.
    fn check_operator(&mut self, name: &str, member: bool, is_static: bool, fd: &FnDecl, line: u32) {
        let op = name.trim_start_matches("operator");
        if op.starts_with("\"\"") || matches!(op, "new" | "delete" | "new[]" | "delete[]" | "co_await" | "." | ".*" | "::" | "?" | ":") {
            return;
        }
        let ptys: Vec<QTy> = fd.params.iter().map(|&p| self.var(p).map(|v| v.ty.clone()).unwrap_or(QTy::unknown())).collect();
        if ptys.iter().any(|t| t.is_unknown()) {
            return;
        }
        if fd.variadic {
            self.error(line, "OPR-2", format!("'{}' is variadic", name));
            return;
        }
        let call_like = op == "()" || op == "[]";
        if is_static && !call_like {
            self.error(line, "OPR-2", format!("'{}' declared static", name));
            return;
        }
        if !member && matches!(op, "=" | "()" | "[]" | "->") {
            self.error(line, "OPR-2", format!("'{}' must be a member function", name));
            return;
        }
        if !member {
            let class_or_enum = |t: &QTy| matches!(nonref(t).t, Ty::Class(_) | Ty::Enum(_));
            if !ptys.iter().any(class_or_enum) {
                self.error(line, "OPR-2", format!("non-member '{}' has no parameter of class or enumeration type", name));
                return;
            }
        }
        if call_like {
            return;
        }
        let n = ptys.len() + if member && !is_static && !fd.has_explicit_this { 1 } else { 0 };
        let ok = match op {
            "+" | "-" | "*" | "&" => n == 1 || n == 2,
            "!" | "~" | "->" => n == 1,
            "++" | "--" => {
                if n == 2 && ptys.last().is_some_and(|t| t.t != Ty::Int) {
                    self.error(line, "OPR-2", format!("the parameter of postfix '{}' must be int", name));
                    return;
                }
                n == 1 || n == 2
            }
            _ => n == 2,
        };
        if !ok {
            self.error(line, "OPR-2", format!("'{}' has the wrong number of operands ({})", name, n));
        }
    }

    /// DEFARG-8: after a parameter with a default argument, each later
    /// parameter has one.
    fn check_trailing_defaults(&mut self, have: &[bool], line: u32) {
        if let Some(first) = have.iter().position(|&h| h) {
            if have[first..].iter().any(|&h| !h) {
                self.error(line, "DEFARG-8", "parameter without a default argument after one with a default argument");
            }
        }
    }

    /// DEFARG-5: only a function declaration has default arguments.
    fn check_no_defaults_in(&mut self, d: &Declarator, line: u32) {
        for op in &d.ops {
            if let DOp::Func(fd) = op {
                if fd.defaults > 0 {
                    self.error(line, "DEFARG-5", "default argument in a declarator that does not declare a function");
                    return;
                }
            }
        }
    }

    /// ATTR-2: attributes that do not apply to a variable or data member.
    fn check_object_attrs(&mut self, field: bool, bitfield: bool) {
        for a in self.decl_attrs.clone() {
            match a.name.as_str() {
                "noreturn" => self.error(a.line, "ATTR-2", "'noreturn' applies only to functions"),
                "nodiscard" if !field => self.error(a.line, "ATTR-2", "'nodiscard' on a variable"),
                "no_unique_address" if bitfield => self.error(a.line, "ATTR-2", "'no_unique_address' on a bit-field"),
                _ => {}
            }
        }
    }

    fn declare_variable(&mut self, ds: &DeclSpec, d: &Declarator, mut ty: QTy, ctx: Ctx, auto_ty: &mut Option<QTy>) -> PR<(Option<DeclId>, bool)> {
        let line = d.line;
        let name = d.name.clone().unwrap_or_default();
        self.check_object_attrs(false, false);
        self.check_no_defaults_in(d, line);
        if !self.in_template() && self.is_abstract(&ty.t) && storage_is_object(ds) {
            // VIRT-8: no object of an abstract class.
            self.error(line, "VIRT-8", format!("variable '{}' of an abstract class type", name));
        }
        let storage = if ds.storage.contains(&Storage::Extern) {
            Storage::Extern
        } else if ds.storage.contains(&Storage::Static) {
            Storage::Static
        } else if ds.storage.contains(&Storage::ThreadLocal) {
            Storage::ThreadLocal
        } else {
            Storage::Auto
        };
        let local = ctx == Ctx::Block || ctx == Ctx::Init;
        let class_member = if let Ctx::Class(c) = ctx { Some(c) } else { None };
        let target_scope = d.qual.unwrap_or_else(|| self.decl_scope());
        if let Some(c) = class_member {
            // CLS-9: limits on a static data member.
            if ds.mutable {
                self.error(line, "CLS-9", "static data member declared mutable");
            } else if !self.classes[c].name.is_empty() && name == self.classes[c].name {
                self.error(line, "CLS-9", format!("static data member with the name of its class '{}'", name));
            } else if self.classes[c].name.is_empty() || self.enclosing_unnamed(c) {
                self.error(line, "CLS-9", "static data member in a class with no name");
            }
        }
        if ds.inline && local {
            self.error(line, "INL-2", "'inline' on a block-scope variable");
        }
        if ds.consteval {
            self.error(line, "CEV-3", "'consteval' on a variable");
        }
        if ds.virtual_ || ds.explicit {
            let rule = if ds.virtual_ { "VIRT-2" } else { "CTOR-9" };
            self.error(line, rule, "'virtual' or 'explicit' on a variable");
        }
        if ds.constinit && local && storage == Storage::Auto {
            // CINIT-2: constinit needs static or thread storage.
            self.error(line, "CINIT-2", "'constinit' on a variable with automatic storage");
        }
        if name == "main" && target_scope == self.global {
            // FN-4: a global variable must not be named main.
            self.error(line, "FN-4", "global variable named 'main'");
        }
        // Push qualified scope for the initializer.
        let saved = self.stack.clone();
        if let Some(q) = d.qual {
            self.stack.push(q);
        }
        // Existing declaration in the same scope.
        let prev = self.scopes[target_scope].names.get(&name).cloned().unwrap_or_default();
        let mut redecl: Option<DeclId> = None;
        for &p in &prev {
            match &self.decls[p].k {
                DK::Var(v) => {
                    let same_type = v.ty.t == ty.t || v.ty.is_unknown() || ty.is_unknown() || matches!((&v.ty.t, &ty.t), (Ty::Array(a, _), Ty::Array(b, _)) if a == b);
                    let both_block_auto = local && !v.param && v.storage != Storage::Extern && storage != Storage::Extern;
                    if d.qual.is_some() || (!local && (v.storage == Storage::Extern || storage == Storage::Extern || v.class.is_some())) {
                        if !same_type {
                            self.error(line, "NAM-3", format!("'{}' redeclared with a different type", name));
                        }
                        redecl = Some(p);
                    } else if local && v.param {
                        // NAM-3, FN-1: the outermost block shares the parameter scope.
                        self.error(line, "NAM-3", format!("'{}' redeclares a parameter", name));
                    } else if both_block_auto || local {
                        self.error(line, "NAM-3", format!("'{}' declared twice in one scope", name));
                    } else if !v.defined || !self.var_has_init_or_def(ds) {
                        redecl = Some(p);
                    } else {
                        self.error(line, "NAM-3", format!("'{}' defined twice", name));
                    }
                }
                DK::Field(_) if class_member.is_some() => {
                    self.error(line, "CLS-9", format!("member '{}' declared twice", name));
                }
                DK::Func(_) | DK::Type(_) | DK::Namespace(_) | DK::Enumerator(..) | DK::Template(_) => {
                    if !(d.qual.is_some() && matches!(self.decls[p].k, DK::Func(_))) {
                        self.error(line, "NAM-3", format!("'{}' already declared as a different kind of entity", name));
                    }
                }
                DK::Class(_) | DK::Enum(_) => {
                    // A variable can hide a class name in the same scope.
                }
                _ => {}
            }
        }
        // Conflict with names of an enclosing condition / init (IF-10).
        if local {
            self.check_condition_conflict(&name, line);
        }
        // ODR-1: a static data member defined twice outside its class.
        if let (Some(q), Some(p)) = (d.qual, redecl) {
            if matches!(self.scopes[q].kind, SK::Class(_)) && !self.in_template() && !self.static_member_defs.insert(p) {
                self.error(line, "ODR-1", format!("static data member '{}' defined twice", name));
            }
        }
        if d.qual.is_some() && redecl.is_none() && !self.in_template() {
            if let SK::Class(c) = self.scopes[target_scope].kind {
                if !self.class_uncertain(c) {
                    self.error(line, "CLS-23", format!("no static member '{}' in the class", name));
                }
            } else {
                self.error(line, "NS-15", format!("no member '{}' declared in the namespace", name));
            }
        }
        // Initializer
        let init_line = self.line();
        let mut init: Option<Expr> = None;
        let mut form = InitForm::None;
        let id_placeholder = {
            // NAM-1: the name is in scope in its own initializer.
            let vi = VarInfo {
                ty: ty.clone(),
                storage,
                constexpr: ds.constexpr,
                constinit: ds.constinit,
                init: None,
                param: false,
                local,
                dependent: false,
                defined: storage != Storage::Extern,
                class: class_member.or_else(|| if let SK::Class(c) = self.scopes[target_scope].kind { Some(c) } else { None }),
            };
            match redecl {
                Some(p) => p,
                None => self.add_decl_in(target_scope, &name, DK::Var(vi), line),
            }
        };
        self.auto_in_own_init = if ds.auto.is_some() { Some(id_placeholder) } else { None };
        self.record_attrs(id_placeholder);
        let r = (|| -> PR<()> {
            if self.eat("=") {
                if self.at("{") {
                    init = Some(self.parse_braced_init()?);
                    form = InitForm::CopyList;
                } else {
                    init = Some(self.parse_assignment()?);
                    form = InitForm::Copy;
                }
            } else if self.at("{") {
                init = Some(self.parse_braced_init()?);
                form = InitForm::DirectList;
            } else if self.at("(") {
                self.pos += 1;
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
                form = InitForm::Paren(args.len());
                init = Some(Expr::new(EK::InitList(args), QTy::unknown(), Cat::PR, init_line));
            }
            Ok(())
        })();
        self.auto_in_own_init = None;
        self.stack = saved;
        r?;
        // Placeholder deduction (AUTO rules).
        if ds.auto.is_some() && !ds.concept_auto || ds.auto.is_some() {
            match &init {
                None => {
                    if ty.t == Ty::Unknown || ty.t.contains_unknown() {
                        // AUTO-8: a variable declared with auto must have an initializer.
                        self.error(line, "AUTO-8", "variable declared with 'auto' has no initializer");
                    }
                }
                Some(e) => {
                    let ded = self.deduce_auto(ds, d, &ty, e, &form, line);
                    if let Some(q) = ded {
                        if let Some(prev_t) = auto_ty.as_ref() {
                            let _ = prev_t;
                        }
                        ty = q;
                        *auto_ty = Some(ty.clone());
                    }
                }
            }
        }
        if ds.constexpr {
            // CX-1: a constexpr variable is const.
            ty.c = true;
            if let Ty::Ptr(_) = ty.t {
                ty.c = true;
            }
        }
        // Array bound from the initializer.
        if let Ty::Array(el, None) = &ty.t {
            if let Some(e) = &init {
                if let Some(n) = self.init_list_len(e, el) {
                    if n == 0 {
                        self.error(line, "DCL-19", "array without a bound has an empty initializer");
                    }
                    ty = QTy { t: Ty::Array(el.clone(), Some(n as u64)), c: ty.c, v: ty.v };
                }
            }
        }
        // Checks without an initializer.
        if init.is_none() && !self.in_template() {
            let is_def = storage != Storage::Extern && class_member.is_none();
            if ty.t.is_ref() && is_def {
                // DCL-3: a reference variable must have an initializer.
                self.error(line, "DCL-18", "reference variable without an initializer");
            }
            // CTOR: a const class object can be default-initialized when
            // the class initializes its members; the front end does not
            // check that, so class types are not reported here.
            let class_with_ctor = matches!(&ty.t, Ty::Class(_));
            if ds.constexpr && !class_with_ctor {
                // CX-1. A class with a constexpr default constructor needs
                // no written initializer; the rule text does not say so.
                self.error(line, "CX-1", "constexpr variable without an initializer");
            }
            if ds.constinit && is_def && !class_with_ctor {
                self.error(line, "CINIT-1", "constinit variable without an initializer");
            }
            if ty.c && is_def && !ds.constexpr && self.is_scalar(&ty) {
                // CV-3: a const scalar definition needs an initializer.
                self.error(line, "CV-3", "const variable of scalar type without an initializer");
            }
            if ty.c && is_def && !ds.constexpr {
                if let Ty::Array(el, _) = &ty.t {
                    if self.is_scalar(el) {
                        self.error(line, "CV-3", "const array of scalars without an initializer");
                    }
                }
            }
            if let Ty::Array(_, None) = ty.t {
                if is_def && class_member.is_none() && d.qual.is_none() {
                    self.error(line, "DCL-19", "array without a bound and without an initializer");
                }
            }
            if is_def && !self.in_template() {
                if let Ty::Class(c) = ty.t {
                    if !self.classes[c].complete {
                        self.error(line, "TYN-7", "variable of incomplete class type");
                    }
                }
                if ty.t == Ty::Void {
                    self.error(line, "DCL-20", "variable of type void");
                }
            }
        }
        if let Some(c) = class_member {
            // static data member in class: CLS-3
            let _ = c;
            if init.is_some() && !ds.constexpr && !ds.inline && !(ty.c && (ty.t.is_integral() || matches!(ty.t, Ty::Enum(_)))) && !self.in_template() {
                self.error(line, "CLS-23", "in-class initializer for a static data member that is not const integral, inline, or constexpr");
            }
        }
        if init.is_some() && storage == Storage::Extern && local {
            self.error(line, "STO-13", "block-scope extern declaration with an initializer");
        }
        // Convert the initializer.
        let written_init = init.is_some();
        let mut converted = None;
        if let Some(e) = init {
            let ce = self.convert_init(e, &ty, form, init_line);
            converted = Some(ce);
        } else if storage != Storage::Extern && class_member.is_none() && !self.in_template() {
            if let Some(c) = elem_class(&ty.t) {
                self.check_default_init(c, line);
            }
            // CTOR-4: a const object without an initializer.
            if ty.c && !ds.constexpr {
                if let Some(c) = elem_class(&ty.t) {
                    if self.const_default_ok(c, 0) == Some(false) {
                        self.error(line, "CTOR-4", format!("const object of '{}' without an initializer leaves a member without a value", self.classes[c].name));
                    }
                }
            }
            // Default-initialization runs the default constructor.
            if let Ty::Class(c) = ty.t {
                if self.classes[c].complete && !self.classes[c].ctors.is_empty() {
                    let e = self.ctor_call(c, &[], &ty, line);
                    if matches!(e.k, EK::Construct(..)) {
                        converted = Some(e);
                    }
                }
            }
        }
        if !self.in_template() && storage != Storage::Extern && class_member.is_none() {
            if let Some(c) = elem_class(&ty.t) {
                // UNI-11, DTOR-3: a variable is destroyed at its end, so its
                // destructor must not be deleted.
                if !ty.t.is_ref() && self.classes[c].complete {
                    if self.uni3_deleted(c, crate::special::Sm::Dtor) {
                        self.error(line, "UNI-11", format!("the destructor of '{}' is deleted (UNI-3)", self.classes[c].name));
                    } else if self.dtor_unusable(c) {
                        self.error(line, "DTOR-3", format!("variable of '{}', whose destructor is deleted or not accessible", self.classes[c].name));
                    }
                }
            }
        }
        // Constant initialization checks: CX-1, CINIT-1, CLS-20.
        if let Some(ce) = &converted {
            let needs_const = ds.constexpr || ds.constinit || (class_member.is_some() && ty.c && !ds.inline && (ty.t.is_integral() || matches!(ty.t, Ty::Enum(_))));
            if needs_const && !self.in_template() && !ty.is_unknown() {
                let rule = if ds.constexpr {
                    "CX-1"
                } else if ds.constinit {
                    "CINIT-1"
                } else {
                    "CLS-20"
                };
                // Record the init first so that self-reference is visible.
                self.set_var_init(id_placeholder, &ty, ce.clone(), ds);
                match self.const_eval_init(id_placeholder) {
                    Ok(()) => {}
                    Err(crate::consteval::CeErr::NotConst(r, msg)) => {
                        self.error_because(init_line, rule, r, format!("initializer is not a constant expression ({}: {})", r, msg));
                    }
                    Err(crate::consteval::CeErr::Unsupported(msg)) => {
                        self.unsupported(init_line, format!("constant initializer: {}", msg));
                    }
                }
            }
        }
        if converted.is_none() && (ds.constexpr || ds.constinit) && storage != Storage::Extern && class_member.is_none() && !self.in_template() && matches!(ty.t, Ty::Class(_)) {
            // CX-1, CINIT-1: a class object without an initializer is
            // default-initialized, and every scalar part needs a value
            // (CE-21).
            if let DK::Var(v) = &mut self.decls[id_placeholder].k {
                v.ty = ty.clone();
            }
            let rule = if ds.constexpr { "CX-1" } else { "CINIT-1" };
            match self.const_eval_init(id_placeholder) {
                Ok(()) => {}
                Err(crate::consteval::CeErr::NotConst(r, msg)) => {
                    self.error_because(line, rule, r, format!("default-initialization is not a constant expression ({}: {})", r, msg));
                }
                Err(crate::consteval::CeErr::Unsupported(msg)) => {
                    self.unsupported(line, format!("constant initializer: {}", msg));
                }
            }
        }
        if ds.constexpr && !self.in_template() {
            // CX-1: the type of a constexpr variable is a literal type
            // (CE-23).
            if self.literal_type(&ty) == Some(false) {
                self.error(line, "CX-1", "constexpr variable of a type that is not a literal type (CE-23)");
            }
        }
        if let Some(p) = redecl {
            // CINIT-2: if a declaration has constinit, the initializing
            // declaration has it too (Q-0025).
            if self.var(p).is_some_and(|v| v.constinit) && !ds.constinit && converted.is_some() {
                self.error(line, "CINIT-2", "initializing declaration lacks the 'constinit' of an earlier declaration");
            }
        }
        // ATTR-6: a local scalar variable that may be unused.
        if local && storage == Storage::Auto && redecl.is_none() && ctx == Ctx::Block && (self.is_scalar(&ty) || self.plain_class(&ty.t, 0)) && !self.has_attr(id_placeholder, "maybe_unused") && !self.in_template() {
            self.unused_cands.push((id_placeholder, line));
        }
        // JMP-6, SW-5: variables a jump must not skip.
        if local && storage == Storage::Auto && redecl.is_none() {
            // CTOR-23, DTOR-9: a class with a trivial default constructor
            // and a trivial destructor can be skipped.
            let nontrivial_class = match self.skippable_type(&ty.t) {
                Some(ok) => !ok,
                None => match &ty.t {
                    Ty::Class(c) => !self.classes[*c].ctors.is_empty() || self.classes[*c].dtor.is_some() || self.classes[*c].opaque,
                    _ => false,
                },
            };
            if written_init || nontrivial_class {
                self.jump_vars.insert(id_placeholder);
            }
        }
        // Store the final type and initializer.
        if let DK::Var(v) = &mut self.decls[id_placeholder].k {
            v.ty = ty.clone();
            v.constexpr |= ds.constexpr;
            v.constinit |= ds.constinit;
            if let Some(ce) = converted {
                v.init = Some(Rc::new(ce));
                v.defined = true;
            }
            if storage != Storage::Extern {
                v.defined = true;
            }
        }
        Ok((Some(id_placeholder), false))
    }

    fn var_has_init_or_def(&self, ds: &DeclSpec) -> bool {
        !ds.storage.contains(&Storage::Extern)
    }

    pub fn set_var_init(&mut self, id: DeclId, ty: &QTy, e: Expr, ds: &DeclSpec) {
        if let DK::Var(v) = &mut self.decls[id].k {
            v.ty = ty.clone();
            v.init = Some(Rc::new(e));
            v.constexpr |= ds.constexpr;
        }
    }

    pub fn is_scalar(&self, q: &QTy) -> bool {
        q.t.is_arithmetic() || matches!(q.t, Ty::Ptr(_) | Ty::Enum(_) | Ty::NullPtr | Ty::MemPtr(..))
    }

    fn init_list_len(&self, e: &Expr, el: &QTy) -> Option<usize> {
        match &e.k {
            EK::InitList(items) => {
                // A string literal inside braces initializes a char array.
                if items.len() == 1 {
                    if let EK::Str(s) = &items[0].k {
                        if el.t.is_integral() && el.t != Ty::Bool && self.sizeof_ty(&el.t).is_some() {
                            return Some(s.len() + 1);
                        }
                    }
                }
                if let Ty::Array(..) | Ty::Class(_) = el.t {
                    // brace elision: count only if every item is braced
                    if items.iter().all(|i| matches!(i.k, EK::InitList(_))) || matches!(el.t, Ty::Class(_)) && items.iter().all(|i| !matches!(i.k, EK::InitList(_))) == false {
                        return Some(items.len());
                    }
                    if let Ty::Array(_, Some(n)) = &el.t {
                        if items.iter().all(|i| !matches!(i.k, EK::InitList(_))) {
                            return Some(items.len().div_ceil(*n as usize));
                        }
                    }
                    return None;
                }
                Some(items.len())
            }
            EK::Str(s) => Some(s.len() + 1),
            _ => None,
        }
    }

    // ---- functions ----

    fn declare_function(&mut self, ds: &DeclSpec, d: &Declarator, ty: QTy, ctx: Ctx) -> PR<(Option<DeclId>, bool)> {
        let line = d.line;
        let name = d.name.clone().unwrap_or_default();
        let fd = d.func().unwrap().clone();
        let Ty::Func(mut fty) = ty.t.clone() else { unreachable!() };
        let class = match ctx {
            Ctx::Class(c) => Some(c),
            _ => d.qual.and_then(|s| if let SK::Class(c) = self.scopes[s].kind { Some(c) } else { None }),
        };
        let in_class_body = matches!(ctx, Ctx::Class(_));
        let is_friend = ds.friend;
        if is_friend {
            if let Some(c) = self.current_class() {
                self.friend_classes.insert(c);
            }
        }
        let member_class = if is_friend { None } else { class };
        // Deallocation functions are non-throwing, on every declaration.
        if name == "operatordelete" || name == "operatordelete[]" {
            fty.noexcept = true;
        }
        let kind = match &d.kind {
            NameKind::Ctor => FnKind::Ctor,
            NameKind::Dtor => FnKind::Dtor,
            NameKind::Conversion => FnKind::Conversion,
            NameKind::Operator(_) | NameKind::Literal => FnKind::Operator,
            NameKind::Plain => FnKind::Normal,
        };
        if ds.auto.is_some() && fd.trailing.is_none() {
            fty.ret = QTy::unknown(); // deduced later
        }
        if ds.auto.is_none() && fd.trailing.is_some() && kind != FnKind::Conversion {
            if ds.ty.is_some() {
                // DCL-7: a trailing return type needs `auto` at the start.
                self.error(line, "DCL-21", "trailing return type without 'auto'");
            }
        }
        let is_static = ds.storage.contains(&Storage::Static);
        if ds.explicit && !self.in_template() {
            // CTOR-9: only a constructor or conversion function declared
            // in its class can have explicit.
            if !matches!(kind, FnKind::Ctor | FnKind::Conversion) {
                self.error(line, "CTOR-9", format!("'explicit' on '{}', which is not a constructor or conversion function", name));
            } else if d.qual.is_some() && !in_class_body {
                self.error(line, "CTOR-9", "'explicit' on a declaration outside the class");
            }
        }
        if matches!(kind, FnKind::Ctor | FnKind::Dtor) && !self.in_template() {
            // CTOR-1, DTOR-1: the form of a constructor or destructor.
            let rule = if kind == FnKind::Ctor { "CTOR-1" } else { "DTOR-1" };
            let what = if kind == FnKind::Ctor { "constructor" } else { "destructor" };
            if is_static {
                self.error(line, rule, format!("{} declared static", what));
            } else if ds.virtual_ && kind == FnKind::Ctor {
                self.error(line, rule, "constructor declared virtual");
            } else if fd.cv_c || fd.cv_v || fd.refq != 0 {
                self.error(line, rule, format!("{} with a cv-qualifier or ref-qualifier", what));
            } else if fd.has_explicit_this {
                self.error(line, rule, format!("{} with an explicit object parameter", what));
            } else if ds.ty.is_some() || !ds.kw.is_empty() || ds.auto.is_some() {
                self.error(line, rule, format!("{} with a return type", what));
            } else if kind == FnKind::Dtor && (!fd.params.is_empty() || fd.variadic) {
                self.error(line, rule, "destructor with parameters");
            } else if kind == FnKind::Dtor {
                if let Some(c) = class {
                    if !self.class_uncertain(c) && name != format!("~{}", self.classes[c].name) {
                        self.error(line, rule, format!("destructor name '{}' is not the class name", name));
                    }
                }
            }
        }
        // DCL-7, CLS-7: qualifiers on non-member and static functions.
        if member_class.is_none() && d.qual.is_none() && !d.qual_unknown && (fd.cv_c || fd.cv_v || fd.refq != 0) && !self.in_template() {
            self.error(line, "DCL-21", "cv-qualifier or ref-qualifier on a function that is not a member");
        }
        if member_class.is_some() && is_static && (fd.cv_c || fd.cv_v || fd.refq != 0) {
            self.error(line, "CLS-27", "static member function with a cv-qualifier or ref-qualifier");
        }
        if is_static && ds.virtual_ {
            self.error(line, "CLS-27", "static member function declared virtual");
        }
        if fd.has_explicit_this && (is_static || ds.virtual_ || fd.cv_c || fd.cv_v || fd.refq != 0) {
            self.error(line, "CLS-14", "explicit object member function that is static, virtual, or qualified");
        }
        if ds.storage.contains(&Storage::ThreadLocal) {
            self.error(line, "STO-6", "function declared thread_local");
        }
        if ds.storage.contains(&Storage::Static) && ctx == Ctx::Block {
            self.error(line, "STO-6", "function declared static in a block");
        }
        if ds.constinit {
            self.error(line, "CINIT-2", "'constinit' on a function");
        }
        if ds.mutable {
            self.error(line, "STO-6", "function declared mutable");
        }
        // TR-3 / FN-4: main.
        let target_scope = d.qual.unwrap_or_else(|| if in_class_body && is_friend { self.enclosing_ns_scope() } else { self.decl_scope() });
        let is_main = name == "main" && target_scope == self.global && member_class.is_none();
        if is_main {
            if self.pending_template.is_some() {
                self.error(line, "TR-3", "'main' declared as a template");
            }
            if ds.auto.is_some() {
                self.error(line, "FN-4", "'main' with a deduced return type");
            } else if fty.ret.t != Ty::Int && !fty.ret.is_unknown() {
                self.error(line, "TR-3", "'main' must return int");
            }
            if ds.inline || ds.constexpr || ds.consteval {
                self.error(line, "FN-4", "'main' declared inline, constexpr, or consteval");
            }
            if is_static {
                self.error(line, "STO-6", "'main' declared static");
            }
            let ok_params = fty.params.is_empty()
                || (fty.params.len() == 2
                    && fty.params[0].t == Ty::Int
                    && matches!(&fty.params[1].t, Ty::Ptr(p) if matches!(&p.t, Ty::Ptr(c) if c.t == Ty::Char)));
            if !ok_params || fty.variadic {
                // TR-3 rejects other parameter lists, but FN-6 makes them
                // implementation-defined (Q-0024). This front end follows
                // FN-6 and the EDG baseline and accepts them.
                self.warn(line, "FN-6", "'main' has a parameter list other than () or (int, char**)");
            }
        }
        // Find a previous declaration of the same function.
        let template = self.pending_template.is_some();
        self.pending_template = None;
        let prev = if in_class_body && !is_friend {
            let sc = self.classes[class.unwrap()].scope;
            self.scopes[sc].names.get(&name).cloned().unwrap_or_default()
        } else {
            self.scopes[target_scope].names.get(&name).cloned().unwrap_or_default()
        };
        let mut existing: Option<DeclId> = None;
        let own_scope = if in_class_body && !is_friend { Some(self.classes[class.unwrap()].scope) } else { None };
        for &p in &prev {
            if let Some(os) = own_scope {
                if self.decls[p].scope != os {
                    // brought in by a using-declaration: a member of the
                    // derived class hides it (DER-3)
                    continue;
                }
            }
            match &self.decls[p].k {
                DK::Func(f) => {
                    if f.template != template {
                        continue;
                    }
                    // Two templates whose parameters depend on template
                    // parameters can differ in ways the front end does not
                    // model, such as a requires-clause (OVL-41).
                    let distinct_templates = template && f.fty.ret != fty.ret && f.fty.params.iter().chain(fty.params.iter()).any(|q| q.is_unknown());
                    if f.fty.params == fty.params && f.fty.cv_c == fty.cv_c && f.fty.cv_v == fty.cv_v && f.fty.refq == fty.refq && !(self.at("requires") && in_class_body) && !distinct_templates {
                        // (A member with a trailing requires-clause can
                        // overload one with the same parameters.)
                        existing = Some(p);
                    } else if f.fty.params == fty.params && in_class_body && f.fty.refq != fty.refq && (f.fty.refq == 0 || fty.refq == 0) {
                        // CLS-6: overloads must all have ref-qualifiers or none.
                        self.error(line, "CLS-26", "overloads with and without ref-qualifiers");
                    } else if f.fty.params == fty.params && in_class_body && (f.is_static || is_static) && f.fty.refq == fty.refq {
                        self.error(line, "CLS-9", "same member function declared static and non-static");
                    }
                }
                DK::Var(_) | DK::Field(_) | DK::Type(_) | DK::Namespace(_) | DK::Enumerator(..) => {
                    if !(d.qual.is_some()) && !(in_class_body && self.decls[p].scope != self.classes[class.unwrap()].scope) {
                        self.error(line, if in_class_body { "CLS-9" } else { "NAM-3" }, format!("'{}' already declared as a different kind of entity", name));
                    }
                }
                _ => {}
            }
        }
        if d.qual.is_some() && existing.is_none() && !d.qual_unknown && !self.in_template() && !template {
            if let SK::Class(c) = self.scopes[target_scope].kind {
                if !self.class_uncertain(c) && kind != FnKind::Ctor && kind != FnKind::Dtor {
                    // CLS-4: the class must declare the member first.
                    self.error(line, "CLS-24", format!("no member function '{}' with this signature declared in the class", name));
                }
            } else if !self.scopes[target_scope].uncertain {
                self.error(line, "NS-15", format!("'{}' was not declared in the namespace", name));
            }
        }
        let mut trailing_requires = false;
        // virt-specifiers come before the body (VIRT grammar)
        while self.at("override") || self.at("final") {
            self.pos += 1;
        }
        if self.at("requires") {
            self.pos += 1;
            self.skip_constraint()?;
            trailing_requires = true;
        }
        let deleted = self.at("=") && self.at_k(1, "delete");
        let defaulted = self.at("=") && self.at_k(1, "default");
        if self.at(":") && kind == FnKind::Ctor {
            // CTOR-15: mem-initializers only in a definition.
            let save = self.pos;
            self.skip_ctor_init();
            let decl_only = self.at(";");
            self.pos = save;
            if decl_only {
                self.error(line, "CTOR-15", "mem-initializers in a constructor declaration that is not a definition");
                self.skip_ctor_init();
            }
        }
        let is_def = self.at("{") || self.at(":") || self.at("try") || deleted || defaulted;
        if is_def && ctx == Ctx::Block {
            // FN-9: no function definition inside a function body.
            self.error(line, "FN-9", "function definition inside a function body");
        }
        let id = match existing {
            Some(p) => {
                if in_class_body && !is_friend && !self.in_template() && self.func(p).is_some_and(|f| f.class.is_some()) {
                    // CTOR-1, DTOR-1, CLS-9: not declared two times in the
                    // class.
                    let rule = match kind {
                        FnKind::Ctor => "CTOR-1",
                        FnKind::Dtor => "DTOR-1",
                        _ => "CLS-9",
                    };
                    self.error(line, rule, format!("'{}' declared two times in its class", name));
                }
                // DEL-2: `= delete` on the first declaration only, and no
                // other definition of a deleted function.
                if !self.in_template() {
                    let (pdel, pdef0) = self.func(p).map(|f| (f.deleted, f.defined)).unwrap_or((false, false));
                    if deleted {
                        self.error(line, "DEL-2", format!("'= delete' on a declaration of '{}' that is not the first", name));
                    } else if pdel && (is_def || defaulted) {
                        self.error(line, "DEL-2", format!("definition of the deleted function '{}'", name));
                    }
                    let _ = pdef0;
                }
                // ATTR-2: noreturn and carries_dependency must be on the
                // first declaration.
                for a in self.decl_attrs.clone() {
                    if (a.name == "noreturn" || a.name == "carries_dependency") && self.func(p).is_some_and(|f| !f.first_attrs.contains(&a.name)) {
                        self.error(a.line, "ATTR-2", format!("'{}' on a declaration of a function first declared without it", a.name));
                    }
                }
                let (pc, pce, pne, pdef, pret) = {
                    let f = self.func(p).unwrap();
                    (f.constexpr, f.consteval, f.fty.noexcept, f.defined, f.fty.ret.clone())
                };
                if pc != ds.constexpr && !(kind == FnKind::Normal && false) && !self.in_template() {
                    // CX-5: all declarations must agree on constexpr.
                    if pdef || is_def || true {
                        self.error(line, "CX-5", format!("declarations of '{}' disagree on 'constexpr'", name));
                    }
                }
                if pce != ds.consteval {
                    self.error(line, "CEV-3", format!("declarations of '{}' disagree on 'consteval'", name));
                }
                if pne != fty.noexcept && !self.in_template() {
                    // DCL-14: all declarations must agree on noexcept.
                    self.error(line, "DCL-14", format!("declarations of '{}' disagree on noexcept", name));
                }
                if pret != fty.ret && !pret.is_unknown() && !fty.ret.is_unknown() {
                    // FN-2: all declarations have the same return type.
                    self.error(line, "FN-14", format!("declarations of '{}' have different return types", name));
                }
                if pdef && is_def && !self.in_template() {
                    self.error(line, "FN-14", format!("function '{}' defined twice", name));
                }
                // a later declaration's parameters are used for the body;
                // default arguments accumulate over declarations (DEFARG)
                let new_defaults: Vec<Option<Rc<Expr>>> = fd.params.iter().map(|&p| self.var(p).and_then(|v| v.init.clone())).collect();
                let deferred_defaults: Vec<bool> = fd.params.iter().map(|&p| self.deferred.iter().any(|d| d.param == Some(p))).collect();
                let same_scope = self.decls[p].scope == target_scope || d.qual.is_some() || in_class_body;
                let old: Vec<bool> = self.func(p).map(|f| f.default_exprs.iter().map(|x| x.is_some()).collect()).unwrap_or_default();
                if same_scope && old.len() == new_defaults.len() && !self.in_template() {
                    let mut merged = Vec::new();
                    for i in 0..old.len() {
                        let new = new_defaults[i].is_some() || deferred_defaults[i];
                        if new && old[i] {
                            // DEFARG-9: no default argument given again.
                            self.error(line, "DEFARG-9", format!("default argument for parameter {} of '{}' given again", i + 1, name));
                        }
                        merged.push(new || old[i]);
                    }
                    self.check_trailing_defaults(&merged, line);
                }
                if let Some(f) = self.func_mut(p) {
                    if is_def {
                        f.params = fd.params.clone();
                    }
                    if f.default_exprs.len() == new_defaults.len() {
                        for (i, nd) in new_defaults.into_iter().enumerate() {
                            if nd.is_some() {
                                f.default_exprs[i] = nd;
                            }
                        }
                    }
                    let mut cnt = 0;
                    for i in (0..f.default_exprs.len()).rev() {
                        if f.default_exprs[i].is_some() || deferred_defaults.get(i).copied().unwrap_or(false) {
                            cnt += 1;
                        } else {
                            break;
                        }
                    }
                    f.defaults = f.defaults.max(fd.defaults).max(cnt);
                }
                p
            }
            None => {
                if kind == FnKind::Operator && !self.in_template() && !template {
                    self.check_operator(&name, member_class.is_some(), is_static, &fd, line);
                }
                if deleted && is_main {
                    // DEL-2: main must not be deleted.
                    self.error(line, "DEL-2", "'main' declared deleted");
                }
                if defaulted && member_class.is_none() && !is_friend && !self.in_template() {
                    // DEL-4: `= default` only on special members and
                    // comparison operators (a non-member comparison is
                    // DCMP-3).
                    let cmp = matches!(name.as_str(), "operator==" | "operator!=" | "operator<" | "operator>" | "operator<=" | "operator>=" | "operator<=>");
                    if !cmp {
                        self.error(line, "DEL-4", format!("'= default' on '{}', which is not a special member function or a comparison operator", name));
                    }
                }
                if !self.in_template() {
                    let have: Vec<bool> = fd.params.iter().map(|&p| self.var(p).is_some_and(|v| v.init.is_some()) || self.deferred.iter().any(|d| d.param == Some(p))).collect();
                    self.check_trailing_defaults(&have, line);
                    // DEFARG-6: no default arguments on an operator
                    // function other than operator().
                    // OPR-3 allows them on operator[] too (report-2 finding G9).
                    if kind == FnKind::Operator && name != "operator()" && name != "operator[]" && have.iter().any(|&h| h) {
                        self.error(line, "DEFARG-6", format!("default argument on '{}'", name));
                    }
                }
                // Deallocation functions are non-throwing.
                if name == "operatordelete" || name == "operatordelete[]" {
                    fty.noexcept = true;
                }
                let fi = FuncInfo {
                    fty: (*fty).clone(),
                    params: fd.params.clone(),
                    defaults: fd.defaults,
                    body: None,
                    defined: false,
                    constexpr: ds.constexpr || ds.consteval,
                    consteval: ds.consteval,
                    deleted,
                    defaulted,
                    class: member_class,
                    kind,
                    is_static,
                    is_virtual: ds.virtual_,
                    template,
                    explicit: ds.explicit,
                    opaque: false,
                    is_main,
                    explicit_obj: fd.has_explicit_this,
                    default_exprs: fd.params.iter().map(|&p| self.var(p).and_then(|v| v.init.clone())).collect(),
                    ret_auto: None,
                    mem_inits: Vec::new(),
                    first_attrs: self.decl_attrs.iter().map(|a| a.name.clone()).collect(),
                    first_defaulted: defaulted,
                    constrained: trailing_requires,
                };
                let sc = if in_class_body && !is_friend { self.classes[class.unwrap()].scope } else { target_scope };
                let id = self.add_decl_in(sc, &name, DK::Func(fi), line);
                if self.lang_c > 0 && member_class.is_none() {
                    self.c_fns.insert(id);
                }
                if member_class.is_none() && !is_friend && ctx != Ctx::Block && !template && !self.in_template() && (is_static || self.in_unnamed_ns(sc)) {
                    self.internal_fns.insert(id);
                }
                if let Some(c) = member_class {
                    if kind == FnKind::Ctor {
                        self.classes[c].ctors.push(id);
                    }
                    if kind == FnKind::Dtor {
                        self.classes[c].dtor = Some(id);
                    }
                    if kind == FnKind::Conversion {
                        self.classes[c].has_conv = true;
                        self.classes[c].conv_targets.push(fty.ret.clone());
                    }
                    if ds.virtual_ {
                        self.classes[c].has_virtual = true;
                    }
                }
                id
            }
        };
        // ACC-7, NAM-29: a friend first declared in a class is hidden
        // from ordinary lookup until the namespace declares it.
        if in_class_body && is_friend && d.qual.is_none() {
            if let Some(c) = class {
                let v = self.class_friends.entry(c).or_default();
                if !v.contains(&id) {
                    v.push(id);
                }
                match existing {
                    None => {
                        self.hidden_friends.insert(id, vec![c]);
                    }
                    Some(_) => {
                        if let Some(h) = self.hidden_friends.get_mut(&id) {
                            h.push(c);
                        }
                    }
                }
            }
        } else if !in_class_body && existing.is_some() && d.qual.is_some() && self.hidden_friends.contains_key(&id) && self.decl_scope() != target_scope && !self.in_template() {
            // ACC-17: a qualified declaration outside the namespace does
            // not find a friend that is still hidden.
            self.error(line, "ACC-17", format!("'{}' is a friend that the namespace has not declared", name));
        } else if !in_class_body && existing.is_some() && self.scopes[target_scope].kind == SK::Namespace {
            self.hidden_friends.remove(&id);
        }
        if ds.constexpr || ds.consteval {
            if let Some(f) = self.func_mut(id) {
                f.constexpr = true;
            }
        }
        // Placeholder return type: remember how to deduce it.
        let ret_auto = if let Some(ta) = &fd.trailing_auto {
            Some(ta.clone())
        } else if ds.auto.is_some() && fd.trailing.is_none() {
            let mut d2 = d.clone();
            d2.ops.pop();
            d2.name = None;
            Some((ds.clone(), d2))
        } else {
            None
        };
        if !self.in_template() && !template {
            if let Some(p) = existing {
                // RET-4: every declaration uses the same placeholder.
                let prev_auto = self.func(p).is_some_and(|f| f.ret_auto.is_some());
                if prev_auto != ret_auto.is_some() {
                    self.error(line, "RET-4", format!("declarations of '{}' disagree on a placeholder return type", name));
                }
            }
            if ret_auto.is_some() && (ds.virtual_ || self.func(id).is_some_and(|f| f.is_virtual)) {
                // RET-5: no deduced return type on a virtual function.
                self.error(line, "RET-5", format!("virtual function '{}' with a deduced return type", name));
            }
        }
        let cmp = matches!(name.as_str(), "operator==" | "operator!=" | "operator<" | "operator>" | "operator<=" | "operator>=" | "operator<=>");
        if defaulted && cmp && !self.in_template() {
            self.check_defaulted_comparison(&name, member_class, if is_friend { class } else { None }, existing.is_some() && is_friend, template, ds, &fd, ret_auto.is_some(), &fty.ret, line);
        }
        self.record_attrs(id);
        if ret_auto.is_some() {
            if let Some(f) = self.func_mut(id) {
                f.ret_auto = ret_auto.clone();
                f.fty.ret = QTy::unknown();
            }
        }
        // virt-specifiers and pure specifier
        loop {
            if self.at("override") || self.at("final") {
                self.pos += 1;
                continue;
            }
            break;
        }
        if self.at("requires") {
            self.pos += 1;
            self.skip_constraint()?;
        }
        if let Some(c) = self.func(id).and_then(|f| f.class) {
            if kind == FnKind::Ctor || name == "operator=" {
                self.check_copy_decl(id, c, name == "operator=", defaulted, existing.is_none(), line);
            }
        }
        if self.at("=") && self.peek(1).kind == TK::Int && self.peek(1).text == "0" {
            self.pos += 2;
            if let Some(c) = member_class {
                self.classes[c].has_virtual = true;
            }
            self.pure_fns.insert(id);
            if let Some(c) = member_class {
                if !ds.virtual_ && self.classes[c].bases.is_empty() && !self.in_template() {
                    // CLS-9: only a virtual function can be pure.
                    self.error(line, "CLS-9", format!("'= 0' on '{}', which is not virtual", name));
                }
            }
            return Ok((Some(id), false));
        }
        if deleted || defaulted {
            self.pos += 2;
            self.expect(";")?;
            if let Some(f) = self.func_mut(id) {
                f.defined = true;
                f.deleted |= deleted;
                f.defaulted |= defaulted;
            }
            return Ok((Some(id), true));
        }
        if !is_def {
            return Ok((Some(id), false));
        }
        if let Some(f) = self.func_mut(id) {
            f.defined = true;
            f.template |= template;
        }
        // FN-12: complete parameter and return types in a definition.
        if !self.in_template() && !template {
            let ret = self.func(id).unwrap().fty.ret.clone();
            if let Ty::Class(c) = ret.t {
                if !self.classes[c].complete && !in_class_body {
                    self.error(line, "FN-12", "function definition with an incomplete return type");
                }
            }
            for &pid in &fd.params {
                if let Some(v) = self.var(pid) {
                    if let Ty::Class(c) = v.ty.t {
                        if !self.classes[c].complete && !in_class_body {
                            self.error(line, "FN-12", "function definition with a parameter of incomplete type");
                        }
                    }
                }
            }
        }
        // Body: deferred in a class body (NAM-6), parsed now otherwise.
        if in_class_body {
            let start = self.pos;
            self.skip_function_body()?;
            let end = self.pos;
            self.deferred.push(Deferred { func: Some(id), field: None, param: None, start, end, stack: self.stack.clone(), templ: self.templ, class: class.unwrap() });
            return Ok((Some(id), true));
        }
        let saved = self.stack.clone();
        if let Some(q) = d.qual {
            // A qualified definition sees the scope it names.
            self.push_enclosing_for(q);
        }
        let r = self.parse_function_body(id, fd.scope, member_class, ret_auto.is_some());
        self.stack = saved;
        r?;
        Ok((Some(id), true))
    }

    fn enclosing_ns_scope(&self) -> usize {
        for &s in self.stack.iter().rev() {
            if self.scopes[s].kind == SK::Namespace {
                return s;
            }
        }
        self.global
    }

    /// Push the scope `q` and the scopes that enclose it, for a qualified
    /// definition.
    pub fn push_enclosing_for(&mut self, q: usize) {
        let mut chain = Vec::new();
        let mut s = Some(q);
        while let Some(x) = s {
            if self.stack.contains(&x) {
                break;
            }
            chain.push(x);
            s = self.scopes[x].parent;
        }
        for x in chain.into_iter().rev() {
            self.stack.push(x);
        }
    }

    fn skip_function_body(&mut self) -> PR<()> {
        if self.eat("try") {
            self.skip_ctor_init();
            self.skip_balanced();
            while self.at("catch") {
                self.pos += 1;
                self.skip_balanced();
                self.skip_balanced();
            }
            return Ok(());
        }
        self.skip_ctor_init();
        if !self.at("{") {
            return Err(self.err("expected function body"));
        }
        self.skip_balanced();
        Ok(())
    }

    fn skip_ctor_init(&mut self) {
        if self.eat(":") {
            loop {
                // mem-initializer-id
                while self.is_ident() || self.at("::") || self.at("template") {
                    self.pos += 1;
                    if self.at("<") {
                        self.skip_balanced();
                    }
                }
                if self.at("(") || self.at("{") {
                    self.skip_balanced();
                } else {
                    return;
                }
                self.eat("...");
                if !self.eat(",") {
                    return;
                }
            }
        }
    }

    /// Parse the deferred bodies of a class after its definition.
    pub fn run_deferred(&mut self) {
        let list = std::mem::take(&mut self.deferred);
        for df in list {
            let save_pos = self.pos;
            let save_stack = std::mem::replace(&mut self.stack, df.stack.clone());
            let save_templ = self.templ;
            self.templ = df.templ;
            self.pos = df.start;
            if let Some(fid) = df.func {
                let (scope, auto_ret) = {
                    let f = self.func(fid).unwrap();
                    let scope = f.params.first().map(|&p| self.decls[p].scope);
                    (scope, f.ret_auto.is_some())
                };
                let scope = scope.unwrap_or_else(|| self.new_scope(SK::Params, Some(self.cur_scope()), ""));
                if let Err(e) = self.parse_function_body(fid, scope, Some(df.class), auto_ret) {
                    self.unsupported(e.line, format!("parse: {}", e.msg));
                    if let Some(f) = self.func_mut(fid) {
                        f.opaque = true;
                    }
                }
            } else if let Some(pid) = df.param {
                let line = self.line();
                // DEFARG-4: `this` in a default argument, outside sizeof and
                // decltype.
                let toks: Vec<String> = self.toks[df.start..df.end].iter().map(|t| t.text.clone()).collect();
                if toks.iter().any(|t| t == "this") && !toks.iter().any(|t| t == "sizeof" || t == "decltype" || t == "noexcept") {
                    self.error(line, "DEFARG-4", "default argument uses 'this'");
                }
                let saved_class_depth = self.class_depth;
                self.class_depth = 0;
                let r = if self.at("{") { self.parse_braced_init() } else { self.parse_assignment() };
                self.class_depth = saved_class_depth;
                match r {
                    Ok(e) => {
                        let ty = self.var(pid).map(|v| v.ty.clone()).unwrap_or(QTy::unknown());
                        let form = if matches!(e.k, EK::InitList(_)) { InitForm::CopyList } else { InitForm::Copy };
                        let ce = self.convert_init(e, &ty, form, line);
                        self.check_default_arg(&ce, line);
                        if let DK::Var(v) = &mut self.decls[pid].k {
                            v.init = Some(Rc::new(ce));
                        }
                    }
                    Err(e) => self.unsupported(e.line, format!("parse: {}", e.msg)),
                }
            } else if let Some(fid) = df.field {
                let line = self.line();
                self.dmi_depth += 1;
                let r = (|| -> PR<Expr> {
                    if self.eat("=") {
                        if self.at("{") {
                            self.parse_braced_init()
                        } else {
                            self.parse_assignment()
                        }
                    } else {
                        self.parse_braced_init()
                    }
                })();
                match r {
                    Ok(e) => {
                        let ty = match &self.decls[fid].k {
                            DK::Field(f) => f.ty.clone(),
                            _ => QTy::unknown(),
                        };
                        let form = if matches!(e.k, EK::InitList(_)) { InitForm::DirectList } else { InitForm::Copy };
                        let ce = self.convert_init(e, &ty, form, line);
                        if let DK::Field(f) = &mut self.decls[fid].k {
                            f.init = Some(Rc::new(ce));
                        }
                    }
                    Err(e) => self.unsupported(e.line, format!("parse: {}", e.msg)),
                }
                self.dmi_depth -= 1;
            }
            self.pos = save_pos;
            self.stack = save_stack;
            self.templ = save_templ;
        }
    }

    /// ATTR-6: a class whose objects can be unused without effect: no
    /// user-provided constructor or destructor, in the class, its bases,
    /// or its members.
    pub fn plain_class(&self, t: &Ty, depth: u32) -> bool {
        let Ty::Class(c) = t else { return false };
        let ci = &self.classes[*c];
        if depth > 8 || !ci.complete || self.class_uncertain(*c) || ci.key == 2 {
            return false;
        }
        let user = |d: DeclId| self.func(d).is_none_or(|f| !f.first_defaulted && !f.deleted);
        if ci.ctors.iter().any(|&d| user(d)) || ci.dtor.is_some_and(user) {
            return false;
        }
        if ci.bases.iter().any(|b| b.class.is_none_or(|bc| !self.plain_class(&Ty::Class(bc), depth + 1))) {
            return false;
        }
        // ATTR-6: the members, also of array elements.
        fn elem(t: &Ty) -> &Ty {
            match t {
                Ty::Array(e, _) => elem(&e.t),
                t => t,
            }
        }
        ci.fields.iter().all(|&f| match &self.decls[f].k {
            DK::Field(fi) => match elem(&fi.ty.t) {
                t @ Ty::Class(_) => self.plain_class(t, depth + 1),
                t => !t.is_ref() && !t.contains_unknown(),
            },
            _ => false,
        })
    }

    pub fn parse_function_body(&mut self, fid: DeclId, param_scope: usize, class: Option<usize>, deduce: bool) -> PR<()> {
        let mark = self.unused_cands.len();
        let r = self.parse_function_body_in(fid, param_scope, class, deduce);
        // ATTR-6: warn about a local variable that the program never uses
        // (with -Wunused or -Wall, as the matrix compilers do).
        let cands: Vec<(DeclId, u32)> = self.unused_cands.drain(mark..).collect();
        if self.wflag(&["-Wunused", "-Wunused-variable", "-Wunused-but-set-variable", "-Wall"]) && r.is_ok() {
            for (id, line) in cands {
                let group = self.binding_groups.get(&id).cloned().unwrap_or_else(|| vec![id]);
                let read = group.iter().any(|g| self.use_count.get(g).copied().unwrap_or(0) > self.assign_count.get(g).copied().unwrap_or(0));
                if !read {
                    self.warn(line, "ATTR-6", format!("unused variable '{}'", self.decls[id].name));
                }
            }
        }
        r
    }

    fn parse_function_body_in(&mut self, fid: DeclId, param_scope: usize, class: Option<usize>, deduce: bool) -> PR<()> {
        let (ret, kind, is_static, cv_c, cv_v, consteval) = {
            let f = self.func(fid).unwrap();
            (f.fty.ret.clone(), f.kind, f.is_static, f.fty.cv_c, f.fty.cv_v, f.consteval)
        };
        if !self.in_template() {
            // VIRT-8: a definition must not take or return an abstract
            // class by value.
            let params = self.func(fid).map(|f| f.fty.params.clone()).unwrap_or_default();
            if self.is_abstract(&ret.t) || params.iter().any(|p| self.is_abstract(&p.t)) {
                self.error(self.decls[fid].line, "VIRT-8", "function definition with a parameter or return value of an abstract class");
            }
        }
        let mut ctx = FnCtx::new(Some(ret));
        ctx.deduce = deduce;
        ctx.ret_auto = self.func(fid).and_then(|f| f.ret_auto.clone());
        ctx.ctor_dtor = matches!(kind, FnKind::Ctor | FnKind::Dtor);
        ctx.func = Some(fid);
        ctx.class = class;
        ctx.is_static = is_static;
        ctx.this_const = cv_c;
        ctx.this_volatile = cv_v;
        ctx.consteval_ctx = consteval;
        ctx.scope_base = self.stack.len();
        self.fns.push(ctx);
        self.stack.push(param_scope);
        let r = (|| -> PR<Stmt> {
            let is_try = self.eat("try");
            if self.at(":") {
                let mi = self.parse_ctor_initializers(class, fid)?;
                if let Some(f) = self.func_mut(fid) {
                    f.mem_inits = mi;
                }
            }
            let body = self.parse_compound_in(param_scope)?;
            if is_try {
                self.parse_handlers()?;
                if let Some(f) = self.func_mut(fid) {
                    f.opaque = true;
                }
            }
            Ok(body)
        })();
        self.stack.pop();
        let depth = self.fns.len();
        self.check_jumps(depth);
        let ctx = self.fns.pop().unwrap();
        let body = r?;
        // JMP-5: each goto names a label of its own function.
        for (l, line) in &ctx.gotos {
            if !ctx.labels.contains_key(l) {
                self.error(*line, "JMP-14", format!("goto names an undefined label '{}'", l));
            }
        }
        if deduce && ctx.deduced.is_none() && !self.in_template() {
            // RET-3: a reference or pointer placeholder needs a return
            // statement with an expression.
            let refptr = ctx.ret_auto.as_ref().is_some_and(|(_, d)| d.ops.iter().any(|o| matches!(o, DOp::LRef | DOp::RRef | DOp::Ptr(..))));
            if refptr {
                self.error(self.decls[fid].line, "RET-3", "reference or pointer placeholder return type without a return statement with a value");
            }
        }
        if deduce {
            let t = ctx.deduced.clone().unwrap_or(QTy::new(Ty::Void));
            if let Some(f) = self.func_mut(fid) {
                f.fty.ret = t;
            }
        }
        // ATTR-14: control should not return from a noreturn function.
        if self.has_attr(fid, "noreturn") && (has_return(&body) || can_complete(self, &body)) {
            self.warn(self.decls[fid].line, "ATTR-14", "control can return from a 'noreturn' function");
        }
        if let Some(f) = self.func_mut(fid) {
            f.body = Some(Rc::new(body));
        }
        Ok(())
    }

    fn parse_ctor_initializers(&mut self, class: Option<usize>, fid: DeclId) -> PR<Vec<MemInit>> {
        self.expect(":")?;
        let mut out = Vec::new();
        // CTOR-15, CTOR-16: what each mem-initializer names.
        let mut seen: Vec<(Option<DeclId>, Option<usize>)> = Vec::new();
        let mut delegating = false;
        let mut count = 0;
        let mut field_order: Vec<(usize, u32)> = Vec::new();
        loop {
            let line = self.line();
            // mem-initializer-id
            let scope = self.parse_nested_name()?;
            let name = self.ident()?;
            if self.at("<") {
                self.skip_template_args()?;
            }
            let target = match (scope, class) {
                (None, Some(c)) => self.lookup_member(c, &name).or_else(|| self.lookup(&name)),
                (Some(Ok(s)), _) => self.lookup_in_scope(s, &name, true),
                _ => None,
            };
            let mut args = Vec::new();
            let braced = self.at("{");
            if braced {
                args.push(self.parse_braced_init()?);
            } else {
                self.expect("(")?;
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
            }
            self.eat("...");
            count += 1;
            if target.is_none() && class.is_some_and(|c| !self.class_uncertain(c)) && !self.in_template() {
                self.error(line, "CTOR-15", format!("'{}' is not a member or base of the class", name));
            }
            if let (Some(ids), Some(c), false) = (&target, class, self.in_template()) {
                let id = ids[0];
                let named_class = match &self.decls[id].k {
                    DK::Class(b) => Some(*b),
                    DK::Type(q) => match q.t {
                        Ty::Class(b) => Some(b),
                        _ => None,
                    },
                    _ => None,
                };
                let key = match named_class {
                    Some(b) => (None, Some(b)),
                    None => (Some(id), None),
                };
                if seen.contains(&key) {
                    self.error(line, "CTOR-15", format!("'{}' is initialized two times", name));
                }
                seen.push(key);
                if self.var(id).is_some() {
                    self.error(line, "CTOR-15", format!("'{}' is a static data member", name));
                }
                if let Some(b) = named_class {
                    if b == c {
                        delegating = true;
                        // CTOR-8: not a delegation to itself.
                        if !braced && !self.class_uncertain(c) {
                            let ty = QTy::new(Ty::Class(c));
                            let e = self.ctor_call(c, &args, &ty, line);
                            if matches!(e.k, EK::Construct(t, _) if t == fid) {
                                self.error(line, "CTOR-8", "a constructor delegates to itself");
                            }
                        }
                    } else if !self.class_uncertain(c) {
                        let direct = self.classes[c].bases.iter().any(|x| x.class == Some(b));
                        if !direct && !self.has_virtual_base(c, b, 0) {
                            self.error(line, "CTOR-15", format!("'{}' is not a direct or virtual base of the class", name));
                        }
                    }
                }
            }
            // A mem-initializer for a direct member: convert its arguments
            // to the member's type, for constant evaluation.
            let field = target.as_ref().and_then(|ids| ids.iter().copied().find(|&i| matches!(&self.decls[i].k, DK::Field(fi) if Some(fi.class) == class)));
            let init = match field {
                Some(fd) if !self.in_template() => {
                    let fty = match &self.decls[fd].k {
                        DK::Field(fi) => fi.ty.clone(),
                        _ => QTy::unknown(),
                    };
                    let e = if braced {
                        Some(self.convert_init(args.remove(0), &fty, InitForm::DirectList, line))
                    } else if args.is_empty() {
                        let e = Expr::new(EK::InitList(vec![]), QTy::unknown(), Cat::PR, line);
                        Some(self.convert_init(e, &fty, InitForm::DirectList, line))
                    } else {
                        let n = args.len();
                        let e = Expr::new(EK::InitList(args), QTy::unknown(), Cat::PR, line);
                        Some(self.convert_init(e, &fty, InitForm::Paren(n), line))
                    };
                    e.map(Rc::new)
                }
                _ => None,
            };
            if let Some(fd) = field {
                if let DK::Field(fi) = &self.decls[fd].k {
                    field_order.push((fi.index, line));
                }
            }
            out.push(MemInit { field, init });
            if !self.eat(",") {
                break;
            }
        }
        // CTOR-13: warn when the list is not in the order of
        // initialization.
        if self.wflag(&["-Wall", "-Wreorder"]) && !self.in_template() {
            if let Some(w) = field_order.windows(2).find(|w| w[1].0 < w[0].0) {
                self.warn(w[1].1, "CTOR-13", "the member initializer list is not in the order of initialization");
            }
        }
        if delegating && count > 1 {
            // CTOR-16: a delegating mem-initializer stands alone.
            self.error(self.line(), "CTOR-16", "a delegating constructor has another mem-initializer");
        }
        Ok(out)
    }

    /// Whether `b` is a virtual base of `c`, at any depth (DER-6).
    pub fn has_virtual_base(&self, c: usize, b: usize, depth: u32) -> bool {
        depth < 16
            && self.classes[c].bases.iter().any(|x| match x.class {
                Some(bc) => (x.virt && bc == b) || self.has_virtual_base(bc, b, depth + 1),
                None => false,
            })
    }

    pub fn parse_handlers(&mut self) -> PR<()> {
        if !self.at("catch") {
            return Err(self.err("try block without a handler"));
        }
        while self.eat("catch") {
            self.expect("(")?;
            self.push_scope(SK::Block);
            let r = (|| -> PR<()> {
                if !self.eat("...") {
                    let ds = self.parse_decl_specs(None)?;
                    let base = self.ds_base(&ds)?;
                    let d = self.parse_declarator(true, true)?;
                    let ty = self.apply_ops(base, &d.ops, d.line);
                    if let Some(n) = d.name {
                        let vi = VarInfo { ty, storage: Storage::Auto, constexpr: false, constinit: false, init: None, param: false, local: true, dependent: false, defined: true, class: None };
                        self.add_decl(&n, DK::Var(vi), d.line);
                    }
                }
                self.expect(")")?;
                let s = self.cur_scope();
                self.parse_compound_in(s)?;
                Ok(())
            })();
            self.pop_scope();
            r?;
        }
        Ok(())
    }

    /// A scope inside an unnamed namespace (LNK rules: internal linkage).
    pub fn in_unnamed_ns(&self, mut s: usize) -> bool {
        loop {
            if s != self.global && self.scopes[s].kind == SK::Namespace && self.scopes[s].name.is_empty() {
                return true;
            }
            match self.scopes[s].parent {
                Some(p) => s = p,
                None => return false,
            }
        }
    }

    /// ODR-9, ODR-11: a function with internal linkage that the unit
    /// calls must be defined in the unit. ODR-9 needs no diagnostic. The
    /// warning-flag table of the translation page turns the ODR-11
    /// warning on with no flag and makes it an error under
    /// `-pedantic-errors`, which every test passes, so the front end
    /// rejects and names both rules.
    fn check_internal_uses(&mut self) {
        let mut seen = std::collections::HashSet::new();
        for (id, line) in self.internal_uses.clone() {
            if !seen.insert(id) {
                continue;
            }
            if self.func(id).is_some_and(|f| !f.defined && !f.deleted && !f.opaque) && !self.scopes[self.decls[id].scope].uncertain {
                self.error_because(line, "ODR-9", "ODR-11", format!("function '{}' has internal linkage and is used but not defined", self.decls[id].name));
            }
        }
    }

    fn check_main(&mut self) {
        // TR-3: a program needs main; a missing main fails at link, which a
        // front end checking one unit cannot observe.
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InitForm {
    None,
    Copy,
    CopyList,
    DirectList,
    Paren(usize),
}

/// ATTR-14: whether a statement contains a return statement.
fn has_return(s: &Stmt) -> bool {
    match s {
        Stmt::Return(_) => true,
        Stmt::Block(v) => v.iter().any(has_return),
        Stmt::If { then, els, .. } => has_return(then) || els.as_deref().is_some_and(has_return),
        Stmt::While { body, .. } | Stmt::Do { body, .. } | Stmt::For { body, .. } | Stmt::Switch { body, .. } => has_return(body),
        _ => false,
    }
}

/// ATTR-14: whether control can reach the end of a statement. The
/// analysis is simple: a throw, a call of a noreturn function, and a loop
/// without a condition or a break end control; anything else can complete.
fn can_complete(p: &P, s: &Stmt) -> bool {
    fn has_break(s: &Stmt) -> bool {
        match s {
            Stmt::Break => true,
            Stmt::Block(v) => v.iter().any(has_break),
            Stmt::If { then, els, .. } => has_break(then) || els.as_deref().is_some_and(has_break),
            _ => false,
        }
    }
    match s {
        Stmt::Block(v) => v.last().is_none_or(|l| can_complete(p, l)),
        Stmt::Expr(e) => match &e.k {
            EK::Throw(_) => false,
            EK::Call(_, _, Some(f)) => !p.has_attr(*f, "noreturn"),
            _ => true,
        },
        Stmt::For { cond: None, body, .. } => has_break(body),
        Stmt::If { then, els: Some(els), .. } => can_complete(p, then) || can_complete(p, els),
        _ => true,
    }
}

/// A variable declaration that creates an object (not `extern`).
fn storage_is_object(ds: &DeclSpec) -> bool {
    !ds.storage.contains(&Storage::Extern) && !ds.typedef
}

/// The class of a type, or of the elements of an array type.
pub fn elem_class(t: &Ty) -> Option<usize> {
    match t {
        Ty::Array(e, _) => elem_class(&e.t),
        Ty::Class(c) => Some(*c),
        _ => None,
    }
}
