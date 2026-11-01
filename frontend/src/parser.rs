// Parser state, entities, scopes, and name lookup.
//
// Name lookup follows specification/foundations/names.md: scopes nest, a use
// searches outward and stops at the first scope that declares the name
// (NAM-2), class member bodies see the whole class (NAM-6), and labels
// have their own function-wide name space (NAM-7). Using directives act
// at the innermost namespace around the directive and the named
// namespace (NAM-23), qualified lookup follows NAM-13, and member lookup
// tracks base subobjects (NAM-17, DER-4). Argument-dependent lookup
// (NAM-27 to NAM-32) is in expr2.rs.

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
    /// The mem-initializers of a constructor.
    pub mem_inits: Vec<MemInit>,
    /// The attributes of the first declaration (ATTR-2).
    pub first_attrs: Vec<String>,
    /// The first declaration has `= default` (CTOR-23, DTOR-9, CPY-27).
    pub first_defaulted: bool,
    /// A trailing requires-clause (OVL-24, OVL-30).
    pub constrained: bool,
    /// The handlers of a function try block (TRY-9, TRY-10).
    pub fn_try: Option<Rc<Vec<Handler>>>,
    /// A statement of the body failed to parse.
    pub parse_failed: bool,
    /// The declaration says `noexcept(false)` (EXS-4, UNW-5).
    pub noexcept_false: bool,
}

/// One attribute: its name without namespace, whether it had a
/// namespace, its argument expression for `assume`, and its line.
#[derive(Clone, Debug)]
pub struct Attr {
    pub name: String,
    pub ns: bool,
    pub assume: Option<Expr>,
    pub line: u32,
}

