// tidal-rules: TR-15
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: structure 'Pair' is too large
// tidal-error-clang-18: static assertion failed
// tidal-note: Q-0350. Clang 18 accepts a class of 2^61 bytes and gives sizeof a wrong value, so the static_assert fails. Clang 22 rejects the class. GCC and EDG accept it.
static_assert(sizeof(void*) == 8);
struct Pair { char first[1ull << 60]; char second[1ull << 60]; };
Pair past;
static_assert(sizeof(Pair) == (1ull << 61));
