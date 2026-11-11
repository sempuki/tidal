//! The token stream and the preprocessing slice. Moved from the old
//! front end's `lexer.rs`; a token now has a span. Tidal leaves lexical
//! structure out of scope (charter), so the slice and the punctuators
//! come from general knowledge. `lex` calls `foundations::odr` for the
//! directive lines (INC), `foundations::keywords` for the alternative
//! spellings (KW-2), and `expressions::primary` for the literals (PRIM).

use crate::diag::Diags;
use crate::expressions::primary::lexical;
use crate::foundations::{keywords, odr};
use crate::source::{FileId, SourceMap, Span};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TK {
    Ident,
    Punct,
    Int,
    Float,
    Char,
    Str,
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prefix {
    None,
    U8,
    U16,
    U32,
    Wide,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISize {
    None,
    L,
    LL,
    Z,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSuffix {
    None,
    F,
    L,
    Ext,
}

#[derive(Clone, Debug)]
pub enum Lit {
    Int { value: u128, decimal: bool, unsigned: bool, size: ISize },
    Float { value: f64, suffix: FSuffix },
    /// `value` is the value of the literal in its type, `multi` is set for
    /// a multi-character literal of type int.
    Char { prefix: Prefix, value: i128, multi: bool },
    /// Code units, without the terminating null.
    Str { prefix: Prefix, units: Vec<u32> },
    /// A literal with a user-defined suffix (OPR-8): the suffix, the
    /// literal without it, and the source characters of a numeric
    /// literal without it, for the raw form.
    UserDefined { suffix: String, base: Box<Lit>, raw: String },
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TK,
    pub text: String,
    pub span: Span,
    pub lit: Option<Lit>,
}

impl Token {
    pub fn is(&self, s: &str) -> bool {
        matches!(self.kind, TK::Punct | TK::Ident) && self.text == s
    }
}

const PUNCTS: &[&str] = &[
    "<=>", "->*", "...", "<<=", ">>=", "::", "->", ".*", "++", "--", "<<", ">>", "<=", ">=", "==", "!=", "&&", "||",
    "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "##", "{", "}", "[", "]", "(", ")", ";", ":", "?", ".", "+",
    "-", "*", "/", "%", "^", "&", "|", "~", "!", "=", "<", ">", ",", "#",
];

/// The `#define` names of the slice, and the files that `#pragma once`
/// closed (INC-1 to INC-5).
#[derive(Default)]
struct Pp {
    /// Names that `#define` has defined, with their replacement text.
    defines: std::collections::HashMap<String, String>,
    once: std::collections::HashSet<PathBuf>,
    depth: u32,
    expanding: u32,
}

/// One conditional group (`conditional_group` of the ODR page): whether
/// its current branch is active, whether a branch was taken, and whether
/// the text around it is active.
struct Cond {
    active: bool,
    taken: bool,
    outer: bool,
}

/// The tokens of one translation unit, ending with an end-of-file token.
pub struct Lexed {
    pub toks: Vec<Token>,
    pub map: SourceMap,
}

/// Lex a translation unit whose main file is `path`. `#include "file"`
/// lines are replaced by the tokens of the file (INC-1).
pub fn lex_file(src: &str, path: Option<&Path>, d: &mut Diags) -> Lexed {
    let mut l = Lexer { map: SourceMap::new(path), pp: Pp::default(), d };
    if let Some(p) = path {
        if src.lines().any(odr::syntax::pragma_once) {
            l.pp.once.insert(p.canonicalize().unwrap_or(p.to_path_buf()));
        }
    }
    let mut out = l.lex_in(src, FileId(0), path);
    let span = out.last().map_or(Span::new(FileId(0), 1, 1), |t| t.span);
    out.push(Token { kind: TK::Eof, text: String::new(), span, lit: None });
    Lexed { toks: out, map: l.map }
}

/// One line for each token and each diagnostic, in the form that
/// `tidal-fe lex` prints, so that `tools/frontend_check.py` can compare
/// the two lexers. The line is the line in the unit's main file.
pub fn dump(lexed: &Lexed, d: &Diags) -> String {
    let mut s = String::new();
    for t in &lexed.toks {
        s.push_str(&format!("{:?}\t{}\t{}\t{:?}\n", t.kind, escape(&t.text), lexed.map.unit_line(t.span), t.lit));
    }
    for x in &d.list {
        s.push_str(&format!("diag\t{:?}\t{:?}\t{}\t{}\n", x.sev, x.rule, lexed.map.unit_line(x.span), x.msg));
    }
    s
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\n', "\\n").replace('\t', "\\t").replace('\r', "\\r")
}

struct Lexer<'a> {
    map: SourceMap,
    pp: Pp,
    d: &'a mut Diags,
}

impl Lexer<'_> {
    /// Handle one directive line other than a conditional (the text after
    /// `#`). Returns the tokens it adds.
    fn directive(&mut self, text: &str, at: Span, path: Option<&Path>) -> Vec<Token> {
        let (word, rest) = odr::syntax::directive_word(text);
        match word {
            "include" => {
                let line = format!("#{}", text);
                let Some(name) = odr::syntax::include_directive(&line) else {
                    self.d.unsupported(at, "#include form other than \"file\" (outside the specification)");
                    return vec![];
                };
                let Some(base) = path else {
                    self.d.unsupported(at, "#include without a file name for the unit");
                    return vec![];
                };
                // INC-2: the directory of the file that holds the directive.
                let file = base.parent().unwrap_or(Path::new(".")).join(name);
                let Ok(src) = std::fs::read_to_string(&file) else {
                    self.d.error(at, Some("INC-3"), format!("no file matches the header name \"{}\"", name));
                    return vec![];
                };
                let canon = file.canonicalize().unwrap_or(file.clone());
                if self.pp.once.contains(&canon) {
                    return vec![]; // INC-5
                }
                if src.lines().any(odr::syntax::pragma_once) {
                    self.pp.once.insert(canon);
                }
                if self.pp.depth > 64 {
                    self.d.unsupported(at, "#include nested too deeply");
                    return vec![];
                }
                self.pp.depth += 1;
                let id = self.map.add(&file, at);
                let toks = self.lex_in(&src, id, Some(&file));
                self.pp.depth -= 1;
                toks
            }
            "pragma" if rest == "once" => vec![],
            "define" => {
                let k = rest.find(|c: char| !(c.is_alphanumeric() || c == '_')).unwrap_or(rest.len());
                let (name, value) = (&rest[..k], &rest[k..]);
                if name.is_empty() || value.starts_with('(') {
                    self.d.unsupported(at, "#define of a function-like macro (outside the specification)");
                    return vec![];
                }
                // INC-4: a guard_value does not change the guard.
                self.pp.defines.insert(name.to_string(), value.trim().to_string());
                vec![]
            }
            "undef" => {
                self.pp.defines.remove(rest);
                vec![]
            }
            _ => {
                self.d.unsupported(at, "preprocessor directive (outside the specification)");
                vec![]
            }
        }
    }

    /// The tokens of one file. `path` is the file's path for includes, and
    /// `None` for the replacement text of a `#define` name, whose tokens
    /// take the span of the name (`at`).
    fn lex_in(&mut self, src: &str, file: FileId, path: Option<&Path>) -> Vec<Token> {
        self.lex_text(src, file, path, None)
    }

    fn lex_text(&mut self, src: &str, file: FileId, path: Option<&Path>, at: Option<Span>) -> Vec<Token> {
        let mut conds: Vec<Cond> = Vec::new();
        let b: Vec<char> = src.chars().collect();
        let mut i = 0;
        let mut line = 1u32;
        let mut ls = 0usize; // index of the first character of the line
        let mut out: Vec<Token> = Vec::new();
        let n = b.len();
        let mut last_line = 0u32; // the line of the last token made from this text
        while i < n {
            let c = b[i];
            let sp = at.unwrap_or(Span::new(file, line, (i - ls) as u32 + 1));
            if c == '\n' {
                line += 1;
                i += 1;
                ls = i;
                continue;
            }
            if c.is_whitespace() {
                i += 1;
                continue;
            }
            if c == '/' && i + 1 < n && b[i + 1] == '/' {
                while i < n && b[i] != '\n' {
                    i += 1;
                }
                continue;
            }
            if c == '/' && i + 1 < n && b[i + 1] == '*' {
                i += 2;
                while i + 1 < n && !(b[i] == '*' && b[i + 1] == '/') {
                    if b[i] == '\n' {
                        line += 1;
                        ls = i + 1;
                    }
                    i += 1;
                }
                i += 2;
                continue;
            }
            let is_directive = c == '#' && (out.is_empty() || last_line != line) && line_start(&b, i);
            if !conds.iter().all(|c| c.active) && !is_directive {
                // Text in a branch that is not taken.
                while i < n && b[i] != '\n' {
                    i += 1;
                }
                continue;
            }
            if is_directive {
                let st = i + 1;
                while i < n && b[i] != '\n' {
                    i += 1;
                }
                let raw: String = b[st..i].iter().collect();
                let text = raw.split("//").next().unwrap_or("").trim().to_string();
                let dline = format!("#{}", text);
                let outer = conds.iter().all(|c| c.active);
                // The conditional groups of the ODR page's grammar. INC-4: a
                // later inclusion of a guarded file adds no text.
                if let Some((word, rest)) = odr::syntax::if_line(&dline) {
                    let v = match word {
                        "ifndef" => Some(!self.pp.defines.contains_key(rest)),
                        "ifdef" => Some(self.pp.defines.contains_key(rest)),
                        _ if !outer => Some(false),
                        _ => eval_if(rest, &self.pp).map(|x| x != 0),
                    };
                    let v = v.unwrap_or_else(|| {
                        self.d.unsupported(sp, "#if expression (outside the specification)");
                        false
                    });
                    conds.push(Cond { active: outer && v, taken: v, outer });
                } else if let Some((word, rest)) = odr::syntax::else_line(&dline) {
                    let Some(top) = conds.last() else {
                        self.d.unsupported(sp, "#else without #if (outside the specification)");
                        continue;
                    };
                    let (outer, taken) = (top.outer, top.taken);
                    let v = match word {
                        "else" => Some(true),
                        "elifdef" => Some(self.pp.defines.contains_key(rest)),
                        "elifndef" => Some(!self.pp.defines.contains_key(rest)),
                        _ if taken || !outer => Some(false),
                        _ => eval_if(rest, &self.pp).map(|x| x != 0),
                    };
                    let v = v.unwrap_or_else(|| {
                        self.d.unsupported(sp, "#elif expression (outside the specification)");
                        false
                    });
                    let top = conds.last_mut().unwrap();
                    top.active = outer && !taken && v;
                    top.taken = taken || v;
                } else if odr::syntax::directive_word(&text).0 == "endif" {
                    if conds.pop().is_none() {
                        self.d.unsupported(sp, "#endif without #if (outside the specification)");
                    }
                } else if outer {
                    let toks = self.directive(&text, sp, path);
                    if !toks.is_empty() {
                        last_line = line;
                    }
                    out.extend(toks);
                }
                continue;
            }
            let before = out.len();
            i = self.token(&b, i, sp, file, path, &mut out, &mut line, &mut ls);
            if out.len() > before {
                last_line = line;
            }
        }
        out
    }

    /// Lex one token (or one `#define` name's replacement) at `i`.
    #[allow(clippy::too_many_arguments)]
    fn token(&mut self, b: &[char], mut i: usize, sp: Span, file: FileId, path: Option<&Path>, out: &mut Vec<Token>, line: &mut u32, ls: &mut usize) -> usize {
        let n = b.len();
        let c = b[i];
        // Identifiers, keywords, and prefixed literals.
        if lexical::id_start(c) {
            let st = i;
            while i < n && (lexical::id_start(b[i]) || lexical::digit(b[i])) {
                i += 1;
            }
            let word: String = b[st..i].iter().collect();
            if i < n && (b[i] == '"' || b[i] == '\'') {
                let raw = word.ends_with('R');
                let prefix = if word == "R" { Some(Prefix::None) } else { lexical::char_prefix(word.strip_suffix('R').unwrap_or(&word)) };
                if let Some(p) = prefix {
                    if raw && b[i] == '"' {
                        let (end, tok) = lexical::raw_string(b, i, p, sp, self.d);
                        // count the line breaks inside the raw string
                        let breaks = tok.text.matches('\n').count() as u32;
                        if breaks > 0 {
                            *line += breaks;
                            if let Some(k) = (i..end.min(n)).rev().find(|&k| b[k] == '\n') {
                                *ls = k + 1;
                            }
                        }
                        out.push(tok);
                        return end;
                    } else if !raw {
                        return lexical::quoted(b, i, p, sp, out, self.d);
                    }
                }
            }
            if let Some(v) = self.pp.defines.get(&word).filter(|v| !v.is_empty()).cloned() {
                // An object-like name from `#define` with a value
                // (outside the specification).
                if self.pp.expanding < 16 {
                    self.pp.expanding += 1;
                    let toks = self.lex_text(&v, file, path, Some(sp));
                    self.pp.expanding -= 1;
                    out.extend(toks);
                    return i;
                }
            }
            if let Some(p) = keywords::alt_token(&word) {
                out.push(Token { kind: TK::Punct, text: p.to_string(), span: sp, lit: None });
            } else {
                out.push(Token { kind: TK::Ident, text: word, span: sp, lit: None });
            }
            return i;
        }
        if lexical::digit(c) || (c == '.' && i + 1 < n && lexical::digit(b[i + 1])) {
            let st = i;
            while i < n {
                let ch = b[i];
                if (ch == '+' || ch == '-') && i > st && matches!(b[i - 1], 'e' | 'E' | 'p' | 'P') {
                    // exponent sign; for hex literals e/E are digits, but
                    // `0x1e+1` is still one pp-number.
                    i += 1;
                    continue;
                }
                if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                    i += 1;
                    continue;
                }
                if ch == '\'' && i + 1 < n && (b[i + 1].is_alphanumeric() || b[i + 1] == '_') {
                    i += 1;
                    continue;
                }
                break;
            }
            let text: String = b[st..i].iter().collect();
            out.push(lexical::number(&text, sp, self.d));
            return i;
        }
        if c == '"' || c == '\'' {
            return lexical::quoted(b, i, Prefix::None, sp, out, self.d);
        }
        // Punctuators. `<::` is not `<:` `:` (no digraphs). `[[` stays two
        // tokens, and the parser reads an attribute from them.
        for p in PUNCTS {
            let pc: Vec<char> = p.chars().collect();
            if i + pc.len() <= n && b[i..i + pc.len()] == pc[..] {
                out.push(Token { kind: TK::Punct, text: p.to_string(), span: sp, lit: None });
                return i + pc.len();
            }
        }
        self.d.error(sp, None, format!("unexpected character '{}'", c));
        i + 1
    }
}

