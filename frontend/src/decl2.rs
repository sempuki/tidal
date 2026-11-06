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
    pub tparams: Vec<crate::templ::TParam>,
    pub depth: u32,
    pub dependent: bool,
}

impl P {
    /// Start a parse whose diagnostics are held back.
    pub fn begin_held(&mut self) -> (u32, u32, usize) {
        self.d.muted += 1;
        (self.d.muted_errors, self.d.muted_unsup, self.d.list.len())
    }

    /// End a parse that `begin_held` started. True when it had an error
    /// or an unsupported construct.
    pub fn end_held(&mut self, h: (u32, u32, usize)) -> bool {
        self.d.muted -= 1;
        let bad = self.d.muted_errors != h.0 || self.d.muted_unsup != h.1 || self.d.list.len() != h.2;
        self.d.muted_errors = h.0;
        self.d.muted_unsup = h.1;
        self.d.list.truncate(h.2);
        bad
    }

    /// TYN-9: why class `c` is not C-like, beyond a base, a member
    /// function, or a default member initializer of the class itself:
    /// a member of another kind, a lambda expression in the class, or a
    /// member class that is not C-like.
    pub fn not_c_like(&self, c: usize, depth: u32) -> Option<String> {
        if depth > 8 {
            return None;
        }
        let sc = self.classes[c].scope;
        let inside = |p: &P, mut s: usize| -> bool {
            loop {
                if s == sc {
                    return true;
                }
                match p.scopes[s].parent {
                    Some(q) => s = q,
                    None => return false,
                }
            }
        };
        if self.classes.iter().any(|k| k.name == "(lambda)" && self.scopes[k.scope].parent.is_some_and(|q| inside(self, q))) {
            return Some("it has a lambda expression".into());
        }
        let mut names: Vec<(&String, &Vec<DeclId>)> = self.scopes[sc].names.iter().collect();
        names.sort_by_key(|x| x.0.clone());
        for (n, ids) in names {
            for &d in ids {
                if self.decls[d].scope != sc || self.decls[d].line == 0 {
                    continue;
                }
                match &self.decls[d].k {
                    DK::Field(_) | DK::Enum(_) | DK::Enumerator(..) => {}
                    DK::Class(k) if *k == c => {}
                    DK::Class(k) => {
                        {
                            if !self.classes[*k].bases.is_empty() || self.classes[*k].fields.iter().any(|&f| matches!(&self.decls[f].k, DK::Field(fi) if fi.init.is_some())) {
                                return Some(format!("the member class '{}' has a base or a default member initializer", n));
                            }
                            if let Some(why) = self.not_c_like(*k, depth + 1) {
                                return Some(format!("the member class '{}' is not C-like: {}", n, why));
                            }
                        }
                    }
                    DK::Func(f) if f.defaulted || f.class != Some(c) => {}
                    DK::Func(_) => return Some(format!("'{}' is a member function", n)),
                    DK::Type(_) => return Some(format!("'{}' is a member alias", n)),
                    DK::Var(_) => return Some(format!("'{}' is a static data member", n)),
                    _ => return Some(format!("'{}' is not a data member, a member class, or a member enumeration", n)),
                }
            }
        }
        None
    }

    pub fn parse_translation_unit(&mut self) {
        while self.tok().kind != TK::Eof {
            if self.at("}") {
                self.error(self.line(), "TR-12", "unexpected '}'");
                self.pos += 1;
                continue;
            }
            self.parse_declaration_recover(Ctx::Namespace);
        }
        self.finish_lazy();
        self.check_main();
        self.check_internal_uses();
    }

