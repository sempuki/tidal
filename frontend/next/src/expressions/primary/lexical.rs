//! The lexical productions of `expressions/primary`'s Syntax segment, one
//! function each, and the literal checks (PRIM) that `lex` calls. The
//! literal code moved from the old front end's `lexer.rs`. A function for
//! a character class takes a character. A recognizer takes the characters
//! and a start index and gives the index after the form, or `None`.
//! `identifier`, `literal`, and `string_literal` are token-level
//! functions of `syntax.rs`.

use crate::diag::Diags;
use crate::lex::{FSuffix, ISize, Lit, Prefix, Token, TK};
use crate::source::Span;

// ---------------------------------------------------------------------
// Character classes

/// `id_start: letter | "_" | ucn | a character outside the basic
/// character set that Unicode lets start an identifier`. The lexer takes
/// any character above U+007F here, as the old front end did.
pub fn id_start(c: char) -> bool {
    letter(c) || c == '_' || (c as u32) > 127
}

/// `letter`: one of the 26 Latin letters, in either case.
pub fn letter(c: char) -> bool {
    c.is_ascii_alphabetic()
}

/// `digit: "0" | nonzero_digit`.
pub fn digit(c: char) -> bool {
    c == '0' || nonzero_digit(c)
}

/// `nonzero_digit: nonzero_octal | "8" | "9"`.
pub fn nonzero_digit(c: char) -> bool {
    nonzero_octal(c) || c == '8' || c == '9'
}

/// `octal_digit: "0" | nonzero_octal`.
pub fn octal_digit(c: char) -> bool {
    c == '0' || nonzero_octal(c)
}

/// `nonzero_octal: "1" | ... | "7"`.
pub fn nonzero_octal(c: char) -> bool {
    ('1'..='7').contains(&c)
}

/// `binary_digit: "0" | "1"`.
pub fn binary_digit(c: char) -> bool {
    c == '0' || c == '1'
}

/// `hex_digit: digit | hex_letter`.
pub fn hex_digit(c: char) -> bool {
    digit(c) || hex_letter(c)
}

/// `hex_letter`: a to f in either case.
pub fn hex_letter(c: char) -> bool {
    matches!(c, 'a'..='f' | 'A'..='F')
}

/// `d_char`: a basic character other than a space, `(`, `)`, `\`, or a
/// control character.
pub fn d_char(c: char) -> bool {
    c.is_ascii() && !(c == ' ' || c == '(' || c == ')' || c == '\\' || c.is_control())
}

/// `raw_char`: a character, line breaks included, that does not end the
/// raw string. `close` is `)` followed by the delimiter and `"`.
pub fn raw_char(b: &[char], i: usize, close: &[char]) -> bool {
    i < b.len() && !(i + close.len() <= b.len() && b[i..i + close.len()] == close[..])
}

/// `c_char: escape | a character other than "'", "\", or a line break`.
pub fn c_char(c: char) -> bool {
    c != '\'' && c != '\\' && c != '\n'
}

/// `s_char: escape | a character other than '"', "\", or a line break`.
pub fn s_char(c: char) -> bool {
    c != '"' && c != '\\' && c != '\n'
}

/// `simple_escape`: the character after `\` and the code point it means.
pub fn simple_escape(c: char) -> Option<u32> {
    Some(match c {
        'n' => 10,
        't' => 9,
        'v' => 11,
        'b' => 8,
        'r' => 13,
        'f' => 12,
        'a' => 7,
        '\\' => 92,
        '?' => 63,
        '\'' => 39,
        '"' => 34,
        _ => return None,
    })
}

/// `char_prefix: "u8" | "u" | "U" | "L"`.
pub fn char_prefix(w: &str) -> Option<Prefix> {
    Some(match w {
        "u8" => Prefix::U8,
        "u" => Prefix::U16,
        "U" => Prefix::U32,
        "L" => Prefix::Wide,
        _ => return None,
    })
}

// ---------------------------------------------------------------------
// Recognizers

fn many(b: &[char], mut i: usize, f: fn(char) -> bool) -> usize {
    while i < b.len() && f(b[i]) {
        i += 1;
    }
    i
}

