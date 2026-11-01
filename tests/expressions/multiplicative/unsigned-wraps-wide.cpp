// tidal-rules: MUL-8
// tidal-expect: accept
static_assert(~0ull * ~0ull == 1);
static_assert(~0u * 3u == ~0u - 2u);
static_assert((~0ull >> 1) * 4ull == ~0ull - 3ull);
static_assert(0u * ~0u == 0u);
static_assert(~0u / 2u * 2u + ~0u % 2u == ~0u);
