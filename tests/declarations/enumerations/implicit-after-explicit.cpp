// tidal-rules: ENUM-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
enum E { a = 5, b, c = 1, d };
static_assert(b == 6 && d == 2);
