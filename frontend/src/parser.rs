// Parser state, entities, scopes, and name lookup.
//
// Name lookup is outside the spec (charter). The model here follows
// NAM-P1 to NAM-P4 in spec/foundations/names.md: scopes nest, a use
// searches outward and stops at the first scope that declares the name
// (NAM-2), class member bodies see the whole class (NAM-6), and labels
// have their own function-wide name space (NAM-7). Argument-dependent
// lookup (NAM-5) is not implemented: an unqualified call whose name is
// not found is treated as unknown instead of rejected.

use crate::ast::*;
use crate::diag::Diags;
use crate::lexer::{Lit, Token, TK};
use crate::types::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Storage {
    Auto,
    Static,
    Extern,
    ThreadLocal,
}

#[derive(Clone, Debug)]
pub struct VarInfo {
    pub ty: QTy,
    pub storage: Storage,
    pub constexpr: bool,
    pub constinit: bool,
    pub init: Option<Rc<Expr>>,
    pub param: bool,
    pub local: bool,
    /// Value depends on a template parameter.
    pub dependent: bool,
    pub defined: bool,
    pub class: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FnKind {
    Normal,
    Ctor,
    Dtor,
    Conversion,
    Operator,
}

#[derive(Clone, Debug)]
pub struct FuncInfo {
    pub fty: FnTy,
    pub params: Vec<DeclId>,
    pub defaults: usize,
    pub body: Option<Rc<Stmt>>,
    pub defined: bool,
    pub constexpr: bool,
    pub consteval: bool,
    pub deleted: bool,
    pub defaulted: bool,
    pub class: Option<usize>,
    pub kind: FnKind,
    pub is_static: bool,
    pub is_virtual: bool,
    pub template: bool,
    pub explicit: bool,
    /// The body contains something the front end did not understand.
    pub opaque: bool,
    pub is_main: bool,
    pub explicit_obj: bool,
    pub default_exprs: Vec<Option<Rc<Expr>>>,
    pub ret_auto: Option<(crate::decl::DeclSpec, crate::decl::Declarator)>,
}

#[derive(Clone, Debug)]
pub struct FieldInfo {
    pub ty: QTy,
    pub class: usize,
    pub index: usize,
    pub bits: Option<u32>,
    pub mutable: bool,
    pub init: Option<Rc<Expr>>,
    pub align: u64,
    pub nua: bool,
}

#[derive(Clone, Debug)]
pub enum TemplKind {
    Class(usize),
    Func,
    Alias,
    /// A variable template: the primary initializer and partial
    /// specializations, each as (parameter names, argument patterns,
    /// initializer).
    Var { params: Vec<String>, init: Option<Rc<Expr>>, specs: Vec<(Vec<String>, Vec<QTyPat>, Rc<Expr>)> },
    Concept,
}

#[derive(Clone, Debug)]
pub enum QTyPat {
    Param(usize),
    Ty(QTy),
}

#[derive(Clone, Debug)]
pub enum DK {
    Var(VarInfo),
    Func(FuncInfo),
    Type(QTy),
    Class(usize),
    Enum(usize),
    Enumerator(usize, i128),
    Namespace(usize),
    Field(FieldInfo),
    Template(TemplKind),
    /// A template parameter that names a type.
    TypeParam,
}

#[derive(Clone, Debug)]
pub struct Decl {
    pub name: String,
    pub k: DK,
    pub line: u32,
    pub scope: usize,
}

#[derive(Clone, Debug)]
pub struct BaseInfo {
    pub class: Option<usize>,
    pub virt: bool,
    pub access: u8,
}

#[derive(Clone, Debug)]
pub struct ClassInfo {
    pub name: String,
    pub key: u8, // 0 struct, 1 class, 2 union
    pub scope: usize,
    pub complete: bool,
    pub fields: Vec<DeclId>,
    pub bases: Vec<BaseInfo>,
    pub has_conv: bool,
    pub conv_targets: Vec<QTy>,
    pub ctors: Vec<DeclId>,
    pub dtor: Option<DeclId>,
    pub has_virtual: bool,
    /// A member's or base's layout is not modeled.
    pub template: bool,
    /// The class has members or bases the front end did not model.
    pub opaque: bool,
    pub line: u32,
    pub align: u64,
}

#[derive(Clone, Debug)]
pub struct EnumInfo {
    pub name: String,
    pub scoped: bool,
    pub fixed: Option<Ty>,
    pub underlying: Ty,
    pub complete: bool,
    pub enumerators: Vec<(String, i128)>,
    pub scope: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SK {
    Namespace,
    Class(usize),
    Block,
    Params,
    Template,
    Enum(usize),
}

#[derive(Clone, Debug)]
pub struct ScopeInfo {
    pub kind: SK,
    pub names: HashMap<String, Vec<DeclId>>,
    pub parent: Option<usize>,
    pub using_ns: Vec<usize>,
    /// Name of the namespace, for qualified display.
    pub name: String,
    /// A using-directive or using-declaration that the front end could
    /// not resolve makes failed lookups here uncertain.
    pub uncertain: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SwitchCtx {
    pub start_set: std::collections::HashSet<DeclId>,
    pub cond_ty: Option<QTy>,
    pub cases: HashMap<i128, u32>,
    pub has_default: bool,
}

#[derive(Clone, Debug)]
pub struct FnCtx {
    pub ret: Option<QTy>,
    /// `auto` return type to deduce.
    pub deduce: bool,
    pub deduced: Option<QTy>,
    pub ctor_dtor: bool,
    pub labels: HashMap<String, u32>,
    pub gotos: Vec<(String, u32)>,
    pub loops: u32,
    pub breakable: u32,
    pub switches: Vec<SwitchCtx>,
    pub func: Option<DeclId>,
    pub class: Option<usize>,
    pub is_static: bool,
    pub this_const: bool,
    pub this_volatile: bool,
    pub consteval_ctx: bool,
    pub is_lambda: bool,
    pub scope_base: usize,
    pub lambda_mutable: bool,
    pub lambda_copy_default: bool,
    pub lambda_refs: Vec<String>,
    pub ret_auto: Option<(crate::decl::DeclSpec, crate::decl::Declarator)>,
}

impl FnCtx {
    pub fn new(ret: Option<QTy>) -> FnCtx {
        FnCtx {
            ret,
            deduce: false,
            deduced: None,
            ctor_dtor: false,
            labels: HashMap::new(),
            gotos: Vec::new(),
            loops: 0,
            breakable: 0,
            switches: Vec::new(),
            func: None,
            class: None,
            is_static: false,
            this_const: false,
            this_volatile: false,
            consteval_ctx: false,
            is_lambda: false,
            scope_base: 0,
            lambda_mutable: false,
            lambda_copy_default: false,
            lambda_refs: Vec::new(),
            ret_auto: None,
        }
    }
}

#[derive(Debug)]
pub struct PErr {
    pub line: u32,
    pub msg: String,
}

pub type PR<T> = Result<T, PErr>;

pub struct Deferred {
    pub func: Option<DeclId>,
    pub field: Option<DeclId>,
    pub param: Option<DeclId>,
    pub start: usize,
    pub end: usize,
    pub stack: Vec<usize>,
    pub templ: u32,
    pub class: usize,
}

pub struct P {
    pub toks: Vec<Token>,
    pub pos: usize,
    pub d: Diags,
    pub decls: Vec<Decl>,
    pub classes: Vec<ClassInfo>,
    pub enums: Vec<EnumInfo>,
    pub scopes: Vec<ScopeInfo>,
    pub stack: Vec<usize>,
    pub fns: Vec<FnCtx>,
    pub templ: u32,
    pub class_depth: u32,
    pub deferred: Vec<Deferred>,
    pub global: usize,
    pub werror: bool,
    /// Names of constructs that were skipped, for the unsupported list.
    pub in_unevaluated: u32,
    pub ce_depth: u32,
    pub pending_template: Option<crate::decl2::PendingTemplate>,
    pub skipped_template_check: bool,
    pub auto_in_own_init: Option<DeclId>,
    /// Inside template argument lists, `>` ends the expression.
    pub no_gt: u32,
    /// Inside the braces of an enumeration being defined.
    pub string_lits: usize,
    pub goto_sets: Vec<(String, u32, std::collections::HashSet<DeclId>, usize)>,
    pub label_sets: Vec<(String, u32, std::collections::HashSet<DeclId>, usize)>,
    pub jump_vars: std::collections::HashSet<DeclId>,
    pub conflicts: HashMap<usize, (usize, &'static str)>,
    pub discarded: u32,
    pub last_align: u64,
    pub saw_alignas: bool,
    pub align_unknown: bool,
    pub saw_nua: bool,
    pub dmi_depth: u32,
    pub cx_branches: Vec<usize>,
    pub cx_counter: usize,
}

impl P {
    pub fn new(toks: Vec<Token>, d: Diags) -> P {
        let mut p = P {
            toks,
            pos: 0,
            d,
            decls: Vec::new(),
            classes: Vec::new(),
            enums: Vec::new(),
            scopes: Vec::new(),
            stack: Vec::new(),
            fns: Vec::new(),
            templ: 0,
            class_depth: 0,
            deferred: Vec::new(),
            global: 0,
            werror: false,
            in_unevaluated: 0,
            ce_depth: 0,
            pending_template: None,
            skipped_template_check: false,
            auto_in_own_init: None,
            no_gt: 0,
            string_lits: 0,
            goto_sets: Vec::new(),
            label_sets: Vec::new(),
            jump_vars: Default::default(),
            conflicts: HashMap::new(),
            discarded: 0,
            last_align: 0,
            saw_alignas: false,
            align_unknown: false,
            saw_nua: false,
            dmi_depth: 0,
            cx_branches: Vec::new(),
            cx_counter: 0,
        };
        let g = p.new_scope(SK::Namespace, None, "");
        p.global = g;
        p.stack.push(g);
        p
    }

