// tidal-rules: VIRT-18
// tidal-expect: reject
struct B {};
struct D : B {};
D* f(B* p) { return dynamic_cast<D*>(p); }
