// tidal-rules: TYP-4
// tidal-expect: accept
// tidal-note: long long has 64 bits on both matrix targets. The lowest value has no literal, so the test writes it as the highest value negated, minus 1.
constexpr long long ll_max = 9223372036854775807LL;
constexpr long long ll_min = -ll_max - 1;
static_assert(ll_max == static_cast<long long>(~0ull >> 1));
static_assert(ll_min < 0 && ll_min + ll_max == -1);
static_assert(static_cast<unsigned long long>(ll_min) == 9223372036854775808ull);
static_assert(0ull - 1ull == 18446744073709551615ull);
static_assert(static_cast<short>(32767) + 1 == 32768);
static_assert(static_cast<short>(65535) == -1);
static_assert(static_cast<short>(32768) == -32768);
