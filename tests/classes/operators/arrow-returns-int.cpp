// tidal-rules: OPR-14
// tidal-expect: reject
// tidal-error-clang: member reference type .int. is not a pointer
// tidal-error-gcc: yields non-pointer result
// tidal-error-edg: applied to .int. instead of to a pointer type
struct S { int operator->(); };
int f(S s) { return s->x; }
