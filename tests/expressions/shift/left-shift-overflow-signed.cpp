// tidal-rules: SHF-5
// tidal-expect: accept
constexpr int min_int = -int(~0u >> 1) - 1;
static_assert(1 << 31 == min_int);
static_assert((3 << 30) == -(1 << 30));
