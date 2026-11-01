// tidal-rules: TYP-14
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: GCC keeps extra precision in intermediates on this target (MUL-17), so each sum is rounded by a cast.
static_assert(static_cast<double>(1.0 + 0x1p-53) == 1.0);
static_assert(static_cast<double>(1.0 + 0x1.8p-52) == 1.0 + 0x1p-51);
static_assert(static_cast<double>(1.0 + 0x1.1p-53) == 1.0 + 0x1p-52);
static_assert(static_cast<float>(1.0f + 0x1p-24f) == 1.0f);
static_assert(static_cast<float>(1.0f + 0x1.8p-23f) == 1.0f + 0x1p-22f);
