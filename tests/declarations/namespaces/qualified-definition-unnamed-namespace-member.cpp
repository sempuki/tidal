// tidal-rules: NS-15
// tidal-expect: reject
// tidal-error-clang: does not match any declaration in namespace
// tidal-error-gcc: should have been declared inside
// tidal-error-edg: has no actual member
namespace geo { namespace { int area(); } }
int geo::area() { return 0; }      // area is declared in the unnamed namespace
