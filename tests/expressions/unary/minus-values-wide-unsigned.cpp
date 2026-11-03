// tidal-rules: UN-3
// tidal-expect: accept
// tidal-note: Unsigned long and unsigned long long operands, and a float.
static_assert(-1ull == ~0ull);
static_assert(-3ul == ~0ul - 2);
static_assert(-0ull == 0ull);
static_assert(-1.5f == 0.0f - 1.5f);
