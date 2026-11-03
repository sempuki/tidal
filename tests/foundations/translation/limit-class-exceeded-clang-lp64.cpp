// tidal-rules: TR-15
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-expect-clang-18: accept
// tidal-error-clang: structure 'Pair' is too large
// tidal-note: Q-0012. Clang 22's largest class on x86-64 has 2^60 - 1 bytes. Clang 18, GCC, and EDG accept a class of 2^60 bytes.
static_assert(sizeof(void*) == 8);
struct Pair { char first[1ull << 59]; char second[1ull << 59]; };
Pair past;
