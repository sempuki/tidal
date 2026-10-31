// tidal-rules: VIRT-18
// tidal-expect: accept
struct B {};
struct D : B {};
B* f(D* p) { return dynamic_cast<B*>(p); }
