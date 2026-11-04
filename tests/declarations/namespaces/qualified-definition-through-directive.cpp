// tidal-rules: NS-15
// tidal-expect: reject
// tidal-error-clang: does not match any declaration in namespace
// tidal-error-gcc: should have been declared inside
// tidal-error-edg: has no actual member
namespace shapes { int area(); }
namespace geo { using namespace shapes; }
int geo::area() { return 0; }      // area is declared in shapes, not in geo
