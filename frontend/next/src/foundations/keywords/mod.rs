//! `foundations/keywords`: the keywords (KW-1), the alternative
//! spellings (KW-2), and the words with a meaning only in some places
//! (KW-3). `lex` calls `alt_token`, and `syntax` calls `is_keyword` so
//! that no `identifier` is a keyword.

/// KW-1: the keywords. The grammar derives no name from a keyword.
pub const KEYWORDS: &[&str] = &[
    // fundamental types
    "void", "bool", "char", "char8_t", "char16_t", "char32_t", "wchar_t", "short", "int", "long", "signed", "unsigned",
    "float", "double",
    // placeholders and type queries
    "auto", "decltype",
    // specifiers
    "const", "volatile", "static", "extern", "thread_local", "register", "mutable", "inline", "constexpr", "consteval",
    "constinit", "typedef", "friend", "virtual", "explicit",
    // classes and access
    "class", "struct", "union", "enum", "public", "protected", "private", "operator", "this",
    // other declarations
    "namespace", "using", "static_assert", "alignas", "asm", "export", "template", "typename", "concept", "requires",
    // statements
    "if", "else", "switch", "case", "default", "for", "while", "do", "break", "continue", "return", "goto", "try",
    "catch",
    // expressions
    "new", "delete", "sizeof", "alignof", "noexcept", "typeid", "throw", "nullptr", "true", "false", "static_cast",
    "dynamic_cast", "const_cast", "reinterpret_cast",
    // coroutines (out of scope)
    "co_await", "co_yield", "co_return",
    // the alternative spellings (KW-2) are keywords too; `lex` turns them
    // into the operators they name
    "and", "or", "not", "bitand", "bitor", "xor", "compl", "and_eq", "or_eq", "xor_eq", "not_eq",
];

/// KW-1: whether a word is a keyword.
pub fn is_keyword(w: &str) -> bool {
    KEYWORDS.contains(&w)
}

/// KW-2 (and LOG-13): the operator that an alternative spelling names.
pub fn alt_token(s: &str) -> Option<&'static str> {
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

/// KW-3: the identifiers with a special meaning in some places only. The
/// parser reads them by position, with no lookup.
pub const CONTEXTUAL: &[&str] = &["override", "final", "import", "module"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords_and_spellings() {
        assert!(is_keyword("typeid"));
        assert!(is_keyword("co_await"));
        assert!(!is_keyword("final"));
        assert!(!is_keyword("override"));
        assert_eq!(alt_token("bitor"), Some("|"));
        assert_eq!(alt_token("final"), None);
        assert!(CONTEXTUAL.contains(&"final"));
        // Every alternative spelling is a keyword.
        for w in ["and", "or", "not", "bitand", "bitor", "xor", "compl", "and_eq", "or_eq", "xor_eq", "not_eq"] {
            assert!(is_keyword(w) && alt_token(w).is_some());
        }
    }
}
