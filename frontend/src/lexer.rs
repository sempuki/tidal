// Lexer. Tidal leaves lexical structure out of scope (charter), so most of
// this file comes from general knowledge. The literal checks cite the
// PRIM rules, which do describe literal spellings.

use crate::diag::Diags;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TK {
    Ident,
    Punct,
    Int,
    Float,
    Char,
    Str,
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prefix {
    None,
    U8,
    U16,
    U32,
    Wide,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISize {
    None,
    L,
    LL,
    Z,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub line: u32,
    pub lit: Option<Lit>,
}

const PUNCTS: &[&str] = &[
    "<=>", "->*", "...", "<<=", ">>=", "::", "->", ".*", "++", "--", "<<", ">>", "<=", ">=", "==", "!=",
    "&&", "||", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "##", "[[", "{", "}", "[", "]", "(", ")",
    ";", ":", "?", ".", "+", "-", "*", "/", "%", "^", "&", "|", "~", "!", "=", "<", ">", ",", "#",
];

pub fn alt_token(s: &str) -> Option<&'static str> {
    // LOG-13: alternative spellings of operators.
    Some(match s {
        "and" => "&&",
        "or" => "||",
        "not" => "!",
        "not_eq" => "!=",
        "bitand" => "&",
        "bitor" => "|",
        "xor" => "^",
        "compl" => "~",
        "and_eq" => "&=",
        "or_eq" => "|=",
        "xor_eq" => "^=",
        _ => return None,
    })
}

/// State of the `#include` slice (INC-1 to INC-5): the guard names that
/// `#define` has defined and the files that `#pragma once` closed.
#[derive(Default)]
pub struct Pp {
    /// Names that `#define` has defined, with their replacement text.
    defines: std::collections::HashMap<String, String>,
    once: std::collections::HashSet<std::path::PathBuf>,
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

/// The value of the expression of an `#if` or `#elif` line, with object-like
/// names replaced and `defined` applied. Preprocessing beyond the include
/// slice is outside the specification; this covers integer expressions only.
fn eval_if(text: &str, pp: &Pp) -> Option<i128> {
    // Tokens: numbers, names, and operators.
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

pub fn lex(src: &str, d: &mut Diags) -> Vec<Token> {
    lex_file(src, None, d)
}

/// Lex a translation unit whose main file is `path`. `#include "file"`
/// lines are replaced by the tokens of the file (INC-1). The tokens of a
/// header carry the line of the directive.
pub fn lex_file(src: &str, path: Option<&std::path::Path>, d: &mut Diags) -> Vec<Token> {
    let mut pp = Pp::default();
    if let Some(p) = path {
        if src_has_pragma_once(src) {
            pp.once.insert(p.canonicalize().unwrap_or(p.to_path_buf()));
        }
    }
    let mut out = lex_in(src, path, d, &mut pp);
    let line = out.last().map_or(1, |t| t.line);
    out.push(Token { kind: TK::Eof, text: String::new(), line, lit: None });
    out
}

/// INC-5: a `pragma_once` line anywhere in the file.
fn src_has_pragma_once(src: &str) -> bool {
    src.lines().map(str::trim).any(|l| {
        let w: Vec<&str> = l.trim_start_matches('#').split_whitespace().take_while(|w| !w.starts_with("//")).collect();
        l.starts_with('#') && w == ["pragma", "once"]
    })
}

/// Handle one directive line (without the `#`). Returns the tokens it
/// adds, and sets `*skip` when an `#ifndef` names a defined guard.
fn directive(text: &str, line: u32, path: Option<&std::path::Path>, d: &mut Diags, pp: &mut Pp) -> Vec<Token> {
    let text = text.split("//").next().unwrap_or("").trim();
    let (word, rest) = match text.find(|c: char| !(c.is_alphanumeric() || c == '_')) {
        Some(k) => (&text[..k], text[k..].trim()),
        None => (text, ""),
    };
    match word {
        "include" => {
            if !(rest.starts_with('"') && rest.len() >= 2 && rest[1..].contains('"')) {
                d.unsupported(line, "#include form other than \"file\" (outside the specification)");
                return vec![];
            }
            let name = &rest[1..1 + rest[1..].find('"').unwrap()];
            let Some(base) = path else {
                d.unsupported(line, "#include without a file name for the unit");
                return vec![];
            };
            // INC-2: the directory of the file that holds the directive.
            let file = base.parent().unwrap_or(std::path::Path::new(".")).join(name);
            let Ok(src) = std::fs::read_to_string(&file) else {
                d.error(line, Some("INC-3"), format!("no file matches the header name \"{}\"", name));
                return vec![];
            };
            let canon = file.canonicalize().unwrap_or(file.clone());
            if pp.once.contains(&canon) {
                return vec![]; // INC-5
            }
            if src_has_pragma_once(&src) {
                pp.once.insert(canon);
            }
            if pp.depth > 64 {
                d.unsupported(line, "#include nested too deeply");
                return vec![];
            }
            pp.depth += 1;
            let mut toks = lex_in(&src, Some(&file), d, pp);
            pp.depth -= 1;
            for t in &mut toks {
                t.line = line;
            }
            toks
        }
        "pragma" if rest == "once" => vec![],
        "define" => {
            let k = rest.find(|c: char| !(c.is_alphanumeric() || c == '_')).unwrap_or(rest.len());
            let (name, value) = (&rest[..k], &rest[k..]);
            if name.is_empty() || value.starts_with('(') {
                d.unsupported(line, "#define of a function-like macro (outside the specification)");
                return vec![];
            }
            // INC-4: a guard_value does not change the guard.
            pp.defines.insert(name.to_string(), value.trim().to_string());
            vec![]
        }
        "undef" => {
            pp.defines.remove(rest);
            vec![]
        }
        _ => {
            d.unsupported(line, "preprocessor directive (outside the specification)");
            vec![]
        }
    }
}

fn lex_in(src: &str, path: Option<&std::path::Path>, d: &mut Diags, pp: &mut Pp) -> Vec<Token> {
    let mut conds: Vec<Cond> = Vec::new();
    let b: Vec<char> = src.chars().collect();
    let mut i = 0;
    let mut line = 1u32;
    let mut out = Vec::new();
    let n = b.len();
    while i < n {
        let c = b[i];
        if c == '\n' {
            line += 1;
            i += 1;
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
                }
                i += 1;
            }
            i += 2;
            continue;
        }
        let is_directive = c == '#' && out.last().map_or(true, |t: &Token| t.line != line) && line_start(&b, i);
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
            let text: String = b[st..i].iter().collect();
            let text = text.split("//").next().unwrap_or("").trim().to_string();
            let (word, rest) = match text.find(|c: char| !(c.is_alphanumeric() || c == '_')) {
                Some(k) => (text[..k].to_string(), text[k..].trim().to_string()),
                None => (text.clone(), String::new()),
            };
            let outer = conds.iter().all(|c| c.active);
            // The conditional groups of the ODR page's grammar. INC-4: a
            // later inclusion of a guarded file adds no text.
            match word.as_str() {
                "ifndef" | "ifdef" | "if" => {
                    let v = match word.as_str() {
                        "ifndef" => Some(!pp.defines.contains_key(&rest)),
                        "ifdef" => Some(pp.defines.contains_key(&rest)),
                        _ if !outer => Some(false),
                        _ => eval_if(&rest, pp).map(|x| x != 0),
                    };
                    let v = v.unwrap_or_else(|| {
                        d.unsupported(line, "#if expression (outside the specification)");
                        false
                    });
                    conds.push(Cond { active: outer && v, taken: v, outer });
                }
                "elif" | "elifdef" | "elifndef" | "else" => {
                    let Some(top) = conds.last() else {
                        d.unsupported(line, "#else without #if (outside the specification)");
                        continue;
                    };
                    let outer = top.outer;
                    let taken = top.taken;
                    let v = match word.as_str() {
                        "else" => Some(true),
                        "elifdef" => Some(pp.defines.contains_key(&rest)),
                        "elifndef" => Some(!pp.defines.contains_key(&rest)),
                        _ if taken || !outer => Some(false),
                        _ => eval_if(&rest, pp).map(|x| x != 0),
                    };
                    let v = v.unwrap_or_else(|| {
                        d.unsupported(line, "#elif expression (outside the specification)");
                        false
                    });
                    let top = conds.last_mut().unwrap();
                    top.active = outer && !taken && v;
                    top.taken = taken || v;
                }
                "endif" => {
                    if conds.pop().is_none() {
                        d.unsupported(line, "#endif without #if (outside the specification)");
                    }
                }
                _ if outer => {
                    let toks = directive(&text, line, path, d, pp);
                    out.extend(toks);
                }
                _ => {}
            }
            continue;
        }
        // Identifiers, keywords, and prefixed literals.
        if c.is_alphabetic() || c == '_' || (c as u32) > 127 {
            let st = i;
            while i < n && (b[i].is_alphanumeric() || b[i] == '_' || (b[i] as u32) > 127) {
                i += 1;
            }
            let word: String = b[st..i].iter().collect();
            if i < n && (b[i] == '"' || b[i] == '\'') {
                let (prefix, raw) = match word.as_str() {
                    "u8" => (Some(Prefix::U8), false),
                    "u" => (Some(Prefix::U16), false),
                    "U" => (Some(Prefix::U32), false),
                    "L" => (Some(Prefix::Wide), false),
                    "R" => (Some(Prefix::None), true),
                    "u8R" => (Some(Prefix::U8), true),
                    "uR" => (Some(Prefix::U16), true),
                    "UR" => (Some(Prefix::U32), true),
                    "LR" => (Some(Prefix::Wide), true),
                    _ => (None, false),
                };
                if let Some(p) = prefix {
                    if raw && b[i] == '"' {
                        i = lex_raw_string(&b, i, p, line, &mut out, d, &mut line.clone());
                        // count newlines inside the raw string
                        let txt = &out.last().unwrap().text;
                        line += txt.matches('\n').count() as u32;
                        continue;
                    } else if !raw {
                        i = lex_quoted(&b, i, p, line, &mut out, d);
                        continue;
                    }
                }
            }
            if let Some(v) = pp.defines.get(&word).filter(|v| !v.is_empty()).cloned() {
                // An object-like name from `#define` with a value
                // (outside the specification).
                if pp.expanding < 16 {
                    pp.expanding += 1;
                    let mut toks = lex_in(&v, None, d, pp);
                    pp.expanding -= 1;
                    for t in &mut toks {
                        t.line = line;
                    }
                    out.extend(toks);
                    continue;
                }
            }
            if let Some(p) = alt_token(&word) {
                out.push(Token { kind: TK::Punct, text: p.to_string(), line, lit: None });
            } else {
                out.push(Token { kind: TK::Ident, text: word, line, lit: None });
            }
            continue;
        }
        if c.is_ascii_digit() || (c == '.' && i + 1 < n && b[i + 1].is_ascii_digit()) {
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
            out.push(lex_number(&text, line, d));
            continue;
        }
        if c == '"' || c == '\'' {
            i = lex_quoted(&b, i, Prefix::None, line, &mut out, d);
            continue;
        }
        // Punctuators. `<::` is not `<:` `:` (no digraphs supported).
        let mut matched = false;
        for p in PUNCTS {
            let pc: Vec<char> = p.chars().collect();
            if i + pc.len() <= n && b[i..i + pc.len()] == pc[..] {
                // `[[` only as an attribute start; keep it as two tokens and let
                // the parser decide.
                if *p == "[[" {
                    continue;
                }
                out.push(Token { kind: TK::Punct, text: p.to_string(), line, lit: None });
                i += pc.len();
                matched = true;
                break;
            }
        }
        if !matched {
            d.error(line, None, format!("unexpected character '{}'", c));
            i += 1;
        }
    }
    out
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

fn lex_raw_string(
    b: &[char],
    mut i: usize,
    prefix: Prefix,
    line: u32,
    out: &mut Vec<Token>,
    d: &mut Diags,
    _l: &mut u32,
) -> usize {
    // b[i] == '"'
    i += 1;
    let st = i;
    while i < b.len() && b[i] != '(' {
        let c = b[i];
        // PRIM-19: delimiter characters.
        if c == ' ' || c == ')' || c == '\\' || c.is_control() || c == '"' {
            d.error(line, Some("PRIM-19"), "invalid character in raw string delimiter");
            break;
        }
        i += 1;
    }
    let delim: String = b[st..i].iter().collect();
    if delim.chars().count() > 16 {
        d.error(line, Some("PRIM-19"), "raw string delimiter longer than 16 characters");
    }
    i += 1;
    let body_start = i;
    let close: Vec<char> = format!("){}\"", delim).chars().collect();
    while i < b.len() && !(i + close.len() <= b.len() && b[i..i + close.len()] == close[..]) {
        i += 1;
    }
    let body: String = b[body_start..i.min(b.len())].iter().collect();
    i += close.len();
    let units = encode(&body.chars().map(|c| c as u32).collect::<Vec<_>>(), prefix);
    let text = body.clone();
    out.push(Token { kind: TK::Str, text, line, lit: Some(Lit::Str { prefix, units }) });
    i
}

/// Encode code points as code units of the given prefix.
fn encode(cps: &[u32], prefix: Prefix) -> Vec<u32> {
    let mut v = Vec::new();
    for &cp in cps {
        match prefix {
            Prefix::None | Prefix::U8 => {
                if let Some(ch) = char::from_u32(cp) {
                    let mut buf = [0u8; 4];
                    for byte in ch.encode_utf8(&mut buf).bytes() {
                        v.push(byte as u32);
                    }
                } else {
                    v.push(cp);
                }
            }
            Prefix::U16 => {
                if let Some(ch) = char::from_u32(cp) {
                    let mut buf = [0u16; 2];
                    for u in ch.encode_utf16(&mut buf) {
                        v.push(*u as u32);
                    }
                } else {
                    v.push(cp);
                }
            }
            Prefix::U32 | Prefix::Wide => v.push(cp),
        }
    }
    v
}

/// An element of a quoted literal: either a source character (to be
/// encoded) or a numeric escape (one code unit).
enum Elem {
    Cp(u32),
    Unit(u32),
}

fn lex_quoted(b: &[char], mut i: usize, prefix: Prefix, line: u32, out: &mut Vec<Token>, d: &mut Diags) -> usize {
    let q = b[i];
    i += 1;
    let mut elems = Vec::new();
    let mut bad = false;
    let mut named = false;
    while i < b.len() && b[i] != q && b[i] != '\n' {
        if b[i] == '\\' && i + 1 < b.len() {
            i += 1;
            let e = b[i];
            i += 1;
            let simple = match e {
                'n' => Some(10),
                't' => Some(9),
                'v' => Some(11),
                'b' => Some(8),
                'r' => Some(13),
                'f' => Some(12),
                'a' => Some(7),
                '\\' => Some(92),
                '?' => Some(63),
                '\'' => Some(39),
                '"' => Some(34),
                _ => None,
            };
            if let Some(v) = simple {
                elems.push(Elem::Cp(v));
                continue;
            }
            match e {
                '0'..='7' => {
                    let mut v = e.to_digit(8).unwrap();
                    let mut k = 1;
                    while k < 3 && i < b.len() && b[i].is_digit(8) {
                        v = v * 8 + b[i].to_digit(8).unwrap();
                        i += 1;
                        k += 1;
                    }
                    elems.push(Elem::Unit(v));
                }
                'o' | 'x' => {
                    let radix = if e == 'o' { 8 } else { 16 };
                    let braced = i < b.len() && b[i] == '{';
                    if braced {
                        i += 1;
                    }
                    let mut v: u64 = 0;
                    let mut nd = 0;
                    let mut overflow = false;
                    while i < b.len() && b[i].is_digit(radix) {
                        v = v.checked_mul(radix as u64).and_then(|x| x.checked_add(b[i].to_digit(radix).unwrap() as u64)).unwrap_or_else(|| {
                            overflow = true;
                            u64::MAX
                        });
                        i += 1;
                        nd += 1;
                    }
                    if braced {
                        if i < b.len() && b[i] == '}' {
                            i += 1;
                        } else {
                            bad = true;
                        }
                    }
                    if nd == 0 || (e == 'o' && !braced) {
                        // PRIM-18: `\x` with no digits.
                        d.error(line, Some("PRIM-18"), "escape sequence with no digits");
                        bad = true;
                    }
                    if overflow || v > u32::MAX as u64 {
                        d.error(line, Some("PRIM-18"), "escape sequence out of range");
                        bad = true;
                    }
                    elems.push(Elem::Unit(v.min(u32::MAX as u64) as u32));
                }
                'u' | 'U' => {
                    let braced = e == 'u' && i < b.len() && b[i] == '{';
                    let mut v: u32 = 0;
                    let mut nd = 0;
                    if braced {
                        i += 1;
                        while i < b.len() && b[i].is_ascii_hexdigit() {
                            v = v.saturating_mul(16).saturating_add(b[i].to_digit(16).unwrap());
                            i += 1;
                            nd += 1;
                        }
                        if i < b.len() && b[i] == '}' {
                            i += 1;
                        }
                    } else {
                        let want = if e == 'u' { 4 } else { 8 };
                        while nd < want && i < b.len() && b[i].is_ascii_hexdigit() {
                            v = v * 16 + b[i].to_digit(16).unwrap();
                            i += 1;
                            nd += 1;
                        }
                        if nd != want {
                            d.error(line, Some("PRIM-18"), "incomplete universal character name");
                            bad = true;
                        }
                    }
                    if (0xD800..=0xDFFF).contains(&v) || v > 0x10FFFF {
                        d.error(line, Some("PRIM-18"), "universal character name names a surrogate or is out of range");
                        bad = true;
                    }
                    elems.push(Elem::Cp(v));
                }
                'N' => {
                    // PRIM-5 named escapes need the Unicode name table, which
                    // this front end does not have.
                    d.unsupported(line, "named escape \\N{...} (needs Unicode name table)");
                    bad = true;
                    named = true;
                    while i < b.len() && b[i] != '}' && b[i] != q {
                        i += 1;
                    }
                    if i < b.len() && b[i] == '}' {
                        i += 1;
                    }
                    elems.push(Elem::Cp('?' as u32));
                }
                _ => {
                    d.error(line, Some("PRIM-18"), format!("unknown escape sequence '\\{}'", e));
                    bad = true;
                    elems.push(Elem::Cp(e as u32));
                }
            }
        } else {
            elems.push(Elem::Cp(b[i] as u32));
            i += 1;
        }
    }
    if i >= b.len() || b[i] != q {
        // TR-12: an open literal derives from no production.
        d.error(line, Some("TR-12"), "unterminated literal");
    }
    // A literal still open at the end of the file ends there (G180).
    i = (i + 1).min(b.len());
    // user-defined literal suffix
    let mut udl = false;
    let suf_start = i;
    if i < b.len() && (b[i].is_alphabetic() || b[i] == '_') {
        while i < b.len() && (b[i].is_alphanumeric() || b[i] == '_') {
            i += 1;
        }
        udl = true;
    }
    let suffix: String = b[suf_start..i].iter().collect();
    if named {
        // The unsupported note is out already; the literal stays empty.
        elems.clear();
        if q != '"' {
            elems.push(Elem::Cp('?' as u32));
        }
    }
    let first = out.len();
    let end = lex_quoted_value(elems, q, i, prefix, line, bad, out, d);
    if udl && out.len() == first + 1 {
        let t = out.pop().unwrap();
        let base = Box::new(t.lit.unwrap_or(Lit::Str { prefix, units: vec![] }));
        out.push(Token { kind: t.kind, text: suffix, line, lit: Some(Lit::UserDefined { suffix: String::new(), base, raw: String::new() }) });
        let n = out.len() - 1;
        let sfx = out[n].text.clone();
        if let Some(Lit::UserDefined { suffix: s, .. }) = &mut out[n].lit {
            *s = sfx;
        }
    }
    end
}

#[allow(clippy::too_many_arguments)]
fn lex_quoted_value(elems: Vec<Elem>, q: char, i: usize, prefix: Prefix, line: u32, mut bad: bool, out: &mut Vec<Token>, d: &mut Diags) -> usize {
    if q == '"' {
        let mut units = Vec::new();
        for e in &elems {
            match e {
                Elem::Cp(c) => units.extend(encode(&[*c], prefix)),
                Elem::Unit(u) => units.push(*u),
            }
        }
        out.push(Token { kind: TK::Str, text: String::new(), line, lit: Some(Lit::Str { prefix, units }) });
        return i;
    }
    // Character literal: PRIM-5, PRIM-6, PRIM-18, PRIM-23.
    let unit_max: u64 = match prefix {
        Prefix::None | Prefix::U8 => 0xFF,
        Prefix::U16 => 0xFFFF,
        Prefix::U32 | Prefix::Wide => 0xFFFF_FFFF,
    };
    if elems.is_empty() {
        d.error(line, Some("PRIM-18"), "empty character literal");
        bad = true;
    }
    let mut value: i128 = 0;
    let mut multi = false;
    if !bad {
        if elems.len() > 1 {
            if prefix != Prefix::None {
                d.error(line, Some("PRIM-18"), "more than one character in a prefixed character literal");
            } else {
                // PRIM-6: implementation-defined; x86-64 Linux packs bytes.
                multi = true;
                let mut v: u32 = 0;
                for e in &elems {
                    let u = match e {
                        Elem::Cp(c) | Elem::Unit(c) => *c,
                    };
                    v = (v << 8) | (u & 0xFF);
                }
                value = v as i32 as i128;
            }
        } else {
            match &elems[0] {
                Elem::Unit(u) => {
                    if (*u as u64) > unit_max {
                        d.error(line, Some("PRIM-18"), "escape sequence too large for the character type");
                    }
                    value = *u as i128;
                }
                Elem::Cp(c) => {
                    let units = encode(&[*c], prefix);
                    if units.len() != 1 || (units[0] as u64) > unit_max {
                        if prefix == Prefix::None {
                            // PRIM-23: conditionally supported, no
                            // implementation in the matrix accepts it.
                            d.error(line, Some("PRIM-23"), "character does not fit in one code unit");
                        } else {
                            d.error(line, Some("PRIM-18"), "character does not fit in one code unit of its type");
                        }
                    }
                    value = units[0] as i128;
                }
            }
            if prefix == Prefix::None && value > 127 {
                // PRIM-6: plain char is signed on the matrix targets.
                value -= 256;
            }
        }
    }
    out.push(Token { kind: TK::Char, text: String::new(), line, lit: Some(Lit::Char { prefix, value, multi }) });
    i
}

/// The numeric part of a literal with a user-defined suffix. OPR-8: the
/// raw form and the template take the characters, so a value that no
/// integer type holds is an error (PRIM-2) only for the numeric form. Such
/// a value is kept as `u128::MAX`, which `literal_call` checks.
fn lex_udl_base(num: &str, line: u32, d: &mut Diags) -> (TK, Lit) {
    let mut inner = Diags::default();
    let base = lex_number(num, line, &mut inner);
    let mut too_large = false;
    for x in inner.list {
        if x.rule == Some("PRIM-2") && x.msg == "integer literal too large for any type" {
            too_large = true;
        } else {
            d.list.push(x);
        }
    }
    let mut lit = base.lit.unwrap_or(Lit::Int { value: 0, decimal: true, unsigned: false, size: ISize::None });
    if too_large {
        if let Lit::Int { value, .. } = &mut lit {
            *value = u128::MAX;
        }
    }
    (base.kind, lit)
}

fn lex_number(text: &str, line: u32, d: &mut Diags) -> Token {
    let lower = text.to_ascii_lowercase();
    let is_hex = lower.starts_with("0x");
    let is_float = if is_hex {
        lower.contains('.') || lower.contains('p')
    } else {
        lower.contains('.') || (lower.contains('e') && !lower.starts_with("0b"))
    };
    if let Some(k) = text.find('_') {
        if k > 0 && !lower.starts_with("0x") || k > 2 {
            // OPR-8: a suffix that starts with `_` is a user-defined suffix.
            let num = &text[..k];
            let (kind, lit) = lex_udl_base(num, line, d);
            return Token { kind, text: text.to_string(), line, lit: Some(Lit::UserDefined { suffix: text[k..].to_string(), base: Box::new(lit), raw: num.to_string() }) };
        }
    }
    let tok = |lit: Lit| Token { kind: if matches!(lit, Lit::Float { .. }) { TK::Float } else { TK::Int }, text: text.to_string(), line, lit: Some(lit) };
    if text.matches('.').count() > 1 {
        // PRIM-26: the literal runs over each `.`, and the run is not one
        // valid literal.
        d.error(line, Some("PRIM-26"), format!("'{}' is not one valid numeric literal", text));
        return tok(Lit::Float { value: 0.0, suffix: FSuffix::None });
    }
    // Digit separators: PRIM-16.
    let chars: Vec<char> = text.chars().collect();
    for (k, &c) in chars.iter().enumerate() {
        if c == '\'' {
            let ok = k > 0 && k + 1 < chars.len() && chars[k - 1].is_ascii_hexdigit() && chars[k + 1].is_ascii_hexdigit();
            if !ok {
                d.error(line, Some("PRIM-16"), "digit separator must be between two digits");
            }
        }
    }
    let clean: String = text.chars().filter(|&c| c != '\'').collect();
    if is_float {
        return tok(lex_float(&clean, is_hex, line, d));
    }
    let (base, body_start) = if is_hex {
        (16, 2)
    } else if lower.starts_with("0b") {
        (2, 2)
    } else if clean.len() > 1 && clean.starts_with('0') {
        (8, 1)
    } else {
        (10, 0)
    };
    let cb: Vec<char> = clean.chars().collect();
    let mut k = body_start;
    let mut value: u128 = 0;
    let mut overflow = false;
    let mut ndig = 0;
    while k < cb.len() && cb[k].is_ascii_alphanumeric() {
        let c = cb[k];
        let dv = c.to_digit(16);
        let is_digit = match base {
            16 => c.is_ascii_hexdigit(),
            _ => c.is_ascii_digit(),
        };
        if !is_digit {
            break;
        }
        let dv = dv.unwrap();
        if dv >= base {
            d.error(line, Some("PRIM-16"), format!("invalid digit '{}' in a base-{} literal", c, base));
            return tok(Lit::Int { value: 0, decimal: base == 10, unsigned: false, size: ISize::None });
        }
        match value.checked_mul(base as u128).and_then(|v| v.checked_add(dv as u128)) {
            Some(v) => value = v,
            None => overflow = true,
        }
        k += 1;
        ndig += 1;
    }
    if (base == 16 || base == 2) && ndig == 0 {
        d.error(line, Some("PRIM-16"), "base prefix without digits");
    }
    let suffix: String = cb[k..].iter().collect();
    let (unsigned, size) = match parse_int_suffix(&suffix) {
        Some(s) => s,
        // OPR-8, OPR-9 (Q-0383): a suffix that is not a form of the syntax
        // names a literal operator, also without `_`.
        None if suffix.starts_with(|c: char| c.is_ascii_alphabetic()) && suffix.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') => {
            let num = &text[..text.len() - suffix.len()];
            let (_, lit) = lex_udl_base(num, line, d);
            return Token { kind: TK::Int, text: text.to_string(), line, lit: Some(Lit::UserDefined { suffix: suffix.clone(), base: Box::new(lit), raw: num.to_string() }) };
        }
        None => {
            d.error(line, Some("PRIM-16"), format!("invalid integer suffix '{}'", suffix));
            (false, ISize::None)
        }
    };
    if overflow || value > u64::MAX as u128 {
        d.error(line, Some("PRIM-2"), "integer literal too large for any type");
        value = 0;
    }
    tok(Lit::Int { value, decimal: base == 10, unsigned, size })
}

