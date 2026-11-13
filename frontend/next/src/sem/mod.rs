//! `sem`: the arenas, entities, scopes, and typed trees of analysis
//! (`DESIGN-rewrite.md` sections 2 and 3). `names` is the declare pass
//! that answers the parser's questions (build step 2). The rest is the
//! data of build step 3: the type table (`types`), the typed tree
//! (`tree`), the entities and scopes of a `Unit`, and the `Check` context
//! that the page modules' `check.rs` parts share (`check`).
//!
//! This module names no rule of a page. The rules that decide what an
//! entity is sit in the page that declares it.

pub mod check;
pub mod names;
pub mod tree;
pub mod types;

use crate::diag::Diag;
use crate::source::Span;
use crate::target::Target;
use std::collections::HashMap;
use std::rc::Rc;
use tree::{Expr, Init, Stmt};
use types::{EnumId, FnType, IntK, QType};

pub type EntityId = usize;
pub type ScopeId = usize;

/// The storage duration of a variable (`foundations/storage`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Storage {
    Static,
    Thread,
    Automatic,
}

/// The linkage of a name (`declarations/storage-linkage`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linkage {
    None,
    Internal,
    External,
}

#[derive(Clone, Debug)]
pub struct Var {
    pub ty: QType,
    pub storage: Storage,
    pub constexpr: bool,
    pub constinit: bool,
    pub inline: bool,
    pub is_extern: bool,
    pub is_static: bool,
    pub param: bool,
    /// A definition was seen (a declaration with `extern` and no
    /// initializer is not one).
    pub defined: bool,
    pub init: Option<Init>,
    /// The initializer is a constant expression, so the variable can be
    /// usable in constant expressions (CE rules).
    pub const_init: bool,
    /// Whether a use was seen, for ATTR-6, and `[[maybe_unused]]`.
    pub used: bool,
    pub maybe_unused: bool,
    /// The alignment that `alignas` asks for.
    pub align: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct Func {
    pub ty: Rc<FnType>,
    pub params: Vec<EntityId>,
    /// The default argument of each parameter, if any.
    pub defaults: Vec<Option<Expr>>,
    pub body: Option<Rc<Stmt>>,
    pub constexpr: bool,
    pub consteval: bool,
    pub inline: bool,
    pub is_static: bool,
    pub deleted: bool,
    pub noreturn: bool,
    pub nodiscard: bool,
    /// `extern "C"`.
    pub c_linkage: bool,
    /// The return type is a placeholder that no return statement has
    /// deduced yet (RET-3).
    pub pending_return: bool,
    /// Declared with a placeholder return type (RET-4).
    pub placeholder: bool,
    /// Named by an expression somewhere in the unit (ODR-9).
    pub used: bool,
    pub defined: bool,
    /// The body could not be checked in full, so the function does not
    /// run.
    pub unsupported: Option<String>,
    /// The class of a member function, and for a non-static one the
    /// cv-qualifiers of its object.
    pub class: Option<types::ClassId>,
    /// A constructor or a destructor.
    pub special: Special,
    pub explicit: bool,
    pub member_static: bool,
    pub obj_cv: (bool, bool),
}

#[derive(Clone, Debug)]
pub enum EntKind {
    Var(Var),
    Func(Func),
    /// An enumerator, with the type it has inside the enumerator list
    /// of an enumeration with no fixed type (ENUM-10, ENUM-13).
    Enumerator { en: EnumId, value: i128, ty: IntK },
    Typedef(QType),
    Namespace(ScopeId),
    Enum(EnumId),
    /// The name of a class (`classes/members`).
    Class(types::ClassId),
    /// A non-static data member of a class.
    Field(Field),
    /// An entity of a kind that build step 3 does not model (a class, a
    /// template); a name that finds it gives `unsupported`.
    Unsupported(&'static str),
}

#[derive(Clone, Debug)]
pub struct Entity {
    pub name: Rc<str>,
    pub kind: EntKind,
    pub span: Span,
    pub scope: ScopeId,
    pub linkage: Linkage,
    /// `[[deprecated]]`, with its message.
    pub deprecated: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopeKind {
    Namespace,
    /// The members of a class.
    Class,
    Block,
    /// The parameters of a function definition, with its body's
    /// outermost block (FN rules).
    Function,
    Prototype,
    Enum,
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub kind: ScopeKind,
    pub parent: Option<ScopeId>,
    pub names: HashMap<Rc<str>, Vec<EntityId>>,
    /// Namespaces that a using-directive or an inline or unnamed
    /// namespace makes visible here.
    pub using: Vec<ScopeId>,
    pub inline: bool,
    pub name: Rc<str>,
}

/// The kind of a special member function.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Special {
    #[default]
    None,
    Ctor,
    Dtor,
}

/// What a constructor does before its body (CTOR-5, CTOR-6): an
/// initializer for each data member in order, or a call of another
/// constructor of the class.
#[derive(Clone, Debug)]
pub enum MemInits {
    Fields(Vec<Init>),
    Delegate(Init),
}

/// `class`, `struct`, or `union`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassKey {
    Class,
    Struct,
    Union,
}

/// The access of a member (`classes/access`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    Public,
    Protected,
    Private,
}

