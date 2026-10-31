// tidal-rules: INL-2
// tidal-expect: reject
// tidal-error-clang: redefinition of .f
// tidal-error-gcc: redefinition of .int f\(\)
// tidal-error-edg: has already been defined
inline int f() { return 1; }
inline int f() { return 1; }
