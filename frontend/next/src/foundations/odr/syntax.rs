//! The Syntax segment of `foundations/odr`: the productions that
//! preprocessing reads before the start symbol applies (the mark
//! `preprocessing`). One function each. A directive line is passed
//! without its line break. `lex` calls these functions for the slice of
//! preprocessing that Tidal describes (INC-1 to INC-5).

/// The text of a directive line after `#` and white space, without a
/// comment at its end, or `None` for a line that is not a directive.
fn directive_text(line: &str) -> Option<&str> {
    let t = line.trim_start();
    let rest = t.strip_prefix('#')?;
    Some(rest.split("//").next().unwrap_or("").trim())
}

/// The first word of a directive and the text after it.
pub fn directive_word(text: &str) -> (&str, &str) {
    match text.find(|c: char| !(c.is_alphanumeric() || c == '_')) {
        Some(k) => (&text[..k], text[k..].trim()),
        None => (text, ""),
    }
}

/// `include_directive: "#" "include" '"' header_name '"'`. Gives the
/// header name. INC-1.
pub fn include_directive(line: &str) -> Option<&str> {
    let (word, rest) = directive_word(directive_text(line)?);
    if word != "include" {
        return None;
    }
    let inner = rest.strip_prefix('"')?;
    let end = inner.find('"')?;
    header_name(&inner[..end])
}

/// `header_name`: any characters other than a double quote or a line break.
pub fn header_name(s: &str) -> Option<&str> {
    (!s.contains(['"', '\n'])).then_some(s)
}

/// `guarded_header: {blank_line} include_guard {blank_line}`. Gives the
/// guard's name when the whole file is one guard (INC-4).
pub fn guarded_header(src: &str) -> Option<String> {
    let lines: Vec<&str> = src.lines().collect();
    let first = lines.iter().position(|l| !blank_line(l))?;
    let last = lines.iter().rposition(|l| !blank_line(l))?;
    include_guard(&lines[first..=last])
}

/// `include_guard`: `#ifndef NAME`, `#define NAME [guard_value]`, the
/// guarded lines, and `#endif`. Gives NAME.
pub fn include_guard(lines: &[&str]) -> Option<String> {
    if lines.len() < 3 {
        return None;
    }
    let (w, name) = directive_word(directive_text(lines[0])?);
    if w != "ifndef" || name.is_empty() {
        return None;
    }
    let (w2, rest) = directive_word(directive_text(lines[1])?);
    let (dname, value) = directive_word(rest);
    if w2 != "define" || dname != name {
        return None;
    }
    guard_value(value)?;
    let (we, _) = directive_word(directive_text(lines[lines.len() - 1])?);
    if we != "endif" {
        return None;
    }
    let mut k = 2;
    while k < lines.len() - 1 {
        k = guarded_line(lines, k)?;
    }
    (k == lines.len() - 1).then(|| name.to_string())
}

/// `guard_value`: any tokens up to the end of the line.
pub fn guard_value(s: &str) -> Option<&str> {
    (!s.contains('\n')).then_some(s.trim())
}

/// `guarded_line: text_line | conditional_group`. Gives the index after it.
pub fn guarded_line(lines: &[&str], k: usize) -> Option<usize> {
    if text_line(lines[k]) {
        return Some(k + 1);
    }
    conditional_group(lines, k)
}

/// `conditional_group: if_line {guarded_line} {else_line {guarded_line}} "#" "endif"`.
/// Gives the index after the `#endif`.
pub fn conditional_group(lines: &[&str], k: usize) -> Option<usize> {
    if_line(lines[k])?;
    let mut k = k + 1;
    while k < lines.len() {
        if else_line(lines[k]).is_some() {
            k += 1;
            continue;
        }
        if let Some(t) = directive_text(lines[k]) {
            if directive_word(t).0 == "endif" {
                return Some(k + 1);
            }
        }
        k = guarded_line(lines, k)?;
    }
    None
}

/// `if_line: "#" ("if" | "ifdef" | "ifndef") ...`. Gives the word and the rest.
pub fn if_line(line: &str) -> Option<(&str, &str)> {
    let (w, rest) = directive_word(directive_text(line)?);
    matches!(w, "if" | "ifdef" | "ifndef").then_some((w, rest))
}

/// `else_line: "#" ("elif" | "elifdef" | "elifndef") ... | "#" "else"`.
pub fn else_line(line: &str) -> Option<(&str, &str)> {
    let (w, rest) = directive_word(directive_text(line)?);
    matches!(w, "elif" | "elifdef" | "elifndef" | "else").then_some((w, rest))
}

/// `text_line`: a line that does not start with `#` and one of the words
/// of the conditional groups.
pub fn text_line(line: &str) -> bool {
    match directive_text(line) {
        None => true,
        Some(t) => !matches!(directive_word(t).0, "if" | "ifdef" | "ifndef" | "elif" | "elifdef" | "elifndef" | "else" | "endif"),
    }
}

/// `blank_line`: a line that holds only white space and a comment.
pub fn blank_line(line: &str) -> bool {
    let t = line.trim();
    t.is_empty() || t.starts_with("//") || (t.starts_with("/*") && t.ends_with("*/"))
}

/// `pragma_once: "#" "pragma" "once"` (INC-5).
pub fn pragma_once(line: &str) -> bool {
    let l = line.trim();
    let w: Vec<&str> = l.trim_start_matches('#').split_whitespace().take_while(|w| !w.starts_with("//")).collect();
    l.starts_with('#') && w == ["pragma", "once"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directives() {
        assert_eq!(include_directive("#include \"a.h\""), Some("a.h"));
        assert_eq!(include_directive("  #  include \"b c.h\" // note"), Some("b c.h"));
        assert_eq!(include_directive("#include <a.h>"), None);
        assert!(pragma_once("#pragma once"));
        assert!(!pragma_once("#pragma one"));
        assert_eq!(if_line("#ifndef G"), Some(("ifndef", "G")));
        assert_eq!(else_line("#else"), Some(("else", "")));
        assert!(text_line("int x;"));
        assert!(text_line("#define X 1"));
        assert!(!text_line("#endif"));
        assert!(blank_line("   // c"));
        assert_eq!(header_name("x\"y"), None);
        assert_eq!(guard_value(" 1 "), Some("1"));
    }

    #[test]
    fn guards() {
        let src = "\n#ifndef G\n#define G\nint x;\n#if A\nint y;\n#else\nint z;\n#endif\n#endif\n\n";
        assert_eq!(guarded_header(src), Some("G".to_string()));
        assert_eq!(guarded_header("#ifndef G\n#define H\n#endif\n"), None);
        let lines = ["#if X", "a", "#elif Y", "b", "#endif", "c"];
        assert_eq!(conditional_group(&lines, 0), Some(5));
        assert_eq!(guarded_line(&lines, 5), Some(6));
        assert_eq!(include_guard(&["#ifndef Q", "#define Q 2", "#endif"]), Some("Q".to_string()));
    }
}
