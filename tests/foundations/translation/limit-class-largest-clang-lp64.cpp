// tidal-rules: TR-15
// tidal-expect: accept
// tidal-note: An object of a class of 2^60 - 1 bytes is the largest that Clang 22 accepts on x86-64. Every compiler in the matrix accepts it.
static_assert(sizeof(void*) == 8);
struct Pair { char first[1ull << 59]; char second[(1ull << 59) - 1]; };
Pair largest;
static_assert(sizeof(Pair) == (1ull << 60) - 1);
