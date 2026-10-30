// tidal-rules: SHF-2
// tidal-expect: accept
enum Plain { Three = 3 };
static_assert(1 << Three == 8);
static_assert(true << 2 == 4);
static_assert('\x01' << 3 == 8);
static_assert(1LL << 40 == 1099511627776LL);
