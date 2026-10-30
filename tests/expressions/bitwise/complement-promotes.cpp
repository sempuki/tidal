// tidal-rules: BIT-4
// tidal-expect: accept
constexpr unsigned char zero = 0;
static_assert(~zero == -1);
static_assert(~0u == static_cast<unsigned>(-1));
