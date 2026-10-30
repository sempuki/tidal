// tidal-rules: CONV-2
// tidal-expect: accept
constexpr unsigned u = -1;
constexpr unsigned char uc = 300;
constexpr unsigned short us = -2;
static_assert(u == ~0u && uc == 44 && us == 65534);
