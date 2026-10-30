// tidal-rules: NS-8
// tidal-expect: reject
namespace A { int x; }
namespace B { int x; }
using namespace A;
using namespace B;
int y = x;
