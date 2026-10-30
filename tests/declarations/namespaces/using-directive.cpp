// tidal-rules: NS-8
// tidal-expect: accept
namespace N { constexpr int x = 9; }
using namespace N;
static_assert(x == 9);
