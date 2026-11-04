// tidal-rules: NS-15 NS-3
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: qualified name is not allowed in namespace member declaration
// tidal-note: Q-0375. area is declared in the inline namespace v2, so NS-15 checks the definition against v2. The definition is in geo, which encloses v2, so it is valid. GCC and Clang accept. EDG checks against geo, the namespace that the name gives, and rejects.
namespace geo {
  inline namespace v2 { int area(); }
  int geo::area() { return 4; }      // defines geo::v2::area
}
int main() { return geo::v2::area() == 4 ? 0 : 1; }
