// tidal-rules: CE-16
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
constexpr int later();
constexpr int v = later();
constexpr int later() { return 1; }
