// tidal-rules: CVF-1
// tidal-expect: reject
// tidal-error-clang: no viable conversion
// tidal-error-gcc: discards qualifiers
// tidal-error-edg: no suitable conversion function
struct S { operator int() { return 1; } };
int f(const S& s) { return s; }
