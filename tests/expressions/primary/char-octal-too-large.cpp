// tidal-rules: PRIM-18
// tidal-expect: reject
// tidal-error-clang: octal escape sequence out of range
// tidal-error-gcc: octal escape sequence out of range
// tidal-error-edg: character value is out of range
char c = '\777';
