// tidal-rules: TR-14
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: array is too large
// tidal-note: Q-0012. An array of 2^63 - 1 bytes is the largest that GCC accepts on x86-64. Clang rejects it, and EDG accepts it.
static_assert(sizeof(void*) == 8);
char largest[(1ull << 63) - 1];
