// tidal-rules: TYP-10
// tidal-expect: accept
static_assert(16777216.0f + 1.0f == 16777216.0f);
static_assert(8388608.0f + 1.0f == 8388609.0f);
static_assert(9007199254740992.0 + 1.0 == 9007199254740992.0);
static_assert(4503599627370496.0 + 1.0 == 4503599627370497.0);
