// tidal-rules: BIT-2
// tidal-expect: accept
enum Plain { A = 1, B = 2 };
static_assert((A | B) == 3);
static_assert((true & 1) == 1);
static_assert(('a' & 0x0F) == 1);
static_assert((5L ^ 1) == 4);
