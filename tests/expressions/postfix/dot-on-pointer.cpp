// tidal-rules: PFX-11
// tidal-expect: reject
// tidal-error-clang: member reference type .S \*. is a pointer
// tidal-error-gcc: which is of pointer type
// tidal-error-edg: must have class type but it has type .S \*.
struct S { int v; };
int f(S* p) { return p.v; }
