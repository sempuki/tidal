// tidal-rules: TYP-14
// tidal-expect: accept
// One unit in the last place of 1.0 is 0x1p-23 in float and 0x1p-52 in double.
static_assert(1.0 + 0x1p-53 == 1.0);
static_assert(1.0 + 0x1.8p-52 == 1.0 + 0x1p-51);
static_assert(1.0 + 0x1.1p-53 == 1.0 + 0x1p-52);
static_assert(1.0 + 0x1.fp-54 == 1.0);
static_assert(-1.0 - 0x1p-53 == -1.0);
static_assert(1.0f + 0x1p-24f == 1.0f);
static_assert(1.0f + 0x1.8p-23f == 1.0f + 0x1p-22f);
