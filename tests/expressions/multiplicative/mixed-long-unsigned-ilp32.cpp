// tidal-rules: MUL-15
// tidal-expect: accept
// tidal-flags: -m32
static_assert(sizeof(long) == sizeof(unsigned));
static_assert(__is_same(decltype(-1L / 2u), unsigned long));
static_assert(-1L / 2u == ~0ul / 2u);
