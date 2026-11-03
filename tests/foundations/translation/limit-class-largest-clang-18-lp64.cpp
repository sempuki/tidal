// tidal-rules: TR-15
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-expect-clang-18: accept
// tidal-error-clang: structure 'Pair' is too large
// tidal-note: Q-0012. Clang 18 gives the right size for a class of up to 2^61 - 1 bytes on x86-64. Clang 22 rejects this class. GCC and EDG accept it.
static_assert(sizeof(void*) == 8);
struct Pair { char first[1ull << 60]; char second[(1ull << 60) - 1]; };
Pair largest;
static_assert(sizeof(Pair) == (1ull << 61) - 1);
