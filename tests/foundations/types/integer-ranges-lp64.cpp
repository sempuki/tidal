// tidal-rules: TYP-4
// tidal-expect: accept
static_assert(0u - 1u == 4294967295u);
static_assert(0ul - 1ul == 18446744073709551615ul);
static_assert(0ull - 1ull == 18446744073709551615ull);
constexpr int int_max = 2147483647;
static_assert(-int_max - 1 < 0);
static_assert(static_cast<unsigned short>(-1) == 65535);
