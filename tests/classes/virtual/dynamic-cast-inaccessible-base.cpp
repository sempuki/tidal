// tidal-rules: VIRT-18
// tidal-expect: reject
struct A { virtual ~A() {} };
struct D : private A {};
A* f(D* d) { return dynamic_cast<A*>(d); }
