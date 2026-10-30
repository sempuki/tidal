// tidal-rules: CMP-1
// tidal-expect: accept
static_assert((1 < 2 == 3 > 4) == false);
static_assert((3 > 2) == 1);
static_assert(1 + 1 == 2 && 2 * 2 > 3);
static_assert(!(1 == 2 != 0));