/// A class: its name, key, member scope, and non-static data members in
/// the order of their declarations.
#[derive(Clone, Debug)]
pub struct ClassInfo {
    pub name: Rc<str>,
    pub key: ClassKey,
    pub scope: ScopeId,
    pub fields: Vec<EntityId>,
    pub complete: bool,
}

/// The closure type of a lambda expression (`expressions/lambdas`): its
/// call operator and the number of its captures.
#[derive(Clone, Debug)]
pub struct Closure {
    pub op: EntityId,
    pub captures: usize,
}

/// A non-static data member: its type, its class, its position among
/// the class's data members, its default member initializer, and its
/// access.
#[derive(Clone, Debug)]
pub struct Field {
    pub ty: QType,
    pub class: types::ClassId,
    pub index: usize,
    pub init: Option<tree::Init>,
    pub mutable: bool,
    pub access: Access,
}

#[derive(Clone, Debug)]
pub struct EnumInfo {
    pub name: Rc<str>,
    pub scoped: bool,
    /// The underlying type: fixed, or chosen when the enumeration is
    /// complete. None while an enumeration with no fixed type is open.
    pub underlying: Option<IntK>,
    pub fixed: bool,
    pub enumerators: Vec<EntityId>,
    /// The scope of a scoped enumeration's enumerators.
    pub scope: ScopeId,
    pub complete: bool,
}

/// The result of analysis for one translation unit: read-only once
/// analysis is done.
#[derive(Debug)]
pub struct Unit {
    pub entities: Vec<Entity>,
    pub scopes: Vec<Scope>,
    pub enums: Vec<EnumInfo>,
    pub classes: Vec<ClassInfo>,
    /// The variables with static storage, in the order of their
    /// definitions (SINIT rules).
    pub statics: Vec<EntityId>,
    pub diags: Vec<Diag>,
    pub target: Target,
    /// The global scope of the translation unit being checked. A program
    /// of several units checks them one after another in one arena, each
    /// with its own global scope.
    pub root: ScopeId,
    /// The global scope of each unit checked so far.
    pub roots: Vec<ScopeId>,
    /// The first entity of the unit being checked.
    pub first_entity: EntityId,
    /// For a declaration with external linkage that its unit does not
    /// define, the definition in another unit (LNK rules, linking).
    pub links: HashMap<EntityId, EntityId>,
    /// The access of each member that is not a data member.
    pub access: HashMap<EntityId, Access>,
    /// The member initializers of each constructor with a body.
    pub mem_inits: HashMap<EntityId, MemInits>,
    /// The constructors defaulted on their first declaration, which act
    /// as the implicit ones (CTOR-2).
    pub defaulted: std::collections::HashSet<EntityId>,
    /// The closure types, by class.
    pub closures: HashMap<types::ClassId, Closure>,
}

impl Default for Unit {
    fn default() -> Unit {
        Unit {
            entities: Vec::new(),
            scopes: vec![Scope { kind: ScopeKind::Namespace, parent: None, names: HashMap::new(), using: Vec::new(), inline: false, name: "".into() }],
            enums: Vec::new(),
            classes: Vec::new(),
            statics: Vec::new(),
            diags: Vec::new(),
            target: Target::default(),
            root: 0,
            roots: vec![0],
            first_entity: 0,
            links: HashMap::new(),
            access: HashMap::new(),
            mem_inits: HashMap::new(),
            defaulted: Default::default(),
            closures: HashMap::new(),
        }
    }
}

