// tidal-rules: CLS-5
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct S { void f(); };
void g(volatile S& s) { s.f(); }
