// tidal-rules: UN-7
// tidal-expect: reject
// tidal-error-gcc: outside the bounds of array
// tidal-note: Gap review 2026-10-31.
constexpr int f() { int a[2] = {1, 2}; return *(a + 2); }
static_assert(f() == 0);
