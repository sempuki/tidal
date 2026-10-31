// tidal-rules: PRIM-16
// tidal-expect: reject
// tidal-error-clang: invalid digit .9. in octal constant
// tidal-error-gcc: invalid digit .9. in octal constant
// tidal-error-edg: invalid octal digit
int x = 019;
