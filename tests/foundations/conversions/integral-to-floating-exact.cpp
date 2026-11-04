// tidal-rules: CONV-6
// tidal-expect: accept
constexpr float f = 16777216;
constexpr double d = 1LL << 52;
static_assert(f == 16777216.0f && d == 4503599627370496.0);