    pub fn parse_declaration_recover(&mut self, ctx: Ctx) -> Option<Stmt> {
        let start = self.pos;
        self.pending_attrs.clear();
        match self.parse_declaration(ctx) {
            Ok(s) => s,
            Err(e) => {
                self.parse_failure(&e);
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
        let mark = self.pack_uses.len();
        let r = self.parse_declaration_inner(ctx);
        self.check_pack_uses(mark);
        self.decl_attrs = saved;
        r
    }

    fn parse_declaration_inner(&mut self, ctx: Ctx) -> PR<Option<Stmt>> {
        let line = self.line();
        // A statement read the attributes, alignas among them, already.
        if self.align_end != self.pos {
            self.last_align = 0;
            self.saw_alignas = false;
        }
        self.align_end = usize::MAX;
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
        if matches!(ctx, Ctx::Namespace | Ctx::Class(_)) {
            if let Some((tid, k)) = self.at_guide() {
                // A deduction guide (CTAD-11).
                self.parse_guide(tid, k, ctx)?;
                return Ok(Some(Stmt::Null));
            }
        }
        if matches!(ctx, Ctx::Namespace | Ctx::Class(_)) && self.abbrev.as_ref().is_none_or(|a| a.closed) && self.pending_template.is_none() && !matches!(self.inst, Some(crate::templ::InstReq::Func { .. })) && self.looks_abbreviated() {
            // DED-34: an abbreviated function template.
            self.parse_abbreviated(ctx)?;
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
            "extern" if self.at_k(1, "template") && !self.at_k(2, "<") && !self.in_template() && self.inst_depth == 0 => {
                // An explicit instantiation declaration (INST-27).
                self.pos += 2;
                self.access_free += 1;
                let r = self.parse_explicit_inst(ctx, true, line);
                self.access_free -= 1;
                r?;
                return Ok(Some(Stmt::Null));
            }
            "export" | "module" | "import" => {
                return Err(self.err("modules (outside the specification)"));
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
                        // NS-14: an alias in the same scope names the same
                        // namespace.
                        let cur = self.decl_scope();
                        let clash = self.scopes[cur].names.get(&name).cloned().unwrap_or_default().into_iter().any(|d| match self.decls[d].k {
                            DK::Namespace(o) => o != s,
                            _ => true,
                        });
                        if clash {
                            self.error(line, "NS-14", format!("'{}' already declared, not as an alias for the same namespace", name));
                        }
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
                    if self.scopes[s].parent != Some(cur) || self.scopes[s].name != *name {
                        // NS-14: an alias does not reopen its namespace.
                        self.error(line, "NS-14", format!("'{}' is a namespace alias, and a namespace definition cannot use it", name));
                    }
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

    /// LSP-4, LSP-7, LSP-2, TR-3: a C name is one entity across
    /// namespaces. The declarations of one C name agree in kind and type,
    /// a C name is not `main`, and it does not have the spelling of a
    /// global variable with C++ linkage.
    fn check_c_name(&mut self, id: DeclId, name: &str, line: u32) {
        if name == "main" {
            let what = if matches!(self.decls[id].k, DK::Func(_)) { "function" } else { "variable" };
            self.error(line, "TR-3", format!("a {} named 'main' with C language linkage", what));
            return;
        }
        let ty_of = |p: &Self, d: DeclId| -> Option<(bool, Ty)> {
            match &p.decls[d].k {
                DK::Func(f) => Some((true, Ty::Func(Box::new(FnTy { noexcept: false, ..f.fty.clone() })))),
                DK::Var(v) => Some((false, v.ty.t.clone())),
                _ => None,
            }
        };
        let Some((is_fn, t)) = ty_of(self, id) else { return };
        let others: Vec<DeclId> = self.c_names.get(name).cloned().unwrap_or_default();
        for o in others {
            if o == id {
                continue;
            }
            let Some((ofn, ot)) = ty_of(self, o) else { continue };
            if t.contains_unknown() || ot.contains_unknown() {
                continue;
            }
            if ofn != is_fn {
                self.error(line, "LSP-7", format!("the C name '{}' names a function and a variable", name));
                return;
            }
            let same = if is_fn { t == ot } else { strip_array_bound(&t) == strip_array_bound(&ot) };
            if !same {
                self.error(line, "LSP-7", format!("declarations of the C name '{}' with different types", name));
                return;
            }
        }
        if !is_fn {
            // LSP-7: a global variable with C++ linkage of the same name.
            let globals: Vec<DeclId> = self.scopes[self.global].names.get(name).cloned().unwrap_or_default();
            if globals.iter().any(|&g| g != id && matches!(self.decls[g].k, DK::Var(_)) && !self.c_names.get(name).is_some_and(|v| v.contains(&g)) && !self.var_internal.contains(&g)) {
                self.error(line, "LSP-7", format!("the C name '{}' has the spelling of a global variable with C++ linkage", name));
                return;
            }
        }
        self.c_names.entry(name.to_string()).or_default().push(id);
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
        // LSP-1: the innermost linkage specification gives the language.
        let saved_lang = self.lang_c;
        if c {
            self.lang_c += 1;
        } else {
            self.lang_c = 0;
        }
        let r = (|| {
            if self.eat("{") {
                let save = std::mem::replace(&mut self.lang_c_single, false);
                let save_single = std::mem::replace(&mut self.link_single, false);
                while !self.at("}") && self.tok().kind != TK::Eof {
                    self.parse_declaration_recover(Ctx::Namespace);
                }
                self.lang_c_single = save;
                self.link_single = save_single;
                self.expect("}")?;
            } else {
                let save = std::mem::replace(&mut self.lang_c_single, c);
                let save_single = std::mem::replace(&mut self.link_single, true);
                let r = self.parse_declaration(Ctx::Namespace);
                self.lang_c_single = save;
                self.link_single = save_single;
                r?;
            }
            Ok(())
        })();
        self.lang_c = saved_lang;
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
                Ty::Unknown | Ty::Dep(_) => {}
                _ => self.error(line, "ENUM-20", "using enum must name an enumeration"),
            }
            return Ok(());
        }
        // alias declaration: using name [attrs] = type-id ;
        if self.is_ident() && (self.at_k(1, "=") || (self.at_k(1, "[") && self.at_k(2, "["))) {
            let name = self.ident()?;
            let a0 = self.pos;
            self.skip_attributes();
            // ATTR-2: `nodiscard` and `noreturn` do not go on a type alias.
            for k in a0..self.pos {
                let t = &self.toks[k];
                if t.kind == TK::Ident && matches!(t.text.as_str(), "nodiscard" | "noreturn") && !(k > 0 && self.toks[k - 1].text == "::") && self.toks.get(k + 1).is_some_and(|n| n.text != "::") {
                    let msg = format!("'{}' on a type alias", t.text);
                    self.error(line, "ATTR-2", msg);
                }
            }
            self.expect("=")?;
            self.implicit_tn = true;
            let q = self.parse_type_id()?;
            self.expect(";")?;
            if let Some(crate::templ::InstReq::Alias) = &self.inst {
                // TPL-28: the specialization is the type the alias gives.
                self.inst = None;
                self.inst_out = Some(crate::templ::InstOut::Type(q));
                return Ok(());
            }
            if let Some(pt) = self.pending_template.take() {
                let _ = pt;
                let cur = self.decl_scope();
                let id = self.add_decl_in(cur, &name, DK::Template(TemplKind::Alias), line);
                self.last_templ = Some(crate::templ::Last::Alias(id));
                return Ok(());
            }
            self.declare_typedef(&name, q, line, ctx);
            return Ok(());
        }
        // using-declaration
        if ctx == Ctx::Init {
            self.error(line, "IF-21", "using-declaration as an init-statement");
        }
        if let Ctx::Class(cc) = ctx {
            // `using Base<...>::Base;` inherits constructors (CTAD-16).
            let n0 = self.tok().text.clone();
            let mut k = 1;
            if self.at_k(1, "<") {
                k = self.skip_angle_at(1);
            }
            if self.at_k(k, "::") && self.peek(k + 1).text == n0 && self.at_k(k + 2, ";") {
                self.inherits_ctors.entry(cc).or_default().push(n0);
            }
        }
        loop {
            let typename = self.eat("typename");
            // PACK-21: `using Bs::f...;` names the member in each base.
            let first = self.tok().text.clone();
            let single = self.at_k(1, "::") && self.peek(2).kind == TK::Ident && !self.at_k(3, "::") && !self.at_k(3, "<");
            let (items, _) = self.parse_expansion(|p| {
                let scope = p.parse_nested_name()?;
                if p.eat("operator") {
                    return Ok((scope, None));
                }
                let mut name = p.ident()?;
                if p.eat("~") {
                    name = p.ident()?;
                }
                // DER-8: `using B::B;` where B names a class through a
                // parameter or an alias inherits its constructors.
                if single && name == first {
                    if let Some(Ok(s)) = scope {
                        if let SK::Class(c) = p.scopes[s].kind {
                            name = p.classes[c].name.clone();
                        }
                    }
                }
                Ok((scope, Some(name)))
            })?;
            if items.iter().any(|x| x.1.is_none()) {
                // A using declaration of an operator function.
                self.recover_until_semicolon();
                return Ok(());
            }
            self.eat("...");
            for (scope, name) in items {
            let name = name.unwrap_or_default();
            if scope.is_none() {
                // NS-7: a using declaration needs a qualified name.
                self.error(line, "NS-16", "using declaration without a qualified name");
            }
            match scope {
                Some(Ok(s)) => {
                    let is_class_scope = matches!(self.scopes[s].kind, SK::Class(_));
                    if is_class_scope && ctx == Ctx::Namespace {
                        self.error(line, "NS-9", "using declaration at namespace scope names a class member");
                    }
                    if let (SK::Class(c), Ctx::Class(cc)) = (self.scopes[s].kind.clone(), ctx) {
                        if !self.in_template() && !self.class_uncertain(cc) && !self.class_uncertain(c) && c != cc {
                            // DER-9: the named class is a base, and a direct
                            // base for inherited constructors.
                            let direct = self.classes[cc].bases.iter().any(|b| b.class == Some(c));
                            if self.classes[c].name == name && !direct {
                                self.error(line, "DER-9", format!("'{}' is not a direct base of '{}', so its constructors are not inherited", self.classes[c].name, self.classes[cc].name));
                            } else if !self.is_base_of(c, cc) {
                                self.error(line, "DER-9", format!("'{}' is not a base of '{}'", self.classes[c].name, self.classes[cc].name));
                            }
                        }
                    }
                    if let SK::Class(c) = self.scopes[s].kind {
                        if self.classes[c].name == name {
                            // DER-8: inherited constructors, modeled when
                            // the base is a complete direct base whose
                            // constructors are not templates.
                            if let Ctx::Class(cc) = ctx {
                                let ok = !self.in_template()
                                    && !self.class_uncertain(cc)
                                    && !self.class_uncertain(c)
                                    && self.classes[c].complete
                                    && !self.classes[c].opaque
                                    && self.classes[cc].bases.iter().any(|b| b.class == Some(c))
                                    && self.classes[c].ctors.iter().all(|&i| self.func(i).is_some_and(|f| !f.template));
                                if ok {
                                    self.inh_bases.entry(cc).or_default().push(c);
                                } else {
                                    self.classes[cc].opaque = true;
                                }
                            }
                        }
                    }
                    match self.lookup_in_scope(s, &name, true) {
                        Some(ids) => {
                            if ids.iter().any(|&i| matches!(self.decls[i].k, DK::Namespace(_))) {
                                self.error(line, "NS-16", "using declaration names a namespace");
                            }
                            if let (SK::Class(c), Ctx::Class(cc)) = (self.scopes[s].kind.clone(), ctx) {
                                // ACC-13: the named base member must be
                                // accessible in the derived class.
                                if c != cc && !self.in_template() && !self.class_uncertain(c) && self.classes[c].name != name && !ids.is_empty() && ids.iter().all(|&i| !self.accessible(i, true)) {
                                    self.error(line, "ACC-13", format!("the using declaration names '{}', which is not accessible in '{}'", name, self.classes[cc].name));
                                }
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
                            // ACC-2: the members that a using-declaration
                            // names in a class take the access of the
                            // using-declaration.
                            if let (SK::Class(cc), Some(a)) = (self.scopes[cur].kind.clone(), self.access_stack.last().copied()) {
                                for &i in &ids {
                                    self.using_access.insert((cc, i), a);
                                }
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
                _ if self.in_template() && matches!(ctx, Ctx::Class(_)) && single && name == first => {
                    // DER-8: the constructors of a dependent base.
                }
                _ if self.in_template() && matches!(ctx, Ctx::Class(_)) && !name.is_empty() => {
                    // DEP-23: a member of a dependent base. With `typename`
                    // the name is a type, without it a value (DEP-19). A
                    // specialization looks the name up again.
                    let cur = self.decl_scope();
                    if typename {
                        self.add_decl_in(cur, &name, DK::Type(crate::templ::dep(Dep::Opaque)), line);
                    } else {
                        let vi = VarInfo { ty: QTy::unknown(), storage: Storage::Static, constexpr: false, constinit: false, init: None, param: false, local: false, dependent: true, defined: true, class: None };
                        let id = self.add_decl_in(cur, &name, DK::Var(vi), line);
                        self.dep_using.insert(id);
                    }
                }
                _ => {
                    let cur = self.decl_scope();
                    self.scopes[cur].uncertain = true;
                }
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
                    DK::Class(c) if Ty::Class(*c) == q.t => {
                        self.typedef_named.insert(p);
                        return;
                    }
                    DK::Enum(e) if Ty::Enum(*e) == q.t => {
                        self.typedef_named.insert(p);
                        return;
                    }
                    _ => {
                        self.error(line, "TYN-2", format!("'{}' already declared as something else", name));
                        return;
                    }
                }
            }
        }
        // TYN-3: a typedef names an unnamed class.
        if let Ty::Class(c) = q.t {
            if self.classes[c].name.is_empty() && !self.in_template() && self.classes[c].complete {
                // TYN-9: the class is C-like.
                let sc = self.classes[c].scope;
                let has_fn = self.scopes[sc].names.values().flatten().any(|&d| self.func(d).is_some_and(|f| f.class == Some(c) && !f.defaulted) && self.decls[d].line != 0);
                let has_init = self.classes[c].fields.iter().any(|&f| matches!(&self.decls[f].k, DK::Field(fi) if fi.init.is_some()) || self.deferred.iter().any(|d| d.field == Some(f)));
                if !self.classes[c].bases.is_empty() || has_fn || has_init {
                    self.error(line, "TYN-9", "a class named for linkage by a typedef has a base, a member function, or a default member initializer");
                } else if let Some(why) = self.not_c_like(c, 0) {
                    self.error(line, "TYN-9", format!("a class named for linkage by a typedef is not C-like: {}", why));
                }
            }
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

    /// The parameters of a template head, from `<` to `>`, declared in
    /// the current scope with depth `depth`.
    pub fn parse_tparams(&mut self, depth: u32) -> PR<Vec<crate::templ::TParam>> {
        use crate::templ::{TParam, TPK};
        self.expect("<")?;
        let mut params: Vec<TParam> = Vec::new();
        if self.eat(">") {
            return Ok(params);
        }
        loop {
            let pl = self.line();
            let idx = params.len() as u32;
            if (self.at("class") || self.at("typename")) && !(self.peek(1).kind == TK::Ident && !is_keyword(&self.peek(1).text) && self.at_k(2, "::")) {
                self.pos += 1;
                let pack = self.eat("...");
                let name = if self.is_ident() { self.ident()? } else { String::new() };
                self.check_tparam_name(&name, &params, pl);
                let id = self.add_decl(&name, DK::TypeParam, pl);
                self.tparam.insert(id, (depth, idx));
                if pack {
                    self.pack_decls.insert(id);
                }
                let mut default = None;
                if self.eat("=") {
                    let st = self.pos;
                    self.no_gt += 1;
                    self.implicit_tn = true;
                    let r = self.parse_type_id();
                    self.no_gt -= 1;
                    r?;
                    default = Some(self.copy_toks(st, self.pos));
                    if pack {
                        self.error(pl, "TPL-34", "a template parameter pack has a default argument");
                    }
                }
                params.push(TParam { name, kind: TPK::Type, pack, default, line: pl, dauto: false, pattern: None });
            } else if self.at("template") {
                self.pos += 1;
                let inner_scope = self.push_scope(SK::Template);
                let _ = inner_scope;
                let outer_cons = self.tparam_cons.take();
                let inner = self.parse_tparams(u32::MAX - 2);
                let inner_cons = std::mem::replace(&mut self.tparam_cons, outer_cons);
                // A requires clause on the parameter's head (CPT-8).
                let inner = match inner {
                    Ok(v) if self.at("requires") => {
                        self.pos += 1;
                        let saved = std::mem::take(&mut self.cons_unmodeled);
                        let r = self.cons_clause(false);
                        self.cons_unmodeled = saved;
                        r.map(|_| v)
                    }
                    x => x,
                };
                self.pop_scope();
                let inner = inner?;
                self.pos += 1; // class/typename
                let pack = self.eat("...");
                let name = if self.is_ident() { self.ident()? } else { String::new() };
                self.check_tparam_name(&name, &params, pl);
                let id = self.add_decl(&name, DK::Template(TemplKind::Class(usize::MAX)), pl);
                self.tparam.insert(id, (depth, idx));
                if pack {
                    self.pack_decls.insert(id);
                }
                self.ttp_inner.insert(id, std::rc::Rc::new(inner.clone()));
                let mut default = None;
                if self.eat("=") {
                    let st = self.pos;
                    self.skip_qualified_name()?;
                    default = Some(self.copy_toks(st, self.pos));
                    if pack {
                        self.error(pl, "TPL-34", "a template parameter pack has a default argument");
                    }
                }
                let inner = std::rc::Rc::new(inner);
                if let Some(n) = inner_cons {
                    self.ttp_cons.insert(std::rc::Rc::as_ptr(&inner) as usize, n);
                }
                params.push(TParam { name, kind: TPK::Templ(inner), pack, default, line: pl, dauto: false, pattern: None });
            } else if let Some((ctid, nlen)) = self.concept_at() {
                // A type constraint (CPT-17 to CPT-21).
                self.tparam_constrained = true;
                let cstart = self.pos;
                self.pos += nlen;
                let mut extra = Vec::new();
                let mut lt = None;
                if self.at("<") {
                    lt = Some(self.pos);
                    extra = self.parse_raw_targs()?;
                }
                let cend = self.pos;
                let placeholder = self.at("auto") || (self.at("decltype") && self.at_k(1, "(") && self.at_k(2, "auto"));
                let param = if placeholder {
                    // A constant parameter with a constrained placeholder.
                    let dauto = self.at("decltype");
                    self.pos += if dauto { 4 } else { 1 };
                    let d = self.parse_declarator(true, true)?;
                    let name = d.name.clone().unwrap_or_default();
                    self.check_tparam_name(&name, &params, pl);
                    let mut default = None;
                    if self.eat("=") {
                        let st = self.pos;
                        self.templ += 1;
                        self.no_gt += 1;
                        let e = self.parse_conditional();
                        self.no_gt -= 1;
                        self.templ -= 1;
                        e?;
                        default = Some(self.copy_toks(st, self.pos));
                    }
                    let vi = VarInfo { ty: QTy::unknown(), storage: Storage::Auto, constexpr: false, constinit: false, init: None, param: true, local: false, dependent: true, defined: true, class: None };
                    let id = self.add_decl(&name, DK::Var(vi), pl);
                    self.tparam.insert(id, (depth, idx));
                    if d.pack {
                        self.pack_decls.insert(id);
                    }
                    TParam { name, kind: TPK::Auto, pack: d.pack, default, line: pl, dauto, pattern: None }
                } else {
                    let pack = self.eat("...");
                    let name = if self.is_ident() { self.ident()? } else { String::new() };
                    self.check_tparam_name(&name, &params, pl);
                    let id = self.add_decl(&name, DK::TypeParam, pl);
                    self.tparam.insert(id, (depth, idx));
                    if pack {
                        self.pack_decls.insert(id);
                    }
                    let mut default = None;
                    if self.eat("=") {
                        let st = self.pos;
                        self.no_gt += 1;
                        self.implicit_tn = true;
                        let r = self.parse_type_id();
                        self.no_gt -= 1;
                        r?;
                        default = Some(self.copy_toks(st, self.pos));
                        if pack {
                            self.error(pl, "TPL-34", "a template parameter pack has a default argument");
                        }
                    }
                    TParam { name, kind: TPK::Type, pack, default, line: pl, dauto: false, pattern: None }
                };
                params.push(param.clone());
                {
                    let n = self.type_constraint_norm(ctid, extra, &param, depth, idx as usize, &params, (cstart, cstart + nlen, lt, cend), pl);
                    self.tparam_cons = crate::cons::and(self.tparam_cons.take(), n);
                }
            } else {
                // TPL-11: a class template name alone is a type here.
                self.in_tparam_decl = true;
                self.implicit_tn = true;
                let ds = self.parse_decl_specs(None);
                self.in_tparam_decl = false;
                let ds = ds?;
                let auto = ds.auto.is_some();
                let base = self.ds_base(&ds).unwrap_or(QTy::unknown());
                let d = self.parse_declarator(true, true)?;
                let mut ty = self.apply_ops(base, &d.ops, pl);
                // TPL-8: an array or function type becomes a pointer.
                ty = match &ty.t {
                    Ty::Array(el, _) => QTy::new(Ty::Ptr(el.clone())),
                    Ty::Func(_) => QTy::new(Ty::Ptr(Box::new(ty.clone()))),
                    _ => ty,
                };
                // TPL-32: top-level const and volatile are dropped.
                if !ty.t.is_ref() {
                    ty.c = false;
                    ty.v = false;
                }
                let name = d.name.clone().unwrap_or_default();
                self.check_tparam_name(&name, &params, pl);
                if (!ds.storage.is_empty() || ds.defined_class.is_some() || ds.defined_enum.is_some()) && self.inst_depth == 0 {
                    // TPL-33: no storage class, and no class or enumeration
                    // defined.
                    self.error(pl, "TPL-33", "a constant template parameter with a storage class or a type definition");
                }
                if !auto && !ty.is_unknown() && self.inst_depth == 0 && !self.tparam_type_ok(&ty) {
                    self.error(pl, "TPL-7", "the type of a constant template parameter is not a structural type");
                }
                let mut default = None;
                if self.eat("=") {
                    let st = self.pos;
                    self.templ += 1;
                    self.no_gt += 1;
                    let e = self.parse_conditional();
                    self.no_gt -= 1;
                    self.templ -= 1;
                    e?;
                    default = Some(self.copy_toks(st, self.pos));
                    if d.pack {
                        self.error(pl, "TPL-34", "a template parameter pack has a default argument");
                    }
                }
                let vi = VarInfo { ty: ty.clone(), storage: Storage::Auto, constexpr: false, constinit: false, init: None, param: true, local: false, dependent: true, defined: true, class: None };
                let id = self.add_decl(&name, DK::Var(vi), pl);
                self.tparam.insert(id, (depth, idx));
                if d.pack {
                    self.pack_decls.insert(id);
                }
                // DED-12: a placeholder with declarator operators, as in `auto*`,
                // is a pattern for the type of the argument.
                let pattern = if auto && !d.ops.is_empty() && ds.auto == Some(false) { Some(ty.clone()) } else { None };
                let kind = if auto || ty.is_unknown() && ds.auto.is_some() { TPK::Auto } else { TPK::Const(ty) };
                params.push(TParam { name, kind, pack: d.pack, default, line: pl, dauto: ds.auto == Some(true), pattern });
            }
            if self.eat(",") {
                continue;
            }
            self.expect(">")?;
            return Ok(params);
        }
    }

    fn parse_template(&mut self, ctx: Ctx) -> PR<()> {
        // INST-34, TSP-29: an explicit instantiation or explicit
        // specialization does not check access in the names it uses.
        let free = !self.at_k(1, "<") || self.at_k(2, ">");
        if free {
            self.access_free += 1;
        }
        let r = self.parse_template_inner(ctx);
        if free {
            self.access_free -= 1;
        }
        r
    }

    /// TSP-26: after the first `template <>`, the number of `template <>`
    /// heads and the number of class template-ids in the qualifier of the
    /// declarator of a function. None when the declaration is not such a
    /// function.
    fn tsp26_counts(&self) -> Option<(usize, usize)> {
        let mut k = self.pos;
        let mut heads = 1;
        while self.toks.get(k).is_some_and(|t| t.text == "template") && self.toks.get(k + 1).is_some_and(|t| t.text == "<") && self.toks.get(k + 2).is_some_and(|t| t.text == ">") {
            heads += 1;
            k += 3;
        }
        if self.toks.get(k).is_some_and(|t| t.text == "template") {
            // A head with parameters follows; a member template stays a
            // template.
            return None;
        }
        // The first `(` at depth 0, before any `{`, `;`, or `=`.
        let mut depth = 0i32;
        let mut open = None;
        let mut j = k;
        while j < self.toks.len() {
            let t = self.toks[j].text.as_str();
            match t {
                "<" | "[" => depth += 1,
                ">" | "]" => depth -= 1,
                ">>" => depth -= 2,
                "(" if depth == 0 => {
                    open = Some(j);
                    break;
                }
                "(" => {}
                "{" | ";" | "=" if depth == 0 => return None,
                "operator" => return None,
                _ => {}
            }
            j += 1;
        }
        let open = open?;
        // Walk back over the declarator's qualified name.
        let mut fixed = 0;
        let mut j = open;
        let skip_args_back = |j: usize| -> Option<usize> {
            // `j` is just past a `>`; return the position of its `<`.
            let mut d = 0i32;
            let mut i = j;
            while i > k {
                i -= 1;
                match self.toks[i].text.as_str() {
                    ">" => d += 1,
                    ">>" => d += 2,
                    "<" => {
                        d -= 1;
                        if d == 0 {
                            return Some(i);
                        }
                    }
                    _ => {}
                }
            }
            None
        };
        // The final name, with template arguments or not.
        if j > k && matches!(self.toks[j - 1].text.as_str(), ">" | ">>") {
            j = skip_args_back(j)?;
        }
        if j == k || self.toks[j - 1].kind != TK::Ident {
            return None;
        }
        j -= 1;
        while j >= k + 2 && self.toks[j - 1].text == "::" {
            j -= 1;
            if matches!(self.toks[j - 1].text.as_str(), ">" | ">>") {
                j = skip_args_back(j)?;
                fixed += 1;
            }
            if j == k || self.toks[j - 1].kind != TK::Ident {
                return None;
            }
            j -= 1;
        }
        Some((heads, fixed))
    }

    fn parse_template_inner(&mut self, ctx: Ctx) -> PR<()> {
        let line = self.line();
        self.expect("template")?;
        if !self.at("<") {
            // An explicit instantiation (INST-16 to INST-29).
            if self.in_template() || self.inst_depth > 0 {
                self.templ += 1;
                let r = self.parse_declaration(ctx);
                self.templ -= 1;
                r?;
                return Ok(());
            }
            return self.parse_explicit_inst(ctx, false, line);
        }
        let specialization = self.at_k(1, ">");
        if specialization {
            // An explicit specialization is an ordinary declaration of
            // one specialization (TSP-1, TSP-2).
            self.pos += 2;
            if let Ctx::Block = ctx {
                self.error(line, "TSP-5", "explicit specialization in a block");
                self.recover();
                return Ok(());
            }
            if self.inst_depth == 0 && matches!(ctx, Ctx::Namespace) {
                if self.templ > 0 {
                    // TSP-26: at namespace scope, `template <>` after the
                    // head of an enclosing template leaves that template
                    // unspecialized.
                    self.error(line, "TSP-26", "an explicit specialization of a member must fix every enclosing class template");
                } else if let Some((heads, fixed)) = if self.explicit_spec { None } else { self.tsp26_counts() } {
                    if heads < fixed {
                        // TSP-26: one `template <>` for each class template
                        // that the qualifier fixes.
                        self.error(line, "TSP-26", format!("the explicit specialization fixes {} class templates and has {} 'template <>'", fixed, heads));
                    }
                }
            }
            if self.lang_c > 0 && self.inst_depth == 0 {
                // TPL-3: no explicit specialization with C language linkage.
                self.error(line, "TPL-3", "an explicit specialization with C language linkage");
            }
            let saved_spec = self.explicit_spec;
            let saved_last = self.last_templ.take();
            self.explicit_spec = true;
            let in_dep = self.templ > 0;
            if in_dep {
                self.templ += 1;
            }
            let r = self.parse_declaration(ctx);
            if in_dep {
                self.templ -= 1;
            }
            self.pending_template = None;
            self.explicit_spec = saved_spec;
            self.last_templ = saved_last;
            r?;
            return Ok(());
        }
        let depth = self.tdepth;
        let scope = self.push_scope(SK::Template);
        let _ = scope;
        let r = self.parse_tparams(depth);
        let tparams = match r {
            Ok(v) => v,
            Err(e) => {
                self.pop_scope();
                return Err(e);
            }
        };
        let params: Vec<String> = tparams.iter().map(|p| p.name.clone()).collect();
        let had_type_constraint = std::mem::take(&mut self.tparam_constrained);
        // CNS-1: the type constraints of the parameters, then the requires
        // clause.
        let saved_unmodeled = std::mem::take(&mut self.cons_unmodeled);
        let mut head_cons = self.tparam_cons.take();
        let mut parts = (head_cons.clone(), None);
        let mut had_clause = false;
        if self.at("requires") {
            self.pos += 1;
            had_clause = true;
            let saved_head = self.cons_head.replace((Rc::new(tparams.clone()), depth));
            let c = self.cons_clause(false);
            self.cons_head = saved_head;
            match c {
                Ok(c) => {
                    parts.1 = c.clone();
                    head_cons = crate::cons::and(head_cons, c)
                }
                Err(e) => {
                    self.cons_unmodeled = saved_unmodeled;
                    self.pop_scope();
                    return Err(e);
                }
            }
        }
        let constrained = std::mem::replace(&mut self.cons_unmodeled, saved_unmodeled);
        if self.at("concept") || (self.at("friend") && self.at_k(1, "concept")) {
            let r = self.parse_concept_def(ctx, tparams, depth, had_type_constraint || had_clause, line);
            self.pop_scope();
            return r;
        }
        if let Ctx::Block = ctx {
            // Templates cannot be declared in a block.
            self.pop_scope();
            self.error(line, "FN-9", "template declaration in a block");
            self.recover();
            return Ok(());
        }
        let dependent = self.templ > 0;
        let body_start = self.pos;
        let mut friend_default = false;
        if self.inst_depth == 0 {
            if self.lang_c > 0 {
                // TPL-3: no template with C language linkage.
                self.error(line, "TPL-3", "a template with C language linkage");
            }
            if let Ctx::Class(c) = ctx {
                if self.class_is_local(c) {
                    // TPL-2: no template in a local class.
                    self.error(line, "TPL-2", "a template declared in a local class");
                }
            }
            if self.at("friend") || (self.at_k(1, "friend") && !self.at("friend")) {
                let def_fn = self.friend_template_defines_function();
                friend_default = def_fn && tparams.iter().any(|p| p.default.is_some());
                if tparams.iter().any(|p| p.default.is_some()) && !def_fn {
                    // TPL-37: a friend declaration of a template gives no
                    // default, unless it defines a function template.
                    self.error(line, "TPL-37", "a friend template declaration gives a default template argument");
                }
            }
        }
        let saved_ft = self.friend_templ.take();
        let saved_last = self.last_templ.take();
        let saved_hc = std::mem::replace(&mut self.head_constrained, constrained);
        let saved_cons = std::mem::replace(&mut self.head_cons, head_cons.clone());
        let saved_parts = std::mem::replace(&mut self.head_parts, parts.clone());
        // DED-34: placeholder parameters add invented parameters.
        let saved_abbrev = if self.looks_abbreviated() {
            Some(self.abbrev.replace(crate::cons::Abbrev { depth, base: tparams.len(), params: vec![], cons: None, closed: false }))
        } else {
            None
        };
        self.pending_template = Some(PendingTemplate { params, specialization: false, tparams: tparams.clone(), depth, dependent });
        self.templ += 1;
        self.tdepth += 1;
        let r = self.parse_declaration(ctx);
        self.tdepth -= 1;
        self.templ -= 1;
        self.pending_template = None;
        self.pop_scope();
        self.head_cons = saved_cons;
        self.head_parts = saved_parts;
        let mut tparams = tparams;
        let mut head_cons = head_cons;
        if let Some(sa) = saved_abbrev {
            if let Some(ab) = std::mem::replace(&mut self.abbrev, sa) {
                tparams.extend(ab.params);
                parts.0 = crate::cons::and(parts.0, ab.cons.clone());
                head_cons = crate::cons::and(head_cons, ab.cons);
            }
        }
        let last = std::mem::replace(&mut self.last_templ, saved_last);
        r?;
        let friend_id = std::mem::replace(&mut self.friend_templ, saved_ft);
        if self.inst_depth == 0 {
            match (&last, friend_id) {
                (_, Some(id)) => self.check_friend_default(id, &tparams, friend_default, line),
                (Some(crate::templ::Last::Func(id)), None) => self.check_friend_default(*id, &tparams, false, line),
                _ => {}
            }
        }
        let body_end = self.pos;
        // A member template defined outside its class: the enclosing
        // head records the whole declaration (TPL-24).
        let outer = match (&last, dependent) {
            (Some(crate::templ::Last::Member(t, p)), true) => Some(crate::templ::Last::Member(*t, p.clone())),
            _ => None,
        };
        self.head_constrained = constrained;
        let saved_cons = std::mem::replace(&mut self.head_cons, head_cons);
        let saved_parts = std::mem::replace(&mut self.head_parts, parts);
        self.record_template(last, tparams, depth, dependent, body_start, body_end, ctx, line);
        self.head_parts = saved_parts;
        self.head_cons = saved_cons;
        self.head_constrained = saved_hc;
        if let Some(o) = outer {
            self.last_templ = Some(o);
        }
        Ok(())
    }

    /// An abbreviated function template: a function declaration with
    /// placeholder parameters and no template head (DED-34).
    fn parse_abbreviated(&mut self, ctx: Ctx) -> PR<()> {
        let line = self.line();
        let depth = self.tdepth;
        let dependent = self.templ > 0;
        self.push_scope(SK::Template);
        let body_start = self.pos;
        let saved_abbrev = self.abbrev.replace(crate::cons::Abbrev { depth, base: 0, params: vec![], cons: None, closed: false });
        let saved_last = self.last_templ.take();
        let saved_hc = std::mem::replace(&mut self.head_constrained, false);
        let saved_cons = self.head_cons.take();
        let saved_parts0 = std::mem::take(&mut self.head_parts);
        self.pending_template = Some(PendingTemplate { params: vec![], specialization: false, tparams: vec![], depth, dependent });
        self.templ += 1;
        self.tdepth += 1;
        let r = self.parse_declaration(ctx);
        self.tdepth -= 1;
        self.templ -= 1;
        self.pending_template = None;
        self.pop_scope();
        self.head_parts = saved_parts0;
        let ab = std::mem::replace(&mut self.abbrev, saved_abbrev);
        let last = std::mem::replace(&mut self.last_templ, saved_last);
        let body_end = self.pos;
        if r.is_ok() {
            if let Some(ab) = ab {
                self.head_cons = ab.cons.clone();
                let saved_parts = std::mem::replace(&mut self.head_parts, (ab.cons, None));
                self.head_constrained = false;
                self.record_template(last, ab.params, depth, dependent, body_start, body_end, ctx, line);
                self.head_parts = saved_parts;
            }
        }
        self.head_cons = saved_cons;
        self.head_constrained = saved_hc;
        r.map(|_| ())
    }

    /// A concept definition after its template head (CPT-1 to CPT-7).
    fn parse_concept_def(&mut self, ctx: Ctx, tparams: Vec<crate::templ::TParam>, depth: u32, head_constrained: bool, line: u32) -> PR<()> {
        if self.eat("friend") {
            self.error(line, "CPT-4", "a friend declaration declares a concept");
        } else if !matches!(ctx, Ctx::Namespace) {
            self.error(line, "CPT-4", "a concept is defined outside namespace scope");
        }
        self.expect("concept")?;
        let name = self.ident()?;
        if head_constrained {
            self.error(line, "CPT-5", format!("the template head of the concept '{}' has constraints", name));
        }
        if self.at("<") {
            self.error(line, "CPT-7", format!("a partial specialization of the concept '{}'", name));
            self.recover();
            return Ok(());
        }
        self.skip_attributes();
        if !self.at("=") {
            self.error(line, "CPT-7", format!("a declaration of the concept '{}' that is not its definition", name));
            self.recover();
            return Ok(());
        }
        self.pos += 1;
        let scope = self.decl_scope();
        if self.scopes[scope].names.get(&name).is_some_and(|v| !v.is_empty()) {
            self.error(line, "CPT-7", format!("the concept '{}' is declared again", name));
        }
        // CPT-6: the definition does not name the concept.
        let mut k = 0;
        loop {
            let t = self.peek(k);
            if t.kind == TK::Eof || (t.kind == TK::Punct && t.text == ";") {
                break;
            }
            if t.kind == TK::Ident && t.text == name {
                self.error(line, "CPT-6", format!("the definition of the concept '{}' uses its own name", name));
                self.recover();
                return Ok(());
            }
            k += 1;
        }
        let start = self.pos;
        let saved_head = self.cons_head.replace((Rc::new(tparams.clone()), depth));
        let saved_unmodeled = std::mem::take(&mut self.cons_unmodeled);
        self.templ += 1;
        self.in_concept_def = true;
        let r = self.cons_expr();
        self.in_concept_def = false;
        self.templ -= 1;
        self.cons_head = saved_head;
        let unmodeled = std::mem::replace(&mut self.cons_unmodeled, saved_unmodeled);
        let n = r?;
        let end = self.pos;
        self.expect(";")?;
        let decl = self.add_decl_in(scope, &name, DK::Template(TemplKind::Concept), line);
        let (vid, base) = self.origin_of(start);
        let src = crate::templ::Source { toks: self.copy_toks(start, end), stack: self.stack.clone(), ctx, access: 0, limit: self.decls.len(), vid, base };
        let info = crate::templ::TemplInfo {
            name,
            kind: crate::templ::TKind::Concept,
            params: tparams,
            depth,
            src: Some(src),
            stack: self.stack.clone(),
            decl,
            scope,
            line,
            dependent: self.templ > 0,
            explicit: vec![],
            partials: vec![],
            insts: vec![],
            members: vec![],
            fn_pack: None,
            fresh: false,
            constrained: unmodeled || n.is_none(),
            cons: n,
            cons_key: String::new(),
        };
        self.templs.push(info);
        let t = self.templs.len() - 1;
        self.decl_tid.insert(decl, t);
        self.record_attrs(decl);
        Ok(())
    }

    /// Whether a declarator names a member of an instance or of an
    /// explicitly specialized class (TSP-3, TSP-22).
    fn in_instance_member(&self, d: &Declarator) -> bool {
        match d.qual {
            Some(s) => matches!(self.scopes[s].kind, SK::Class(c) if self.in_instance(c)),
            None => false,
        }
    }

    /// Skip what is left of a function declaration after an error.
    fn recover_function_body(&mut self) {
        while !self.at(";") && !self.at("{") && self.tok().kind != TK::Eof {
            if self.at("(") {
                self.skip_balanced();
            } else {
                self.pos += 1;
            }
        }
        if self.at("{") {
            self.skip_balanced();
        } else {
            self.eat(";");
        }
    }

    /// DED-32, DED-33, TSP-4: the template and arguments of the
    /// specialization that a declaration with the function type `ft`
    /// names.
    fn match_declared_spec(&mut self, cands: &[DeclId], targs: Option<&std::rc::Rc<Vec<crate::templ::RawArg>>>, ft: &QTy, line: u32) -> Option<(usize, Vec<TArg>)> {
        self.match_declared_spec_rule(cands, targs, ft, line, "TSP-4")
    }

    fn match_declared_spec_rule(&mut self, cands: &[DeclId], targs: Option<&std::rc::Rc<Vec<crate::templ::RawArg>>>, ft: &QTy, line: u32, rule: &'static str) -> Option<(usize, Vec<TArg>)> {
        use crate::templ::Binds;
        let mut found: Vec<(usize, Vec<TArg>)> = Vec::new();
        let mut failed_cons = false;
        for &c in cands {
            let t = self.decl_tid[&c];
            let params = self.templs[t].params.clone();
            let mut b = Binds::new(self.templs[t].depth, &params);
            if let Some(raw) = targs {
                let me = self.d.muted_errors;
                self.d.muted += 1;
                let r = self.targs_for(t, (**raw).clone(), line, true);
                self.d.muted -= 1;
                match r {
                    Ok(Some(v)) if self.d.muted_errors == me => {
                        for (i, a) in v.into_iter().enumerate() {
                            b.vals[i] = Some(a);
                        }
                    }
                    _ => continue,
                }
            }
            let Some(f) = self.func(c).cloned() else { continue };
            let pt = QTy::new(Ty::Func(Box::new(f.fty.clone())));
            let ok = if f.ret_auto.is_some() && matches!(&ft.t, Ty::Func(af) if af.ret.is_unknown()) {
                // A deduced return type: the parameters only.
                match (&pt.t, &ft.t) {
                    (Ty::Func(pf), Ty::Func(af)) => pf.params.len() == af.params.len() && pf.params.iter().zip(af.params.iter()).all(|(p, a)| self.deduce_ty(p, a, &mut b)),
                    _ => false,
                }
            } else {
                self.deduce_ty(&pt, ft, &mut b)
            };
            if !ok || b.unknown {
                continue;
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
            if complete {
                // CNS-13: a template whose constraints fail is not found.
                match self.templ_sat(t, &args) {
                    crate::cons::Sat::No | crate::cons::Sat::Error => {
                        failed_cons = true;
                        continue;
                    }
                    _ => {}
                }
                found.push((t, args));
            }
        }
        if found.is_empty() {
            if failed_cons {
                self.error(line, "CNS-13", "the declared specialization does not satisfy the constraints of the template");
                return None;
            }
            self.error(line, rule, "no function template has a specialization with the declared type");
            return None;
        }
        if found.len() == 1 {
            return found.pop();
        }
        // DED-33, ORD-9: partial ordering chooses one.
        for i in 0..found.len() {
            let best = (0..found.len()).all(|j| {
                if i == j {
                    return true;
                }
                let (a, b) = self.func_order(found[i].0, found[j].0, None);
                a && !b
            });
            if best {
                return Some(found.swap_remove(i));
            }
        }
        self.error(line, "DED-33", "more than one function template matches the declaration, and none is more specialized");
        None
    }

    /// TSP-11: the head and the argument list of a partial
    /// specialization.
    fn check_partial(&mut self, t: usize, tparams: &[crate::templ::TParam], pats: &[TArg], depth: u32, line: u32) {
        use crate::templ::Binds;
        if tparams.iter().any(|p| p.default.is_some()) {
            self.error(line, "TSP-11", "a parameter of a partial specialization has a default");
            return;
        }
        // The argument list is not the primary template's parameters again.
        let n = self.templs[t].params.len();
        let same = !tparams.iter().any(|p| matches!(p.kind, crate::templ::TPK::Auto))
            && pats.len() == n
            && tparams.len() == n
            && pats.iter().enumerate().all(|(i, a)| {
                let k = (depth, i as u32);
                match a {
                    TArg::Type(q) => !q.c && !q.v && matches!(&q.t, Ty::Dep(d) if **d == Dep::Param(k.0, k.1)),
                    // DED-12: a constant parameter of another type than the
                    // primary template's makes another list.
                    TArg::Val(TVal::Param(x, y)) => {
                        (*x, *y) == k
                            && match (&tparams[i].kind, &self.templs[t].params[i].kind) {
                                (crate::templ::TPK::Const(a), crate::templ::TPK::Const(b)) => a.t == b.t || matches!(b.t, Ty::Dep(_)),
                                (crate::templ::TPK::Const(_), crate::templ::TPK::Auto) => false,
                                _ => true,
                            }
                    }
                    TArg::Templ(TName::Param(x, y)) => (*x, *y) == k,
                    TArg::Expand(x) => matches!(&**x, TArg::Type(q) if matches!(&q.t, Ty::Dep(d) if **d == Dep::Param(k.0, k.1))),
                    TArg::Pack(v) => v.len() == 1 && matches!(&v[0], TArg::Expand(x) if matches!(&**x, TArg::Type(q) if matches!(&q.t, Ty::Dep(d) if **d == Dep::Param(k.0, k.1)))),
                    _ => false,
                }
            });
        if same {
            // CNS-36: allowed when the partial specialization is more
            // constrained than the primary template.
            let mine = self.head_cons.clone();
            let prim_cons = self.templs[t].cons.clone();
            let pd = self.templs[t].depth;
            let ident: Vec<TArg> = pats.to_vec();
            let me_args: Vec<TArg> = pats.to_vec();
            if mine.is_some() && self.more_constrained(&mine, depth, &me_args, &prim_cons, pd, &ident) {
                return;
            }
            if mine.is_some() {
                self.error(line, "CNS-36", "a partial specialization with the primary template's arguments is not more constrained than the primary template");
            } else {
                self.error(line, "TSP-11", "the argument list of a partial specialization is the primary template's parameter list");
            }
            return;
        }
        // ORD-14 (Q-0174): a list that only reorders the type parameters
        // matches every argument list, so it is not more specialized than
        // the primary template.
        let mut seen = vec![false; tparams.len()];
        let permuted = !same
            && self.head_cons.is_none()
            && pats.len() == n
            && tparams.len() == n
            && tparams.iter().all(|p| matches!(p.kind, crate::templ::TPK::Type) && !p.pack)
            && self.templs[t].params.iter().all(|p| matches!(p.kind, crate::templ::TPK::Type) && !p.pack)
            && pats.iter().all(|a| match a {
                TArg::Type(q) if !q.c && !q.v => match &q.t {
                    Ty::Dep(d) => match &**d {
                        Dep::Param(x, y) if *x == depth && (*y as usize) < seen.len() && !seen[*y as usize] => {
                            seen[*y as usize] = true;
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                },
                _ => false,
            });
        if permuted {
            self.error(line, "TSP-11", "the argument list of a partial specialization only reorders the parameters, so it is not more specialized than the primary template");
            return;
        }
        // A constant argument whose parameter type uses another parameter
        // is not a fixed value.
        let prim = self.templs[t].params.clone();
        for (i, a) in pats.iter().enumerate() {
            if let (Some(crate::templ::TPK::Const(pt)), TArg::Val(TVal::Const(..))) = (prim.get(i).map(|p| &p.kind), a) {
                if matches!(pt.t, Ty::Dep(_)) {
                    self.error(line, "TSP-11", "a fixed value for a parameter whose type depends on another parameter");
                    return;
                }
            }
        }
        // Every pack is expanded.
        let unexpanded = pats.iter().flat_map(|a| match a {
            TArg::Pack(v) => v.iter().collect::<Vec<_>>(),
            x => vec![x],
        }).any(|a| match a {
            TArg::Type(q) => matches!(&q.t, Ty::Dep(d) if matches!(**d, Dep::Param(dd, i) if dd == depth && tparams.get(i as usize).is_some_and(|p| p.pack))),
            TArg::Val(TVal::Param(dd, i)) => *dd == depth && tparams.get(*i as usize).is_some_and(|p| p.pack),
            _ => false,
        });
        if unexpanded {
            self.error(line, "TSP-11", "a pack of a partial specialization is not expanded");
            return;
        }
        // Each parameter appears where deduction can find it.
        let b = Binds::new(depth, tparams);
        let mut seen: Vec<u32> = Vec::new();
        for a in pats {
            self.deducible_params(a, &b, &mut seen);
        }
        if (0..tparams.len() as u32).any(|i| !seen.contains(&i)) {
            self.error(line, "TSP-11", "a parameter of a partial specialization is not deducible from its argument list");
            return;
        }
        // A pack expansion is the last argument.
        let flat: Vec<&TArg> = pats.iter().flat_map(|a| match a {
            TArg::Pack(v) => v.iter().collect::<Vec<_>>(),
            x => vec![x],
        }).collect();
        if let Some(i) = flat.iter().position(|a| matches!(a, TArg::Expand(_))) {
            if i + 1 != flat.len() {
                self.error(line, "TSP-11", "a pack expansion is not the last argument of a partial specialization");
            }
        }
    }

    /// The parameters that an argument pattern gives values to.
    fn deducible_params(&self, a: &TArg, b: &crate::templ::Binds, out: &mut Vec<u32>) {
        match a {
            TArg::Type(q) => self.deducible_in_ty(q, b, out),
            TArg::Val(TVal::Param(d, i)) if *d == b.depth => out.push(*i),
            TArg::Templ(TName::Param(d, i)) if *d == b.depth => out.push(*i),
            TArg::Pack(v) => {
                for x in v {
                    self.deducible_params(x, b, out);
                }
            }
            TArg::Expand(x) => self.deducible_params(x, b, out),
            _ => {}
        }
    }

    fn deducible_in_ty(&self, q: &QTy, b: &crate::templ::Binds, out: &mut Vec<u32>) {
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
                        self.deducible_params(a, b, out);
                    }
                }
                Dep::Array(el, bound) => {
                    self.deducible_in_ty(el, b, out);
                    if let TVal::Param(dd, i) = bound {
                        if *dd == b.depth {
                            out.push(*i);
                        }
                    }
                }
                Dep::MemPtr(c, m) => {
                    self.deducible_in_ty(c, b, out);
                    self.deducible_in_ty(m, b, out);
                }
                _ => {}
            },
            Ty::Ptr(x) | Ty::LRef(x) | Ty::RRef(x) | Ty::Array(x, _) | Ty::MemPtr(_, x) => self.deducible_in_ty(x, b, out),
            Ty::Func(f) => {
                self.deducible_in_ty(&f.ret, b, out);
                for p in &f.params {
                    self.deducible_in_ty(p, b, out);
                }
            }
            _ => {}
        }
    }

    /// An explicit instantiation, after `template` (or `extern template`
    /// for a declaration): INST-16 to INST-29.
    fn parse_explicit_inst(&mut self, ctx: Ctx, ext: bool, line: u32) -> PR<()> {
        if !matches!(ctx, Ctx::Namespace) {
            // INST-21: at namespace scope.
            self.error(line, "INST-21", "an explicit instantiation outside namespace scope");
            self.recover();
            return Ok(());
        }
        if !self.decl_attrs.is_empty() || (self.at("[") && self.at_k(1, "[")) {
            // INST-25: no attribute.
            self.error(line, "INST-25", "an attribute on an explicit instantiation");
            self.skip_attributes();
        }
        let kind_key = if ext { "decl" } else { "def" };
        if self.at("class") || self.at("struct") || self.at("union") {
            let key = match self.tok().text.as_str() {
                "struct" => 0u8,
                "class" => 1,
                _ => 2,
            };
            self.pos += 1;
            let scope = self.parse_nested_name()?;
            let name = self.ident()?;
            let found = match scope {
                None => self.lookup(&name),
                Some(Ok(s)) => self.lookup_in_scope(s, &name, true),
                Some(Err(())) => None,
            };
            if !self.at("<") {
                // A member class: `template struct Outer<int>::Inner;`.
                let c = found.and_then(|ids| ids.iter().find_map(|&i| self.class_of_decl(i)));
                self.expect(";")?;
                if let Some(c) = c {
                    if !ext {
                        self.explicit_inst_class(c, line);
                    }
                } else {
                    self.error(line, "INST-22", format!("'{}' names no specialization", name));
                }
                return Ok(());
            }
            let tid = found.and_then(|ids| ids.iter().find_map(|&i| self.tid_of(i)));
            let Some(t) = tid.filter(|&t| self.templs[t].kind == crate::templ::TKind::Class) else {
                self.skip_template_args()?;
                self.recover_until_semicolon();
                self.eat(";");
                self.error(line, "INST-22", format!("'{}' is not a class template", name));
                return Ok(());
            };
            let raw = self.parse_raw_targs()?;
            self.expect(";")?;
            let here = self.decl_scope();
            if ext && self.in_unnamed_ns(self.templs[t].scope) {
                // INST-29 (Q-0234): no explicit instantiation declaration
                // of a specialization with internal linkage.
                self.error(line, "INST-29", "an explicit instantiation declaration of a specialization with internal linkage");
            }
            if !self.scope_encloses(here, self.templs[t].scope) || (scope.is_none() && here != self.templs[t].scope) {
                // INST-21: in the template's namespace or one that encloses
                // it, with a qualified name outside the template's own.
                self.error(line, "INST-21", "an explicit instantiation outside the namespace of its template, or without a qualified name");
                return Ok(());
            }
            let Ok(Some(args)) = self.targs_for(t, raw, line, false) else { return Ok(()) };
            let pk = match self.decls[self.templs[t].decl].k {
                DK::Template(TemplKind::Class(pc)) if pc != usize::MAX => Some(self.classes[pc].key),
                _ => None,
            };
            if pk.is_some_and(|pk| (pk == 2) != (key == 2)) {
                self.error(line, "TSP-32", "the class key of the explicit instantiation does not agree with the template's");
            }
            let k = format!("class {} {}", t, self.show_targs(&args));
            self.note_explicit_inst(&k, ext, line);
            let q = self.class_ref(t, args.clone(), line);
            if let Ty::Class(c) = q.t {
                let defined = self.templs[t].src.is_some() || self.templs[t].partials.iter().any(|p| p.src.is_some()) || self.classes[c].complete;
                if !defined {
                    // INST-20: the class template is defined.
                    self.error(line, "INST-20", format!("an explicit instantiation of '{}', whose template is not defined", self.templs[t].name));
                    return Ok(());
                }
                self.require_class(c, line);
                if !ext && !self.templs[t].explicit.iter().any(|x| x.1 == crate::templ::Inst::Class(c)) {
                    self.explicit_inst_class(c, line);
                }
            }
            let _ = kind_key;
            return Ok(());
        }
        let ds = self.parse_decl_specs(None)?;
        if ds.inline || ds.constexpr || ds.consteval || ds.storage.iter().any(|s| *s != Storage::ThreadLocal) {
            // INST-25: no inline, constexpr, consteval, or storage class.
            self.error(line, "INST-25", "an explicit instantiation with 'inline', 'constexpr', or a storage class");
        }
        if (ds.ty.is_none() && ds.auto.is_none() && !self.is_ctor_start(None)) || self.at(";") {
            // INST-23: a class needs its class key.
            self.error(line, "INST-23", "an explicit instantiation of a class without its class key");
            self.recover_until_semicolon();
            self.eat(";");
            return Ok(());
        }
        let d = match self.parse_declarator(false, false) {
            Ok(d) => d,
            Err(_) => {
                // INST-22: the name is a template declared before.
                self.error(line, "INST-22", "an explicit instantiation names no declared template");
                self.recover();
                return Ok(());
            }
        };
        let fn_targs = self.decl_fn_targs.take();
        let name = d.name.clone().unwrap_or_default();
        if self.at("<") && !d.template_args {
            // INST-22: the name is a template declared before.
            self.error(line, "INST-22", format!("'{}' is not a declared template", name));
            self.recover();
            return Ok(());
        }
        if self.at(",") || self.at("=") || self.at("{") || (self.at("(") && d.func().is_none()) {
            // INST-23: one declarator, no initializer or body.
            self.error(line, "INST-23", "an explicit instantiation with more than one declarator, an initializer, or a body");
            self.recover();
            return Ok(());
        }
        let base = match (&ds.ty, ds.auto) {
            (Some(t), _) => t.clone(),
            (None, Some(_)) => QTy::unknown(),
            (None, None) => QTy::new(Ty::Void),
        };
        if d.template_args && d.func().is_none() {
            // A variable template: `template int table<int>[3];`.
            let found = match d.qual {
                Some(s) => self.lookup_in_scope(s, &name, true),
                None => self.lookup(&name),
            };
            let tid = found.and_then(|ids| ids.iter().find_map(|&i| self.tid_of(i)));
            let raw = self.parse_raw_targs()?;
            let mut written_bound: Option<i128> = None;
            while !self.at(";") && self.tok().kind != TK::Eof {
                if self.at("=") || self.at("{") {
                    // INST-23: no initializer.
                    self.error(line, "INST-23", "an explicit instantiation with an initializer");
                    self.recover();
                    return Ok(());
                }
                if self.at("[") {
                    if written_bound.is_none() && self.peek(1).kind == TK::Int && self.at_k(2, "]") {
                        written_bound = self.peek(1).text.trim_end_matches(|c: char| c.is_ascii_alphabetic()).parse::<i128>().ok();
                    }
                    self.skip_balanced();
                } else {
                    self.pos += 1;
                }
            }
            self.expect(";")?;
            let Some(t) = tid else {
                self.error(line, "INST-22", format!("'{}' is not a variable template", name));
                return Ok(());
            };
            let here = self.decl_scope();
            if !self.scope_encloses(here, self.templs[t].scope) || (d.qual.is_none() && here != self.templs[t].scope) {
                // INST-21 (Q-0235): in the template's namespace or one that
                // encloses it, with a qualified name outside its own.
                self.error(line, "INST-21", "an explicit instantiation outside the namespace of its template, or without a qualified name");
                return Ok(());
            }
            let tdecl = self.templs[t].decl;
            let static_src = self.templs[t].src.as_ref().is_some_and(|s| s.toks.iter().take_while(|x| x.text != "=" && x.text != "{" && x.text != ";" && x.text != "<").any(|x| x.text == "static"));
            if ext && (self.var_internal.contains(&tdecl) || static_src || self.var(tdecl).is_some_and(|v| v.storage == Storage::Static) || self.in_unnamed_ns(self.templs[t].scope)) {
                // INST-29 (Q-0234): no explicit instantiation declaration
                // of a specialization with internal linkage.
                self.error(line, "INST-29", "an explicit instantiation declaration of a specialization with internal linkage");
            }
            if let Ok(Some(args)) = self.targs_for(t, raw, line, false) {
                let k = format!("var {} {}", t, self.show_targs(&args));
                self.note_explicit_inst(&k, ext, line);
                if !ext {
                    let defined = self.templs[t].src.as_ref().is_some_and(|s| s.toks.iter().any(|x| x.kind == TK::Punct && (x.text == "=" || x.text == "{")));
                    if !defined {
                        // INST-20: the definition can come later in the unit.
                        self.later_explicit.push((t, args, line, true));
                    } else if let Some(v) = self.var_inst(t, args, line) {
                        if let Some(errs) = self.body_errors.remove(&v) {
                            self.d.list.extend(errs);
                        }
                        // INST-22 (Q-0346): the written type matches the
                        // specialization's type, also its bound.
                        if let (Some(m), Some(Ty::Array(_, Some(n)))) = (written_bound, self.var(v).map(|x| x.ty.t.clone())) {
                            if m != n as i128 {
                                self.error(line, "INST-22", format!("the explicit instantiation writes the bound {}, and the specialization has the bound {}", m, n));
                            }
                        }
                        // INST-22: a written type that is not an array and
                        // differs from the specialization's.
                        let vt = self.var(v).map(|x| x.ty.clone());
                        if let Some(vt) = vt {
                            if written_bound.is_none() && d.ops.is_empty() && !base.is_unknown() && !vt.is_unknown() && !matches!(vt.t, Ty::Array(..)) && base.t != vt.t {
                                self.error(line, "INST-22", format!("the explicit instantiation writes the type {}, and the specialization has the type {}", self.show_ty(&base), self.show_ty(&vt)));
                            }
                        }
                    }
                }
            }
            return Ok(());
        }
        self.expect(";")?;
        let ty = self.apply_ops(base, &d.ops, line);
        let Ty::Func(mut ft) = ty.t.clone() else {
            // A static data member of a class specialization.
            if let Some(s) = d.qual {
                if let Some(v) = self.lookup_in_scope(s, &name, true).and_then(|ids| ids.into_iter().find(|&i| self.var(i).is_some())) {
                    if !ext {
                        self.lazy_static_defs.remove(&v);
                        if let Some(errs) = self.body_errors.remove(&v) {
                            self.d.list.extend(errs);
                        }
                    }
                    return Ok(());
                }
            }
            self.error(line, "INST-22", format!("'{}' names no specialization", name));
            return Ok(());
        };
        if ds.auto.is_some() {
            ft.ret = QTy::unknown();
        }
        let ftq = QTy::new(Ty::Func(ft.clone()));
        if let Some(s) = d.qual {
            if let SK::Class(c) = self.scopes[s].kind {
                if self.in_instance(c) && fn_targs.is_none() {
                    // A member function of a class specialization.
                    let ids = self.lookup_in_scope(s, &name, true).unwrap_or_default();
                    let m = ids.into_iter().find(|&i| self.func(i).is_some_and(|f| !f.template && f.fty.params == ft.params && f.fty.cv_c == ft.cv_c));
                    match m {
                        Some(m) => {
                            if self.func(m).is_some_and(|f| f.defaulted && !f.first_defaulted || f.body.is_none() && !self.lazy_bodies.contains_key(&m) && f.kind == FnKind::Ctor && !f.defined) {
                                self.error(line, "INST-24", "an explicit instantiation of a special member function that is declared implicitly");
                                return Ok(());
                            }
                            let k = format!("fn {}", m);
                            self.note_explicit_inst(&k, ext, line);
                            if !ext {
                                self.force_use(m);
                            }
                        }
                        None => {
                            let rule = if d.kind == NameKind::Ctor || d.kind == NameKind::Dtor { "INST-24" } else { "INST-22" };
                            self.error(line, rule, format!("'{}' names no member function of the class specialization", name));
                        }
                    }
                    return Ok(());
                }
            }
        }
        let scope = d.qual.unwrap_or_else(|| self.decl_scope());
        let cands: Vec<DeclId> = self.lookup_in_scope(scope, &name, true).unwrap_or_default().into_iter().filter(|&i| self.is_func_templ(i)).collect();
        if d.qual.is_none() && cands.is_empty() {
            if let Some(ids) = self.lookup(&name) {
                if ids.iter().any(|&i| self.is_func_templ(i)) {
                    // INST-21: outside the template's namespace the name is
                    // qualified.
                    self.error(line, "INST-21", "an explicit instantiation outside the template's namespace needs a qualified name");
                    return Ok(());
                }
            }
        }
        if cands.is_empty() {
            self.error(line, "INST-22", format!("'{}' is not a function template", name));
            return Ok(());
        }
        let ftq = if ft.noexcept { ftq } else {
            // The declaration can leave out the template's `noexcept`.
            let mut f2 = (*ft).clone();
            if cands.iter().any(|&c| self.func(c).is_some_and(|f| f.fty.noexcept)) {
                f2.noexcept = true;
            }
            QTy::new(Ty::Func(Box::new(f2)))
        };
        let Some((t, args)) = self.match_declared_spec_rule(&cands, fn_targs.as_ref(), &ftq, line, "INST-22") else { return Ok(()) };
        if !self.scope_encloses(self.decl_scope(), self.templs[t].scope) {
            // INST-21: in the template's namespace or one that encloses it.
            self.error(line, "INST-21", "an explicit instantiation outside the namespace of its template");
            return Ok(());
        }
        if ds.auto.is_none() && self.func(self.templs[t].decl).is_some_and(|f| f.ret_auto.is_some()) {
            // INST-22: a deduced return type is named with `auto`.
            self.error(line, "INST-22", "an explicit instantiation writes the deduced return type");
            return Ok(());
        }
        let tdecl = self.templs[t].decl;
        let static_src = self.templs[t].src.as_ref().is_some_and(|s| s.toks.iter().take_while(|x| x.text != "(" && x.text != "{" && x.text != ";").any(|x| x.text == "static"));
        if ext && self.func(tdecl).is_some_and(|f| f.class.is_none()) && (self.internal_fns.contains(&tdecl) || static_src || self.in_unnamed_ns(self.templs[t].scope)) {
            // INST-29 (Q-0234): no explicit instantiation declaration of a
            // specialization with internal linkage.
            self.error(line, "INST-29", "an explicit instantiation declaration of a specialization with internal linkage");
        }
        let k = format!("fn {} {}", t, self.show_targs(&args));
        self.note_explicit_inst(&k, ext, line);
        if self.templs[t].explicit.iter().any(|x| x.0 == args) {
            // INST-18: an explicit specialization stays the definition.
            return Ok(());
        }
        if let crate::templ::SpecRes::Ok(s) = self.func_spec(t, &args, line) {
            if !ext {
                let has_body = self.templs[t].src.as_ref().is_some_and(|src| src.toks.iter().any(|x| x.kind == TK::Punct && (x.text == "{" || x.text == "=")));
                if !has_body {
                    // INST-20: the definition can come later in the unit.
                    self.later_explicit.push((t, args, line, false));
                    return Ok(());
                }
                self.force_use(s);
            }
        }
        Ok(())
    }

    /// INST-26: one explicit instantiation definition, and no declaration
    /// after it.
    fn note_explicit_inst(&mut self, key: &str, ext: bool, line: u32) {
        let prev = self.explicit_insts.get(key).copied();
        match (prev, ext) {
            (Some(true), false) => self.error(line, "INST-26", "a second explicit instantiation definition of one specialization"),
            (Some(true), true) => self.error(line, "INST-26", "an explicit instantiation declaration after the definition"),
            _ => {}
        }
        if !ext {
            self.explicit_insts.insert(key.to_string(), true);
        } else {
            self.explicit_insts.entry(key.to_string()).or_insert(false);
        }
    }

    /// A definition that an explicit instantiation makes now, with its
    /// diagnostics.
    fn force_use(&mut self, f: DeclId) {
        if self.lazy_bodies.contains_key(&f) {
            self.run_lazy_body(f);
        }
        if self.spec_of.contains_key(&f) {
            self.func_body(f);
        }
        if let Some(errs) = self.body_errors.remove(&f) {
            self.d.list.extend(errs);
        }
    }

    /// INST-17: the explicit instantiation of a class makes each member
    /// that is defined, except member templates.
    fn explicit_inst_class(&mut self, c: usize, line: u32) {
        self.require_class(c, line);
        let members: Vec<DeclId> = self.scopes[self.classes[c].scope].names.values().flatten().copied().collect();
        for m in members {
            match &self.decls[m].k {
                DK::Func(f) if !f.template => {
                    // CNS-15: only the members whose constraints are
                    // satisfied.
                    if self.fn_cons.contains_key(&m) && self.fn_sat(m) != crate::cons::Sat::Yes {
                        continue;
                    }
                    if self.replayed_defs.contains(&m) || self.lazy_bodies.contains_key(&m) || self.body_errors.contains_key(&m) {
                        self.force_use(m);
                    }
                }
                DK::Var(_) => {
                    self.lazy_static_defs.remove(&m);
                    if let Some(errs) = self.body_errors.remove(&m) {
                        self.d.list.extend(errs);
                    }
                }
                DK::Class(mc) if *mc != c => {
                    let mc = *mc;
                    if self.in_instance(mc) {
                        self.explicit_inst_class(mc, line);
                    }
                }
                _ => {}
            }
        }
        for f in self.classes[c].fields.clone() {
            if let Some(errs) = self.body_errors.remove(&f) {
                self.d.list.extend(errs);
            }
        }
    }

    /// Whether a class is local to a function (TPL-2).
    pub fn class_is_local(&self, c: usize) -> bool {
        let mut s = self.scopes[self.classes[c].scope].parent;
        while let Some(x) = s {
            match self.scopes[x].kind {
                SK::Block | SK::Params => return true,
                SK::Namespace => return false,
                _ => {}
            }
            s = self.scopes[x].parent;
        }
        false
    }

    /// After a template head and `friend`: whether the declaration defines
    /// a function (a body follows the parameters).
    fn friend_template_defines_function(&self) -> bool {
        let mut k = 0;
        let mut depth = 0i32;
        loop {
            let t = self.peek(k);
            if t.kind == TK::Eof {
                return false;
            }
            if t.kind == TK::Punct {
                match t.text.as_str() {
                    "(" | "[" => depth += 1,
                    ")" | "]" => depth -= 1,
                    "{" if depth == 0 => return k > 0 && self.peek(k - 1).text != "class" && self.peek(k - 1).text != "struct",
                    ";" if depth == 0 => return false,
                    _ => {}
                }
            }
            k += 1;
        }
    }

    /// TPL-18: a template parameter name is not declared twice in one
    /// head.
    fn check_tparam_name(&mut self, name: &str, params: &[crate::templ::TParam], line: u32) {
        if !name.is_empty() && params.iter().any(|p| p.name == name) && self.inst_depth == 0 {
            self.error(line, "TPL-18", format!("the template parameter '{}' is declared twice", name));
        }
    }

    /// DER-8: at the end of the definition of `d`, the constructors that
    /// it inherits. Each is a constructor of `d` with the parameters of the
    /// base constructor, which initializes the base with them. A class
    /// without its own constructors keeps its implicit default
    /// constructor.
    pub fn inherit_ctors(&mut self, d: usize, line: u32) {
        let Some(bases) = self.inh_bases.get(&d).cloned() else { return };
        let own = self.classes[d].ctors.clone();
        let mut added = Vec::new();
        for b in bases {
            for bc in self.classes[b].ctors.clone() {
                let Some(f) = self.func(bc).cloned() else { continue };
                if f.template || self.copy_param(bc, b, false).is_some() || f.params.len() <= f.defaults {
                    // Default, copy, and move constructors are not inherited.
                    continue;
                }
                if own.iter().any(|&o| self.func(o).is_some_and(|g| g.fty.params == f.fty.params && g.fty.variadic == f.fty.variadic)) {
                    // A constructor of `d` with the same parameters hides it.
                    continue;
                }
                let mut args = Vec::new();
                let mut ok = true;
                for &p in &f.params {
                    let Some(v) = self.var(p) else { ok = false; break };
                    let t = v.ty.clone();
                    let x = match &t.t {
                        Ty::LRef(x) => Expr::new(EK::Var(p), (**x).clone(), Cat::L, line),
                        Ty::RRef(x) => Expr::new(EK::Var(p), (**x).clone(), Cat::X, line),
                        _ if self.is_scalar(&t) => {
                            let lv = Expr::new(EK::Var(p), t.clone(), Cat::L, line);
                            Expr::new(EK::Conv(Conv::LToR, Box::new(lv)), t.unqual(), Cat::PR, line)
                        }
                        _ => {
                            ok = false;
                            break;
                        }
                    };
                    args.push(x);
                }
                let init = if ok && !f.fty.variadic { Some(Rc::new(Expr::new(EK::Construct(bc, args), QTy::new(Ty::Class(b)), Cat::PR, line))) } else { None };
                let fi = FuncInfo {
                    class: Some(d),
                    mem_inits: vec![MemInit { field: None, base: Some(b), init }],
                    body: Some(Rc::new(Stmt::Block(vec![]))),
                    defined: true,
                    defaulted: false,
                    first_defaulted: false,
                    opaque: false,
                    fn_try: None,
                    parse_failed: false,
                    ..f.clone()
                };
                let sc = self.classes[d].scope;
                let id = self.add_decl_in(sc, "\u{1}inherited", DK::Func(fi), line);
                let acc = self.member_access.get(&bc).copied().unwrap_or(0);
                self.member_access.insert(id, acc);
                if let Some(e) = self.noexcept_errors.get(&bc).cloned() {
                    self.noexcept_errors.insert(id, e);
                }
                self.inherited_from.insert(id, bc);
                added.push(id);
            }
        }
        // A class that inherits constructors is not an aggregate, also when
        // it inherits none.
        if own.is_empty() {
            // The implicit default constructor stays (CTOR-20 decides
            // whether it is deleted).
            let fi = FuncInfo {
                fty: FnTy { ret: QTy::new(Ty::Void), params: vec![], variadic: false, noexcept: false, cv_c: false, cv_v: false, refq: 0, noexcept_param: None },
                params: vec![],
                defaults: 0,
                body: None,
                defined: true,
                constexpr: true,
                consteval: false,
                deleted: false,
                defaulted: true,
                class: Some(d),
                kind: FnKind::Ctor,
                is_static: false,
                is_virtual: false,
                template: false,
                explicit: false,
                opaque: false,
                is_main: false,
                explicit_obj: false,
                default_exprs: vec![],
                ret_auto: None,
                mem_inits: vec![],
                first_attrs: vec![],
                first_defaulted: true,
                constrained: false,
                fn_try: None,
                parse_failed: false,
                noexcept_false: false,
            };
            let sc = self.classes[d].scope;
            let id = self.add_decl_in(sc, "\u{1}implicit default", DK::Func(fi), line);
            self.member_access.insert(id, 0);
            self.implicit_default.insert(id);
            self.classes[d].ctors.push(id);
        }
        self.classes[d].ctors.extend(added);
    }

    /// TPL-38: a friend function template defined with a default is the
    /// only declaration of its template.
    fn check_friend_default(&mut self, id: DeclId, _tparams: &[crate::templ::TParam], friend_default: bool, line: u32) {
        let Some(f) = self.func(id) else { return };
        if f.class.is_some() {
            return;
        }
        // A declaration with the same name, function type, head, and
        // constraints is merged into the same DeclId (TPL-20, TPL-22), so
        // the DeclId names the function template.
        let earlier = !self.tfn_declared.insert(id);
        if friend_default {
            if earlier {
                self.error(line, "TPL-38", "a friend template definition with a default is not the only declaration of the template");
            }
            self.friend_defaults.push(id);
        } else if self.friend_defaults.contains(&id) {
            self.error(line, "TPL-38", "a function template is declared again after a friend definition with a default");
        }
    }

    /// TPL-6, TPL-7: a structural type for a constant parameter.
    fn tparam_type_ok(&self, t: &QTy) -> bool {
        match &t.t {
            Ty::RRef(_) | Ty::Void => false,
            Ty::LRef(x) => !matches!(x.t, Ty::Void),
            Ty::Class(c) => {
                let lit = self.literal_type(t).unwrap_or(true);
                lit && self.structural(t) && !self.classes[*c].opaque
            }
            _ => true,
        }
    }

    /// Keep the record of a template after its declaration (templ.rs).
    #[allow(clippy::too_many_arguments)]
    fn record_template(&mut self, last: Option<crate::templ::Last>, tparams: Vec<crate::templ::TParam>, depth: u32, dependent: bool, start: usize, end: usize, ctx: Ctx, line: u32) {
        use crate::templ::*;
        let Some(last) = last else { return };
        let head_constrained = std::mem::take(&mut self.head_constrained) || matches!(last, Last::Func(id) if self.func(id).is_some_and(|f| f.constrained));
        // CNS-1: the head's constraints, then the trailing requires clause.
        let mut cons = self.head_cons.clone();
        if let Last::Func(id) = &last {
            cons = crate::cons::and(cons, self.fn_cons.get(id).cloned());
        }
        let (vid, base) = self.origin_of(start);
        let src = Source { toks: self.copy_toks(start, end), stack: self.stack.clone(), ctx, access: self.access_stack.last().copied().unwrap_or(0), limit: if self.inst_depth > 0 { self.lookup_limit } else { self.decls.len() }, vid, base };
        let scope = self.decl_scope();
        if !dependent && self.inst_depth == 0 {
            if let Last::Func(id) = &last {
                // TPL-35: each parameter after a pack has a default or is
                // deducible from the function parameters (DED-20).
                if let (Some(i), Some(f)) = (tparams.iter().position(|p| p.pack), self.func(*id)) {
                    let b = crate::templ::Binds::new(depth, &tparams);
                    let mut seen: Vec<u32> = Vec::new();
                    for p in &f.fty.params {
                        self.deducible_in_ty(p, &b, &mut seen);
                    }
                    if (i + 1..tparams.len()).any(|j| tparams[j].default.is_none() && !seen.contains(&(j as u32))) {
                        self.error(line, "TPL-35", "a template parameter after a pack has no default and is not deducible");
                    }
                }
            }
            let primary = matches!(last, Last::Class(_) | Last::Var(_) | Last::Alias(_));
            if primary {
                // TPL-30: a pack is the last parameter.
                if let Some(i) = tparams.iter().position(|p| p.pack) {
                    if i + 1 != tparams.len() {
                        self.error(line, "TPL-30", "a template parameter pack is not the last parameter");
                    }
                }
                // TPL-14: each parameter after one with a default has one,
                // with the defaults of all declarations (TPL-15).
                let earlier = match &last {
                    Last::Class(c) => self.class_tid.get(c).map(|&t| self.templs[t].params.clone()),
                    _ => None,
                };
                let has = |i: usize| tparams[i].default.is_some() || earlier.as_ref().is_some_and(|e| e.get(i).is_some_and(|p| p.default.is_some()));
                if let Some(i) = (0..tparams.len()).find(|&i| has(i)) {
                    if (i..tparams.len()).any(|j| !has(j) && !tparams[j].pack) {
                        self.error(line, "TPL-14", "a template parameter without a default follows one with a default");
                    }
                }
            }
        }
        if let Last::Member(..) = &last {
            if tparams.iter().any(|p| p.default.is_some()) && self.inst_depth == 0 {
                // TPL-17: no default in a member definition outside the
                // class, in either head.
                self.error(line, "TPL-17", "a member definition outside its class template gives a default template argument");
            }
        }
        let new_info = |p: &P, name: String, kind: TKind, decl: DeclId, src: Option<Source>| TemplInfo {
            name,
            kind,
            params: tparams.clone(),
            depth,
            src,
            stack: p.stack.clone(),
            decl,
            scope,
            line,
            dependent,
            explicit: vec![],
            partials: vec![],
            insts: vec![],
            members: vec![],
            fn_pack: None,
            fresh: false,
            constrained: head_constrained,
            cons: cons.clone(),
            cons_key: p.head_key(None),
        };
        match last {
            Last::Class(c) => {
                let defined = self.classes[c].complete;
                if self.class_tid.get(&c).is_some_and(|&t| self.templs[t].src.is_none() && self.templs[t].insts.is_empty() && self.templs[t].fresh) {
                    let t = self.class_tid[&c];
                    self.templs[t].fresh = false;
                    if defined {
                        self.templs[t].src = Some(src);
                    }
                    return;
                }
                match self.class_tid.get(&c).copied() {
                    Some(t) => {
                        if !dependent && self.inst_depth == 0 && self.templs[t].cons_key != self.head_key(None) {
                            // CNS-26: equivalent constraints in the same places.
                            self.error(line, "CNS-26", format!("the constraints of '{}' differ from an earlier declaration", self.templs[t].name));
                        } else if !dependent && self.inst_depth == 0 {
                            if !crate::templ::heads_equivalent(&self.templs[t].params, &tparams) {
                                // TPL-20: equivalent heads.
                                self.error(line, "TPL-20", format!("the template head of '{}' differs from an earlier declaration", self.templs[t].name));
                            } else if self.templs[t].params.iter().zip(tparams.iter()).any(|(a, b)| a.default.is_some() && b.default.is_some()) {
                                // TPL-16: one default per parameter.
                                self.error(line, "TPL-16", format!("a default template argument of '{}' is given again", self.templs[t].name));
                            }
                        }
                        if defined && self.templs[t].src.is_none() {
                            self.templs[t].src = Some(src);
                            // A later definition replaces the parameter names
                            // and adds defaults (TPL-15).
                            let old = std::mem::take(&mut self.templs[t].params);
                            let mut ps = tparams.clone();
                            for (i, p) in ps.iter_mut().enumerate() {
                                if p.default.is_none() {
                                    p.default = old.get(i).and_then(|o| o.default.clone());
                                }
                            }
                            self.templs[t].params = ps;
                        } else {
                            let mut ps = std::mem::take(&mut self.templs[t].params);
                            for (i, p) in ps.iter_mut().enumerate() {
                                if p.default.is_none() {
                                    p.default = tparams.get(i).and_then(|o| o.default.clone());
                                }
                            }
                            self.templs[t].params = ps;
                        }
                    }
                    None => {
                        let decl = (0..self.decls.len()).rev().find(|&d| matches!(self.decls[d].k, DK::Template(TemplKind::Class(x)) if x == c));
                        let Some(decl) = decl else { return };
                        let name = self.classes[c].name.clone();
                        let info = new_info(self, name, TKind::Class, decl, if defined { Some(src) } else { None });
                        self.templs.push(info);
                        let t = self.templs.len() - 1;
                        self.class_tid.insert(c, t);
                        self.decl_tid.insert(decl, t);
                    }
                }
            }
            Last::Partial(t, pats) => {
                if self.inst_depth == 0 && !dependent {
                    self.check_partial(t, &tparams, &pats, depth, line);
                }
                let defined = self.at_end_of_definition(start, end);
                let pclass = self.last_partial_class.take();
                let ck = self.cons_key(&cons);
                if let Some(k) = self.templs[t].partials.iter().position(|p| p.pats == pats && crate::templ::heads_equivalent(&p.params, &tparams) && self.cons_key(&p.cons) == ck) {
                    if defined && self.inst_depth > 0 && self.templs[t].partials[k].src.is_some() && self.templs[t].partials[k].line != line {
                        // INST-6: two member partial specializations become
                        // the same one in the specialization.
                        self.error(line, "INST-6", format!("the instantiation makes two definitions of one partial specialization of '{}'", self.templs[t].name));
                    }
                    if defined {
                        self.templs[t].partials[k].src = Some(src);
                        self.templs[t].partials[k].class = pclass;
                    }
                } else {
                    self.templs[t].partials.push(Partial { params: tparams.clone(), pats, src: if defined { Some(src) } else { None }, depth, line, class: pclass, constrained: head_constrained, cons: cons.clone() });
                }
            }
            Last::VarPartial(t, pats) => {
                if self.inst_depth == 0 && !dependent {
                    self.check_partial(t, &tparams, &pats, depth, line);
                }
                self.templs[t].partials.push(Partial { params: tparams.clone(), pats, src: Some(src), depth, line, class: None, constrained: head_constrained, cons: cons.clone() });
            }
            Last::Func(id) => {
                let defined = self.func(id).is_some_and(|f| f.defined);
                match self.decl_tid.get(&id).copied() {
                    Some(t) => {
                        if !dependent && self.inst_depth == 0 && self.templs[t].params.len() == tparams.len() && self.templs[t].params.iter().zip(tparams.iter()).any(|(a, b)| a.default.is_some() && b.default.is_some()) {
                            // TPL-16: one default per parameter.
                            self.error(line, "TPL-16", format!("a default template argument of '{}' is given again", self.templs[t].name));
                        }
                        if defined {
                            // A later definition keeps the defaults of the
                            // earlier declarations (TPL-15).
                            let old = self.templs[t].params.clone();
                            self.templs[t].src = Some(src);
                            self.templs[t].params = tparams.clone();
                            if old.len() == tparams.len() {
                                for (i, o) in old.into_iter().enumerate() {
                                    if self.templs[t].params[i].default.is_none() && o.default.is_some() {
                                        self.templs[t].params[i].default = o.default;
                                    }
                                }
                            }
                        }
                        self.templs[t].constrained |= head_constrained;
                        if self.templs[t].cons.is_none() {
                            self.templs[t].cons = cons.clone();
                        }
                    }
                    None => {
                        let name = self.decls[id].name.clone();
                        let mut info = new_info(self, name, TKind::Func, id, Some(src));
                        info.fn_pack = self.func(id).and_then(|f| f.params.iter().position(|&p| self.pack_params.contains(&p)));
                        self.templs.push(info);
                        let t = self.templs.len() - 1;
                        self.decl_tid.insert(id, t);
                    }
                }
            }
            Last::VarDef(id) => {
                if let Some(&t) = self.decl_tid.get(&id) {
                    self.templs[t].src = Some(src);
                    self.templs[t].params = tparams.clone();
                }
            }
            Last::Var(id) => {
                let name = self.decls[id].name.clone();
                let info = new_info(self, name, TKind::Var, id, Some(src));
                self.templs.push(info);
                let t = self.templs.len() - 1;
                self.decl_tid.insert(id, t);
            }
            Last::Alias(id) => {
                let name = self.decls[id].name.clone();
                let info = new_info(self, name, TKind::Alias, id, Some(src));
                self.templs.push(info);
                let t = self.templs.len() - 1;
                self.decl_tid.insert(id, t);
            }
            Last::Member(t, pats) => {
                if dependent || self.templs[t].dependent {
                    return;
                }
                let tdepth = self.templs[t].depth;
                let _ = tdepth;
                let primary = pats.len() == tparams.len()
                    && pats.iter().enumerate().all(|(i, a)| match a {
                        TArg::Type(q) => !q.c && !q.v && matches!(&q.t, Ty::Dep(d) if **d == Dep::Param(depth, i as u32)),
                        TArg::Val(TVal::Param(d, j)) => *d == depth && *j == i as u32,
                        TArg::Templ(TName::Param(d, j)) => *d == depth && *j == i as u32,
                        TArg::Expand(x) => matches!(&**x, TArg::Type(q) if matches!(&q.t, Ty::Dep(d) if **d == Dep::Param(depth, i as u32))),
                        TArg::Pack(v) => v.len() == 1 && matches!(&v[0], TArg::Expand(_)),
                        _ => false,
                    });
                if !primary && self.inst_depth == 0 && pats.len() == tparams.len() && self.templs[t].partials.is_empty() {
                    // TPL-43: the head's parameters in their order, and a
                    // pack with `...`.
                    self.error(line, "TPL-43", "the arguments after the class name do not list the head's parameters in order");
                    return;
                }
                if self.inst_depth == 0 && !self.templs[t].dependent && pats.len() == self.templs[t].params.len() && tparams.len() != pats.len() && primary_like(&pats) {
                    // TPL-20, TPL-24: a member template needs its own head.
                    self.error(line, "TPL-20", "the head of a member definition does not match the class template's head");
                    return;
                }
                if primary && self.inst_depth == 0 && self.templs[t].partials.is_empty() && self.templs[t].cons_key != self.head_key(None) {
                    // TPL-23, CNS-26 (Q-0385): the definition repeats the
                    // class's head with its constraints.
                    self.error(line, "CNS-26", format!("the constraints of the head differ from those of the class template '{}'", self.templs[t].name));
                    return;
                }
                let partial = if primary {
                    None
                } else {
                    let parts = self.templs[t].partials.clone();
                    let mut found = None;
                    for (k, pt) in parts.iter().enumerate() {
                        if self.partial_at_least(&tparams, &pats, depth, &pt.params, &pt.pats, pt.depth) && self.partial_at_least(&pt.params, &pt.pats, pt.depth, &tparams, &pats, depth) {
                            found = Some(k);
                            break;
                        }
                    }
                    match found {
                        Some(k) => Some(k),
                        None => return,
                    }
                };
                let m = Member { params: tparams.clone(), pats, src, depth, partial };
                self.templs[t].members.push(m.clone());
                // The instances made before the definition get it now.
                let insts: Vec<(Vec<TArg>, Inst, Option<usize>)> = self.templs[t].insts.clone();
                for (args, inst, pidx) in insts {
                    if let Inst::Class(c) = inst {
                        self.replay_member(&m, c, &args, pidx);
                    }
                }
            }
        }
    }

    /// Whether the tokens from `start` to `end` hold a definition: a body
    /// in braces.
    fn at_end_of_definition(&self, start: usize, end: usize) -> bool {
        self.toks[start..end.min(self.toks.len())].iter().any(|t| t.kind == TK::Punct && t.text == "{")
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
        let after_head = self.pending_template.is_some();
        self.templ_class_defined = false;
        self.implicit_tn = matches!(ctx, Ctx::Namespace | Ctx::Class(_));
        let ds = self.parse_decl_specs(class)?;
        if after_head && self.inst_depth == 0 && std::mem::take(&mut self.templ_class_defined) && !self.at(";") {
            self.error(line, "TPL-4", "a class template definition followed by a declarator");
        }
        if after_head && self.inst_depth == 0 {
            // TPL-4: the declaration after a template head declares one
            // named entity.
            if ds.typedef {
                self.error(line, "TPL-4", "a typedef declaration after a template head");
            } else if ds.defined_class.is_some_and(|c| self.classes[c].name.is_empty()) {
                self.error(line, "TPL-4", "a class without a name after a template head");
            } else if ds.defined_class.is_some() && !self.at(";") {
                self.error(line, "TPL-4", "a class template definition followed by a declarator");
            }
        }
        if ds.friend {
            if let Ctx::Class(c) = ctx {
                self.friend_classes.insert(c);
                // ACC-6: a friend class, or a declaration the front end
                // does not track one by one.
                let ends = self.at(";");
                let templ = after_head || self.in_template();
                let name_of_friend = match ds.ty.as_ref().map(|t| &t.t) {
                    Some(Ty::Class(x)) => self.classes[*x].name.clone(),
                    _ => String::new(),
                };
                // ACC-20: a templated friend `C<T>::f` names the member `f`
                // of each specialization of `C`, and no other member.
                let mut member_name = None;
                if templ && !ends {
                    let (mut k, mut depth) = (self.pos, 0i32);
                    let mut last = None;
                    while k < self.toks.len() {
                        let t = &self.toks[k];
                        match t.text.as_str() {
                            "<" if t.kind == TK::Punct => depth += 1,
                            ">" if t.kind == TK::Punct => depth -= 1,
                            ">>" if t.kind == TK::Punct => depth -= 2,
                            "::" if depth == 0 => last = Some(k),
                            "(" | ";" | "{" | "=" if depth <= 0 => break,
                            _ => {}
                        }
                        k += 1;
                    }
                    if let Some(l) = last {
                        if self.toks.get(l + 1).is_some_and(|t| t.kind == TK::Ident && t.text != "operator") && l + 2 == k && self.toks[k].text == "(" && l > self.pos {
                            member_name = Some(self.toks[l + 1].text.clone());
                        }
                    }
                }
                let fs = self.friend_sets.entry(c).or_default();
                if let Some(n) = member_name {
                    fs.names.insert(n);
                } else if templ {
                    fs.open = true;
                } else if ends {
                    match ds.ty.as_ref().map(|t| &t.t) {
                        Some(Ty::Class(x)) => {
                            fs.classes.insert(*x);
                            // ACC-20: a specialization is a friend alone; its
                            // name would match every specialization.
                            if !self.inst_of.contains_key(x) {
                                fs.class_names.insert(name_of_friend.clone());
                            }
                        }
                        _ => fs.open = true,
                    }
                }
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
                if ds.inline && (ds.defined_class.is_some() || ds.defined_enum.is_some()) {
                    // INL-2: inline applies to a function or a variable.
                    self.error(line, "INL-2", "'inline' on a declaration that only defines a type");
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
            if ds.friend && ds.ty.is_some() && matches!(ctx, Ctx::Class(_)) {
                // ACC-5: `friend T;` with a simple type specifier names a
                // friend class; a type that is not a class is ignored.
                return Ok(vec![]);
            }
            return Err(PErr { line, msg: "declaration does not declare anything".into() });
        }
        if ds.ty.is_none() && ds.auto.is_none() && !self.is_ctor_start(class) && !self.at("~") && !self.at("operator") && !self.at("(") && !self.at_qualified_operator() {
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
            if self.is_ident() && self.peek(1).kind == TK::Ident && self.in_template() {
                // DEP-19: a name that a using declaration takes from a
                // dependent base without `typename` is a value.
                let name = self.tok().text.clone();
                if let Some(ids) = self.lookup(&name) {
                    if ids.iter().all(|&i| matches!(&self.decls[i].k, DK::Var(v) if v.dependent && v.ty.is_unknown() && !v.local)) {
                        self.error(line, "DEP-19", format!("'{}' names a value of a dependent base; a type needs 'typename'", name));
                        self.recover();
                        return Ok(vec![]);
                    }
                }
            }
            return Err(PErr { line, msg: format!("expected a declaration, found '{}'", self.tok_desc()) });
        }
        if ds.auto.is_some() && (self.at("[") || ((self.at("&") || self.at("&&")) && self.at_k(1, "["))) {
            if (ds.concept_auto || ds.cons_auto.is_some()) && !self.in_template() {
                // SB-15 (Q-0016): no constrained auto.
                self.error(line, "SB-15", "structured binding declared with a constrained auto");
            }
            return self.parse_structured_binding(&ds, ctx, line);
        }
        self.templ_head_decl = after_head && self.inst_depth == 0;
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
            if d.pack && d.func().is_none() && self.inst_depth == 0 {
                // PACK-2: only a parameter has `...` in its declarator.
                self.error(line, "PACK-2", "a declarator that is not a parameter has '...'");
            }
            if d.func().is_none() && self.at("requires") {
                // CPT-13: a trailing clause only after a function declarator.
                self.error(self.line(), "CPT-13", "a trailing requires clause after a declarator that does not declare a function");
                self.pos += 1;
                let _ = self.cons_clause(false)?;
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
                if std::mem::take(&mut self.templ_head_decl) {
                    // TPL-4: one declarator after a template head.
                    self.error(line, "TPL-4", "a declaration with two or more declarators after a template head");
                }
                continue;
            }
            break;
        }
        self.templ_head_decl = false;
        self.expect(";")?;
        Ok(ids)
    }

    /// SB-3: the named non-static data members of a class or of the one
    /// base that has them. None when they are split (SB-13) or a class is
    /// not modeled.
    fn sb_members(&self, c: usize, depth: u32) -> Option<Vec<DeclId>> {
        let ci = &self.classes[c];
        if depth > 16 || !ci.complete || ci.opaque || ci.template {
            return None;
        }
        let own: Vec<DeclId> = ci.fields.iter().copied().filter(|&f| matches!(&self.decls[f].k, DK::Field(fi) if !(fi.bits.is_some() && self.decls[f].name.is_empty()))).collect();
        let mut from_bases: Vec<Vec<DeclId>> = Vec::new();
        for b in &ci.bases {
            let bc = b.class?;
            let m = self.sb_members(bc, depth + 1)?;
            if !m.is_empty() {
                from_bases.push(m);
            }
        }
        match (own.is_empty(), from_bases.len()) {
            (_, 0) => Some(own),
            (true, 1) => from_bases.pop(),
            _ => Some(vec![usize::MAX]),
        }
    }

    /// SB-3, SB-13: the members that the names of a structured binding of
    /// class `c` refer to.
    fn sb_fields(&mut self, c: usize, line: u32) -> Option<Vec<DeclId>> {
        let quiet = self.in_template();
        if self.classes[c].key == 2 {
            if !quiet {
                self.error(line, "SB-13", "a structured binding of a union");
            }
            return None;
        }
        if !self.classes[c].anon_unions.is_empty() {
            if !quiet {
                self.error(line, "SB-13", "a structured binding of a class with an anonymous union member");
            }
            return None;
        }
        let fs = self.sb_members(c, 0)?;
        if fs.contains(&usize::MAX) {
            if !quiet {
                self.error(line, "SB-13", "the members of a structured binding are in more than one class");
            }
            return None;
        }
        let Some(&f0) = fs.first() else { return Some(fs) };
        let DK::Field(fi) = &self.decls[f0].k else { return None };
        let owner = fi.class;
        if !self.classes[owner].anon_unions.is_empty() {
            if !quiet {
                self.error(line, "SB-13", "a structured binding of a class with an anonymous union member");
            }
            return None;
        }
        for &f in &fs {
            if let Err(r) = self.member_access_ok(f, c) {
                if !quiet {
                    self.error(line, r, format!("structured binding of the member '{}', which is not accessible here (SB-13)", self.decls[f].name));
                }
                return None;
            }
        }
        Some(fs)
    }

    /// Structured bindings (specification/declarations/structured-bindings.md).
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
        if ds.v > 0 && !self.in_template() && self.wflag(&["-Wall", "-Wdeprecated", "-Wvolatile", "-Wdeprecated-volatile"]) {
            // VOL-5: a volatile structured binding declaration is deprecated.
            self.warn(line, "VOL-5", "a structured binding declaration with 'volatile' is deprecated");
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
        self.bind_names(ds, is_ref, init, &names, ctx, line, il)
    }

    /// SB-1 to SB-4: the element types, the hidden variable, and the
    /// names of a structured binding with initializer `init`. The hidden
    /// variable, if any, comes first in the result.
    #[allow(clippy::too_many_arguments)]
    pub fn bind_names(&mut self, ds: &DeclSpec, is_ref: Option<bool>, init: Expr, names: &[(String, u32)], ctx: Ctx, line: u32, il: u32) -> PR<Vec<DeclId>> {
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
                let tuple = self.lookup_in_scope(self.global, "std", false).and_then(|ids| match self.decls[ids[0]].k {
                    DK::Namespace(sc) => self.lookup_in_scope(sc, "tuple_size", false),
                    _ => None,
                }).is_some();
                if tuple && !self.in_template() && !init.ty.is_unknown() && !matches!(init.k, EK::Unknown(_)) {
                    if let Some(ids) = self.bind_tuple(ds, is_ref, &init, c, names, ctx, line, il)? {
                        return Ok(ids);
                    }
                }
                let tuple = tuple && (self.in_template() || init.ty.is_unknown() || matches!(init.k, EK::Unknown(_)));
                let fs = if tuple { None } else { self.sb_fields(c, il) };
                if let Some(fs) = fs {
                    let mut v = Vec::new();
                    for &f in &fs {
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
            Ty::Unknown | Ty::Dep(_) => {}
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
        // SB-1: the hidden variable that the names refer into, for an array
        // or a class whose members are bound one by one.
        let mut hidden: Option<DeclId> = None;
        let mut places: Vec<Expr> = Vec::new();
        if !self.in_template() && elems.as_ref().is_some_and(|e| e.len() == names.len()) && !init.ty.is_unknown() && !matches!(init.k, EK::Unknown(_)) {
            let mut base = if is_ref.is_some() { et.clone() } else { et.unqual() };
            base.c |= ds.c > 0;
            base.v |= ds.v > 0;
            let hty = match is_ref {
                Some(true) => QTy::new(Ty::LRef(Box::new(base.clone()))),
                Some(false) if init.cat == Cat::L => QTy::new(Ty::LRef(Box::new(base.clone()))),
                Some(false) => QTy::new(Ty::RRef(Box::new(base.clone()))),
                None => base.clone(),
            };
            let mark = self.d.list.len();
            let hinit = if is_ref.is_none() && matches!(base.t, Ty::Array(..)) {
                // SB-3: each element of an array is copied.
                match &et.t {
                    Ty::Array(el, Some(n)) if self.is_scalar(el) => {
                        let src = init.clone();
                        let items: Vec<Expr> = (0..*n as i128).map(|i| {
                            let d = Expr::new(EK::Conv(Conv::ArrayDecay, Box::new(src.clone())), QTy::new(Ty::Ptr(el.clone())), Cat::PR, il);
                            let x = Expr::new(EK::Index(Box::new(d), Box::new(Expr::new(EK::Int(i), QTy::int(), Cat::PR, il))), (**el).clone(), Cat::L, il);
                            Expr::new(EK::Conv(Conv::LToR, Box::new(x)), el.unqual(), Cat::PR, il)
                        }).collect();
                        if init.cat == Cat::L && items.len() < 4096 {
                            Some(Expr::new(EK::InitList(items), QTy::unknown(), Cat::PR, il))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                Some(self.convert_init(init.clone(), &hty, InitForm::Copy, il))
            };
            match hinit {
                Some(hinit) if self.d.list.len() == mark && !matches!(hinit.k, EK::Unknown(_)) => {
                    let hinit = if matches!(hinit.k, EK::InitList(_)) { self.convert_init(hinit, &hty, InitForm::CopyList, il) } else { hinit };
                    let vi = VarInfo { ty: hty, storage, constexpr: false, constinit: false, init: Some(Rc::new(hinit)), param: false, local: ctx != Ctx::Namespace, dependent: false, defined: true, class: None };
                    let hid = self.add_decl("\u{1}sb", DK::Var(vi), line);
                    let var = Expr::new(EK::Var(hid), base.clone(), Cat::L, il);
                    let es = elems.clone().unwrap();
                    match &base.t {
                        Ty::Array(el, _) => {
                            for (i, t) in es.iter().enumerate() {
                                let d = Expr::new(EK::Conv(Conv::ArrayDecay, Box::new(var.clone())), QTy::new(Ty::Ptr(el.clone())), Cat::PR, il);
                                places.push(Expr::new(EK::Index(Box::new(d), Box::new(Expr::new(EK::Int(i as i128), QTy::int(), Cat::PR, il))), t.clone(), Cat::L, il));
                            }
                        }
                        Ty::Class(c) => {
                            let fs: Vec<DeclId> = self.sb_members(*c, 0).unwrap_or_default();
                            for (f, t) in fs.iter().zip(es.iter()) {
                                let t = nonref(t);
                                places.push(Expr::new(EK::Member(Box::new(var.clone()), *f), t, Cat::L, il));
                            }
                        }
                        _ => {}
                    }
                    if places.len() == names.len() {
                        hidden = Some(hid);
                    } else {
                        places.clear();
                    }
                }
                _ => self.d.list.truncate(mark),
            }
        }
        let mut ids = Vec::new();
        for (i, (n, nl)) in names.iter().enumerate() {
            let ty = elems.as_ref().and_then(|e| e.get(i).cloned()).unwrap_or(QTy::unknown());
            let vi = VarInfo { ty, storage, constexpr: false, constinit: false, init: None, param: false, local: ctx != Ctx::Namespace, dependent: true, defined: true, class: None };
            let id = self.add_decl(n, DK::Var(vi), *nl);
            if let Some(p) = places.get(i) {
                self.sb_place.insert(id, Rc::new(p.clone()));
            }
            ids.push(id);
        }
        // ATTR-6: a structured binding declaration that is never read.
        let maybe_unused = self.decl_attrs.iter().any(|a| a.name == "maybe_unused") || self.pending_attrs.iter().any(|a| a.name == "maybe_unused");
        if ctx == Ctx::Block && storage == Storage::Auto && !ids.is_empty() && !maybe_unused && !self.in_template() {
            self.unused_cands.push((ids[0], line));
            self.binding_groups.insert(ids[0], ids.clone());
        }
        if let Some(h) = hidden {
            ids.insert(0, h);
        }
        Ok(ids)
    }

    /// A class template of namespace `std`.
    fn std_templ(&self, name: &str) -> Option<usize> {
        let ids = self.lookup_in_scope(self.global, "std", false)?;
        let DK::Namespace(sc) = self.decls[ids[0]].k else { return None };
        let ids = self.lookup_in_scope(sc, name, false)?;
        self.tid_of(ids[0])
    }

    /// SB-5, SB-14, SB-17: the tuple protocol. None when
    /// `std::tuple_size<E>` is not complete or has no `value`, so the
    /// names refer to data members (SB-3).
    #[allow(clippy::too_many_arguments)]
    fn bind_tuple(&mut self, ds: &DeclSpec, is_ref: Option<bool>, init: &Expr, c: usize, names: &[(String, u32)], ctx: Ctx, line: u32, il: u32) -> PR<Option<Vec<DeclId>>> {
        let et = nonref(&init.ty);
        let mut base = if is_ref.is_some() { et.clone() } else { et.unqual() };
        base.c |= ds.c > 0;
        base.v |= ds.v > 0;
        let Some(ts) = self.std_templ("tuple_size") else { return Ok(None) };
        let q = self.class_inst(ts, vec![TArg::Type(base.clone())], il);
        let Ty::Class(tc) = q.t else { return Ok(None) };
        self.require_class(tc, il);
        if !self.classes[tc].complete || self.class_uncertain(tc) {
            return Ok(None);
        }
        let Some(vids) = self.lookup_member(tc, "value") else { return Ok(None) };
        let Some(v) = self.var(vids[0]).cloned() else { return Ok(None) };
        let ve = Expr::new(EK::Var(vids[0]), nonref(&v.ty), Cat::L, il);
        let Ok(n) = self.const_int(&ve) else { return Ok(None) };
        let storage = if ds.storage.contains(&Storage::Static) {
            Storage::Static
        } else if ds.storage.contains(&Storage::ThreadLocal) {
            Storage::ThreadLocal
        } else {
            Storage::Auto
        };
        let local = ctx != Ctx::Namespace;
        let mut ids = Vec::new();
        let mut refs = Vec::new();
        let mut places: Vec<Option<(QTy, DeclId)>> = vec![None; names.len()];
        if n != names.len() as i128 {
            // SB-2: one name per element.
            self.error(line, "SB-2", format!("structured binding declares {} names but the tuple size is {}", names.len(), n));
        } else {
            // SB-1: the hidden variable.
            let hty = match is_ref {
                Some(true) => QTy::new(Ty::LRef(Box::new(base.clone()))),
                Some(false) if init.cat == Cat::L => QTy::new(Ty::LRef(Box::new(base.clone()))),
                Some(false) => QTy::new(Ty::RRef(Box::new(base.clone()))),
                None => base.clone(),
            };
            let mark = self.d.list.len();
            let hinit = self.convert_init(init.clone(), &hty, InitForm::Copy, il);
            if self.d.list.len() == mark && !matches!(hinit.k, EK::Unknown(_)) {
                let vi = VarInfo { ty: hty, storage, constexpr: false, constinit: false, init: Some(Rc::new(hinit)), param: false, local, dependent: false, defined: true, class: None };
                let hid = self.add_decl("\u{1}sb", DK::Var(vi), line);
                ids.push(hid);
                let var = Expr::new(EK::Var(hid), base.clone(), Cat::L, il);
                // SB-17: `e` is an lvalue for `&`, and an xvalue otherwise.
                let arg = if is_ref == Some(true) { var } else { self.build_cast(crate::expr::CastKind::Static, QTy::new(Ty::RRef(Box::new(base.clone()))), var, il) };
                let te = self.std_templ("tuple_element");
                let member_get = self.lookup_member(c, "get").filter(|v| v.iter().any(|&d| self.is_func_templ(d)));
                for (i, place) in places.iter_mut().enumerate() {
                    // LSUP-3: the element type.
                    let ti = te.and_then(|te| {
                        let pty = match self.templs[te].params.first().map(|p| p.kind.clone()) {
                            Some(crate::templ::TPK::Const(t)) => t,
                            _ => QTy::new(Ty::size_t()),
                        };
                        let ie = Expr::new(EK::Int(i as i128), QTy::int(), Cat::PR, il);
                        let tv = self.const_targ(ie, &pty, il, true).ok()?;
                        let q = self.class_inst(te, vec![TArg::Val(tv), TArg::Type(base.clone())], il);
                        let Ty::Class(ec) = q.t else { return None };
                        self.require_class(ec, il);
                        let tids = self.lookup_member(ec, "type")?;
                        match &self.decls[tids[0]].k {
                            DK::Type(t) => Some(t.clone()),
                            DK::Class(x) => Some(QTy::new(Ty::Class(*x))),
                            _ => None,
                        }
                    });
                    let Some(ti) = ti else {
                        // SB-14: the element type must exist.
                        self.error(il, "SB-14", format!("std::tuple_element<{}, E>::type does not exist for the structured binding", i));
                        break;
                    };
                    // SB-5 (Q-0376): the index is a constant of type
                    // std::size_t.
                    let raw = Rc::new(vec![crate::templ::RawArg::Expr(Expr::new(EK::Int(i as i128), QTy::new(Ty::size_t()), Cat::PR, il))]);
                    let mark = self.d.list.len();
                    let g = match &member_get {
                        Some(fs) => {
                            self.call_explicit = Some(raw);
                            self.call_overloads(Some(arg.clone()), fs.clone(), vec![], il)
                        }
                        None => {
                            let (fs, _) = self.adl_lookup("get", std::slice::from_ref(&arg));
                            let fs: Vec<DeclId> = fs.into_iter().filter(|&d| self.is_func_templ(d)).collect();
                            if fs.is_empty() {
                                // SB-14: argument-dependent lookup finds no `get`.
                                self.error(il, "SB-14", "no 'get' for the structured binding: argument-dependent lookup finds no function template");
                                break;
                            }
                            self.call_explicit = Some(raw);
                            self.call_overloads(None, fs, vec![arg.clone()], il)
                        }
                    };
                    if self.d.list.len() != mark || matches!(g.k, EK::Unknown(_)) || g.ty.is_unknown() {
                        break;
                    }
                    let bt = nonref(&ti);
                    let lref = g.cat == Cat::L || matches!(ti.t, Ty::LRef(_));
                    let rty = if lref { QTy::new(Ty::LRef(Box::new(bt.clone()))) } else { QTy::new(Ty::RRef(Box::new(bt.clone()))) };
                    let mark = self.d.list.len();
                    let r = self.convert_init(g, &rty, InitForm::Copy, il);
                    if self.d.list.len() != mark || matches!(r.k, EK::Unknown(_)) {
                        break;
                    }
                    let vi = VarInfo { ty: rty, storage, constexpr: false, constinit: false, init: Some(Rc::new(r)), param: false, local, dependent: false, defined: true, class: None };
                    let rid = self.add_decl("\u{1}sbr", DK::Var(vi), line);
                    refs.push(rid);
                    *place = Some((ti, rid));
                }
            }
        }
        let complete = places.iter().all(|p| p.is_some());
        let mut name_ids = Vec::new();
        for (i, (nm, nl)) in names.iter().enumerate() {
            let ty = places[i].as_ref().map(|p| p.0.clone()).unwrap_or(QTy::unknown());
            let vi = VarInfo { ty: ty.clone(), storage, constexpr: false, constinit: false, init: None, param: false, local, dependent: true, defined: true, class: None };
            let id = self.add_decl(nm, DK::Var(vi), *nl);
            if let (true, Some((_, rid))) = (complete, &places[i]) {
                self.sb_place.insert(id, Rc::new(Expr::new(EK::Var(*rid), nonref(&ty), Cat::L, il)));
            }
            name_ids.push(id);
        }
        let maybe_unused = self.decl_attrs.iter().any(|a| a.name == "maybe_unused") || self.pending_attrs.iter().any(|a| a.name == "maybe_unused");
        if ctx == Ctx::Block && storage == Storage::Auto && !name_ids.is_empty() && !maybe_unused {
            self.unused_cands.push((name_ids[0], line));
            self.binding_groups.insert(name_ids[0], name_ids.clone());
        }
        if !complete {
            // The names are declared, and the hidden parts are left out.
            return Ok(Some(name_ids));
        }
        ids.extend(refs);
        ids.extend(name_ids);
        Ok(Some(ids))
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
        if ds.typedef && ds.has_alignas {
            self.error(line, "ALN-2", "alignas on a typedef");
        }
        if ds.consteval && ds.constexpr {
            self.error(line, "CX-5", "both 'consteval' and 'constexpr'");
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
        // UNI-9: an anonymous union has public non-static data members only.
        if !self.in_template() {
            let sc = self.classes[c].scope;
            let mut ids: Vec<DeclId> = self.scopes[sc].names.values().flatten().copied().filter(|&d| self.decls[d].scope == sc).collect();
            ids.sort();
            for d in ids {
                let what = match &self.decls[d].k {
                    DK::Func(_) | DK::Template(_) => Some("a member function"),
                    DK::Var(_) => Some("a static data member"),
                    DK::Class(x) if *x != c => Some("a nested type"),
                    DK::Type(_) | DK::Enum(_) => Some("a nested type"),
                    DK::Field(_) if self.member_access.get(&d).is_some_and(|&a| a != 0) => Some("a member that is not public"),
                    _ => None,
                };
                if let Some(w) = what {
                    self.error(line, "UNI-9", format!("an anonymous union with {}", w));
                    break;
                }
            }
        }
        // Members of an anonymous union are members of the enclosing scope.
        if ctx == Ctx::Namespace && !ds.storage.contains(&Storage::Static) && !self.in_unnamed_ns(self.decl_scope()) {
            // STO-11: an anonymous union at namespace scope must be static,
            // unless it is in an unnamed namespace (Q-0045).
            self.error(line, "STO-11", "anonymous union at namespace scope must be declared static");
        }
        let fields = self.classes[c].fields.clone();
        let cur = self.decl_scope();
        if let Ctx::Class(cc) = ctx {
            // UNI-7: the anonymous union is an unnamed member of the class.
            // Its members are named in the class scope, and a use goes
            // through the unnamed member.
            if !self.classes[cc].anon_unions.contains(&c) {
                self.classes[cc].anon_unions.push(c);
            }
            let idx = self.classes[cc].fields.len();
            let fi = FieldInfo { ty: QTy::new(Ty::Class(c)), class: cc, index: idx, bits: None, mutable: false, init: None, align: 0, nua: false };
            let id = self.add_decl_in(cur, "", DK::Field(fi), line);
            self.classes[cc].fields.push(id);
            self.anon_parent.insert(c, (cc, id));
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
                self.scopes[cur].names.entry(name).or_default().push(f);
            }
            // The names of a nested anonymous union.
            let inner: Vec<(String, Vec<DeclId>)> = self.scopes[self.classes[c].scope].names.iter().filter(|(_, v)| v.iter().any(|&d| matches!(self.decls[d].k, DK::Field(_)) && self.decls[d].scope != self.classes[c].scope)).map(|(n, v)| (n.clone(), v.clone())).collect();
            for (n, v) in inner {
                self.scopes[cur].names.entry(n).or_default().extend(v);
            }
            return;
        }
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
            self.check_no_defaults_in(d, line);
            self.declare_typedef(&name, ty, line, ctx);
            return Ok((None, false));
        }
        if let Ty::Func(ft) = &ty.t {
            if !matches!(d.ops.last(), Some(DOp::Func(_))) && self.inst_depth > 0 && matches!(ctx, Ctx::Class(_)) && d.ops.is_empty() {
                // INST-6: a member declared without a function declarator
                // gets a function type from the arguments.
                self.error(line, "INST-6", format!("the member '{}' gets the function type {} from the template arguments", name, self.show_ty(&ty)));
                self.skip_initializer();
                return Ok((None, false));
            }
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
                    noexcept_false: false,
                    trailing: None,
                    scope,
                    defaults: 0,
                    has_explicit_this: false,
                    trailing_auto: None,
                    noexcept_dep: false,
                    noexcept_param: None,
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
        if let Some(crate::templ::InstReq::Var { name: n }) = &self.inst {
            if *n == name && !d.qual_unknown {
                // The variable of a specialization of a variable template.
                self.inst = None;
                if d.template_args {
                    self.skip_template_args()?;
                }
                let mut d2 = d.clone();
                d2.template_args = false;
                self.inst_fresh = true;
                let r = self.declare_variable(ds, &d2, ty, ctx, auto_ty);
                self.inst_fresh = false;
                if let Ok((Some(id), _)) = &r {
                    self.hide_decl(*id);
                    self.inst_out = Some(crate::templ::InstOut::Var(*id));
                }
                return r;
            }
        }
        if d.template_args && std::mem::take(&mut self.explicit_spec) && !self.in_template() {
            return self.explicit_var_spec(ds, d, ty, ctx, auto_ty);
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
                if self.pending_template.take().is_some() && self.inst_depth == 0 {
                    // TPL-27: a non-static data member is not a template.
                    self.error(line, "TPL-27", "a non-static data member declared as a template");
                }
                return self.declare_field(ds, d, ty, c);
            }
            if d.qual.is_none() && self.class_is_local(c) && !self.in_template() {
                // CLS-28: a local class has no static data member.
                self.error(line, "CLS-28", format!("static data member '{}' in a local class", name));
            }
        }
        if let Some(pt) = self.pending_template.take() {
            self.last_decl_qual = d.qual;
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

    /// Remove a declaration from its scope's names: a specialization is
    /// found through its template only.
    pub fn hide_decl(&mut self, id: DeclId) {
        let s = self.decls[id].scope;
        let name = self.decls[id].name.clone();
        if let Some(v) = self.scopes[s].names.get_mut(&name) {
            v.retain(|&x| x != id);
        }
    }

    /// `template <> T v<args> = ...;`: the variable for one
    /// specialization of a variable template (TSP-2).
    fn explicit_var_spec(&mut self, ds: &DeclSpec, d: &Declarator, ty: QTy, ctx: Ctx, auto_ty: &mut Option<QTy>) -> PR<(Option<DeclId>, bool)> {
        let name = d.name.clone().unwrap_or_default();
        let line = d.line;
        let found = match d.qual {
            Some(s) => self.lookup_in_scope(s, &name, true),
            None => self.lookup(&name),
        };
        let tid = found.and_then(|ids| ids.iter().find_map(|&i| self.tid_of(i)));
        let Some(t) = tid else {
            self.error(line, "TSP-4", format!("'{}' is not a template", name));
            self.skip_template_args()?;
            self.skip_initializer();
            return Ok((None, false));
        };
        let raw = self.parse_raw_targs()?;
        let args = match self.targs_for(t, raw, line, false) {
            Ok(Some(a)) => a,
            _ => {
                self.skip_initializer();
                return Ok((None, false));
            }
        };
        if self.templs[t].insts.iter().any(|x| x.0 == args && matches!(x.1, crate::templ::Inst::Var(_))) {
            self.error(line, "TSP-6", format!("explicit specialization of '{}' after its first use", name));
        }
        if self.inst_depth == 0 {
            // CNS-13: an explicit specialization satisfies the constraints.
            self.cns13_ok(t, &args, line);
        }
        let mut d2 = d.clone();
        d2.template_args = false;
        self.inst_fresh = true;
        let r = self.declare_variable(ds, &d2, ty, ctx, auto_ty);
        self.inst_fresh = false;
        if let Ok((Some(id), _)) = &r {
            self.hide_decl(*id);
            if let Some(x) = self.templs[t].explicit.iter_mut().find(|x| x.0 == args) {
                x.1 = crate::templ::Inst::Var(*id);
            } else {
                self.templs[t].explicit.push((args.clone(), crate::templ::Inst::Var(*id)));
            }
            self.spec_of.insert(*id, (t, args));
        }
        r
    }

    fn declare_var_template(&mut self, ds: &DeclSpec, name: &str, ty: QTy, pt: PendingTemplate, line: u32) -> PR<(Option<DeclId>, bool)> {
        let _ = (ds, ty);
        if let Some(q) = self.last_decl_qual.take() {
            // TPL-26: the definition of a static data member template
            // outside its class.
            if let SK::Class(_) = self.scopes[q].kind {
                let found = self.scopes[q].names.get(name).cloned().unwrap_or_default();
                if let Some(&id) = found.iter().find(|&&i| matches!(self.decls[i].k, DK::Template(TemplKind::Var { .. }))) {
                    if self.eat("=") {
                        let _ = self.parse_assignment()?;
                    } else {
                        self.skip_initializer();
                    }
                    self.last_templ = Some(crate::templ::Last::VarDef(id));
                    return Ok((Some(id), false));
                }
            }
        }
        let mut init = None;
        if self.eat("=") {
            let e = if self.at("{") { self.parse_braced_init()? } else { self.parse_assignment()? };
            init = Some(Rc::new(e));
        } else if self.at("{") {
            let e = self.parse_braced_init()?;
            init = Some(Rc::new(e));
        } else {
            self.skip_initializer();
        }
        let cur = self.decl_scope();
        if let Some(&prev) = self.scopes[cur].names.get(name).and_then(|v| v.iter().find(|&&i| matches!(self.decls[i].k, DK::Template(TemplKind::Var { .. })))) {
            // Another declaration of the variable template: the
            // definition gives its tokens.
            if init.is_some() {
                self.last_templ = Some(crate::templ::Last::VarDef(prev));
            }
            return Ok((Some(prev), false));
        }
        if self.inst_depth == 0 && self.scopes[cur].names.get(name).is_some_and(|v| !v.is_empty()) {
            // TPL-21: a variable template shares its name with nothing.
            self.error(line, "TPL-21", format!("the variable template '{}' has the name of another entity in its scope", name));
        }
        let id = self.add_decl_in(cur, name, DK::Template(TemplKind::Var { params: pt.params, init, specs: vec![] }), line);
        self.last_templ = Some(crate::templ::Last::Var(id));
        Ok((Some(id), false))
    }

    fn declare_var_template_spec(&mut self, name: &str, pt: PendingTemplate, line: u32) -> PR<(Option<DeclId>, bool)> {
        // The declarator stopped at `<`: the arguments are the pattern
        // of a partial specialization (TSP-10).
        let _ = pt;
        let found = self.lookup(name);
        let was_found = found.is_some();
        let target = found.and_then(|ids| ids.into_iter().find_map(|i| self.tid_of(i)));
        match target {
            Some(t) => {
                let raw = self.parse_raw_targs()?;
                if let Some(pats) = self.pattern_targs(t, raw) {
                    self.last_templ = Some(crate::templ::Last::VarPartial(t, pats));
                }
            }
            None => {
                if !was_found && !self.lookup_uncertain() && self.inst_depth == 0 {
                    // TSP-31: a partial specialization needs its primary
                    // template declared first.
                    self.error(line, "TSP-31", format!("a partial specialization of '{}', which is not declared", name));
                }
                self.skip_template_args()?
            }
        }
        if self.eat("=") {
            let _ = self.parse_assignment()?;
        } else {
            self.skip_initializer();
        }
        Ok((None, false))
    }

    fn declare_field(&mut self, ds: &DeclSpec, d: &Declarator, ty: QTy, c: usize) -> PR<(Option<DeclId>, bool)> {
        let line = d.line;
        self.require_ty(&ty, line);
        let name = d.name.clone().unwrap_or_default();
        self.check_object_attrs(true, self.at(":"));
        if ds.virtual_ {
            // VIRT-2: only a member function can be virtual.
            self.error(line, "VIRT-2", format!("'virtual' on the data member '{}'", name));
        }
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
        if ds.inline {
            // INL-2: inline applies to a function or a variable.
            self.error(line, "INL-2", "non-static data member declared inline");
        }
        if self.classes[c].key == 2 && ty.t.is_ref() && !self.in_template() {
            // UNI-2: no member of reference type in a union.
            self.error(line, "UNI-2", format!("union member '{}' of reference type", name));
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
            if matches!(nonref(&e.ty).t, Ty::Enum(en) if self.enums[en].scoped) && !self.in_template() {
                // BF-2 (Q-0313): a width of scoped enumeration type.
                self.error(bl, "BF-2", "bit-field width has a scoped enumeration type");
            }
            let e = match self.int_class_convert(&e) {
                Err(why) => {
                    // BF-1, BF-2: a width of class type.
                    self.error(bl, "BF-2", format!("bit-field width does not convert to an integral constant: {}", why));
                    e
                }
                Ok(Some(x)) => x,
                Ok(None) => e,
            };
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
                    if let Some(sz) = self.sizeof_ty(&ty.t) {
                        if v > 0 && (v as u64) > sz * 8 && !self.in_template() {
                            // BF-8: a width larger than the type.
                            self.warn(bl, "BF-8", format!("bit-field width {} is larger than the {} bits of its type", v, sz * 8));
                        }
                    }
                    bits = Some(v.max(0) as u32);
                }
                Err(_) => {
                    // BF-2: the width is an integral constant.
                    // A floating value, or a variable that is not constant.
                    let floating = nonref(&e.ty).t.is_floating();
                    let mut plain_var = false;
                    walk_expr(&e, &mut |x| {
                        if let EK::Var(v) = x.k {
                            if let DK::Var(vi) = &self.decls[v].k {
                                plain_var |= !vi.constexpr && !vi.ty.c && !vi.param;
                            }
                        }
                    });
                    if !self.in_template() && (floating || plain_var) {
                        self.error(bl, "BF-2", "bit-field width is not an integral constant");
                    }
                    bits = Some(1);
                }
            }
            if name.is_empty() && (ty.c || ty.v) && !self.in_template() {
                // BF-2: an unnamed bit-field has no cv-qualifier.
                self.error(line, "BF-2", "an unnamed bit-field with a cv-qualifier");
            }
            if !(ty.t.is_integral() || matches!(ty.t, Ty::Enum(_) | Ty::Unknown | Ty::Dep(_))) {
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
            // NAM-39: a data member can share its name with a nested class
            // or enumeration.
            if prev.iter().any(|&p| !matches!(self.decls[p].k, DK::Class(cc) if cc == c) && !matches!(self.decls[p].k, DK::Class(_) | DK::Enum(_))) && !name.is_empty() {
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

    /// OPR-9, OPR-10: the limits on a literal operator.
    fn check_literal_operator(&mut self, suffix: &str, member: bool, fd: &FnDecl, line: u32) {
        if !suffix.starts_with('_') {
            self.warn(line, "OPR-9", format!("literal suffix '{}' does not start with '_'", suffix));
        }
        if member {
            self.error(line, "OPR-10", "a literal operator must not be a member function");
            return;
        }
        if self.lang_c > 0 {
            self.error(line, "OPR-10", "a literal operator must have C++ linkage");
            return;
        }
        let ptys: Vec<QTy> = fd.params.iter().map(|&p| self.var(p).map(|v| v.ty.clone()).unwrap_or(QTy::unknown())).collect();
        if ptys.iter().any(|t| t.is_unknown()) {
            return;
        }
        if fd.defaults > 0 {
            self.error(line, "OPR-10", "a literal operator must have no default argument");
            return;
        }
        let chars = [Ty::Char, Ty::Char8, Ty::Char16, Ty::Char32, Ty::WChar];
        let ok = match ptys.as_slice() {
            [a] => {
                let u = a.unqual().t;
                matches!(u, Ty::ULLong | Ty::LDouble) || chars.contains(&u) || matches!(&a.t, Ty::Ptr(x) if x.t == Ty::Char && x.c)
            }
            [a, b] => matches!(&a.t, Ty::Ptr(x) if x.c && chars.contains(&x.t)) && b.unqual().t == Ty::size_t(),
            _ => false,
        };
        if !ok {
            self.error(line, "OPR-10", "a literal operator has a parameter list that OPR-8 does not give");
        }
    }

    /// OPR-2: the limits on an operator function.
    fn check_operator(&mut self, name: &str, member: bool, is_static: bool, fd: &FnDecl, line: u32) {
        let op = name.trim_start_matches("operator");
        if let Some(suffix) = op.strip_prefix("\"\"") {
            self.check_literal_operator(suffix, member, fd, line);
            return;
        }
        if matches!(op, "new" | "delete" | "new[]" | "delete[]" | "co_await" | "." | ".*" | "::" | "?" | ":") {
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
    pub fn check_no_defaults_in(&mut self, d: &Declarator, line: u32) {
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
        if !(ds.storage.contains(&Storage::Extern) && !self.at("=") && !self.at("{") && !self.at("(")) && !(matches!(ctx, Ctx::Class(_)) && !self.at("=") && !self.at("{")) {
            // INST-1: an object needs its class complete.
            self.require_ty(&ty, line);
        }
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
        } else if self.link_single && ctx == Ctx::Namespace && !self.at("=") && !self.at("{") && !self.at("(") {
            // LSP rules: the one declaration of a linkage specification
            // without braces declares a variable as `extern` does.
            Storage::Extern
        } else {
            Storage::Auto
        };
        let local = ctx == Ctx::Block || ctx == Ctx::Init;
        let class_member = if let Ctx::Class(c) = ctx { Some(c) } else { None };
        let target_scope = d.qual.map(|q| self.inline_target(q, &name)).unwrap_or_else(|| self.decl_scope());
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
            self.error(line, "CX-5", "'consteval' on a variable");
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
        if let Some(q) = d.qual {
            if class_member.is_none() && !local && !self.inst_fresh && self.inst_depth == 0 && matches!(self.scopes[q].kind, SK::Namespace) {
                // Q-0375: a member of an inline namespace of `q` checks
                // against that inline namespace.
                let t = if matches!(self.scopes[target_scope].kind, SK::Namespace) { target_scope } else { q };
                self.check_ns_enclosing(t, line);
            }
        }
        // Push qualified scope for the initializer.
        let saved = self.stack.clone();
        if let Some(q) = d.qual {
            self.stack.push(q);
        }
        // Existing declaration in the same scope.
        let prev = if self.inst_fresh { Vec::new() } else { self.scopes[target_scope].names.get(&name).cloned().unwrap_or_default() };
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
                    } else if local && v.storage == Storage::Extern && storage == Storage::Extern {
                        // STMT-8: two block-scope extern declarations
                        // declare the same entity.
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
                    // A variable can hide a class name in the same scope,
                    // but not a typedef name (TYN-2).
                    if self.typedef_named.contains(&p) && !local {
                        self.error(line, "TYN-2", format!("'{}' is a typedef name in this scope", name));
                    }
                }
                _ => {}
            }
        }
        if let Some(p) = redecl {
            if class_member.is_none() && !self.in_template() && !self.inst_fresh && !matches!(self.scopes[target_scope].kind, SK::Class(_)) {
                if storage == Storage::Static && !self.var_internal.contains(&p) {
                    // LNK-10: static after a declaration with external
                    // linkage.
                    self.error(line, "LNK-10", format!("'{}' declared static after a declaration with external linkage", name));
                }
                if storage != Storage::Extern && !ds.inline && self.var(p).is_some_and(|v| v.storage == Storage::Extern && v.init.is_some() && !v.constexpr) {
                    // ODR-15, ODR-1: extern with an initializer is a
                    // definition, so a later definition defines the
                    // variable twice.
                    self.error(line, "ODR-1", format!("'{}' defined twice: the extern declaration with an initializer is a definition", name));
                }
                if ds.storage.contains(&Storage::ThreadLocal) != self.thread_vars.contains(&p) {
                    // STO-12: the declarations agree on thread_local.
                    self.error(line, "STO-12", format!("declarations of '{}' disagree on 'thread_local'", name));
                }
            }
        }
        // Conflict with names of an enclosing condition / init (IF-10).
        if local {
            self.check_condition_conflict(&name, line);
        }
        // ODR-1: a static data member defined twice outside its class.
        let explicit = std::mem::take(&mut self.explicit_spec);
        // ODR-15: outside the class and without an initializer, the
        // declaration of a member that the class declares constexpr, or an
        // explicit specialization of a static data member, is no
        // definition.
        let no_init = self.at(";") || self.at(",");
        let odr15_decl = no_init && !self.in_template() && match (d.qual, redecl) {
            (Some(q), Some(p)) if matches!(self.scopes[q].kind, SK::Class(_)) => explicit || self.var(p).is_some_and(|v| v.constexpr),
            _ => false,
        };
        if let (Some(q), Some(p)) = (d.qual, redecl.filter(|_| !odr15_decl)) {
            if matches!(self.scopes[q].kind, SK::Class(_)) && !self.in_template() {
                // SINIT-7: the definition's place in the order of the
                // unit's definitions.
                // A definition adds no declaration, so two definitions in a
                // row share `n`; the count of definitions orders them.
                let n = (self.decls.len(), self.static_member_pos.len());
                self.static_member_pos.entry(p).or_insert(n);
            }
            if matches!(self.scopes[q].kind, SK::Class(_)) && !self.in_template() && !self.static_member_defs.insert(p) && !(explicit && self.replayed_defs.contains(&p)) {
                self.error(line, "ODR-1", format!("static data member '{}' defined twice", name));
            }
            if self.member_replay {
                self.replayed_defs.insert(p);
                self.member_replay_decl = Some(p);
                if !self.used_statics.contains(&p) {
                    self.lazy_static_defs.insert(p);
                }
            }
            if explicit {
                self.lazy_static_defs.remove(&p);
            }
        }
        if d.qual.is_some() && redecl.is_none() && !self.in_template() && !self.inst_fresh {
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
                Some(p) => {
                    // ALN-3: a definition after a declaration with alignas
                    // has the same alignas.
                    if !self.align_unknown && !self.in_template() {
                        let before = self.var_align.get(&p).copied();
                        let now = if ds.has_alignas && ds.align > 0 { Some(ds.align as u64) } else { None };
                        match (before, now) {
                            (Some(a), Some(b)) if a != b => self.error(line, "ALN-3", "a redeclaration with a different alignas"),
                            (Some(_), None) if storage != Storage::Extern => self.error(line, "ALN-3", "a definition without the alignas of an earlier declaration"),
                            (None, Some(b)) => {
                                self.var_align.insert(p, b);
                            }
                            _ => {}
                        }
                    }
                    p
                }
                None => {
                    let id = self.add_decl_in(target_scope, &name, DK::Var(vi), line);
                    if ds.storage.contains(&Storage::ThreadLocal) {
                        self.thread_vars.insert(id);
                    }
                    if ds.has_alignas && ds.align > 0 && !self.align_unknown {
                        self.var_align.insert(id, ds.align as u64);
                    }
                    // LNK rules: a namespace-scope variable has internal
                    // linkage if it is static, in an unnamed namespace, or
                    // const and neither inline nor extern.
                    if !local && class_member.is_none() && !matches!(self.scopes[target_scope].kind, SK::Class(_)) {
                        // ODR-12: the specialization of a variable template is
                        // one entity in the program, also when it is const.
                        let internal = storage == Storage::Static || self.in_unnamed_ns(target_scope) || ((ty.c || ds.constexpr) && !ty.v && !ds.inline && storage != Storage::Extern && !self.inst_fresh);
                        if internal {
                            self.var_internal.insert(id);
                        }
                        if self.lang_c > 0 && !self.inst_fresh {
                            if self.lang_c_single && storage == Storage::Static {
                                self.error(line, "LSP-2", "the form of a linkage specification without braces declares a static entity");
                            }
                            if storage != Storage::Static {
                                self.check_c_name(id, &name, line);
                            }
                        }
                    }
                    id
                }
            }
        };
        self.auto_in_own_init = if ds.auto.is_some() { Some(id_placeholder) } else { None };
        self.record_attrs(id_placeholder);
        // INST-5 (Q-0231): the initializer of an inline static data member
        // of a class specialization is made only when a use needs it. The
        // front end parses it at once and drops its errors (an
        // approximation, as for scoped enumerations).
        let lazy_init = class_member.is_some() && ds.inline && !ds.constexpr && self.inst_depth > 0 && !self.in_template();
        if lazy_init {
            self.d.muted += 1;
        }
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
                // PACK-13: an initializer can be a pack expansion.
                let args = self.parse_call_args()?;
                form = InitForm::Paren(args.len());
                init = Some(Expr::new(EK::InitList(args), QTy::unknown(), Cat::PR, init_line));
            }
            Ok(())
        })();
        self.auto_in_own_init = None;
        self.stack = saved;
        if lazy_init {
            self.d.muted -= 1;
        }
        r?;
        // CTAD-1, CTAD-25: a class template named without arguments takes
        // the type that the guides deduce.
        if let Some(tid) = ds.ctad {
            if !self.in_template() && d.ops.is_empty() {
                match self.ctad(tid, init.as_ref(), &form, line) {
                    Some(q) if !q.is_unknown() => {
                        let q = add_cv(&q, ty.c || ds.c > 0, ty.v || ds.v > 0);
                        if let Some(prev) = auto_ty.as_ref() {
                            if prev.unqual() != q.unqual() && !prev.is_unknown() {
                                // CTAD-3: each declarator deduces the same
                                // type.
                                self.error(line, "CTAD-3", "the declarators of one declaration deduce different types");
                            }
                        }
                        *auto_ty = Some(q.clone());
                        ty = q;
                        self.require_ty(&ty, line);
                        if let DK::Var(v) = &mut self.decls[id_placeholder].k {
                            v.ty = ty.clone();
                        }
                    }
                    Some(_) => {}
                    None => self.unsupported(line, "deduction of class template arguments"),
                }
            } else if !d.ops.is_empty() && self.inst_depth == 0 {
                // CTAD-2: not in a pointer, reference, or array declarator.
                self.error(line, "CTAD-2", "a deduced class template type in a pointer, reference, or array declarator");
            }
        }
        // INST-10: an object of an instance with no constructor of its
        // own uses the default member initializers that its initializer
        // leaves.
        if let Ty::Class(c) = &ty.t {
            let c = *c;
            if self.inst_of.contains_key(&c) && self.classes[c].ctors.is_empty() {
                let given = match &init {
                    Some(Expr { k: EK::InitList(v), .. }) => v.len(),
                    _ => 0,
                };
                for fd in self.classes[c].fields.clone().into_iter().skip(given) {
                    if let Some(errs) = self.body_errors.remove(&fd) {
                        self.d.list.extend(errs);
                    }
                }
            }
        }
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
                        // AUTO-22: each declarator deduces the same type
                        // for the placeholder.
                        let base = peel_ops(&q, &d.ops);
                        if let (Some(prev), Some(b)) = (auto_ty.as_ref(), base.as_ref()) {
                            if !prev.is_unknown() && !b.is_unknown() && prev.unqual() != b.unqual() && !self.in_template() {
                                self.error(line, "AUTO-22", "the declarators of one declaration deduce different types for the placeholder");
                            }
                        }
                        ty = q;
                        *auto_ty = base.or(Some(QTy::unknown()));
                        // AUTO-16, CPT-23: the type constraint on the type
                        // deduced for the placeholder.
                        if !self.in_template() {
                            self.placeholder_check(ds, d, &ty, "AUTO-16", line);
                        }
                    }
                }
            }
        }
        if ds.constexpr {
            // CX-1: a constexpr variable is const; for an array, so are its
            // elements.
            ty.c = true;
            ty = crate::types::array_cv_norm(&ty);
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
        if !self.in_template() && storage != Storage::Extern && class_member.is_none() {
            // INST-11: an object of a class specialization needs the
            // virtual member functions that are not pure, as the matrix
            // compilers make them. They are made at the end of the unit.
            let mut t = &ty.t;
            while let Ty::Array(e, _) = t {
                t = &e.t;
            }
            if let Ty::Class(c) = t {
                let c = *c;
                if self.in_instance(c) {
                    let fns: Vec<DeclId> = self.scopes[self.classes[c].scope].names.values().flatten().copied().filter(|&f| self.decls[f].scope == self.classes[c].scope && self.func(f).is_some_and(|fi| fi.is_virtual && !fi.template) && !self.pure_fns.contains(&f)).collect();
                    for f in fns {
                        self.use_fn(f);
                    }
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
            if ds.constexpr && !class_with_ctor && !odr15_decl {
                // CX-1. A class with a constexpr default constructor needs
                // no written initializer; the rule text does not say so.
                self.error(line, "CX-1", "constexpr variable without an initializer");
            }
            // CINIT-1, CE-24: a constinit variable without an initializer
            // is zero-initialized, which is a constant initializer for a
            // scalar or an array of scalars.
            let zero_ok = |t: &Ty| -> bool {
                let mut t = t;
                while let Ty::Array(e, _) = t {
                    t = &e.t;
                }
                !matches!(t, Ty::Class(_) | Ty::Unknown | Ty::Dep(_)) && !t.is_ref()
            };
            if ds.constinit && is_def && !class_with_ctor && !zero_ok(&ty.t) {
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
            if init.is_some() && !ds.constexpr && !ds.inline && !(ty.c && !ty.v && (ty.t.is_integral() || matches!(ty.t, Ty::Enum(_)))) && !self.in_template() {
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
            // A static data member defined after its class initializes in
            // the scope of the class, with the class's access (ACC-1).
            let qual_class = d.qual.and_then(|q| match self.scopes[q].kind {
                SK::Class(c) => Some(c),
                _ => None,
            });
            if let Some(c) = qual_class {
                self.check_classes.push(c);
            }
            let ce = self.convert_init(e, &ty, form, init_line);
            if qual_class.is_some() {
                self.check_classes.pop();
            }
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
        if let Some(ce) = &converted {
            self.check_volatile_assign(ce, true);
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
        self.check_volatile_assign(&e, true);
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
                if let Ty::Array(inner, _) = &el.t {
                    // DCL-6: a string literal initializes one whole array
                    // of characters.
                    if inner.t.is_integral() && inner.t != Ty::Bool && items.iter().all(|i| matches!(i.k, EK::Str(_) | EK::InitList(_))) {
                        return Some(items.len());
                    }
                }
                if let Ty::Array(..) | Ty::Class(_) = el.t {
                    // brace elision: count only if every item is braced
                    if items.iter().all(|i| matches!(i.k, EK::InitList(_))) || matches!(el.t, Ty::Class(_)) && items.iter().all(|i| !matches!(i.k, EK::InitList(_))) == false {
                        return Some(items.len());
                    }
                    // An element of the class type itself initializes one
                    // element, with no brace elision (INIT rules). A class
                    // with a constructor of its own is no aggregate, so each
                    // item initializes one element (INIT-5).
                    if let Ty::Class(c) = el.t {
                        let ctor_class = self.classes[c].complete && self.classes[c].ctors.iter().any(|&f| self.func(f).is_some_and(|fi| !fi.first_defaulted && !fi.deleted));
                        if ctor_class || items.iter().all(|i| matches!(i.k, EK::InitList(_)) || nonref(&i.ty).t == Ty::Class(c)) {
                            return Some(items.len());
                        }
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
        let explicit = std::mem::take(&mut self.explicit_spec);
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
                self.friend_sets.entry(c).or_default().names.insert(name.clone());
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
        let mut is_static = ds.storage.contains(&Storage::Static);
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
        if kind == FnKind::Conversion && self.pending_template.is_some() && self.last_conv_auto && self.inst_depth == 0 {
            // CVF-7 (Q-0388): a conversion function template whose name has
            // a placeholder.
            self.error(line, "CVF-7", "a conversion function template whose name is a placeholder");
        }
        if kind == FnKind::Conversion && !self.in_template() {
            // CVF-7: the form of a conversion function.
            let nparams = fd.params.len().saturating_sub(if fd.has_explicit_this { 1 } else { 0 });
            let target = d.conv_ty.clone().unwrap_or_else(QTy::unknown);
            if ds.ty.is_some() || !ds.kw.is_empty() || ds.auto.is_some() {
                self.error(line, "CVF-7", "a conversion function with a return type before 'operator'");
            } else if nparams > 0 || fd.variadic {
                self.error(line, "CVF-7", "a conversion function with parameters");
            } else if is_static {
                self.error(line, "CVF-7", "a static conversion function");
            } else if class.is_none() && d.qual.is_none() && !d.qual_unknown && !is_friend {
                self.error(line, "CVF-7", "a conversion function that is not a member");
            } else if matches!(target.t, Ty::Func(_) | Ty::Array(..)) {
                self.error(line, "CVF-7", "a conversion function to a function or array type");
            } else if let Some(c) = class {
                // CVF-6: a conversion to the class, a base, or void is
                // never used, so an implementation warns.
                let t = nonref(&target).unqual();
                let useless = t.t == Ty::Void || matches!(t.t, Ty::Class(b) if b == c || self.is_base_of(b, c));
                if useless {
                    self.warn(line, "CVF-6", "a conversion function to its own class, a base class, or void is never used by a conversion");
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
        if self.pending_template.is_some() && self.inst_depth == 0 && in_class_body {
            // TPL-25: no virtual member function template, and no
            // destructor template.
            if ds.virtual_ {
                self.error(line, "TPL-25", "a member function template declared virtual");
            } else if kind == FnKind::Dtor {
                self.error(line, "TPL-25", "a destructor declared as a template");
            }
        }
        if fd.has_explicit_this && class.is_none() && d.qual.is_none() && !d.qual_unknown && !self.in_template() {
            // CLS-14: only a member function has an explicit object
            // parameter.
            self.error(line, "CLS-14", "an explicit object parameter on a function that is not a member");
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
        if ds.has_alignas && !self.in_template() {
            self.error(line, "ALN-2", "alignas on a function");
        }
        if ds.mutable {
            self.error(line, "STO-6", "function declared mutable");
        }
        // TR-3 / FN-4: main.
        let target_scope = d.qual.map(|q| self.inline_target(q, &name)).unwrap_or_else(|| if in_class_body && is_friend { self.enclosing_ns_scope() } else { self.decl_scope() });
        let is_main = name == "main" && target_scope == self.global && member_class.is_none();
        if is_main {
            if self.pending_template.is_some() {
                self.error(line, "TR-3", "'main' declared as a template");
            }
            if ds.auto.is_some() && fd.trailing.is_none() {
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
            // FN-6 (Q-0024): other lists are implementation-defined. This
            // front end accepts the lists that Clang accepts on the matrix
            // targets, (int) and (int, char**, char**), and rejects the
            // others under TR-3 (a front-end choice, decision 0035).
            let charpp = |q: &QTy| matches!(&q.t, Ty::Ptr(p) if matches!(&p.t, Ty::Ptr(c) if c.t == Ty::Char));
            let clang_ok = (fty.params.len() == 1 && fty.params[0].t == Ty::Int) || (fty.params.len() == 3 && fty.params[0].t == Ty::Int && charpp(&fty.params[1]) && charpp(&fty.params[2]));
            if (!ok_params || fty.variadic) && !self.in_template() && !fty.params.iter().any(|q| q.is_unknown()) {
                if clang_ok && !fty.variadic {
                    self.warn(line, "FN-6", "'main' has a parameter list other than () or (int, char**)");
                } else {
                    self.error(line, "TR-3", "'main' has a parameter list that this implementation does not accept (FN-6)");
                }
            }
        }
        let fn_targs = self.decl_fn_targs.take();
        if fn_targs.is_some() && self.pending_template.is_some() && !explicit && self.inst_depth == 0 {
            // TSP-7: no partial specialization of a function template.
            self.error(line, "TSP-7", format!("a partial specialization of the function template '{}'", name));
        }
        // TSP-2, DED-32: the specialization that an explicit
        // specialization of a function template declares.
        let mut explicit_of: Option<(usize, Vec<TArg>)> = None;
        if explicit && self.pending_template.is_none() && !self.in_template() && self.inst_depth == 0 {
            let scope = match (d.qual, ctx) {
                (Some(q), _) => q,
                (None, Ctx::Class(c)) => self.classes[c].scope,
                _ => self.decl_scope(),
            };
            let cands: Vec<DeclId> = self.lookup_in_scope(scope, &name, true).unwrap_or_default().into_iter().filter(|&i| self.is_func_templ(i)).collect();
            if !cands.is_empty() || (fn_targs.is_some() && !self.in_instance_member(d)) {
                let ft = QTy::new(Ty::Func(fty.clone()));
                match self.match_declared_spec(&cands, fn_targs.as_ref(), &ft, line) {
                    Some(x) => explicit_of = Some(x),
                    None => {
                        self.recover_function_body();
                        return Ok((None, true));
                    }
                }
            }
        }
        if is_friend && !explicit && fn_targs.is_some() && self.pending_template.is_none() && !self.in_template() && self.inst_depth == 0 && d.qual.is_none() && !self.at("{") {
            // DED-32: a friend declaration with template arguments names a
            // specialization of a function template. The friend's name is
            // registered above, so the specialization has access.
            let cands: Vec<DeclId> = self.lookup(&name).unwrap_or_default().into_iter().filter(|&i| self.is_func_templ(i)).collect();
            if !cands.is_empty() {
                let ft = QTy::new(Ty::Func(fty.clone()));
                if let Some((t, args)) = self.match_declared_spec(&cands, fn_targs.as_ref(), &ft, line) {
                    let _ = self.func_spec(t, &args, line);
                    return Ok((None, true));
                }
            }
        }
        if explicit && self.inst_depth == 0 {
            if ds.friend {
                // TSP-12: no friend explicit specialization.
                self.error(line, "TSP-12", "a friend declaration declares an explicit specialization");
            }
            if let Some(SK::Class(c)) = d.qual.map(|s| self.scopes[s].kind.clone()) {
                if self.inst_of.get(&c).is_some_and(|(t, _)| self.templs[*t].explicit.iter().any(|x| x.1 == crate::templ::Inst::Class(c))) {
                    // TSP-23: a member of an explicitly specialized class
                    // is defined without `template <>`.
                    self.error(line, "TSP-23", "a member of an explicitly specialized class is defined with 'template <>'");
                } else if self.inst_of.contains_key(&c) && matches!(kind, FnKind::Ctor | FnKind::Dtor) || name == "operator=" && self.inst_of.contains_key(&c) {
                    let declared = self.lookup_in_scope(self.classes[c].scope, &name, false).unwrap_or_default().into_iter().any(|i| self.func(i).is_some_and(|f| f.fty.params == fty.params && f.fty.cv_c == fty.cv_c));
                    if !declared {
                        // TSP-27: no explicit specialization of a special
                        // member that the class declares implicitly.
                        self.error(line, "TSP-27", "an explicit specialization of a special member function that is declared implicitly");
                    }
                }
            }
        }
        if explicit && self.inst_depth == 0 && fd.defaults > 0 && d.qual.is_some_and(|s| matches!(self.scopes[s].kind, SK::Class(c) if self.inst_of.contains_key(&c) && !self.templs[self.inst_of[&c].0].explicit.iter().any(|x| x.1 == crate::templ::Inst::Class(c)))) {
            // TSP-8: no default argument in a member of an implicit
            // instantiation given explicitly.
            self.error(line, "TSP-8", "a default argument in an explicit specialization of a member");
        }
        if let Some((t, _)) = &explicit_of {
            // The specialization of a static member function template is
            // static.
            if self.func(self.templs[*t].decl).is_some_and(|f| f.is_static) {
                is_static = true;
            }
        }
        if let Some((t, args)) = &explicit_of {
            if self.templs[*t].insts.iter().any(|x| &x.0 == args && matches!(x.1, crate::templ::Inst::Func(_))) {
                // TSP-6: declared after a use that made the specialization.
                self.error(line, "TSP-6", format!("explicit specialization of '{}' after its first use", name));
            }
            if ds.storage.iter().any(|s| *s != Storage::ThreadLocal) {
                // TSP-24: no storage class other than thread_local.
                self.error(line, "TSP-24", "an explicit specialization with a storage class");
            }
            if fd.defaults > 0 {
                // TSP-8: no default argument.
                self.error(line, "TSP-8", "a default argument in an explicit specialization of a function template");
            }
        }
        // Find a previous declaration of the same function.
        let template = self.pending_template.is_some();
        let pending_full = self.pending_template.take();
        let pending_depth = pending_full.as_ref().map(|p| p.depth).unwrap_or(0);
        let pending_head: Option<Vec<crate::templ::TParam>> = pending_full.map(|p| p.tparams);
        // The trailing requires clause (CPT-11, CNS-1 part 4), read before
        // the search for an earlier declaration: a clause is part of the
        // function's identity (CNS-28).
        let mut clause: Option<Option<Rc<crate::cons::CNorm>>> = None;
        let mut clause_key: Option<Vec<String>> = None;
        let mut clause_unmodeled = false;
        let mut clause_failed = false;
        let save_virt = self.pos;
        while self.at("override") || self.at("final") {
            self.pos += 1;
        }
        if self.at("requires") {
            let cl = self.line();
            self.pos += 1;
            let st = self.pos;
            let saved_head = self.cons_head.take();
            if let Some(h) = &pending_head {
                self.cons_head = Some((Rc::new(h.clone()), pending_depth));
            }
            let saved_unmodeled = std::mem::take(&mut self.cons_unmodeled);
            // INST-12 (Q-0392): the clause of a member of a local class in
            // a specialization waits for a use. The front end parses it
            // with its errors held back, and a clause with an error is a
            // clause it does not model.
            let local = self.inst_depth > 0 && !self.in_template() && matches!(ctx, Ctx::Class(_)) && self.stack.iter().any(|&x| matches!(self.scopes[x].kind, SK::Block | SK::Params));
            let held = if local { Some(self.begin_held()) } else { None };
            let r = self.cons_clause(false);
            let failed = held.is_some_and(|h| self.end_held(h));
            self.cons_head = saved_head;
            // INST-12: a substitution failure in the waiting clause makes
            // the constraint not satisfied at a use.
            clause_failed = failed;
            clause_unmodeled = std::mem::replace(&mut self.cons_unmodeled, saved_unmodeled) && !failed;
            let n = r?;
            clause_key = Some(self.clause_key(st, self.pos));
            clause = Some(n);
            if self.at("->") {
                // CPT-12: the clause comes after a trailing return type.
                self.error(cl, "CPT-12", "a trailing requires clause before the trailing return type");
                self.pos += 1;
                let _ = self.parse_type_id();
            }
            if self.inst_depth == 0 && self.templ == 0 && !template {
                // CPT-14: only a templated function has a trailing clause.
                self.error(cl, "CPT-14", format!("the function '{}' is not templated and has a trailing requires clause", name));
            } else if ctx == Ctx::Block && self.inst_depth == 0 {
                self.error(cl, "CPT-14", format!("the function '{}' declared in a block has a trailing requires clause", name));
            }
            // CPT-15: a function that overrides a virtual function of a
            // base is virtual too.
            let overrides = match ctx {
                Ctx::Class(c) if !ds.virtual_ => self.classes[c].bases.clone().iter().filter_map(|b| b.class).any(|b| !self.class_uncertain(b) && self.lookup_member(b, &name).is_some_and(|ids| ids.iter().any(|&f| self.func(f).is_some_and(|fi| fi.is_virtual)))),
                _ => false,
            };
            if (ds.virtual_ || overrides) && self.inst_depth == 0 {
                // CPT-15: no trailing clause on a virtual function.
                self.error(cl, "CPT-15", format!("the virtual function '{}' has a trailing requires clause", name));
            }
        } else {
            self.pos = save_virt;
        }
        let inst_req = match &self.inst {
            Some(crate::templ::InstReq::Func { name: n, .. }) if (*n == name || (d.kind == NameKind::Conversion && n.starts_with("operator "))) && !d.qual_unknown => self.inst.take(),
            _ => None,
        };
        // A specialization in a replay: the declaration only, or with its
        // definition when a use needs it.
        let sig_only = matches!(inst_req, Some(crate::templ::InstReq::Func { existing: None, .. }));
        let prev = if let Some((t, args)) = &explicit_of {
            // A later declaration of the same explicit specialization.
            match self.templs[*t].explicit.iter().find(|x| &x.0 == args) {
                Some((_, crate::templ::Inst::Func(d))) if inst_req.is_none() => vec![*d],
                _ => Vec::new(),
            }
        } else if inst_req.is_some() {
            Vec::new()
        } else if in_class_body && !is_friend {
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
                    // TPL-22: a different function type, also in the
                    // result type, makes another template.
                    let distinct_templates = template && f.fty.ret != fty.ret;
                    // TPL-20, TPL-22: heads that are not equivalent make
                    // another template.
                    let heads_differ = template
                        && match (&pending_head, self.decl_tid.get(&p)) {
                            (Some(h), Some(&t)) => !crate::templ::heads_equivalent(h, &self.templs[t].params),
                            _ => false,
                        };
                    // CNS-26: other constraints declare another template.
                    let keys_differ = template && self.fn_head_key.get(&p).is_some_and(|k| *k != self.head_key(self.abbrev.as_ref().and_then(|a| a.cons.clone())));
                    // CNS-27, CNS-40: a placeholder parameter declares a
                    // template of its own, apart from one whose head writes
                    // the parameter.
                    let invented_differ = template && self.fn_invented.get(&p).is_some_and(|&k| k != self.abbrev.as_ref().map_or(0, |a| a.params.len()));
                    let distinct_templates = distinct_templates || heads_differ || keys_differ || invented_differ;
                    // CNS-26, CNS-28, CNS-29: the trailing clauses must be
                    // equivalent.
                    let same_clause = self.fn_clause_toks.get(&p) == clause_key.as_ref();
                    if !same_clause {
                        continue;
                    }
                    // CLS-9: explicit object parameters of different types
                    // declare two functions.
                    let obj_differ = f.explicit_obj && fd.has_explicit_this && {
                        let a = f.params.first().and_then(|&q| self.var(q)).map(|v| v.ty.clone());
                        let b = fd.params.first().and_then(|&q| self.var(q)).map(|v| v.ty.clone());
                        a.as_ref().is_some_and(|a| !a.is_unknown()) && b.as_ref().is_some_and(|b| !b.is_unknown()) && a != b
                    };
                    if obj_differ {
                        continue;
                    }
                    if f.fty.params == fty.params && f.fty.variadic == fty.variadic && f.fty.cv_c == fty.cv_c && f.fty.cv_v == fty.cv_v && f.fty.refq == fty.refq && !distinct_templates {
                        // (A member with a trailing requires-clause can
                        // overload one with the same parameters.)
                        existing = Some(p);
                    } else if f.fty.params == fty.params && in_class_body && f.fty.refq != fty.refq && (f.fty.refq == 0 || fty.refq == 0) && f.fty.cv_c == fty.cv_c && f.fty.cv_v == fty.cv_v {
                        // CLS-6: overloads must all have ref-qualifiers or none.
                        self.error(line, "CLS-26", "overloads with and without ref-qualifiers");
                    } else if f.fty.params == fty.params && f.fty.variadic == fty.variadic && !distinct_templates && in_class_body && (f.is_static || is_static) && f.fty.refq == fty.refq {
                        self.error(line, "CLS-9", "same member function declared static and non-static");
                    }
                }
                DK::Var(_) | DK::Field(_) | DK::Type(_) | DK::Namespace(_) | DK::Enumerator(..) => {
                    // DEP-23: a using declaration of a dependent base can
                    // name member functions.
                    if !(d.qual.is_some()) && !(in_class_body && self.decls[p].scope != self.classes[class.unwrap()].scope) && !self.dep_using.contains(&p) {
                        self.error(line, if in_class_body { "CLS-9" } else { "NAM-3" }, format!("'{}' already declared as a different kind of entity", name));
                    }
                }
                _ => {}
            }
        }
        if d.qual.is_some() && existing.is_none() && self.in_template() && self.inst_depth == 0 && clause.is_some() == false || (d.qual.is_some() && existing.is_none() && self.in_template() && self.inst_depth == 0 && clause.is_some()) {
            // CNS-29: a definition outside the class repeats the trailing
            // clause of the member it defines.
            if let Some(q) = d.qual {
                if let SK::Class(_) = self.scopes[q].kind {
                    let same_params: Vec<DeclId> = prev.iter().copied().filter(|&p| self.func(p).is_some_and(|f| f.fty.params == fty.params && !f.template) && self.fn_clause_toks.contains_key(&p)).collect();
                    if !same_params.is_empty() && !same_params.iter().any(|p| self.fn_clause_toks.get(p) == clause_key.as_ref()) {
                        self.error(line, "CNS-29", format!("the definition of '{}' outside its class has no trailing clause equivalent to the member's", name));
                    }
                }
            }
        }
        if d.qual.is_some() && existing.is_none() && !d.qual_unknown && !self.in_template() && !template && explicit_of.is_none() && inst_req.is_none() {
            if let SK::Class(c) = self.scopes[target_scope].kind {
                if !self.class_uncertain(c) && kind != FnKind::Ctor && kind != FnKind::Dtor {
                    // CLS-4: the class must declare the member first.
                    self.error(line, "CLS-24", format!("no member function '{}' with this signature declared in the class", name));
                }
            } else if !self.scopes[target_scope].uncertain {
                self.error(line, "NS-15", format!("'{}' was not declared in the namespace", name));
            }
        }
        if let Some(q) = d.qual {
            if !in_class_body && inst_req.is_none() && self.inst_depth == 0 && !d.qual_unknown && matches!(self.scopes[q].kind, SK::Namespace) {
                // Q-0375: a member of an inline namespace of `q` checks
                // against that inline namespace.
                let t = if matches!(self.scopes[target_scope].kind, SK::Namespace) { target_scope } else { q };
                self.check_ns_enclosing(t, line);
            }
        }
        let trailing_requires = clause_unmodeled;
        // virt-specifiers come before the body (VIRT grammar)
        let mut virt_spec = (false, false);
        while self.at("override") || self.at("final") {
            let seen = if self.at("override") { &mut virt_spec.0 } else { &mut virt_spec.1 };
            let twice = *seen;
            *seen = true;
            if twice {
                // VIRT-3: each specifier at most once.
                let t = self.tok().text.clone();
                self.error(self.line(), "VIRT-3", format!("'{}' appears twice", t));
            }
            self.pos += 1;
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
        if is_friend && in_class_body && !is_def && self.inst_depth == 0 {
            if let Some(k) = &clause_key {
                // CNS-31 (Q-0267): a friend with a trailing requires clause
                // must be a definition when it is not a template, or when
                // its constraints use a template parameter of the enclosing
                // class.
                // The key names a template parameter as `$depth.index`.
                let uses_outer = k.iter().any(|t| t.strip_prefix('$').and_then(|r| r.split('.').next()).and_then(|d| d.parse::<u32>().ok()).is_some_and(|d| d < pending_depth));
                if !template || uses_outer {
                    self.error(line, "CNS-31", format!("a friend declaration of '{}' with a trailing requires clause is not a definition", name));
                }
            }
        }
        if is_def && is_friend && !self.in_template() {
            if let Ctx::Class(fc) = ctx {
                if self.class_is_local(fc) {
                    // CLS-28: no friend function defined in a local class.
                    self.error(line, "CLS-28", format!("friend function '{}' defined in a local class", name));
                }
            }
        }
        if is_def && ctx == Ctx::Block {
            // FN-9: no function definition inside a function body.
            self.error(line, "FN-9", "function definition inside a function body");
        }
        if ds.inline && ctx == Ctx::Block {
            // INL-2: no inline on a function declared in a block.
            self.error(line, "INL-2", "'inline' on a function declared in a block");
        }
        // DEFARG-11: a friend declaration with a default argument is a
        // definition and the only declaration of the function.
        let friend_defaults = in_class_body && is_friend && !self.in_template() && fd.params.iter().any(|&q| self.var(q).is_some_and(|v| v.init.is_some()) || self.deferred.iter().any(|d| d.param == Some(q)));
        if friend_defaults && !is_def {
            self.error(line, "DEFARG-11", format!("a friend declaration of '{}' with a default argument that is not a definition", name));
        } else if friend_defaults && existing.is_some() {
            self.error(line, "DEFARG-11", format!("a friend declaration of '{}' with a default argument after another declaration", name));
        } else if let Some(p) = existing {
            if self.friend_default_fns.contains(&p) && !self.in_template() {
                self.error(line, "DEFARG-11", format!("another declaration of '{}', whose friend declaration gives a default argument", name));
            }
        }
        if let Some(p) = existing {
            if self.lang_c > 0 && member_class.is_none() && !self.in_template() && !self.c_fns.contains(&p) && self.func(p).is_some_and(|f| f.class.is_none()) {
                // LSP-7: a C declaration after a C++ declaration of the
                // same function.
                self.error(line, "LSP-7", format!("a declaration of '{}' with C language linkage after one with C++ linkage", name));
            }
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
                    } else if pdel && explicit && self.replayed_defs.contains(&p) {
                        // TSP-3: the explicit specialization replaces the
                        // deleted definition from the template.
                        if let Some(f) = self.func_mut(p) {
                            f.deleted = false;
                        }
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
                // A consteval function also counts as constexpr in its
                // record, so the comparison takes consteval apart.
                if (pc && !pce) != (ds.constexpr && !ds.consteval) && pce == ds.consteval && !self.in_template() {
                    // CX-5: all declarations must agree on constexpr.
                    if pdef || is_def || true {
                        self.error(line, "CX-5", format!("declarations of '{}' disagree on 'constexpr'", name));
                    }
                }
                if pce != ds.consteval {
                    self.error(line, "CX-5", format!("declarations of '{}' disagree on 'consteval'", name));
                }
                if is_static && member_class.is_none() && !in_class_body && !self.in_template() && !template && !self.internal_fns.contains(&p) && ctx != Ctx::Block {
                    // LNK-10: static after a declaration with external
                    // linkage.
                    self.error(line, "LNK-10", format!("'{}' declared static after a declaration with external linkage", name));
                }
                if ds.inline && pdef && !is_def && !self.inline_fns.contains(&p) && !self.in_template() && !in_class_body && member_class.is_none() {
                    // INL-2: inline on a declaration after the definition.
                    self.error(line, "INL-2", format!("'{}' declared inline after its definition", name));
                }
                if pne != fty.noexcept && !self.in_template() {
                    // DCL-14: all declarations must agree on noexcept.
                    self.error(line, "DCL-14", format!("declarations of '{}' disagree on noexcept", name));
                }
                if pret != fty.ret && !pret.is_unknown() && !fty.ret.is_unknown() {
                    // FN-2: all declarations have the same return type.
                    self.error(line, "FN-14", format!("declarations of '{}' have different return types", name));
                }
                if pdef && is_def && !self.in_template() && !(explicit && self.replayed_defs.contains(&p)) {
                    self.error(line, "FN-14", format!("function '{}' defined twice", name));
                }
                if explicit && self.replayed_defs.remove(&p) {
                    // TSP-3: an explicit specialization of a member of an
                    // instance replaces the definition from the template.
                    self.lazy_bodies.remove(&p);
                    self.body_errors.remove(&p);
                    if let Some(f) = self.func_mut(p) {
                        f.body = None;
                        f.defined = false;
                        f.mem_inits.clear();
                    }
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
                    if kind == FnKind::Ctor && !in_class_body && d.qual.is_some() {
                        // DEFARG-12: a default argument outside the class
                        // must not make a default, copy, or move
                        // constructor.
                        let from = |v: &[bool], k: usize| v.len() >= k && v[k..].iter().all(|&h| h);
                        let first_self = fd.params.first().and_then(|&q| self.var(q)).is_some_and(|v| match &v.ty.t {
                            Ty::LRef(t) | Ty::RRef(t) => member_class.is_some_and(|c| t.t == Ty::Class(c)),
                            _ => false,
                        });
                        if (from(&merged, 0) && !from(&old, 0)) || (first_self && from(&merged, 1) && !from(&old, 1)) {
                            self.error(line, "DEFARG-12", format!("a default argument outside the class makes '{}' a default, copy, or move constructor", name));
                        }
                    }
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
                if matches!(kind, FnKind::Ctor | FnKind::Dtor) && (ds.constexpr || ds.consteval) && !self.in_template() {
                    if let Some(c) = member_class {
                        if self.any_virtual_base(c, 0) {
                            // DER-12: no constexpr constructor or destructor
                            // in a class with a virtual base.
                            self.error(line, "DER-12", format!("a constexpr {} of '{}', which has a virtual base", if kind == FnKind::Ctor { "constructor" } else { "destructor" }, self.classes[c].name));
                        }
                    }
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
                    // DEFARG-8 does not apply to a function parameter pack,
                    // nor to the parameters that a pack expands to.
                    let have_or_pack: Vec<bool> = fd.params.iter().zip(&have).map(|(p, &h)| h || self.pack_params.contains(p)).collect();
                    if self.inst_depth == 0 {
                        self.check_trailing_defaults(&have_or_pack, line);
                    }
                    // DEFARG-6: no default arguments on an operator
                    // function other than operator().
                    // OPR-3 allows them on operator[] too (report-2 finding G9).
                    // Allocation and deallocation functions are not operator
                    // functions of an expression (NEW-23).
                    let alloc = matches!(name.as_str(), "operatornew" | "operatornew[]" | "operatordelete" | "operatordelete[]");
                    if kind == FnKind::Operator && name != "operator()" && name != "operator[]" && !alloc && have.iter().any(|&h| h) {
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
                    fn_try: None,
                    parse_failed: false,
                    noexcept_false: fd.noexcept_false,
                };
                let sc = if in_class_body && !is_friend { self.classes[class.unwrap()].scope } else { target_scope };
                let id = self.add_decl_in(sc, &name, DK::Func(fi), line);
                if d.qual_unknown && !is_friend {
                    // A member of a class template defined after the class
                    // (TPL-23): not a function of the namespace.
                    self.qual_templ_defs.insert(id);
                }
                if inst_req.is_some() {
                    self.inst_out = Some(crate::templ::InstOut::Func(id));
                }
                if self.lang_c > 0 && member_class.is_none() {
                    self.c_fns.insert(id);
                    if self.lang_c_single && is_static {
                        self.error(line, "LSP-2", "the form of a linkage specification without braces declares a static entity");
                    }
                    if !is_static && !self.in_template() && ctx != Ctx::Block {
                        self.check_c_name(id, &name, line);
                        if is_def && !self.c_fn_defs.insert(name.clone()) {
                            // LSP-4, FN-14: the C functions of one spelling
                            // are one entity, also in an unnamed namespace,
                            // so a second definition is not valid.
                            self.error(line, "FN-14", format!("the C function '{}' is defined twice", name));
                        }
                    }
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
                        self.prospective_dtors.entry(c).or_default().push(id);
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
        if friend_defaults {
            self.friend_default_fns.insert(id);
        }
        if ds.inline || ds.constexpr || ds.consteval {
            self.inline_fns.insert(id);
        }
        if let Some(errs) = self.pending_noexcept_errors.take() {
            self.noexcept_errors.insert(id, errs);
        }
        if let Some(src) = self.pending_explicit_cond.take() {
            if kind == FnKind::Ctor {
                self.explicit_cond.insert(id, src);
            }
        }
        if template && inst_req.is_none() && !self.fn_head_key.contains_key(&id) {
            let k = self.head_key(self.abbrev.as_ref().and_then(|a| a.cons.clone()));
            self.fn_head_key.insert(id, k);
            let n = self.abbrev.as_ref().map_or(0, |a| a.params.len());
            self.fn_invented.insert(id, n);
        }
        if clause_failed {
            self.fn_sat_cache.insert(id, crate::cons::Sat::No);
        }
        if let Some(c) = clause {
            if inst_req.is_none() {
                if let Some(n) = c {
                    self.fn_cons.insert(id, n);
                }
                if let Some(k) = clause_key.clone() {
                    self.fn_clause_toks.insert(id, k);
                }
            }
        }
        if inst_req.is_some() || explicit_of.is_some() {
            // A specialization is found through its template only.
            self.hide_decl(id);
            if let Some(c) = member_class {
                self.classes[c].ctors.retain(|&x| x != id);
            }
        }
        if let Some((t, args)) = explicit_of.take() {
            match self.templs[t].explicit.iter_mut().find(|x| x.0 == args) {
                Some(x) => x.1 = crate::templ::Inst::Func(id),
                None => self.templs[t].explicit.push((args.clone(), crate::templ::Inst::Func(id))),
            }
            let label = format!("{}<{}>", self.templs[t].name, self.show_targs(&args));
            self.fn_label.insert(id, label);
            self.spec_of.insert(id, (t, args));
        }
        if template && !d.qual_unknown && (!is_friend || is_def) {
            // A friend template defined in its class is a template too.
            self.last_templ = Some(crate::templ::Last::Func(id));
        }
        if template && is_friend && !d.qual_unknown {
            self.friend_templ = Some(id);
        }
        // ACC-7, NAM-29: a friend first declared in a class is hidden
        // from ordinary lookup until the namespace declares it.
        if in_class_body && is_friend && d.qual.is_none() && inst_req.is_none() {
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
                let seen = if self.at("override") { &mut virt_spec.0 } else { &mut virt_spec.1 };
                let twice = *seen;
                *seen = true;
                if twice {
                    // VIRT-3: each specifier at most once.
                    let t = self.tok().text.clone();
                    self.error(self.line(), "VIRT-3", format!("'{}' appears twice", t));
                }
                self.pos += 1;
                continue;
            }
            break;
        }
        if (virt_spec.0 || virt_spec.1) && self.in_template() && in_class_body && !is_friend {
            // Kept for the check of a class template's pattern (VIRT-3).
            self.virt_specs.insert(id, virt_spec);
        }
        if (virt_spec.0 || virt_spec.1) && !self.in_template() {
            if !in_class_body || is_friend {
                // VIRT-3: the specifiers occur only in the class.
                self.error(line, "VIRT-3", "'override' or 'final' on a declaration outside the class");
            } else {
                self.virt_specs.insert(id, virt_spec);
            }
        }
        if ds.virtual_ && self.inst_depth == 0 && !self.in_template() {
            // VIRT-2: where `virtual` can occur.
            if !in_class_body || is_friend {
                self.error(line, "VIRT-2", format!("'virtual' on '{}' outside its class definition", name));
            } else if member_class.is_some_and(|c| self.classes[c].key == 2) {
                self.error(line, "VIRT-2", format!("'virtual' on '{}', a member of a union", name));
            }
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
            if is_friend && !self.in_template() {
                // VIRT-9: a pure specifier only on a virtual member.
                self.error(line, "VIRT-9", format!("'= 0' on the friend '{}'", name));
            }
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
        if (deleted || defaulted) && self.inst_depth > 0 && member_class.is_some_and(|c| self.inst_of.contains_key(&c)) {
            self.replayed_defs.insert(id);
        }
        if deleted || defaulted {
            self.pos += 2;
            self.expect(";")?;
            if let Some(f) = self.func_mut(id) {
                f.defined = true;
                f.deleted |= deleted;
                f.defaulted |= defaulted;
            }
            if let Some(c) = self.func(id).and_then(|f| f.class) {
                if defaulted && existing.is_some() && !in_class_body && !self.in_template() && self.classes[c].complete && matches!(name.as_str(), "operator==" | "operator<=>") {
                    // DCMP-8 (Q-0062): a comparison defaulted after its
                    // class must not be one that would be deleted.
                    self.synth_defaulted_cmp(id);
                    if self.dcmp_deleted.contains_key(&id) {
                        self.error(line, "DCMP-8", format!("'{}' defaulted after its class would be deleted", name));
                    }
                }
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
        if self.inst_depth > 0 && (self.member_replay || member_class.is_some_and(|c| self.inst_of.contains_key(&c))) && !explicit {
            self.replayed_defs.insert(id);
        }
        if self.member_replay {
            self.member_replay_decl = Some(id);
        }
        // FN-12: complete parameter and return types in a definition.
        if !self.in_template() && !template && !sig_only {
            let (ret, params) = {
                let f = self.func(id).unwrap();
                (f.fty.ret.clone(), f.fty.params.clone())
            };
            self.require_ty(&ret, line);
            for p in params {
                self.require_ty(&p, line);
            }
        }
        if !self.in_template() && !template && !sig_only {
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
        if sig_only {
            // DED-24: substitution makes the declaration only.
            if let Some(f) = self.func_mut(id) {
                f.defined = false;
            }
            self.skip_function_body()?;
            return Ok((Some(id), true));
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
        } else if let Some(cs) = d.pattern_scope.filter(|_| self.in_template()) {
            // NAM-21: the body of a member of a class template defined
            // outside its class sees the members of the pattern class.
            if !self.stack.contains(&cs) {
                self.stack.push(cs);
            }
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
        let mut snapshot: Option<Rc<Vec<crate::lexer::Token>>> = None;
        // INST-5: the body of a member function of an instance waits for a
        // use. Each such body is recorded first, so that a constructor
        // body parsed below can use a member declared after it.
        let mut rest = Vec::new();
        for df in list {
            if let Some(fid) = df.func {
                let lazy = !self.force_bodies && self.in_instance(df.class) && self.func(fid).is_some_and(|f| !matches!(f.kind, FnKind::Ctor | FnKind::Dtor));
                if lazy {
                    let toks = snapshot.get_or_insert_with(|| Rc::new(self.toks.clone())).clone();
                    self.lazy_bodies.insert(fid, (toks, df));
                    self.lazy_limits.insert(fid, self.lookup_limit);
                    self.lazy_origins.insert(fid, self.tok_origin);
                    continue;
                }
            }
            rest.push(df);
        }
        // A body that splits a `>>` inserts a token, which moves the
        // bodies after it and the place to return to.
        let l0 = self.ins_log.len();
        for mut df in rest {
            df.start = self.adjust_pos(df.start, l0);
            df.end = self.adjust_pos(df.end, l0);
            let save_pos = self.pos;
            let save_mark = self.ins_log.len();
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
                // INST-5: the body of a constructor of an instance is made
                // with its class, and its diagnostics wait for a use.
                let keep = !self.force_bodies && self.in_instance(df.class) && self.func(fid).is_some_and(|f| f.kind == FnKind::Ctor && !f.constexpr && !f.consteval);
                let mark = self.d.list.len();
                let saved_uses = if keep { self.collect_uses.replace(Vec::new()) } else { self.collect_uses.take() };
                if let Err(e) = self.parse_function_body(fid, scope, Some(df.class), auto_ret) {
                    self.parse_failure(&e);
                    if let Some(f) = self.func_mut(fid) {
                        f.opaque = true;
                    }
                }
                let uses = std::mem::replace(&mut self.collect_uses, saved_uses);
                if let Some(u) = uses.filter(|u| !u.is_empty()) {
                    self.ctor_uses.insert(fid, u);
                }
                if keep && self.d.list.len() > mark {
                    let kept: Vec<crate::diag::Diag> = self.d.list.drain(mark..).collect();
                    self.body_errors.insert(fid, kept);
                }
            } else if let Some(pid) = df.param {
                let keep = self.in_instance(df.class);
                let mark = self.d.list.len();
                self.run_deferred_param(pid, &df);
                if keep && self.d.list.len() > mark {
                    let kept: Vec<crate::diag::Diag> = self.d.list.drain(mark..).collect();
                    self.default_errors.insert(pid, kept);
                }
                self.pos = self.adjust_pos(save_pos, save_mark);
                self.stack = save_stack;
                self.templ = save_templ;
                continue;
            } else if let Some(fid) = df.field {
                let line = self.line();
                // INST-10: a default member initializer of an instance is
                // made when a constructor uses it; its errors wait.
                let keep = self.in_instance(df.class);
                let mark = self.d.list.len();
                self.dmi_depth += 1;
                let braced = self.at("{") || (self.at("=") && self.peek(1).text == "{");
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
                        if !self.in_template() && self.member_binds_temporary(&ty, &ce, braced) {
                            // CTOR-17 (Q-0084): a constructor that uses the
                            // initializer binds the member to a temporary
                            // that ends with the constructor. The error
                            // waits for such a constructor.
                            self.dmi_temps.entry(df.class).or_default().push((fid, line));
                            let ctors = self.classes[df.class].ctors.clone();
                            for k in ctors {
                                self.check_ctor_dmi_temps(k, df.class);
                            }
                        }
                        if let DK::Field(f) = &mut self.decls[fid].k {
                            f.init = Some(Rc::new(ce));
                        }
                    }
                    Err(e) => self.parse_failure(&e),
                }
                self.dmi_depth -= 1;
                if keep && self.d.list.len() > mark {
                    let kept: Vec<crate::diag::Diag> = self.d.list.drain(mark..).collect();
                    self.body_errors.insert(fid, kept);
                }
            }
            self.pos = self.adjust_pos(save_pos, save_mark);
            self.stack = save_stack;
            self.templ = save_templ;
        }
    }

    /// CTOR-18 (Q-0083): a default argument of a constructor of class `c`
    /// must not make an object of a class nested in `c` with a default
    /// member initializer that the object uses.
    fn check_nested_dmi_use(&mut self, e: &Expr, c: usize, pid: DeclId, line: u32) {
        let is_ctor_param = self.classes[c].ctors.iter().any(|&k| self.func(k).is_some_and(|f| f.params.contains(&pid)));
        if self.in_template() || !is_ctor_param {
            return;
        }
        let mut found: Option<usize> = None;
        walk_expr(e, &mut |x: &Expr| {
            if found.is_some() {
                return;
            }
            let Ty::Class(n) = x.ty.t else { return };
            let uses = match &x.k {
                EK::InitList(v) => v.len() < self.classes[n].fields.len(),
                EK::Construct(k, _) => self.func(*k).is_some_and(|f| f.body.is_none() || f.first_defaulted),
                _ => false,
            };
            if uses && n != c && self.class_nested_in(n, c) && self.classes[n].fields.iter().any(|&f| matches!(&self.decls[f].k, DK::Field(fi) if fi.init.is_some())) {
                found = Some(n);
            }
        });
        if let Some(n) = found {
            self.error(line, "CTOR-18", format!("the default argument uses a default member initializer of '{}' before the enclosing class is complete", self.classes[n].name));
        }
    }

    /// Whether class `n` is nested in class `c`.
    pub fn class_nested_in(&self, n: usize, c: usize) -> bool {
        let mut s = self.scopes[self.classes[n].scope].parent;
        while let Some(x) = s {
            if self.scopes[x].kind == SK::Class(c) {
                return true;
            }
            s = self.scopes[x].parent;
        }
        false
    }

    /// A default argument of a member function, parsed after its class.
    fn run_deferred_param(&mut self, pid: DeclId, df: &Deferred) {
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
                self.check_nested_dmi_use(&ce, df.class, pid, line);
                if let DK::Var(v) = &mut self.decls[pid].k {
                    v.init = Some(Rc::new(ce));
                }
            }
            Err(e) => self.parse_failure(&e),
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

    /// CTOR-17: whether the initialization of a member of type `ty` binds
    /// a temporary: a reference bound to a prvalue, or an
    /// `std::initializer_list<E>` set from a braced list.
    pub fn member_binds_temporary(&self, ty: &QTy, ce: &Expr, braced: bool) -> bool {
        if ty.t.is_ref() {
            return Self::binds_temporary(ce);
        }
        braced && matches!(ty.t, Ty::Class(c) if self.init_list_elem(c).is_some())
    }

    /// CTOR-17 (Q-0084): constructor `k` of class `c`, once its
    /// mem-initializers are known, uses each default member initializer
    /// that binds a temporary and that no mem-initializer replaces.
    pub fn check_ctor_dmi_temps(&mut self, k: DeclId, c: usize) {
        let Some(list) = self.dmi_temps.get(&c).cloned() else { return };
        let Some(f) = self.func(k) else { return };
        if f.body.is_none() || f.defaulted || f.deleted || f.template {
            return;
        }
        if f.mem_inits.iter().any(|m| m.base == Some(c)) {
            return;
        }
        let given: Vec<DeclId> = f.mem_inits.iter().filter_map(|m| m.field).collect();
        for (fid, line) in list {
            if !given.contains(&fid) {
                self.error(line, "CTOR-17", format!("the default member initializer binds the member '{}' to a temporary object, and a constructor uses it", self.decls[fid].name));
                return;
            }
        }
    }

    /// Whether a reference initialization binds a temporary object
    /// (CTOR-17).
    pub fn binds_temporary(e: &Expr) -> bool {
        matches!(&e.k, EK::Conv(Conv::Bind, x) if x.cat == Cat::PR)
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
            if kind == FnKind::Ctor && !self.in_template() {
                if let Some(c) = class {
                    self.check_virtual_base_inits(c, fid);
                }
            }
            let body = self.parse_compound_in(param_scope)?;
            if is_try {
                let hs = self.parse_handlers()?;
                let nonvoid = self.func(fid).is_some_and(|f| matches!(f.kind, FnKind::Normal | FnKind::Operator | FnKind::Conversion) && f.fty.ret.t != Ty::Void && !f.fty.ret.is_unknown() && !f.is_main);
                if nonvoid && !self.in_template() && hs.iter().any(|h| can_complete(self, &h.body)) {
                    // TRY-12: a handler of a function try block that can
                    // reach its end in a function that returns a value.
                    self.warn(self.decls[fid].line, "TRY-12", "a handler of the function try block can reach its end without a return statement");
                }
                if self.func(fid).is_some_and(|f| f.kind == FnKind::Ctor) {
                    // TRY-13: a handler of a constructor's function try
                    // block that names a non-static member.
                    if !self.in_template() && hs.iter().any(|h| names_member(&h.body, self)) {
                        self.warn(self.decls[fid].line, "TRY-13", "a handler of a constructor's function try block names a non-static member");
                    }
                    // TRY-17: no return in a handler of a constructor's
                    // function try block.
                    if hs.iter().any(|h| has_return(&h.body)) {
                        self.error(self.decls[fid].line, "TRY-17", "a return statement in a handler of a constructor's function try block");
                    }
                }
                if let Some(f) = self.func_mut(fid) {
                    // Overload resolution still treats the function as one
                    // it does not model; the interpreter runs it.
                    f.opaque = true;
                    f.fn_try = Some(Rc::new(hs));
                }
            }
            Ok(body)
        })();
        self.stack.pop();
        let depth = self.fns.len();
        self.check_jumps(depth);
        let ctx = self.fns.pop().unwrap();
        let body = r?;
        if !deduce && !self.in_template() && self.func(fid).is_some_and(|f| matches!(f.kind, FnKind::Normal | FnKind::Operator | FnKind::Conversion) && f.fty.ret.t != Ty::Void && !f.fty.ret.is_unknown() && !f.is_main && f.fn_try.is_none()) && falls_off(self, &body) {
            // FN-7: control can reach the end of a function that returns
            // a value.
            self.warn(self.decls[fid].line, "FN-7", format!("control can reach the end of '{}', which returns a value", self.decls[fid].name));
        }
        if kind == FnKind::Ctor {
            // INST-10: a constructor uses the default member initializers
            // of the members that it does not initialize.
            if let Some(c) = class.filter(|c| self.inst_of.contains_key(c)) {
                let inits: Vec<DeclId> = self.func(fid).map(|f| f.mem_inits.iter().filter_map(|m| m.field).collect()).unwrap_or_default();
                let delegating = self.func(fid).is_some_and(|f| f.mem_inits.iter().any(|m| m.base == Some(c)));
                if !delegating {
                    for fd in self.classes[c].fields.clone() {
                        if !inits.contains(&fd) {
                            if let Some(errs) = self.body_errors.remove(&fd) {
                                self.d.list.extend(errs);
                            }
                        }
                    }
                }
            }
        }
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
            // In a template, a return value of a dependent type leaves the
            // type to the specialization (DEP-2).
            let t = ctx.deduced.clone().unwrap_or(if self.in_template() { QTy::unknown() } else { QTy::new(Ty::Void) });
            if !self.in_template() {
                // CPT-22: the type constraint of the return type.
                if let Some((rds, rd)) = ctx.ret_auto.clone() {
                    self.placeholder_check(&rds, &rd, &t, "CPT-22", self.decls[fid].line);
                }
            }
            if let Some(f) = self.func_mut(fid) {
                f.fty.ret = t;
            }
        }
        // UNW-8: a throw in a non-throwing function that no enclosing
        // handler of the function catches.
        if !self.in_template() && self.func(fid).is_some_and(|f| f.fty.noexcept && f.fn_try.is_none()) {
            let mut found = None;
            unw8_stmt(self, &body, &mut Vec::new(), &mut found);
            if let Some(line) = found {
                self.warn(line, "UNW-8", "a throw in a non-throwing function that no handler of the function catches");
            }
        }
        // ATTR-14: control should not return from a noreturn function.
        if self.has_attr(fid, "noreturn") && (has_return(&body) || can_complete(self, &body)) {
            self.warn(self.decls[fid].line, "ATTR-14", "control can return from a 'noreturn' function");
        }
        if let Some(f) = self.func_mut(fid) {
            f.body = Some(Rc::new(body));
        }
        if kind == FnKind::Ctor && !self.in_template() {
            if let Some(c) = class {
                self.check_ctor_dmi_temps(fid, c);
            }
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
            // PACK-14: `Bs(args)...` initializes each base of the pack.
            let (items, expanded) = self.parse_expansion(|p| {
                // mem-initializer-id
                let start = p.pos;
                let scope = p.parse_nested_name()?;
                let name = p.ident()?;
                let mut named_spec: Option<usize> = None;
                if p.at("<") {
                    // A template-id names the specialization, also one of a
                    // pack expansion (PACK-14).
                    let after_name = p.pos;
                    p.pos = start;
                    let mark = p.d.list.len();
                    match p.parse_type_id() {
                        Ok(QTy { t: Ty::Class(b), .. }) if p.d.list.len() == mark && (p.at("(") || p.at("{")) => named_spec = Some(b),
                        _ => {
                            p.d.list.truncate(mark);
                            p.pos = after_name;
                            p.skip_template_args()?;
                        }
                    }
                }
                let mut target = match (scope, class) {
                    _ if named_spec.is_some() => {
                        let b = named_spec.unwrap();
                        let bname = p.classes[b].name.clone();
                        p.lookup_in_scope(p.classes[b].scope, &bname, false)
                    }
                    (None, Some(c)) => p.lookup_member(c, &name).or_else(|| p.lookup(&name)),
                    (Some(Ok(s)), _) => p.lookup_in_scope(s, &name, true),
                    _ => None,
                };
                if let Some(ids) = &target {
                    if let Some(v) = p.pack_binds.get(&ids[0]).cloned() {
                        p.last_pack_ref = Some(ids[0]);
                        p.pack_refs.push(v.len());
                        if let Some(TArg::Type(QTy { t: Ty::Class(b), .. })) = v.get(p.pack_index.unwrap_or(0)) {
                            let b = *b;
                            let bname = p.classes[b].name.clone();
                            target = p.lookup_in_scope(p.classes[b].scope, &bname, false);
                        }
                    }
                }
                let mut args = Vec::new();
                let braced = p.at("{");
                if braced {
                    args.push(p.parse_braced_init()?);
                } else {
                    p.expect("(")?;
                    // PACK-13: an argument can be a pack expansion.
                    args = p.parse_call_args()?;
                }
                Ok((name, target, args, braced))
            })?;
            if !expanded {
                self.eat("...");
            }
            for (name, target, mut args, braced) in items {
            if expanded && !self.in_template() && target.as_ref().is_some_and(|ids| matches!(self.decls[ids[0]].k, DK::Field(_))) {
                // PACK-15: a pack expansion of a mem-initializer names a
                // base.
                self.error(line, "PACK-15", format!("the pack expansion in the mem-initializer names the data member '{}'", name));
            }
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
            let field = target.as_ref().and_then(|ids| ids.iter().copied().find(|&i| matches!(&self.decls[i].k, DK::Field(fi) if Some(fi.class) == class || class.is_some_and(|c| self.anon_top(fi.class, c).is_some()))));
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
                        let e = Expr::new(EK::InitList(args.clone()), QTy::unknown(), Cat::PR, line);
                        Some(self.convert_init(e, &fty, InitForm::Paren(n), line))
                    };
                    if let Some(x) = &e {
                        if self.member_binds_temporary(&fty, x, braced) {
                            // CTOR-17 (Q-0084): the temporary ends with the
                            // constructor.
                            self.error(line, "CTOR-17", format!("the mem-initializer binds the member '{}' to a temporary object", name));
                        }
                    }
                    e.map(Rc::new)
                }
                _ => None,
            };
            if let Some(fd) = field {
                let top = match (&self.decls[fd].k, class) {
                    (DK::Field(fi), Some(c)) => self.anon_top(fi.class, c).unwrap_or(fd),
                    _ => fd,
                };
                if let DK::Field(fi) = &self.decls[top].k {
                    field_order.push((fi.index, line));
                }
            }
            // A base or a delegation: the constructor call, for the
            // interpreter. Its diagnostics are dropped, since the checks
            // above already judged the mem-initializer.
            let base = if field.is_none() && !self.in_template() {
                target.as_ref().and_then(|ids| match &self.decls[ids[0]].k {
                    DK::Class(b) => Some(*b),
                    DK::Type(q) => match q.t {
                        Ty::Class(b) => Some(b),
                        _ => None,
                    },
                    _ => None,
                })
            } else {
                None
            };
            let init = match (base, class) {
                (Some(b), Some(c)) if init.is_none() && (b == c || self.classes[c].bases.iter().any(|x| x.class == Some(b)) || self.has_virtual_base(c, b, 0)) && !self.class_uncertain(b) => {
                    let mark = self.d.list.len();
                    let ty = QTy::new(Ty::Class(b));
                    let e = if braced {
                        self.convert_init(args[0].clone(), &ty, InitForm::DirectList, line)
                    } else if args.is_empty() {
                        let e = Expr::new(EK::InitList(vec![]), QTy::unknown(), Cat::PR, line);
                        self.convert_init(e, &ty, InitForm::DirectList, line)
                    } else {
                        let n = args.len();
                        let e = Expr::new(EK::InitList(args.clone()), QTy::unknown(), Cat::PR, line);
                        self.convert_init(e, &ty, InitForm::Paren(n), line)
                    };
                    self.d.list.truncate(mark);
                    if matches!(e.k, EK::Unknown(_)) { None } else { Some(Rc::new(e)) }
                }
                _ => init,
            };
            out.push(MemInit { field, base, init });
            }
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
        if let Some(c) = class {
            // UNI-8: one variant member of a union is initialized.
            if self.classes[c].key == 2 && out.iter().filter(|m: &&MemInit| m.field.is_some()).count() > 1 && !self.in_template() {
                self.error(self.line(), "UNI-8", "mem-initializers for two members of a union");
            }
        }
        Ok(out)
    }

    /// DER-13: a constructor of a class that can be the most derived
    /// names each virtual base that has no default constructor.
    fn check_virtual_base_inits(&mut self, c: usize, fid: DeclId) {
        if !self.any_virtual_base(c, 0) || self.class_uncertain(c) || self.is_abstract(&Ty::Class(c)) {
            return;
        }
        let inits: Vec<Option<usize>> = self.func(fid).map(|f| f.mem_inits.iter().map(|m| m.base).collect()).unwrap_or_default();
        if inits.contains(&Some(c)) {
            return;
        }
        let mut vbases = Vec::new();
        let mut todo = vec![(c, 0u32)];
        while let Some((x, d)) = todo.pop() {
            if d > 16 {
                continue;
            }
            for b in &self.classes[x].bases {
                if let Some(bc) = b.class {
                    if b.virt && !vbases.contains(&bc) {
                        vbases.push(bc);
                    }
                    todo.push((bc, d + 1));
                }
            }
        }
        for v in vbases {
            let ci = &self.classes[v];
            if !ci.complete || ci.ctors.is_empty() || self.class_uncertain(v) || inits.contains(&Some(v)) {
                continue;
            }
            let no_default = ci.ctors.iter().all(|&k| self.func(k).is_some_and(|f| !f.template && !f.fty.variadic && f.params.len() > f.defaults && !self.inherited_from.contains_key(&k)));
            if no_default {
                let line = self.decls[fid].line;
                self.error(line, "DER-13", format!("the constructor of '{}' does not initialize the virtual base '{}', which has no default constructor", self.classes[c].name, self.classes[v].name));
            }
        }
    }

    /// Whether `c` has a virtual base, at any depth.
    pub fn any_virtual_base(&self, c: usize, depth: u32) -> bool {
        depth < 16 && self.classes[c].bases.iter().any(|x| x.virt || x.class.is_some_and(|b| self.any_virtual_base(b, depth + 1)))
    }

    /// Whether `b` is a virtual base of `c`, at any depth (DER-6).
    pub fn has_virtual_base(&self, c: usize, b: usize, depth: u32) -> bool {
        depth < 16
            && self.classes[c].bases.iter().any(|x| match x.class {
                Some(bc) => (x.virt && bc == b) || self.has_virtual_base(bc, b, depth + 1),
                None => false,
            })
    }

    pub fn parse_handlers(&mut self) -> PR<Vec<Handler>> {
        if !self.at("catch") {
            return Err(self.err("try block without a handler"));
        }
        let mut out = Vec::new();
        while self.at("catch") {
            let hline = self.line();
            self.pos += 1;
            self.expect("(")?;
            self.push_scope(SK::Block);
            let r = (|| -> PR<Handler> {
                let mut var = None;
                let mut hty = None;
                if !self.eat("...") {
                    let ds = self.parse_decl_specs(None)?;
                    let base = self.ds_base(&ds)?;
                    let d = self.parse_declarator(true, true)?;
                    let ty = self.apply_ops(base, &d.ops, d.line);
                    self.check_handler_param(&ds, &d, &ty);
                    hty = Some(ty.clone());
                    if let Some(n) = d.name {
                        let vi = VarInfo { ty, storage: Storage::Auto, constexpr: false, constinit: false, init: None, param: false, local: true, dependent: false, defined: true, class: None };
                        var = Some(self.add_decl(&n, DK::Var(vi), d.line));
                    }
                }
                self.expect(")")?;
                let s = self.cur_scope();
                self.enter_try_region();
                let body = self.parse_compound_in(s);
                self.try_regions.pop();
                Ok(Handler { var, ty: hty, body: Box::new(body?) })
            })();
            self.pop_scope();
            let h = r?;
            if out.last().is_some_and(|x: &Handler| x.ty.is_none()) {
                // TRY-3: a handler for `...` comes last.
                self.error(hline, "TRY-3", "a handler after the handler for '...'");
            } else if !self.in_template() {
                if let Some(t) = &h.ty {
                    let t2 = nonref(t).unqual();
                    if !t2.is_unknown() && out.iter().any(|x| x.ty.is_some() && !x.ty.as_ref().unwrap().is_unknown() && handler_catches(self, &x.ty, Some(&t2))) {
                        // TRY-4: an earlier handler always takes this one's
                        // exceptions.
                        self.warn(hline, "TRY-4", "an earlier handler of the try block always catches what this handler catches");
                    }
                }
            }
            out.push(h);
        }
        Ok(out)
    }

    /// TRY-5: what a handler parameter must not be.
    fn check_handler_param(&mut self, ds: &crate::decl::DeclSpec, d: &crate::decl::Declarator, ty: &QTy) {
        let line = d.line;
        if !ds.storage.is_empty() || ds.register || ds.constexpr || ds.inline {
            self.error(line, "TRY-5", "a handler parameter with a storage class");
            return;
        }
        if ds.auto.is_some() {
            self.error(line, "TRY-5", "a handler parameter declared with 'auto'");
            return;
        }
        if d.qual.is_some() || d.qual_unknown {
            self.error(line, "TRY-5", "a handler parameter with a qualified name");
            return;
        }
        if self.in_template() || ty.is_unknown() {
            return;
        }
        match &ty.t {
            Ty::Void => self.error(line, "TRY-5", "a handler parameter of type void"),
            Ty::RRef(_) => self.error(line, "TRY-5", "a handler parameter of rvalue reference type"),
            Ty::LRef(x) if matches!(x.t, Ty::Array(_, None)) => self.error(line, "TRY-5", "a handler parameter that refers to an array of unknown bound"),
            Ty::Ptr(x) if matches!(x.t, Ty::Array(_, None)) => self.error(line, "TRY-5", "a handler parameter that points to an array of unknown bound"),
            _ => {}
        }
        let (inner, by_value) = match &ty.t {
            Ty::Ptr(x) | Ty::LRef(x) | Ty::RRef(x) => ((**x).clone(), false),
            _ => (ty.clone(), true),
        };
        if let Ty::Class(c) = inner.t {
            self.require_class(c, line);
            if !self.classes[c].complete && !self.class_uncertain(c) {
                self.error(line, "TRY-5", format!("a handler parameter of incomplete class '{}'", self.classes[c].name));
            } else if by_value && self.is_abstract(&inner.t) {
                self.error(line, "TRY-5", format!("a handler parameter of abstract class '{}' by value", self.classes[c].name));
            } else if by_value && self.classes[c].complete && !self.class_uncertain(c) {
                // TRY-5: the parameter is copied from the exception
                // object, so the copy constructor is usable.
                let name = self.classes[c].name.clone();
                if let crate::special::CopySt::Deleted(_) = self.lv_status(c, false, 0) {
                    self.error(line, "TRY-5", format!("a handler parameter of '{}', whose copy constructor is deleted", name));
                } else if let Some(f) = self.sm_decl(c, crate::special::Sm::CopyCtor) {
                    if !self.accessible(f, false) {
                        self.error(line, "TRY-5", format!("a handler parameter of '{}', whose copy constructor is not accessible", name));
                    }
                }
            }
        }
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
        Stmt::Try(b, hs) => has_return(b) || hs.iter().any(|h| has_return(&h.body)),
        _ => false,
    }
}

/// ATTR-14: whether control can reach the end of a statement. The
/// analysis is simple: a throw, a call of a noreturn function, and a loop
/// without a condition or a break end control; anything else can complete.
/// TRY-13: whether a statement names a non-static data member of the
/// object.
fn names_member(s: &Stmt, p: &P) -> bool {
    let mut found = false;
    let mut ex = |e: &Expr| {
        walk_expr(e, &mut |x| {
            if let EK::Member(o, f) = &x.k {
                if matches!(o.k, EK::Unary(UOp::Deref, ref t) if matches!(t.k, EK::This)) && matches!(p.decls[*f].k, DK::Field(_)) {
                    found = true;
                }
            }
        })
    };
    fn go(s: &Stmt, p: &P, ex: &mut dyn FnMut(&Expr)) {
        match s {
            Stmt::Expr(e) | Stmt::Return(Some(e)) => ex(e),
            Stmt::Decl(ids) => {
                for &d in ids {
                    if let DK::Var(v) = &p.decls[d].k {
                        if let Some(i) = &v.init {
                            ex(i);
                        }
                    }
                }
            }
            Stmt::Block(v) => v.iter().for_each(|x| go(x, p, ex)),
            Stmt::If { then, els, .. } => {
                go(then, p, ex);
                if let Some(e) = els {
                    go(e, p, ex);
                }
            }
            _ => {}
        }
    }
    go(s, p, &mut ex);
    found
}

/// Whether a handler of type `h` (None for `...`) catches an exception
/// object of static type `e` (TRY-2). None for a rethrow.
fn handler_catches(p: &P, h: &Option<QTy>, e: Option<&QTy>) -> bool {
    let Some(h) = h else { return true };
    let Some(e) = e else { return false };
    let h = nonref(h).unqual();
    let e = e.unqual();
    if h.is_unknown() || e.is_unknown() || h.t == e.t {
        return true;
    }
    match (&h.t, &e.t) {
        (Ty::Class(b), Ty::Class(d)) => p.is_base_of(*b, *d),
        (Ty::Ptr(hb), Ty::Ptr(ed)) => match (&hb.t, &ed.t) {
            (Ty::Class(b), Ty::Class(d)) => p.is_base_of(*b, *d),
            (Ty::Void, _) => true,
            (x, y) => x == y,
        },
        (Ty::Ptr(_), Ty::NullPtr) => true,
        _ => false,
    }
}

/// UNW-8: find a throw that the enclosing handlers do not catch.
fn unw8_stmt(p: &P, s: &Stmt, hs: &mut Vec<Vec<Option<QTy>>>, found: &mut Option<u32>) {
    if found.is_some() {
        return;
    }
    let ex = |e: &Expr, hs: &mut Vec<Vec<Option<QTy>>>, found: &mut Option<u32>| {
        walk_expr(e, &mut |x| {
            if found.is_some() {
                return;
            }
            if let EK::Throw(op) = &x.k {
                let t = op.as_ref().map(|o| {
                    let t = nonref(&o.ty);
                    match &t.t {
                        Ty::Array(el, _) => QTy::new(Ty::Ptr(el.clone())),
                        Ty::Func(_) => QTy::new(Ty::Ptr(Box::new(t.clone()))),
                        _ => t,
                    }
                });
                if t.as_ref().is_some_and(|t| t.is_unknown()) {
                    return;
                }
                if !hs.iter().any(|v| v.iter().any(|h| handler_catches(p, h, t.as_ref()))) {
                    *found = Some(x.line);
                }
            }
        });
    };
    match s {
        Stmt::Expr(e) | Stmt::Assume(e) => ex(e, hs, found),
        Stmt::Return(Some(e)) => ex(e, hs, found),
        Stmt::Decl(ids) => {
            for &d in ids {
                if let DK::Var(v) = &p.decls[d].k {
                    if let Some(i) = &v.init {
                        ex(i, hs, found);
                    }
                }
            }
        }
        Stmt::Block(v) => {
            for x in v {
                unw8_stmt(p, x, hs, found);
            }
        }
        Stmt::If { init, cond, then, els, .. } => {
            if let Some(i) = init {
                unw8_stmt(p, i, hs, found);
            }
            if let Some(CondK::Expr(e) | CondK::Decl(_, e)) = cond {
                ex(e, hs, found);
            }
            unw8_stmt(p, then, hs, found);
            if let Some(e) = els {
                unw8_stmt(p, e, hs, found);
            }
        }
        Stmt::While { cond: CondK::Expr(e) | CondK::Decl(_, e), body } => {
            ex(e, hs, found);
            unw8_stmt(p, body, hs, found);
        }
        Stmt::Do { body, cond } => {
            unw8_stmt(p, body, hs, found);
            ex(cond, hs, found);
        }
        Stmt::For { init, cond, inc, body } => {
            if let Some(i) = init {
                unw8_stmt(p, i, hs, found);
            }
            if let Some(CondK::Expr(e) | CondK::Decl(_, e)) = cond {
                ex(e, hs, found);
            }
            if let Some(e) = inc {
                ex(e, hs, found);
            }
            unw8_stmt(p, body, hs, found);
        }
        Stmt::Switch { init, cond: CondK::Expr(e) | CondK::Decl(_, e), body } => {
            if let Some(i) = init {
                unw8_stmt(p, i, hs, found);
            }
            ex(e, hs, found);
            unw8_stmt(p, body, hs, found);
        }
        Stmt::Case(_, b) | Stmt::Default(b) | Stmt::Label(_, b) => unw8_stmt(p, b, hs, found),
        Stmt::Try(body, handlers) => {
            hs.push(handlers.iter().map(|h| h.ty.clone()).collect());
            unw8_stmt(p, body, hs, found);
            hs.pop();
            // TRY-8: a handler is not inside its own try block.
            for h in handlers {
                unw8_stmt(p, &h.body, hs, found);
            }
        }
        _ => {}
    }
}

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
        Stmt::Try(b, hs) => can_complete(p, b) || hs.iter().any(|h| can_complete(p, &h.body)),
        _ => true,
    }
}

/// FN-7: whether control can reach the end of `s`. When the answer is
/// not clear, such as after a label or in a discarded branch, the
/// function says no, so no warning comes.
fn falls_off(p: &P, s: &Stmt) -> bool {
    fn breaks(s: &Stmt) -> bool {
        match s {
            Stmt::Break => true,
            Stmt::Block(v) => v.iter().any(breaks),
            Stmt::If { then, els, .. } => breaks(then) || els.as_deref().is_some_and(breaks),
            Stmt::Label(_, x) | Stmt::Case(_, x) | Stmt::Default(x) => breaks(x),
            Stmt::Try(b, hs) => breaks(b) || hs.iter().any(|h| breaks(&h.body)),
            _ => false,
        }
    }
    fn has_default(s: &Stmt) -> bool {
        match s {
            Stmt::Default(_) => true,
            Stmt::Block(v) => v.iter().any(has_default),
            Stmt::Label(_, x) | Stmt::Case(_, x) => has_default(x),
            _ => false,
        }
    }
    fn has_label(s: &Stmt) -> bool {
        match s {
            Stmt::Label(..) => true,
            Stmt::Block(v) => v.iter().any(has_label),
            Stmt::If { then, els, .. } => has_label(then) || els.as_deref().is_some_and(has_label),
            Stmt::While { body, .. } | Stmt::Do { body, .. } | Stmt::For { body, .. } | Stmt::Switch { body, .. } => has_label(body),
            Stmt::Case(_, x) | Stmt::Default(x) => has_label(x),
            Stmt::Try(b, hs) => has_label(b) || hs.iter().any(|h| has_label(&h.body)),
            _ => false,
        }
    }
    let always = |c: Option<&CondK>| match c {
        None => true,
        Some(CondK::Expr(e)) => p.const_int_quiet(e).is_ok_and(|v| v != 0),
        Some(CondK::Decl(..)) => false,
    };
    match s {
        Stmt::Return(_) | Stmt::Goto(_) | Stmt::Unsupported(_) | Stmt::Break | Stmt::Continue => false,
        Stmt::Expr(e) => match &e.k {
            EK::Throw(_) => false,
            EK::Call(_, _, Some(f)) => !p.has_attr(*f, "noreturn"),
            EK::Unknown(_) => false,
            _ => true,
        },
        Stmt::Block(v) => !v.iter().any(has_label) && v.iter().all(|x| falls_off(p, x)),
        Stmt::If { then, els, kind, cond, .. } => match kind {
            // FN-7: a constant condition selects one branch.
            IfKind::Normal if !has_label(then) && !els.as_deref().is_some_and(has_label) && matches!(cond, Some(CondK::Expr(e)) if p.const_int_quiet(e).is_ok()) => {
                let Some(CondK::Expr(e)) = cond else { unreachable!() };
                if p.const_int_quiet(e).is_ok_and(|v| v != 0) {
                    falls_off(p, then)
                } else {
                    els.as_deref().is_none_or(|e| falls_off(p, e))
                }
            }
            IfKind::Normal => falls_off(p, then) || els.as_deref().is_none_or(|e| falls_off(p, e)),
            _ => falls_off(p, then) && els.as_deref().is_none_or(|e| falls_off(p, e)),
        },
        Stmt::While { cond, body } => !always(Some(cond)) || breaks(body),
        Stmt::For { cond, body, .. } => !always(cond.as_ref()) || breaks(body),
        Stmt::Do { body, cond } => {
            if breaks(body) {
                return true;
            }
            let c = CondK::Expr(cond.clone());
            !always(Some(&c)) && falls_off(p, body)
        }
        Stmt::Switch { body, .. } => breaks(body) || !has_default(body) || falls_off(p, body),
        Stmt::Label(..) => false,
        Stmt::Case(_, x) | Stmt::Default(x) => falls_off(p, x),
        Stmt::Try(b, hs) => falls_off(p, b) || hs.iter().any(|h| falls_off(p, &h.body)),
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

/// Whether the arguments of a member definition are only parameters,
/// each once.
fn primary_like(pats: &[TArg]) -> bool {
    let mut seen = Vec::new();
    for a in pats {
        let k = match a {
            TArg::Type(q) if !q.c && !q.v => match &q.t {
                Ty::Dep(d) => match &**d {
                    Dep::Param(x, y) => (*x, *y),
                    _ => return false,
                },
                _ => return false,
            },
            TArg::Val(TVal::Param(x, y)) => (*x, *y),
            TArg::Templ(TName::Param(x, y)) => (*x, *y),
            _ => return false,
        };
        if seen.contains(&k) {
            return false;
        }
        seen.push(k);
    }
    true
}

/// An array type with its bound left out, for C names declared with and
/// without a bound.
fn strip_array_bound(t: &Ty) -> Ty {
    match t {
        Ty::Array(e, _) => Ty::Array(e.clone(), None),
        t => t.clone(),
    }
}

/// The type that declarator operators `ops` were applied to, to give `t`
/// (AUTO-22).
fn peel_ops(t: &QTy, ops: &[DOp]) -> Option<QTy> {
    let mut t = t.clone();
    for op in ops.iter().rev() {
        t = match (op, &t.t) {
            (DOp::Ptr(..), Ty::Ptr(x)) => (**x).clone(),
            (DOp::LRef | DOp::RRef, Ty::LRef(x) | Ty::RRef(x)) => (**x).clone(),
            (DOp::Array(_), Ty::Array(x, _)) => (**x).clone(),
            _ => return None,
        };
    }
    Some(t)
}
