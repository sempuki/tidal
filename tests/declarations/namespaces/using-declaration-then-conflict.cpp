// tidal-rules: NS-16
// tidal-expect: reject
// tidal-error-clang: conflicts with target of using declaration already in scope
// tidal-error-gcc: int x. conflicts with a previous declaration
// tidal-error-edg: x. has already been declared in the current scope
namespace N { int x; }
using N::x;
int x;
