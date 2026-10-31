// tidal-rules: CVF-5
// tidal-expect: run
struct B { operator int() const { return 5; } };
struct D : B {};
int main() { int x = D{}; return x == 5 ? 0 : 1; }
