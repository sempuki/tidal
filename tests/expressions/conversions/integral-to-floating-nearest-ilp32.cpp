// tidal-rules: CONV-7
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32. The same choice as on x86-64. 33554433 lies nearer to 33554432, and 16777219 and 1 + 2^-24 lie halfway, so each goes to the neighbor whose last bit is zero.
constexpr float near = 33554433;
static_assert(near == 33554432.0f);
constexpr float tie = 16777219;
static_assert(tie == 16777220.0f);
constexpr float tie_down = 1.0 + 0x1p-24;
static_assert(tie_down == 1.0f);
constexpr float above = 1.0 + 0x1.8p-24;
static_assert(above == 1.0f + 0x1p-23f);
