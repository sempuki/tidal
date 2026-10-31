// tidal-rules: VIRT-1
// tidal-expect: run
struct A { virtual int f() { return 1; } };
struct B : A {};
struct C : B { int f() override { return 3; } };
int main() { C c; A& r = c; return r.f() == 3 ? 0 : 1; }
