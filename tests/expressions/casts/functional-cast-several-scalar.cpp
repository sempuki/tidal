// tidal-rules: CAST-10
// tidal-expect: reject
// tidal-error-clang: excess elements in scalar initializer
// tidal-error-gcc: expression list treated as compound expression in functional cast
// tidal-error-edg: expected a .\).
int f() { return int(1, 2); }
