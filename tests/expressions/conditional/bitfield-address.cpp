// tidal-rules: COND-9
// tidal-expect: reject
// tidal-error-clang: address of bit-field requested
// tidal-error-gcc: take address of bit-field
// tidal-error-edg: address of a bit field
// tidal-note: Gap review 2026-10-30.
struct S { int a : 3; int b : 3; };
void f(S& s, bool c) { auto p = &(c ? s.a : s.b); }
