// tidal-rules: COND-9
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
struct S { int a : 3; int b : 3; };
void f(S& s, bool c) { auto p = &(c ? s.a : s.b); }
