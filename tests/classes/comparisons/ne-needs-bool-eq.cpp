// tidal-rules: DCMP-6
// tidal-expect: reject
// tidal-error-clang: rewritten .!=. comparison to be .bool
// tidal-error-gcc: is not .bool
// tidal-error-edg: does not return type bool
struct S { int v; int operator==(int o) const { return v == o; } };
bool f(S s) { return s != 1; }
