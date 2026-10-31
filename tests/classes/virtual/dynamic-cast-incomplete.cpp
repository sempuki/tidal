// tidal-rules: VIRT-10
// tidal-expect: reject
struct B { virtual ~B() {} };
struct D;
D* f(B* p) { return dynamic_cast<D*>(p); }
