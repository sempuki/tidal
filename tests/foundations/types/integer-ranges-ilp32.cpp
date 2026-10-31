// tidal-rules: TYP-4
// tidal-expect: accept
// tidal-flags: -m32
static_assert(0u - 1u == 4294967295u);
static_assert(0ul - 1ul == 4294967295ul);
static_assert(0ull - 1ull == 18446744073709551615ull);
