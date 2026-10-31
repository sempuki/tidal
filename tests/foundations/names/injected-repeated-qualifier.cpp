// tidal-rules: NAM-33
// tidal-expect: accept
struct C { static constexpr int value = 1; };
static_assert(C::C::C::value == 1);
