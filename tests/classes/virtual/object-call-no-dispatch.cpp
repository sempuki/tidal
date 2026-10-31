// tidal-rules: VIRT-1
// tidal-expect: run
struct B { virtual int f() { return 1; } };
struct D : B { int f() override { return 2; } };
int main() { D d; B b = d; return b.f() == 1 ? 0 : 1; }
