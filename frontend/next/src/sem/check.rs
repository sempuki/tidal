//! The `Check` context that the `check.rs` part of each page uses
//! (`DESIGN-rewrite.md` section 3). It holds the unit being built, the
//! tokens, the current scope, the function being checked, and the hooks
//! of a proposal (section 7). A fact that a caller passes to a page
//! function is a parameter of that function, not a field here.
//!
//! This module names no rule. `error` takes the rule ID from the page
//! that calls it.

use super::tree::{Expr, Stmt, StmtId};
use super::types::QType;
use super::{EntityId, ScopeId, Unit};
use crate::diag::{Diag, Severity};
use crate::lex::Token;
use crate::source::Span;
use crate::syntax::{Kid, Node};
use std::collections::HashMap;
use std::rc::Rc;

/// The node-check point of section 7: a check of a proposal that runs on
/// each typed node that a production of the named kind made, after the
/// page's own check.
#[derive(Clone, Copy)]
pub struct NodeCheck {
    pub production: &'static str,
    pub check: fn(&mut Check, &Node, &Expr),
}

/// The `Ext` node point of section 7: the check of a statement node that
/// a proposal's alternative made. It gives an ordinary typed statement.
pub type ExtStmt = fn(&mut Check, &Node) -> Stmt;

/// What a proposal attaches to analysis.
#[derive(Clone, Default)]
pub struct Hooks {
    pub proposal: Option<&'static str>,
    pub node_checks: Vec<NodeCheck>,
    pub ext_stmt: Option<ExtStmt>,
}

/// A statement that `break` or `continue` can act on (JMP-1, JMP-2): its
/// id, whether it is a loop, and the labels that mark it.
#[derive(Clone, Debug)]
pub struct JumpTarget {
    pub id: StmtId,
    pub is_loop: bool,
    pub labels: Vec<Rc<str>>,
}

/// A `switch` whose body is being checked: its id, the type of its
/// condition after promotion, its `case` values, whether it has a
/// `default`, and how many variables were live at its start.
#[derive(Clone, Debug)]
pub struct SwitchCtx {
    pub id: StmtId,
    pub ty: QType,
    pub cases: Vec<i128>,
    pub default: bool,
    pub live_at_start: usize,
    /// How many `if constexpr` statements were open at its start (IF-17).
    pub ifs_at_start: usize,
}

/// The function whose body is being checked.
#[derive(Clone, Debug, Default)]
pub struct FnCtx {
    pub id: Option<EntityId>,
    pub ret: Option<QType>,
    /// The return type is a placeholder that the first return deduces.
    pub deduce: bool,
    pub constexpr: bool,
    pub consteval: bool,
    pub is_main: bool,
    /// The loops and switches that enclose the statement being checked,
    /// innermost last.
    pub targets: Vec<JumpTarget>,
    /// The labels that the next statement carries (a chain of labels).
    pub pending_labels: Vec<Rc<str>>,
    /// The labels defined in the function, and the `goto` statements,
    /// each with the variables with an initializer in scope there (JMP-6)
    /// and the `if constexpr` statements that enclose it (IF-17).
    pub labels: HashMap<Rc<str>, (Span, Vec<EntityId>, Vec<StmtId>)>,
    pub gotos: Vec<(Rc<str>, Span, Vec<EntityId>, Vec<StmtId>)>,
    /// The `if constexpr` statements that enclose the statement being
    /// checked, innermost last.
    pub constexpr_ifs: Vec<StmtId>,
    /// The automatic variables with an initializer in scope.
    pub live: Vec<EntityId>,
    /// The switches that enclose the statement, innermost last.
    pub switches: Vec<SwitchCtx>,
    /// The function body has a statement the machine does not run.
    pub unsupported: Option<String>,
    /// Depth of discarded statements of `if constexpr`, whose returns
    /// deduce nothing (RET-1).
    pub discarded: u32,
}

pub struct Check<'a> {
    pub u: Unit,
    pub toks: &'a [Token],
    pub scope: ScopeId,
    pub func: Option<FnCtx>,
    pub hooks: Hooks,
    pub werror: bool,
    pub wflags: Vec<String>,
    next_id: StmtId,
    /// Depth of unevaluated operands (`sizeof`, `decltype`), where a use
    /// of a name is no use (UNEV rules).
    pub unevaluated: u32,
    /// Declarations get C language linkage (LSP-1).
    pub lang_c: bool,
    /// The names that the parser's declare pass recorded in any scope.
    pub parser_names: std::collections::HashSet<String>,
    /// The entities with C language linkage, by name (LSP-4).
    pub c_names: HashMap<Rc<str>, EntityId>,
    /// The declarator being checked names a type (an alias or a
    /// typedef), where a function type can carry qualifiers (DCL-15).
    pub naming_type: bool,
    /// The parameters declared with an array type (UN-19).
    pub array_params: std::collections::HashSet<EntityId>,
    /// For each variable, the uses outside unevaluated operands and the
    /// uses as the left operand of `=` (ATTR-6).
    pub uses: HashMap<EntityId, u32>,
    pub assigned: HashMap<EntityId, u32>,
}

