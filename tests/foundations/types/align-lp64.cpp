// tidal-rules: TYP-2
// tidal-expect: accept
static_assert(alignof(short) == 2 && alignof(int) == 4 && alignof(long) == 8);
static_assert(alignof(long long) == 8 && alignof(void*) == 8);
static_assert(alignof(float) == 4 && alignof(double) == 8 && alignof(long double) == 16);
