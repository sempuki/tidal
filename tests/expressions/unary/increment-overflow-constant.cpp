// tidal-rules: UN-21
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
constexpr int f() { int i = 2147483647; ++i; return i; }
static_assert(f() < 0);
