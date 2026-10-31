// tidal-rules: CE-17
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
constexpr int f(bool b) { if (b) { goto done; } return 1; done: return 2; }
static_assert(f(false) == 1);
