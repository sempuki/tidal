// tidal-rules: NS-7
// tidal-expect: reject
// tidal-error-clang: target of using declaration conflicts with declaration already in scope
// tidal-error-gcc: N::x. conflicts with a previous declaration
// tidal-error-edg: x. has already been declared in the current scope
namespace N { int x; }
int x;
using N::x;
