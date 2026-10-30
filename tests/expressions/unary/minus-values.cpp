// tidal-rules: UN-3
// tidal-expect: accept
static_assert(-(5) == 0 - 5);
static_assert(-0u == 0u);
static_assert(-1u == ~0u);
static_assert(-2.5 == 0.0 - 2.5);
