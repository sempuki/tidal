// tidal-rules: DER-8
// tidal-expect: run
struct B { int v; B(int x, int y = 10) : v(x + y) {} };
struct D : B { using B::B; };
int main() { D a(1); D b(1, 2); return a.v == 11 && b.v == 3 ? 0 : 1; }
