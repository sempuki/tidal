// tidal-rules: NS-8
// tidal-expect: accept
namespace A { int x; }
namespace B { int x; }
using namespace A;
using namespace B;
int y = A::x;
