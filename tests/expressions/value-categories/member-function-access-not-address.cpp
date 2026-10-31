// tidal-rules: VC-2
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
struct S { int g(); };
auto f(S s) { return &s.g; }
