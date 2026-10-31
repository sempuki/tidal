// tidal-rules: NAM-17
// tidal-expect: accept
struct base { static constexpr long value = 1; };
struct derived : base { static constexpr int value = 2; };
static_assert(derived::value == 2);
static_assert(derived::base::value == 1);
