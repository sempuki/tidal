// tidal-rules: MUL-15
// tidal-expect: accept
static_assert(sizeof(long) > sizeof(unsigned));
static_assert(__is_same(decltype(-1L / 2u), long));
static_assert(-1L / 2u == 0);
