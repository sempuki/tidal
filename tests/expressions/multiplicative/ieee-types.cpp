// tidal-rules: MUL-20
// tidal-expect: accept
// tidal-note: Q-0004. The digits of each type match the IEEE 754 formats on x86-64.
constexpr double largest_double = 0x1.fffffffffffffp1023;
static_assert(1.0f + 0x1p-23f != 1.0f && 1.0f + 0x1p-24f == 1.0f);
static_assert(1.0 + 0x1p-52 != 1.0 && 1.0 + 0x1p-53 == 1.0);
static_assert(1.0L + 0x1p-63L != 1.0L && 1.0L + 0x1p-64L == 1.0L);
static_assert(largest_double / 0x1p1023 > 1.99);
static_assert(sizeof(float) == 4 && sizeof(double) == 8);
