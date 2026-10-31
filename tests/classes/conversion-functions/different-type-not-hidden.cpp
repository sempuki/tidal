// tidal-rules: CVF-5
// tidal-expect: run
struct B { operator int() const { return 1; } };
struct D : B { operator double() const { return 2.0; } };
int main() { int x = static_cast<const B&>(D{}); D d; int y = d.operator int(); return x == 1 && y == 1 ? 0 : 1; }
