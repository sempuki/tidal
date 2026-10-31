// tidal-rules: VIRT-15
// tidal-expect: reject
struct B { virtual B* get(); };
struct D;
struct E : B { D* get(); };