impl Unit {
    pub fn new_scope(&mut self, kind: ScopeKind, parent: Option<ScopeId>, name: &str) -> ScopeId {
        self.scopes.push(Scope { kind, parent, names: HashMap::new(), using: Vec::new(), inline: false, name: name.into() });
        self.scopes.len() - 1
    }
    pub fn add(&mut self, e: Entity) -> EntityId {
        let scope = e.scope;
        let name = e.name.clone();
        self.entities.push(e);
        let id = self.entities.len() - 1;
        if !name.is_empty() {
            self.scopes[scope].names.entry(name).or_default().push(id);
        }
        id
    }
    /// An incomplete type: void, an array of unknown bound, or a class
    /// whose definition has not ended, also as an array element.
    pub fn is_incomplete(&self, t: &QType) -> bool {
        t.is_incomplete() || t.base_elem().class().is_some_and(|c| !self.classes[c].complete)
    }
    pub fn field(&self, id: EntityId) -> Option<&Field> {
        match &self.entities.get(id)?.kind {
            EntKind::Field(f) => Some(f),
            _ => None,
        }
    }
    /// The types of a class's data members, in order.
    pub fn field_types(&self, c: types::ClassId) -> Vec<QType> {
        self.classes[c].fields.iter().filter_map(|&f| self.field(f).map(|x| x.ty.clone())).collect()
    }
    pub fn var(&self, id: EntityId) -> Option<&Var> {
        match &self.entities.get(id)?.kind {
            EntKind::Var(v) => Some(v),
            _ => None,
        }
    }
    pub fn var_mut(&mut self, id: EntityId) -> Option<&mut Var> {
        match &mut self.entities.get_mut(id)?.kind {
            EntKind::Var(v) => Some(v),
            _ => None,
        }
    }
    pub fn func(&self, id: EntityId) -> Option<&Func> {
        match &self.entities.get(id)?.kind {
            EntKind::Func(f) => Some(f),
            _ => None,
        }
    }
    pub fn func_mut(&mut self, id: EntityId) -> Option<&mut Func> {
        match &mut self.entities.get_mut(id)?.kind {
            EntKind::Func(f) => Some(f),
            _ => None,
        }
    }
    /// The function named `main` in the global namespace of a unit.
    pub fn main(&self) -> Option<EntityId> {
        let defined = |e: &EntityId| self.func(*e).is_some_and(|f| f.body.is_some());
        let all: Vec<EntityId> = self.roots.iter().filter_map(|&r| self.scopes[r].names.get("main")).flatten().copied().filter(|&e| self.func(e).is_some()).collect();
        all.iter().copied().find(defined).or_else(|| all.first().copied())
    }
    /// Start the next translation unit of a program in this arena: a new
    /// global scope, and no diagnostics yet.
    pub fn next_unit(&mut self) {
        let root = self.new_scope(ScopeKind::Namespace, None, "");
        self.root = root;
        self.roots.push(root);
        self.first_entity = self.entities.len();
        self.diags.clear();
    }
    /// The namespace scope that holds a scope, itself included.
    pub fn enclosing_namespace(&self, mut s: ScopeId) -> ScopeId {
        while self.scopes[s].kind != ScopeKind::Namespace {
            match self.scopes[s].parent {
                Some(p) => s = p,
                None => break,
            }
        }
        s
    }
    /// The scope is inside a function (a block or parameter scope).
    pub fn is_local(&self, mut s: ScopeId) -> bool {
        loop {
            match self.scopes[s].kind {
                ScopeKind::Block | ScopeKind::Function | ScopeKind::Prototype => return true,
                ScopeKind::Namespace | ScopeKind::Class => return false,
                ScopeKind::Enum => {}
            }
            match self.scopes[s].parent {
                Some(p) => s = p,
                None => return false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn var(ty: QType) -> EntKind {
        EntKind::Var(Var {
            ty,
            storage: Storage::Static,
            constexpr: false,
            constinit: false,
            inline: false,
            is_extern: false,
            is_static: false,
            param: false,
            defined: true,
            init: None,
            const_init: false,
            used: false,
            maybe_unused: false,
            align: None,
        })
    }

    #[test]
    fn entities_are_found_by_scope_and_name() {
        let mut u = Unit::default();
        let ns = u.new_scope(ScopeKind::Namespace, Some(0), "n");
        let blk = u.new_scope(ScopeKind::Block, Some(ns), "");
        let id = u.add(Entity { name: "x".into(), kind: var(QType::int()), span: Span::default(), scope: blk, linkage: Linkage::None, deprecated: None });
        assert_eq!(u.scopes[blk].names["x"], vec![id]);
        assert!(u.is_local(blk));
        assert!(!u.is_local(ns));
        assert_eq!(u.enclosing_namespace(blk), ns);
        assert!(u.var(id).is_some() && u.func(id).is_none());
    }
}
