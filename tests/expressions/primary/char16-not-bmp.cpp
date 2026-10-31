// tidal-rules: PRIM-18
// tidal-expect: reject
// tidal-error-clang: character too large for enclosing character literal type
// tidal-error-gcc: not encodable in a single code unit
// tidal-error-edg: invalid character for char16_t literal
auto c = u'\U0001F600';
