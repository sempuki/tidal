// tidal-rules: ALN-1
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: Q-0071. GCC uses the last alignas on a class, not the strictest. Clang and EDG use the strictest.
struct alignas(4) alignas(16) alignas(8) S { char c; };
static_assert(alignof(S) == 16);
