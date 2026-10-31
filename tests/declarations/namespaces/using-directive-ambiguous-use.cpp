// tidal-rules: NS-17
// tidal-expect: reject
// tidal-error-clang: reference to .x. is ambiguous
// tidal-error-gcc: reference to .x. is ambiguous
// tidal-error-edg: x. is ambiguous
namespace A { int x; }
namespace B { int x; }
using namespace A;
using namespace B;
int y = x;
