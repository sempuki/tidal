// tidal-rules: CAST-1
// tidal-expect: accept
static_assert(static_cast<int>(3.9) == 3);
static_assert(static_cast<unsigned char>(300) == 44);
static_assert(static_cast<double>(1) / 2 == 0.5);
