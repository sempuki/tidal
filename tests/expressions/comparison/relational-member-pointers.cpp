// tidal-rules: CMP-9
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
struct S { int a, b; };
bool f() { return &S::a < &S::b; }
