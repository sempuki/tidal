// tidal-rules: PRIM-18
// tidal-expect: reject
// tidal-error-clang: hex escape sequence out of range
// tidal-error-gcc: hex escape sequence out of range
// tidal-error-edg: character value is out of range
char c = '\x100';
