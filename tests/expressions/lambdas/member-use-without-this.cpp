// tidal-rules: LAM-17
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct S { int v; int f() { return [] { return v; }(); } };
