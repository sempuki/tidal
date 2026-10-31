// Diagnostics: errors (with a Tidal rule ID when one applies), notes about
// constructs the front end does not understand, and warnings for
// `recommended` rules.

#[derive(Clone, Debug, PartialEq)]
pub enum Severity {
    Error,
    Unsupported,
    Warning,
}

#[derive(Clone, Debug)]
pub struct Diag {
    pub sev: Severity,
    pub rule: Option<&'static str>,
    /// A second rule: the rule that the constant evaluation broke, when
    /// `rule` only says that a constant was needed.
    pub cause: Option<&'static str>,
    pub msg: String,
    pub line: u32,
}

#[derive(Default)]
pub struct Diags {
    pub list: Vec<Diag>,
    /// Depth of "tentative" parsing. Diagnostics produced while tentative
    /// are dropped by the caller on backtrack.
    pub muted: u32,
}

impl Diags {
    pub fn error(&mut self, line: u32, rule: Option<&'static str>, msg: impl Into<String>) {
        if self.muted > 0 {
            return;
        }
        self.list.push(Diag { sev: Severity::Error, rule, cause: None, msg: msg.into(), line });
    }
    pub fn unsupported(&mut self, line: u32, msg: impl Into<String>) {
        if self.muted > 0 {
            return;
        }
        self.list.push(Diag { sev: Severity::Unsupported, rule: None, cause: None, msg: msg.into(), line });
    }
    pub fn warning(&mut self, line: u32, rule: &'static str, msg: impl Into<String>) {
        if self.muted > 0 {
            return;
        }
        self.list.push(Diag { sev: Severity::Warning, rule: Some(rule), cause: None, msg: msg.into(), line });
    }
    pub fn has_errors(&self) -> bool {
        self.list.iter().any(|d| d.sev == Severity::Error)
    }
}
