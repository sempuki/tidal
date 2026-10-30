// tidal-rules: SHF-1
// tidal-expect: accept
static_assert(1 << 2 << 1 == 8);
static_assert(1 << 1 + 1 == 4);
static_assert((1 << 2 < 5) == true);
static_assert(16 >> 2 >> 1 == 2);
