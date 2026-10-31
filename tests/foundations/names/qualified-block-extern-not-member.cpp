// tidal-rules: NAM-14
// tidal-expect: reject
// tidal-error-clang: no member named .value. in namespace .n.
// tidal-error-gcc: value. is not a member of .n
// tidal-error-edg: namespace .n. has no member .value.
namespace n { inline void f() { extern int value; } }
int read() { return n::value; }
