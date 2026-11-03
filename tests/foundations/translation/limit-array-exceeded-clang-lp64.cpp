// tidal-rules: TR-14
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: array is too large
// tidal-note: Q-0012. Clang's largest array on x86-64 has 2^61 - 1 bytes. GCC and EDG accept an array of 2^61 bytes.
static_assert(sizeof(void*) == 8);
char past[1ull << 61];
