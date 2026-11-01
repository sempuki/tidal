// tidal-rules: MUL-6 MUL-7
// tidal-expect: accept
constexpr int lowest = -int(~0u >> 1) - 1;
static_assert(lowest / 1 == lowest);
static_assert(lowest % 1 == 0);
static_assert(lowest / 2 == -(int(~0u >> 1) / 2) - 1);
static_assert(lowest % 2 == 0);
static_assert(lowest % 3 == -2);
static_assert(lowest % -2 == 0);
static_assert(lowest / -2 == int(~0u >> 1) / 2 + 1);
static_assert((lowest / 3) * 3 + lowest % 3 == lowest);
