// tidal-rules: CLS-4
// tidal-expect: reject
// tidal-error-clang: does not match any declaration in .S
// tidal-error-gcc: no declaration matches
// tidal-error-edg: has no member .f
struct S {};
void S::f() {}
