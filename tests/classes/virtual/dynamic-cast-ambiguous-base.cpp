// tidal-rules: VIRT-18
// tidal-expect: reject
// tidal-error-clang: ambiguous conversion from derived class
// tidal-error-gcc: is an ambiguous base
// tidal-error-edg: base class .A. is ambiguous
struct A { virtual ~A() {} };
struct B : A {};
struct C : A {};
struct D : B, C {};
A* f(D* d) { return dynamic_cast<A*>(d); }