fn parse_int_suffix(s: &str) -> Option<(bool, ISize)> {
    let size_of = |t: &str| -> Option<ISize> {
        Some(match t {
            "" => ISize::None,
            "l" | "L" => ISize::L,
            "ll" | "LL" => ISize::LL,
            "z" | "Z" => ISize::Z,
            _ => return None,
        })
    };
    if s.is_empty() {
        return Some((false, ISize::None));
    }
    if let Some(r) = s.strip_prefix(['u', 'U']) {
        return size_of(r).map(|z| (true, z));
    }
    if let Some(r) = s.strip_suffix(['u', 'U']) {
        if r.is_empty() {
            return None;
        }
        return size_of(r).map(|z| (true, z));
    }
    size_of(s).map(|z| (false, z))
}

fn lex_float(s: &str, hex: bool, line: u32, d: &mut Diags) -> Lit {
    let lower = s.to_ascii_lowercase();
    let bytes: Vec<char> = lower.chars().collect();
    // Find the end of the numeric part.
    let mut k = if hex { 2 } else { 0 };
    let mut mant = String::new();
    while k < bytes.len() && (bytes[k] == '.' || if hex { bytes[k].is_ascii_hexdigit() } else { bytes[k].is_ascii_digit() }) {
        mant.push(bytes[k]);
        k += 1;
    }
    let mut exp: i64 = 0;
    let mut has_exp = false;
    if k < bytes.len() && ((hex && bytes[k] == 'p') || (!hex && bytes[k] == 'e')) {
        has_exp = true;
        k += 1;
        let mut neg = false;
        if k < bytes.len() && (bytes[k] == '+' || bytes[k] == '-') {
            neg = bytes[k] == '-';
            k += 1;
        }
        let st = k;
        while k < bytes.len() && bytes[k].is_ascii_digit() {
            exp = exp.saturating_mul(10).saturating_add(bytes[k].to_digit(10).unwrap() as i64);
            k += 1;
        }
        if st == k {
            d.error(line, Some("PRIM-17"), "exponent has no digits");
        }
        if neg {
            exp = -exp;
        }
    }
    if hex && !has_exp {
        d.error(line, Some("PRIM-17"), "hexadecimal floating literal needs an exponent");
    }
    let suffix = &s[k.min(s.len())..];
    let suf = match suffix {
        "" => FSuffix::None,
        "f" | "F" => FSuffix::F,
        "l" | "L" => FSuffix::L,
        "f16" | "F16" | "f32" | "F32" | "f64" | "F64" | "f128" | "F128" | "bf16" | "BF16" => FSuffix::Ext,
        _ => {
            d.error(line, Some("PRIM-17"), format!("invalid floating literal suffix '{}'", suffix));
            FSuffix::None
        }
    };
    let value = if hex {
        let mut v: f64 = 0.0;
        let mut scale: i64 = 0;
        let mut after_dot = false;
        for c in mant.chars() {
            if c == '.' {
                after_dot = true;
                continue;
            }
            v = v * 16.0 + c.to_digit(16).unwrap() as f64;
            if after_dot {
                scale -= 4;
            }
        }
        v * 2f64.powi((exp + scale) as i32)
    } else {
        let txt = format!("{}e{}", if mant.starts_with('.') { format!("0{}", mant) } else { mant.clone() }, exp);
        if suf == FSuffix::F {
            txt.parse::<f32>().map(|x| x as f64).unwrap_or(0.0)
        } else {
            txt.parse::<f64>().unwrap_or(0.0)
        }
    };
    Lit::Float { value, suffix: suf }
}
