// tidal-rules: NS-7
// tidal-expect: accept
namespace N { constexpr int x = 8; }
using N::x;
static_assert(x == 8);
