// tidal-rules: CMP-3
// tidal-expect: accept
static_assert(sizeof(short) < sizeof(int));
static_assert(static_cast<short>(-1) < static_cast<unsigned short>(1));
static_assert(static_cast<signed char>(-1) < static_cast<unsigned char>(1));
static_assert(!(-1 < 1ull));
static_assert(-1ll == ~0ull);
static_assert(-1 > 0u);
