// tidal-rules: PRIM-26
// tidal-expect: reject
// tidal-error-clang: invalid suffix .\+1. on integer constant
// tidal-error-gcc: numeric literal operator
// tidal-error-edg: extra text after expected end of number
int x = 0xe+1;
