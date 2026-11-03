// tidal-rules: ALN-7
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-expect-gcc-14: accept
// tidal-error-clang: cannot be applied to an enumeration
// tidal-error-gcc: alignas.* on enumerated type
// tidal-note: Q-0073. EDG and GCC 14 accept alignas on an enumeration. Clang and GCC 16 reject it. The enumeration is scoped and has a fixed underlying type.
enum class alignas(16) E : int { a };