/// One mem-initializer: a member (`field`), or a base or the class
/// itself for a delegating constructor (`base`). `init` is None when the
/// front end could not form the initialization.
#[derive(Clone, Debug)]
pub struct MemInit {
    pub field: Option<DeclId>,
    pub base: Option<usize>,
    pub init: Option<Rc<Expr>>,
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
    /// The anonymous unions that are members of the class (UNI-3).
    pub anon_unions: Vec<usize>,
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
    /// The -W flags of the run, for warnings that only a flag enables.
    pub wflags: Vec<String>,
    /// ATTR-6: the variables that an expression names, and the local
    /// variables that could be unused.
    pub used_vars: std::collections::HashSet<DeclId>,
    pub unused_cands: Vec<(DeclId, u32)>,
    /// ATTR-6: the number of uses of each variable, and of uses as the
    /// left operand of a plain assignment, which do not read it.
    pub use_count: HashMap<DeclId, u32>,
    pub assign_count: HashMap<DeclId, u32>,
    /// ATTR-6: the names of one structured binding declaration, under
    /// its first name.
    pub binding_groups: HashMap<DeclId, Vec<DeclId>>,
    /// The access of class members (0 public, 1 protected, 2 private),
    /// the access in effect in each class body being parsed, and the
    /// classes that declare a friend.
    pub member_access: HashMap<DeclId, u8>,
    pub access_stack: Vec<u8>,
    pub friend_classes: std::collections::HashSet<usize>,
    /// Names of constructs that were skipped, for the unsupported list.
    pub in_unevaluated: u32,
    /// ODR-9: functions with internal linkage, and the first evaluated
    /// call of each.
    pub internal_fns: std::collections::HashSet<DeclId>,
    pub internal_uses: Vec<(DeclId, u32)>,
    /// ODR-1: static data members with a definition outside the class.
    pub static_member_defs: std::collections::HashSet<DeclId>,
    /// The classes that stand for `std::initializer_list<E>` (ICS-16),
    /// with their element types.
    pub init_list_classes: Vec<(QTy, usize)>,
    /// The parenthesized arguments of the `new` initializer being parsed.
    pub new_paren_args: Vec<Expr>,
    /// The braced initializer of the `new` expression being parsed.
    pub new_braced: Option<Expr>,
    /// The last ICS selection formed every sequence and found no viable
    /// function.
    pub ics_no_viable: bool,
    /// The operands after a built-in candidate won (OVL-11).
    pub op_converted: Option<Vec<Expr>>,
    /// The call being resolved goes through `&f`, so it has no implied
    /// object argument (OVL-21).
    pub call_no_object: bool,
    /// The operator being resolved compares a `<=>` result with 0 inside
    /// a rewritten candidate, so it has no rewritten candidates (OVL-43).
    pub inner_rewrite: bool,
    /// Depth of `extern "C"` linkage specifications.
    pub lang_c: u32,
    /// Functions declared with C language linkage.
    pub c_fns: std::collections::HashSet<DeclId>,
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
    /// Attributes parsed since the start of the current statement or
    /// declaration (specification/declarations/attributes.md).
    pub pending_attrs: Vec<Attr>,
    /// The standard attributes of each entity, over all its declarations
    /// (ATTR-4, ATTR-5, ATTR-14).
    pub entity_attrs: HashMap<DeclId, Vec<String>>,
    /// Pure virtual functions (VIRT-8).
    pub pure_fns: std::collections::HashSet<DeclId>,
    /// The standard attributes of each class.
    pub class_attrs: HashMap<usize, Vec<String>>,
    /// The leading attributes of the declaration being parsed.
    pub decl_attrs: Vec<Attr>,
    /// Friend functions first declared in a class body, with the classes
    /// that declare them. Ordinary and qualified lookup do not find them
    /// until a declaration at namespace scope (ACC-7, NAM-14).
    pub hidden_friends: HashMap<DeclId, Vec<usize>>,
    /// The friend functions that each class declares (NAM-29).
    pub class_friends: HashMap<usize, Vec<DeclId>>,
    /// Inline namespaces (NAM-13).
    pub inline_ns: std::collections::HashSet<usize>,
    /// Namespace-scope variables with internal linkage (LNK rules).
    pub var_internal: std::collections::HashSet<DeclId>,
    /// Inside a base clause, where `C::C` names the class (NAM-33).
    pub in_base_clause: u32,
    /// The token position of the `(` of a call whose callee is an
    /// unqualified name that allows argument-dependent lookup (NAM-31).
    pub adl_pos: Option<usize>,
    /// Set by build_call for the call being built (NAM-31, NAM-32).
    pub call_adl: bool,
    /// The captures of each closure's `operator()`: the variable, the
    /// member of the closure that holds it, and whether the capture is by
    /// reference (LAM rules).
    pub closure_caps: HashMap<DeclId, Vec<(DeclId, usize, bool)>>,
    /// The member of a closure that holds the captured `this`, and whether
    /// it is a copy of `*this`.
    pub closure_this: HashMap<DeclId, (usize, bool)>,
    /// The conversion function of a closure with no captures, and its call
    /// operator (LAM-10).
    pub closure_conv: HashMap<DeclId, DeclId>,
    /// Functions that return a lambda with reference captures (LAM-19).
    pub ref_lambda_fns: std::collections::HashSet<DeclId>,
    /// ENUM-10: the type of an enumerator inside its enumerator list.
    pub enumerator_ty: HashMap<DeclId, Ty>,
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
            wflags: Vec::new(),
            used_vars: Default::default(),
            unused_cands: Vec::new(),
            use_count: HashMap::new(),
            assign_count: HashMap::new(),
            binding_groups: HashMap::new(),
            member_access: HashMap::new(),
            access_stack: Vec::new(),
            friend_classes: Default::default(),
            in_unevaluated: 0,
            internal_fns: Default::default(),
            internal_uses: Vec::new(),
            static_member_defs: Default::default(),
            init_list_classes: Vec::new(),
            new_paren_args: Vec::new(),
            new_braced: None,
            ics_no_viable: false,
            op_converted: None,
            call_no_object: false,
            inner_rewrite: false,
            lang_c: 0,
            c_fns: Default::default(),
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
            enumerator_ty: HashMap::new(),
            pending_attrs: Vec::new(),
            entity_attrs: HashMap::new(),
            class_attrs: HashMap::new(),
            pure_fns: std::collections::HashSet::new(),
            decl_attrs: Vec::new(),
            hidden_friends: HashMap::new(),
            class_friends: HashMap::new(),
            inline_ns: Default::default(),
            var_internal: Default::default(),
            in_base_clause: 0,
            adl_pos: None,
            call_adl: false,
            ref_lambda_fns: Default::default(),
            closure_caps: Default::default(),
            closure_this: Default::default(),
            closure_conv: Default::default(),
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
    /// An error under `rule` whose cause is a broken `cause` rule, such as
    /// a constant that needs a constant expression (CX-1) and an
    /// evaluation that reached an undefined operation.
    pub fn error_because(&mut self, line: u32, rule: &'static str, cause: &'static str, msg: impl Into<String>) {
        self.d.error(line, Some(rule), msg);
        if self.d.muted == 0 {
            if let Some(d) = self.d.list.last_mut() {
                if rule != cause {
                    d.cause = Some(cause);
                }
            }
        }
    }
    /// Record the current declaration's attributes on an entity.
    pub fn record_attrs(&mut self, id: DeclId) {
        let names: Vec<String> = self.decl_attrs.iter().map(|a| a.name.clone()).collect();
        if !names.is_empty() {
            let e = self.entity_attrs.entry(id).or_default();
            for n in names {
                if !e.contains(&n) {
                    e.push(n);
                }
            }
        }
    }
    /// VIRT-8: the pure virtual functions of a class without a final
    /// overrider, by name and parameter types. None when the front end
    /// does not model the class.
    fn pure_left(&self, c: usize, depth: u32) -> Option<Vec<(String, Vec<QTy>)>> {
        let ci = &self.classes[c];
        if ci.template || ci.opaque || !ci.complete || depth > 32 {
            return None;
        }
        let mut own: Vec<(String, Vec<QTy>, bool)> = Vec::new();
        for (n, ids) in &self.scopes[ci.scope].names {
            for &i in ids {
                if let DK::Func(f) = &self.decls[i].k {
                    if f.class == Some(c) {
                        own.push((n.clone(), f.fty.params.clone(), self.pure_fns.contains(&i)));
                    }
                }
            }
        }
        let mut out = Vec::new();
        for b in &ci.bases {
            let bc = b.class?;
            for (n, ps) in self.pure_left(bc, depth + 1)? {
                if n.starts_with('~') {
                    continue;
                }
                if !own.iter().any(|(on, ops, _)| *on == n && *ops == ps) {
                    out.push((n, ps));
                }
            }
        }
        for (n, ps, pure) in own {
            if pure {
                out.push((n, ps));
            }
        }
        Some(out)
    }
    pub fn is_abstract(&self, t: &Ty) -> bool {
        match t {
            Ty::Class(c) => self.pure_left(*c, 0).is_some_and(|v| !v.is_empty()),
            Ty::Array(el, _) => self.is_abstract(&el.t),
            _ => false,
        }
    }
    /// CTOR-3: whether the implicit or defaulted default constructor of a
    /// class is deleted. None when the front end cannot tell, or when the
    /// class has a user-provided default constructor.
    pub fn default_ctor_deleted(&self, c: usize, depth: u32) -> Option<bool> {
        let ci = &self.classes[c];
        if ci.template || ci.opaque || !ci.complete || depth > 16 {
            return None;
        }
        if !ci.ctors.is_empty() {
            let d = self.default_ctor(c)?;
            let f = self.func(d)?;
            if f.deleted {
                return Some(true);
            }
            if !f.defaulted {
                return Some(false);
            }
        }
        if self.default_parts_inaccessible(c) {
            return Some(true);
        }
        // a base or member that cannot be default-constructed or destroyed
        let unusable = |t: &Ty| -> Option<bool> {
            let t = match t {
                Ty::Array(e, _) => &e.t,
                t => t,
            };
            let Ty::Class(m) = t else { return Some(false) };
            let mi = &self.classes[*m];
            if let Some(dt) = mi.dtor {
                if self.func(dt)?.deleted {
                    return Some(true);
                }
            }
            if !mi.ctors.is_empty() && self.default_ctor(*m).is_none() {
                let all_known = mi.ctors.iter().all(|&i| self.func(i).is_some_and(|f| !f.template));
                return if all_known { Some(true) } else { None };
            }
            self.default_ctor_deleted(*m, depth + 1)
        };
        for b in &ci.bases {
            if unusable(&Ty::Class(b.class?))? {
                return Some(true);
            }
        }
        for &fd in &ci.fields {
            let DK::Field(fi) = &self.decls[fd].k else { continue };
            if ci.key == 2 {
                continue;
            }
            if fi.init.is_none() {
                if fi.ty.t.is_ref() {
                    return Some(true);
                }
                if fi.ty.c {
                    match &fi.ty.t {
                        Ty::Class(m) => {
                            let user_default = self.default_ctor(*m).and_then(|d| self.func(d)).is_some_and(|f| !f.defaulted);
                            let leaves_unset = self.classes[*m].fields.iter().any(|&g| matches!(&self.decls[g].k, DK::Field(x) if x.init.is_none()));
                            if !user_default && leaves_unset {
                                return Some(true);
                            }
                        }
                        Ty::Array(..) => {}
                        _ => return Some(true),
                    }
                }
            }
            if fi.init.is_none() {
                if unusable(&fi.ty.t)? {
                    return Some(true);
                }
            } else if let Ty::Class(m) = &fi.ty.t {
                if let Some(dt) = self.classes[*m].dtor {
                    if self.func(dt)?.deleted {
                        return Some(true);
                    }
                }
            }
        }
        Some(false)
    }

