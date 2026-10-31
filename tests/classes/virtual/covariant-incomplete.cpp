// tidal-rules: VIRT-4
// tidal-expect: reject
struct B { virtual B* get(); };
struct D;
struct E : B { D* get(); };
