// tidal-rules: PFX-4
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
constexpr int f() { int a[2] = {}; a[2] = 1; return 0; }
static_assert(f() == 0);
