//! The `Names` oracle: what the parser asks about a name, and how it
//! tells analysis what a declaration declared (`DESIGN-rewrite.md`
//! section 2). The parser does no lookup of its own. `sem::names`
//! implements the oracle over a declare pass that records names and their
//! kinds (build step 2), and `Stub` answers from a fixed table for unit
//! tests.

use super::Node;
use crate::lex::Token;

/// The kind of template a name names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TKind {
    Class,
    Function,
    Variable,
    Alias,
    Concept,
}

/// What a name is, as far as the grammar needs to know.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Lookup found nothing. The parser reads the name as a value.
    Unknown,
    Value,
    Type,
    Namespace,
    Template(TKind),
}

/// The scope that a qualifier names, as the parser resolved it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qual {
    /// No qualifier: unqualified lookup.
    None,
    /// `::` alone.
    Global,
    /// A namespace, class, or enumeration that the oracle knows.
    Scope(u32),
    /// A dependent qualifier: the oracle answers by the reading that the
    /// syntax chose (`typename`, `template`, or a position of DEP-20).
    Dependent,
}

/// The oracle's answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Found {
    pub kind: Kind,
    /// The scope of a namespace, class, or enumeration.
    pub scope: Option<u32>,
    /// The name is a template parameter, so a template-id that uses it is
    /// dependent.
    pub param: bool,
}

impl Found {
    pub const UNKNOWN: Found = Found { kind: Kind::Unknown, scope: None, param: false };

    /// The name can begin a type: a type, a class template or alias
    /// template (whose arguments a placeholder deduces), or a concept
    /// before `auto`.
    pub fn is_type(&self) -> bool {
        matches!(self.kind, Kind::Type | Kind::Template(TKind::Class) | Kind::Template(TKind::Alias))
    }
    pub fn is_template(&self) -> bool {
        matches!(self.kind, Kind::Template(_))
    }
}

/// The scope the parser opens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enter {
    /// A namespace, class, or enumeration scope that `declared` gave.
    Scope(u32),
    /// A block, a function body, or a condition.
    Block,
    /// The parameters of a template head.
    Template,
    /// The parameters of a function declarator or a lambda.
    Prototype,
}

/// What the parser knows about a declaration when it reports it.
#[derive(Clone, Copy, Default)]
pub struct DeclCtx<'n> {
    /// The declaration's `decl_specifiers`, if it has them.
    pub specs: Option<&'n Node>,
    /// The declaration follows a template head.
    pub template: bool,
    /// The declaration is a template parameter.
    pub param: bool,
}

pub trait Names {
    /// What `name` is in the scope `q` names, or by unqualified lookup.
    /// This answers the three questions of the design: whether the name
    /// is a type, whether it is a template and of which kind, and whether
    /// it is a namespace or a class.
    fn find(&self, q: Qual, name: &str) -> Found;
    /// Whether a member template with this name is known, for a name
    /// after `.` or `->` with no `template`. The type of the object
    /// expression is not known to the parser.
    fn member_template(&self, name: &str) -> bool;
    /// The name of the class whose body is being read, if the innermost
    /// scope is that class.
    fn class_name(&self) -> Option<String>;
    /// The scope of the class template `name` while its definition, or a
    /// definition of its member, is being read: a template-id of it with
    /// dependent arguments names the current instantiation there (model
    /// section 2).
    fn current(&self, name: &str) -> Option<u32>;
    /// Whether the innermost scope is a block scope.
    fn block_scope(&self) -> bool;
    /// Record what a declaration, a head, or a declarator declares. Gives
    /// the scopes a namespace, class, or enumeration opens.
    fn declared(&mut self, node: &Node, toks: &[Token], ctx: DeclCtx) -> Vec<u32>;
    fn enter(&mut self, e: Enter);
    fn leave(&mut self);
    /// The open scopes, innermost last.
    fn stack(&self) -> Vec<u32>;
    fn set_stack(&mut self, s: Vec<u32>);
    /// How many scopes are open.
    fn depth(&self) -> usize {
        self.stack().len()
    }
}

/// An oracle with a fixed table, for unit tests.
#[derive(Default)]
pub struct Stub {
    pub types: Vec<&'static str>,
    pub templates: Vec<(&'static str, TKind)>,
    pub namespaces: Vec<&'static str>,
    pub class: Option<&'static str>,
    depth: Vec<u32>,
}

impl Stub {
    pub fn new(types: &[&'static str], templates: &[(&'static str, TKind)]) -> Stub {
        Stub { types: types.to_vec(), templates: templates.to_vec(), ..Stub::default() }
    }
}

impl Names for Stub {
    fn find(&self, q: Qual, name: &str) -> Found {
        if q == Qual::Dependent {
            return Found::UNKNOWN;
        }
        let kind = if let Some((_, k)) = self.templates.iter().find(|(n, _)| *n == name) {
            Kind::Template(*k)
        } else if self.types.contains(&name) {
            Kind::Type
        } else if self.namespaces.contains(&name) {
            Kind::Namespace
        } else {
            Kind::Unknown
        };
        let scope = matches!(kind, Kind::Type | Kind::Namespace | Kind::Template(TKind::Class)).then_some(1);
        Found { kind, scope, param: name == "T" }
    }
    fn member_template(&self, name: &str) -> bool {
        self.templates.iter().any(|(n, _)| *n == name)
    }
    fn class_name(&self) -> Option<String> {
        self.class.map(str::to_string)
    }
    fn block_scope(&self) -> bool {
        false
    }
    fn current(&self, _: &str) -> Option<u32> {
        None
    }
    fn declared(&mut self, _: &Node, _: &[Token], _: DeclCtx) -> Vec<u32> {
        vec![]
    }
    fn enter(&mut self, _: Enter) {
        self.depth.push(0);
    }
    fn leave(&mut self) {
        self.depth.pop();
    }
    fn stack(&self) -> Vec<u32> {
        self.depth.clone()
    }
    fn set_stack(&mut self, s: Vec<u32>) {
        self.depth = s;
    }
}
