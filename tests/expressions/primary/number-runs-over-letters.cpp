// tidal-rules: PRIM-26
// tidal-expect: reject
// tidal-error-clang: invalid suffix .u8. on integer constant
// tidal-error-gcc: numeric literal operator .operator""u8.
// tidal-error-edg: extra text after expected end of number
int x = 1u8;
