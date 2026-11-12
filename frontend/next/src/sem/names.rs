//! The declare pass of build step 2: a scope graph that records names
//! and their kinds, and the `Names` oracle over it. It knows no types
//! and does no overload resolution. Step 3 replaces it with the entities
//! and scopes of `sem` (`DESIGN-rewrite.md`, sections 2 and 5).
//!
//! The search follows the simple parts of `foundations/names`: a name is
//! found in the innermost scope that declares it, a class scope also
//! searches its independent bases, and a using-directive or an inline or
//! unnamed namespace makes its namespace's names visible. A class with a
//! dependent base answers "unknown" for a name it does not declare.

use crate::lex::{Token, TK};
use crate::syntax::names::{DeclCtx, Enter, Found, Kind, Names, Qual, TKind};
use crate::syntax::{Kid, Node};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SK {
    Namespace,
    Class,
    Enum,
    Block,
    Template,
    Prototype,
}

#[derive(Clone, Copy, Debug)]
struct Entry {
    kind: Kind,
    scope: Option<u32>,
    param: bool,
}

#[derive(Debug)]
struct Scope {
    kind: SK,
    parent: Option<u32>,
    name: String,
    names: HashMap<String, Entry>,
    children: HashMap<String, u32>,
    bases: Vec<u32>,
    dep_base: bool,
    using: Vec<u32>,
}

impl Scope {
    fn new(kind: SK, name: &str, parent: Option<u32>) -> Scope {
        Scope { kind, parent, name: name.to_string(), names: HashMap::new(), children: HashMap::new(), bases: Vec::new(), dep_base: false, using: Vec::new() }
    }
}

/// The oracle of build step 2.
pub struct Declare {
    scopes: Vec<Scope>,
    stack: Vec<u32>,
    member_templates: HashSet<String>,
}

impl Default for Declare {
    fn default() -> Self {
        Declare { scopes: vec![Scope::new(SK::Namespace, "", None)], stack: vec![0], member_templates: HashSet::new() }
    }
}

fn found(e: &Entry) -> Found {
    Found { kind: e.kind, scope: e.scope, param: e.param }
}

/// The text of the identifier tokens among a node's direct children.
fn direct_idents<'t>(n: &Node, toks: &'t [Token]) -> Vec<&'t str> {
    n.kids
        .iter()
        .filter_map(|k| match k {
            Kid::Tok(t) if toks[*t].kind == TK::Ident && !crate::foundations::keywords::is_keyword(&toks[*t].text) => Some(toks[*t].text.as_str()),
            _ => None,
        })
        .collect()
}

impl Declare {
    /// Every name the declare pass recorded, in any scope.
    pub fn all_names(&self) -> std::collections::HashSet<String> {
        self.scopes.iter().flat_map(|s| s.names.keys().cloned()).collect()
    }

    fn lookup_in(&self, s: u32, name: &str, depth: u32) -> Option<Found> {
        if depth > 24 {
            return None;
        }
        let sc = &self.scopes[s as usize];
        if let Some(e) = sc.names.get(name) {
            return Some(found(e));
        }
        for &b in &sc.bases {
            if let Some(f) = self.lookup_in(b, name, depth + 1) {
                return Some(f);
            }
        }
        for &u in &sc.using {
            if let Some(f) = self.lookup_in(u, name, depth + 1) {
                return Some(f);
            }
        }
        None
    }

    /// The scope a new declaration goes into: the innermost scope that is
    /// not a template head's or a prototype's.
    fn target(&self) -> u32 {
        *self.stack.iter().rev().find(|&&s| !matches!(self.scopes[s as usize].kind, SK::Template | SK::Prototype)).unwrap_or(&0)
    }

    fn innermost(&self) -> u32 {
        *self.stack.last().unwrap_or(&0)
    }

    fn add(&mut self, s: u32, name: &str, e: Entry) {
        if name.is_empty() {
            return;
        }
        let names = &mut self.scopes[s as usize].names;
        match names.get(name) {
            // A redeclaration, an instantiation, or an overload of a
            // template keeps the name a template.
            Some(old) if matches!(old.kind, Kind::Template(_)) && e.kind == Kind::Value => {}
            // A class and a variable of the same name: the variable hides
            // the class; a later redeclaration of the class keeps it so.
            Some(old) if old.kind == Kind::Value && e.kind == Kind::Type && e.scope.is_some() => {}
            Some(old) if old.scope.is_some() && e.scope.is_none() && old.kind != Kind::Value && e.kind != Kind::Value => {}
            _ => {
                names.insert(name.to_string(), e);
            }
        }
    }

