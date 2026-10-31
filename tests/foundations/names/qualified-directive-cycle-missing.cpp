// tidal-rules: NAM-14
// tidal-expect: reject
// tidal-error-clang: no member named .value. in namespace .b.
// tidal-error-gcc: value. is not a member of .b
// tidal-error-edg: namespace .b. has no member .value.
namespace b { }
namespace a { using namespace b; }
namespace b { using namespace a; }
int read() { return b::value; }
