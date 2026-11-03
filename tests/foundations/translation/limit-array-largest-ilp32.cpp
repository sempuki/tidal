// tidal-rules: TR-14
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-flags: -m32
// tidal-error-gcc: exceeds maximum object size
// tidal-note: Q-0012. The size of the array is the largest value of std::size_t with -m32. Clang and EDG accept it, and GCC rejects it.
static_assert(sizeof(void*) == 4);
char largest[0xffffffffu];
