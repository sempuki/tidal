// tidal-rules: NS-8
// tidal-expect: accept
namespace A { constexpr int x = 10; }
namespace B { using namespace A; }
using namespace B;
static_assert(x == 10);
