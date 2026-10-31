// tidal-rules: NAM-17
// tidal-expect: accept
struct outer { struct inner { static constexpr int value = 3; }; };
static_assert(outer::inner::value == 3);
