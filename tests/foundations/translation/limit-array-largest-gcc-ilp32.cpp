// tidal-rules: TR-14
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: An array of 2^31 - 1 bytes is the largest that GCC accepts with -m32. Every compiler in the matrix accepts it.
static_assert(sizeof(void*) == 4);
char largest[0x7fffffffu];