    /// Whether a class is nested in a class with no name (CLS-9).
    pub fn enclosing_unnamed(&self, c: usize) -> bool {
        let mut s = self.scopes[self.classes[c].scope].parent;
        while let Some(x) = s {
            if let SK::Class(o) = self.scopes[x].kind {
                if self.classes[o].name.is_empty() {
                    return true;
                }
            }
            s = self.scopes[x].parent;
        }
        false
    }
    pub fn has_attr(&self, id: DeclId, name: &str) -> bool {
        self.entity_attrs.get(&id).is_some_and(|v| v.iter().any(|n| n == name))
    }
    pub fn error_norule(&mut self, line: u32, msg: impl Into<String>) {
        self.d.error(line, None, msg);
    }
    pub fn unsupported(&mut self, line: u32, msg: impl Into<String>) {
        self.d.unsupported(line, msg);
    }
    /// Whether a member is accessible where the parser is now. A class
    /// with a friend counts as accessible, since the front end does not
    /// track friends one by one. `base` is true for a member of a base
    /// class used by a derived class (protected is then accessible).
    pub fn accessible(&self, id: DeclId, base: bool) -> bool {
        let a = self.member_access.get(&id).copied().unwrap_or(0);
        if a == 0 || (a == 1 && base) {
            return true;
        }
        let owner = match &self.decls[id].k {
            DK::Func(f) => f.class,
            DK::Field(f) => Some(f.class),
            _ => match self.scopes[self.decls[id].scope].kind {
                SK::Class(c) => Some(c),
                _ => None,
            },
        };
        let Some(owner) = owner else { return true };
        if self.friend_classes.contains(&owner) || self.class_uncertain(owner) {
            return true;
        }
        let mut here: Vec<usize> = self.fns.iter().filter_map(|f| f.class).collect();
        for &s in &self.stack {
            if let SK::Class(c) = self.scopes[s].kind {
                here.push(c);
            }
        }
        here.iter().any(|&c| c == owner || (a == 1 && self.is_base_of_p(owner, c)))
    }
    /// Whether `base` is a public base of `derived` along each step, or
    /// the parser is inside `derived` or a class that declares a friend.
    pub fn base_accessible(&self, derived: usize, base: usize) -> bool {
        if self.friend_classes.contains(&derived) || self.class_uncertain(derived) {
            return true;
        }
        let inside = self.fns.iter().filter_map(|f| f.class).chain(self.stack.iter().filter_map(|&s| match self.scopes[s].kind {
            SK::Class(c) => Some(c),
            _ => None,
        }));
        let inside: Vec<usize> = inside.collect();
        if inside.contains(&derived) {
            return true;
        }
        self.classes[derived].bases.iter().any(|b| match b.class {
            Some(bc) if bc == base => b.access == 0,
            Some(bc) => b.access == 0 && self.is_base_of_p(base, bc) && self.base_accessible(bc, base),
            None => true,
        })
    }
    pub fn is_base_of_p(&self, base: usize, derived: usize) -> bool {
        self.classes[derived].bases.iter().any(|b| b.class.is_some_and(|bc| bc == base || self.is_base_of_p(base, bc)))
    }
    /// Whether one of the -W flags is on.
    pub fn wflag(&self, names: &[&str]) -> bool {
        self.wflags.iter().any(|f| names.iter().any(|n| f == n))
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
        let id = self.add_decl_in0(scope, name, k, line);
        if let (SK::Class(_), Some(&a)) = (&self.scopes[scope].kind, self.access_stack.last()) {
            self.member_access.insert(id, a);
        }
        id
    }
    fn add_decl_in0(&mut self, scope: usize, name: &str, k: DK, line: u32) -> DeclId {
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
    /// scope that declares the name. The members of a namespace that a
    /// using directive names act as members of the innermost namespace
    /// that encloses both the directive and that namespace (NAM-23).
    pub fn lookup(&self, name: &str) -> Option<Vec<DeclId>> {
        self.lookup_full(name).0
    }
    /// Unqualified lookup, and whether the result is ambiguous (NAM-24,
    /// NAM-26, DER-4).
    pub fn lookup_full(&self, name: &str) -> (Option<Vec<DeclId>>, bool) {
        let noms = self.nominations();
        for &s in self.stack.iter().rev() {
            let (r, amb) = self.lookup_level(s, name, &noms);
            if let Some(r) = r {
                return (Some(r), amb);
            }
        }
        (None, false)
    }
    /// The declarations of a name at one scope of unqualified lookup.
    pub fn lookup_level(&self, s: usize, name: &str, noms: &[(usize, usize)]) -> (Option<Vec<DeclId>>, bool) {
        let mut sources: Vec<Vec<DeclId>> = Vec::new();
        let own = self.own_names(s, name);
        let mut amb = false;
        if !own.is_empty() {
            sources.push(own);
        } else if let SK::Class(c) = self.scopes[s].kind {
            if let Some((r, a)) = self.member_lookup(c, name) {
                amb = a;
                sources.push(r);
            }
        }
        if self.scopes[s].kind == SK::Namespace {
            for &(t, n) in noms {
                if t == s {
                    let v = self.own_names(n, name);
                    if !v.is_empty() {
                        sources.push(v);
                    }
                }
            }
        }
        if sources.is_empty() {
            return (None, false);
        }
        amb |= self.sources_ambiguous(&sources);
        (Some(merge_sources(sources)), amb)
    }
    /// The names a scope declares itself, without hidden friends.
    pub fn own_names(&self, s: usize, name: &str) -> Vec<DeclId> {
        match self.scopes[s].names.get(name) {
            Some(v) => v.iter().copied().filter(|d| !self.hidden_friends.contains_key(d)).collect(),
            None => Vec::new(),
        }
    }
    /// The using directives in effect: (namespace where the members act,
    /// nominated namespace). A directive is in effect in the scopes on
    /// the stack, and the directives of a nominated namespace follow.
    pub fn nominations(&self) -> Vec<(usize, usize)> {
        let mut out: Vec<(usize, usize)> = Vec::new();
        for &s in &self.stack {
            let mut seen = vec![s];
            let mut todo: Vec<usize> = self.scopes[s].using_ns.clone();
            while let Some(n) = todo.pop() {
                if seen.contains(&n) {
                    continue;
                }
                seen.push(n);
                let mut t = self.common_ns(s, n);
                if !self.stack.contains(&t) {
                    t = self.global;
                }
                if !out.contains(&(t, n)) {
                    out.push((t, n));
                }
                todo.extend(self.scopes[n].using_ns.iter().copied());
            }
        }
        out
    }
    /// The innermost namespace that encloses both scopes.
    fn common_ns(&self, a: usize, b: usize) -> usize {
        let chain = |mut x: Option<usize>| {
            let mut v = Vec::new();
            while let Some(s) = x {
                if self.scopes[s].kind == SK::Namespace {
                    v.push(s);
                }
                x = self.scopes[s].parent;
            }
            v
        };
        let ca = chain(Some(a));
        let cb = chain(Some(b));
        ca.into_iter().find(|s| cb.contains(s)).unwrap_or(self.global)
    }
    /// Whether declarations from several places name different entities
    /// that are not all functions (NAM-14, NAM-24, NAM-26).
    pub fn sources_ambiguous(&self, sources: &[Vec<DeclId>]) -> bool {
        if sources.len() < 2 {
            return false;
        }
        let mut keys: Vec<(u8, usize)> = Vec::new();
        let mut all_funcs = true;
        for src in sources {
            let has_value = src.iter().any(|&d| !self.is_tag(d));
            for &d in src {
                if has_value && self.is_tag(d) {
                    continue;
                }
                if !matches!(self.decls[d].k, DK::Func(_) | DK::Template(TemplKind::Func)) {
                    all_funcs = false;
                }
                let k = self.entity_key(d);
                if !keys.contains(&k) {
                    keys.push(k);
                }
            }
        }
        !all_funcs && keys.len() > 1
    }
    fn is_tag(&self, d: DeclId) -> bool {
        matches!(self.decls[d].k, DK::Class(_) | DK::Enum(_))
    }
    /// One key per entity: a class and an alias of it are one entity.
    pub fn entity_key(&self, d: DeclId) -> (u8, usize) {
        match &self.decls[d].k {
            DK::Class(c) => (1, *c),
            DK::Enum(e) => (2, *e),
            DK::Type(q) => match q.t {
                Ty::Class(c) => (1, c),
                Ty::Enum(e) => (2, e),
                _ => (0, d),
            },
            _ => (0, d),
        }
    }
    /// Qualified lookup in a namespace (NAM-13): the namespace and its
    /// inline namespaces, then, if that finds nothing, the namespaces
    /// that its using directives name. Returns the declarations and
    /// whether they are ambiguous (NAM-14).
    pub fn lookup_qualified_ns(&self, s: usize, name: &str) -> (Option<Vec<DeclId>>, bool) {
        let mut visited = vec![s];
        let sources = self.qual_ns(s, name, &mut visited);
        if sources.is_empty() {
            return (None, false);
        }
        let amb = self.sources_ambiguous(&sources);
        (Some(merge_sources(sources)), amb)
    }
    fn inline_names(&self, s: usize, name: &str, out: &mut Vec<DeclId>, depth: u32) {
        for d in self.own_names(s, name) {
            if !out.contains(&d) {
                out.push(d);
            }
        }
        if depth > 16 {
            return;
        }
        for &n in &self.scopes[s].using_ns {
            if self.inline_ns.contains(&n) && self.scopes[n].parent == Some(s) {
                self.inline_names(n, name, out, depth + 1);
            }
        }
    }
    fn qual_ns(&self, s: usize, name: &str, visited: &mut Vec<usize>) -> Vec<Vec<DeclId>> {
        let mut own = Vec::new();
        self.inline_names(s, name, &mut own, 0);
        if !own.is_empty() {
            return vec![own];
        }
        let mut out = Vec::new();
        for &n in &self.scopes[s].using_ns.clone() {
            if visited.contains(&n) || (self.inline_ns.contains(&n) && self.scopes[n].parent == Some(s)) {
                continue;
            }
            visited.push(n);
            out.extend(self.qual_ns(n, name, visited));
        }
        out
    }
    /// Qualified lookup in any scope, with ambiguity (NAM-13, NAM-17,
    /// NAM-19).
    pub fn lookup_qualified(&self, s: usize, name: &str) -> (Option<Vec<DeclId>>, bool) {
        match self.scopes[s].kind {
            SK::Namespace => self.lookup_qualified_ns(s, name),
            SK::Class(c) => match self.member_lookup(c, name) {
                Some((v, a)) => (Some(v), a),
                None => (None, false),
            },
            _ => {
                let v = self.own_names(s, name);
                if v.is_empty() {
                    (None, false)
                } else {
                    (Some(v), false)
                }
            }
        }
    }
    /// Lookup of a member in a class and its bases (NAM-17, DER-3,
    /// DER-4). Returns the declarations and whether they are found in
    /// more than one base subobject.
    pub fn member_lookup(&self, c: usize, name: &str) -> Option<(Vec<DeclId>, bool)> {
        let own = self.own_names(self.classes[c].scope, name);
        if !own.is_empty() {
            return Some((own, false));
        }
        let mut found: Vec<SubFound> = Vec::new();
        self.sub_lookup(c, name, &mut vec![c], false, &mut found, 0);
        if found.is_empty() {
            return None;
        }
        // DER-4: a declaration in a class hides the same name in a
        // virtual base of that class.
        let snapshot = found.clone();
        found.retain(|f| !(f.virt_key && snapshot.iter().any(|g| g.class != f.class && self.is_base_of_p(f.class, g.class))));
        let mut ids: Vec<DeclId> = Vec::new();
        for f in &found {
            for &d in &f.decls {
                if !ids.contains(&d) {
                    ids.push(d);
                }
            }
        }
        let mut keys: Vec<&Vec<usize>> = Vec::new();
        for f in &found {
            if !keys.contains(&&f.key) {
                keys.push(&f.key);
            }
        }
        let classes: std::collections::HashSet<usize> = found.iter().map(|f| f.class).collect();
        let shareable = ids.iter().all(|&d| match &self.decls[d].k {
            DK::Var(_) | DK::Type(_) | DK::Class(_) | DK::Enum(_) | DK::Enumerator(..) | DK::Template(_) => true,
            DK::Func(f) => f.is_static,
            _ => false,
        });
        let amb = keys.len() > 1 && !(classes.len() == 1 && shareable);
        Some((ids, amb))
    }
    /// Whether unqualified lookup of `name` finds it in the bases of an
    /// enclosing class only through a private base of another class
    /// (NAM-36, ACC-14, ACC-18).
    pub fn private_base_name(&self, name: &str) -> bool {
        let noms = self.nominations();
        for &s in self.stack.iter().rev() {
            if !self.own_names(s, name).is_empty() {
                return false;
            }
            if let SK::Class(c) = self.scopes[s].kind {
                if self.class_uncertain(c) {
                    return false;
                }
                if self.member_lookup(c, name).is_some() {
                    return self.path_access(c, name, 0, false) == Some(false);
                }
            }
            if self.lookup_level(s, name, &noms).0.is_some() {
                return false;
            }
        }
        false
    }
    /// Some(true) when a path to the name has no private edge below the
    /// first, Some(false) when every path has one, None when not found.
    fn path_access(&self, c: usize, name: &str, depth: u32, blocked: bool) -> Option<bool> {
        if depth > 20 {
            return None;
        }
        let mut res: Option<bool> = None;
        for b in &self.classes[c].bases {
            let Some(bc) = b.class else { return None };
            if self.friend_classes.contains(&bc) {
                return None;
            }
            let bl = blocked || (depth > 0 && b.access == 2);
            let r = if !self.own_names(self.classes[bc].scope, name).is_empty() { Some(!bl) } else { self.path_access(bc, name, depth + 1, bl) };
            match r {
                Some(true) => return Some(true),
                Some(false) => res = Some(false),
                None => {}
            }
        }
        res
    }
    fn sub_lookup(&self, c: usize, name: &str, path: &mut Vec<usize>, virt: bool, out: &mut Vec<SubFound>, depth: u32) {
        if depth > 20 {
            return;
        }
        let bases = self.classes[c].bases.clone();
        for b in bases {
            let Some(bc) = b.class else { continue };
            // The subobject: the path from the last virtual base, or the
            // whole path.
            let mut key: Vec<usize> = if b.virt { vec![usize::MAX, bc] } else { let mut k = path.clone(); k.push(bc); k };
            let v = virt || b.virt;
            if !b.virt && virt {
                // keep the virtual start of the path
                let mut k = path.clone();
                k.push(bc);
                key = k;
            }
            let own = self.own_names(self.classes[bc].scope, name);
            if !own.is_empty() {
                out.push(SubFound { decls: own, class: bc, key, virt_key: v });
                continue;
            }
            let mut p2 = key.clone();
            self.sub_lookup(bc, name, &mut p2, v, out, depth + 1);
        }
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
    /// Lookup in one named scope. With `with_using`, a qualified lookup
    /// (NAM-13); without it, only the scope and, for a class, its bases.
    pub fn lookup_in_scope(&self, s: usize, name: &str, with_using: bool) -> Option<Vec<DeclId>> {
        if with_using {
            return self.lookup_qualified(s, name).0;
        }
        let v = self.own_names(s, name);
        if !v.is_empty() {
            return Some(v);
        }
        if let SK::Class(c) = self.scopes[s].kind {
            return self.member_lookup(c, name).map(|r| r.0);
        }
        None
    }
    pub fn lookup_in_bases(&self, c: usize, name: &str, _depth: u32) -> Option<Vec<DeclId>> {
        let mut found: Vec<SubFound> = Vec::new();
        self.sub_lookup(c, name, &mut vec![c], false, &mut found, 0);
        let mut ids: Vec<DeclId> = Vec::new();
        for f in &found {
            for &d in &f.decls {
                if !ids.contains(&d) {
                    ids.push(d);
                }
            }
        }
        if ids.is_empty() {
            None
        } else {
            Some(ids)
        }
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

#[derive(Clone, Debug)]
pub struct SubFound {
    pub decls: Vec<DeclId>,
    pub class: usize,
    pub key: Vec<usize>,
    pub virt_key: bool,
}

/// Join the declarations from several places, without repeats.
pub fn merge_sources(sources: Vec<Vec<DeclId>>) -> Vec<DeclId> {
    let mut out = Vec::new();
    for v in sources {
        for d in v {
            if !out.contains(&d) {
                out.push(d);
            }
        }
    }
    out
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
