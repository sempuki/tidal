// tidal-rules: PRIM-23
// tidal-expect: reject
// tidal-error-clang: character too large for enclosing character literal type
// tidal-error-gcc: not encodable in a single execution character code unit
// tidal-error-edg: cannot be represented in the specified character type
// tidal-note: Conditionally supported; no implementation in the matrix supports it. The character is written directly in the source, and UTF-8 needs three code units for it.
auto c = '€';
