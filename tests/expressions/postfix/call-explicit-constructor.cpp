// tidal-rules: PFX-16
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .f.
// tidal-error-gcc: could not convert .1. from .int. to .S.
// tidal-error-edg: no suitable constructor exists to convert from .int. to .S.
struct S { explicit S(int) {} };
void f(S);
void g() { f(1); }
