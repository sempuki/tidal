//! Unit tests of the syntax part.

use super::syntax::*;
use crate::syntax::testing::*;

#[test]
fn primaries() {
    assert!(ok("this", primary));
    assert!(ok("(1)", primary));
    assert!(ok("\"a\" \"b\"", primary));
    assert!(sx("(xs + ...)", primary).unwrap().contains("(fold_expr"));
    assert!(ok("(... && xs)", fold_expr));
    assert!(ok("(0 + ... + xs)", fold_expr));
    assert!(ok("<<=", fold_op));
    assert!(sx("N::x", id_expression).unwrap().contains("(qualified_name"));
    assert!(sx("f<int>", id_expression).unwrap().contains("(template_id"));
    assert!(!sx("g<int>", id_expression).is_some());
    assert!(ok("::N::V<T>::x", qualified_name));
    assert!(ok("nullptr", literal));
}

#[test]
fn names() {
    let t = toks("N::V<int>::y");
    let mut st = stub();
    let mut p = crate::syntax::Parser::new(&t, &mut st, &[]);
    let (n, _, last) = nested_name(&mut p).unwrap();
    assert_eq!(last, Some("V"));
    assert_eq!(n.prod, "nested_name");
    assert!(identifier(&mut p).is_ok());
    assert!(ok("~T", |p| unqualified_id(p, crate::syntax::names::Qual::None, false)));
    let _ = string_literal;
}

#[test]
fn lexical_productions() {
    use super::lexical::*;
    let c = |s: &str| s.chars().collect::<Vec<char>>();
    assert!(id_start('_') && id_start('a') && !id_start('1') && letter('Z') && !letter('_'));
    assert!(digit('0') && nonzero_digit('9') && !nonzero_digit('0') && octal_digit('7') && !octal_digit('8'));
    assert!(nonzero_octal('1') && binary_digit('1') && hex_digit('F') && hex_letter('a') && !hex_letter('g'));
    assert_eq!(decimal_digits(&c("12'3u"), 0), Some(4));
    assert_eq!(octal_digits(&c("017"), 0), Some(3));
    assert_eq!(hex_digits(&c("0xFF"), 0), Some(4));
    assert_eq!(binary_digits(&c("0b10'1"), 0), Some(6));
    assert_eq!(integer_digits(&c("42"), 0), Some(2));
    assert_eq!(digit_seq(&c("1'000"), 0), Some(5));
    assert_eq!(hex_digit_seq(&c("a'b"), 0), Some(3));
    assert_eq!(exponent(&c("e-10"), 0), Some(4));
    assert_eq!(binary_exponent(&c("p+3"), 0), Some(3));
    assert_eq!(decimal_float(&c("1.5e3"), 0), Some(5));
    assert_eq!(decimal_float(&c(".5"), 0), Some(2));
    assert_eq!(decimal_float(&c("15"), 0), None);
    assert_eq!(hex_float(&c("0x1.8p1"), 0), Some(7));
    assert_eq!(hex_quad(&c("00e9"), 0), Some(4));
    assert_eq!(ucn(&c("\\u00e9"), 0), Some(6));
    assert_eq!(ucn(&c("\\u{1F600}"), 0), Some(9));
    assert_eq!(escape(&c("\\n"), 0), Some(2));
    assert_eq!(escape(&c("\\x{41}"), 0), Some(6));
    assert_eq!(escape(&c("\\N{LATIN SMALL LETTER A}"), 0), Some(24));
    assert_eq!(char_name(&c("SPACE"), 0), Some(5));
    assert_eq!(simple_escape('t'), Some(9));
    assert_eq!(delimiter(&c("ab(x"), 0), 2);
    assert!(d_char('a') && !d_char('(') && !d_char(' '));
    assert!(raw_char(&c("a)x\""), 0, &c(")x\"")) && !raw_char(&c(")x\""), 0, &c(")x\"")));
    assert!(c_char('a') && !c_char('\'') && s_char('\'') && !s_char('"'));
    assert_eq!(char_prefix("u8"), Some(crate::lex::Prefix::U8));
    assert_eq!(ud_suffix(&c("_km"), 0), Some(3));
    assert!(unsigned_suffix("U"));
    assert_eq!(length_suffix("ll"), Some(crate::lex::ISize::LL));
    assert_eq!(integer_suffix("uLL"), Some((true, crate::lex::ISize::LL)));
    assert_eq!(integer_suffix("lu"), Some((true, crate::lex::ISize::L)));
    assert_eq!(integer_suffix("q"), None);
    assert_eq!(float_suffix("bf16"), Some(crate::lex::FSuffix::Ext));
}

#[test]
fn literal_values_and_checks() {
    use super::lexical::*;
    use crate::diag::Diags;
    use crate::lex::Lit;
    let sp = crate::source::Span::default();
    let mut d = Diags::default();
    let t = number("0x1F", sp, &mut d);
    assert!(matches!(t.lit, Some(Lit::Int { value: 31, .. })));
    let t = integer_literal("10'0", "100", sp, &mut d);
    assert!(matches!(t.lit, Some(Lit::Int { value: 100, decimal: true, .. })));
    assert!(matches!(floating_literal("1.5", false, sp, &mut d), Lit::Float { value, .. } if value == 1.5));
    assert!(d.list.is_empty());
    number("18446744073709551616", sp, &mut d);
    assert_eq!(d.list[0].rule, Some("PRIM-2"));
    let mut out = Vec::new();
    let b: Vec<char> = "'ab'".chars().collect();
    quoted(&b, 0, crate::lex::Prefix::None, sp, &mut out, &mut d);
    assert!(matches!(out[0].lit, Some(Lit::Char { multi: true, .. })));
    let t = char_literal(vec![Elem::Cp('A' as u32)], crate::lex::Prefix::None, false, sp, &mut d);
    assert!(matches!(t.lit, Some(Lit::Char { value: 65, .. })));
    let b: Vec<char> = "\"x(a)x\"".chars().collect();
    let (end, tok) = raw_string(&b, 0, crate::lex::Prefix::None, sp, &mut d);
    assert_eq!((end, tok.text.as_str()), (7, "a"));
    assert_eq!(encode(&[0xE9], crate::lex::Prefix::U8), vec![0xC3, 0xA9]);
}
