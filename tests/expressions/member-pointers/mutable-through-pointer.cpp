// tidal-rules: MPTR-10
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct S { mutable int m = 0; };
void f(const S& s) { int S::* pm = &S::m; s.*pm = 1; }
