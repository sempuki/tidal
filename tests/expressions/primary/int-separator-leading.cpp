// tidal-rules: PRIM-16
// tidal-expect: reject
// tidal-error-clang: invalid suffix .x.1F. on integer constant
// tidal-error-gcc: digit separator after base indicator
// tidal-error-edg: digit separator cannot appear here
int x = 0x'1F;
