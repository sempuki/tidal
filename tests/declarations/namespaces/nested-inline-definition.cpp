// tidal-rules: NS-2
// tidal-expect: accept
namespace A::inline V1 { constexpr int x = 4; }
static_assert(A::x == 4 && A::V1::x == 4);
