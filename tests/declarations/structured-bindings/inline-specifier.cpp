// tidal-rules: SB-11
// tidal-expect: reject
// tidal-error-clang: cannot be declared .inline
// tidal-error-gcc: structured binding declaration cannot be .inline
// tidal-error-edg: structured binding cannot be declared .inline
// tidal-note: Gap review 2026-10-31.
struct P { int a, b; };
inline auto [x, y] = P{1, 2};
