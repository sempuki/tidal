// tidal-rules: PFX-11
// tidal-expect: reject
// tidal-error-clang: member reference type .S. is not a pointer
// tidal-error-gcc: base operand of .->. has non-pointer type
// tidal-error-edg: applied to .S. instead of to a pointer type
struct S { int v; };
int f(S s) { return s->v; }
