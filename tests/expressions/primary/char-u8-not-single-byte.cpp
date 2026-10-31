// tidal-rules: PRIM-18
// tidal-expect: reject
// tidal-error-clang: character too large for enclosing character literal type
// tidal-error-gcc: not encodable in a single code unit
// tidal-error-edg: cannot occupy more than one code unit
auto c = u8'\u00e9';
