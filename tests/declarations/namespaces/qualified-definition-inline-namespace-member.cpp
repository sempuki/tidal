// tidal-rules: NS-15 NS-3
// tidal-expect: run
namespace geo { inline namespace v2 { int area(); } }
int geo::area() { return 4; }      // area is a member of geo through v2
int main() { return geo::v2::area() == 4 ? 0 : 1; }
