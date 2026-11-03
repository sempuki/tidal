// tidal-rules: TR-14
// tidal-expect: accept
// tidal-note: An array of 2^61 - 1 bytes is the largest that Clang accepts on x86-64. Every compiler in the matrix accepts it.
static_assert(sizeof(void*) == 8);
char largest[(1ull << 61) - 1];