impl<'a> Check<'a> {
    pub fn new(toks: &'a [Token], u: Unit) -> Check<'a> {
        Check { u, toks, scope: 0, func: None, hooks: Hooks::default(), werror: false, wflags: Vec::new(), next_id: 0, unevaluated: 0, lang_c: false, parser_names: Default::default(), c_names: HashMap::new(), naming_type: false, array_params: Default::default(), uses: HashMap::new(), assigned: HashMap::new() }
    }

    // -----------------------------------------------------------------
    // Diagnostics

    /// Report an error with its rule; the result is the error type, so a
    /// caller can return it for the construct.
    pub fn error(&mut self, span: Span, rule: &'static str, msg: impl Into<String>) -> QType {
        self.u.diags.push(Diag { sev: Severity::Error, rule: Some(rule), cause: None, msg: msg.into(), span });
        QType::error()
    }
    /// An error whose second rule is the one a constant evaluation broke.
    pub fn error_cause(&mut self, span: Span, rule: &'static str, cause: Option<&'static str>, msg: impl Into<String>) {
        self.u.diags.push(Diag { sev: Severity::Error, rule: Some(rule), cause, msg: msg.into(), span });
    }
    /// A construct that build step 3 does not model.
    pub fn unsupported(&mut self, span: Span, msg: impl Into<String>) -> QType {
        self.u.diags.push(Diag { sev: Severity::Unsupported, rule: None, cause: None, msg: msg.into(), span });
        QType::error()
    }
    /// A warning of a `recommended` rule; an error under `-Werror`.
    pub fn warning(&mut self, span: Span, rule: &'static str, msg: impl Into<String>) {
        let msg = msg.into();
        if self.werror {
            self.u.diags.push(Diag { sev: Severity::Error, rule: Some(rule), cause: None, msg: format!("{} (warning treated as error)", msg), span });
        } else {
            self.u.diags.push(Diag { sev: Severity::Warning, rule: Some(rule), cause: None, msg, span });
        }
    }
    /// One of the `-W` flags is on.
    pub fn wflag(&self, names: &[&str]) -> bool {
        self.wflags.iter().any(|f| names.iter().any(|n| f == n))
    }
    /// A name that lookup did not find. When a construct before it was
    /// unsupported and the parser saw a declaration of the name, the
    /// declaration is likely one that analysis did not model, so the use
    /// is unsupported rather than an error of `rule`.
    pub fn not_found(&mut self, span: Span, rule: &'static str, name: &str) -> QType {
        let unsup = self.u.diags.iter().any(|d| d.sev == Severity::Unsupported);
        if (unsup && self.parser_names.contains(name)) || name.starts_with("__") {
            return self.unsupported(span, format!("'{}' names a declaration the front end did not model", name));
        }
        self.error(span, rule, format!("'{}' is not declared", name))
    }
    pub fn errors(&self) -> usize {
        self.u.diags.iter().filter(|d| d.sev == Severity::Error).count()
    }

    // -----------------------------------------------------------------
    // Tokens and nodes

    pub fn text(&self, k: &Kid) -> &'a str {
        match k {
            Kid::Tok(t) => self.toks[*t].text.as_str(),
            _ => "",
        }
    }
    /// The texts of a node's direct tokens.
    pub fn words(&self, n: &Node) -> Vec<&'a str> {
        n.words(self.toks)
    }
    pub fn has_word(&self, n: &Node, w: &str) -> bool {
        self.words(n).contains(&w)
    }
    /// The first direct token of a node.
    pub fn first_word(&self, n: &Node) -> &'a str {
        n.kids.iter().find_map(|k| matches!(k, Kid::Tok(_)).then(|| self.text(k))).unwrap_or("")
    }
    pub fn token(&self, k: &Kid) -> Option<&'a Token> {
        match k {
            Kid::Tok(t) => Some(&self.toks[*t]),
            _ => None,
        }
    }
    /// The source text of a subtree, tokens joined by spaces.
    pub fn source(&self, n: &Node) -> String {
        let mut ts = Vec::new();
        n.tokens(&mut ts);
        ts.iter().map(|&t| self.toks[t].text.as_str()).collect::<Vec<_>>().join(" ")
    }

    // -----------------------------------------------------------------
    // Scopes

    pub fn enter(&mut self, kind: super::ScopeKind) -> ScopeId {
        let s = self.u.new_scope(kind, Some(self.scope), "");
        self.scope = s;
        s
    }
    pub fn leave(&mut self, to: ScopeId) {
        self.scope = to;
    }
    pub fn next_stmt_id(&mut self) -> StmtId {
        self.next_id += 1;
        self.next_id
    }
    pub fn in_function(&self) -> bool {
        self.func.is_some()
    }

    // -----------------------------------------------------------------
    // The hooks of a proposal

    /// Run the node checks registered for a production on a typed node.
    pub fn node_checks(&mut self, n: &Node, e: &Expr) {
        if self.hooks.node_checks.is_empty() {
            return;
        }
        let checks: Vec<NodeCheck> = self.hooks.node_checks.iter().filter(|c| c.production == n.prod).copied().collect();
        for c in checks {
            (c.check)(self, n, e);
        }
    }
}
