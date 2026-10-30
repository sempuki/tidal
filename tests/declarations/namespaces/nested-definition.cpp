// tidal-rules: NS-2
// tidal-expect: accept
namespace A::B::C { constexpr int x = 3; }
static_assert(A::B::C::x == 3);
