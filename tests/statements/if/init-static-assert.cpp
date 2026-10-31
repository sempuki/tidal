// tidal-rules: IF-21
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
void f(int v) { if (static_assert(true); v) {} }