    fn new_scope(&mut self, parent: u32, kind: SK, name: &str) -> u32 {
        if !name.is_empty() {
            if let Some(&c) = self.scopes[parent as usize].children.get(name) {
                if self.scopes[c as usize].kind == kind {
                    return c;
                }
            }
        }
        self.scopes.push(Scope::new(kind, name, Some(parent)));
        let id = self.scopes.len() as u32 - 1;
        if !name.is_empty() {
            self.scopes[parent as usize].children.insert(name.to_string(), id);
        }
        id
    }

    /// The scope a `nested_name` node names.
    fn resolve_nested(&self, nn: &Node, toks: &[Token]) -> Qual {
        let mut q = Qual::None;
        for k in &nn.kids {
            match k {
                Kid::Node(n) if n.prod == "nested_name" => q = self.resolve_nested(n, toks),
                Kid::Node(n) if (n.prod == "decltype_spec" || n.prod == "decltype_expr") => q = Qual::Dependent,
                Kid::Node(n) if n.prod == "template_id" => {
                    let name = n.find("template_name").map(|t| direct_idents(t, toks)).and_then(|v| v.first().copied()).unwrap_or("");
                    let dep = self.args_dependent(n, toks);
                    q = match (dep && q == Qual::None).then(|| self.current(name)).flatten() {
                        Some(s) => Qual::Scope(s),
                        None if dep && self.bare_params(n, toks) && q != Qual::Dependent => match self.find(q, name) {
                            Found { kind: Kind::Template(TKind::Class), scope: Some(s), .. } => Qual::Scope(s),
                            _ => Qual::Dependent,
                        },
                        None => self.step(q, name, dep),
                    };
                }
                Kid::Tok(t) if toks[*t].text == "::" && q == Qual::None && nn.kids.len() == 1 => q = Qual::Global,
                Kid::Tok(t) if toks[*t].text == "::" => {}
                Kid::Tok(t) if toks[*t].kind == TK::Ident && toks[*t].text != "template" => q = self.step(q, &toks[*t].text, false),
                _ => {}
            }
        }
        q
    }

    fn step(&self, q: Qual, name: &str, dep: bool) -> Qual {
        if q == Qual::Dependent || dep {
            return Qual::Dependent;
        }
        let f = self.find(q, name);
        if f.param {
            return Qual::Dependent;
        }
        match (f.kind, f.scope) {
            (Kind::Namespace | Kind::Type | Kind::Template(TKind::Class), Some(s)) => Qual::Scope(s),
            _ => Qual::Dependent,
        }
    }

    fn bare_params(&self, tid: &Node, toks: &[Token]) -> bool {
        tid.child("template_args").is_some_and(|a| {
            a.children("template_arg").all(|arg| {
                let mut ids = Vec::new();
                arg.tokens(&mut ids);
                ids.len() == 1 && toks[ids[0]].kind == TK::Ident && self.find(Qual::None, &toks[ids[0]].text).param
            })
        })
    }

    fn args_dependent(&self, tid: &Node, toks: &[Token]) -> bool {
        let mut ids = Vec::new();
        if let Some(a) = tid.child("template_args") {
            a.tokens(&mut ids);
        }
        ids.iter().any(|&i| toks[i].kind == TK::Ident && self.find(Qual::None, &toks[i].text).param)
    }

    /// The scope of the type a `type_id` or specifiers name, for an alias.
    fn type_scope(&self, n: &Node, toks: &[Token]) -> Option<u32> {
        let st = n.find("simple_type")?;
        let mut q = Qual::None;
        if let Some(nn) = st.child("nested_name") {
            q = self.resolve_nested(nn, toks);
        }
        if let Some(t) = st.child("template_id") {
            let name = t.find("template_name").map(|x| direct_idents(x, toks)).and_then(|v| v.first().copied())?;
            return self.find(q, name).scope;
        }
        let name = *direct_idents(st, toks).last()?;
        self.find(q, name).scope
    }

