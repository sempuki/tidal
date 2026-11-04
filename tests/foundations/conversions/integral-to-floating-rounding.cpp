// tidal-rules: CONV-7
// tidal-expect: accept
constexpr float f = 16777217;
static_assert(f == 16777216.0f || f == 16777218.0f);
constexpr float g = 16777219;
static_assert(g == 16777218.0f || g == 16777220.0f);
