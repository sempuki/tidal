// tidal-rules: VIRT-15
// tidal-expect: reject
struct B { virtual B* get(); };
struct D : B { const D* get() override; };
