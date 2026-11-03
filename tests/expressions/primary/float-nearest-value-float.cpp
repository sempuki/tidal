// tidal-rules: PRIM-21
// tidal-expect: accept
// tidal-note: 0.1f lies between 13421772 and 13421773 times 2 to the power -27, nearer the second. Every matrix target gives the nearer value.
static_assert(0.1f == 13421773.0f / 134217728.0f);
static_assert(0.1f != 13421772.0f / 134217728.0f);
