// tidal-rules: VIRT-5
// tidal-expect: run
struct A { virtual int f() { return 1; } };
struct B : virtual A { int f() override { return 2; } };
struct C : virtual A { int f() override { return 3; } };
struct D : B, C { int f() override { return 4; } };
int main() { D d; A& r = d; return r.f() == 4 ? 0 : 1; }
