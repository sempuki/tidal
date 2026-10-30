// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
int g;
struct S { mutable int& r = g; };
