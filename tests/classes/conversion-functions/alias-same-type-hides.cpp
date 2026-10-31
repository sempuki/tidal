// tidal-rules: CVF-5
// tidal-expect: run
using Int = int;
struct B { operator int() const { return 1; } };
struct D : B { operator Int() const { return 2; } };
int main() { int x = D{}; return x == 2 ? 0 : 1; }
