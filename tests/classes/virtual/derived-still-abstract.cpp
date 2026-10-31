// tidal-rules: VIRT-8
// tidal-expect: reject
struct A { virtual void f() = 0; virtual void g() = 0; };
struct B : A { void f() override {} };
B b;
