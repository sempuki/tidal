// tidal-rules: DCL-10
// tidal-expect: accept
static_assert(sizeof(void*) == 8);
char big[1ull << 32];
