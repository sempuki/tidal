// tidal-rules: ALN-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Q-0069. GCC accepts alignas weaker than the natural alignment. Clang and EDG reject it.
struct S { alignas(1) double d; };
