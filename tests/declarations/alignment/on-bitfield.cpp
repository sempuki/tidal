// tidal-rules: ALN-2
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-note: Q-0072. GCC 14 accepts alignas here. GCC 16, Clang, and EDG reject it.
struct S { alignas(4) int b : 3; };
