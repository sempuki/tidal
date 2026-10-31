// tidal-rules: ALN-2
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-error-clang: only applies to variables, data members and tag types
// tidal-error-gcc: alignas.* on a type alias
// tidal-note: Q-0072. GCC 14 accepts alignas here. GCC 16, Clang, and EDG reject it.
using I alignas(16) = int;
