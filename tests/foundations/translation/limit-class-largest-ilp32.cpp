// tidal-rules: TR-15
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-flags: -m32
// tidal-error-gcc: size of type .Triple. is too large
// tidal-note: Q-0012. The size of the class is the largest value of std::size_t with -m32. Clang and EDG accept it, and GCC rejects it.
static_assert(sizeof(void*) == 4);
struct Triple { char a[0x55555555], b[0x55555555], c[0x55555555]; };
Triple largest;
static_assert(sizeof(Triple) == 0xffffffffu);
