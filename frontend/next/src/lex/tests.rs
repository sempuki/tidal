//! Unit tests of the lexer: the tokens, their spans, literals, the
//! preprocessing slice, and the dump that `tools/frontend_check.py`
//! compares with the old front end's.

use super::*;
use crate::diag::Diags;

fn lex(src: &str) -> (Lexed, Diags) {
    let mut d = Diags::default();
    let l = lex_file(src, None, &mut d);
    (l, d)
}

#[test]
fn tokens_have_spans() {
    let (l, d) = lex("int x;\n  y >>= 2;");
    assert!(d.list.is_empty());
    let texts: Vec<&str> = l.toks.iter().map(|t| t.text.as_str()).collect();
    assert_eq!(texts, ["int", "x", ";", "y", ">>=", "2", ";", ""]);
    assert_eq!(l.toks[3].span, Span::new(FileId(0), 2, 3));
    assert_eq!(l.toks.last().unwrap().kind, TK::Eof);
}

#[test]
fn alternative_spellings_and_attributes() {
    let (l, _) = lex("a and b [[x]]");
    assert_eq!(l.toks[1].text, "&&");
    assert_eq!(l.toks[1].kind, TK::Punct);
    assert_eq!(l.toks[3].text, "[");
    assert_eq!(l.toks[4].text, "[");
}

#[test]
fn literals() {
    let (l, d) = lex("0x10 1.5f 'a' u8\"hi\" R\"d(x\ny)d\" 12_km \"s\"_x");
    assert!(d.list.is_empty(), "{:?}", d.list);
    assert!(matches!(l.toks[0].lit, Some(Lit::Int { value: 16, .. })));
    assert!(matches!(l.toks[1].lit, Some(Lit::Float { suffix: FSuffix::F, .. })));
    assert!(matches!(l.toks[2].lit, Some(Lit::Char { value: 97, .. })));
    assert!(matches!(&l.toks[3].lit, Some(Lit::Str { prefix: Prefix::U8, units }) if units.len() == 2));
    assert_eq!(l.toks[4].text, "x\ny");
    assert!(matches!(&l.toks[5].lit, Some(Lit::UserDefined { suffix, .. }) if suffix == "_km"));
    assert!(matches!(&l.toks[6].lit, Some(Lit::UserDefined { suffix, .. }) if suffix == "_x"));
    // The raw string's line break counts for the next line.
    let (l, _) = lex("R\"(a\nb)\" z");
    assert_eq!(l.toks[1].span.line, 2);
}

#[test]
fn literal_checks_name_prim_rules() {
    let (_, d) = lex("'\\q' 0b2 1e 1'");
    let rules: Vec<_> = d.list.iter().map(|x| x.rule).collect();
    assert!(rules.contains(&Some("PRIM-18")));
    assert!(rules.contains(&Some("PRIM-16")));
    assert!(rules.contains(&Some("PRIM-17")));
}

#[test]
fn the_slice() {
    let (l, d) = lex("#define N 3\n#if N > 2\nint a = N;\n#else\nint b;\n#endif\n#undef N\n#ifdef N\nint c;\n#endif\n");
    assert!(d.list.is_empty());
    let texts: Vec<&str> = l.toks.iter().map(|t| t.text.as_str()).collect();
    assert_eq!(texts, ["int", "a", "=", "3", ";", ""]);
    // The replacement takes the span of the name.
    assert_eq!(l.toks[3].span.line, 3);
    let (_, d) = lex("#define F(x) x\n#warning w\n");
    assert_eq!(d.list.len(), 2);
    assert!(d.list.iter().all(|x| x.sev == crate::diag::Severity::Unsupported));
}

#[test]
fn includes_keep_the_directive_line() {
    let dir = std::env::temp_dir().join(format!("tidal-next-lex-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("h.h"), "#pragma once\nint h;\n").unwrap();
    let main = dir.join("m.cpp");
    let src = "\n#include \"h.h\"\n#include \"h.h\"\nint m;\n#include \"none.h\"\n";
    std::fs::write(&main, src).unwrap();
    let mut d = Diags::default();
    let l = lex_file(src, Some(&main), &mut d);
    let texts: Vec<&str> = l.toks.iter().map(|t| t.text.as_str()).collect();
    assert_eq!(texts, ["int", "h", ";", "int", "m", ";", ""]);
    assert_eq!(l.toks[0].span.line, 2);
    assert_eq!(l.map.unit_line(l.toks[0].span), 2);
    assert_eq!(d.list[0].rule, Some("INC-3"));
    let dump = dump(&l, &d);
    assert!(dump.starts_with("Ident\tint\t2\tNone\n"));
    assert!(dump.contains("diag\tError\tSome(\"INC-3\")\t5\t"));
    let _ = std::fs::remove_dir_all(&dir);
}
