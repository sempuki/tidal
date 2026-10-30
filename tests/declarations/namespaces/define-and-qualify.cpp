// tidal-rules: NS-1
// tidal-expect: accept
namespace N { constexpr int x = 1; }
namespace N { constexpr int y = x + 1; }
static_assert(N::x == 1 && N::y == 2);
