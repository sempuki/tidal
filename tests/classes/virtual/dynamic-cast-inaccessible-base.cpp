// tidal-rules: VIRT-18
// tidal-expect: reject
// tidal-error-clang: private base class
// tidal-error-gcc: inaccessible base
// tidal-error-edg: inaccessible base class
struct A { virtual ~A() {} };
struct D : private A {};
A* f(D* d) { return dynamic_cast<A*>(d); }