    /// The name and the qualifier of a declarator's `declarator_id`.
    fn declarator_name<'t>(&self, d: &Node, toks: &'t [Token]) -> Option<&'t str> {
        let id = d.find("declarator_id")?;
        let ide = id.child("id_expression")?;
        let u = ide.child("unqualified_id")?;
        if u.child("template_id").is_some() {
            return None;
        }
        direct_idents(u, toks).first().copied()
    }

    fn class_like(&mut self, n: &Node, toks: &[Token], ctx: DeclCtx) -> Vec<u32> {
        let mut target = self.target();
        if let Some(nn) = n.child("nested_name") {
            if let Qual::Scope(s) = self.resolve_nested(nn, toks) {
                target = s;
            }
        }
        let (name, special) = if let Some(c) = n.child("class_name") {
            (direct_idents(c, toks).first().copied().unwrap_or(""), false)
        } else if let Some(t) = n.child("template_id") {
            (t.find("template_name").map(|x| direct_idents(x, toks)).and_then(|v| v.first().copied()).unwrap_or(""), true)
        } else {
            ("", false)
        };
        if special {
            // A specialization: it adds members to no new name. The
            // primary's scope serves its body.
            let f = self.find(if target == self.target() { Qual::None } else { Qual::Scope(target) }, name);
            let s = f.scope.unwrap_or_else(|| self.new_scope(target, SK::Class, ""));
            return vec![self.copy_scope_for_specialization(target, s, name)];
        }
        let cs = self.new_scope(target, SK::Class, name);
        let kind = if ctx.template { Kind::Template(TKind::Class) } else { Kind::Type };
        let e = Entry { kind, scope: Some(cs), param: false };
        if !name.is_empty() {
            // A variable or function declared first keeps the name; the
            // class is then found only after a class key (NAM-41).
            let hidden = self.scopes[target as usize].names.get(name).is_some_and(|old| old.kind == Kind::Value);
            if !hidden {
                self.scopes[target as usize].names.insert(name.to_string(), e);
            }
            self.scopes[cs as usize].names.insert(name.to_string(), e);
        }
        if let Some(bc) = n.child("base_clause") {
            for b in bc.children("base_spec").filter_map(|b| b.child("base_type")) {
                let mut q = Qual::None;
                if let Some(nn) = b.child("nested_name") {
                    q = self.resolve_nested(nn, toks);
                }
                // A template-id base with dependent arguments, and a
                // `decltype` base, are dependent for this pass.
                let (bname, dep) = if let Some(t) = b.child("template_id") {
                    let name = t.find("template_name").map(|x| direct_idents(x, toks)).and_then(|v| v.first().copied()).unwrap_or("");
                    (name, self.args_dependent(t, toks))
                } else {
                    (b.child("class_name").map(|c| direct_idents(c, toks)).and_then(|v| v.first().copied()).unwrap_or(""), b.child("decltype_expr").is_some())
                };
                let f = if q == Qual::Dependent || dep { Found::UNKNOWN } else { self.find(q, bname) };
                match f.scope {
                    Some(s) if !f.param && q != Qual::Dependent => self.scopes[cs as usize].bases.push(s),
                    _ => self.scopes[cs as usize].dep_base = true,
                }
            }
        }
        vec![cs]
    }

    /// A partial or explicit specialization reads its body in a scope of
    /// its own whose lookups fall back to the primary's members. Its
    /// parent is the scope that a qualifier before its name names, so
    /// that the body sees that scope's names (NAM-21).
    fn copy_scope_for_specialization(&mut self, parent: u32, primary: u32, name: &str) -> u32 {
        let s = self.new_scope(parent, SK::Class, "");
        self.scopes[s as usize].name = name.to_string();
        if let Some(e) = self.scopes[primary as usize].names.get(name).copied() {
            // The injected name of the specialization names the
            // specialization's own scope.
            self.scopes[s as usize].names.insert(name.to_string(), Entry { scope: Some(s), ..e });
        }
        s
    }
}

impl Names for Declare {
    fn find(&self, q: Qual, name: &str) -> Found {
        match q {
            Qual::Dependent => Found::UNKNOWN,
            Qual::Global => self.lookup_in(0, name, 0).unwrap_or(Found::UNKNOWN),
            Qual::Scope(s) => {
                if let Some(f) = self.lookup_in(s, name, 0) {
                    return f;
                }
                Found::UNKNOWN
            }
            Qual::None => {
                // The open scopes, innermost first, and the enclosing
                // scopes of each, for a scope entered again (a member
                // defined outside its class, a deferred body).
                let mut seen = HashSet::new();
                for &s in self.stack.iter().rev() {
                    let mut c = Some(s);
                    while let Some(x) = c {
                        if !seen.insert(x) {
                            break;
                        }
                        if let Some(f) = self.lookup_in(x, name, 0) {
                            return f;
                        }
                        c = self.scopes[x as usize].parent;
                    }
                }
                Found::UNKNOWN
            }
        }
    }

