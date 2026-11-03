// tidal-rules: TR-14
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-flags: -m32
// tidal-error-gcc: exceeds maximum object size
// tidal-note: Q-0012. GCC's largest array with -m32 has 2^31 - 1 bytes. Clang and EDG accept an array of 2^31 bytes.
static_assert(sizeof(void*) == 4);
char past[0x80000000u];
