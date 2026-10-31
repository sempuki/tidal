// tidal-rules: UNEV-1
// tidal-expect: accept
constexpr int z = 0;
constexpr auto n = sizeof(1 / z);
static_assert(n == sizeof(int));
