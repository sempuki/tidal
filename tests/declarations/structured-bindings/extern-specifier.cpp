// tidal-rules: SB-11
// tidal-expect: reject
// tidal-error-clang: cannot be declared .extern
// tidal-error-gcc: structured binding declaration cannot be .extern
// tidal-error-edg: cannot declare an explicit storage class other than static or thread_local
// tidal-note: Gap review 2026-10-31.
struct P { int a, b; };
extern auto [x, y] = P{1, 2};
