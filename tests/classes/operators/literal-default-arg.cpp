// tidal-rules: OPR-10
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: literal operator cannot have a default argument
// tidal-error-edg: literal operator cannot have default arguments
// tidal-note: Q-0060. GCC accepts a default argument on a literal operator. Clang and EDG reject it.
using Size = decltype(sizeof 0);
int operator""_d(const char*, Size = 0);
