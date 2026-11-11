//! Diagnostics: errors with the Tidal rule ID that applies, notes about
//! constructs the front end does not model, and warnings for
//! `recommended` rules. Each diagnostic carries a span.
//!
//! The old front end muted diagnostics during tentative parses. The new
//! front end has no muted state: the parser reports nothing until a
//! declaration fails, and a substitution failure is a value (model
//! section 4), not a muted error.

use crate::source::{SourceMap, Span};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Unsupported,
    Warning,
}

impl Severity {
    pub fn word(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Unsupported => "unsupported",
            Severity::Warning => "warning",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Diag {
    pub sev: Severity,
    pub rule: Option<&'static str>,
    /// A second rule: the rule that the constant evaluation broke, when
    /// `rule` only says that a constant was needed.
    pub cause: Option<&'static str>,
    pub msg: String,
    pub span: Span,
}

impl Diag {
    /// The printed form of the old front end: `FILE:LINE: kind [RULE]: msg`,
    /// where LINE is the line in the unit's main file.
    pub fn render(&self, path: &str, map: &SourceMap) -> String {
        let line = map.unit_line(self.span);
        let kind = self.sev.word();
        match (self.rule, self.cause) {
            (Some(r), Some(c)) => format!("{}:{}: {} [{}, {}]: {}", path, line, kind, r, c, self.msg),
            (Some(r), None) => format!("{}:{}: {} [{}]: {}", path, line, kind, r, self.msg),
            _ => format!("{}:{}: {}: {}", path, line, kind, self.msg),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Diags {
    pub list: Vec<Diag>,
}

impl Diags {
    pub fn error(&mut self, span: Span, rule: Option<&'static str>, msg: impl Into<String>) {
        self.list.push(Diag { sev: Severity::Error, rule, cause: None, msg: msg.into(), span });
    }
    pub fn unsupported(&mut self, span: Span, msg: impl Into<String>) {
        self.list.push(Diag { sev: Severity::Unsupported, rule: None, cause: None, msg: msg.into(), span });
    }
    pub fn warning(&mut self, span: Span, rule: &'static str, msg: impl Into<String>) {
        self.list.push(Diag { sev: Severity::Warning, rule: Some(rule), cause: None, msg: msg.into(), span });
    }
    pub fn has_errors(&self) -> bool {
        self.list.iter().any(|d| d.sev == Severity::Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::{FileId, SourceMap};

    #[test]
    fn render_names_rules_and_unit_line() {
        let mut m = SourceMap::new(None);
        let h = m.add(std::path::Path::new("h.h"), Span::new(FileId(0), 4, 1));
        let mut d = Diags::default();
        d.error(Span::new(h, 10, 3), Some("PRIM-18"), "bad escape");
        d.unsupported(Span::new(FileId(0), 2, 1), "named escape");
        assert!(d.has_errors());
        assert_eq!(d.list[0].render("t.cpp", &m), "t.cpp:4: error [PRIM-18]: bad escape");
        assert_eq!(d.list[1].render("t.cpp", &m), "t.cpp:2: unsupported: named escape");
    }
}
