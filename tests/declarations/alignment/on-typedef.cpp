// tidal-rules: ALN-2
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-error-clang: only applies to variables, data members and tag types
// tidal-error-gcc: alignas.* on a type alias
// tidal-error-edg: alignas does not apply here
// tidal-note: Q-0072. GCC 14 accepts alignas here. GCC 16, Clang, and EDG reject it.
alignas(16) typedef int I;