/// Only white space comes before position `i` on its line.
fn line_start(b: &[char], i: usize) -> bool {
    let mut k = i;
    while k > 0 {
        k -= 1;
        match b[k] {
            '\n' => return true,
            ' ' | '\t' => continue,
            _ => return false,
        }
    }
    true
}

/// The value of the expression of an `#if` or `#elif` line, with object-like
/// names replaced and `defined` applied. Preprocessing beyond the include
/// slice is outside the specification; this covers integer expressions only.
fn eval_if(text: &str, pp: &Pp) -> Option<i128> {
    let mut toks: Vec<String> = Vec::new();
    let b: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c.is_whitespace() {
            i += 1;
        } else if c.is_alphanumeric() || c == '_' {
            let st = i;
            while i < b.len() && (b[i].is_alphanumeric() || b[i] == '_') {
                i += 1;
            }
            toks.push(b[st..i].iter().collect());
        } else {
            let two: String = b[i..(i + 2).min(b.len())].iter().collect();
            if ["&&", "||", "==", "!=", "<=", ">=", "<<", ">>"].contains(&two.as_str()) {
                toks.push(two);
                i += 2;
            } else {
                toks.push(c.to_string());
                i += 1;
            }
        }
    }
    // `defined`, then replacement of names.
    let mut out: Vec<String> = Vec::new();
    let mut k = 0;
    while k < toks.len() {
        let t = &toks[k];
        if t == "defined" {
            let (name, next) = if toks.get(k + 1).is_some_and(|x| x == "(") { (toks.get(k + 2)?.clone(), k + 4) } else { (toks.get(k + 1)?.clone(), k + 2) };
            out.push(if pp.defines.contains_key(&name) { "1".into() } else { "0".into() });
            k = next;
            continue;
        }
        if t.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_') {
            match pp.defines.get(t) {
                Some(v) if !v.is_empty() => {
                    let x = eval_if(v, pp)?;
                    out.push(x.to_string());
                }
                _ => out.push("0".into()),
            }
        } else {
            out.push(t.clone());
        }
        k += 1;
    }
    fn prec(op: &str) -> Option<u8> {
        Some(match op {
            "||" => 1,
            "&&" => 2,
            "|" => 3,
            "^" => 4,
            "&" => 5,
            "==" | "!=" => 6,
            "<" | ">" | "<=" | ">=" => 7,
            "<<" | ">>" => 8,
            "+" | "-" => 9,
            "*" | "/" | "%" => 10,
            _ => return None,
        })
    }
    fn unary(t: &[String], k: &mut usize) -> Option<i128> {
        let x = t.get(*k)?.clone();
        *k += 1;
        match x.as_str() {
            "!" => Some((unary(t, k)? == 0) as i128),
            "-" => Some(-unary(t, k)?),
            "+" => unary(t, k),
            "~" => Some(!unary(t, k)?),
            "(" => {
                let v = binary(t, k, 0)?;
                if t.get(*k)? != ")" {
                    return None;
                }
                *k += 1;
                Some(v)
            }
            _ => {
                let d = x.trim_end_matches(|c: char| matches!(c, 'u' | 'U' | 'l' | 'L'));
                if let Some(h) = d.strip_prefix("0x").or_else(|| d.strip_prefix("0X")) {
                    i128::from_str_radix(h, 16).ok()
                } else {
                    d.parse().ok()
                }
            }
        }
    }
    fn binary(t: &[String], k: &mut usize, min: u8) -> Option<i128> {
        let mut l = unary(t, k)?;
        while let Some(op) = t.get(*k) {
            let Some(p) = prec(op) else { break };
            if p < min {
                break;
            }
            let op = op.clone();
            *k += 1;
            let r = binary(t, k, p + 1)?;
            l = match op.as_str() {
                "||" => ((l != 0) || (r != 0)) as i128,
                "&&" => ((l != 0) && (r != 0)) as i128,
                "|" => l | r,
                "^" => l ^ r,
                "&" => l & r,
                "==" => (l == r) as i128,
                "!=" => (l != r) as i128,
                "<" => (l < r) as i128,
                ">" => (l > r) as i128,
                "<=" => (l <= r) as i128,
                ">=" => (l >= r) as i128,
                "<<" => l.checked_shl(r as u32)?,
                ">>" => l.checked_shr(r as u32)?,
                "+" => l.checked_add(r)?,
                "-" => l.checked_sub(r)?,
                "*" => l.checked_mul(r)?,
                "/" => l.checked_div(r)?,
                _ => l.checked_rem(r)?,
            };
        }
        Some(l)
    }
    let mut k = 0;
    let v = binary(&out, &mut k, 0)?;
    (k == out.len()).then_some(v)
}

#[cfg(test)]
mod tests;
