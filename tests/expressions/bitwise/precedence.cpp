// tidal-rules: BIT-1
// tidal-expect: accept
static_assert((6 & 3 ^ 1 | 8) == (((6 & 3) ^ 1) | 8));
static_assert((1 | 2 ^ 3 & 4) == (1 | (2 ^ (3 & 4))));
static_assert((1 & 2 == 2) == (1 & (2 == 2)));
static_assert((4 & 2 == 0) == 0);
static_assert((~0 & 1) == 1);
