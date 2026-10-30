// tidal-rules: MUL-10
// tidal-expect: reject
constexpr unsigned short big = 65535;
static_assert(sizeof(int) == 4);
static_assert(big * big != 0);