/// A digit sequence with optional `'` between two digits of the class.
fn separated(b: &[char], i: usize, first: fn(char) -> bool, rest: fn(char) -> bool) -> Option<usize> {
    if i >= b.len() || !first(b[i]) {
        return None;
    }
    let mut k = i + 1;
    loop {
        if k < b.len() && rest(b[k]) {
            k += 1;
        } else if k + 1 < b.len() && b[k] == '\'' && rest(b[k + 1]) {
            k += 2;
        } else {
            return Some(k);
        }
    }
}

/// `decimal_digits: nonzero_digit {["'"] digit}`.
pub fn decimal_digits(b: &[char], i: usize) -> Option<usize> {
    separated(b, i, nonzero_digit, digit)
}

/// `octal_digits: "0" {["'"] octal_digit}`.
pub fn octal_digits(b: &[char], i: usize) -> Option<usize> {
    separated(b, i, |c| c == '0', octal_digit)
}

fn prefixed(b: &[char], i: usize, lower: char, digits: fn(char) -> bool) -> Option<usize> {
    if i + 1 < b.len() && b[i] == '0' && b[i + 1].to_ascii_lowercase() == lower {
        separated(b, i + 2, digits, digits)
    } else {
        None
    }
}

/// `hex_digits: ("0x" | "0X") hex_digit {["'"] hex_digit}`.
pub fn hex_digits(b: &[char], i: usize) -> Option<usize> {
    prefixed(b, i, 'x', hex_digit)
}

/// `binary_digits: ("0b" | "0B") binary_digit {["'"] binary_digit}`.
pub fn binary_digits(b: &[char], i: usize) -> Option<usize> {
    prefixed(b, i, 'b', binary_digit)
}

/// `integer_digits: decimal_digits | octal_digits | hex_digits | binary_digits`.
pub fn integer_digits(b: &[char], i: usize) -> Option<usize> {
    hex_digits(b, i).or_else(|| binary_digits(b, i)).or_else(|| decimal_digits(b, i)).or_else(|| octal_digits(b, i))
}

/// `digit_seq: digit {["'"] digit}`.
pub fn digit_seq(b: &[char], i: usize) -> Option<usize> {
    separated(b, i, digit, digit)
}

/// `hex_digit_seq: hex_digit {["'"] hex_digit}`.
pub fn hex_digit_seq(b: &[char], i: usize) -> Option<usize> {
    separated(b, i, hex_digit, hex_digit)
}

fn signed_exp(b: &[char], i: usize, letters: [char; 2]) -> Option<usize> {
    if i >= b.len() || !letters.contains(&b[i]) {
        return None;
    }
    let mut k = i + 1;
    if k < b.len() && (b[k] == '+' || b[k] == '-') {
        k += 1;
    }
    digit_seq(b, k)
}

/// `exponent: ("e" | "E") ["+" | "-"] digit_seq`.
pub fn exponent(b: &[char], i: usize) -> Option<usize> {
    signed_exp(b, i, ['e', 'E'])
}

/// `binary_exponent: ("p" | "P") ["+" | "-"] digit_seq`.
pub fn binary_exponent(b: &[char], i: usize) -> Option<usize> {
    signed_exp(b, i, ['p', 'P'])
}

/// `decimal_float`: `digit_seq "." [digit_seq] [exponent]`,
/// `"." digit_seq [exponent]`, or `digit_seq exponent`.
pub fn decimal_float(b: &[char], i: usize) -> Option<usize> {
    if i < b.len() && b[i] == '.' {
        let k = digit_seq(b, i + 1)?;
        return Some(exponent(b, k).unwrap_or(k));
    }
    let k = digit_seq(b, i)?;
    if k < b.len() && b[k] == '.' {
        let k = digit_seq(b, k + 1).unwrap_or(k + 1);
        return Some(exponent(b, k).unwrap_or(k));
    }
    exponent(b, k)
}

