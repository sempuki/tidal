// tidal-rules: TR-14
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: array is too large
// tidal-error-gcc: exceeds maximum object size
// tidal-note: Q-0012. GCC's largest array on x86-64 has 2^63 - 1 bytes. EDG accepts every array whose size fits in std::size_t.
static_assert(sizeof(void*) == 8);
char past[1ull << 63];
