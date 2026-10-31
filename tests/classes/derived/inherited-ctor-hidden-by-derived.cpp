// tidal-rules: DER-8
// tidal-expect: run
struct B { int v; B(int) : v(1) {} };
struct D : B { using B::B; D(int) : B(0) { v = 2; } };
int main() { D d(0); return d.v == 2 ? 0 : 1; }
