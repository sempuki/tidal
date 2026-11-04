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

/// The friends of one class (ACC-6).
#[derive(Clone, Debug, Default)]
pub struct FriendSet {
    pub classes: std::collections::HashSet<usize>,
    /// The names of friend classes: an elaborated friend declaration can
    /// introduce the class that a later definition completes.
    pub class_names: std::collections::HashSet<String>,
    pub names: std::collections::HashSet<String>,
    pub open: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SwitchCtx {
    pub start_set: std::collections::HashSet<DeclId>,
    pub cond_ty: Option<QTy>,
    pub cases: HashMap<i128, u32>,
    pub has_default: bool,
}

/// A generic lambda: where its tokens are, and its placeholder
/// parameters as invented template parameters (LAM-15).
#[derive(Clone, Debug)]
pub struct GenLambda {
    pub toks: Rc<Vec<crate::lexer::Token>>,
    pub start: usize,
    pub stack: Vec<usize>,
    pub fns: Vec<FnCtx>,
    pub depth: u32,
    pub params: Vec<crate::templ::TParam>,
    pub ptys: Vec<QTy>,
    pub limit: usize,
    pub origin: (usize, usize),
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
    /// LAM-18: the variables a lambda captures by copy explicitly.
    pub lambda_copies: Vec<DeclId>,
    /// The capture default of a lambda (Some(true) for `=`), whether it
    /// captures `this` explicitly, and the variables it names.
    pub lambda_default: Option<bool>,
    pub lambda_this: bool,
    pub lambda_named: Vec<DeclId>,
    pub ret_auto: Option<(crate::decl::DeclSpec, crate::decl::Declarator)>,
    /// THR-1: the number of declarations at the start of each try block
    /// of the function that encloses the parser.
    pub try_marks: Vec<usize>,
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
            lambda_copies: Vec::new(),
            lambda_default: None,
            lambda_this: false,
            lambda_named: Vec::new(),
            ret_auto: None,
            try_marks: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct PErr {
    pub line: u32,
    pub msg: String,
}

pub type PR<T> = Result<T, PErr>;

/// Parse failures at constructs that the front end knows it does not
/// model: these stay unsupported and are not TR-12 errors.
pub fn parse_known_unsupported(msg: &str) -> bool {
    ["outside the specification", "asm declaration", "uncertain scope", "named escape", "compound literal", "case range"].iter().any(|k| msg.contains(k))
}

#[derive(Clone)]
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
    /// ACC-6: the friends that each class declares. `open` is set when a
    /// friend declaration names something the front end does not track.
    pub friend_sets: HashMap<usize, FriendSet>,
    /// Names of constructs that were skipped, for the unsupported list.
    pub in_unevaluated: u32,
    /// ODR-9: functions with internal linkage, and the first evaluated
    /// call of each.
    pub internal_fns: std::collections::HashSet<DeclId>,
    pub internal_uses: Vec<(DeclId, u32)>,
    /// ODR-1: static data members with a definition outside the class.
    pub static_member_defs: std::collections::HashSet<DeclId>,
    /// For a static data member defined outside its class, the number of
    /// declarations that came before the definition (SINIT-7).
    pub static_member_pos: HashMap<DeclId, (usize, usize)>,
    /// A contextual conversion to bool is being formed (CVF-3).
    pub contextual: bool,
    /// CVF-3: a direct-initialization, such as `static_cast`, which also
    /// uses explicit conversion functions.
    pub direct_conv: bool,
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
    /// DEP-20: the next decl-specifiers are in a type-only position, so
    /// a qualified name with a dependent qualifier names a type.
    pub implicit_tn: bool,
    /// DEP-20: the parameters of the next parameter list are in
    /// type-only positions.
    pub params_tn: bool,
    /// DEP-20: the parameters of the list being parsed.
    pub in_params_tn: bool,
    /// DEP-19: the parameters of the next parameter list belong to a
    /// function declared with an unqualified name outside a class.
    pub params_value: bool,
    pub in_params_value: bool,
    /// The alignment from `alignas` in the head of the class whose body
    /// `class_body` parses next.
    pub head_align: u64,
    /// ORD-7: reverse the parameters of the first or the second template
    /// in the next `func_order`.
    pub order_rev: (bool, bool),
    /// DEP-23: the values that a using declaration names in a dependent
    /// base, in a pattern.
    pub dep_using: std::collections::HashSet<DeclId>,
    /// Inside the braces of an enumeration being defined.
    pub string_lits: usize,
    pub goto_sets: Vec<(String, u32, std::collections::HashSet<DeclId>, usize)>,
    /// TRY-7: the try blocks and handlers that enclose the parser, as
    /// markers in the jump sets.
    pub try_regions: Vec<usize>,
    pub try_region_next: usize,
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
    /// The virt-specifiers of a member function: override, final (VIRT-3).
    pub virt_specs: HashMap<DeclId, (bool, bool)>,
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
    /// Templates (templ.rs): their records, the template that each
    /// declaration names, and the template of each pattern class.
    pub templs: Vec<crate::templ::TemplInfo>,
    pub decl_tid: HashMap<DeclId, usize>,
    pub class_tid: HashMap<usize, usize>,
    /// Template parameters: (depth of the head, index).
    pub tparam: HashMap<DeclId, (u32, u32)>,
    /// The number of template heads around the parser.
    pub tdepth: u32,
    pub last_templ: Option<crate::templ::Last>,
    pub inst: Option<crate::templ::InstReq>,
    pub inst_out: Option<crate::templ::InstOut>,
    /// Injected class names of instances (TARG-11).
    pub injected: HashMap<DeclId, usize>,
    /// The template and arguments of each class made from a template.
    pub inst_of: HashMap<usize, (usize, Vec<TArg>)>,
    /// The template and arguments of each function or variable
    /// specialization.
    pub spec_of: HashMap<DeclId, (usize, Vec<TArg>)>,
    /// The value of each bound constant parameter.
    pub const_param_val: HashMap<DeclId, Expr>,
    /// The arguments of each bound template parameter pack.
    pub pack_binds: HashMap<DeclId, Vec<TArg>>,
    pub inst_depth: u32,
    pub inst_count: u32,
    pub class_label: HashMap<usize, String>,
    pub fn_label: HashMap<DeclId, String>,
    /// Parsing the declaration after `template <>`.
    pub explicit_spec: bool,
    /// Errors in the bodies of members of instances, reported when the
    /// member is used.
    pub body_errors: HashMap<DeclId, Vec<crate::diag::Diag>>,
    /// The uses that the body of a constructor of an instance makes,
    /// made when the constructor is used (INST-5).
    pub ctor_uses: HashMap<DeclId, Vec<DeclId>>,
    /// An anonymous union that is a member of a class: the class and the
    /// unnamed member that holds the union (UNI-7).
    pub anon_parent: HashMap<usize, (usize, DeclId)>,
    /// The generic lambdas, by the call operator of their closure type
    /// (LAM-15), and the specializations made for calls.
    pub generic_lambdas: HashMap<DeclId, Rc<GenLambda>>,
    pub generic_insts: Vec<(DeclId, Vec<QTy>, Option<DeclId>, bool, bool)>,
    /// The types of the placeholder parameters of the generic lambda that
    /// is parsed again for a call.
    pub lambda_inst: Option<Vec<QTy>>,
    /// The parameters or the result type of the generic lambda parsed
    /// again had an error: substitution failed (DED-29).
    pub lambda_inst_failed: bool,
    /// The closure types of generic lambdas without captures, whose
    /// conversion to a pointer to function the front end does not make.
    pub generic_conv: std::collections::HashSet<usize>,
    /// ALN-1: the alignment that alignas gives a variable.
    pub var_align: HashMap<DeclId, u64>,
    /// The token after the last attribute list that held alignas.
    pub align_end: usize,
    pub collect_uses: Option<Vec<DeclId>>,
    /// A declaration in a replay that declares a new entity.
    pub inst_fresh: bool,
    pub in_inst_body: bool,
    /// A member definition replayed for an instance.
    pub member_replay: bool,
    pub inst_busy: std::collections::HashSet<usize>,
    pub class_targ_objs: Vec<(String, DeclId)>,
    /// Function definitions that a replay made, which an explicit
    /// specialization of the member can replace.
    pub replayed_defs: std::collections::HashSet<DeclId>,
    /// Function parameters declared as packs.
    pub pack_params: std::collections::HashSet<DeclId>,
    /// TPL-38: friend function templates defined with a default.
    pub friend_defaults: Vec<DeclId>,
    /// CPT-26: the classes whose members are accessible where a
    /// constraint is checked, kept while the check replays its atoms.
    pub check_classes: Vec<usize>,
    /// DEP-15: the argument tokens of each partial specialization's class.
    pub spec_arg_toks: Vec<String>,
    pub partial_arg_toks: HashMap<usize, Vec<String>>,
    /// DEP-15: a qualified name read as a value although a declarator
    /// follows it; the parse failure after it is an error.
    pub dep15_hit: std::cell::Cell<u32>,
    /// The places where `expect` split a `>>` and inserted a token, in
    /// the current tokens. A position recorded before an insertion moves
    /// with it (`adjust_pos`).
    pub ins_log: Vec<usize>,
    /// ILC-3 in CTAD: the initializer-list phase is running, and no
    /// viable guide in it is not an error.
    pub ctad_list_phase: bool,
    pub ctad_none: bool,
    /// CTAD-27: the templates made for aggregate candidates.
    pub agg_guide_tids: std::collections::HashSet<usize>,
    /// TSP-22: member classes of instances given by an explicit
    /// specialization.
    pub explicit_member_classes: std::collections::HashSet<usize>,
    /// The function templates declared so far (TPL-38).
    pub tfn_declared: std::collections::HashSet<DeclId>,
    pub friend_templ: Option<DeclId>,
    /// DER-8: the bases whose constructors a class inherits, the
    /// constructor of the base for each inherited one, and the implicit
    /// default constructors made beside them.
    pub inh_bases: HashMap<usize, Vec<usize>>,
    pub inherited_from: HashMap<DeclId, DeclId>,
    pub implicit_default: std::collections::HashSet<DeclId>,
    /// LAM-26: the closure classes of lambdas with captures.
    pub capturing_closures: std::collections::HashSet<usize>,
    /// SB-1: the place that each structured binding names.
    pub sb_place: HashMap<DeclId, Rc<Expr>>,
    /// The last type name that named a bound or declared type pack.
    pub last_pack_ref: Option<DeclId>,
    /// The class template of the last dependent template-id before `::`.
    pub last_qual_templ: Option<(usize, Vec<TArg>)>,
    /// TPL-42: the definition of a member enumeration of a class template
    /// outside the class: the template, the name, and the position of `{`.
    pub member_enum_defs: HashMap<(usize, String), Rc<Vec<Token>>>,
    /// The values of designated initializers (LIST-5), by the number in
    /// the item `designator NAME N`.
    pub designated: Vec<Expr>,
    pub last_partial_class: Option<usize>,
    /// The head of each bound template template parameter (TPL-40).
    pub ttp_inner: HashMap<DeclId, std::rc::Rc<Vec<crate::templ::TParam>>>,
    /// Function specializations whose definitions are being made.
    pub body_busy: std::collections::HashSet<DeclId>,
    /// The explicit template arguments of the call being built.
    pub call_explicit: Option<std::rc::Rc<Vec<crate::templ::RawArg>>>,
    /// Pairs of candidates of the call being resolved where the first
    /// comes from a more specialized template (OVL-28).
    pub tie_more: Vec<(DeclId, DeclId)>,
    /// Inside the n-th copy of a pack expansion in a specialization.
    pub pack_index: Option<usize>,
    /// The lengths of the packs that the pattern being parsed names.
    pub pack_refs: Vec<usize>,
    /// The parameters that a function parameter pack became, under each
    /// of them (and under a placeholder for an empty pack).
    pub fn_param_packs: HashMap<DeclId, Vec<DeclId>>,
    /// The bodies of member functions of instances, made when a use
    /// needs them (INST-5, INST-7): the tokens and the deferred body.
    pub lazy_bodies: HashMap<DeclId, (std::rc::Rc<Vec<crate::lexer::Token>>, Deferred)>,
    /// Parse deferred bodies now, also those of instances.
    pub force_bodies: bool,
    pub finished_specs: std::collections::HashSet<DeclId>,
    pub in_tparam_decl: bool,
    pub pending_default_errors: Option<Vec<crate::diag::Diag>>,
    /// The definitions that a use needs, made at the end of the unit
    /// (INST-31).
    pub used_later: Vec<DeclId>,
    /// The errors of scoped enumerations of instances (INST-5).
    pub enum_errors: HashMap<usize, Vec<crate::diag::Diag>>,
    pub member_replay_decl: Option<DeclId>,
    pub finishing: bool,
    /// The errors of default arguments of specializations (INST-9).
    pub default_errors: HashMap<DeclId, Vec<crate::diag::Diag>>,
    pub depth_reported: bool,
    pub last_decl_qual: Option<usize>,
    /// Static data members whose definitions a replay made and no use
    /// has needed yet (INST-8).
    pub lazy_static_defs: std::collections::HashSet<DeclId>,
    /// In a replay: the declarations that namespace lookup sees (DEP-5).
    pub lookup_limit: usize,
    pub lazy_limits: HashMap<DeclId, usize>,
    /// The scopes that bind template parameters in replays.
    pub binding_scopes: std::collections::HashSet<usize>,
    /// Lookups that found a bound template parameter.
    pub binding_hits: u32,
    /// The bases of instances whose types depend on a parameter (DEP-11).
    pub dep_bases: std::collections::HashSet<(usize, usize)>,
    pub templ_head_decl: bool,
    pub templ_class_defined: bool,
    /// The template arguments written after the name of a function
    /// declaration.
    pub decl_fn_targs: Option<std::rc::Rc<Vec<crate::templ::RawArg>>>,
    /// The explicit instantiations of the unit: whether each is a
    /// definition (INST-26).
    pub explicit_insts: HashMap<String, bool>,
    /// Explicit instantiation definitions whose template is defined later
    /// (INST-20): the template, arguments, line, and whether a variable.
    pub later_explicit: Vec<(usize, Vec<TArg>, u32, bool)>,
    pub used_statics: std::collections::HashSet<DeclId>,
    pub pending_noexcept_errors: Option<Vec<crate::diag::Diag>>,
    /// The errors of the exception specifications of specializations,
    /// reported when a call chooses one (DED-27).
    pub noexcept_errors: HashMap<DeclId, Vec<crate::diag::Diag>>,
    /// In a substitution: the count of muted errors at its start.
    pub sfinae_mark: Option<u32>,
    /// Where the current tokens come from: the token vector and the
    /// index of the first token (main unit: vector 0, index 0).
    pub tok_origin: (usize, usize),
    pub lazy_origins: HashMap<DeclId, (usize, usize)>,
    /// Facts that the parse of a template's pattern recorded at a token,
    /// for its specializations (DEP rules).
    pub pattern_facts: HashMap<(usize, usize), u8>,
    pub call_adl_independent: bool,
    /// The template head being recorded has constraints.
    pub head_constrained: bool,
    pub tparam_constrained: bool,
    /// The position of the last `friend` keyword of decl-specifiers.
    pub friend_spec_pos: Option<usize>,
    /// Constraints (cons.rs): the head whose parameters an atomic
    /// constraint can name, a part that the front end cannot model, and
    /// the checks made so far.
    pub cons_head: Option<(Rc<Vec<crate::templ::TParam>>, u32)>,
    pub cons_unmodeled: bool,
    pub atom_counter: usize,
    #[allow(clippy::type_complexity)]
    pub sat_cache: HashMap<(usize, usize), Vec<(Vec<TArg>, Vec<usize>, crate::cons::Sat)>>,
    pub sat_busy: Vec<((usize, usize), Vec<TArg>)>,
    /// The trailing requires clause of a function.
    pub fn_cons: HashMap<DeclId, Rc<crate::cons::CNorm>>,
    pub fn_clause_toks: HashMap<DeclId, Vec<String>>,
    pub fn_sat_cache: HashMap<DeclId, crate::cons::Sat>,
    /// The constraints of the head being recorded (CNS-1, parts 1 and 2),
    /// and of the type constraints of the parameters so far.
    pub head_cons: Option<Rc<crate::cons::CNorm>>,
    pub tparam_cons: Option<Rc<crate::cons::CNorm>>,
    /// An abbreviated function template being declared (DED-34), the
    /// nesting of parameter lists, and the placeholder parameters so far
    /// in the outermost one.
    pub abbrev: Option<crate::cons::Abbrev>,
    /// The type constraints and the requires clause of the head being
    /// declared, for equivalence (CNS-26); the key of each declaration
    /// of a function template.
    pub head_parts: (Option<Rc<crate::cons::CNorm>>, Option<Rc<crate::cons::CNorm>>),
    pub fn_head_key: HashMap<DeclId, String>,
    pub in_concept_def: bool,
    /// Deduction of class template arguments (ctad.rs): the user guides
    /// of each class template, the parameters that give no forwarding
    /// reference, the tie-breakers of the guides, and the class template
    /// that a type specifier named without arguments.
    pub user_guides: HashMap<usize, Vec<crate::ctad::Guide>>,
    pub no_fwd: Option<(u32, u32)>,
    pub extra_ties: Vec<(DeclId, DeclId)>,
    pub ctad_tid: Option<usize>,
    /// The base types of each class as written, with a flag for a pack
    /// expansion; the classes that inherit constructors by the name of a
    /// base (CTAD-16); the condition of a conditional `explicit` of a
    /// constructor in a template.
    pub class_base_types: HashMap<usize, Vec<(QTy, bool)>>,
    pub inherits_ctors: HashMap<usize, Vec<String>>,
    pub explicit_cond: HashMap<DeclId, crate::templ::Source>,
    pub pending_explicit_cond: Option<crate::templ::Source>,
    /// The defaulted comparisons whose bodies were made (dcmp.rs).
    pub dcmp_done: HashMap<DeclId, bool>,
    /// Classes declared `final` (DER-1).
    pub final_classes: std::collections::HashSet<usize>,
    /// Defaulted comparisons that DCMP-2 or DCMP-5 deletes, with the
    /// rule that a use breaks.
    pub dcmp_deleted: HashMap<DeclId, &'static str>,
    pub dcmp_eq: std::collections::HashSet<usize>,
    /// The packs of the template being defined, and the places where the
    /// pattern names one (PACK-8).
    pub pack_decls: std::collections::HashSet<DeclId>,
    pub pack_uses: Vec<(usize, u32)>,
    /// The prospective destructors of each class (CNS-19).
    pub prospective_dtors: HashMap<usize, Vec<DeclId>>,
    pub param_nest: u32,
    pub auto_param_k: usize,
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
            friend_sets: HashMap::new(),
            in_unevaluated: 0,
            internal_fns: Default::default(),
            internal_uses: Vec::new(),
            static_member_defs: Default::default(),
            static_member_pos: HashMap::new(),
            contextual: false,
            direct_conv: false,
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
            implicit_tn: false,
            params_tn: false,
            in_params_tn: false,
            params_value: false,
            in_params_value: false,
            head_align: 0,
            order_rev: (false, false),
            dep_using: Default::default(),
            string_lits: 0,
            goto_sets: Vec::new(),
            try_regions: Vec::new(),
            try_region_next: 0,
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
            virt_specs: HashMap::new(),
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
            templs: Vec::new(),
            decl_tid: HashMap::new(),
            class_tid: HashMap::new(),
            tparam: HashMap::new(),
            tdepth: 0,
            last_templ: None,
            inst: None,
            inst_out: None,
            injected: HashMap::new(),
            member_enum_defs: HashMap::new(),
            designated: Vec::new(),
            inst_of: HashMap::new(),
            spec_of: HashMap::new(),
            const_param_val: HashMap::new(),
            pack_binds: HashMap::new(),
            inst_depth: 0,
            inst_count: 0,
            class_label: HashMap::new(),
            fn_label: HashMap::new(),
            explicit_spec: false,
            body_errors: HashMap::new(),
            ctor_uses: HashMap::new(),
            anon_parent: HashMap::new(),
            generic_lambdas: HashMap::new(),
            generic_insts: Vec::new(),
            lambda_inst: None,
            lambda_inst_failed: false,
            generic_conv: std::collections::HashSet::new(),
            var_align: HashMap::new(),
            align_end: usize::MAX,
            collect_uses: None,
            inst_fresh: false,
            in_inst_body: false,
            member_replay: false,
            inst_busy: Default::default(),
            class_targ_objs: Vec::new(),
            replayed_defs: Default::default(),
            pack_params: Default::default(),
            friend_defaults: Vec::new(),
            check_classes: Vec::new(),
            spec_arg_toks: Vec::new(),
            partial_arg_toks: HashMap::new(),
            dep15_hit: std::cell::Cell::new(0),
            ins_log: Vec::new(),
            ctad_list_phase: false,
            ctad_none: false,
            agg_guide_tids: Default::default(),
            explicit_member_classes: Default::default(),
            tfn_declared: Default::default(),
            friend_templ: None,
            inh_bases: HashMap::new(),
            inherited_from: HashMap::new(),
            implicit_default: Default::default(),
            capturing_closures: Default::default(),
            sb_place: HashMap::new(),
            last_pack_ref: None,
            last_qual_templ: None,
            last_partial_class: None,
            ttp_inner: HashMap::new(),
            body_busy: Default::default(),
            call_explicit: None,
            tie_more: Vec::new(),
            pack_index: None,
            pack_refs: Vec::new(),
            fn_param_packs: HashMap::new(),
            lazy_bodies: HashMap::new(),
            force_bodies: false,
            finished_specs: Default::default(),
            in_tparam_decl: false,
            pending_default_errors: None,
            used_later: Vec::new(),
            enum_errors: HashMap::new(),
            member_replay_decl: None,
            finishing: false,
            default_errors: HashMap::new(),
            depth_reported: false,
            last_decl_qual: None,
            lazy_static_defs: Default::default(),
            lookup_limit: usize::MAX,
            lazy_limits: HashMap::new(),
            binding_scopes: Default::default(),
            binding_hits: 0,
            dep_bases: Default::default(),
            templ_head_decl: false,
            templ_class_defined: false,
            decl_fn_targs: None,
            explicit_insts: HashMap::new(),
            later_explicit: Vec::new(),
            used_statics: Default::default(),
            pending_noexcept_errors: None,
            noexcept_errors: HashMap::new(),
            sfinae_mark: None,
            tok_origin: (0, 0),
            lazy_origins: HashMap::new(),
            pattern_facts: HashMap::new(),
            call_adl_independent: false,
            head_constrained: false,
            tparam_constrained: false,
            friend_spec_pos: None,
            cons_head: None,
            cons_unmodeled: false,
            atom_counter: 0,
            sat_cache: HashMap::new(),
            sat_busy: Vec::new(),
            fn_cons: HashMap::new(),
            fn_clause_toks: HashMap::new(),
            fn_sat_cache: HashMap::new(),
            head_cons: None,
            tparam_cons: None,
            abbrev: None,
            head_parts: (None, None),
            fn_head_key: HashMap::new(),
            in_concept_def: false,
            user_guides: HashMap::new(),
            no_fwd: None,
            extra_ties: Vec::new(),
            ctad_tid: None,
            class_base_types: HashMap::new(),
            inherits_ctors: HashMap::new(),
            explicit_cond: HashMap::new(),
            pending_explicit_cond: None,
            dcmp_done: HashMap::new(),
            final_classes: std::collections::HashSet::new(),
            dcmp_deleted: HashMap::new(),
            dcmp_eq: Default::default(),
            pack_decls: Default::default(),
            pack_uses: Vec::new(),
            prospective_dtors: HashMap::new(),
            param_nest: 0,
            auto_param_k: 0,
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
    /// A token position recorded when `ins_log` had `mark` entries, in
    /// the current tokens.
    pub fn adjust_pos(&self, mut x: usize, mark: usize) -> usize {
        for &q in self.ins_log.iter().skip(mark) {
            if q <= x {
                x += 1;
            }
        }
        x
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
            self.ins_log.push(self.pos + 1);
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
                    if t.text == ">>" && depth == 1 {
                        // The second `>` of `>>` closes an outer list.
                        self.toks[self.pos].text = ">".to_string();
                        let t2 = self.toks[self.pos].clone();
                        self.toks.insert(self.pos + 1, t2);
                    }
                    let t = self.tok();
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
    /// A token sequence that the parser does not derive. TR-12 makes it
    /// an error. The parse stays unsupported when the front end knows that
    /// it does not model the construct, or when an earlier unknown name
    /// left the scope uncertain, so that the failure can come from the
    /// front end and not from the program.
    pub fn parse_failure(&mut self, e: &PErr) {
        if self.dep15_hit.take() == e.line && e.line != 0 && self.in_template() {
            // DEP-15, DEP-19: the member of another specialization is a
            // value, so a declarator after it breaks the grammar (DEP-29).
            self.error(e.line, "DEP-29", format!("the grammar does not derive this: {} (a member of a specialization other than the current instantiation needs 'typename')", e.msg));
            return;
        }
        let uncertain = self.stack.iter().any(|&s| self.scopes[s].uncertain || matches!(self.scopes[s].kind, SK::Class(c) if self.class_uncertain(c)));
        if parse_known_unsupported(&e.msg) || uncertain {
            self.unsupported(e.line, format!("parse: {}", e.msg));
        } else {
            self.error(e.line, "TR-12", format!("the grammar does not derive this: {}", e.msg));
        }
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
        if self.class_uncertain(owner) {
            return true;
        }
        let mut here: Vec<usize> = self.fns.iter().filter_map(|f| f.class).chain(self.check_classes.iter().copied()).collect();
        for &s in &self.stack {
            if let SK::Class(c) = self.scopes[s].kind {
                here.push(c);
            }
        }
        if self.friend_classes.contains(&owner) {
            // ACC-6: a friend class or function of the owner.
            match self.friend_sets.get(&owner) {
                Some(fs) if !fs.open => {
                    if here.iter().any(|c| fs.classes.contains(c) || fs.class_names.contains(&self.classes[*c].name)) {
                        return true;
                    }
                    if self.fns.iter().any(|f| f.func.is_some_and(|d| fs.names.contains(&self.decls[d].name))) {
                        return true;
                    }
                }
                _ => return true,
            }
        }
        here.iter().any(|&c| c == owner || (a == 1 && self.is_base_of_p(owner, c)))
    }
    /// Whether a using-declaration in `c` or a class between `c` and the
    /// member's class brings the member `id` into scope (ACC-9).
    pub fn named_by_using(&self, c: usize, name: &str, id: DeclId, depth: u32) -> bool {
        if depth > 16 || self.scopes[self.decls[id].scope].kind == SK::Class(c) {
            return false;
        }
        if self.scopes[self.classes[c].scope].names.get(name).is_some_and(|v| v.contains(&id)) {
            return true;
        }
        self.classes[c].bases.iter().any(|b| b.class.is_some_and(|bc| self.named_by_using(bc, name, id, depth + 1)))
    }
    /// ACC-2, ACC-4, ACC-6: whether a member named through an object of
    /// class `naming` is accessible here. Err gives the rule.
    pub fn member_access_ok(&self, id: DeclId, naming: usize) -> Result<(), &'static str> {
        let SK::Class(owner) = self.scopes[self.decls[id].scope].kind else { return Ok(()) };
        if self.class_uncertain(owner) || self.class_uncertain(naming) {
            return Ok(());
        }
        let mut here: Vec<usize> = self.fns.iter().filter_map(|f| f.class).chain(self.check_classes.iter().copied()).collect();
        for &s in &self.stack {
            if let SK::Class(c) = self.scopes[s].kind {
                here.push(c);
            }
        }
        let friend_of = |p: &P, c: usize| -> bool {
            if !p.friend_classes.contains(&c) {
                return false;
            }
            match p.friend_sets.get(&c) {
                Some(fs) if !fs.open => here.iter().any(|h| fs.classes.contains(h) || fs.class_names.contains(&p.classes[*h].name)) || p.fns.iter().any(|f| f.func.is_some_and(|d| fs.names.contains(&p.decls[d].name))),
                _ => true,
            }
        };
        if owner != naming && self.is_base_of_p(owner, naming) && !self.base_accessible(naming, owner) && !friend_of(self, naming) {
            return Err("ACC-2");
        }
        match self.member_access.get(&id).copied().unwrap_or(0) {
            0 => Ok(()),
            2 => {
                if here.contains(&owner) || friend_of(self, owner) {
                    Ok(())
                } else {
                    Err("ACC-2")
                }
            }
            _ => {
                if here.contains(&owner) || friend_of(self, owner) {
                    return Ok(());
                }
                // ACC-4: in a derived class, only through an object of that
                // class or a class derived from it.
                let ok = here.iter().any(|&h| self.is_base_of_p(owner, h) && (naming == h || self.is_base_of_p(h, naming)) || (friend_of(self, h) && self.is_base_of_p(owner, h)));
                if ok { Ok(()) } else { Err("ACC-4") }
            }
        }
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
        if !name.is_empty() && self.inst_depth == 0 && matches!(self.scopes[scope].kind, SK::Class(_) | SK::Block | SK::Params) {
            // TPL-18: a name declared again in the scope of a template
            // parameter with that name.
            let mut s = self.scopes[scope].parent;
            while let Some(x) = s {
                match self.scopes[x].kind {
                    SK::Namespace => break,
                    SK::Template if !self.binding_scopes.contains(&x) => {
                        if self.scopes[x].names.get(name).is_some_and(|v| v.iter().any(|&d| self.tparam.contains_key(&d))) {
                            self.d.error(line, Some("TPL-18"), format!("'{}' is declared again in the scope of the template parameter with that name", name));
                            break;
                        }
                    }
                    _ => {}
                }
                s = self.scopes[x].parent;
            }
        }
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
        self.lookup_level_k(s, name, noms, self.in_base_clause == 0)
    }
    /// `hide` false: the lookup of a class or enumeration name after a
    /// class key, which a variable of the same name does not hide.
    pub fn lookup_level_k(&self, s: usize, name: &str, noms: &[(usize, usize)], hide: bool) -> (Option<Vec<DeclId>>, bool) {
        let mut sources: Vec<Vec<DeclId>> = Vec::new();
        let mut own = self.own_names(s, name);
        if self.scopes[s].kind == SK::Namespace {
            // DEP-5: in a specialization, a namespace shows the
            // declarations before the template's definition.
            own.retain(|&d| d < self.lookup_limit);
        }
        let mut amb = false;
        // NAM-40: a variable, function, data member, or enumerator hides
        // a class or enumeration of the same name in the same scope.
        if hide && self.hides_class(&own) {
            own.retain(|&d| !matches!(self.decls[d].k, DK::Class(_) | DK::Enum(_)));
        }
        if !own.is_empty() {
            sources.push(own);
        } else if let SK::Class(c) = self.scopes[s].kind {
            if let Some((r, a)) = self.member_lookup_unqual(c, name) {
                amb = a;
                sources.push(r);
            }
        }
        if self.scopes[s].kind == SK::Namespace {
            for &(t, n) in noms {
                if t == s {
                    let mut v = self.own_names(n, name);
                    v.retain(|&d| d < self.lookup_limit);
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
    /// NAM-40: the declarations hold a class or enumeration and a
    /// variable, data member, function, or enumerator that hides it.
    pub fn hides_class(&self, own: &[DeclId]) -> bool {
        own.len() > 1 && own.iter().any(|&d| matches!(self.decls[d].k, DK::Class(_) | DK::Enum(_))) && own.iter().any(|&d| match &self.decls[d].k {
            DK::Var(_) | DK::Field(_) | DK::Enumerator(..) => true,
            DK::Func(f) => !matches!(f.kind, FnKind::Ctor | FnKind::Dtor),
            _ => false,
        })
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
        own.retain(|&d| d < self.lookup_limit);
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
    /// Unqualified lookup in a class: the bases whose types depend on a
    /// template parameter are not searched (DEP-11).
    pub fn member_lookup_unqual(&self, c: usize, name: &str) -> Option<(Vec<DeclId>, bool)> {
        if !self.classes[c].bases.iter().any(|b| b.class.is_some_and(|bc| self.dep_bases.contains(&(c, bc)))) {
            return self.member_lookup(c, name);
        }
        let own = self.own_names(self.classes[c].scope, name);
        if !own.is_empty() {
            return Some((own, false));
        }
        let mut found: Vec<(Vec<DeclId>, bool)> = Vec::new();
        for b in &self.classes[c].bases {
            let Some(bc) = b.class else { continue };
            if self.dep_bases.contains(&(c, bc)) {
                continue;
            }
            if let Some(r) = self.member_lookup(bc, name) {
                found.push(r);
            }
        }
        match found.len() {
            0 => None,
            1 => found.pop(),
            _ => {
                let mut ids = Vec::new();
                for (v, _) in &found {
                    for &d in v {
                        if !ids.contains(&d) {
                            ids.push(d);
                        }
                    }
                }
                Some((ids, true))
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
        self.class_layout(c).map(|(s, a, _)| (s, a))
    }

    /// The size, alignment, and data size (the size without the padding
    /// at the end) of a class (src/layout.rs).
    pub fn class_layout(&self, c: usize) -> Option<(u64, u64, u64)> {
        self.layout(c).map(|l| (l.size, l.align, l.dsize))
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
