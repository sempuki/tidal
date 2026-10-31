// tidal-rules: VIRT-4
// tidal-expect: reject
struct B { virtual B* get(); };
struct X : private B {};
struct D : B { X* get(); };
