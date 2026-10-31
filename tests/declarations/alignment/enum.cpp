// tidal-rules: ALN-1
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-expect-gcc-14: accept
// tidal-note: Q-0073. EDG and GCC 14 accept alignas on an enumeration. Clang and GCC 16 reject it.
enum alignas(8) E { a };