/// `hex_float`: a hexadecimal mantissa with an optional point, then a
/// binary exponent.
pub fn hex_float(b: &[char], i: usize) -> Option<usize> {
    if !(i + 1 < b.len() && b[i] == '0' && matches!(b[i + 1], 'x' | 'X')) {
        return None;
    }
    let mut k = i + 2;
    if k < b.len() && b[k] == '.' {
        k = hex_digit_seq(b, k + 1)?;
    } else {
        k = hex_digit_seq(b, k)?;
        if k < b.len() && b[k] == '.' {
            k = hex_digit_seq(b, k + 1).unwrap_or(k + 1);
        }
    }
    binary_exponent(b, k)
}

/// `hex_quad: hex_digit hex_digit hex_digit hex_digit`.
pub fn hex_quad(b: &[char], i: usize) -> Option<usize> {
    (i + 4 <= b.len() && b[i..i + 4].iter().all(|&c| hex_digit(c))).then_some(i + 4)
}

/// `ucn: "\u" hex_quad | "\U" hex_quad hex_quad | "\u{" hex_digit {hex_digit} "}"`.
pub fn ucn(b: &[char], i: usize) -> Option<usize> {
    if i + 1 >= b.len() || b[i] != '\\' {
        return None;
    }
    match b[i + 1] {
        'u' if b.get(i + 2) == Some(&'{') => {
            let k = many(b, i + 3, hex_digit);
            (k > i + 3 && b.get(k) == Some(&'}')).then_some(k + 1)
        }
        'u' => hex_quad(b, i + 2),
        'U' => hex_quad(b, i + 2).and_then(|k| hex_quad(b, k)),
        _ => None,
    }
}

/// `char_name`: the name of a Unicode character. The front end has no
/// name table, so it takes the characters that can form a name, and the
/// lexer reports a named escape as unsupported (PRIM-5).
pub fn char_name(b: &[char], i: usize) -> Option<usize> {
    let k = many(b, i, |c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == ' ' || c == '-');
    (k > i).then_some(k)
}

/// `escape`: `\` and a simple escape, an octal or hexadecimal escape, a
/// universal character name, or a named escape.
pub fn escape(b: &[char], i: usize) -> Option<usize> {
    if i + 1 >= b.len() || b[i] != '\\' {
        return None;
    }
    let c = b[i + 1];
    if simple_escape(c).is_some() {
        return Some(i + 2);
    }
    if octal_digit(c) {
        let mut k = i + 2;
        while k < i + 4 && k < b.len() && octal_digit(b[k]) {
            k += 1;
        }
        return Some(k);
    }
    let braced = |k: usize, f: fn(char) -> bool| {
        let e = many(b, k, f);
        (e > k && b.get(e) == Some(&'}')).then_some(e + 1)
    };
    match c {
        'o' if b.get(i + 2) == Some(&'{') => braced(i + 3, octal_digit),
        'x' if b.get(i + 2) == Some(&'{') => braced(i + 3, hex_digit),
        'x' => {
            let e = many(b, i + 2, hex_digit);
            (e > i + 2).then_some(e)
        }
        'N' if b.get(i + 2) == Some(&'{') => char_name(b, i + 3).filter(|&e| b.get(e) == Some(&'}')).map(|e| e + 1),
        _ => ucn(b, i),
    }
}

/// `delimiter: {d_char}`. Gives the index after it.
pub fn delimiter(b: &[char], i: usize) -> usize {
    many(b, i, d_char)
}

/// `ud_suffix: identifier`, at a literal's end.
pub fn ud_suffix(b: &[char], i: usize) -> Option<usize> {
    if i < b.len() && id_start(b[i]) {
        Some(many(b, i, |c| id_start(c) || digit(c)))
    } else {
        None
    }
}

/// `unsigned_suffix: "u" | "U"`.
pub fn unsigned_suffix(s: &str) -> bool {
    s == "u" || s == "U"
}

/// `length_suffix: "l" | "L" | "ll" | "LL" | "z" | "Z"`.
pub fn length_suffix(t: &str) -> Option<ISize> {
    Some(match t {
        "l" | "L" => ISize::L,
        "ll" | "LL" => ISize::LL,
        "z" | "Z" => ISize::Z,
        _ => return None,
    })
}

