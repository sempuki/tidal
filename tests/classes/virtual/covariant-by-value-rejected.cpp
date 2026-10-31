// tidal-rules: VIRT-15
// tidal-expect: reject
struct B { virtual B make(); };
struct D : B { D make(); };
