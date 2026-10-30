// tidal-rules: BIT-5
// tidal-expect: accept
static_assert((-1 & 0xFF) == 255);
static_assert((-8 | 1) == -7);
static_assert((-1 ^ 1) == -2);
static_assert(~-1 == 0);