/// `integer_suffix: unsigned_suffix [length_suffix] | length_suffix [unsigned_suffix]`.
/// The empty suffix gives `(false, None)` too.
pub fn integer_suffix(s: &str) -> Option<(bool, ISize)> {
    let size_of = |t: &str| if t.is_empty() { Some(ISize::None) } else { length_suffix(t) };
    if s.is_empty() {
        return Some((false, ISize::None));
    }
    if let Some(r) = s.strip_prefix(['u', 'U']) {
        return size_of(r).map(|z| (true, z));
    }
    if let Some(r) = s.strip_suffix(['u', 'U']) {
        debug_assert!(unsigned_suffix(&s[r.len()..]));
        return size_of(r).map(|z| (true, z));
    }
    size_of(s).map(|z| (false, z))
}

/// `float_suffix: "f" | "F" | "l" | "L" | "f16" | ... | "BF16"`.
pub fn float_suffix(s: &str) -> Option<FSuffix> {
    Some(match s {
        "" => FSuffix::None,
        "f" | "F" => FSuffix::F,
        "l" | "L" => FSuffix::L,
        "f16" | "F16" | "f32" | "F32" | "f64" | "F64" | "f128" | "F128" | "bf16" | "BF16" => FSuffix::Ext,
        _ => return None,
    })
}

// ---------------------------------------------------------------------
// Literals and their checks

