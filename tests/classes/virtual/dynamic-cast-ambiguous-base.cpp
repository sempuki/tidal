// tidal-rules: VIRT-18
// tidal-expect: reject
struct A { virtual ~A() {} };
struct B : A {};
struct C : A {};
struct D : B, C {};
A* f(D* d) { return dynamic_cast<A*>(d); }
