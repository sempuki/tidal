// tidal-rules: TYP-10
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: The largest finite value of each type has all fraction bits set and the largest exponent of its format.
constexpr float largest_float = 0x1.fffffep127f;
constexpr double largest_double = 0x1.fffffffffffffp1023;
constexpr long double largest_long_double = 0x1.fffffffffffffffep16383L;
static_assert(largest_float / 0x1p127f > 1.99f);
static_assert(largest_double / 0x1p1023 > 1.99);
static_assert(largest_long_double / 0x1p16383L > 1.99L);
static_assert(1.1e4932L > 1.0e4932L && largest_long_double > 1.18e4932L);