/// Encode code points as code units of the given prefix.
pub fn encode(cps: &[u32], prefix: Prefix) -> Vec<u32> {
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

/// A raw string literal from the `"` at `i` (PRIM-19). Gives the index
/// after it and its token, whose text is the body.
pub fn raw_string(b: &[char], mut i: usize, prefix: Prefix, sp: Span, d: &mut Diags) -> (usize, Token) {
    i += 1;
    let st = i;
    while i < b.len() && b[i] != '(' {
        let c = b[i];
        // PRIM-19: delimiter characters. The old front end also refuses
        // a double quote here.
        if !d_char(c) || c == '"' {
            d.error(sp, Some("PRIM-19"), "invalid character in raw string delimiter");
            break;
        }
        i += 1;
    }
    let delim: String = b[st..i].iter().collect();
    if delim.chars().count() > 16 {
        d.error(sp, Some("PRIM-19"), "raw string delimiter longer than 16 characters");
    }
    i += 1;
    let body_start = i;
    let close: Vec<char> = format!("){}\"", delim).chars().collect();
    while raw_char(b, i, &close) {
        i += 1;
    }
    let body: String = b[body_start..i.min(b.len())].iter().collect();
    i += close.len();
    let units = encode(&body.chars().map(|c| c as u32).collect::<Vec<_>>(), prefix);
    (i, Token { kind: TK::Str, text: body, span: sp, lit: Some(Lit::Str { prefix, units }) })
}

/// An element of a quoted literal: either a source character (to be
/// encoded) or a numeric escape (one code unit).
pub enum Elem {
    Cp(u32),
    Unit(u32),
}

/// What an escape sequence gave: the element, and whether it was bad or a
/// named escape.
struct Esc {
    elem: Elem,
    bad: bool,
    named: bool,
}

/// The value of the escape after the `\` at `i - 1` (PRIM-18). `i` moves
/// past it. `q` is the quote that ends the literal.
fn escape_value(b: &[char], i: &mut usize, q: char, sp: Span, d: &mut Diags) -> Esc {
    let e = b[*i];
    *i += 1;
    let ok = |elem| Esc { elem, bad: false, named: false };
    if let Some(v) = simple_escape(e) {
        return ok(Elem::Cp(v));
    }
    let mut bad = false;
    match e {
        '0'..='7' => {
            let mut v = e.to_digit(8).unwrap();
            let mut k = 1;
            while k < 3 && *i < b.len() && octal_digit(b[*i]) {
                v = v * 8 + b[*i].to_digit(8).unwrap();
                *i += 1;
                k += 1;
            }
            ok(Elem::Unit(v))
        }
        'o' | 'x' => {
            let radix = if e == 'o' { 8 } else { 16 };
            let braced = *i < b.len() && b[*i] == '{';
            if braced {
                *i += 1;
            }
            let mut v: u64 = 0;
            let mut nd = 0;
            let mut overflow = false;
            while *i < b.len() && b[*i].is_digit(radix) {
                v = v.checked_mul(radix as u64).and_then(|x| x.checked_add(b[*i].to_digit(radix).unwrap() as u64)).unwrap_or_else(|| {
                    overflow = true;
                    u64::MAX
                });
                *i += 1;
                nd += 1;
            }
            if braced {
                if *i < b.len() && b[*i] == '}' {
                    *i += 1;
                } else {
                    bad = true;
                }
            }
            if nd == 0 || (e == 'o' && !braced) {
                d.error(sp, Some("PRIM-18"), "escape sequence with no digits");
                bad = true;
            }
            if overflow || v > u32::MAX as u64 {
                d.error(sp, Some("PRIM-18"), "escape sequence out of range");
                bad = true;
            }
            Esc { elem: Elem::Unit(v.min(u32::MAX as u64) as u32), bad, named: false }
        }
        'u' | 'U' => {
            let braced = e == 'u' && *i < b.len() && b[*i] == '{';
            let mut v: u32 = 0;
            let mut nd = 0;
            if braced {
                *i += 1;
                while *i < b.len() && hex_digit(b[*i]) {
                    v = v.saturating_mul(16).saturating_add(b[*i].to_digit(16).unwrap());
                    *i += 1;
                }
                if *i < b.len() && b[*i] == '}' {
                    *i += 1;
                }
            } else {
                let want = if e == 'u' { 4 } else { 8 };
                while nd < want && *i < b.len() && hex_digit(b[*i]) {
                    v = v * 16 + b[*i].to_digit(16).unwrap();
                    *i += 1;
                    nd += 1;
                }
                if nd != want {
                    d.error(sp, Some("PRIM-18"), "incomplete universal character name");
                    bad = true;
                }
            }
            if (0xD800..=0xDFFF).contains(&v) || v > 0x10FFFF {
                d.error(sp, Some("PRIM-18"), "universal character name names a surrogate or is out of range");
                bad = true;
            }
            Esc { elem: Elem::Cp(v), bad, named: false }
        }
        'N' => {
            // PRIM-5 named escapes need the Unicode name table, which
            // this front end does not have.
            d.unsupported(sp, "named escape \\N{...} (needs Unicode name table)");
            while *i < b.len() && b[*i] != '}' && b[*i] != q {
                *i += 1;
            }
            if *i < b.len() && b[*i] == '}' {
                *i += 1;
            }
            Esc { elem: Elem::Cp('?' as u32), bad: true, named: true }
        }
        _ => {
            d.error(sp, Some("PRIM-18"), format!("unknown escape sequence '\\{}'", e));
            Esc { elem: Elem::Cp(e as u32), bad: true, named: false }
        }
    }
}

/// A character or string literal from the quote at `i`, with its
/// user-defined suffix. Pushes its token and gives the index after it.
pub fn quoted(b: &[char], mut i: usize, prefix: Prefix, sp: Span, out: &mut Vec<Token>, d: &mut Diags) -> usize {
    let q = b[i];
    i += 1;
    let mut elems = Vec::new();
    let mut bad = false;
    let mut named = false;
    while i < b.len() && b[i] != q && b[i] != '\n' {
        if b[i] == '\\' && i + 1 < b.len() {
            i += 1;
            let e = escape_value(b, &mut i, q, sp, d);
            bad |= e.bad;
            named |= e.named;
            elems.push(e.elem);
        } else {
            elems.push(Elem::Cp(b[i] as u32));
            i += 1;
        }
    }
    if i >= b.len() || b[i] != q {
        d.error(sp, None, "unterminated literal");
    }
    // The old front end went past the end here at the end of a file.
    i = (i + 1).min(b.len());
    // user-defined literal suffix
    let suf_start = i;
    let mut udl = false;
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
    let tok = if q == '"' { string_value(elems, prefix, sp) } else { char_literal(elems, prefix, bad, sp, d) };
    out.push(tok);
    if udl && out.len() == first + 1 {
        let t = out.pop().unwrap();
        let base = Box::new(t.lit.unwrap_or(Lit::Str { prefix, units: vec![] }));
        out.push(Token { kind: t.kind, text: suffix.clone(), span: sp, lit: Some(Lit::UserDefined { suffix, base, raw: String::new() }) });
    }
    i
}

fn string_value(elems: Vec<Elem>, prefix: Prefix, sp: Span) -> Token {
    let mut units = Vec::new();
    for e in &elems {
        match e {
            Elem::Cp(c) => units.extend(encode(&[*c], prefix)),
            Elem::Unit(u) => units.push(*u),
        }
    }
    Token { kind: TK::Str, text: String::new(), span: sp, lit: Some(Lit::Str { prefix, units }) }
}

/// `char_literal`: the value of a character literal from its elements
/// (PRIM-5, PRIM-6, PRIM-18, PRIM-23).
pub fn char_literal(elems: Vec<Elem>, prefix: Prefix, mut bad: bool, sp: Span, d: &mut Diags) -> Token {
    let unit_max: u64 = match prefix {
        Prefix::None | Prefix::U8 => 0xFF,
        Prefix::U16 => 0xFFFF,
        Prefix::U32 | Prefix::Wide => 0xFFFF_FFFF,
    };
    if elems.is_empty() {
        d.error(sp, Some("PRIM-18"), "empty character literal");
        bad = true;
    }
    let mut value: i128 = 0;
    let mut multi = false;
    if !bad {
        if elems.len() > 1 {
            if prefix != Prefix::None {
                d.error(sp, Some("PRIM-18"), "more than one character in a prefixed character literal");
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
                        d.error(sp, Some("PRIM-18"), "escape sequence too large for the character type");
                    }
                    value = *u as i128;
                }
                Elem::Cp(c) => {
                    let units = encode(&[*c], prefix);
                    if units.len() != 1 || (units[0] as u64) > unit_max {
                        if prefix == Prefix::None {
                            // PRIM-23: conditionally supported, no
                            // implementation in the matrix accepts it.
                            d.error(sp, Some("PRIM-23"), "character does not fit in one code unit");
                        } else {
                            d.error(sp, Some("PRIM-18"), "character does not fit in one code unit of its type");
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
    Token { kind: TK::Char, text: String::new(), span: sp, lit: Some(Lit::Char { prefix, value, multi }) }
}

/// The numeric part of a literal with a user-defined suffix. OPR-8: the
/// raw form and the template take the characters, so a value that no
/// integer type holds is an error (PRIM-2) only for the numeric form. Such
/// a value is kept as `u128::MAX`.
fn udl_base(num: &str, sp: Span, d: &mut Diags) -> (TK, Lit) {
    let mut inner = Diags::default();
    let base = number(num, sp, &mut inner);
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

/// A preprocessing number: an `integer_literal` or a `floating_literal`,
/// with digit separators (PRIM-16) and a user-defined suffix (OPR-8).
pub fn number(text: &str, sp: Span, d: &mut Diags) -> Token {
    let lower = text.to_ascii_lowercase();
    let is_hex = lower.starts_with("0x");
    let is_float = if is_hex { lower.contains('.') || lower.contains('p') } else { lower.contains('.') || (lower.contains('e') && !lower.starts_with("0b")) };
    if let Some(k) = text.find('_') {
        if k > 0 && !lower.starts_with("0x") || k > 2 {
            // OPR-8: a suffix that starts with `_` is a user-defined suffix.
            let num = &text[..k];
            let (kind, lit) = udl_base(num, sp, d);
            return Token { kind, text: text.to_string(), span: sp, lit: Some(Lit::UserDefined { suffix: text[k..].to_string(), base: Box::new(lit), raw: num.to_string() }) };
        }
    }
    let tok = |lit: Lit| Token { kind: if matches!(lit, Lit::Float { .. }) { TK::Float } else { TK::Int }, text: text.to_string(), span: sp, lit: Some(lit) };
    if text.matches('.').count() > 1 {
        // PRIM-26: the literal runs over each `.`, and the run is not one
        // valid literal.
        d.error(sp, Some("PRIM-26"), format!("'{}' is not one valid numeric literal", text));
        return tok(Lit::Float { value: 0.0, suffix: FSuffix::None });
    }
    // Digit separators: PRIM-16.
    let chars: Vec<char> = text.chars().collect();
    for (k, &c) in chars.iter().enumerate() {
        if c == '\'' {
            let ok = k > 0 && k + 1 < chars.len() && hex_digit(chars[k - 1]) && hex_digit(chars[k + 1]);
            if !ok {
                d.error(sp, Some("PRIM-16"), "digit separator must be between two digits");
            }
        }
    }
    let clean: String = text.chars().filter(|&c| c != '\'').collect();
    if is_float {
        return tok(floating_literal(&clean, is_hex, sp, d));
    }
    integer_literal(text, &clean, sp, d)
}

/// `integer_literal`: the value, base, and suffix of an integer literal
/// whose separators are removed (`clean`). PRIM-2, PRIM-16.
pub fn integer_literal(text: &str, clean: &str, sp: Span, d: &mut Diags) -> Token {
    let lower = text.to_ascii_lowercase();
    let tok = |lit: Lit| Token { kind: TK::Int, text: text.to_string(), span: sp, lit: Some(lit) };
    let (base, body_start) = if lower.starts_with("0x") {
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
        let is_digit = if base == 16 { hex_digit(c) } else { digit(c) };
        if !is_digit {
            break;
        }
        let dv = c.to_digit(16).unwrap();
        if dv >= base {
            d.error(sp, Some("PRIM-16"), format!("invalid digit '{}' in a base-{} literal", c, base));
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
        d.error(sp, Some("PRIM-16"), "base prefix without digits");
    }
    let suffix: String = cb[k..].iter().collect();
    let (unsigned, size) = match integer_suffix(&suffix) {
        Some(s) => s,
        // OPR-8, OPR-9 (Q-0383): a suffix that is not a form of the syntax
        // names a literal operator, also without `_`.
        None if suffix.starts_with(|c: char| c.is_ascii_alphabetic()) && suffix.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') => {
            let num = &text[..text.len() - suffix.len()];
            let (_, lit) = udl_base(num, sp, d);
            return Token { kind: TK::Int, text: text.to_string(), span: sp, lit: Some(Lit::UserDefined { suffix: suffix.clone(), base: Box::new(lit), raw: num.to_string() }) };
        }
        None => {
            d.error(sp, Some("PRIM-16"), format!("invalid integer suffix '{}'", suffix));
            (false, ISize::None)
        }
    };
    if overflow || value > u64::MAX as u128 {
        d.error(sp, Some("PRIM-2"), "integer literal too large for any type");
        value = 0;
    }
    tok(Lit::Int { value, decimal: base == 10, unsigned, size })
}

/// `floating_literal`: the value and suffix of a floating literal whose
/// separators are removed. PRIM-17.
pub fn floating_literal(s: &str, hex: bool, sp: Span, d: &mut Diags) -> Lit {
    let lower = s.to_ascii_lowercase();
    let bytes: Vec<char> = lower.chars().collect();
    let mut k = if hex { 2 } else { 0 };
    let mut mant = String::new();
    while k < bytes.len() && (bytes[k] == '.' || if hex { hex_digit(bytes[k]) } else { digit(bytes[k]) }) {
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
        while k < bytes.len() && digit(bytes[k]) {
            exp = exp.saturating_mul(10).saturating_add(bytes[k].to_digit(10).unwrap() as i64);
            k += 1;
        }
        if st == k {
            d.error(sp, Some("PRIM-17"), "exponent has no digits");
        }
        if neg {
            exp = -exp;
        }
    }
    if hex && !has_exp {
        d.error(sp, Some("PRIM-17"), "hexadecimal floating literal needs an exponent");
    }
    let suffix = &s[k.min(s.len())..];
    let suf = float_suffix(suffix).unwrap_or_else(|| {
        d.error(sp, Some("PRIM-17"), format!("invalid floating literal suffix '{}'", suffix));
        FSuffix::None
    });
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
