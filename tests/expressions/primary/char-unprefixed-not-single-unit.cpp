// tidal-rules: PRIM-23
// tidal-expect: reject
// tidal-error-clang: character too large for enclosing character literal type
// tidal-error-gcc: not encodable in a single execution character code unit
// tidal-error-edg: cannot be represented in the specified character type
// tidal-note: Gap review 2026-10-31. Conditionally supported; no implementation in the matrix supports it.
auto c = '\u00e9';
