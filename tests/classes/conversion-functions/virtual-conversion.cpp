// tidal-rules: CVF-1
// tidal-expect: run
struct B { virtual operator int() const { return 1; } };
struct D : B { operator int() const override { return 2; } };
int main() { D d; const B& b = d; int x = b; return x == 2 ? 0 : 1; }
