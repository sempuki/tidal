// tidal-rules: VIRT-4
// tidal-expect: reject
struct B { virtual B make(); };
struct D : B { D make(); };
