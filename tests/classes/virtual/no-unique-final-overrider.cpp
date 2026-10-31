// tidal-rules: VIRT-5
// tidal-expect: reject
struct A { virtual void f(); };
struct B : virtual A { void f() override; };
struct C : virtual A { void f() override; };
struct D : B, C {};