    // ---- tokens ----
    pub fn tok(&self) -> &Token {
        &self.toks[self.pos.min(self.toks.len() - 1)]
    }
    pub fn peek(&self, k: usize) -> &Token {
        &self.toks[(self.pos + k).min(self.toks.len() - 1)]
    }
    pub fn line(&self) -> u32 {
        self.tok().line
    }
    pub fn at(&self, s: &str) -> bool {
        let t = self.tok();
        (t.kind == TK::Punct || t.kind == TK::Ident) && t.text == s
    }
    pub fn at_k(&self, k: usize, s: &str) -> bool {
        let t = self.peek(k);
        (t.kind == TK::Punct || t.kind == TK::Ident) && t.text == s
    }
    pub fn eat(&mut self, s: &str) -> bool {
        if self.at(s) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    pub fn expect(&mut self, s: &str) -> PR<()> {
        if self.eat(s) {
            return Ok(());
        }
        // A `>>` closes two template argument lists.
        if s == ">" && self.at(">>") {
            self.toks[self.pos].text = ">".to_string();
            let mut t = self.toks[self.pos].clone();
            t.text = ">".to_string();
            self.toks.insert(self.pos + 1, t);
            self.pos += 1;
            return Ok(());
        }
        if s == ">" && self.at(">=") {
            self.toks[self.pos].text = "=".to_string();
            return Ok(());
        }
        Err(self.err(format!("expected '{}' but found '{}'", s, self.tok_desc())))
    }
    pub fn tok_desc(&self) -> String {
        let t = self.tok();
        match t.kind {
            TK::Eof => "end of file".into(),
            TK::Int | TK::Float => "number".into(),
            TK::Char => "character literal".into(),
            TK::Str => "string literal".into(),
            _ => t.text.clone(),
        }
    }
    pub fn err(&self, msg: impl Into<String>) -> PErr {
        PErr { line: self.line(), msg: msg.into() }
    }
    pub fn is_ident(&self) -> bool {
        self.tok().kind == TK::Ident && !is_keyword(&self.tok().text)
    }
    pub fn ident(&mut self) -> PR<String> {
        if self.is_ident() {
            let s = self.tok().text.clone();
            self.pos += 1;
            Ok(s)
        } else {
            Err(self.err(format!("expected identifier, found '{}'", self.tok_desc())))
        }
    }
    /// Skip a balanced group starting at an opening bracket.
    pub fn skip_balanced(&mut self) {
        let open = self.tok().text.clone();
        let close = match open.as_str() {
            "(" => ")",
            "[" => "]",
            "{" => "}",
            "<" => ">",
            _ => {
                self.pos += 1;
                return;
            }
        };
        let mut depth = 0i32;
        loop {
            let t = self.tok();
            if t.kind == TK::Eof {
                return;
            }
            if t.kind == TK::Punct {
                if t.text == open {
                    depth += 1;
                } else if t.text == close || (close == ">" && t.text == ">>") {
                    depth -= if t.text == ">>" { 2 } else { 1 };
                    if depth <= 0 {
                        self.pos += 1;
                        return;
                    }
                } else if close == ">" && (t.text == "(" || t.text == "[" || t.text == "{") {
                    self.skip_balanced();
                    continue;
                } else if close == ">" && (t.text == ";" || t.text == "{" || t.text == "}") {
                    return;
                }
            }
            self.pos += 1;
        }
    }
    /// Skip to the end of the current declaration or statement.
    pub fn recover(&mut self) {
        loop {
            let t = self.tok();
            if t.kind == TK::Eof {
                return;
            }
            if t.kind == TK::Punct {
                match t.text.as_str() {
                    ";" => {
                        self.pos += 1;
                        return;
                    }
                    "{" => {
                        self.skip_balanced();
                        if self.at(";") {
                            self.pos += 1;
                        }
                        return;
                    }
                    "(" | "[" => {
                        self.skip_balanced();
                        continue;
                    }
                    "}" => return,
                    _ => {}
                }
            }
            self.pos += 1;
        }
    }

    pub fn error(&mut self, line: u32, rule: &'static str, msg: impl Into<String>) {
        self.d.error(line, Some(rule), msg);
    }
    pub fn error_norule(&mut self, line: u32, msg: impl Into<String>) {
        self.d.error(line, None, msg);
    }
    pub fn unsupported(&mut self, line: u32, msg: impl Into<String>) {
        self.d.unsupported(line, msg);
    }
    pub fn warn(&mut self, line: u32, rule: &'static str, msg: impl Into<String>) {
        if self.werror {
            self.d.error(line, Some(rule), format!("{} (warning treated as error)", msg.into()));
        } else {
            self.d.warning(line, rule, msg);
        }
    }

    // ---- scopes ----
    pub fn new_scope(&mut self, kind: SK, parent: Option<usize>, name: &str) -> usize {
        self.scopes.push(ScopeInfo { kind, names: HashMap::new(), parent, using_ns: Vec::new(), name: name.to_string(), uncertain: false });
        self.scopes.len() - 1
    }
    pub fn cur_scope(&self) -> usize {
        *self.stack.last().unwrap()
    }
    pub fn push_scope(&mut self, kind: SK) -> usize {
        let parent = self.cur_scope();
        let s = self.new_scope(kind, Some(parent), "");
        self.stack.push(s);
        s
    }
    pub fn pop_scope(&mut self) {
        self.stack.pop();
    }
    pub fn add_decl(&mut self, name: &str, k: DK, line: u32) -> DeclId {
        let scope = self.cur_scope();
        self.add_decl_in(scope, name, k, line)
    }
    pub fn add_decl_in(&mut self, scope: usize, name: &str, k: DK, line: u32) -> DeclId {
        self.decls.push(Decl { name: name.to_string(), k, line, scope });
        let id = self.decls.len() - 1;
        if !name.is_empty() {
            self.scopes[scope].names.entry(name.to_string()).or_default().push(id);
        }
        id
    }
    /// The innermost scope that is not a template-parameter scope or a
    /// function-parameter scope used for prototypes.
    pub fn decl_scope(&self) -> usize {
        for &s in self.stack.iter().rev() {
            if !matches!(self.scopes[s].kind, SK::Template | SK::Enum(_)) {
                return s;
            }
        }
        self.global
    }
    pub fn in_block(&self) -> bool {
        matches!(self.scopes[self.decl_scope()].kind, SK::Block | SK::Params)
    }
    pub fn in_template(&self) -> bool {
        self.templ > 0
    }

    /// Unqualified lookup (NAM-2): search outward, stop at the first
    /// scope that declares the name.
    pub fn lookup(&self, name: &str) -> Option<Vec<DeclId>> {
        for &s in self.stack.iter().rev() {
            if let Some(r) = self.lookup_in_scope(s, name, true) {
                return Some(r);
            }
        }
        None
    }
    /// Is a failed lookup certain? It is not when a scope on the stack
    /// has uncertain contents or the code is in a template.
    pub fn lookup_uncertain(&self) -> bool {
        if self.in_template() {
            return true;
        }
        for &s in self.stack.iter() {
            if self.scopes[s].uncertain {
                return true;
            }
            if let SK::Class(c) = self.scopes[s].kind {
                if self.class_uncertain(c) {
                    return true;
                }
            }
        }
        false
    }
    pub fn class_uncertain(&self, c: usize) -> bool {
        let ci = &self.classes[c];
        ci.template || ci.opaque || ci.bases.iter().any(|b| b.class.map_or(true, |bc| self.class_uncertain(bc)))
    }
    pub fn lookup_in_scope(&self, s: usize, name: &str, with_using: bool) -> Option<Vec<DeclId>> {
        if let Some(v) = self.scopes[s].names.get(name) {
            if !v.is_empty() {
                return Some(v.clone());
            }
        }
        if let SK::Class(c) = self.scopes[s].kind {
            if let Some(r) = self.lookup_in_bases(c, name, 0) {
                return Some(r);
            }
        }
        if with_using {
            let mut found = Vec::new();
            let mut seen = vec![s];
            self.lookup_using(s, name, &mut found, &mut seen);
            if !found.is_empty() {
                return Some(found);
            }
        }
        None
    }
    fn lookup_using(&self, s: usize, name: &str, found: &mut Vec<DeclId>, seen: &mut Vec<usize>) {
        for &n in &self.scopes[s].using_ns {
            if seen.contains(&n) {
                continue;
            }
            seen.push(n);
            if let Some(v) = self.scopes[n].names.get(name) {
                for &d in v {
                    if !found.contains(&d) {
                        found.push(d);
                    }
                }
            }
            self.lookup_using(n, name, found, seen);
        }
    }
    pub fn lookup_in_bases(&self, c: usize, name: &str, depth: u32) -> Option<Vec<DeclId>> {
        if depth > 20 {
            return None;
        }
        let mut res: Option<Vec<DeclId>> = None;
        for b in &self.classes[c].bases {
            if let Some(bc) = b.class {
                let sc = self.classes[bc].scope;
                let r = self.scopes[sc].names.get(name).cloned().filter(|v| !v.is_empty()).or_else(|| self.lookup_in_bases(bc, name, depth + 1));
                if let Some(r) = r {
                    match &mut res {
                        None => res = Some(r),
                        Some(v) => {
                            for d in r {
                                if !v.contains(&d) {
                                    v.push(d);
                                }
                            }
                        }
                    }
                }
            }
        }
        res
    }
    /// Lookup of a member of a class, including bases.
    pub fn lookup_member(&self, c: usize, name: &str) -> Option<Vec<DeclId>> {
        let sc = self.classes[c].scope;
        self.lookup_in_scope(sc, name, false)
    }

    // ---- entity helpers ----
    pub fn var(&self, id: DeclId) -> Option<&VarInfo> {
        match &self.decls[id].k {
            DK::Var(v) => Some(v),
            _ => None,
        }
    }
    pub fn func(&self, id: DeclId) -> Option<&FuncInfo> {
        match &self.decls[id].k {
            DK::Func(f) => Some(f),
            _ => None,
        }
    }
    pub fn func_mut(&mut self, id: DeclId) -> Option<&mut FuncInfo> {
        match &mut self.decls[id].k {
            DK::Func(f) => Some(f),
            _ => None,
        }
    }
    pub fn enum_underlying(&self, e: usize) -> Ty {
        self.enums[e].underlying.clone()
    }
    pub fn is_scoped_enum(&self, t: &Ty) -> bool {
        matches!(t, Ty::Enum(e) if self.enums[*e].scoped)
    }
    pub fn is_unscoped_enum(&self, t: &Ty) -> bool {
        matches!(t, Ty::Enum(e) if !self.enums[*e].scoped)
    }
    pub fn class_size(&self, c: usize) -> Option<(u64, u64)> {
        let ci = &self.classes[c];
        if !ci.complete || ci.opaque || ci.template {
            return None;
        }
        // Layout: bases and fields in order, with natural alignment.
        // Bit-fields and virtual functions make the class opaque here.
        let mut size = 0u64;
        let mut align = 1u64;
        if ci.has_virtual || ci.bases.iter().any(|b| b.virt) {
            return None;
        }
        let mut items: Vec<(QTy, u64)> = Vec::new();
        for &f in &ci.fields {
            if let DK::Field(fi) = &self.decls[f].k {
                if fi.bits.is_some() {
                    return None;
                }
                if fi.nua {
                    if let Ty::Class(fc) = fi.ty.t {
                        if self.classes[fc].fields.is_empty() && self.classes[fc].bases.is_empty() {
                            continue; // ATTR: an empty member takes no space
                        }
                    }
                }
                items.push((fi.ty.clone(), fi.align));
            }
        }
        if !ci.bases.is_empty() {
            return None;
        }
        align = align.max(ci.align);
        for (t, fa) in &items {
            let s = self.sizeof_ty(&t.t)?;
            let a = self.alignof_ty(&t.t)?.max(*fa);
            if ci.key == 2 {
                size = size.max(s);
            } else {
                size = size.div_ceil(a) * a + s;
            }
            align = align.max(a);
        }
        if size == 0 {
            size = 1;
        }
        size = size.div_ceil(align) * align;
        Some((size, align))
    }
    pub fn sizeof_ty(&self, t: &Ty) -> Option<u64> {
        size_of(t, &|c| self.class_size(c), &|e| self.enum_underlying(e))
    }
    pub fn alignof_ty(&self, t: &Ty) -> Option<u64> {
        align_of(t, &|c| self.class_size(c), &|e| self.enum_underlying(e))
    }
    pub fn is_complete(&self, t: &Ty) -> bool {
        match t {
            Ty::Void => false,
            Ty::Class(c) => self.classes[*c].complete,
            Ty::Enum(e) => self.enums[*e].complete || self.enums[*e].fixed.is_some() || self.enums[*e].scoped,
            Ty::Array(i, n) => n.is_some() && self.is_complete(&i.t),
            _ => true,
        }
    }

    pub fn lit_token(&self) -> Option<Lit> {
        self.tok().lit.clone()
    }
}

pub fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "alignas"
            | "alignof"
            | "asm"
            | "auto"
            | "bool"
            | "break"
            | "case"
            | "catch"
            | "char"
            | "char8_t"
            | "char16_t"
            | "char32_t"
            | "class"
            | "concept"
            | "const"
            | "consteval"
            | "constexpr"
            | "constinit"
            | "const_cast"
            | "continue"
            | "co_await"
            | "co_return"
            | "co_yield"
            | "decltype"
            | "default"
            | "delete"
            | "do"
            | "double"
            | "dynamic_cast"
            | "else"
            | "enum"
            | "explicit"
            | "export"
            | "extern"
            | "false"
            | "float"
            | "for"
            | "friend"
            | "goto"
            | "if"
            | "inline"
            | "int"
            | "long"
            | "mutable"
            | "namespace"
            | "new"
            | "noexcept"
            | "nullptr"
            | "operator"
            | "private"
            | "protected"
            | "public"
            | "register"
            | "reinterpret_cast"
            | "requires"
            | "return"
            | "short"
            | "signed"
            | "sizeof"
            | "static"
            | "static_assert"
            | "static_cast"
            | "struct"
            | "switch"
            | "template"
            | "this"
            | "thread_local"
            | "throw"
            | "true"
            | "try"
            | "typedef"
            | "typeid"
            | "typename"
            | "union"
            | "unsigned"
            | "using"
            | "virtual"
            | "void"
            | "volatile"
            | "wchar_t"
            | "while"
    )
}
