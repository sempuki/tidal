// tidal-rules: NAM-14
// tidal-expect: reject
// tidal-error-clang: reference to .value. is ambiguous
// tidal-error-gcc: reference to .value. is ambiguous
// tidal-error-edg: value. is ambiguous
namespace a { int value = 1; }
namespace b { int value = 2; }
namespace both { using namespace a; using namespace b; }
int read() { return both::value; }
