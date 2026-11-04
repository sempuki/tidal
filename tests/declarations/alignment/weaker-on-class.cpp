// tidal-rules: ALN-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: less than minimum alignment
// tidal-error-edg: less than the default alignment
// tidal-note: Q-0069. GCC accepts alignas weaker than the natural alignment of a class and keeps alignment 4. Clang and EDG reject it.
struct alignas(1) S { int x; };
