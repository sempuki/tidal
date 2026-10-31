// tidal-rules: NAM-26
// tidal-expect: reject
// tidal-error-clang: reference to .item. is ambiguous
// tidal-error-gcc: reference to .item. is ambiguous
// tidal-error-edg: item. is ambiguous
namespace a { struct item { }; }
namespace b { int item = 0; }
using namespace a; using namespace b;
int read() { return item; }
