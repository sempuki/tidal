// tidal-rules: CMP-3
// tidal-expect: accept
static_assert(!(-1 < 0u));
static_assert(-1 == ~0u);
static_assert(-1L < 0u || sizeof(long) == sizeof(unsigned));
