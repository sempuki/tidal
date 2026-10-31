// tidal-rules: SB-11
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
// tidal-expect-edg: accept
// tidal-expect-gcc-14: accept
// tidal-note: Q-PENDING: EDG and GCC 14 accept constinit on a structured binding. Clang and GCC 16 reject it.
struct P { int a, b; };
constinit auto [x, y] = P{1, 2};
