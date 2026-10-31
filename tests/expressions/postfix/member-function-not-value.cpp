// tidal-rules: PFX-14
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct S { int f() { return 1; } };
void g(S s) { auto x = s.f; }
