// tidal-rules: CX-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
constexpr const int& r = 1;
static_assert(r == 1);
