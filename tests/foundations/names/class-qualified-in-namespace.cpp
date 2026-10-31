// tidal-rules: NAM-17
// tidal-expect: accept
namespace n { struct box { static constexpr int value = 1; }; }
static_assert(n::box::value == 1);
