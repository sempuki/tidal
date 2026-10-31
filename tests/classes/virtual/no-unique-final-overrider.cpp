// tidal-rules: VIRT-5
// tidal-expect: reject
// tidal-error-clang: has more than one final overrider
// tidal-error-gcc: no unique final overrider
// tidal-error-edg: override of virtual function .A::f. is ambiguous
struct A { virtual void f(); };
struct B : virtual A { void f() override; };
struct C : virtual A { void f() override; };
struct D : B, C {};
