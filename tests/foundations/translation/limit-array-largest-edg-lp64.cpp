// tidal-rules: TR-14
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: array is too large
// tidal-error-gcc: exceeds maximum object size
// tidal-note: Q-0012. The size of the array is the largest value of std::size_t. EDG accepts it, and Clang and GCC reject it.
static_assert(sizeof(void*) == 8);
char largest[~0ull];
