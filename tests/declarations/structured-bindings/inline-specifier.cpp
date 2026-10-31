// tidal-rules: SB-11
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct P { int a, b; };
inline auto [x, y] = P{1, 2};
