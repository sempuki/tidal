// tidal-rules: VIRT-3
// tidal-expect: reject
struct A { virtual void f(); };
struct B : A { void f() override final; };
struct C : B { void f(); };
