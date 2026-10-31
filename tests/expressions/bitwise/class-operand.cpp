// tidal-rules: BIT-3
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
struct S {};
int f(S s) { return s & 1; }
