// tidal-rules: ALN-2
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-error-clang: cannot be applied to a bit-field
// tidal-error-gcc: alignas.* on bit-field
// tidal-error-edg: does not apply to bit fields
// tidal-note: Q-0072. GCC 14 accepts alignas here. GCC 16, Clang, and EDG reject it.
struct S { alignas(4) int b : 3; };
