// tidal-rules: MPTR-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct S { mutable int m = 0; };
void f(const S& s) { s.m = 1; }
