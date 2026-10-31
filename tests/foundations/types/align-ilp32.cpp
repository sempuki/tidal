// tidal-rules: TYP-2
// tidal-expect: accept
// tidal-flags: -m32
static_assert(alignof(short) == 2 && alignof(int) == 4 && alignof(long) == 4);
static_assert(alignof(long long) == 4 && alignof(void*) == 4);
static_assert(alignof(float) == 4 && alignof(double) == 4 && alignof(long double) == 4);
