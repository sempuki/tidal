// tidal-rules: NAM-14
// tidal-expect: reject
// tidal-error-clang: type named .item. is hidden by a declaration in a different namespace
// tidal-error-gcc: reference to .item. is ambiguous
// tidal-error-edg: item. is ambiguous
namespace a { struct item { }; }
namespace b { int item = 0; }
namespace top { using namespace a; using namespace b; }
int read() { return top::item; }
