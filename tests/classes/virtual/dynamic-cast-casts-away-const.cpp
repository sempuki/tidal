// tidal-rules: VIRT-18
// tidal-expect: reject
struct B { virtual ~B() {} };
struct D : B {};
D* f(const B* p) { return dynamic_cast<D*>(p); }
