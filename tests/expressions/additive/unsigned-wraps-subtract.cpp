// tidal-rules: ADD-5
// tidal-expect: accept
static_assert(0u - 1u == ~0u);
static_assert(~0u + 2u == 1u);
static_assert(0ull - 2ull == ~0ull - 1ull);
static_assert(3u - 5 == ~0u - 1u);
static_assert(static_cast<unsigned char>(0) - 1 == -1);
