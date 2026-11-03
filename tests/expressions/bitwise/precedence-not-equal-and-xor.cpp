// tidal-rules: BIT-1
// tidal-expect: accept
// tidal-note: != against ^, & against | and ^ in a longer chain, and ~ against &.
static_assert((5 ^ 1 != 0) == (5 ^ (1 != 0)));
static_assert((1 | 6 & 3 ^ 4) == (1 | ((6 & 3) ^ 4)));
static_assert((12 & 10 & 9) == ((12 & 10) & 9));
static_assert((~5 & 7) == ((~5) & 7));
