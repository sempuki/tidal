// tidal-rules: CONV-6
// tidal-expect: accept
constexpr float f = 0.1;
static_assert(f != 0.0f && f < 0.11f);
