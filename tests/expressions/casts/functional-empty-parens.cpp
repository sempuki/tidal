// tidal-rules: CAST-18
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
static_assert(int() == 0);
static_assert(double() == 0.0);
void f() { void(); }
