// tidal-rules: VIRT-16
// tidal-expect: run
struct A { virtual int f() = 0; };
int A::f() { return 5; }
struct B : A { int f() override { return A::f() + 1; } };
int main() { B b; return b.f() == 6 ? 0 : 1; }
