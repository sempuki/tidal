// tidal-rules: NAM-15
// tidal-expect: accept
struct box { static constexpr int value = 4; };
box b;
static_assert(decltype(b)::value == 4);
