// tidal-rules: MUL-8
// tidal-expect: accept
static_assert(~0u * 2u == ~0u - 1u);
static_assert(0x8000'0000'0000'0000ull * 2ull == 0);
static_assert(~0u / 2u == ~0u >> 1);
