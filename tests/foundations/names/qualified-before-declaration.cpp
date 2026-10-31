// tidal-rules: NAM-14
// tidal-expect: reject
// tidal-error-clang: no member named .value. in namespace .n.
// tidal-error-gcc: value. is not a member of .n
// tidal-error-edg: namespace .n. has no member .value.
namespace n { }
int read() { return n::value; }
namespace n { int value = 1; }
