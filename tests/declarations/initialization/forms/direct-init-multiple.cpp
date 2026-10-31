// tidal-rules: INIT-7
// tidal-expect: reject
// tidal-error-clang: excess elements in scalar initializer
// tidal-error-gcc: expression list treated as compound expression
// tidal-error-edg: too many initializer values
int x(1, 2);
