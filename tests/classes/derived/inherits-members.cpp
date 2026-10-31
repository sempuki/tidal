// tidal-rules: DER-2
// tidal-expect: run
struct B { int x = 3; int get() const { return x; } };
struct D : B { int y = 4; };
int main() { D d; return d.get() + d.y == 7 ? 0 : 1; }
