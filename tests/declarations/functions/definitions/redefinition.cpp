// tidal-rules: FN-14
// tidal-expect: reject
// tidal-error-clang: redefinition of .f
// tidal-error-gcc: redefinition of .int f\(\)
// tidal-error-edg: has already been defined
int f() { return 1; }
int f() { return 2; }
