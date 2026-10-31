// tidal-rules: DER-3
// tidal-expect: run
struct B { int f() { return 1; } };
struct D : B { int f() { return 2; } };
int main() { D d; return d.B::f() == 1 && d.f() == 2 ? 0 : 1; }
