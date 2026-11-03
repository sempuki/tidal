// tidal-rules: TR-15
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: A class of 2^31 - 1 bytes is the largest that GCC accepts with -m32. Every compiler in the matrix accepts it.
static_assert(sizeof(void*) == 4);
struct Pair { char first[0x40000000]; char second[0x3fffffff]; };
Pair largest;
static_assert(sizeof(Pair) == 0x7fffffff);
