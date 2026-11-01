// tidal-rules: PRIM-26
// tidal-expect: accept
constexpr unsigned long long operator""_k(unsigned long long v) { return v * 1000; }
static_assert(2_k == 2000);
static_assert(0x1p+2 == 4.0);
static_assert(0x1P-1 == 0.5);
