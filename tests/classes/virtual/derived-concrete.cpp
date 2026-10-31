// tidal-rules: VIRT-8
// tidal-expect: run
struct A { virtual int f() = 0; };
struct B : A { int f() override { return 3; } };
int main() { B b; A& r = b; return r.f() == 3 ? 0 : 1; }
