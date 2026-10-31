// tidal-rules: COND-1
// tidal-expect: reject
// tidal-error-clang: not contextually convertible to .bool.
// tidal-error-gcc: could not convert .S\(\). from .S. to .bool.
// tidal-error-edg: must have bool type
struct S {};
int f() { return S{} ? 1 : 2; }
