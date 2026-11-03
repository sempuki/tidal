// tidal-rules: TR-15
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-flags: -m32
// tidal-error-gcc: size of type .Pair. is too large
// tidal-note: Q-0012. GCC's largest class with -m32 has 2^31 - 1 bytes. Clang and EDG accept a class of 2^31 bytes.
static_assert(sizeof(void*) == 4);
struct Pair { char first[0x40000000]; char second[0x40000000]; };
Pair past;
