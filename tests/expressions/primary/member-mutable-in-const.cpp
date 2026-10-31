// tidal-rules: PRIM-12
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct S { mutable int hits = 0; int* p = nullptr; void touch() const { ++hits; *p = 1; } };
