// tidal-rules: PRIM-17
// tidal-expect: reject
// tidal-error-clang: exponent has no digits
// tidal-error-gcc: exponent has no digits
// tidal-error-edg: extra text after expected end of number
double d = 1e+;
