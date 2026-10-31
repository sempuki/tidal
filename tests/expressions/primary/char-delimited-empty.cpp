// tidal-rules: PRIM-18
// tidal-expect: reject
// tidal-error-clang: delimited escape sequence cannot be empty
// tidal-error-gcc: empty delimited escape sequence
// tidal-error-edg: delimited escape sequence must have at least one character
char c = '\x{}';
