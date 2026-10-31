// tidal-rules: CE-15
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
int a, b;
constexpr bool lt = &a < &b;
