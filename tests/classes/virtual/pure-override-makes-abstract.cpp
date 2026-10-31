// tidal-rules: VIRT-8
// tidal-expect: reject
struct A { virtual void f() {} };
struct B : A { void f() override = 0; };
B b;
