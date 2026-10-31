// tidal-rules: DER-3
// tidal-expect: run
struct B { int f(int) { return 1; } };
struct D : B { using B::f; int f(int) { return 2; } };
int main() { D d; return d.f(0) == 2 ? 0 : 1; }
