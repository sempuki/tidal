// tidal-rules: TYP-2
// tidal-expect: accept
// tidal-flags: -m32
// tidal-expect-edg-7.0-ilp32: reject
// tidal-error-edg-7.0-ilp32: static assertion failed
// tidal-note: Q-0120. EDG's i686 configuration aligns long long to 8 bytes; GCC and Clang align it to 4.
static_assert(alignof(short) == 2 && alignof(int) == 4 && alignof(long) == 4);
static_assert(alignof(long long) == 4 && alignof(void*) == 4);
static_assert(alignof(float) == 4 && alignof(double) == 4 && alignof(long double) == 4);