    fn current(&self, name: &str) -> Option<u32> {
        let mut seen = HashSet::new();
        for &s in self.stack.iter().rev() {
            let mut c = Some(s);
            while let Some(x) = c {
                if !seen.insert(x) {
                    break;
                }
                let sc = &self.scopes[x as usize];
                if sc.kind == SK::Class && sc.name == name {
                    return Some(x);
                }
                c = sc.parent;
            }
        }
        None
    }

    fn member_template(&self, name: &str) -> bool {
        self.member_templates.contains(name)
    }

    fn class_name(&self) -> Option<String> {
        let s = self.stack.iter().rev().find(|&&s| self.scopes[s as usize].kind != SK::Template)?;
        let sc = &self.scopes[*s as usize];
        (sc.kind == SK::Class).then(|| sc.name.clone())
    }

    fn block_scope(&self) -> bool {
        self.scopes[self.target() as usize].kind == SK::Block
    }

    fn declared(&mut self, n: &Node, toks: &[Token], ctx: DeclCtx) -> Vec<u32> {
        match n.prod {
            "class_head" | "union_def" => self.class_like(n, toks, ctx),
            "enum_head" | "enum_opaque" => {
                let target = self.target();
                let Some(name) = direct_idents(n, toks).last().copied() else { return vec![] };
                let es = self.new_scope(target, SK::Enum, name);
                self.add(target, name, Entry { kind: Kind::Type, scope: Some(es), param: false });
                vec![es]
            }
            "elaborated_type" => {
                if n.child("template_id").is_some() || n.child("nested_name").is_some() {
                    return vec![];
                }
                let Some(name) = direct_idents(n, toks).last().copied() else { return vec![] };
                let f = self.find(Qual::None, name);
                if f.kind == Kind::Unknown || f.kind == Kind::Value || ctx.template {
                    let kind = if ctx.template { Kind::Template(TKind::Class) } else { Kind::Type };
                    let target = self.target();
                    let cs = self.new_scope(target, SK::Class, name);
                    self.add(target, name, Entry { kind, scope: Some(cs), param: false });
                    self.scopes[cs as usize].names.insert(name.to_string(), Entry { kind, scope: Some(cs), param: false });
                }
                vec![]
            }
            "declarator" => {
                let Some(name) = self.declarator_name(n, toks) else { return vec![] };
                let words: Vec<&str> = ctx.specs.map(|s| {
                    let mut ids = Vec::new();
                    s.tokens(&mut ids);
                    ids.iter().map(|&i| toks[i].text.as_str()).collect()
                }).unwrap_or_default();
                if words.contains(&"friend") {
                    return vec![];
                }
                let is_fn = crate::declarations::declarators::syntax::declares_function(n);
                // A function with a placeholder parameter type is a template.
                let abbreviated = crate::declarations::declarators::syntax::function_suffix_of(n)
                    .and_then(|fs| fs.child("parameters"))
                    .is_some_and(|ps| ps.children("parameter").any(|prm| prm.child("decl_specifiers").and_then(|s| s.find("placeholder")).is_some()));
                let ctx = DeclCtx { template: ctx.template || abbreviated, ..ctx };
                let (kind, scope) = if words.contains(&"typedef") {
                    (Kind::Type, ctx.specs.and_then(|s| self.type_scope(s, toks)))
                } else if ctx.template {
                    (Kind::Template(if is_fn { TKind::Function } else { TKind::Variable }), None)
                } else {
                    (Kind::Value, None)
                };
                let target = if ctx.param { self.innermost() } else { self.target() };
                if self.scopes[target as usize].kind == SK::Class && self.scopes[target as usize].name == name {
                    // A constructor: the name stays the class's.
                    return vec![];
                }
                if ctx.template && self.scopes[target as usize].kind == SK::Class {
                    self.member_templates.insert(name.to_string());
                }
                self.add(target, name, Entry { kind, scope, param: ctx.param });
                vec![]
            }
            "parameter" => {
                if let Some(d) = n.child("declarator") {
                    if let Some(name) = self.declarator_name(d, toks) {
                        let s = self.innermost();
                        self.scopes[s as usize].names.insert(name.to_string(), Entry { kind: Kind::Value, scope: None, param: ctx.param });
                    }
                }
                vec![]
            }
            "type_param" | "template_template_param" | "constant_param" => {
                if let Some(name) = direct_idents(n, toks).last().copied() {
                    let kind = match n.prod {
                        "type_param" => Kind::Type,
                        "template_template_param" => Kind::Template(TKind::Class),
                        _ => Kind::Value,
                    };
                    let s = self.innermost();
                    self.scopes[s as usize].names.insert(name.to_string(), Entry { kind, scope: None, param: true });
                }
                vec![]
            }
            "alias_decl" => {
                let Some(name) = direct_idents(n, toks).first().copied() else { return vec![] };
                let kind = if ctx.template { Kind::Template(TKind::Alias) } else { Kind::Type };
                let scope = n.child("type_id").and_then(|t| self.type_scope(t, toks));
                let target = self.target();
                self.add(target, name, Entry { kind, scope, param: false });
                vec![]
            }
            "namespace_def" => {
                let words = n.words(toks);
                let inline_all = words.first() == Some(&"inline");
                let mut parent = self.target();
                let mut out = Vec::new();
                match n.child("ns_path") {
                    None => {
                        let s = self.new_scope(parent, SK::Namespace, "");
                        self.scopes[parent as usize].using.push(s);
                        out.push(s);
                    }
                    Some(path) => {
                        let mut inline_next = inline_all && path.words(toks).iter().filter(|w| **w == "::").count() == 0;
                        for k in &path.kids {
                            let Kid::Tok(t) = k else { continue };
                            let w = toks[*t].text.as_str();
                            if w == "inline" {
                                inline_next = true;
                                continue;
                            }
                            if w == "::" {
                                continue;
                            }
                            let s = self.new_scope(parent, SK::Namespace, w);
                            self.add(parent, w, Entry { kind: Kind::Namespace, scope: Some(s), param: false });
                            if inline_next && !self.scopes[parent as usize].using.contains(&s) {
                                self.scopes[parent as usize].using.push(s);
                            }
                            inline_next = false;
                            out.push(s);
                            parent = s;
                        }
                    }
                }
                out
            }
            "namespace_alias" | "using_directive" => {
                let q = n.child("nested_name").map_or(Qual::None, |nn| self.resolve_nested(nn, toks));
                let Some(name) = direct_idents(n, toks).last().copied() else { return vec![] };
                let f = self.find(q, name);
                let target = self.target();
                if n.prod == "using_directive" {
                    if let Some(s) = f.scope {
                        self.scopes[target as usize].using.push(s);
                    }
                } else if let Some(alias) = direct_idents(n, toks).first().copied() {
                    self.add(target, alias, Entry { kind: Kind::Namespace, scope: f.scope, param: false });
                }
                vec![]
            }
            "using_decl" => {
                let target = self.target();
                let mut typename = false;
                for k in &n.kids {
                    match k {
                        Kid::Tok(t) if toks[*t].text == "typename" => typename = true,
                        Kid::Tok(t) if toks[*t].text == "," => typename = false,
                        Kid::Node(qn) if qn.prod == "qualified_name" => {
                            let q = qn.child("nested_name").map_or(Qual::None, |nn| self.resolve_nested(nn, toks));
                            let Some(name) = qn.child("unqualified_id").map(|u| direct_idents(u, toks)).and_then(|v| v.first().copied()) else { continue };
                            let f = if typename { Found { kind: Kind::Type, scope: None, param: false } } else { self.find(q, name) };
                            let kind = if f.kind == Kind::Unknown { Kind::Value } else { f.kind };
                            if self.scopes[target as usize].name != name {
                                self.add(target, name, Entry { kind, scope: f.scope, param: false });
                            }
                        }
                        _ => {}
                    }
                }
                vec![]
            }
            "concept_definition" => {
                if let Some(name) = direct_idents(n, toks).first().copied() {
                    let target = self.target();
                    self.add(target, name, Entry { kind: Kind::Template(TKind::Concept), scope: None, param: false });
                }
                vec![]
            }
            "sb_decl" | "capture" => {
                let target = self.target();
                for name in direct_idents(n, toks) {
                    self.add(target, name, Entry { kind: Kind::Value, scope: None, param: false });
                }
                vec![]
            }
            _ => vec![],
        }
    }

    fn enter(&mut self, e: Enter) {
        let s = match e {
            Enter::Scope(s) => s,
            Enter::Block | Enter::Template | Enter::Prototype => {
                let kind = match e {
                    Enter::Block => SK::Block,
                    Enter::Template => SK::Template,
                    _ => SK::Prototype,
                };
                let parent = self.innermost();
                self.new_scope(parent, kind, "")
            }
        };
        self.stack.push(s);
    }

    fn leave(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }

    fn stack(&self) -> Vec<u32> {
        self.stack.clone()
    }

    fn set_stack(&mut self, s: Vec<u32>) {
        self.stack = s;
        if self.stack.is_empty() {
            self.stack.push(0);
        }
    }

    fn depth(&self) -> usize {
        self.stack.len()
    }
}
