// tidal-rules: DER-8
// tidal-expect: run
struct B { int v; B(int x) : v(x) {} B(int x, int y) : v(x + y) {} };
struct D : B { using B::B; int w = 9; };
int main() { D a(1); D b(2, 3); return a.v == 1 && b.v == 5 && a.w == 9 ? 0 : 1; }
