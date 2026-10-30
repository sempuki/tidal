// tidal-rules: CONV-2
// tidal-expect: accept
constexpr signed char sc = 200;
constexpr int i = 4294967295u;
constexpr short s = 40000;
static_assert(sc == -56 && i == -1 && s == -25536);
