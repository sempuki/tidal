// tidal-rules: PFX-17
// tidal-expect: accept
struct Doubler { constexpr int operator()(int v) const { return 2 * v; } };
static_assert(Doubler{}(4) == 8);
