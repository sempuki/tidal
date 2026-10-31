// tidal-rules: TYP-1
// tidal-expect: accept
// tidal-flags: -m32
static_assert(sizeof(short) == 2 && sizeof(int) == 4);
static_assert(sizeof(long) == 4 && sizeof(unsigned long) == 4);
static_assert(sizeof(long long) == 8 && sizeof(unsigned long long) == 8);
