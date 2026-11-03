// tidal-rules: SHF-1
// tidal-expect: accept
// tidal-note: >> against - and >, and a chain that mixes << and >>.
static_assert(64 >> 1 + 1 == 16);
static_assert(64 >> 3 - 1 == 16);
static_assert((8 >> 1 > 3) == true);
static_assert(1 << 3 >> 1 == 4);
