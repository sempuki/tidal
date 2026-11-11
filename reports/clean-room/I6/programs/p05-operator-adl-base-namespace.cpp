// DEP-9, NAM-27: an operator found in the namespace of the argument's base class.
namespace geo { struct Shape {}; }
namespace app { struct Circle : geo::Shape {}; }
template <class T> bool same(const T& a, const T& b) { return a == b; }
namespace geo { bool operator==(const Shape&, const Shape&) { return true; } }
int main() { return same(app::Circle{}, app::Circle{}) ? 0 : 1; }
