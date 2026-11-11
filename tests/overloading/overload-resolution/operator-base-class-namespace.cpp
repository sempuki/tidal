// tidal-rules: OVL-7 NAM-27
// tidal-expect: run
// tidal-note: I6-12. The class Circle is in the global namespace, and its base geo::Shape is in geo. The non-member candidates of + come from argument-dependent lookup, which takes the namespaces of the bases (NAM-27), so geo's operator+ is found. GCC, Clang, and EDG accept.
namespace geo {
struct Shape {};
int operator+(Shape, Shape) { return 0; }
}
struct Circle : geo::Shape {};
int main() { return Circle{} + Circle{}; }
