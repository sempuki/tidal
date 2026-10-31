// tidal-rules: REF-5
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
constexpr int f() { int* p = nullptr; int& r = *p; (void)r; return 0; }
static_assert(f() == 0);
