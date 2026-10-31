// tidal-rules: VIRT-15
// tidal-expect: reject
struct B { virtual B* get(); };
struct X : private B {};
struct D : B { X* get(); };
