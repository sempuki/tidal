// tidal-rules: NAM-26
// tidal-expect: reject
// tidal-error-clang: reference to .pick. is ambiguous
// tidal-error-gcc: reference to .pick. is ambiguous
// tidal-error-edg: pick. is ambiguous
namespace a { int pick(int); }
namespace b { int pick = 0; }
using namespace a;
using namespace b;
int read() { return pick; }
